use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamOnlineEvent {
    #[serde(rename = "type")]
    pub kind: StreamType,
    pub started_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StreamType {
    Live,
    PlayList,
    WatchParty,
    Premiere,
    Rerun,
}
