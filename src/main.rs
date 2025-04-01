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
    body: String,
}

#[get("/")]
async fn monitor(req: HttpRequest, body: String) -> Result<impl Responder> {
    println!("Incoming request: {:?}", req);

    let mut headers: HashMap<String, String> = HashMap::new();
    req.headers().iter().for_each(|(k, v)| { headers.insert(k.to_string(), format!("{}", v.to_str().unwrap())); });

    let mut query: HashMap<String, String> = HashMap::new();
    req.query_string().split("&").for_each(|q| {
        let mut s = q.split("=");
        query.insert(s.next().unwrap_or("unknown").to_string(), s.next().unwrap_or("unknown").to_string());
    });

    let result = Request {
        url: req.uri().to_string(),
        path: req.path().to_string(),
        query,
        headers,
        body
    };

    Ok(web::Json(result))
}
