mod types;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use cetkaik_core::absolute::*;
use cetkaik_full_state_transition::{Rate, Season};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::{env, sync::Mutex};
use types::{
    AbsoluteCoord, MoveToBePolled, RetRandomCancel, RetRandomEntry, RetTaXot, RetTyMok,
    WhoGoesFirst,
};

#[derive(Deserialize)]
struct Info {
    username: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct AccessToken(Uuid);

impl AccessToken {
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
struct BotToken(Uuid);

impl BotToken {
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

struct RoomId(Uuid);

impl std::fmt::Display for RoomId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct AppState {
    access_counter: Mutex<i32>,
    waiting_list: Mutex<HashSet<AccessToken>>,
    person_to_room: Mutex<HashMap<AccessToken, RoomInfoWithPerspective>>,
    bot_to_room: Mutex<HashMap<BotToken, RoomInfoWithPerspective>>,
    room_to_bot: Mutex<HashMap<RoomId, BotToken>>,
    room_to_gamestate: Mutex<HashMap<RoomId, GameState>>,
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

struct RoomInfoWithPerspective {
    room_id: RoomId,
    is_first_move_my_move: [WhoGoesFirst; 4],
    is_IA_down_for_me: bool,
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
async fn mainpoll(info: web::Json<Info>) -> impl Responder {
    println!("Welcome {}!", info.username);
    HttpResponse::Ok().body(format!("Welcome {}!", info.username))
}

#[post("/infpoll")]
async fn infpoll(info: web::Json<Info>) -> impl Responder {
    println!("Welcome {}!", info.username);
    HttpResponse::Ok().body(format!("Welcome {}!", info.username))
}

#[post("/whethertymok/tymok")]
async fn whethertymok_tymok(info: web::Json<Info>) -> impl Responder {
    println!("Welcome {}!", info.username);
    HttpResponse::Ok().json(&RetTyMok::Err)
}

#[post("/whethertymok/taxot")]
async fn whethertymok_taxot(info: web::Json<Info>) -> impl Responder {
    println!("Welcome {}!", info.username);
    HttpResponse::Ok().json(&RetTaXot::Err)
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
async fn random_entry(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(random_entry_(false, data))
}
use uuid::Uuid;

use crate::types::RetVsCpuEntry;

fn open_a_room(_token: AccessToken, _new_token: AccessToken, _is_staging: bool) -> RoomId {
    RoomId(Uuid::new_v4())
}

fn open_a_room_against_bot(_token: BotToken, _new_token: AccessToken, _is_staging: bool) -> RoomId {
    RoomId(Uuid::new_v4())
}

pub trait RemoveRandom {
    type Item;

    fn remove_random<R: rand::Rng>(&mut self, rng: &mut R) -> Option<Self::Item>;
}

// code adopted from https://stackoverflow.com/questions/53755017/can-i-randomly-sample-from-a-hashset-efficiently
impl<T> RemoveRandom for Vec<T> {
    type Item = T;

    fn remove_random<R: rand::Rng>(&mut self, rng: &mut R) -> Option<Self::Item> {
        if self.is_empty() {
            None
        } else {
            let index = rng.gen_range(0..self.len());
            Some(self.swap_remove(index))
        }
    }
}

fn random_entry_(is_staging: bool, data: web::Data<AppState>) -> RetRandomEntry {
    use rand::Rng;
    let new_token = AccessToken(Uuid::new_v4());
    let mut rng = rand::thread_rng();
    let mut waiting_list = data.waiting_list.lock().unwrap();
    let mut person_to_room = data.person_to_room.lock().unwrap();
    let mut room_to_gamestate = data.room_to_gamestate.lock().unwrap();
    let mut waiting_list_vec: Vec<AccessToken> = (*waiting_list).iter().cloned().collect();
    let opt_token = waiting_list_vec.remove_random(&mut rng);
    if let Some(token) = opt_token {
        (*waiting_list).remove(&token);
        let room_id = open_a_room(token, new_token, is_staging);

        let is_first_turn_newtoken_turn: [WhoGoesFirst; 4] = [
            WhoGoesFirst::new(&mut rng),
            WhoGoesFirst::new(&mut rng),
            WhoGoesFirst::new(&mut rng),
            WhoGoesFirst::new(&mut rng),
        ];

        let is_ia_down_for_newtoken: bool = rng.gen();

        person_to_room.insert(
            new_token,
            RoomInfoWithPerspective {
                room_id,
                is_first_move_my_move: is_first_turn_newtoken_turn.clone(),
                is_IA_down_for_me: is_ia_down_for_newtoken,
            },
        );
        person_to_room.insert(
            token,
            RoomInfoWithPerspective {
                room_id,
                is_first_move_my_move: [
                    is_first_turn_newtoken_turn[0].not(),
                    is_first_turn_newtoken_turn[1].not(),
                    is_first_turn_newtoken_turn[2].not(),
                    is_first_turn_newtoken_turn[3].not(),
                ],
                is_IA_down_for_me: !is_ia_down_for_newtoken,
            },
        );

        let is_ia_owner_s_turn =
            is_first_turn_newtoken_turn[0 /* spring */].result == is_ia_down_for_newtoken;
        room_to_gamestate.insert(
            room_id,
            GameState {
                tam_itself_is_tam_hue: true,
                season: Season::Iei2,
                rate: Rate::X1,
                ia_owner_s_score: 20,
                is_ia_owner_s_turn,
                f: Field {
                    board: yhuap_initial_board(),
                    a_side_hop1zuo1: vec![],
                    ia_side_hop1zuo1: vec![],
                },
                waiting_for_after_half_acceptance: None,
                moves_to_be_polled: [vec![], vec![], vec![], vec![]],
            },
        );
        return RetRandomEntry::LetTheGameBegin {
            access_token: format!("{}", new_token),
            is_first_move_my_move: is_first_turn_newtoken_turn[0 /* spring */].clone(),
            is_ia_down_for_me: is_ia_down_for_newtoken,
        };
    }

    todo!()
}

#[post("/random/poll")]
async fn random_poll(info: web::Json<Info>) -> impl Responder {
    println!("Welcome {}!", info.username);
    HttpResponse::Ok().body(format!("Welcome {}!", info.username))
}

#[post("/random/entry/staging")]
async fn random_entry_staging(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(random_entry_(true, data))
}

#[post("/random/poll/staging")]
async fn random_poll_staging(info: web::Json<Info>) -> impl Responder {
    println!("Welcome {}!", info.username);
    HttpResponse::Ok().body(format!("Welcome {}!", info.username))
}

#[post("/random/cancel")]
async fn random_cancel(
    msg: web::Json<MsgWithAccessToken>,
    data: web::Data<AppState>,
) -> impl Responder {
    HttpResponse::Ok().json(random_entrance_cancel(false, msg, data))
}

#[post("/random/cancel/staging")]
async fn random_cancel_staging(
    msg: web::Json<MsgWithAccessToken>,
    data: web::Data<AppState>,
) -> impl Responder {
    HttpResponse::Ok().json(random_entrance_cancel(true, msg, data))
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]

struct MsgWithAccessToken {
    access_token: String,
}

use big_s::S;
fn random_entrance_cancel(
    _is_staging: bool,
    msg: web::Json<MsgWithAccessToken>,
    data: web::Data<AppState>,
) -> RetRandomCancel {
    if let Ok(access_token) = AccessToken::parse_str(&msg.access_token) {
        let person_to_room = data.person_to_room.lock().unwrap();
        let mut waiting_list = data.waiting_list.lock().unwrap();
        match person_to_room.get(&access_token) {
            // you already have a room. you cannot cancel
            Some(_) => RetRandomCancel::Ok { cancellable: false },
            None => {
                if waiting_list.contains(&access_token) {
                    // not yet assigned a room, but is in the waiting list
                    waiting_list.remove(&access_token);
                    RetRandomCancel::Ok { cancellable: true }
                } else {
                    // You told me to cancel, but I don't know you. Hmm...
                    // well, at least you can cancel
                    RetRandomCancel::Ok { cancellable: true }
                }
            }
        }
    } else {
        RetRandomCancel::Err {
            why_illegal: S("access token could not be parsed"),
        }
    }
}

#[post("/vs_cpu/entry")]
async fn vs_cpu_entry(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(vs_cpu_entry_(false, data))
}

#[post("/vs_cpu/entry/staging")]
async fn vs_cpu_entry_staging(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(vs_cpu_entry_(true, data))
}

fn vs_cpu_entry_(is_staging: bool, data: web::Data<AppState>) -> RetVsCpuEntry {
    use rand::Rng;
    let new_token = AccessToken(Uuid::new_v4());
    let bot_token = BotToken(Uuid::new_v4());
    let room_id = open_a_room_against_bot(bot_token, new_token, is_staging);
    let mut rng = rand::thread_rng();
    let is_first_turn_newtoken_turn = [
        WhoGoesFirst::new(&mut rng),
        WhoGoesFirst::new(&mut rng),
        WhoGoesFirst::new(&mut rng),
        WhoGoesFirst::new(&mut rng),
    ];

    let is_ia_down_for_newtoken: bool = rng.gen();
    let mut person_to_room = data.person_to_room.lock().unwrap();
    let mut room_to_gamestate = data.room_to_gamestate.lock().unwrap();
    let mut bot_to_room = data.bot_to_room.lock().unwrap();
    let mut room_to_bot = data.room_to_bot.lock().unwrap();
    person_to_room.insert(
        new_token,
        RoomInfoWithPerspective {
            room_id,
            is_first_move_my_move: is_first_turn_newtoken_turn.clone(),
            is_IA_down_for_me: is_ia_down_for_newtoken,
        },
    );

    bot_to_room.insert(
        bot_token,
        RoomInfoWithPerspective {
            room_id,
            is_first_move_my_move: [
                is_first_turn_newtoken_turn[0].not(),
                is_first_turn_newtoken_turn[1].not(),
                is_first_turn_newtoken_turn[2].not(),
                is_first_turn_newtoken_turn[3].not(),
            ],
            is_IA_down_for_me: !is_ia_down_for_newtoken,
        },
    );
    room_to_bot.insert(room_id, bot_token);
    room_to_gamestate.insert(
        room_id,
        GameState {
            tam_itself_is_tam_hue: true,
            season: Season::Iei2,
            rate: Rate::X1,
            ia_owner_s_score: 20,
            is_ia_owner_s_turn: is_first_turn_newtoken_turn[0].result == is_ia_down_for_newtoken,
            f: Field {
                board: yhuap_initial_board(),
                a_side_hop1zuo1: vec![],
                ia_side_hop1zuo1: vec![],
            },
            waiting_for_after_half_acceptance: None,
            moves_to_be_polled: [vec![], vec![], vec![], vec![]],
        },
    );

    RetVsCpuEntry::LetTheGameBegin {
        access_token: format!("{}", new_token),
        is_first_move_my_move: is_first_turn_newtoken_turn[0].clone(),
        is_ia_down_for_me: is_ia_down_for_newtoken,
    }
}
