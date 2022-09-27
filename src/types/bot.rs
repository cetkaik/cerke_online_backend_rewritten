use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct BotToken(pub Uuid);

impl BotToken {
    /// # Errors
    /// Returns `Err` if the Uuid is not valid
    pub fn parse_str(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl std::fmt::Display for BotToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.hyphenated().to_string(),)
    }
}


#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(into = "&'static str")]
#[serde(try_from = "&str")]
pub enum TacticsKey {
    VictoryAlmostCertain,
    StrengthenedShaman,
    FreeLunch,
    AvoidDefeat,
    LossAlmostCertain,
    Neutral,
}

impl TryFrom<&str> for TacticsKey {
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "victory_almost_certain" => Ok(TacticsKey::VictoryAlmostCertain),
            "strengthened_shaman" => Ok(TacticsKey::StrengthenedShaman),
            "free_lunch" => Ok(TacticsKey::FreeLunch),
            "avoid_defeat" => Ok(TacticsKey::AvoidDefeat),
            "loss_almost_certain" => Ok(TacticsKey::LossAlmostCertain),
            "neutral" => Ok(TacticsKey::Neutral),
            s => Err(format!("unknown tactics name `{}` found. Please edit cerke_online_backend_rewritten repository.", s)),
        }
    }

    type Error = String;
}

impl From<TacticsKey> for &'static str {
    fn from(a: TacticsKey) -> &'static str {
        match a {
            TacticsKey::VictoryAlmostCertain => "victory_almost_certain",
            TacticsKey::StrengthenedShaman => "strengthened_shaman",
            TacticsKey::FreeLunch => "free_lunch",
            TacticsKey::AvoidDefeat => "avoid_defeat",
            TacticsKey::LossAlmostCertain => "loss_almost_certain",
            TacticsKey::Neutral => "neutral",
        }
    }
}