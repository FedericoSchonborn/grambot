use std::fmt::{self, Formatter};

use serde::{
    de::{Error as DeError, Unexpected, Visitor},
    Deserialize, Deserializer,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct True;

impl<'de> Deserialize<'de> for True {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TrueVisitor;

        impl<'de> Visitor<'de> for TrueVisitor {
            type Value = True;

            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("the literal `true` value")
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                if value {
                    Ok(True)
                } else {
                    Err(DeError::invalid_value(Unexpected::Bool(false), &self))
                }
            }
        }

        deserializer.deserialize_bool(TrueVisitor)
    }
}
