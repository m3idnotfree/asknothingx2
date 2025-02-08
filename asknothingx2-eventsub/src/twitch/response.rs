use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::{Deserialize, Serialize};
use twitch_highway::types::Pagination;

pub use twitch_highway::types::Images;

use super::types::Subscription;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventSubscriptionsResponse {
    pub data: Vec<Subscription>,
    pub total_cost: u64,
    pub max_total_cost: u64,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}
