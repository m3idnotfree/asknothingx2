macro_rules! impl_de_with_subscription_type_must_have_veasion_and_condition {
    ($name:ident {$($field:ident: $ty:ty),*$(,)?}) => {
        impl<'de, Condition: serde::Deserialize<'de>> serde::Deserialize<'de> for $name<Condition> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                #[derive(serde::Deserialize)]
                struct Helper<Condition> {
                    #[serde(rename = "type")]
                    kind: SubscriptionType,
                    version: String,
                    condition:Condition,
                    $($field: $ty,)*
                }

                let helper = Helper::deserialize(deserializer)?;

                let kind = match helper.kind {
                    kind @ SubscriptionType::AutomodMessageHold => {
                        if helper.version == "2" {
                            SubscriptionType::AutomodMessageHoldV2
                        } else {
                            kind
                        }
                    }
                    kind @ SubscriptionType::AutomodMessageUpdate => {
                        if helper.version == "2" {
                            SubscriptionType::AutomodMessageUpdateV2
                        } else {
                            kind
                        }
                    }
                    kind @ SubscriptionType::ChannelModerate => {
                        if helper.version == "2" {
                            SubscriptionType::ChannelModerateV2
                        } else {
                            kind
                        }
                    }
                    _ => helper.kind,
                };

                Ok($name {
                    kind,
                    version: helper.version,
                    condition : helper.condition,
                    $($field: helper.$field,)*
               })
            }
        }
    };
}

macro_rules! impl_de_without_generic_subscription_type_must_have_veasion_and_condition {
    ($name:ident {$($field:ident: $ty:ty),*$(,)?}) => {
        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                #[derive(serde::Deserialize)]
                struct Helper {
                    #[serde(rename = "type")]
                    kind: $crate::twitch::subscription_types::types::SubscriptionType,
                    version: String,
                    $($field: $ty,)*
                }

                let helper = Helper::deserialize(deserializer)?;

                let kind = match helper.kind {
                    kind @ $crate::twitch::subscription_types::types::SubscriptionType::AutomodMessageHold => {
                        if helper.version == "2" {
                            $crate::twitch::subscription_types::types::SubscriptionType::AutomodMessageHoldV2
                        } else {
                            kind
                        }
                    }
                    kind @ $crate::twitch::subscription_types::types::SubscriptionType::AutomodMessageUpdate => {
                        if helper.version == "2" {
                            $crate::twitch::subscription_types::types::SubscriptionType::AutomodMessageUpdateV2
                        } else {
                            kind
                        }
                    }
                    kind @ $crate::twitch::subscription_types::types::SubscriptionType::ChannelModerate => {
                        if helper.version == "2" {
                            $crate::twitch::subscription_types::types::SubscriptionType::ChannelModerateV2
                        } else {
                            kind
                        }
                    }
                    _ => helper.kind,
                };

                Ok($name {
                    kind,
                    version: helper.version,
                    $($field: helper.$field,)*
               })
            }
        }
    };
}

macro_rules! set_condition {
    ($base:expr, $($name:ident: $value:expr),+)=>{
        $(set_condition!(@c $base, $name: $value);)+
    };
    (@c $base:expr, broadcaster_user_id: $value:expr) => {
        $base.set_broadcaster_user_id($value);
    };
    (@c $base:expr, moderator_user_id: $value:expr) => {
        $base.set_moderator_user_id($value);
    };
    (@c $base:expr, broadcaster_id: $value:expr) => {
        $base.set_broadcaster_id($value);
    };
    (@c $base:expr, user_id: $value:expr) => {
        $base.set_user_id($value);
    };
    (@c $base:expr, reward_id: $value:expr) => {
        $base.reward_id($value);
    };
    (@c $base:expr, client_id: $value:expr) => {
        $base.client_id($value);
    };
}

macro_rules! new_request {
    (
    $(#[$meta:meta])*
    $name:ident,
    $type:ident) => {
        $(#[$meta])*
        #[derive(Debug, serde::Serialize)]
        pub struct $name(
            pub  $crate::twitch::subscription_types::request::SubscriptionRequest<
                $crate::twitch::condition::Condition,
            >,
        );

        impl $name {
            pub fn webhook<T: Into<String>>(callback: T, secret: Option<T>) -> Self {
                Self(
                    $crate::twitch::subscription_types::request::SubscriptionRequest::new(
                        $crate::twitch::subscription_types::types::SubscriptionTypes::$type,
                        $crate::twitch::Condition::new(),
                        $crate::twitch::Transport::webhook(callback.into(), secret.map(Into::into)),
                    ),
                )
            }

            pub fn websocket<T: Into<String>>(session_id: T) -> Self {
                Self(
                    $crate::twitch::subscription_types::request::SubscriptionRequest::new(
                        $crate::twitch::subscription_types::types::SubscriptionTypes::$type,
                        $crate::twitch::Condition::new(),
                        $crate::twitch::Transport::websocket(session_id.into()),
                    ),
                )
            }
        }

        impl $crate::twitch::subscription_types::request::IntoSubscriptionRequest for $name {}
    };
    (
    $(#[$meta:meta])*
    $name:ident,
    $type:ident,
    {$($require:ident:$value:expr ),+}) => {
        $(#[$meta])*
        #[derive(Debug, serde::Serialize)]
        pub struct $name(
            pub  $crate::twitch::subscription_types::request::SubscriptionRequest<
                $crate::twitch::condition::Condition,
            >,
        );

        impl $name {
            pub fn webhook<T: Into<String>>($($require:T),+, callback: T, secret: Option<T>) -> Self {
                let mut condition = $crate::twitch::Condition::new();
                set_condition!(condition, $($require: $value),+);

                Self(
                    $crate::twitch::subscription_types::request::SubscriptionRequest::new(
                        $crate::twitch::subscription_types::types::SubscriptionType::$type,
                        condition,
                        $crate::twitch::Transport::webhook(callback.into(), secret.map(Into::into)),
                    ),
                )
            }

            pub fn websocket<T: Into<String>>($($require:T),+, sessin_id: T) -> Self {
                let mut condition = $crate::twitch::Condition::new();
                set_condition!(condition, $($require: $value),+);

                Self(
                    $crate::twitch::subscription_types::request::SubscriptionRequest::new(
                        $crate::twitch::subscription_types::types::SubscriptionType::$type,
                        condition,
                        $crate::twitch::Transport::websocket(sessin_id.into()),
                    ),
                )
            }
        }

        impl $crate::twitch::subscription_types::request::IntoSubscriptionRequest for $name {}
    };
        (
        $(#[$meta:meta])*
        $name:ident, $type:ident, $condition:ident
        $(,{$($require:ident:$value:expr ),+})?
        ) => {
        $(#[$meta])*
        #[derive(Debug, serde::Serialize)]
        pub struct $name(
            pub  $crate::twitch::subscription_types::request::SubscriptionRequest<
                $condition,
            >,
        );

        impl $name {
            pub fn webhook<T: Into<String>>($($($require:T),+,)?callback: T, secret: Option<T>) -> Self {
                Self(
                    $crate::twitch::subscription_types::request::SubscriptionRequest::new(
                        $crate::twitch::subscription_types::types::SubscriptionType::$type,
                        $condition::new($($($value.into()),+)?),
                        $crate::twitch::Transport::webhook(callback.into(), secret.map(Into::into)),
                    ),
                )
            }

            pub fn websocket<T: Into<String>>($($($require:T),+,)?sessin_id: T) -> Self {
                Self(
                    $crate::twitch::subscription_types::request::SubscriptionRequest::new(
                        $crate::twitch::subscription_types::types::SubscriptionType::$type,
                        $condition::new($($($value.into()),+)?),
                        $crate::twitch::Transport::websocket(sessin_id.into()),
                    ),
                )
            }
        }

        impl $crate::twitch::subscription_types::request::IntoSubscriptionRequest for $name {}
    };

}

macro_rules! new_response_payload {
    (
    $(#[$meta:meta])*
    $name:ident,
    $condition:ident,
    $event:ident) => {
        $(#[$meta])*
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        pub struct $name {
            #[serde(flatten)]
            pub payload: $crate::twitch::subscription::EventPayload<$condition, $event>,
        }
    };
}
