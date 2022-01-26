
use cetkaik_full_state_transition::{Rate, Season, Config, state};
use cetkaik_core::absolute::Field;
use rand::{Rng, prelude::ThreadRng};
use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr,Deserialize_repr};

use super::MoveToBePolled;

pub type AbsoluteCoord = cetkaik_core::absolute::Coord;


pub enum Phase { 
    Start(state::A),
    BeforeCiurl(state::CWithoutCiurl),
    AfterCiurl(state::C),
    Moved(state::HandNotResolved),
}

impl Phase { 
    pub fn whose_turn (&self) -> cetkaik_core::absolute::Side {
        match self {
            Phase::Start(x) => x.whose_turn,
            Phase::BeforeCiurl(x) => x.whose_turn,
            Phase::AfterCiurl(x) => x.c.whose_turn,
            Phase::Moved(x ) => x.whose_turn,
        }
    }

    pub fn get_season(&self) -> Season {
        match self {
            Phase::Start(x) => x.season,
            Phase::BeforeCiurl(x) => x.season,
            Phase::AfterCiurl(x) => x.c.season,
            Phase::Moved(x ) => x.season,
        }
    }
}

pub struct GameState {
    pub state: Phase,
    pub config: Config,
    pub waiting_for_after_half_acceptance: Option<SrcStep>,
    pub moves_to_be_polled: [Vec<MovePiece>; 4],
}

impl GameState { 
    pub fn is_ia_owner_s_turn(&self) -> bool {
        self.state.whose_turn() == cetkaik_core::absolute::Side::IASide
    }
}


#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct SrcStep {
    pub src: AbsoluteCoord,
    pub step: AbsoluteCoord,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MovePiece {
    pub mov: MoveToBePolled,
    pub status: Option<HandCompletionStatus>,
    pub by_ia_owner: bool,
}
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
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

impl From<Color> for cetkaik_core::Color {
    fn from(color: Color) -> Self {
        match color {
            Color::Kok1 => Self::Kok1,
            Color::Huok2 => Self::Huok2,
        }
    }
}
impl From<cetkaik_core::Color> for Color {
    fn from(color: cetkaik_core::Color) -> Self {
        match color {
            cetkaik_core::Color::Kok1 => Self::Kok1,
            cetkaik_core::Color::Huok2 => Self::Huok2,
        }
    }
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

impl From<Profession> for cetkaik_core::Profession {
    fn from(prof: Profession) -> Self {
        match prof {
            Profession::Nuak1 => Self::Nuak1,
            Profession::Kauk2 => Self::Kauk2,
            Profession::Gua2 => Self::Gua2,
            Profession::Kaun1 => Self::Kaun1,
            Profession::Dau2 => Self::Dau2,
            Profession::Maun1 => Self::Maun1,
            Profession::Kua2 => Self::Kua2,
            Profession::Tuk2 => Self::Tuk2,
            Profession::Uai1 => Self::Uai1,
            Profession::Io => Self::Io,
        }
    }
}

impl From<cetkaik_core::Profession> for Profession {
    fn from(prof: cetkaik_core::Profession) -> Self {
        match prof {
            cetkaik_core::Profession::Nuak1 => Self::Nuak1,
            cetkaik_core::Profession::Kauk2 => Self::Kauk2,
            cetkaik_core::Profession::Gua2 => Self::Gua2,
            cetkaik_core::Profession::Kaun1 => Self::Kaun1,
            cetkaik_core::Profession::Dau2 => Self::Dau2,
            cetkaik_core::Profession::Maun1 => Self::Maun1,
            cetkaik_core::Profession::Kua2 => Self::Kua2,
            cetkaik_core::Profession::Tuk2 => Self::Tuk2,
            cetkaik_core::Profession::Uai1 => Self::Uai1,
            cetkaik_core::Profession::Io => Self::Io,
        }
    }
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

impl From<usize> for Ciurl {
    fn from(cnt: usize) -> Self {
        use rand::seq::SliceRandom;
        
        let s = [false; 5];
        for i in 0..cnt {
            s[i] = true;
        }
        let rng = rand::thread_rng();
        s.shuffle(&mut rng);
        Self(s[0],s[1],s[2],s[3],s[4])
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
        #[serde(serialize_with="serialize_coord",deserialize_with="deserialize_coord")]
        dest: AbsoluteCoord,
    },
    SrcDst {
        #[serde(serialize_with="serialize_coord",deserialize_with="deserialize_coord")]
        src: AbsoluteCoord,
        #[serde(serialize_with="serialize_coord",deserialize_with="deserialize_coord")]
        dest: AbsoluteCoord,
        #[serde(skip_serializing_if = "Option::is_none")]
        water_entry_ciurl: Option<Ciurl>,
    },
    SrcStepDstFinite {
        #[serde(serialize_with="serialize_coord",deserialize_with="deserialize_coord")]
        src: AbsoluteCoord,
        #[serde(serialize_with="serialize_coord",deserialize_with="deserialize_coord")]
        step: AbsoluteCoord,
        #[serde(serialize_with="serialize_coord",deserialize_with="deserialize_coord")]
        dest: AbsoluteCoord,
        #[serde(skip_serializing_if = "Option::is_none")]
        water_entry_ciurl: Option<Ciurl>,
    },
}
fn serialize_coord<S>(value: &AbsoluteCoord, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&value.0)?;
        seq.serialize_element(&value.1)?;
        seq.end()
    }

struct CoordVisitor;

impl<'de> serde::de::Visitor<'de> for CoordVisitor {
    type Value = AbsoluteCoord;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a coordinate")
    }

    fn visit_seq<V>(self, mut visitor: V) -> Result<AbsoluteCoord, V::Error> where
    V: serde::de::SeqAccess<'de>{
        use cetkaik_core::absolute::{Column,Row,Coord};
        let mut column = None;
        let mut row = None;
        
        for i in 0..2 {
            if let Some(item) = visitor.next_element()? {
                match item {
                    "C" => { column = Some(Column::C)}
                    "K" => { column = Some(Column::K)}
                    "L" => { column = Some(Column::L)}
                    "M" => { column = Some(Column::M)}
                    "N" => { column = Some(Column::N)}
                    "P" => { column = Some(Column::P)}
                    "T" => { column = Some(Column::T)}
                    "X" => { column = Some(Column::X)}
                    "Z" => { column = Some(Column::Z)}

                    "A" => { row = Some(Row::A)} 
                    "AI" => { row = Some(Row::AI)} 
                    "AU" => { row = Some(Row::AU)} 
                    "E" => { row = Some(Row::E)} 
                    "I" => { row = Some(Row::I)} 
                    "O" => { row = Some(Row::O)} 
                    "U" => { row = Some(Row::U)} 
                    "Y" => { row = Some(Row::Y)} 
                    "IA" => { row = Some(Row::IA)} 

                    _ => {
                        return Err(serde::de::Error::invalid_value(
                            serde::de::Unexpected::Str(item),
                            &self,
                        ))
                    }
                }
            } else { 
                return Err(
                    serde::de::Error::invalid_length(i, &"2")
                );
            }
        }

        if let Some(column) = column { 
            if let Some(row) = row {
                Ok( Coord(row, column) )
            } else {
                Err(
                    serde::de::Error::missing_field("row")
                )
            }
        } else {
            Err(
                serde::de::Error::missing_field("column")
            )
        }

    }
}

fn deserialize_coord<'de, D>(deserializer: D) -> Result<AbsoluteCoord, D::Error>
where
D: serde::Deserializer<'de> {
    let visitor = CoordVisitor;
    deserializer.deserialize_tuple(2, visitor)
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