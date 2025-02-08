use serde::{Deserialize, Serialize};
use twitch_highway::types::UserId;

#[derive(Debug, Serialize, Deserialize)]
pub struct WhisperReceived {
    pub from_user_id: UserId,
    pub from_user_name: String,
    pub from_user_login: String,
    pub to_user_id: UserId,
    pub to_user_name: String,
    pub to_user_login: String,
    pub whisper_id: String,
    pub whisper: Whisper,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Whisper {
    text: String,
}
