#[macro_use]
mod util;

mod subscription;
mod websocket_message;

// refactor(metadata): reorganize test directory structure
#[test]
fn main() {
    websocket_message::keepalive();
    websocket_message::notification();
    websocket_message::reconnect();
    websocket_message::revocation();
    websocket_message::welcome();
    websocket_message::deserialize_metadata();

    subscription::channel_follow::request();
    subscription::channel_follow::payload();
    subscription::channel_raid::request();
    subscription::channel_raid::payload();
    subscription::conduit_shard_disabled::request();
    subscription::conduit_shard_disabled::payload();
    subscription::drop_entitlement_grant::request();
    subscription::drop_entitlement_grant::payload();
    subscription::extension_bits_transaction::request();
    subscription::extension_bits_transaction::payload();
}
