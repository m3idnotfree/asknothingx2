use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

/// https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubscriptionTypes {
    // Automod related
    AutomodMessageHold,
    AutomodMessageHoldV2,
    AutomodMessageUpdate,
    AutomodMessageUpdateV2,
    AutomodSettingsUpdate,
    AutomodTermsUpdate,

    // Channel related
    ChannelUpdate,
    ChannelFollow,
    ChannelAdBreakBegin,

    // Chat related
    ChannelChatClear,
    ChannelChatClearUserMessages,
    ChannelChatMessage,
    ChannelChatMessageDelete,
    ChannelChatNotification,
    ChannelChatSettingsUpdate,
    ChannelChatUserMessageHold,
    ChannelChatUserMessageUpdate,

    // Shared Chat
    ChannelSharedChatSessionBegin,
    ChannelSharedChatSessionUpdate,
    ChannelSharedChatSessionEnd,

    // Subscription related
    ChannelSubscribe,
    ChannelSubscriptionEnd,
    ChannelSubscriptionGift,
    ChannelSubscriptionMessage,

    // Channel interactions
    ChannelCheer,
    ChannelRaid,

    // Moderation
    ChannelBan,
    ChannelUnban,
    ChannelUnbanRequestCreate,
    ChannelUnbanRequestResolve,
    ChannelModerate,
    ChannelModerateV2,
    ChannelModeratorAdd,
    ChannelModeratorRemove,

    // Guest Star (Beta)
    ChannelGuestStarSessionBegin,
    ChannelGuestStarSessionEnd,
    ChannelGuestStarGuestUpdate,
    ChannelGuestStarSettingsUpdate,

    // Channel Points
    ChannelPointsAutomaticRewardRedemption,
    ChannelPointsCustomRewardAdd,
    ChannelPointsCustomRewardUpdate,
    ChannelPointsCustomRewardRemove,
    ChannelPointsCustomRewardRedemptionAdd,
    ChannelPointsCustomRewardRedemptionUpdate,

    // Polls and Predictions
    ChannelPollBegin,
    ChannelPollProgress,
    ChannelPollEnd,
    ChannelPredictionBegin,
    ChannelPredictionProgress,
    ChannelPredictionLock,
    ChannelPredictionEnd,

    // Suspicious Users
    ChannelSuspiciousUserMessage,
    ChannelSuspiciousUserUpdate,

    // VIP and Warnings
    ChannelVIPAdd,
    ChannelVIPRemove,
    ChannelWarningAcknowledgement,
    ChannelWarningSend,

    // Charity
    CharityDonation,
    CharityCampaignStart,
    CharityCampaignProgress,
    CharityCampaignStop,

    // Conduit
    ConduitShardDisabled,

    // Drops and Extensions
    DropEntitlementGrant,
    ExtensionBitsTransactionCreate,

    // Goals and Hype Train
    GoalBegin,
    GoalProgress,
    GoalEnd,
    HypeTrainBegin,
    HypeTrainProgress,
    HypeTrainEnd,

    // Shield Mode
    ShieldModeBegin,
    ShieldModeEnd,

    // Shoutouts
    ShoutoutCreate,
    ShoutoutReceived,

    // Stream Status
    StreamOnline,
    StreamOffline,

    // User Authorization
    UserAuthorizationGrant,
    UserAuthorizationRevoke,
    UserUpdate,

    // Whispers
    WhisperReceived,
}

impl SubscriptionTypes {
    pub fn as_str(&self) -> &str {
        match self {
            Self::AutomodMessageHold => "automod.message.hold",
            Self::AutomodMessageHoldV2 => "automod.message.hold",
            Self::AutomodMessageUpdate => "automod.message.update",
            Self::AutomodMessageUpdateV2 => "automod.message.update",
            Self::AutomodSettingsUpdate => "automod.settings.update",
            Self::AutomodTermsUpdate => "automod.terms.update",
            Self::ChannelUpdate => "channel.update",
            Self::ChannelFollow => "channel.follow",
            Self::ChannelAdBreakBegin => "channel.ad_break.begin",
            Self::ChannelChatClear => "channel.chat.clear",
            Self::ChannelChatClearUserMessages => "channel.chat.clear_user_messages",
            Self::ChannelChatMessage => "channel.chat.message",
            Self::ChannelChatMessageDelete => "channel.chat.message_delete",
            Self::ChannelChatNotification => "channel.chat.notification",
            Self::ChannelChatSettingsUpdate => "channel.chat_settings.update",
            Self::ChannelChatUserMessageHold => "channel.chat.user_message_hold",
            Self::ChannelChatUserMessageUpdate => "channel.chat.user_message_update",
            Self::ChannelSharedChatSessionBegin => "channel.shared_chat.begin",
            Self::ChannelSharedChatSessionUpdate => "channel.shared_chat.update",
            Self::ChannelSharedChatSessionEnd => "channel.shared_chat.end",
            Self::ChannelSubscribe => "channel.subscribe",
            Self::ChannelSubscriptionEnd => "channel.subscription.end",
            Self::ChannelSubscriptionGift => "channel.subscription.gift",
            Self::ChannelSubscriptionMessage => "channel.subscription.message",
            Self::ChannelCheer => "channel.cheer",
            Self::ChannelRaid => "channel.raid",
            Self::ChannelBan => "channel.ban",
            Self::ChannelUnban => "channel.unban",
            Self::ChannelUnbanRequestCreate => "channel.unban_request.create",
            Self::ChannelUnbanRequestResolve => "channel.unban_request.resolve",
            Self::ChannelModerate => "channel.moderate",
            Self::ChannelModerateV2 => "channel.moderate",
            Self::ChannelModeratorAdd => "channel.moderator.add",
            Self::ChannelModeratorRemove => "channel.moderator.remove",
            Self::ChannelGuestStarSessionBegin => "channel.guest_star_session.begin",
            Self::ChannelGuestStarSessionEnd => "channel.guest_star_session.end",
            Self::ChannelGuestStarGuestUpdate => "channel.guest_star_guest.update",
            Self::ChannelGuestStarSettingsUpdate => "channel.guest_star_settings.update",
            Self::ChannelPointsAutomaticRewardRedemption => {
                "channel.channel_points_automatic_reward_redemption.add"
            }
            Self::ChannelPointsCustomRewardAdd => "channel.channel_points_custom_reward.add",
            Self::ChannelPointsCustomRewardUpdate => "channel.channel_points_custom_reward.update",
            Self::ChannelPointsCustomRewardRemove => "channel.channel_points_custom_reward.remove",
            Self::ChannelPointsCustomRewardRedemptionAdd => {
                "channel.channel_points_custom_reward_redemption.add"
            }
            Self::ChannelPointsCustomRewardRedemptionUpdate => {
                "channel.channel_points_custom_reward_redemption.update"
            }
            Self::ChannelPollBegin => "channel.poll.begin",
            Self::ChannelPollProgress => "channel.poll.progress",
            Self::ChannelPollEnd => "channel.poll.end",
            Self::ChannelPredictionBegin => "channel.prediction.begin",
            Self::ChannelPredictionProgress => "channel.prediction.progress",
            Self::ChannelPredictionLock => "channel.prediction.lock",
            Self::ChannelPredictionEnd => "channel.prediction.end",
            Self::ChannelSuspiciousUserMessage => "channel.suspicious_user.message",
            Self::ChannelSuspiciousUserUpdate => "channel.suspicious_user.update",
            Self::ChannelVIPAdd => "channel.vip.add",
            Self::ChannelVIPRemove => "channel.vip.remove",
            Self::ChannelWarningAcknowledgement => "channel.warning.acknowledge",
            Self::ChannelWarningSend => "channel.warning.send",
            Self::CharityDonation => "channel.charity_campaign.donate",
            Self::CharityCampaignStart => "channel.charity_campaign.start",
            Self::CharityCampaignProgress => "channel.charity_campaign.progress",
            Self::CharityCampaignStop => "channel.charity_campaign.stop",
            Self::ConduitShardDisabled => "conduit.shard.disabled",
            Self::DropEntitlementGrant => "drop.entitlement.grant",
            Self::ExtensionBitsTransactionCreate => "extension.bits_transaction.create",
            Self::GoalBegin => "channel.goal.begin",
            Self::GoalProgress => "channel.goal.progress",
            Self::GoalEnd => "channel.goal.end",
            Self::HypeTrainBegin => "channel.hype_train.begin",
            Self::HypeTrainProgress => "channel.hype_train.progress",
            Self::HypeTrainEnd => "channel.hype_train.end",
            Self::ShieldModeBegin => "channel.shield_mode.begin",
            Self::ShieldModeEnd => "channel.shield_mode.end",
            Self::ShoutoutCreate => "channel.shoutout.create",
            Self::ShoutoutReceived => "channel.shoutout.receive",
            Self::StreamOnline => "stream.online",
            Self::StreamOffline => "stream.offline",
            Self::UserAuthorizationGrant => "user.authorization.grant",
            Self::UserAuthorizationRevoke => "user.authorization.revoke",
            Self::UserUpdate => "user.update",
            Self::WhisperReceived => "user.whisper.message",
        }
    }

    pub fn version(&self) -> &str {
        match self {
            Self::AutomodMessageHoldV2 | Self::AutomodMessageUpdateV2 => "2",
            Self::ChannelUpdate | Self::ChannelFollow => "2",
            Self::ChannelModerateV2 => "2",
            Self::ChannelGuestStarSessionBegin
            | Self::ChannelGuestStarSessionEnd
            | Self::ChannelGuestStarGuestUpdate
            | Self::ChannelGuestStarSettingsUpdate => "beta",
            _ => "1",
        }
    }
}

impl Serialize for SubscriptionTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for SubscriptionTypes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "automod.message.hold" => Ok(Self::AutomodMessageHold),
            // "automod.message.hold" => Ok(Self::AutomodMessageHoldV2),
            "automod.message.update" => Ok(Self::AutomodMessageUpdate),
            // "automod.message.update" => Ok(Self::AutomodMessageUpdateV2),
            "automod.settings.update" => Ok(Self::AutomodSettingsUpdate),
            "automod.terms.update" => Ok(Self::AutomodTermsUpdate),
            "channel.update" => Ok(Self::ChannelUpdate),
            "channel.follow" => Ok(Self::ChannelFollow),
            "channel.ad_break.begin" => Ok(Self::ChannelAdBreakBegin),
            "channel.chat.clear" => Ok(Self::ChannelChatClear),
            "channel.chat.clear_user_messages" => Ok(Self::ChannelChatClearUserMessages),
            "channel.chat.message" => Ok(Self::ChannelChatMessage),
            "channel.chat.message_delete" => Ok(Self::ChannelChatMessageDelete),
            "channel.chat.notification" => Ok(Self::ChannelChatNotification),
            "channel.chat_settings.update" => Ok(Self::ChannelChatSettingsUpdate),
            "channel.chat.user_message_hold" => Ok(Self::ChannelChatUserMessageHold),
            "channel.chat.user_message_update" => Ok(Self::ChannelChatUserMessageUpdate),
            "channel.shared_chat.begin" => Ok(Self::ChannelSharedChatSessionBegin),
            "channel.shared_chat.update" => Ok(Self::ChannelSharedChatSessionUpdate),
            "channel.shared_chat.end" => Ok(Self::ChannelSharedChatSessionEnd),
            "channel.subscribe" => Ok(Self::ChannelSubscribe),
            "channel.subscription.end" => Ok(Self::ChannelSubscriptionEnd),
            "channel.subscription.gift" => Ok(Self::ChannelSubscriptionGift),
            "channel.subscription.message" => Ok(Self::ChannelSubscriptionMessage),
            "channel.cheer" => Ok(Self::ChannelCheer),
            "channel.raid" => Ok(Self::ChannelRaid),
            "channel.ban" => Ok(Self::ChannelBan),
            "channel.unban" => Ok(Self::ChannelUnban),
            "channel.unban_request.create" => Ok(Self::ChannelUnbanRequestCreate),
            "channel.unban_request.resolve" => Ok(Self::ChannelUnbanRequestResolve),
            "channel.moderate" => Ok(Self::ChannelModerate),
            "channel.moderator.add" => Ok(Self::ChannelModeratorAdd),
            "channel.moderator.remove" => Ok(Self::ChannelModeratorRemove),
            "channel.guest_star_session.begin" => Ok(Self::ChannelGuestStarSessionBegin),
            "channel.guest_star_session.end" => Ok(Self::ChannelGuestStarSessionEnd),
            "channel.guest_star_guest.update" => Ok(Self::ChannelGuestStarGuestUpdate),
            "channel.guest_star_settings.update" => Ok(Self::ChannelGuestStarSettingsUpdate),
            "channel.channel_points_automatic_reward_redemption.add" => {
                Ok(Self::ChannelPointsAutomaticRewardRedemption)
            }
            "channel.channel_points_custom_reward.add" => Ok(Self::ChannelPointsCustomRewardAdd),
            "channel.channel_points_custom_reward.update" => {
                Ok(Self::ChannelPointsCustomRewardUpdate)
            }
            "channel.channel_points_custom_reward.remove" => {
                Ok(Self::ChannelPointsCustomRewardRemove)
            }
            "channel.channel_points_custom_reward_redemption.add" => {
                Ok(Self::ChannelPointsCustomRewardRedemptionAdd)
            }
            "channel.channel_points_custom_reward_redemption.update" => {
                Ok(Self::ChannelPointsCustomRewardRedemptionUpdate)
            }
            "channel.poll.begin" => Ok(Self::ChannelPollBegin),
            "channel.poll.progress" => Ok(Self::ChannelPollProgress),
            "channel.poll.end" => Ok(Self::ChannelPollEnd),
            "channel.prediction.begin" => Ok(Self::ChannelPredictionBegin),
            "channel.prediction.progress" => Ok(Self::ChannelPredictionProgress),
            "channel.prediction.lock" => Ok(Self::ChannelPredictionLock),
            "channel.prediction.end" => Ok(Self::ChannelPredictionEnd),
            "channel.suspicious_user.message" => Ok(Self::ChannelSuspiciousUserMessage),
            "channel.suspicious_user.update" => Ok(Self::ChannelSuspiciousUserUpdate),
            "channel.vip.add" => Ok(Self::ChannelVIPAdd),
            "channel.vip.remove" => Ok(Self::ChannelVIPRemove),
            "channel.warning.acknowledge" => Ok(Self::ChannelWarningAcknowledgement),
            "channel.warning.send" => Ok(Self::ChannelWarningSend),
            "channel.charity_campaign.donate" => Ok(Self::CharityDonation),
            "channel.charity_campaign.start" => Ok(Self::CharityCampaignStart),
            "channel.charity_campaign.progress" => Ok(Self::CharityCampaignProgress),
            "channel.charity_campaign.stop" => Ok(Self::CharityCampaignStop),
            "conduit.shard.disabled" => Ok(Self::ConduitShardDisabled),
            "drop.entitlement.grant" => Ok(Self::DropEntitlementGrant),
            "extension.bits_transaction.create" => Ok(Self::ExtensionBitsTransactionCreate),
            "channel.goal.begin" => Ok(Self::GoalBegin),
            "channel.goal.progress" => Ok(Self::GoalProgress),
            "channel.goal.end" => Ok(Self::GoalEnd),
            "channel.hype_train.begin" => Ok(Self::HypeTrainBegin),
            "channel.hype_train.progress" => Ok(Self::HypeTrainProgress),
            "channel.hype_train.end" => Ok(Self::HypeTrainEnd),
            "channel.shield_mode.begin" => Ok(Self::ShieldModeBegin),
            "channel.shield_mode.end" => Ok(Self::ShieldModeEnd),
            "channel.shoutout.create" => Ok(Self::ShoutoutCreate),
            "channel.shoutout.receive" => Ok(Self::ShoutoutReceived),
            "stream.online" => Ok(Self::StreamOnline),
            "stream.offline" => Ok(Self::StreamOffline),
            "user.authorization.grant" => Ok(Self::UserAuthorizationGrant),
            "user.authorization.revoke" => Ok(Self::UserAuthorizationRevoke),
            "user.update" => Ok(Self::UserUpdate),
            "user.whisper.message" => Ok(Self::WhisperReceived),
            _ => Err(D::Error::custom(format!(
                "unknown subscription type: '{}'",
                s
            ))),
        }
    }
}

#[macro_export]
macro_rules! impl_subscription_type_deserialize {
    ($name:ident, $($field:ident: $ty:ty),*$(,)?) => {
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
