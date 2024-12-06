use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use dotenv::dotenv;
use std::env;

#[derive(Serialize)]
struct ComputeResponse {
    message: String,
    powered_by: String,
}

#[derive(Serialize, Deserialize)]
struct Item {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct NewItem {
    name: String,
}

#[get("/compute")]
async fn compute() -> impl Responder {
    let result = 42; // Example computation
    let response = ComputeResponse {
        message: format!("The result of the computation is: {}", result),
        powered_by: "Sevalla Hosting".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[post("/items")]
async fn create_item(pool: web::Data<PgPool>, item: web::Json<NewItem>) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO items (name) VALUES ($1) RETURNING id, name",
        item.name
    )
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(record) => HttpResponse::Ok().json(Item { id: record.id, name: record.name }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/items/{id}")]
async fn get_item(pool: web::Data<PgPool>, item_id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query_as!(
        Item,
        "SELECT id, name FROM items WHERE id = $1",
        item_id.into_inner()
    )
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(compute)
            .service(create_item)
            .service(get_item)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}