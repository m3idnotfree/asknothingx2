// #![allow(unused)]
macro_rules! expected_de_metadata {
    (
    $deserialized:ident,
    $message_id:literal,
    $message_type:expr,
    $message_timestamp:literal $(,)?
    ) => {
        assert_eq!($deserialized.metadata.message_id, $message_id.to_string());
        assert_eq!($deserialized.metadata.message_type, $message_type);
        assert_eq!(
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
        assert_eq!($deserialized.metadata.subscription_type, $subscription_type);
        assert_eq!(
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

macro_rules! expected_de_payload {
    ($payload:expr, $($($field:ident).+ = $expect:expr),* $(,)?) => {
        $(assert_eq!($payload.$($field).+, $expect);)*
    };

    ($payload:expr, $($($field:ident).+ = $expect:expr),* $(,)?;
     time = ($($($created:ident).+ = $expect_time:expr),* $(,)?)) => {
        deserialized_payload!($payload, $($($field).+ = $expect),*);
        $(
            assert_eq!(
                $payload.$($created).+,
                chrono::DateTime::parse_from_rfc3339($expect_time).unwrap()
            );
        )*
    };

    ($payload:expr, $($($field:ident).+ = $expect:expr),* $(,)?;
     contain = $table:ident, {$($key:literal = $value:literal),* $(,)?}) => {
        deserialized_payload!($payload, $($($field).+ = $expect),*);
        $(
            let value = $payload.$table.get($key);
            assert!(value.is_some());
            assert_eq!(value.unwrap(), $value);
        )*
    };

    // With both time and contain assertions
    ($payload:expr, $($($field:ident).+ = $expect:expr),* $(,)?;
     time = ($($($created:ident).+ = $expect_time:expr),* $(,)?);
     contain = $($table:ident).+, {$($key:literal = $value:literal),* $(,)?}) => {
        deserialized_payload!($payload, $($($field).+ = $expect),*);
        $(
            assert_eq!(
                $payload.$($created).+,
                chrono::DateTime::parse_from_rfc3339($expect_time).unwrap()
            );
        )*
         let field = $payload.$($table).+.clone();
        $(
            // let value = $payload.$table.get($key);
            let value = field.get($key);
            assert!(value.is_some());
            assert_eq!(value.unwrap(), $value);
        )*
     };

    ($payload:expr, $($($field:ident).+ = $expect:expr),* $(,)?;
     time = ($($($created:ident).+ = $expect_time:expr),* $(,)?);
     contain = $($table1:ident).+, {$($key:literal = $value:literal),* $(,)?};
     value = $($table2:ident).+, {$($key2:literal = $value2:literal),* $(,)?}) => {
        deserialized_payload!($payload, $($($field).+ = $expect),*);
        $(
            assert_eq!(
                $payload.$($created).+,
                chrono::DateTime::parse_from_rfc3339($expect_time).unwrap()
            );
        )*
         let field1 = $payload.$($table1).+.clone();
        $(
            let value = field1.get($key);
            assert!(value.is_some());
            assert_eq!(value.unwrap(), $value);
        )*

         let field2 = $payload.$($table2).+.clone();
        $(
            let value = field2.get($key2);
            assert!(value.is_some());
            assert_eq!(value.unwrap(), $value2);
        )*
     };
}

macro_rules! expected_de_session {
    ($name:ident, $($field:ident = $expected:expr),*) => {
        $(assert_eq!($name.payload.session.$field, $expected));*
    };
}

macro_rules! expect_de_subscription {
    ($name:ident, $($field:ident = $expect:expr),* ) => {
        $(assert_eq!($name.payload.subscription.$field, $expect);)*
    };
}

macro_rules! expect_de_condition {
    ($name:ident, $($field:ident = $expect:expr),* ) => {
        $(assert_eq!($name.payload.subscription.condition.$field, $expect);)*
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
        assert_eq!($name.payload.subscription.transport.method, $method);
        assert_eq!($name.payload.subscription.transport.callback, $callback);
        assert_eq!($name.payload.subscription.transport.secret, $secret);
        assert_eq!($name.payload.subscription.transport.session_id, $session);
        assert_eq!($name.payload.subscription.transport.conduit_id, $conduit);
        assert_eq!($name.payload.subscription.transport.connected_at, $connect);
        assert_eq!(
            $name.payload.subscription.transport.disconnected_at,
            $disconnect
        );
    };
}

macro_rules! expect_de_event {
    ($name:ident, $($field:ident = $expect:expr),* ) => {
        $(assert_eq!($name.payload.event.$field, $expect);)*
    };
}
