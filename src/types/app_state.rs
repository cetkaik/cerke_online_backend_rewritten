


use std::collections::{HashMap, HashSet};
use std::{sync::Mutex};
use cetkaik_full_state_transition::state::HandResolved;

use crate::types::{
    AfterHalfAcceptanceMessage, MainMessage, RetAfterHalfAcceptance,
    RetInfPoll, RetMainPoll, RetNormalMove, RetTaXot, RetTyMok, RetWhetherTyMokPoll,
    TamMoveInternal,
};

use super::{AccessToken, GameState, Phase, RoomId, RoomInfoWithPerspective, WhoGoesFirst};

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
        _message: AfterHalfAcceptanceMessage,
        _room_info: &RoomInfoWithPerspective,
    ) -> RetAfterHalfAcceptance {
        todo!()
    }
    pub fn analyze_main_message_and_update(
        &self,
        message: MainMessage,
        room_info: &RoomInfoWithPerspective,
    ) -> RetNormalMove {
        let mut room_to_gamestate = self.room_to_gamestate.lock().unwrap();
        let _game_state: &mut GameState = room_to_gamestate
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
                let _message = cetkaik_full_state_transition::message::NormalMove::TamMoveNoStep {
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
                let _message =
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
                let _message =
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
        let game_state: &mut GameState = room_to_gamestate
            .get_mut(&room_info.room_id)
            .expect("FIXME: cannot happen");
        
        let ia_side = room_info.is_ia_down_for_me;
        if ia_side != game_state.is_ia_owner_s_turn() { 
            return RetTyMok::Err;
        }
        
        if let Phase::Moved(state) = &game_state.state {
            let state_resolved = cetkaik_full_state_transition::resolve(&state, game_state.config);
            if let HandResolved::HandExists { if_taxot, if_tymok } = state_resolved {
                game_state.state = Phase::Start(if_tymok);
                RetTyMok::Ok
            } else { 
                RetTyMok::Err
            }
        } else {
            RetTyMok::Err
        }
    }

    pub fn receive_taxot_and_update(&self, room_info: &RoomInfoWithPerspective) -> RetTaXot {
        let mut room_to_gamestate = self.room_to_gamestate.lock().unwrap();
        let game_state: &mut GameState = room_to_gamestate
            .get_mut(&room_info.room_id)
            .expect("FIXME: cannot happen");

        let ia_side = room_info.is_ia_down_for_me;
        if ia_side != game_state.is_ia_owner_s_turn() { 
            return RetTaXot::Err;
        }

        if let Phase::Moved(state) = &game_state.state {
            let state_resolved = cetkaik_full_state_transition::resolve(&state, game_state.config);
            if let HandResolved::HandExists { if_taxot, if_tymok }  = state_resolved {
                game_state.state = match if_taxot {
                    cetkaik_full_state_transition::IfTaxot::NextSeason(p_state) => {
                        Phase::Start(p_state.choose().0)
                    },
                    cetkaik_full_state_transition::IfTaxot::VictoriousSide(_) => {
                        return RetTaXot::Ok { 
                            is_first_move_my_move: None
                        }
                    },
                };
                
                let mut whos_go_first = WhoGoesFirst::new(&mut rand::thread_rng());
                if whos_go_first.result != game_state.is_ia_owner_s_turn() {
                    whos_go_first = whos_go_first.not();
                }

                RetTaXot::Ok { 
                    is_first_move_my_move: Some(whos_go_first)
                }
            } else { 
                RetTaXot::Err
            }
        } else {
            RetTaXot::Err
        }
    }

    pub fn reply_to_whether_tymok_poll(
        &self,
        room_info: &RoomInfoWithPerspective,
    ) -> RetWhetherTyMokPoll {
        let mut room_to_gamestate = self.room_to_gamestate.lock().unwrap();
        let _game_state: &mut GameState = room_to_gamestate
            .get_mut(&room_info.room_id)
            .expect("FIXME: cannot happen");
        todo!()
    }

    pub fn reply_to_main_poll(&self, room_info: &RoomInfoWithPerspective) -> RetMainPoll {
        let mut room_to_gamestate = self.room_to_gamestate.lock().unwrap();
        let _game_state: &mut GameState = room_to_gamestate
            .get_mut(&room_info.room_id)
            .expect("FIXME: cannot happen");

        todo!()
    }

    pub fn reply_to_inf_poll(&self, room_info: &RoomInfoWithPerspective) -> RetInfPoll {
        let mut room_to_gamestate = self.room_to_gamestate.lock().unwrap();
        let _game_state: &mut GameState = room_to_gamestate
            .get_mut(&room_info.room_id)
            .expect("FIXME: cannot happen");
        todo!()
    }
}