use serde::{Deserialize, Serialize};

use crate::twitch::types::objects::Contribution;

#[derive(Debug, Serialize, Deserialize)]
pub struct HypeEvent {
    pub total: u64,
    pub progress: Option<u64>,
    pub goal: Option<u64>,
    pub top_contributions: Contribution,
    pub last_contribution: Option<Contribution>,
    pub level: u64,
    pub started_at: String,
    pub ended_at: Option<String>,
    pub expires_at: Option<String>,
    pub is_golden_kappa_train: bool,
    pub cooldown_ends_at: Option<String>,
}
