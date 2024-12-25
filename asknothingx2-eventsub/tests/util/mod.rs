macro_rules! expected_de_metadata {
    (
    $deserialized:ident,
    $message_id:literal,
    $message_type:expr,
    $message_timestamp:literal $(,)?
    ) => {
        pretty_assertions::assert_eq!($deserialized.metadata.message_id, $message_id.to_string());
        pretty_assertions::assert_eq!($deserialized.metadata.message_type, $message_type);
        pretty_assertions::assert_eq!(
            $deserialized.metadata.message_timestamp,
            chrono::DateTime::parse_from_rfc3339($message_timestamp).unwrap()
        );
    };
    (
    $deserialized:ident,
    $message_id:literal,
    $message_type:expr,
    $message_timestamp:literal,
    $subscription_type:expr,
    $subscription_version:expr $(,)?
    ) => {
        expected_de_metadata!(
            $deserialized,
            $message_id,
            $message_type,
            $message_timestamp,
        );
        pretty_assertions::assert_eq!($deserialized.metadata.subscription_type, $subscription_type);
        pretty_assertions::assert_eq!(
            $deserialized.metadata.subscription_version,
            $subscription_version
        );
    };
}

#[macro_export]
macro_rules! se_contains {
    ($serialized:expr, $($text:literal),*) => {
        $(assert!($serialized.contains($text));)*
    };
    ($serialized:expr, $($text:literal),*, not = ($($text2:literal),*) ) => {
        $(assert!($serialized.contains($text));)*
        $(assert!(!$serialized.contains($text2));)*
    };
}

macro_rules! expected_de_session {
    ($name:ident, $($field:ident = $expected:expr),*) => {
        $(pretty_assertions::assert_eq!($name.payload.session.$field, $expected));*
    };
}

macro_rules! expect_de_subscription {
    ($name:ident, $($field:ident = $expect:expr),* ) => {
        $(pretty_assertions::assert_eq!($name.payload.subscription.$field, $expect);)*
    };
    (req, $name:ident, $($field:ident = $expect:expr),* ) => {
        $(pretty_assertions::assert_eq!($name.subscription.$field, $expect);)*
    };
}

macro_rules! expect_de_condition {
    ($name:ident, $($field:ident = $expect:expr),* ) => {
        $(pretty_assertions::assert_eq!($name.payload.subscription.condition.$field, $expect);)*
    };
    (req, $name:ident, $($field:ident = $expect:expr),* ) => {
        $(pretty_assertions::assert_eq!($name.subscription.condition.$field, $expect);)*
    };
}

macro_rules! expect_de_transport {
    (
    $name:ident,
    $method:expr,
    $callback:expr,
    $secret:expr,
    $session:expr,
    $conduit:expr,
    $connect:expr,
    $disconnect:expr
    ) => {
        pretty_assertions::assert_eq!($name.payload.subscription.transport.method, $method);
        pretty_assertions::assert_eq!($name.payload.subscription.transport.callback, $callback);
        pretty_assertions::assert_eq!($name.payload.subscription.transport.secret, $secret);
        pretty_assertions::assert_eq!($name.payload.subscription.transport.session_id, $session);
        pretty_assertions::assert_eq!($name.payload.subscription.transport.conduit_id, $conduit);
        pretty_assertions::assert_eq!($name.payload.subscription.transport.connected_at, $connect);
        pretty_assertions::assert_eq!(
            $name.payload.subscription.transport.disconnected_at,
            $disconnect
        );
    };

    (
    req,
    $name:ident,
    $method:expr,
    $callback:expr,
    $secret:expr,
    $session:expr,
    $conduit:expr,
    $connect:expr,
    $disconnect:expr
    ) => {
        pretty_assertions::assert_eq!($name.subscription.transport.method, $method);
        pretty_assertions::assert_eq!($name.subscription.transport.callback, $callback);
        pretty_assertions::assert_eq!($name.subscription.transport.secret, $secret);
        pretty_assertions::assert_eq!($name.subscription.transport.session_id, $session);
        pretty_assertions::assert_eq!($name.subscription.transport.conduit_id, $conduit);
        pretty_assertions::assert_eq!($name.subscription.transport.connected_at, $connect);
        pretty_assertions::assert_eq!($name.subscription.transport.disconnected_at, $disconnect);
    };
}

macro_rules! expect_de_event {
    ($name:ident, $($field:ident = $expect:expr),* ) => {
        $(pretty_assertions::assert_eq!($name.payload.event.$field, $expect);)*
    };
    (req, $name:ident, $($field:ident = $expect:expr),* ) => {
        $(pretty_assertions::assert_eq!($name.event.$field, $expect);)*
    };
}
