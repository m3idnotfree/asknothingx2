#[macro_use]
mod macros;
#[cfg(feature = "twitch-payload")]
pub mod payload;
#[cfg(feature = "twitch-websocket-message")]
pub mod websocket_message;

#[cfg(feature = "twitch-reference")]
pub mod reference;

#[cfg(feature = "twitch-subscription-type")]
pub mod subscription_types;
