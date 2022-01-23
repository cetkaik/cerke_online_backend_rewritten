
use cetkaik_full_state_transition::{Rate, Season};
use cetkaik_core::absolute::Field;
use rand::{Rng, prelude::ThreadRng};
use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr,Deserialize_repr};

use super::MoveToBePolled;

pub type AbsoluteCoord = cetkaik_core::absolute::Coord;

pub struct GameState {
    pub f: Field,
    pub tam_itself_is_tam_hue: bool,
    pub is_ia_owner_s_turn: bool,
    pub waiting_for_after_half_acceptance: Option<SrcStep>,
    pub season: Season,
    pub ia_owner_s_score: isize,
    pub rate: Rate,
    pub moves_to_be_polled: [Vec<MovePiece>; 4],
}
pub struct SrcStep {
    pub src: AbsoluteCoord,
    pub step: AbsoluteCoord,
}

pub struct MovePiece {
    pub mov: MoveToBePolled,
    pub status: Option<HandCompletionStatus>,
    pub by_ia_owner: bool,
}
pub enum HandCompletionStatus {
    TyMok,
    TaXot,
    NotYetDetermined,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Debug, Copy, Clone)]
#[repr(u8)]
pub enum Color {
    Kok1 = 0,
    Huok2 = 1,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Debug, Copy, Clone)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
// Using boolean is natural, and this is also necessary to allow easy interop with the frontend
#[allow(clippy::struct_excessive_bools)]
pub struct Ciurl(bool, bool, bool, bool, bool);

impl Ciurl {
    pub fn new(rng: &mut ThreadRng) -> Ciurl {
        Ciurl(rng.gen(), rng.gen(), rng.gen(), rng.gen(), rng.gen())
    }
    pub fn count(self) -> usize {
        self.0 as usize + self.1 as usize + self.2 as usize + self.3 as usize + self.4 as usize
    }
}

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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone)]
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
        water_entry_ciurl: Option<Ciurl>,
    },
    SrcStepDstFinite {
        src: AbsoluteCoord,
        step: AbsoluteCoord,
        dest: AbsoluteCoord,
        water_entry_ciurl: Option<Ciurl>,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone)]
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