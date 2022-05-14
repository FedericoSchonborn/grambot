use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::types::errors::{ParseDiceKindError, TryFromDiceKindError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct Dice {
    pub emoji: DiceEmoji,
    /// Value of the dice:
    ///
    /// - 1-6 for [`Dice`](DiceEmoji::Dice), [`Bullseye`](DiceEmoji::Bullseye) and [`Bowling`](DiceEmoji::Bowling).
    /// - 1-5 for [`Basketball`](DiceEmoji::Basketball) and [`Football`](DiceEmoji::Football).
    /// - 1-64 for [`SlotMachine`](DiceEmoji::SlotMachine).
    pub value: i8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum DiceEmoji {
    /// Game Die (🎲).
    #[serde(rename = "🎲")]
    Dice,
    /// Bullseye (🎯).
    #[serde(rename = "🎯")]
    Bullseye,
    /// Bowling (🎳).
    #[serde(rename = "🎳")]
    Bowling,
    /// Basketball (🏀).
    #[serde(rename = "🏀")]
    Basketball,
    /// Football (⚽).
    #[serde(rename = "⚽")]
    Football,
    /// Slot Machine (🎰).
    #[serde(rename = "🎰")]
    SlotMachine,
}

impl DiceEmoji {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Dice => "🎲",
            Self::Bullseye => "🎯",
            Self::Bowling => "🎳",
            Self::Basketball => "🏀",
            Self::Football => "⚽",
            Self::SlotMachine => "🎰",
        }
    }
}

impl Default for DiceEmoji {
    fn default() -> Self {
        Self::Dice
    }
}

impl Display for DiceEmoji {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl FromStr for DiceEmoji {
    type Err = ParseDiceKindError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "🎲" => Ok(Self::Dice),
            "🎯" => Ok(Self::Bullseye),
            "🎳" => Ok(Self::Bowling),
            "🏀" => Ok(Self::Basketball),
            "⚽" => Ok(Self::Football),
            "🎰" => Ok(Self::SlotMachine),
            _ => Err(ParseDiceKindError),
        }
    }
}

impl TryFrom<char> for DiceEmoji {
    type Error = TryFromDiceKindError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '🎲' => Ok(Self::Dice),
            '🎯' => Ok(Self::Bullseye),
            '🎳' => Ok(Self::Bowling),
            '🏀' => Ok(Self::Basketball),
            '⚽' => Ok(Self::Football),
            '🎰' => Ok(Self::SlotMachine),
            _ => Err(TryFromDiceKindError),
        }
    }
}
