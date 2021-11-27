use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct AbsoluteCoord(AbsoluteRow, AbsoluteColumn);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, PartialEq, Eq)]
struct InfAfterStepInternal {
    src: AbsoluteCoord,
    step: AbsoluteCoord,

    #[serde(rename = "plannedDirection")]
    coord_signifying_planned_direction: AbsoluteCoord,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
struct WhoGoesFirst {
    result: bool,
    process: Vec<[Ciurl; 2]>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
enum RetTyMok {
    Err,
    Ok,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
enum RetTaXot {
    Err,
    Ok {
        is_first_move_my_move: Option<WhoGoesFirst>,
    },
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
enum RetInfAfterStep {
    Ok { ciurl: Ciurl },
    Err { why_illegal: String },
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
enum RetNormalMove {
    Err { why_illegal: String },
    WithWaterEntry { ciurl: Ciurl },
    WithoutWaterEntry,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
enum RetAfterHalfAcceptance {
    Err { why_illegal: String },
    WithWaterEntry { ciurl: Ciurl },
    WithoutWaterEntry,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "state")]
enum RetRandomEntry {
    #[serde(rename = "in_waiting_list")]
    InWaitingList { access_token: String },

    #[serde(rename = "let_the_game_begin")]
    LetTheGameBegin {
        access_token: String,
        is_first_move_my_move: bool,

        #[serde(rename = "is_IA_down_for_me")]
        is_ia_down_for_me: bool,
    },
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "state")]
enum RetVsCpuEntry {
    #[serde(rename = "let_the_game_begin")]
    LetTheGameBegin {
        access_token: String,
        is_first_move_my_move: bool,

        #[serde(rename = "is_IA_down_for_me")]
        is_ia_down_for_me: bool,
    },
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
enum RetRandomPoll {
    Err { why_illegal: String },
    Ok { ret: RetRandomEntry },
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
enum RetRandomCancel {
    Err { why_illegal: String },
    Ok { cancellable: bool },
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
enum RetWhetherTyMokPoll {
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
enum RetInfPoll {
    MoveMade { content: MoveToBePolled },
    NotYetDetermined,
    Err { why_illegal: String },
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
enum MoveToBePolled {
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
struct FinalResult {
    dest: AbsoluteCoord,
    water_entry_ciurl: Option<Ciurl>,
    thwarted_by_failing_water_entry_ciurl: Option<Ciurl>,
}
