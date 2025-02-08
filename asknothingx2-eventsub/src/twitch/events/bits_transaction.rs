use serde::{Deserialize, Serialize};
use twitch_highway::types::UserId;

use crate::twitch::types::objects::Product;

/// <https://dev.twitch.tv/docs/eventsub/eventsub-reference/#extension-bits-transaction-create-event>
#[derive(Debug, Serialize, Deserialize)]
pub struct BitsTransactionEvent {
    pub extension_client_id: String,
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub product: Product,
}
