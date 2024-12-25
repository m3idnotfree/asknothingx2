use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::twitch::reference::Product;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionBitsTransactionEvent {
    pub extension_client_id: String,
    pub id: String,
    pub broadcaster_user_id: String,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
    pub product: Product,
}
