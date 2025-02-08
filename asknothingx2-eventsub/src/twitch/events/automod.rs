use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageHoldEvent {
    message_id: String,
    message: MessageBody,
    category: String,
    level: u64,
    held_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageHoldV2Event {
    message_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageBody {
    text: String,
}
