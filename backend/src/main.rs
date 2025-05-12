use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenvy::dotenv;

mod routes;
use routes::task_routes::task_routes;

// Simple test route
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("API is live")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create DB pool");

    println!("🚀 Server running at http://0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(task_routes)
            .route("/healthcheck", web::get().to(healthcheck))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

