use std::fmt::Debug;

use rand::prelude::ThreadRng;
use serde::{Deserialize, Serialize};
use super::{AbsoluteCoord, Ciurl, NonTamMoveDotData, TamMoveInternal, bot::TacticsKey};

/* InfAfterStep | AfterHalfAcceptance | NormalMove*/
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum MainMessage {
    InfAfterStep {
        #[serde(flatten)]
        flatten: InfAfterStepInternal,
    },
    NonTamMove {
        data: NonTamMoveDotData,
    },
    TamMove {
        #[serde(flatten)]
        flatten: TamMoveInternal,
    },
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum AfterHalfAcceptanceMessage {
    AfterHalfAcceptance { dest: Option<AbsoluteCoord> },
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Copy, Clone)]
pub struct InfAfterStepInternal {
    src: AbsoluteCoord,
    step: AbsoluteCoord,

    #[serde(rename = "plannedDirection")]
    coord_signifying_planned_direction: AbsoluteCoord,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
pub struct WhoGoesFirst {
    pub result: bool,
    pub process: Vec<[Ciurl; 2]>,
}

impl WhoGoesFirst {
    pub fn new(rng: &mut ThreadRng) -> Self {
        let mut process: Vec<[Ciurl; 2]> = Vec::new();
        loop {
            let ciurl1 = Ciurl::new(rng);
            let ciurl2 = Ciurl::new(rng);
            process.push([ciurl1, ciurl2]);
            if ciurl1.count() > ciurl2.count() {
                return WhoGoesFirst {
                    process,
                    result: true,
                };
            }
            if ciurl1.count() < ciurl2.count() {
                return WhoGoesFirst {
                    process,
                    result: false,
                };
            }
        }
    }

    pub fn not(&self) -> Self {
        WhoGoesFirst {
            process: self.process.iter().map(|[a, b]| [*b, *a]).collect(),
            result: !self.result,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RetTyMok {
    Err,
    Ok,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RetTaXot {
    Err,
    Ok {
        is_first_move_my_move: Option<WhoGoesFirst>,
    },
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RetInfAfterStep {
    Ok { ciurl: Ciurl },
    Err { why_illegal: String },
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RetNormalMove {
    Err { why_illegal: String },
    WithWaterEntry { ciurl: Ciurl },
    WithoutWaterEntry,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RetAfterHalfAcceptance {
    Err { why_illegal: String },
    WithWaterEntry { ciurl: Ciurl },
    WithoutWaterEntry,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RetRandomEntry {
    InWaitingList {
        access_token: String,
    },

    #[serde(rename = "LetTheGameBegin")]
    RoomAlreadyAssigned {
        access_token: String,
        is_first_move_my_move: WhoGoesFirst,

        #[serde(rename = "is_IA_down_for_me")]
        is_ia_down_for_me: bool,
    },
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RetVsCpuEntry {
    LetTheGameBegin {
        access_token: String,
        is_first_move_my_move: WhoGoesFirst,

        #[serde(rename = "is_IA_down_for_me")]
        is_ia_down_for_me: bool,
    },
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RetRandomPoll {
    Err { why_illegal: String },
    Ok { ret: RetRandomEntry },
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RetRandomCancel {
    Err { why_illegal: String },
    Ok { cancellable: bool },
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RetWhetherTyMokPoll {
    TyMok,
    TaXot {
        is_first_move_my_move: Option<WhoGoesFirst>,
    },
    NotYetDetermined,
    Err {
        why_illegal: String,
    },
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RetMainPoll {
    MoveMade {
        content: MoveToBePolled,
        message: Option<TacticsKey>,
    },
    NotYetDetermined,
    Err {
        why_illegal: String,
    },
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RetInfPoll {
    MoveMade { content: MoveToBePolled },
    NotYetDetermined,
    Err { why_illegal: String },
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum MoveToBePolled {
    NonTamMove {
        data: NonTamMoveDotData,
    },
    TamMove {
        #[serde(flatten)]
        flatten: TamMoveInternal,
    },
    InfAfterStep {
        src: AbsoluteCoord,
        step: AbsoluteCoord,

        #[serde(rename = "plannedDirection")]
        coord_signifying_planned_direction: AbsoluteCoord,
        stepping_ciurl: Ciurl,

        #[serde(rename = "finalResult")]
        final_result: Option<FinalResult>,
    },
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct FinalResult {
    pub dest: AbsoluteCoord,
    pub water_entry_ciurl: Option<Ciurl>,
    pub thwarted_by_failing_water_entry_ciurl: Option<Ciurl>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]

pub struct MsgWithAccessToken {
    pub access_token: String,
}
