mod args;

use crate::args::BodyFormat;
use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Result, delete, error, get, options, post, put, web,
};
use base64::prelude::*;
use chrono::serde::ts_seconds;
use clap::Parser;
use colored::Colorize;
use futures::StreamExt;
use maud::{Markup, html};
use serde::{Serialize, Serializer};
use std::collections::BTreeMap;
use std::ops::Deref;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = args::Args::parse();

    println!("Starting catch-all server on {}:{}", args.host, args.port);

    colored::control::set_override(args.use_color);

    let args_data = args.clone();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(args_data.clone()))
            .service(monitor_get)
            .service(monitor_post)
            .service(monitor_put)
            .service(monitor_delete)
            .service(monitor_options)
    })
    .bind((args.host, args.port))?
    .run()
    .await
}

fn serialize_bytes<S>(bytes: &web::BytesMut, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(String::from_utf8_lossy(bytes.to_vec().deref()).deref())
}

#[derive(Debug, Serialize)]
struct Request {
    #[serde(with = "ts_seconds")]
    time: chrono::DateTime<chrono::Utc>,
    url: String,
    path: String,
    method: String,
    query: BTreeMap<String, String>,
    headers: BTreeMap<String, String>,
    cookies: BTreeMap<String, String>,
    #[serde(serialize_with = "serialize_bytes")]
    body: web::BytesMut,
}

#[post("/{path:.*}")]
async fn monitor_post(
    req: HttpRequest,
    body: web::Payload,
    args: web::Data<args::Args>,
) -> Result<HttpResponse> {
    parse_incoming_request(req, body, args).await
}

#[get("/{path:.*}")]
async fn monitor_get(
    req: HttpRequest,
    body: web::Payload,
    args: web::Data<args::Args>,
) -> Result<HttpResponse> {
    parse_incoming_request(req, body, args).await
}

#[put("/{path:.*}")]
async fn monitor_put(
    req: HttpRequest,
    body: web::Payload,
    args: web::Data<args::Args>,
) -> Result<HttpResponse> {
    parse_incoming_request(req, body, args).await
}

#[delete("/{path:.*}")]
async fn monitor_delete(
    req: HttpRequest,
    body: web::Payload,
    args: web::Data<args::Args>,
) -> Result<HttpResponse> {
    parse_incoming_request(req, body, args).await
}

#[options("/{path:.*}")]
async fn monitor_options(
    req: HttpRequest,
    body: web::Payload,
    args: web::Data<args::Args>,
) -> Result<HttpResponse> {
    parse_incoming_request(req, body, args).await
}

async fn parse_incoming_request(
    req: HttpRequest,
    body: web::Payload,
    args: web::Data<args::Args>,
) -> Result<HttpResponse> {
    println!("{}\n", "=== Incoming request".on_bright_yellow());

    match parse_request(req, body, &args).await {
        Ok(request) => {
            print_request(&request, &args.body_format);

            Ok(match request.headers.get("accept") {
                Some(content_type) if content_type.contains("application/json") => {
                    HttpResponse::Ok().json(web::Json(request))
                }
                None | Some(_) => HttpResponse::Ok()
                    .content_type("text/html")
                    .body(get_web_response(request, &args.body_format)),
            })
        }
        Err(err) => {
            println!(
                "{}: {}\n",
                "Error while processing the request".on_bright_red(),
                err
            );
            Ok(HttpResponse::BadRequest().finish())
        }
    }
}

fn print_request(request: &Request, body_format: &BodyFormat) {
    println!("{} {}", request.method.green(), request.url.blue());
    println!("{} {}", "Path:".black().on_white(), request.path.blue());
    print!(
        "{} ({}):\n{}",
        "Query parameters".black().on_white(),
        request.query.len().to_string().bright_magenta(),
        request
            .query
            .iter()
            .map(|(k, v)| format!(
                "  {} {} {}\n",
                k.deref().on_magenta(),
                "=>".bright_cyan(),
                v.deref()
            ))
            .collect::<String>()
    );
    print!(
        "{} ({}):\n{}",
        "Headers:".black().on_white(),
        request.headers.len().to_string().bright_magenta(),
        request
            .headers
            .iter()
            .map(|(k, v)| format!(
                "  {} {} {}\n",
                k.deref().on_magenta(),
                "=>".bright_cyan(),
                v.deref()
            ))
            .collect::<String>()
    );
    print!(
        "{} ({}):\n{}",
        "Cookies:".black().on_white(),
        request.cookies.len().to_string().bright_magenta(),
        request
            .cookies
            .iter()
            .map(|(k, v)| format!(
                "  {} {} {}\n",
                k.deref().on_magenta(),
                "=>".bright_cyan(),
                v.deref()
            ))
            .collect::<String>()
    );

    let body = get_body_formatted(request, body_format);

    println!(
        "{}\n{}",
        format!(
            "Body ({} bytes):",
            request.body.len().to_string().bright_magenta()
        )
        .black()
        .on_white(),
        body
    );
}

fn get_body_formatted(request: &Request, body_format: &BodyFormat) -> String {
    let body = match body_format {
        BodyFormat::Text => String::from_utf8(request.body.to_vec())
            .unwrap_or(BASE64_STANDARD.encode(&request.body)),
        BodyFormat::Base64 => BASE64_STANDARD.encode(&request.body),
        BodyFormat::Hex => hex::encode(&request.body),
    };
    body
}

fn get_web_response(request: Request, body_format: &BodyFormat) -> Markup {
    html! {
        html {
            body {
                h1 { "Incoming request" }

                p { b {"Timestamp: "} (format!("{}", request.time.format("%Y-%m-%d %H:%M:%S%.3f"))) }
                p { b {"Path: "} (request.path) }
                p { b {"Method: "} (request.method) }

                p { b {"Query parameters:"} }
                ul {
                    @for q in &request.query {
                        li { (q.0) " => " (q.1) }
                    }
                }

                p { b {"Headers:"} }
                ul {
                    @for h in &request.headers {
                        li { (h.0) " => " (h.1) }
                    }
                }

                p { b {"Cookies:"} }
                ul {
                    @for c in &request.cookies {
                        li { (c.0) " => " (c.1) }
                    }
                }

                @if request.body.len() > 0 {
                    p { b {"Body: "} (get_body_formatted(&request, body_format)) }
                }
            }
        }
    }
}

async fn parse_request(
    req: HttpRequest,
    mut payload: web::Payload,
    args: &web::Data<args::Args>,
) -> Result<Request> {
    let query = req
        .query_string()
        .split("&")
        .filter(|q| !q.is_empty())
        .map(|q| {
            let mut s = q.split("=");

            return (
                s.next().unwrap_or("unknown").to_string(),
                s.next().unwrap_or("unknown").to_string(),
            );
        })
        .collect();

    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > args.max_size {
            return Err(error::ErrorBadRequest(format!(
                "Overflow, max size of {} bytes exceeded",
                args.max_size
            )));
        }
        body.extend_from_slice(&chunk);
    }

    let request = Request {
        time: chrono::Utc::now(),
        url: req.uri().to_string(),
        path: req.path().to_string(),
        method: req.method().to_string(),
        query,
        headers: req
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), format!("{}", v.to_str().unwrap())))
            .collect(),
        cookies: req
            .cookies()
            .map(|s| {
                s.iter()
                    .map(|c| (c.name().to_string(), c.value().to_string()))
                    .collect()
            })
            .unwrap_or_default(),
        body,
    };

    Ok(request)
}
