use serde::Deserialize;
use serde::Serialize;

use crate::twitch::{IntoCondition, Product};

new_request!(
/// <https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#extensionbits_transactioncreate>
    ExtensionBitsTransactionRequest,
    ExtensionBitsTransactionCreate,
    ExtensionBitsTransactionCreateCondition,
    {
        extension_client_id: extension_client_id
    }
);

new_response_payload!(
/// <https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#extensionbits_transactioncreate>
    ExtensionBitsTransactionPayload,
    ExtensionBitsTransactionCreateCondition,
    ExtensionBitsTransactionEvent
);

/// <https://dev.twitch.tv/docs/eventsub/eventsub-reference/#extension-bits-transaction-create-condition>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtensionBitsTransactionCreateCondition {
    pub extension_client_id: String,
}

impl ExtensionBitsTransactionCreateCondition {
    pub fn new<T: Into<String>>(extension_client_id: T) -> Self {
        Self {
            extension_client_id: extension_client_id.into(),
        }
    }
}

impl IntoCondition for ExtensionBitsTransactionCreateCondition {}

/// <https://dev.twitch.tv/docs/eventsub/eventsub-reference/#extension-bits-transaction-create-event>
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
