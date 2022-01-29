use crate::types::{Ciurl, GameState, MoveToBePolled , bot::*};
use cetkaik_full_state_transition::{Config, message::{AfterHalfAcceptance, NormalMove, InfAfterStep, PureMove}, state};
use cetkaik_core::*;
use rand::prelude::SliceRandom;


struct CerkeBot { 
    
}


pub enum BotMove {
    NormalMove(NormalMove),
    InfAfterStep { 
        dat: InfAfterStep, 
        after: [AfterHalfAcceptance;6]
    }
}


impl BotMove {
    fn from_strict_pure_move(pure_move: &PureMove) -> Self {
        match pure_move {
            PureMove::InfAfterStep(m) => {
                let mut after = [None; 6];
                for i in 1..6i32 {
                    after[i as usize] = if absolute::distance(m.src,m.planned_direction) <= i {
                        Some(m.planned_direction)
                    } else { 
                        None
                    }
                }
                BotMove::InfAfterStep {
                    dat: m.clone(),
                    after: after.map(|dest| AfterHalfAcceptance { dest })
                }
            },
            PureMove::NormalMove(m) => {
                BotMove::NormalMove(m.clone().into())
            },
        }
    }
}

impl From<BotMove> for MoveToBePolled {
    fn from(bot_move: BotMove) -> Self {
        match bot_move {
            BotMove::NormalMove(mov) => {
                match mov.into() {
                    crate::types::NormalMove::NonTamMove { data } => MoveToBePolled::NonTamMove { data},
                    crate::types::NormalMove::TamMove { flatten } => MoveToBePolled::TamMove {flatten},
                }
            },
            BotMove::InfAfterStep { dat, after } => {
                let InfAfterStep { src, step, planned_direction } = dat;
                let ciurl = Ciurl::new(&mut rand::thread_rng());
                let water_ciurl = Ciurl::new(&mut rand::thread_rng());

                let dest = after[ciurl.count()].dest;
                let final_result = match dest  {
                    Some(dest) => Some(crate::types::FinalResult {
                        dest,
                        water_entry_ciurl: todo!(),
                        thwarted_by_failing_water_entry_ciurl: todo!(),
                    }),
                    None => None,
                };

                MoveToBePolled::InfAfterStep{
                    src,
                    step,
                    coord_signifying_planned_direction: planned_direction,
                    stepping_ciurl: ciurl ,
                    final_result
                }                
            } ,
        }
    }
}

pub struct BotMoveWithTactics {
    pub tactics: TacticsKey,
    pub bot_move: BotMove
}

pub fn bot_random(game_state: &state::A, config: Config) -> BotMoveWithTactics {
    let mut rng = rand::thread_rng();
    let (hop1zuo1_candidates,candidates) = game_state.get_candidates(config);
    
    let pure_move = candidates.choose(&mut rng).unwrap();
    BotMoveWithTactics {
        tactics: TacticsKey::Neutral,
        bot_move: BotMove::from_strict_pure_move(pure_move)
    }
}


pub fn bot_move(game_state: &state::A, config: Config) -> BotMoveWithTactics { 
    bot_random(game_state, config)
}