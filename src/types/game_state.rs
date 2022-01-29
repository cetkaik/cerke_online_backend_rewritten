use std::sync::{Arc, Mutex};

use cetkaik_full_state_transition::{Config, message::{AfterHalfAcceptance, InfAfterStep, NormalMove, PureMove}, state::HandResolved};

use crate::types::FinalResult;

use super::{Ciurl, MovePiece, MoveToBePolled, NonTamMoveDotData, Phase, RetAfterHalfAcceptance, RetInfAfterStep, RetNormalMove, RetTaXot, SrcStep, TamMoveInternal, WhoGoesFirst};

#[derive(Debug)]
pub struct GameState {
    pub state: Phase,
    pub config: Config,
    pub waiting_for_after_half_acceptance: Option<SrcStep>,
    pub moves_to_be_polled: [Vec<MovePiece>; 4],
    pub is_first_move_ia_move: Arc<Mutex<[Option<WhoGoesFirst>; 4]>>,
}

impl GameState { 
    pub fn is_ia_owner_s_turn(&self) -> bool {
        self.state.whose_turn() == cetkaik_core::absolute::Side::IASide
    }

    pub fn get_last_move (&self) -> Option<&MovePiece> {
        self.moves_to_be_polled[self.state.get_season() as usize].last()
    }
    
    pub fn get_last_move_mut (&mut self) -> Option<&mut MovePiece> {
        self.moves_to_be_polled[self.state.get_season() as usize].last_mut()
    }
    
    pub fn commit_last_move (&mut self, move_piece: MovePiece ) {
        self.moves_to_be_polled[self.state.get_season() as usize].push(move_piece);
    }

    pub fn apply_normal_move(&mut self, mov: NormalMove) -> RetNormalMove {                
        if let Phase::Start(state) = &self.state {
            match mov {
                NormalMove::NonTamMoveSrcDst { src, dest } => {
                    let next_state = cetkaik_full_state_transition::apply_normal_move(state , mov, self.config);
                    match next_state {
                        Ok(next_state_p) => {
                            let (next_state, ciurl ) = next_state_p.choose();
                            let move_to_be_polled = MoveToBePolled::from(NonTamMoveDotData::SrcDst {
                                src, dest,
                                water_entry_ciurl: ciurl.map(Ciurl::from),
                            });
                            match ciurl {
                                Some(ciurl) => {
                                    self.commit_last_move(
                                        crate::types::MovePiece { mov: move_to_be_polled , status: None, by_ia_owner: self.is_ia_owner_s_turn() }
                                    );                          
                                    self.state = Phase::Moved(next_state);
                                    RetNormalMove::WithWaterEntry {
                                        ciurl: Ciurl::from(ciurl)
                                    }
                                },
                                None => {
                                    self.commit_last_move(
                                        crate::types::MovePiece { mov: move_to_be_polled , status: None, by_ia_owner: self.is_ia_owner_s_turn() }
                                    );
                                    self.state = Phase::Moved(next_state);
                                    RetNormalMove::WithoutWaterEntry
                                }
                            }
                        },
                        Err(e) => RetNormalMove::Err { 
                            why_illegal: e.to_string()
                        }
                    }
                },
                NormalMove::NonTamMoveSrcStepDstFinite { src, step, dest } => {
                    let next_state = cetkaik_full_state_transition::apply_normal_move(state , mov, self.config);
                    match next_state {
                        Ok(next_state_p) => {
                            let (next_state, ciurl ) = next_state_p.choose();
                            let move_to_be_polled = MoveToBePolled::from(NonTamMoveDotData::SrcStepDstFinite {
                                src, step, dest,
                                water_entry_ciurl: ciurl.map(Ciurl::from),
                            });
                            match ciurl {
                                Some(ciurl) => {
                                    self.commit_last_move(
                                        crate::types::MovePiece { mov: move_to_be_polled , status: None, by_ia_owner: self.is_ia_owner_s_turn() }
                                    );
                                    self.state = Phase::Moved(next_state);
                                    RetNormalMove::WithWaterEntry {
                                        ciurl: Ciurl::from(ciurl)
                                    }
                                },
                                None => { 
                                    self.commit_last_move(
                                        crate::types::MovePiece { mov: move_to_be_polled , status: None, by_ia_owner: self.is_ia_owner_s_turn() }
                                    );
                                    self.state = Phase::Moved(next_state);
                                    RetNormalMove::WithoutWaterEntry
                                }
                            }
                        },
                        Err(e) => RetNormalMove::Err { 
                            why_illegal: e.to_string()
                        }
                    }
                },
                NormalMove::NonTamMoveFromHopZuo { color, prof, dest } => {
                    let next_state = cetkaik_full_state_transition::apply_normal_move(state , mov, self.config);
                    let move_to_be_polled = MoveToBePolled::from(NonTamMoveDotData::FromHand {
                        color: color.into(), 
                        profession: prof.into(),
                        dest
                    });
                    match next_state {
                        Ok(next_state_p) => {    
                            self.commit_last_move(
                                crate::types::MovePiece { mov: move_to_be_polled , status: None, by_ia_owner: self.is_ia_owner_s_turn() }
                            );                 
                            self.state = Phase::Moved(next_state_p.choose_when_no_ciurl());
                            RetNormalMove::WithoutWaterEntry 
                        },
                        Err(e) => RetNormalMove::Err { 
                            why_illegal: e.to_string()
                        }
                    }
                },
                NormalMove::TamMoveNoStep { src, first_dest, second_dest } => {                
                    let next_state = cetkaik_full_state_transition::apply_normal_move(state , mov, self.config);
                    let move_to_be_polled = MoveToBePolled::from(TamMoveInternal::NoStep {
                        src,
                        first_dest,
                        second_dest,
                    });
                    match next_state {
                        Ok(next_state_p) => {    
                            self.commit_last_move(
                                crate::types::MovePiece { mov: move_to_be_polled , status: None, by_ia_owner: self.is_ia_owner_s_turn() }
                            );                 
                            self.state = Phase::Moved(next_state_p.choose_when_no_ciurl());
                            RetNormalMove::WithoutWaterEntry 
                        },
                        Err(e) => RetNormalMove::Err { 
                            why_illegal: e.to_string()
                        }
                    } 
                },
                NormalMove::TamMoveStepsDuringFormer { src, step, first_dest, second_dest } => {
                    let next_state = cetkaik_full_state_transition::apply_normal_move(state , mov, self.config);
                    let move_to_be_polled = MoveToBePolled::from(TamMoveInternal::StepsDuringFormer {
                        src,
                        step,
                        first_dest,
                        second_dest,
                    });
                    match next_state {
                        Ok(next_state_p) => {    
                            self.commit_last_move(
                                crate::types::MovePiece { mov: move_to_be_polled , status: None, by_ia_owner: self.is_ia_owner_s_turn() }
                            );                 
                            self.state = Phase::Moved(next_state_p.choose_when_no_ciurl());
                            RetNormalMove::WithoutWaterEntry 
                        },
                        Err(e) => RetNormalMove::Err { 
                            why_illegal: e.to_string()
                        }
                    }
                },
                NormalMove::TamMoveStepsDuringLatter { src, step, first_dest, second_dest } => {
                    let next_state = cetkaik_full_state_transition::apply_normal_move(state , mov, self.config);
                    let move_to_be_polled = MoveToBePolled::from(TamMoveInternal::StepsDuringLatter {
                        src,
                        step,
                        first_dest,
                        second_dest,
                    });
                    match next_state {
                        Ok(next_state_p) => {    
                            self.commit_last_move(
                                crate::types::MovePiece { mov: move_to_be_polled , status: None, by_ia_owner: self.is_ia_owner_s_turn() }
                            );                 
                            self.state = Phase::Moved(next_state_p.choose_when_no_ciurl());
                            RetNormalMove::WithoutWaterEntry 
                        },
                        Err(e) => RetNormalMove::Err { 
                            why_illegal: e.to_string()
                        }
                    }
                },
            }
        } else {
            RetNormalMove::Err { 
                why_illegal: "Invalid State".to_string()
            }
        }
    }

    pub fn apply_inf_after_step (&mut self, mov: InfAfterStep) -> RetInfAfterStep {      
        if let Phase::Start(state) = &self.state {
            let InfAfterStep {src,step,planned_direction} = mov;
            let next_state = cetkaik_full_state_transition::apply_inf_after_step(state , mov, self.config);
            match next_state {
                Ok(next_state_p) => {
                    let (next_state, ciurl_value ) = next_state_p.choose();
                    let ciurl_value = ciurl_value.unwrap();
                    let ciurl = Ciurl::from(ciurl_value);
                    let move_to_be_polled = MoveToBePolled::InfAfterStep {
                        src,
                        step,
                        coord_signifying_planned_direction: planned_direction,
                        stepping_ciurl: ciurl,
                        final_result: None,
                    };
                    self.commit_last_move(
                        crate::types::MovePiece { mov: move_to_be_polled , status: None, by_ia_owner: self.is_ia_owner_s_turn() }
                    );
                    self.state = Phase::AfterCiurl(next_state);
                    RetInfAfterStep::Ok {
                        ciurl
                    }
                },
                Err(e) => RetInfAfterStep::Err { 
                    why_illegal: e.to_string()
                }
            }
        } else {
            RetInfAfterStep::Err { 
                why_illegal: "Invalid State".to_string()
            }
        }
    }

    pub fn apply_pure_move(&mut self, mov: PureMove) {
        match mov {
            PureMove::InfAfterStep(mov) => {
                self.apply_inf_after_step(mov);
            },
            PureMove::NormalMove(mov) => {
                self.apply_normal_move(mov);
            },
        }
    }

    pub fn apply_after_half_acceptance(&mut self, mov: AfterHalfAcceptance) -> RetAfterHalfAcceptance { 
        let AfterHalfAcceptance {dest} = mov;                     
        if let Phase::AfterCiurl(state) = &self.state {
            let next_state = cetkaik_full_state_transition::apply_after_half_acceptance(state , mov, self.config);
            match next_state {
                Ok(next_state_p) => {
                    let (next_state, ciurl_value ) = next_state_p.choose();
                    let ciurl = ciurl_value.map(Ciurl::from);
                    let move_to_be_polled = self.get_last_move_mut().unwrap();
                    match &mut move_to_be_polled.mov {
                        MoveToBePolled::InfAfterStep { src, step, coord_signifying_planned_direction, stepping_ciurl, final_result } => {
                            if let Some(result) = dest.map(|dest_coord| FinalResult { 
                                dest: dest_coord,
                                water_entry_ciurl: ciurl.clone(),
                                thwarted_by_failing_water_entry_ciurl: ciurl.clone()
                            }) {
                                final_result.replace(result);
                            }
                        },
                        _ => {
                            unreachable!("Invalid MoveToBePolled");
                        }
                    };
                    self.state = Phase::Moved(next_state);
                    match ciurl {
                        Some(ciurl) => RetAfterHalfAcceptance::WithWaterEntry {ciurl},
                        None => RetAfterHalfAcceptance::WithoutWaterEntry,
                    }
                },
                Err(e) => RetAfterHalfAcceptance::Err { 
                    why_illegal: e.to_string()
                },
            }
        }else {
            RetAfterHalfAcceptance::Err { 
                why_illegal: "Invalid State".to_string()
            }
        }

    }

    pub fn apply_tymok (&mut self) {
        
    }
    pub fn apply_taxot (&mut self) -> RetTaXot {
        if let Phase::Moved(state) = &self.state { 
            let state_resolved = cetkaik_full_state_transition::resolve(&state, self.config);
            if let HandResolved::HandExists { if_taxot, if_tymok }  = state_resolved {
                self.state = match if_taxot {
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
                if whos_go_first.result != self.is_ia_owner_s_turn() {
                    whos_go_first = whos_go_first.not();
                }
                *(self.is_first_move_ia_move.lock().unwrap().get_mut(self.state.get_season().to_index()).unwrap()) = Some(whos_go_first.clone());

                RetTaXot::Ok { 
                    is_first_move_my_move: Some(whos_go_first)
                }
            } else { 
                RetTaXot::Err
            } 
        }else {
            RetTaXot::Err
        }
    }

    pub fn apply_resolve (&mut self){
        if let Phase::Moved(state) = &self.state { 
            let resolved = cetkaik_full_state_transition::resolve(state, self.config);
            match resolved {
                HandResolved::NeitherTymokNorTaxot(next_state) => {
                    self.state = Phase::Start(next_state);
                },
                HandResolved::HandExists { if_tymok, if_taxot } => {},
                HandResolved::GameEndsWithoutTymokTaxot(_) => {},
            }
        }
    }
    
    pub fn is_first_move_my_move(&self, is_ia_down_for_me:bool , season: usize) -> WhoGoesFirst {
        let is_first_move_ia_move = self.is_first_move_ia_move.lock().unwrap().get_mut(season).unwrap().clone().unwrap();
        if is_ia_down_for_me {
            is_first_move_ia_move
        } else {
            is_first_move_ia_move.not()
        }
    }

    pub fn set_first_mover(&mut self, season: usize, is_ia: bool, rng: &mut rand::prelude::ThreadRng) {
        let mut whos_go_first = WhoGoesFirst::new(rng);
        if whos_go_first.result != is_ia {
            whos_go_first = whos_go_first.not();
        }
        *(self.is_first_move_ia_move.lock().unwrap().get_mut(season).unwrap()) = Some(whos_go_first);
    }

}
