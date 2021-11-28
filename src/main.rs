mod types;

use crate::types::{RetTaXot, WhoGoesFirst};
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use cetkaik_core::absolute::*;
use cetkaik_full_state_transition::{Rate, Season};
use serde::Serializer;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::{env, sync::Mutex};
use types::{AbsoluteCoord, MoveToBePolled, RetRandomEntry, RetTyMok};

#[derive(Deserialize)]
struct Info {
    username: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct AccessToken(Uuid);

impl std::fmt::Display for AccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct BotToken(Uuid);

impl std::fmt::Display for BotToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct AppStateWithCounter {
    counter: Mutex<i32>,
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
        waiting_list: Mutex::new(HashSet::new()),
        person_to_room: Mutex::new(HashMap::new()),
        bot_to_room: Mutex::new(HashMap::new()),
        room_to_bot: Mutex::new(HashMap::new()),
        room_to_gamestate: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
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
async fn random_entry(data: web::Data<AppStateWithCounter>) -> impl Responder {
    let ret_random_entry = random_entry_(false, data);
    HttpResponse::Ok().json(ret_random_entry)
}
use uuid::Uuid;

type RoomId = Uuid;

fn open_a_room(_token: AccessToken, _new_token: AccessToken, _is_staging: bool) -> RoomId {
    Uuid::new_v4()
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

fn random_entry_(is_staging: bool, data: web::Data<AppStateWithCounter>) -> RetRandomEntry {
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
            is_first_move_my_move: is_first_turn_newtoken_turn[0 /* spring */].result, // FIXME: also notify the result
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

#[post("/random/cancel")]
async fn random_cancel(info: web::Json<Info>) -> impl Responder {
    println!("Welcome {}!", info.username);
    HttpResponse::Ok().body(format!("Welcome {}!", info.username))
}

#[post("/random/entry/staging")]
async fn random_entry_staging(
    data: web::Data<AppStateWithCounter>,
) -> impl Responder {
    let ret_random_entry = random_entry_(true, data);
    HttpResponse::Ok().json(ret_random_entry)
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

fn constant_let_the_game_begin<S>(_: &(), s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str("let_the_game_begin")
}

#[derive(Serialize)]
struct RetVsCpuEntry {
    #[serde(serialize_with = "constant_let_the_game_begin")]
    state: (),
    access_token: String,
    is_first_move_my_move: bool,

    #[serde(rename(serialize = "is_IA_down_for_me"))]
    is_ia_down_for_me: bool,
}

#[post("/vs_cpu/entry")]
async fn vs_cpu_entry(info: web::Json<Info>) -> impl Responder {
    println!("Welcome {}!", info.username);
    HttpResponse::Ok().json(RetVsCpuEntry {
        state: (),
        access_token: "foo".to_owned(),
        is_first_move_my_move: true,
        is_ia_down_for_me: true,
    })
}
