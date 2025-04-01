use actix_web::{App, HttpRequest, HttpServer, Responder, Result, get, post, web};
use maud::{Markup, html};
use serde::Serialize;
use std::collections::BTreeMap;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(monitor).service(monitor_post).service(monitor_json))
        .bind(("127.0.0.1", 8092))?
        .run()
        .await
}

#[derive(Debug, Serialize)]
struct Request {
    url: String,
    path: String,
    method: String,
    query: BTreeMap<String, String>,
    headers: BTreeMap<String, String>,
    cookies: BTreeMap<String, String>,
    body: Option<String>,
}

#[post("/")]
async fn monitor_post(req: HttpRequest, body: String) -> Result<Markup> {
    println!("Incoming request: {:?}", req);

    let result = parse_request(req, Some(body));

    get_web_response(result)
}

#[get("/")]
async fn monitor(req: HttpRequest) -> Result<Markup> {
    println!("Incoming request: {:?}", req);

    let result = parse_request(req, None);

    get_web_response(result)
}

fn get_web_response(result: Request) -> Result<Markup> {
    Ok(html! {
        html {
            body {
                h1 { "Incoming request" }

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

                @if result.body.is_some() {
                    p { b {"Body: "} (result.body.unwrap()) }
                }
            }
        }
    })
}

#[get("/json")]
async fn monitor_json(req: HttpRequest) -> Result<impl Responder> {
    println!("Incoming request: {:?}", req);

    let result = parse_request(req, None);

    Ok(web::Json(result))
}

fn parse_request(req: HttpRequest, body: Option<String>) -> Request {
    let query = req
        .query_string()
        .split("&")
        .map(|q| {
            let mut s = q.split("=");
            return (
                s.next().unwrap_or("unknown").to_string(),
                s.next().unwrap_or("unknown").to_string(),
            );
        })
        .collect();

    let result = Request {
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
    result
}
