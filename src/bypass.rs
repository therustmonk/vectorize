//! `bypass` is necessary to force deriving serialization of complex type specs.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize<'a, T, S>(target: T, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize + 'a,
{
    serde::Serialize::serialize(&target, ser)
}

pub fn deserialize<'de, T, D>(des: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    serde::Deserialize::deserialize(des)
}
