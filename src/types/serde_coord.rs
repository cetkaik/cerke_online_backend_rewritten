use serde::de::{Unexpected};

use super::AbsoluteCoord;

pub mod opt {
    use serde::Deserialize;

    use crate::types::AbsoluteCoord;
    use super::CoordVisitor;

    pub fn serialize<S>(value: &Option<AbsoluteCoord>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            match value {
                Some(value) => super::serialize(value, serializer),
                None => serializer.serialize_none()
            }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<AbsoluteCoord>, D::Error>
    where
    D: serde::Deserializer<'de> {
        let visitor = CoordVisitor;
        deserializer.deserialize_tuple(2, visitor).or_else( |opt| 
            Ok(None)
        )
    }

}



pub fn serialize<S>(value: &AbsoluteCoord, serializer: S) -> Result<S::Ok, S::Error>
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
    type Value = Option<AbsoluteCoord>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a coordinate")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
            E: serde::de::Error, {
        Ok(None)
    }

    fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error> where
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
                Ok( Some(Coord(row, column)) )
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

pub fn deserialize<'de, D>(deserializer: D) -> Result<AbsoluteCoord, D::Error>
where
D: serde::Deserializer<'de> {
    let visitor = CoordVisitor;
    deserializer.deserialize_tuple(2, visitor).and_then( |opt| opt.ok_or(
        serde::de::Error::invalid_value(Unexpected::Other("null"), &"[str,str]"  )
    ))
}