use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::{env, sync::Mutex};

#[derive(Deserialize)]
struct Info {
    username: String,
}

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request number: {}", counter)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "23564".to_string())
        .parse()
        .expect("PORT must be a number");
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .route("/", web::get().to(index))
            .service(mainpoll)
            .service(infpoll)
            .service(whethertymok)
            .service(whethertymokpoll)
            .service(slow)
            .service(random_entry)
            .service(random_poll)
            .service(random_cancel)
            .service(random_entry_staging)
            .service(random_poll_staging)
            .service(random_cancel_staging)
            .service(vs_cpu_entry_staging)
            .service(vs_cpu_entry)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}

#[post("/mainpoll")]
async fn mainpoll(info: web::Json<Info>) -> impl Responder {
    println!("Welcome {}!", info.username);
    HttpResponse::Ok().body(format!("Welcome {}!", info.username))
}

#[post("/infpoll")]
async fn infpoll(info: web::Json<Info>) -> impl Responder {
    println!("Welcome {}!", info.username);
    HttpResponse::Ok().body(format!("Welcome {}!", info.username))
}

#[post("/whethertymok")]
async fn whethertymok(info: web::Json<Info>) -> impl Responder {
    println!("Welcome {}!", info.username);
    HttpResponse::Ok().body(format!("Welcome {}!", info.username))
}

#[post("/whethertymokpoll")]
async fn whethertymokpoll(info: web::Json<Info>) -> impl Responder {
    println!("Welcome {}!", info.username);
    HttpResponse::Ok().body(format!("Welcome {}!", info.username))
}

#[post("/slow")]
async fn slow(info: web::Json<Info>) -> impl Responder {
    println!("Welcome {}!", info.username);
    HttpResponse::Ok().body(format!("Welcome {}!", info.username))
}

#[post("/random/entry")]
async fn random_entry(info: web::Json<Info>) -> impl Responder {
    println!("Welcome {}!", info.username);
    HttpResponse::Ok().body(format!("Welcome {}!", info.username))
}

#[post("/random/poll")]
async fn random_poll(info: web::Json<Info>) -> impl Responder {
    println!("Welcome {}!", info.username);
    HttpResponse::Ok().body(format!("Welcome {}!", info.username))
}

#[post("/random/cancel")]
async fn random_cancel(info: web::Json<Info>) -> impl Responder {
    println!("Welcome {}!", info.username);
    HttpResponse::Ok().body(format!("Welcome {}!", info.username))
}

#[post("/random/entry/staging")]
async fn random_entry_staging(info: web::Json<Info>) -> impl Responder {
    println!("Welcome {}!", info.username);
    HttpResponse::Ok().body(format!("Welcome {}!", info.username))
}

#[post("/random/poll/staging")]
async fn random_poll_staging(info: web::Json<Info>) -> impl Responder {
    println!("Welcome {}!", info.username);
    HttpResponse::Ok().body(format!("Welcome {}!", info.username))
}

#[post("/random/cancel/staging")]
async fn random_cancel_staging(info: web::Json<Info>) -> impl Responder {
    println!("Welcome {}!", info.username);
    HttpResponse::Ok().body(format!("Welcome {}!", info.username))
}

#[post("/vs_cpu/entry/staging")]
async fn vs_cpu_entry_staging(info: web::Json<Info>) -> impl Responder {
    println!("Welcome {}!", info.username);
    HttpResponse::Ok().body(format!("Welcome {}!", info.username))
}

#[post("/vs_cpu/entry")]
async fn vs_cpu_entry(info: web::Json<Info>) -> impl Responder {
    println!("Welcome {}!", info.username);
    HttpResponse::Ok().body(format!("Welcome {}!", info.username))
}
