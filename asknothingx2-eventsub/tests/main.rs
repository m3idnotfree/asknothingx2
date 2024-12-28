#[macro_use]
mod util;

mod subscription;
mod websocket_message;
refactor(eventsub): reorganize structure

- Type alias EventPayload, SingePayload
- Reorganize feature flags
- Rename SubscriptionTypes to SubscriptionType
- Add new_request macro warpping SubscriptionRequest
- Add response_payload macro warpping 
- Add fn_expected_request, fn_expected_payload macro for test
