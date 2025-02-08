use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub message_id: String,
    pub message: MessageBody,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageBody {
    pub text: String,
    pub fragments: Fragments,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fragments {
    emotes: Vec<Emote>,
    cheermotes: Vec<Cheermote>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Emote {
    pub text: String,
    pub id: String,
    #[serde(rename = "set-id")]
    pub set_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cheermote {
    pub text: String,
    pub amount: u64,
    pub prefix: String,
    pub tier: u64,

    pub bits: u64,
}
