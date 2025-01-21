use serde::{Deserialize, Serialize};

/// <https://dev.twitch.tv/docs/eventsub/eventsub-reference/#product>
#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub name: String,
    pub bits: u64,
    pub sku: String,
    /// Flag indicating if the product is in development. If in_development is true, bits will be 0.
    pub in_development: bool,
}
