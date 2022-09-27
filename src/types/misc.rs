use serde::{Deserialize, Serialize};

use uuid::Uuid;



/// A type that serialize into `{}`.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Unit {}

#[test]
fn unit_serde() {
    assert_eq!("{}", serde_json::to_string(&Unit {}).unwrap());
    let g: Unit = serde_json::from_str("{}").unwrap();
    assert_eq!(Unit {}, g);
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]

pub struct RoomId(pub Uuid);

impl std::fmt::Display for RoomId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct AccessToken(pub Uuid);

impl AccessToken {
    /// # Errors
    /// Returns `Err` if the Uuid is not valid
    pub fn parse_str(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl std::fmt::Display for AccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.hyphenated().to_string())
    }
}

#[derive(Debug, Clone)]
pub struct RoomInfoWithPerspective {
    pub room_id: RoomId, 
    pub is_ia_down_for_me: bool,
}