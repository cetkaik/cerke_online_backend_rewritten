use std::fmt::Debug;
use rand::prelude::ThreadRng;
use serde::{Deserialize, Serialize};
use super::{AbsoluteCoord, Ciurl, NonTamMoveDotData, TamMoveInternal, bot::TacticsKey};
use super::serde_coord;

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

impl MainMessage {
    #[must_use]
    pub fn process(self) -> MoveToBePolled {
        match self {
            MainMessage::InfAfterStep { flatten } => {
                let InfAfterStepInternal {src,step, coord_signifying_planned_direction} = flatten;
                MoveToBePolled::InfAfterStep {
                    src,
                    step,
                    coord_signifying_planned_direction,
                    stepping_ciurl: Ciurl::new(&mut rand::thread_rng()),
                    final_result: None,
                }
            },
            MainMessage::NonTamMove { data } => {
                MoveToBePolled::NonTamMove {
                    data
                }
            },
            MainMessage::TamMove { flatten } => {
                MoveToBePolled::TamMove {
                    flatten
                }
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub struct MainMessageStruct {
    pub message: MainMessage, 
}
impl From<MainMessage> for MainMessageStruct {
    fn from(message: MainMessage) -> Self {
        Self { 
            message
        }
    }
}

#[test]
fn test_ser_normalmove_nontam(){
    use cetkaik_core::absolute::{Row,Column};
    let mms :MainMessageStruct = MainMessage::NonTamMove { 
        data : NonTamMoveDotData::SrcDst { 
            src: cetkaik_core::absolute::Coord(Row::AI,Column::C),
            dest: cetkaik_core::absolute::Coord(Row::Y,Column::C),
            water_entry_ciurl: None 
        },
    }.into();
    let json_str = r#"{"message":{"type":"NonTamMove","data":{"type":"SrcDst","src":["AI","C"],"dest":["Y","C"]}}}"#;
    assert_eq!(json_str, serde_json::to_string(&mms).unwrap() );

}

#[test]
fn test_normalmove_nontam(){
    use cetkaik_core::absolute::{Row,Column};
    let json_str = r#"{"message":{"type":"NonTamMove","data":{"type":"SrcDst","src":["AI","C"],"dest":["O","C"]}}}"#;
    let result: MainMessageStruct = serde_json::from_str(json_str).unwrap();
    let MainMessageStruct{ message: result } = result;
    
    assert_eq!(result, MainMessage::NonTamMove { 
        data : NonTamMoveDotData::SrcDst { 
            src: cetkaik_core::absolute::Coord(Row::AI,Column::C),
            dest: cetkaik_core::absolute::Coord(Row::O,Column::C),
            water_entry_ciurl: None 
        },
    });

}


#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum AfterHalfAcceptanceMessage {
    AfterHalfAcceptance {
        #[serde(with="serde_coord::opt")]
        dest: Option<AbsoluteCoord>
    },
}


#[test]
fn test_after_half_null(){
    use cetkaik_core::absolute::{Row,Column};
    {
        let json_str = r#"{
            "message": {
                "type": "AfterHalfAcceptance",
                "dest": null
            }
        }"#;
        let result: AfterHalfAcceptanceMessageStruct = serde_json::from_str(json_str).unwrap();
        let AfterHalfAcceptanceMessageStruct{ message: result } = result;
        
        assert_eq!(result, AfterHalfAcceptanceMessage::AfterHalfAcceptance {  
            dest: None
        });
    }
    {            
        let json_str = r#"{
            "message": {
                "type": "AfterHalfAcceptance",
                "dest": [
                    "O",
                    "L"
                ]
            }
        }"#;
        let result: AfterHalfAcceptanceMessageStruct = serde_json::from_str(json_str).unwrap();
        let AfterHalfAcceptanceMessageStruct{ message: result } = result;
        
        assert_eq!(result, AfterHalfAcceptanceMessage::AfterHalfAcceptance {  
            dest: Some(cetkaik_core::absolute::Coord(Row::O,Column::L)),
        });
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub struct AfterHalfAcceptanceMessageStruct {
    pub message: AfterHalfAcceptanceMessage, 
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Copy, Clone)]
pub struct InfAfterStepInternal {
    #[serde(with="serde_coord")]
    pub src: AbsoluteCoord,
    #[serde(with="serde_coord")]
    pub step: AbsoluteCoord,

    #[serde(with="serde_coord")]
    #[serde(rename = "plannedDirection")]
    pub coord_signifying_planned_direction: AbsoluteCoord,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
pub struct WhoGoesFirst {
    pub result: bool,
    pub process: Vec<[Ciurl; 2]>,
}

impl WhoGoesFirst {
    #[must_use]
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

    #[must_use]
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

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
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
        #[serde(with="serde_coord")]
        src: AbsoluteCoord,
        #[serde(with="serde_coord")]
        step: AbsoluteCoord,

        #[serde(with="serde_coord")]
        #[serde(rename = "plannedDirection")]
        coord_signifying_planned_direction: AbsoluteCoord,
        stepping_ciurl: Ciurl,

        #[serde(rename = "finalResult")]
        final_result: Option<FinalResult>,
    },
}

impl From<NonTamMoveDotData> for MoveToBePolled {
    fn from(mov: NonTamMoveDotData) -> Self {
        Self::NonTamMove {
            data: mov
        }
    }
}
impl From<TamMoveInternal> for MoveToBePolled {
    fn from(mov: TamMoveInternal) -> Self {
        Self::TamMove {
            flatten: mov
        }
    }
}


#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
pub struct FinalResult {    
    #[serde(with="serde_coord")]
    pub dest: AbsoluteCoord,
    pub water_entry_ciurl: Option<Ciurl>,
    pub thwarted_by_failing_water_entry_ciurl: Option<Ciurl>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]

pub struct MsgWithAccessToken {
    pub access_token: String,
}
