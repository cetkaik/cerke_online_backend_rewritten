#![warn(clippy::pedantic)]
#![allow(clippy::unused_async)]

pub mod types;
pub mod matching;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use std::collections::{HashMap, HashSet};
use std::{env, sync::Mutex};
use crate::types::{
    AfterHalfAcceptanceMessage, MainMessage, RetAfterHalfAcceptance,
    RetInfPoll, RetMainPoll, RetNormalMove, RetTaXot, RetTyMok, RetWhetherTyMokPoll,
    AppState, RoomInfoWithPerspective, MsgWithAccessToken, AccessToken
};

async fn index(data: web::Data<AppState>) -> String {
    let mut counter = data.access_counter.lock().unwrap();
    *counter += 1;
    format!("Request number: {}", counter)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "23564".to_string())
        .parse()
        .expect("PORT must be a number");
    let app_state = web::Data::new(AppState {
        access_counter: Mutex::new(0),
        waiting_list: Mutex::new(HashSet::new()),
        person_to_room: Mutex::new(HashMap::new()),
        rooms_where_opponent_is_bot: Mutex::new(HashSet::new()),
        room_to_gamestate: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::ORIGIN,
                header::CONTENT_TYPE,
                header::ACCEPT,
                header::AUTHORIZATION,
            ]);

        let origin = env::var("ORIGIN");
        let cors = if let Ok(origin) = origin { 
            cors.allowed_origin(&origin)
        } else { 
            cors.allow_any_origin().send_wildcard()
        };

        App::new()
            .wrap(
                cors,
            )
            .app_data(app_state.clone())
            .route("/", web::get().to(index))
            .service(mainpoll)
            .service(infpoll)
            .service(whethertymok_tymok)
            .service(whethertymok_taxot)
            .service(whethertymokpoll)
            .service(decision_main)
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

#[post("/poll/main")]
async fn mainpoll(data: web::Data<AppState>, auth: BearerAuth) -> impl Responder {
    HttpResponse::Ok().json(main_poll_(auth.token(), &data))
}

fn main_poll_(raw_token: &str, data: &web::Data<AppState>) -> RetMainPoll {
    match parse_token_and_get_room_info(raw_token, data) {
        Err(why_illegal) => RetMainPoll::Err { why_illegal },
        Ok(room_info) => data.reply_to_main_poll(&room_info),
    }
}

#[post("/poll/inf")]
async fn infpoll(data: web::Data<AppState>, auth: BearerAuth) -> impl Responder {
    HttpResponse::Ok().json(inf_poll_(auth.token(), &data))
}

fn inf_poll_(raw_token: &str, data: &web::Data<AppState>) -> RetInfPoll {
    match parse_token_and_get_room_info(raw_token, data) {
        Err(why_illegal) => RetInfPoll::Err { why_illegal },
        Ok(room_info) => data.reply_to_inf_poll(&room_info),
    }
}

#[post("/decision/tymok")]
async fn whethertymok_tymok(data: web::Data<AppState>, auth: BearerAuth) -> impl Responder {
    HttpResponse::Ok().json(whethertymok_tymok_(auth.token(), &data))
}

fn whethertymok_tymok_(raw_token: &str, data: &web::Data<AppState>) -> RetTyMok {
    match parse_token_and_get_room_info(raw_token, data) {
        Err(_why_illegal) => RetTyMok::Err,
        Ok(room_info) => data.receive_tymok_and_update(&room_info),
    }
}

#[post("/decision/taxot")]
async fn whethertymok_taxot(data: web::Data<AppState>, auth: BearerAuth) -> impl Responder {
    HttpResponse::Ok().json(whethertymok_taxot_(auth.token(), &data))
}

fn whethertymok_taxot_(raw_token: &str, data: &web::Data<AppState>) -> RetTaXot {
    match parse_token_and_get_room_info(raw_token, data) {
        Err(_why_illegal) => RetTaXot::Err,
        Ok(room_info) => data.receive_taxot_and_update(&room_info),
    }
}

#[post("/poll/whethertymok")]
async fn whethertymokpoll(data: web::Data<AppState>, auth: BearerAuth) -> impl Responder {
    HttpResponse::Ok().json(whethertymokpoll_(auth.token(), &data))
}

fn whethertymokpoll_(raw_token: &str, data: &web::Data<AppState>) -> RetWhetherTyMokPoll {
    match parse_token_and_get_room_info(raw_token, data) {
        Err(why_illegal) => RetWhetherTyMokPoll::Err { why_illegal },
        Ok(room_info) => data.reply_to_whether_tymok_poll(&room_info),
    }
}

#[post("/decision/main")]
async fn decision_main(
    data: web::Data<AppState>,
    message: web::Json<MainMessage>,
    auth: BearerAuth,
) -> impl Responder {
    HttpResponse::Ok().json(slow_(auth.token(), &data, &message))
}

#[post("/decision/afterhalfacceptance")]
async fn slow2(
    data: web::Data<AppState>,
    message: web::Json<AfterHalfAcceptanceMessage>,
    auth: BearerAuth,
) -> impl Responder {
    HttpResponse::Ok().json(slow2_(auth.token(), &data, &message))
}

fn parse_token_and_get_room_info(
    raw_token: &str,
    data: &web::Data<AppState>,
) -> Result<RoomInfoWithPerspective, String> {
    match AccessToken::parse_str(raw_token) {
        Err(e) => Err(format!(
            "Unparsable access token `{}`; failed because of {}",
            raw_token, e
        )),
        Ok(access_token) => {
            let person_to_room = data.person_to_room.lock().unwrap();
            match person_to_room.get(&access_token) {
                None => Err(format!("Unrecognized access token `{}`", raw_token)),
                Some(room_info) => Ok((*room_info).clone()),
            }
        }
    }
}

fn slow_(
    raw_token: &str,
    data: &web::Data<AppState>,
    message: &web::Json<MainMessage>,
) -> RetNormalMove {
    match parse_token_and_get_room_info(raw_token, data) {
        Err(why_illegal) => RetNormalMove::Err { why_illegal },
        Ok(room_info) => data.analyze_main_message_and_update(**message, &room_info),
    }
}

fn slow2_(
    raw_token: &str,
    data: &web::Data<AppState>,
    message: &web::Json<AfterHalfAcceptanceMessage>,
) -> RetAfterHalfAcceptance {
    match parse_token_and_get_room_info(raw_token, data) {
        Err(why_illegal) => RetAfterHalfAcceptance::Err { why_illegal },
        Ok(room_info) => data.analyze_afterhalfacceptance_message_and_update(**message, &room_info),
    }
}

#[post("/matching/random/entry")]
async fn random_entry(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(matching::random_entry_(false, &data))
}

#[post("/matching/random/entry/staging")]
async fn random_entry_staging(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(matching::random_entry_(true, &data))
}

#[post("/matching/random/poll")]
async fn random_poll(
    msg: web::Json<MsgWithAccessToken>,
    data: web::Data<AppState>,
) -> impl Responder {
    HttpResponse::Ok().json(matching::random_entrance_poll_(false, &msg, &data))
}

#[post("/matching/random/poll/staging")]
async fn random_poll_staging(
    msg: web::Json<MsgWithAccessToken>,
    data: web::Data<AppState>,
) -> impl Responder {
    HttpResponse::Ok().json(matching::random_entrance_poll_(true, &msg, &data))
}

#[post("/matching/random/cancel")]
async fn random_cancel(
    msg: web::Json<MsgWithAccessToken>,
    data: web::Data<AppState>,
) -> impl Responder {
    HttpResponse::Ok().json(matching::random_entrance_cancel(false, &msg, &data))
}

#[post("/matching/random/cancel/staging")]
async fn random_cancel_staging(
    msg: web::Json<MsgWithAccessToken>,
    data: web::Data<AppState>,
) -> impl Responder {
    HttpResponse::Ok().json(matching::random_entrance_cancel(true, &msg, &data))
}

#[post("/matching/vs_cpu/entry")]
async fn vs_cpu_entry(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(matching::vs_cpu_entry_(false, &data))
}

#[post("/matching/vs_cpu/entry/staging")]
async fn vs_cpu_entry_staging(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(matching::vs_cpu_entry_(true, &data))
}
