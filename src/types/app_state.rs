
use cetkaik_full_state_transition::{Rate, Season};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::{env, sync::Mutex};
use crate::types::{
    AbsoluteCoord, AfterHalfAcceptanceMessage, MainMessage, MoveToBePolled, RetAfterHalfAcceptance,
    RetInfPoll, RetMainPoll, RetNormalMove, RetTaXot, RetTyMok, RetWhetherTyMokPoll,
    TamMoveInternal, WhoGoesFirst,
};
use uuid::Uuid;
use super::{AccessToken,RoomInfoWithPerspective,RoomId,GameState};

pub struct AppState {
    pub access_counter: Mutex<i32>,
    pub waiting_list: Mutex<HashSet<AccessToken>>,
    pub person_to_room: Mutex<HashMap<AccessToken, RoomInfoWithPerspective>>,
    pub rooms_where_opponent_is_bot: Mutex<HashSet<RoomId>>,
    pub room_to_gamestate: Mutex<HashMap<RoomId, GameState>>,
}

impl AppState {
    pub fn analyze_afterhalfacceptance_message_and_update(
        &self,
        message: AfterHalfAcceptanceMessage,
        room_info: &RoomInfoWithPerspective,
    ) -> RetAfterHalfAcceptance {
        todo!()
    }
    pub fn analyze_main_message_and_update(
        &self,
        message: MainMessage,
        room_info: &RoomInfoWithPerspective,
    ) -> RetNormalMove {
        let mut room_to_gamestate = self.room_to_gamestate.lock().unwrap();
        let mut game_state: &mut GameState = room_to_gamestate
            .get_mut(&room_info.room_id)
            .expect("FIXME: cannot happen");

        match message {
            MainMessage::TamMove {
                flatten:
                    TamMoveInternal::NoStep {
                        src,
                        first_dest,
                        second_dest,
                    },
            } => {
                let message = cetkaik_full_state_transition::message::NormalMove::TamMoveNoStep {
                    src,
                    first_dest,
                    second_dest,
                };
                todo!()
            }

            MainMessage::TamMove {
                flatten:
                    TamMoveInternal::StepsDuringFormer {
                        src,
                        step,
                        first_dest,
                        second_dest,
                    },
            } => {
                let message =
                    cetkaik_full_state_transition::message::NormalMove::TamMoveStepsDuringFormer {
                        src,
                        step,
                        first_dest,
                        second_dest,
                    };
                todo!()
            }

            MainMessage::TamMove {
                flatten:
                    TamMoveInternal::StepsDuringLatter {
                        src,
                        step,
                        first_dest,
                        second_dest,
                    },
            } => {
                let message =
                    cetkaik_full_state_transition::message::NormalMove::TamMoveStepsDuringLatter {
                        src,
                        step,
                        first_dest,
                        second_dest,
                    };
                todo!()
            }
            _ => todo!(),
        }

        todo!()
    }

    pub fn receive_tymok_and_update(&self, room_info: &RoomInfoWithPerspective) -> RetTyMok {
        let mut room_to_gamestate = self.room_to_gamestate.lock().unwrap();
        let mut game_state: &mut GameState = room_to_gamestate
            .get_mut(&room_info.room_id)
            .expect("FIXME: cannot happen");
        todo!()
    }

    pub fn receive_taxot_and_update(&self, room_info: &RoomInfoWithPerspective) -> RetTaXot {
        let mut room_to_gamestate = self.room_to_gamestate.lock().unwrap();
        let mut game_state: &mut GameState = room_to_gamestate
            .get_mut(&room_info.room_id)
            .expect("FIXME: cannot happen");
        todo!()
    }

    pub fn reply_to_whether_tymok_poll(
        &self,
        room_info: &RoomInfoWithPerspective,
    ) -> RetWhetherTyMokPoll {
        let mut room_to_gamestate = self.room_to_gamestate.lock().unwrap();
        let mut game_state: &mut GameState = room_to_gamestate
            .get_mut(&room_info.room_id)
            .expect("FIXME: cannot happen");
        todo!()
    }

    pub fn reply_to_main_poll(&self, room_info: &RoomInfoWithPerspective) -> RetMainPoll {
        let mut room_to_gamestate = self.room_to_gamestate.lock().unwrap();
        let mut game_state: &mut GameState = room_to_gamestate
            .get_mut(&room_info.room_id)
            .expect("FIXME: cannot happen");

        todo!()
    }

    pub fn reply_to_inf_poll(&self, room_info: &RoomInfoWithPerspective) -> RetInfPoll {
        let mut room_to_gamestate = self.room_to_gamestate.lock().unwrap();
        let mut game_state: &mut GameState = room_to_gamestate
            .get_mut(&room_info.room_id)
            .expect("FIXME: cannot happen");
        todo!()
    }
}