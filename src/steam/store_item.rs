use serde::{Deserialize, Deserializer, de};
use serde_json::Value;
use std::{fmt::Display, hash::Hash, ops::Deref};

use super::StoreItemUserDetails;

#[derive(Debug, Clone, Deserialize)]
pub struct Price {
    /// ISO4217 code
    pub currency: String,

    #[serde(rename = "final")]
    pub value_in_cents: u32,
}

#[derive(Debug, Default, Hash, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct StoreItemId(#[serde(deserialize_with = "deserialize_u32")] pub u32);

impl Display for StoreItemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for StoreItemId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> AsRef<T> for StoreItemId
where
    <StoreItemId as Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref().as_ref()
    }
}

impl From<StoreItemId> for u32 {
    fn from(val: StoreItemId) -> Self {
        val.0
    }
}

impl From<u32> for StoreItemId {
    fn from(val: u32) -> Self {
        StoreItemId(val)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum StoreItemKind {
    #[serde(rename = "app")]
    App,
    #[serde(rename = "sub")]
    Sub,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SteamStoreItem {
    #[serde(rename = "type")]
    pub kind: StoreItemKind,

    pub id: StoreItemId,
    pub name: String,

    #[serde(rename = "tiny_image")]
    pub capsule_url: String,

    pub price: Option<Price>,

    #[serde(skip)]
    pub user_details: Option<StoreItemUserDetails>,
}

impl PartialEq for SteamStoreItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for SteamStoreItem {}

/// deserialize number or string to u32
fn deserialize_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;

    match value {
        Value::Number(num) => num
            .as_u64()
            .and_then(|n| u32::try_from(n).ok())
            .ok_or_else(|| {
                de::Error::invalid_type(
                    de::Unexpected::Other("large or negative number"),
                    &"a u32 or a stringified u32",
                )
            }),
        Value::String(s) => s.parse::<u32>().map_err(de::Error::custom),
        _ => Err(de::Error::invalid_type(
            de::Unexpected::Other("unsupported type"),
            &"a u32 or a stringified u32",
        )),
    }
}
