use actix_web::{web, get, App, HttpServer, Responder, HttpResponse, post};
use sqlx::mysql::MySqlPool;
use serde::Deserialize;
use std::env;

mod types;
use types::*;
mod common;
use common::*;


#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[post("/submit")]
async fn submit_score(pool: web::Data<MySqlPool>, item: web::Json<HighScore>) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO test (num, str) VALUES (?, ?)",
        item.score,
        item.username
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("High score submitted!"),
        Err(e) => {
            eprintln!("Failed to insert high score: {}", e);
            HttpResponse::InternalServerError().body("Failed to submit high score.")
        }
    }
}


#[post("/report")]
async fn report_bug(pool: web::Data<MySqlPool>, item: web::Json<BugReport>) -> impl Responder {
    let validation = item.validate();
    if let Err(e) = validation {
        return HttpResponse::BadRequest().body(e);
    }

    // let result = sqlx::query!(
    //     "INSERT INTO test (num, str) VALUES (?, ?)",
    //     item.score,
    //     item.username
    // )
    //     .execute(pool.get_ref())
    //     .await;

    // match result {
    //     Ok(_) => HttpResponse::Ok().body("High score submitted!"),
    //     Err(e) => {
    //         eprintln!("Failed to insert high score: {}", e);
    //         HttpResponse::InternalServerError().body("Failed to submit high score.")
    //     }
    // }

    HttpResponse::Ok().body("High score submitted!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let pool = MySqlPool::connect(&database_url).await.expect("Uh oh... failed to connect to database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(submit_score)
            .service(report_bug)
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}