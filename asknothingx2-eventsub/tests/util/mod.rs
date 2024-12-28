#[macro_export]
macro_rules! contains {
    ($text:expr, $name:ident, $($expected:literal),+) => {
        contains!(@$name $text, $($expected),+);
    };
    (@contain $text:expr, $($expected:literal),+) => {
        $(assert!($text.contains($expected));)+
    };
    (@not $text:expr, $($expected:literal),+) => {
        $(assert!(!$text.contains($expected));)+
    };
}

macro_rules! meta {
    ($_:expr, $actual:expr, message_timestamp: $expected:expr) => {
        pretty_assertions::assert_eq!(
            $actual,
            chrono::DateTime::parse_from_rfc3339($expected).unwrap()
        );
    };
    ($_:expr, $actual:expr, $field:ident: $expected:expr) => {
        pretty_assertions::assert_eq!($actual, $expected);
    };
}

macro_rules! session {
    ($_:expr, $actual:expr, connected_at: $expected:expr) => {
        pretty_assertions::assert_eq!(
            $actual,
            chrono::DateTime::parse_from_rfc3339($expected).unwrap()
        );
    };
    ($_:expr, $actual:expr, $field:ident: $expected:expr) => {
        pretty_assertions::assert_eq!($actual, $expected);
    };
}

macro_rules! subscription {
    ($_:expr, $actual:expr, created_at: $expected:expr) => {
        pretty_assertions::assert_eq!(
            $actual,
            chrono::DateTime::parse_from_rfc3339($expected).unwrap()
        );
    };
    ($_:expr, $actual:expr, $field:ident: $expected:expr) => {
        pretty_assertions::assert_eq!($actual, $expected);
    };
}

macro_rules! event {
    ($_:expr, $actual:expr, followed_at: $expected:expr) => {
        pretty_assertions::assert_eq!(
            $actual,
            chrono::DateTime::parse_from_rfc3339($expected).unwrap()
        );
    };
    ($_:expr, $actual:expr, $field:ident: $expected:expr) => {
        pretty_assertions::assert_eq!($actual, $expected);
    };
}
macro_rules! transport {
    ($_:expr, $actual:expr, followed_at: $expected:expr) => {
        pretty_assertions::assert_eq!(
            $actual,
            chrono::DateTime::parse_from_rfc3339($expected).unwrap()
        );
    };
    ($_:expr, $actual:expr, $field:ident: $expected:expr) => {
        pretty_assertions::assert_eq!($actual, $expected);
    };
}

macro_rules! condition {
    ($_:expr, $actual:expr, followed_at: $expected:expr) => {
        pretty_assertions::assert_eq!(
            $actual,
            chrono::DateTime::parse_from_rfc3339($expected).unwrap()
        );
    };
    ($_:expr, $actual:expr, $field:ident: $expected:expr) => {
        pretty_assertions::assert_eq!($actual, $expected);
    };
}

macro_rules! expected_payload {
    (
        $default:expr,
        $struct:expr
        $(, $name:ident: {
            $($field:ident: $expect:expr),+ $(,)?
        })+
    ) => {
        $(expected_payload!(@$name $default, $struct, {$($field: $expect),+});)+
    };
    (@condition $default:expr, $struct:expr, {$($field:ident: $expect:expr),+ $(,)?}) => {
        $(condition!($struct, $struct.subscription.condition.$field, $field: $expect);)+
    };
    (@subscription $default:expr, $struct:expr, {$($field:ident: $expect:expr),+ $(,)?}) => {
        $(subscription!($struct, $struct.subscription.$field, $field: $expect);)+
    };
    (@event $default:expr, $struct:expr, {$($field:ident: $expect:expr),+ $(,)?}) => {
        $(event!($struct, $struct.event.$field, $field: $expect);)+
    };
    (@transport $default:expr, $struct:expr, {$($field:ident: $expect:expr),+ $(,)?}) => {
        $(transport!($struct, $struct.subscription.transport.$field, $field: $expect);)+
    };
    (@session $default:expr, $struct:expr, {$($field:ident: $expect:expr),+ $(,)?}) => {
        $(session!($struct, $struct.session.$field, $field: $expect);)+
    };
    (@meta $default:expr, $struct:expr, {$($field:ident: $expect:expr),+ $(,)?}) => {
        $(meta!($struct, $default.metadata.$field, $field: $expect);)+
    };
}

macro_rules! expected_extra {
    (
        $struct:expr,
        {
            $($field:ident: $expect:expr),+ $(,)?
        }
        $(, bool = {
            $($fie:ident: $expe:expr),+ $(,)?
        })?
    ) => {
        $(pretty_assertions::assert_eq!($struct.$field, $expect);)+
        $($(assert_eq!($struct.$fie,$expe);)+)?
    };
}

macro_rules! fn_expected_payload {
    (
        payload: $payload:literal,
        from_str: $from_str:ty
        $(, block $name:ident: {
            $($field:ident: $expect:expr),+ $(,)?
        })*
        $(, extra $($extra:ident).+: {
            $($extra_field:ident: $extra_expected:expr),+ $(,)?
        })*
        $(, se $se:ident: [
            $($se_expected:expr),+ $(,)?
        ])*
    ) => {
        #[test]
        pub fn payload(){
            #[allow(unused_imports)]
            use asknothingx2_eventsub::twitch::{
                subscription_types::SubscriptionType,
                TransportMethod,
            };

            let de = serde_json::from_str::<$from_str>($payload);
            assert!(de.is_ok());
            let de = de.unwrap();
            $(expected_payload!(@$name de, de.payload, {$($field: $expect),+});)*
            $(expected_extra!(de.payload.$($extra).+, {$($extra_field: $extra_expected),+});)?

        $(
            let se = serde_json::to_string(&de);
            assert!(se.is_ok());
            let se = se.unwrap();
            contains!(se, $se, $($se_expected),+);
        )?

        }
    };
    (
        payload: $payload:literal,
        from_str: $from_str:ty,
        prefix: $p:tt
        $(, block $name:ident: {
            $($field:ident: $expect:expr),+ $(,)?
        })*
        $(, extra $($extra:ident).+: {
            $($extra_field:ident: $extra_expected:expr),+ $(,)?
        })*
        $(, se $se:ident: [
            $($se_expected:expr),+ $(,)?
        ])*
    ) => {
        #[test]
        pub fn payload(){
            #[allow(unused_imports)]
            use asknothingx2_eventsub::twitch::{
                subscription_types::SubscriptionType,
                TransportMethod,
            };

            let de = serde_json::from_str::<$from_str>($payload);
            assert!(de.is_ok());
            let de = de.unwrap();
            $(expected_payload!(@$name de, de.$p.payload, {$($field: $expect),+});)*
            $(expected_extra!(de.payload.$($extra).+, {$($extra_field: $extra_expected),+});)?

        $(
            let se = serde_json::to_string(&de);
            assert!(se.is_ok());
            let se = se.unwrap();
            contains!(se, $se, $($se_expected),+);
        )?

        }
    };
}

macro_rules! fn_expected_request {
    (
        request: $request:expr,
        body: {
            $($ident:ident: [$($expected:expr),+ ]),+
        }
    ) => {
        #[test]
        fn request(){
            use asknothingx2_eventsub::twitch::subscription_types::request::IntoSubscriptionRequest;
            let body = $request.into_body();

            $(contains!(body, $ident, $($expected),+);)+
        }
    };
}
