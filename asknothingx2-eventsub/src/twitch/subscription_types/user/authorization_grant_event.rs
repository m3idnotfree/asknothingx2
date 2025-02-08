use asknothingx2_util::oauth::ClientId;
use serde::Deserialize;
use twitch_highway::types::UserId;

#[derive(Debug, Deserialize)]
pub struct AuthorizationGrantEvent {
    pub client_id: ClientId,
    pub user_id: UserId,
    pub user_login: Option<String>,
    pub user_name: Option<String>,
}
