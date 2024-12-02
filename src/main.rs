use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    message: String,
    powered_by: String,
}

#[get("/compute")]
async fn compute() -> impl Responder {
    let result = 42; // Example computation
    let response = Response {
        message: format!("The result of the computation is: {}", result),
        powered_by: "Sevalla Hosting".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(compute)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}