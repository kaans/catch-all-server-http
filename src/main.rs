use std::collections::HashMap;
use actix_web::{get, web, App, HttpRequest, HttpServer, Responder, Result};
use serde::Serialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(monitor)
    })
        .bind(("127.0.0.1", 8092))?
        .run()
        .await
}

#[derive(Serialize)]
struct Request {
    url: String,
    path: String,
    query: HashMap<String, String>,
    headers: HashMap<String, String>,
    cookies: HashMap<String, String>,
    body: String,
}

#[get("/")]
async fn monitor(req: HttpRequest, body: String) -> Result<impl Responder> {
    println!("Incoming request: {:?}", req);

    let query = req.query_string().split("&").map(|q| {
        let mut s = q.split("=");
        return (s.next().unwrap_or("unknown").to_string(), s.next().unwrap_or("unknown").to_string());
    }).collect();

    let result = Request {
        url: req.uri().to_string(),
        path: req.path().to_string(),
        query,
        headers: req.headers().iter().map(|(k, v)| (k.to_string(), format!("{}", v.to_str().unwrap()))).collect(),
        cookies: req.cookies().map(|s| s.iter().map(|c| {
            (c.name().to_string(), c.value().to_string())
        }).collect()).unwrap_or_default(),
        body
    };

    Ok(web::Json(result))
}
