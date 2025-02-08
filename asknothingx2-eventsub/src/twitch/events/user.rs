use asknothingx2_util::oauth::ClientId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Update {
    pub email: String,
    pub email_verified: bool,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Authorization {
    pub client_id: ClientId,
}
