use serde::{
    de::{Error as DeError, Unexpected},
    Deserialize, Deserializer,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct True;

impl<'de> Deserialize<'de> for True {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = bool::deserialize(deserializer)?;
        if value {
            Ok(Self)
        } else {
            Err(DeError::invalid_value(
                Unexpected::Bool(value),
                &"a boolean literal `true`",
            ))
        }
    }
}
