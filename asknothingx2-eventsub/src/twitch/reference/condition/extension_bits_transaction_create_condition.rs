use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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
