


use std::collections::{HashMap, HashSet};
use std::{sync::Mutex};
use cetkaik_full_state_transition::message::{AfterHalfAcceptance, InfAfterStep};
use cetkaik_full_state_transition::state::HandResolved;

use crate::types::{AfterHalfAcceptanceMessage, Ciurl, InfAfterStepInternal, MainMessage, NonTamMoveDotData, RetAfterHalfAcceptance, RetInfPoll, RetMainPoll, RetNormalMove, RetTaXot, RetTyMok, RetWhetherTyMokPoll, TamMoveInternal};

use super::{AccessToken, GameState, Phase, RetInfAfterStep, RoomId, RoomInfoWithPerspective, WhoGoesFirst};

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
        let mut room_to_gamestate = self.room_to_gamestate.lock().unwrap();
        let game_state: &mut GameState = room_to_gamestate
            .get_mut(&room_info.room_id)
            .expect("FIXME: cannot happen");
        println!("{:#?}", game_state.state.whose_turn());
        println!("{:#?}", game_state.state.phase_name());

        let res = match message {
            AfterHalfAcceptanceMessage::AfterHalfAcceptance { dest } => {
                game_state.apply_after_half_acceptance(AfterHalfAcceptance { dest })
            },
        };
        game_state.apply_resolve();
        res
    }
    pub fn analyze_main_message_and_update(
        &self,
        message: MainMessage,
        room_info: &RoomInfoWithPerspective,
    ) -> RetNormalMove {
        let mut room_to_gamestate = self.room_to_gamestate.lock().unwrap();
        let game_state: &mut GameState = room_to_gamestate
            .get_mut(&room_info.room_id)
            .expect("FIXME: cannot happen");
        println!("{:#?}", game_state.state.whose_turn());
        println!("{:#?}", game_state.state.phase_name());

        let res = match message {
            MainMessage::TamMove {
                flatten:
                    TamMoveInternal::NoStep {
                        src,
                        first_dest,
                        second_dest,
                    },
            } => {
                let mov = cetkaik_full_state_transition::message::NormalMove::TamMoveNoStep {
                    src,
                    first_dest,
                    second_dest,
                }; 
                game_state.apply_normal_move(mov)
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
                let mov =
                    cetkaik_full_state_transition::message::NormalMove::TamMoveStepsDuringFormer {
                        src,
                        step,
                        first_dest,
                        second_dest,
                    }; 
                game_state.apply_normal_move(mov)
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
                let mov =
                    cetkaik_full_state_transition::message::NormalMove::TamMoveStepsDuringLatter {
                        src,
                        step,
                        first_dest,
                        second_dest,
                    }; 
                game_state.apply_normal_move(mov)
            },
            MainMessage::NonTamMove {
                data: NonTamMoveDotData::FromHand {                    
                    color,
                    profession,
                    dest,
                }
            } => {
                let mov = cetkaik_full_state_transition::message::NormalMove::NonTamMoveFromHopZuo {                
                    color: color.into(),
                    prof: profession.into(),
                    dest                    
                }; 
                game_state.apply_normal_move(mov)
            },
            MainMessage::NonTamMove {
                data: NonTamMoveDotData::SrcDst {
                    src,
                    dest,
                    water_entry_ciurl,
                }
            } => {
                let mov = cetkaik_full_state_transition::message::NormalMove::NonTamMoveSrcDst {                
                    src,
                    dest
                }; 
                game_state.apply_normal_move(mov)
            },
            MainMessage::NonTamMove {
                data: NonTamMoveDotData::SrcStepDstFinite {
                    src,
                    step,
                    dest,
                    water_entry_ciurl
                }
            } => {
                let mov = cetkaik_full_state_transition::message::NormalMove::NonTamMoveSrcStepDstFinite {                
                    src,
                    step,
                    dest
                };
                game_state.apply_normal_move(mov)
            },
            _ => todo!(),
        };
        game_state.apply_resolve();
        res
    }
    pub fn analyze_inf_after_step_and_update(
        &self,
        message: MainMessage,
        room_info: &RoomInfoWithPerspective,
    ) -> RetInfAfterStep {
        let mut room_to_gamestate = self.room_to_gamestate.lock().unwrap();
        let game_state: &mut GameState = room_to_gamestate
            .get_mut(&room_info.room_id)
            .expect("FIXME: cannot happen");

        println!("{:#?}", game_state.state.whose_turn());
        println!("{:#?}", game_state.state.phase_name());
        
        let res  = match message {
            MainMessage::InfAfterStep {
                flatten: InfAfterStepInternal { src, step, coord_signifying_planned_direction }
            } => {

                let mov = InfAfterStep {
                    src,
                    step,
                    planned_direction: coord_signifying_planned_direction,
                }; 
                game_state.apply_inf_after_step(mov)
            },
            _ => todo!(),
        };
        game_state.apply_resolve();
        res
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
        let game_state: &mut GameState = room_to_gamestate
            .get_mut(&room_info.room_id)
            .expect("FIXME: cannot happen");

        let mov = game_state.get_last_move();
        if let Some(mov) = mov {
            match mov.status {
                Some(crate::types::HandCompletionStatus::TaXot) => RetWhetherTyMokPoll::TaXot {
                    is_first_move_my_move: Some(
                        game_state.is_first_move_my_move(room_info.is_ia_down_for_me, game_state.state.get_season().to_index())
                    )
                },
                Some(crate::types::HandCompletionStatus::TyMok) => RetWhetherTyMokPoll::TyMok,
                Some(crate::types::HandCompletionStatus::NotYetDetermined) => RetWhetherTyMokPoll::NotYetDetermined,
                None => RetWhetherTyMokPoll::NotYetDetermined,
            }
        } else {
            RetWhetherTyMokPoll::NotYetDetermined
        }
    }

    pub fn reply_to_main_poll(&self, room_info: &RoomInfoWithPerspective) -> RetMainPoll {
        use crate::bot::bot::BotMove;
        let mut room_to_gamestate = self.room_to_gamestate.lock().unwrap();
        let game_state: &mut GameState = room_to_gamestate
            .get_mut(&room_info.room_id)
            .expect("FIXME: cannot happen");

        println!("{:#?}", game_state.state.whose_turn());
        println!("{:#?}", game_state.state.phase_name());

        let is_bot = self.rooms_where_opponent_is_bot.lock().unwrap().contains(&room_info.room_id);
        
        let mov = game_state.get_last_move();
        // If the last move is not played by the player, just return what we have.
        if let Some(mov) = mov {
            if room_info.is_ia_down_for_me != mov.by_ia_owner {
                return RetMainPoll::MoveMade {
                    content: mov.mov.clone(),
                    message: None,
                }
            }
        }

        if is_bot {
            match &game_state.state {
                Phase::Start(_) => (),
                Phase::BeforeCiurl(_) => (),
                Phase::AfterCiurl(_) => (),
                Phase::Moved(_) => (),
            };

            if let Phase::Start(state) = &game_state.state {
                println!("{:#?}", game_state.state.whose_turn());
                let bot = crate::bot::bot_move(state, game_state.config);

                match bot.bot_move {
                    BotMove::NormalMove(mov) => {
                        game_state.apply_normal_move(mov);
                    },
                    BotMove::InfAfterStep { dat, after } => {
                        if let RetInfAfterStep::Ok {ciurl} = game_state.apply_inf_after_step(dat) {
                            let ret_after_half = game_state.apply_after_half_acceptance(after[ciurl.count()]);
                            match ret_after_half {
                                RetAfterHalfAcceptance::Err { why_illegal } => todo!(),
                                RetAfterHalfAcceptance::WithWaterEntry { ciurl } => {},
                                RetAfterHalfAcceptance::WithoutWaterEntry => {},
                            }                            
                        } else {
                            todo!()
                        }
                    },
                }
                
                match &game_state.state {
                    Phase::Start(_) => todo!(),
                    Phase::BeforeCiurl(_) => todo!(),
                    Phase::AfterCiurl(_) => todo!(),
                    Phase::Moved(state) => {
                        if state.tam2tysak2_will_trigger_taxottymok {
                            game_state.apply_taxot();
                        } else {
                            game_state.apply_resolve();
                        }
                    },
                }
                println!("{:#?}", game_state.state.whose_turn());

                RetMainPoll::MoveMade {
                    content: game_state.get_last_move().unwrap().mov.clone(),
                    message: Some(bot.tactics),
                } 
            } else { 
                todo!()
            }
            
        } else {
            println!("nonbot");
            RetMainPoll::NotYetDetermined
        }
    }

    pub fn reply_to_inf_poll(&self, room_info: &RoomInfoWithPerspective) -> RetInfPoll {
        use super::MoveToBePolled;

        let mut room_to_gamestate = self.room_to_gamestate.lock().unwrap();
        let game_state: &mut GameState = room_to_gamestate
            .get_mut(&room_info.room_id)
            .expect("FIXME: cannot happen");


        let last_move = game_state.get_last_move();
        
        if let Some(last_move) = last_move {
            if room_info.is_ia_down_for_me == last_move.by_ia_owner {
                RetInfPoll::Err {
                    why_illegal: "it's not your turn".to_string()
                }
            } else {
                match &last_move.mov {
                    MoveToBePolled::InfAfterStep { src, step, coord_signifying_planned_direction, stepping_ciurl, final_result } => {
                        match final_result {
                            Some(_) => RetInfPoll::MoveMade { content: last_move.mov.clone() },
                            None => RetInfPoll::NotYetDetermined,
                        }
                    },
                    _ => {
                        RetInfPoll::Err {
                            why_illegal: "InfAfterStep is not happening".to_string()
                        }
                    }
                }
            }
        } else {
            RetInfPoll::Err {
                why_illegal: "there is no last move".to_string()
            }
        }

    }
}