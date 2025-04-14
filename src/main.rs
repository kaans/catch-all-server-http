use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Result, delete, error, get, options, post, put, web,
};
use chrono::serde::ts_seconds;
use futures::StreamExt;
use maud::{Markup, html};
use serde::{Serialize, Serializer};
use std::collections::BTreeMap;
use std::ops::Deref;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting catch-all server on port 8092");
    HttpServer::new(|| {
        App::new()
            .service(monitor_get)
            .service(monitor_post)
            .service(monitor_put)
            .service(monitor_delete)
            .service(monitor_options)
    })
    .bind(("127.0.0.1", 8092))?
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

const MAX_SIZE: usize = 262_144;

#[post("/{path:.*}")]
async fn monitor_post(req: HttpRequest, body: web::Payload) -> Result<HttpResponse> {
    parse_incoming_request(req, body).await
}

#[get("/{path:.*}")]
async fn monitor_get(req: HttpRequest, body: web::Payload) -> Result<HttpResponse> {
    parse_incoming_request(req, body).await
}

#[put("/{path:.*}")]
async fn monitor_put(req: HttpRequest, body: web::Payload) -> Result<HttpResponse> {
    parse_incoming_request(req, body).await
}

#[delete("/{path:.*}")]
async fn monitor_delete(req: HttpRequest, body: web::Payload) -> Result<HttpResponse> {
    parse_incoming_request(req, body).await
}

#[options("/{path:.*}")]
async fn monitor_options(req: HttpRequest, body: web::Payload) -> Result<HttpResponse> {
    parse_incoming_request(req, body).await
}

async fn parse_incoming_request(req: HttpRequest, body: web::Payload) -> Result<HttpResponse> {
    println!("=== Incoming request\n");

    let request = parse_request(req, body).await?;

    print_request(&request);

    Ok(match request.headers.get("accept") {
        Some(content_type) if content_type.contains("application/json") => {
            HttpResponse::Ok().json(web::Json(request))
        }
        None | Some(_) => HttpResponse::Ok()
            .content_type("text/html")
            .body(get_web_response(request)),
    })
}

fn print_request(request: &Request) {
    println!("{} {}", request.method, request.url);
    println!("Path: {}", request.path);
    print!("Query parameters:\n{}", request.query.iter().map(|(k, v)| format!("\t{} => {}\n", k.deref(), v.deref())).collect::<String>());
    print!("Headers:\n{}", request.headers.iter().map(|(k, v)| format!("\t{} => {}\n", k.deref(), v.deref())).collect::<String>());
    print!("Cookies:\n{}", request.cookies.iter().map(|(k, v)| format!("\t{} => {}\n", k.deref(), v.deref())).collect::<String>());
    println!("Body ({} bytes):\n{}", request.body.len(), String::from_utf8_lossy(request.body.to_vec().deref()));
}

fn get_web_response(result: Request) -> Markup {
    html! {
        html {
            body {
                h1 { "Incoming request" }

                p { b {"Timestamp: "} (format!("{}", result.time.format("%Y-%m-%d %H:%M:%S%.3f"))) }
                p { b {"Path: "} (result.path) }
                p { b {"Method: "} (result.method) }

                p { b {"Query parameters:"} }
                ul {
                    @for q in &result.query {
                        li { (q.0) " => " (q.1) }
                    }
                }

                p { b {"Headers:"} }
                ul {
                    @for h in &result.headers {
                        li { (h.0) " => " (h.1) }
                    }
                }

                p { b {"Cookies:"} }
                ul {
                    @for c in &result.cookies {
                        li { (c.0) " => " (c.1) }
                    }
                }

                @if result.body.len() > 0 {
                    p { b {"Body: "} (String::from_utf8_lossy(result.body.to_vec().deref())) }
                }
            }
        }
    }
}

async fn parse_request(req: HttpRequest, mut payload: web::Payload) -> Result<Request> {
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
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    let result = Request {
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

    Ok(result)
}
