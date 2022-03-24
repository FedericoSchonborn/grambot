use serde::{de::Error as DeError, Deserialize, Deserializer};

use crate::{error::Error, output::ResponseParameters};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Response<T> {
    Ok(T),
    Err(Error),
}

impl<'de, T> Deserialize<'de> for Response<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawResponse<T> {
            pub ok: bool,
            pub description: Option<String>,
            pub result: Option<T>,
            pub error_code: Option<i32>,
            pub parameters: Option<ResponseParameters>,
        }

        let raw = RawResponse::deserialize(deserializer)?;
        if raw.ok {
            Ok(Response::Ok(
                raw.result.ok_or_else(|| DeError::missing_field("result"))?,
            ))
        } else {
            Ok(Response::Err(Error {
                description: raw
                    .description
                    .ok_or_else(|| DeError::missing_field("description"))?,
                error_code: raw.error_code,
                parameters: raw.parameters,
            }))
        }
    }
}
