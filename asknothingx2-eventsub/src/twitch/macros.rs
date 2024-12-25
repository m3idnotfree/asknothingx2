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
                    kind: SubscriptionTypes,
                    version: String,
                    condition:Condition,
                    $($field: $ty,)*
                }

                let helper = Helper::deserialize(deserializer)?;

                let kind = match helper.kind {
                    kind @ SubscriptionTypes::AutomodMessageHold => {
                        if helper.version == "2" {
                            SubscriptionTypes::AutomodMessageHoldV2
                        } else {
                            kind
                        }
                    }
                    kind @ SubscriptionTypes::AutomodMessageUpdate => {
                        if helper.version == "2" {
                            SubscriptionTypes::AutomodMessageUpdateV2
                        } else {
                            kind
                        }
                    }
                    kind @ SubscriptionTypes::ChannelModerate => {
                        if helper.version == "2" {
                            SubscriptionTypes::ChannelModerateV2
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
                    kind: SubscriptionTypes,
                    version: String,
                    $($field: $ty,)*
                }

                let helper = Helper::deserialize(deserializer)?;

                let kind = match helper.kind {
                    kind @ SubscriptionTypes::AutomodMessageHold => {
                        if helper.version == "2" {
                            SubscriptionTypes::AutomodMessageHoldV2
                        } else {
                            kind
                        }
                    }
                    kind @ SubscriptionTypes::AutomodMessageUpdate => {
                        if helper.version == "2" {
                            SubscriptionTypes::AutomodMessageUpdateV2
                        } else {
                            kind
                        }
                    }
                    kind @ SubscriptionTypes::ChannelModerate => {
                        if helper.version == "2" {
                            SubscriptionTypes::ChannelModerateV2
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
