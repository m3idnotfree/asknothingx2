use serde::{Deserialize, Serialize};
use twitch_highway::charity::types::Amount;

#[derive(Debug, Serialize, Deserialize)]
pub struct CharityEvent {
    pub charity_name: String,
    pub charity_description: String,
    pub charity_logo: String,
    pub charity_website: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharityCampaignEvent {
    pub current_amount: Amount,
    pub target_amount: Amount,
    pub started_at: Option<String>,
    pub stopped_at: Option<String>,
}

// With Event, UserEvent,CharityEvent
#[derive(Debug, Serialize, Deserialize)]
pub struct CharityDonationEvent {
    pub campaign_id: String,
    pub amount: Amount,
}
