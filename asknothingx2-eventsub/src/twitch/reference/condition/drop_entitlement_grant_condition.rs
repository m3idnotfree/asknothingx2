use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DropEntitlementGrantCondition {
    pub organization_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
}

impl DropEntitlementGrantCondition {
    pub fn new<T: Into<String>>(organization_id: T) -> Self {
        Self {
            organization_id: organization_id.into(),
            category_id: None,
            campaign_id: None,
        }
    }

    pub fn set_category_id<T: Into<String>>(mut self, category_id: T) -> Self {
        self.category_id = Some(category_id.into());
        self
    }

    pub fn set_campaign_id<T: Into<String>>(mut self, campaign_id: T) -> Self {
        self.campaign_id = Some(campaign_id.into());
        self
    }
}
