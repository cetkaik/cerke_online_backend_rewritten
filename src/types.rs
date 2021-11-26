use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct AbsoluteCoord(AbsoluteRow, AbsoluteColumn);

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(into = "&'static str")]
#[serde(try_from = "&str")]
pub enum AbsoluteColumn {
    K,
    L,
    N,
    T,
    Z,
    X,
    C,
    M,
    P,
}

impl TryFrom<&str> for AbsoluteColumn {
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "K" => Ok(AbsoluteColumn::K),
            "L" => Ok(AbsoluteColumn::L),
            "N" => Ok(AbsoluteColumn::N),
            "T" => Ok(AbsoluteColumn::T),
            "Z" => Ok(AbsoluteColumn::Z),
            "X" => Ok(AbsoluteColumn::X),
            "C" => Ok(AbsoluteColumn::C),
            "M" => Ok(AbsoluteColumn::M),
            "P" => Ok(AbsoluteColumn::P),
            s => Err(format!("invalid column name `{}`", s)),
        }
    }

    type Error = String;
}

impl From<AbsoluteColumn> for &'static str {
    fn from(a: AbsoluteColumn) -> &'static str {
        match a {
            AbsoluteColumn::K => "K",
            AbsoluteColumn::L => "L",
            AbsoluteColumn::N => "N",
            AbsoluteColumn::T => "T",
            AbsoluteColumn::Z => "Z",
            AbsoluteColumn::X => "X",
            AbsoluteColumn::C => "C",
            AbsoluteColumn::M => "M",
            AbsoluteColumn::P => "P",
        }
    }
}

impl From<AbsoluteRow> for &'static str {
    fn from(a: AbsoluteRow) -> &'static str {
        match a {
            AbsoluteRow::A => "A",
            AbsoluteRow::E => "E",
            AbsoluteRow::I => "I",
            AbsoluteRow::U => "U",
            AbsoluteRow::O => "O",
            AbsoluteRow::Y => "Y",
            AbsoluteRow::AI => "AI",
            AbsoluteRow::AU => "AU",
            AbsoluteRow::IA => "IA",
        }
    }
}

impl TryFrom<&str> for AbsoluteRow {
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" => Ok(AbsoluteRow::A),
            "E" => Ok(AbsoluteRow::E),
            "I" => Ok(AbsoluteRow::I),
            "U" => Ok(AbsoluteRow::U),
            "O" => Ok(AbsoluteRow::O),
            "Y" => Ok(AbsoluteRow::Y),
            "AI" => Ok(AbsoluteRow::AI),
            "AU" => Ok(AbsoluteRow::AU),
            "IA" => Ok(AbsoluteRow::IA),
            s => Err(format!("invalid column name `{}`", s)),
        }
    }

    type Error = String;
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(into = "&'static str")]
#[serde(try_from = "&str")]
pub enum AbsoluteRow {
    A,
    E,
    I,
    U,
    O,
    Y,
    AI,
    AU,
    IA,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Color {
    Kok1 = 0,
    Huok2 = 1,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Profession {
    Nuak1 = 0,
    Kauk2 = 1,
    Gua2 = 2,
    Kaun1 = 3,
    Dau2 = 4,
    Maun1 = 5,
    Kua2 = 6,
    Tuk2 = 7,
    Uai1 = 8,
    Io = 9,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Ciurl(bool, bool, bool, bool, bool);

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum NormalMove {
    NonTamMove {
        data: NonTamMoveDotData,
    },
    TamMove {
        #[serde(flatten)]
        flatten: TamMoveInternal,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]

pub enum NonTamMoveDotData {
    FromHand {
        color: Color,
        profession: Profession,
        dest: AbsoluteCoord,
    },
    SrcDst {
        src: AbsoluteCoord,
        dest: AbsoluteCoord,
    },
    SrcStepDstFinite {
        src: AbsoluteCoord,
        step: AbsoluteCoord,
        dest: AbsoluteCoord,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "stepStyle")]
pub enum TamMoveInternal {
    NoStep {
        src: AbsoluteCoord,

        #[serde(rename = "firstDest")]
        first_dest: AbsoluteCoord,

        #[serde(rename = "secondDest")]
        second_dest: AbsoluteCoord,
    },

    StepsDuringFormer {
        src: AbsoluteCoord,
        step: AbsoluteCoord,

        #[serde(rename = "firstDest")]
        first_dest: AbsoluteCoord,

        #[serde(rename = "secondDest")]
        second_dest: AbsoluteCoord,
    },

    StepsDuringLatter {
        src: AbsoluteCoord,
        step: AbsoluteCoord,

        #[serde(rename = "firstDest")]
        first_dest: AbsoluteCoord,

        #[serde(rename = "secondDest")]
        second_dest: AbsoluteCoord,
    },
}

/* InfAfterStep | AfterHalfAcceptance | NormalMove*/
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Message {
    InfAfterStep {
        #[serde(flatten)]
        flatten: InfAfterStepInternal,
    },
    AfterHalfAcceptance {
        dest: Option<AbsoluteCoord>,
    },
    NonTamMove {
        data: NonTamMoveDotData,
    },
    TamMove {
        #[serde(flatten)]
        flatten: TamMoveInternal,
    },
}

#[derive(Serialize, Deserialize)]
struct InfAfterStepInternal {
    src: AbsoluteCoord,
    step: AbsoluteCoord,

    #[serde(rename = "plannedDirection")]
    coord_signifying_planned_direction: AbsoluteCoord,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
enum RetInfAfterStep {
    Ok {
        ciurl: Ciurl,
    },
    Err {
        #[serde(rename = "whyIllegal")]
        why_illegal: String,
    },
}

use serde::ser::{SerializeStruct, Serializer};
impl Serialize for RetInfAfterStep {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            RetInfAfterStep::Ok { ciurl } => {
                let mut state = serializer.serialize_struct("RetInfAfterStep", 2)?;
                state.serialize_field("legal", &true)?;
                state.serialize_field("ciurl", &ciurl)?;
                state.end()
            }
            RetInfAfterStep::Err { why_illegal } => {
                let mut state = serializer.serialize_struct("RetInfAfterStep", 2)?;
                state.serialize_field("legal", &false)?;
                state.serialize_field("whyIllegal", &why_illegal)?;
                state.end()
            }
        }
    }
}

#[test]
fn test_deserialize_ret_inf_after_step_err() {
    let deserialized: RetInfAfterStep = serde_json::from_str(
        r#"{
        "legal": false,
        "whyIllegal": "something went wrong"
    }"#,
    )
    .unwrap();
    assert_eq!(
        deserialized,
        RetInfAfterStep::Err {
            why_illegal: "something went wrong".to_owned()
        }
    )
}

#[test]
fn test_serialize_ret_inf_after_step_err() {
    let serialized: String = serde_json::to_string(&RetInfAfterStep::Err {
        why_illegal: "something went wrong".to_owned(),
    }).unwrap();
    assert_eq!(
        serialized,
        r#"{"legal":false,"whyIllegal":"something went wrong"}"#
    )
}

#[test]
fn test_deserialize_ret_inf_after_step_ok() {
    let deserialized: RetInfAfterStep = serde_json::from_str(
        r#"{
        "legal": true,
        "ciurl": [false, true, false, false, false]
    }"#,
    )
    .unwrap();
    assert_eq!(
        deserialized,
        RetInfAfterStep::Ok {
            ciurl: Ciurl(false, true, false, false, false)
        }
    )
}

#[test]
fn test_serialize_ret_inf_after_step_ok() {
    let serialized: String = serde_json::to_string(&RetInfAfterStep::Ok {
        ciurl: Ciurl(false, true, false, false, false)
    }).unwrap();
    assert_eq!(
        serialized,
        r#"{"legal":true,"ciurl":[false,true,false,false,false]}"#
    )
}
