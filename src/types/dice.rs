use std::{
    fmt::{self, Display, Formatter, Write},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::types::errors::TryFromDiceKindError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct Dice {
    pub emoji: DiceEmoji,
    pub value: i8,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum DiceEmoji {
    #[serde(rename = "🎲")]
    Dice,
    #[serde(rename = "🎯")]
    Bullseye,
    #[serde(rename = "🎳")]
    Bowling,
    #[serde(rename = "🏀")]
    Basketball,
    #[serde(rename = "⚽")]
    Football,
    #[serde(rename = "🎰")]
    SlotMachine,
}

impl Default for DiceEmoji {
    fn default() -> Self {
        Self::Dice
    }
}

impl Display for DiceEmoji {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_char(match self {
            Self::Dice => '🎲',
            Self::Bullseye => '🎯',
            Self::Bowling => '🎳',
            Self::Basketball => '🏀',
            Self::Football => '⚽',
            Self::SlotMachine => '🎰',
        })
    }
}

impl FromStr for DiceEmoji {
    type Err = TryFromDiceKindError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "🎲" => Ok(Self::Dice),
            "🎯" => Ok(Self::Bullseye),
            "🎳" => Ok(Self::Bowling),
            "🏀" => Ok(Self::Basketball),
            "⚽" => Ok(Self::Football),
            "🎰" => Ok(Self::SlotMachine),
            _ => Err(TryFromDiceKindError),
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
