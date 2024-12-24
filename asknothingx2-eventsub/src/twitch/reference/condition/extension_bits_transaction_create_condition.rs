use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExtensionBitsTransactionCreateCondition {
    pub extension_client_id: String,
}

impl ExtensionBitsTransactionCreateCondition {
    pub fn new(extension_client_id: String) -> Self {
        Self {
            extension_client_id,
        }
    }
}
