use asknothingx2_util::api::Method;
use request::{CreateEventSubRequest, GetEventRequest};
use response::EventSubscriptionsResponse;
use twitch_highway::{
    base::TwitchAPIBase,
    request::{EmptyBody, EndpointType, TwitchAPIRequest},
    types::PaginationQuery,
    TwitchAPI,
};
use types::{Condition, SubscriptionId};

pub mod error;
pub mod events;
pub mod request;
pub mod response;
pub mod subscription_types;
pub mod types;

#[cfg(feature = "twitch-websocket")]
pub mod websocket_message;

pub trait EventSubAPI: TwitchAPIBase {
    fn create_eventsub(
        &self,
        request: CreateEventSubRequest<Condition>,
    ) -> TwitchAPIRequest<CreateEventSubRequest<Condition>, EventSubscriptionsResponse>;
    fn delete_eventsub(
        &self,
        subscription_id: SubscriptionId,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody>;
    fn get_eventsub(
        &self,
        opts: Option<GetEventRequest>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, EventSubscriptionsResponse>;
}

impl EventSubAPI for TwitchAPI {
    fn create_eventsub(
        &self,
        request: CreateEventSubRequest<Condition>,
    ) -> TwitchAPIRequest<CreateEventSubRequest<Condition>, EventSubscriptionsResponse> {
        let mut url = self.build_url();
        url.path(["eventsub", "subscriptions"]);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::CreateEventSub,
            url.build(),
            Method::POST,
            headers.build(),
            request,
        )
    }
    fn delete_eventsub(
        &self,
        subscription_id: SubscriptionId,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody> {
        let mut url = self.build_url();
        url.path(["eventsub", "subscriptions"])
            .query("id", subscription_id);

        TwitchAPIRequest::new(
            EndpointType::DeleteEventSub,
            url.build(),
            Method::DELETE,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_eventsub(
        &self,
        opts: Option<GetEventRequest>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, EventSubscriptionsResponse> {
        let mut url = self.build_url();
        url.path(["eventsub", "subscriptions"])
            .query_opt_pairs(opts)
            .query_opt_pairs(pagination);

        TwitchAPIRequest::new(
            EndpointType::GetEventSub,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
