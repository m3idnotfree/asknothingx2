use serde::Serialize;
use twitch_highway::{
    base::{IntoQueryPairs, QueryParams},
    request::IntoRequestBody,
    types::UserId,
};

use super::types::{Status, SubscriptionType, Transport};

#[derive(Debug, Serialize)]
pub struct GetEventRequest {
    pub status: Option<Status>,
    #[serde(rename = "type")]
    pub kind: Option<SubscriptionType>,
    pub user_id: Option<UserId>,
}

impl IntoQueryPairs for GetEventRequest {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();
        params
            .push_opt("status", self.status)
            .push_opt("type", self.kind)
            .push_opt("user_id", self.user_id);

        params.build()
    }
}

#[derive(Debug, Serialize)]
pub struct CreateEventSubRequest<Condition> {
    #[serde(rename = "type")]
    pub kind: SubscriptionType,
    pub version: String,
    pub condition: Condition,
    pub transport: Transport,
}

impl<Condition> CreateEventSubRequest<Condition> {
    pub fn new(kind: SubscriptionType, condition: Condition, transport: Transport) -> Self {
        Self {
            version: kind.version().to_string(),
            kind,
            condition,
            transport,
        }
    }
}

impl<C> IntoRequestBody for CreateEventSubRequest<C>
where
    C: Serialize,
{
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(self).unwrap())
    }
}

impl<'de, Condition: serde::Deserialize<'de>> serde::Deserialize<'de>
    for CreateEventSubRequest<Condition>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct Helper<Condition> {
            #[serde(rename = "type")]
            kind: SubscriptionType,
            version: String,
            condition: Condition,
            transport: Transport,
        }

        let helper = Helper::deserialize(deserializer)?;

        let kind = match (helper.kind, helper.version.as_ref()) {
            (SubscriptionType::AutomodMessageHold, "2") => SubscriptionType::AutomodMessageHoldV2,
            (SubscriptionType::AutomodMessageUpdate, "2") => {
                SubscriptionType::AutomodMessageUpdateV2
            }
            (SubscriptionType::ChannelModerate, "2") => SubscriptionType::ChannelModerateV2,
            (kind, _) => kind,
        };

        Ok(CreateEventSubRequest {
            kind,
            version: helper.version,
            condition: helper.condition,
            transport: helper.transport,
        })
    }
}
