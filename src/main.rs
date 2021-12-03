#![warn(clippy::pedantic)]
#![allow(clippy::unused_async)]
mod types;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{dev::ServiceRequest, post, web, App, Error, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
use cetkaik_core::absolute::Field;
use cetkaik_full_state_transition::{Rate, Season};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::{env, sync::Mutex};
use types::{
    AbsoluteCoord, MainMessage, MoveToBePolled, RetInfPoll, RetMainPoll, RetNormalMove, RetTaXot,
    RetTyMok, RetWhetherTyMokPoll, WhoGoesFirst,
};
use uuid::Uuid;

fn validate_token(str: &str) -> Result<bool, std::io::Error> {
    if (str.eq("a-secure-token")) {
        return Ok(true);
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Authentication failed!",
    ))
}

async fn bearer_auth_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let config = req.app_data::<Config>().cloned().unwrap_or_default();
    match validate_token(credentials.token()) {
        Ok(result) => {
            if result {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct AccessToken(Uuid);

impl AccessToken {
    /// # Errors
    /// Returns `Err` if the Uuid is not valid
    pub fn parse_str(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl std::fmt::Display for AccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_hyphenated().to_string())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct BotToken(Uuid);

impl BotToken {
    /// # Errors
    /// Returns `Err` if the Uuid is not valid
    pub fn parse_str(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl std::fmt::Display for BotToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_hyphenated().to_string(),)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]

pub struct RoomId(Uuid);

impl std::fmt::Display for RoomId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct AppState {
    access_counter: Mutex<i32>,
    waiting_list: Mutex<HashSet<AccessToken>>,
    person_to_room: Mutex<HashMap<AccessToken, RoomInfoWithPerspective>>,
    bot_to_room: Mutex<HashMap<BotToken, RoomInfoWithPerspective>>,
    room_to_bot: Mutex<HashMap<RoomId, BotToken>>,
    room_to_gamestate: Mutex<HashMap<RoomId, GameState>>,
}

impl AppState {
    pub fn analyze_valid_message_and_update(
        &self,
        message: MainMessage,
        room_info: &RoomInfoWithPerspective,
    ) -> RetNormalMove {
        todo!()
    }

    pub fn receive_tymok_and_update(&self, room_info: &RoomInfoWithPerspective) -> RetTyMok {
        todo!()
    }

    pub fn receive_taxot_and_update(&self, room_info: &RoomInfoWithPerspective) -> RetTaXot {
        todo!()
    }

    pub fn reply_to_whether_tymok_poll(
        &self,
        room_info: &RoomInfoWithPerspective,
    ) -> RetWhetherTyMokPoll {
        todo!()
    }

    pub fn reply_to_main_poll(&self, room_info: &RoomInfoWithPerspective) -> RetMainPoll {
        todo!()
    }

    pub fn reply_to_inf_poll(&self, room_info: &RoomInfoWithPerspective) -> RetInfPoll {
        todo!()
    }
}

struct SrcStep {
    src: AbsoluteCoord,
    step: AbsoluteCoord,
}

struct GameState {
    f: Field,
    tam_itself_is_tam_hue: bool,
    is_ia_owner_s_turn: bool,
    waiting_for_after_half_acceptance: Option<SrcStep>,
    season: Season,
    ia_owner_s_score: isize,
    rate: Rate,
    moves_to_be_polled: [Vec<MovePiece>; 4],
}

enum HandCompletionStatus {
    TyMok,
    TaXot,
    NotYetDetermined,
}

struct MovePiece {
    mov: MoveToBePolled,
    status: Option<HandCompletionStatus>,
    by_ia_owner: bool,
}

#[derive(Debug, Clone)]
pub struct RoomInfoWithPerspective {
    room_id: RoomId,
    is_first_move_my_move: [WhoGoesFirst; 4],
    is_ia_down_for_me: bool,
}

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
        bot_to_room: Mutex::new(HashMap::new()),
        room_to_bot: Mutex::new(HashMap::new()),
        room_to_gamestate: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(bearer_auth_validator);
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("*")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![
                        header::ORIGIN,
                        header::CONTENT_TYPE,
                        header::ACCEPT,
                        header::AUTHORIZATION,
                    ]),
            )
            .app_data(app_state.clone())
            .route("/", web::get().to(index))
            .service(mainpoll)
            .service(infpoll)
            .service(whethertymok_tymok)
            .service(whethertymok_taxot)
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
async fn mainpoll(data: web::Data<AppState>, auth: BearerAuth) -> impl Responder {
    HttpResponse::Ok().json(main_poll_(auth.token(), &data))
}

fn main_poll_(raw_token: &str, data: &web::Data<AppState>) -> RetMainPoll {
    match parse_token_and_get_room_info(raw_token, data) {
        Err(why_illegal) => RetMainPoll::Err { why_illegal },
        Ok(room_info) => data.reply_to_main_poll(&room_info),
    }
}

#[post("/infpoll")]
async fn infpoll(data: web::Data<AppState>, auth: BearerAuth) -> impl Responder {
    HttpResponse::Ok().json(inf_poll_(auth.token(), &data))
}

fn inf_poll_(raw_token: &str, data: &web::Data<AppState>) -> RetInfPoll {
    match parse_token_and_get_room_info(raw_token, data) {
        Err(why_illegal) => RetInfPoll::Err { why_illegal },
        Ok(room_info) => data.reply_to_inf_poll(&room_info),
    }
}

#[post("/whethertymok/tymok")]
async fn whethertymok_tymok(data: web::Data<AppState>, auth: BearerAuth) -> impl Responder {
    HttpResponse::Ok().json(whethertymok_tymok_(auth.token(), &data))
}

fn whethertymok_tymok_(raw_token: &str, data: &web::Data<AppState>) -> RetTyMok {
    match parse_token_and_get_room_info(raw_token, data) {
        Err(why_illegal) => RetTyMok::Err,
        Ok(room_info) => data.receive_tymok_and_update(&room_info),
    }
}

#[post("/whethertymok/taxot")]
async fn whethertymok_taxot(data: web::Data<AppState>, auth: BearerAuth) -> impl Responder {
    HttpResponse::Ok().json(whethertymok_taxot_(auth.token(), &data))
}

fn whethertymok_taxot_(raw_token: &str, data: &web::Data<AppState>) -> RetTaXot {
    match parse_token_and_get_room_info(raw_token, data) {
        Err(why_illegal) => RetTaXot::Err,
        Ok(room_info) => data.receive_taxot_and_update(&room_info),
    }
}

#[post("/whethertymokpoll")]
async fn whethertymokpoll(data: web::Data<AppState>, auth: BearerAuth) -> impl Responder {
    HttpResponse::Ok().json(whethertymokpoll_(auth.token(), &data))
}

fn whethertymokpoll_(raw_token: &str, data: &web::Data<AppState>) -> RetWhetherTyMokPoll {
    match parse_token_and_get_room_info(raw_token, data) {
        Err(why_illegal) => RetWhetherTyMokPoll::Err { why_illegal },
        Ok(room_info) => data.reply_to_whether_tymok_poll(&room_info),
    }
}

#[post("/slow")]
async fn slow(
    data: web::Data<AppState>,
    message: web::Json<MainMessage>,
    auth: BearerAuth,
) -> impl Responder {
    HttpResponse::Ok().json(slow_(auth.token(), &data, &message))
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
        Ok(room_info) => data.analyze_valid_message_and_update(**message, &room_info),
    }
}

#[post("/random/entry")]
async fn random_entry(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(matching::random_entry_(false, &data))
}

#[post("/random/entry/staging")]
async fn random_entry_staging(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(matching::random_entry_(true, &data))
}

#[post("/random/poll")]
async fn random_poll(
    msg: web::Json<MsgWithAccessToken>,
    data: web::Data<AppState>,
) -> impl Responder {
    HttpResponse::Ok().json(matching::random_entrance_poll_(false, &msg, &data))
}

#[post("/random/poll/staging")]
async fn random_poll_staging(
    msg: web::Json<MsgWithAccessToken>,
    data: web::Data<AppState>,
) -> impl Responder {
    HttpResponse::Ok().json(matching::random_entrance_poll_(true, &msg, &data))
}
mod matching;

#[post("/random/cancel")]
async fn random_cancel(
    msg: web::Json<MsgWithAccessToken>,
    data: web::Data<AppState>,
) -> impl Responder {
    HttpResponse::Ok().json(matching::random_entrance_cancel(false, &msg, &data))
}

#[post("/random/cancel/staging")]
async fn random_cancel_staging(
    msg: web::Json<MsgWithAccessToken>,
    data: web::Data<AppState>,
) -> impl Responder {
    HttpResponse::Ok().json(matching::random_entrance_cancel(true, &msg, &data))
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]

pub struct MsgWithAccessToken {
    access_token: String,
}

#[post("/vs_cpu/entry")]
async fn vs_cpu_entry(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(matching::vs_cpu_entry_(false, &data))
}

#[post("/vs_cpu/entry/staging")]
async fn vs_cpu_entry_staging(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(matching::vs_cpu_entry_(true, &data))
}
