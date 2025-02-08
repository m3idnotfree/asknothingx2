use asknothingx2_util::api::HeaderMap;
use hmac::{Hmac, Mac};
use rand::Rng;
use sha2::Sha256;
use std::fmt::{self, Write};

use crate::twitch::error::HeaderVerificationError;

const TWITCH_MESSAGE_ID: &str = "twitch-eventsub-message-id";
const TWITCH_MESSAGE_TIMESTAMP: &str = "twitch-eventsub-message-timestamp";
const TWITCH_MESSAGE_SIGNATURE: &str = "twitch-eventsub-message-signature";

pub struct Secret(Vec<u8>);
impl Secret {
    pub fn new() -> Self {
        Self::new_random_bytes()
    }
    fn new_random_bytes() -> Self {
        let mut rng = rand::rng();
        let random_bytes: Vec<u8> = (0..32).map(|_| rng.random::<u8>()).collect();

        Self(random_bytes)
    }

    pub fn hex_encode(&self) -> String {
        self.0.iter().fold(String::new(), |mut output, b| {
            let _ = write!(output, "{b:02X}");
            output
        })
    }

    pub fn verify_header(
        headers: &HeaderMap,
    ) -> Result<(&str, &str, &str), HeaderVerificationError> {
        let message_id = headers
            .get(TWITCH_MESSAGE_ID)
            .ok_or(HeaderVerificationError::MissingHeader(TWITCH_MESSAGE_ID))?;
        let message_id =
            message_id
                .to_str()
                .map_err(|_| HeaderVerificationError::InvalidHeaderValue {
                    name: TWITCH_MESSAGE_ID,
                    value: message_id.as_bytes().escape_ascii().to_string(),
                })?;

        let timestamp =
            headers
                .get(TWITCH_MESSAGE_TIMESTAMP)
                .ok_or(HeaderVerificationError::MissingHeader(
                    TWITCH_MESSAGE_TIMESTAMP,
                ))?;
        let timestamp =
            timestamp
                .to_str()
                .map_err(|_| HeaderVerificationError::InvalidHeaderValue {
                    name: TWITCH_MESSAGE_TIMESTAMP,
                    value: timestamp.as_bytes().escape_ascii().to_string(),
                })?;

        let signature =
            headers
                .get(TWITCH_MESSAGE_SIGNATURE)
                .ok_or(HeaderVerificationError::MissingHeader(
                    TWITCH_MESSAGE_SIGNATURE,
                ))?;
        let signature =
            signature
                .to_str()
                .map_err(|_| HeaderVerificationError::InvalidHeaderValue {
                    name: TWITCH_MESSAGE_SIGNATURE,
                    value: signature.as_bytes().escape_ascii().to_string(),
                })?;

        Ok((message_id, timestamp, signature))
    }

    pub fn verify_twitch_signature(
        &self,
        headers: &HeaderMap,
        body: &[u8],
    ) -> Result<(), HeaderVerificationError> {
        type HmacSha256 = Hmac<Sha256>;

        let (message_id, timestamp, signature) = Self::verify_header(headers)?;

        let signature = signature.strip_prefix("sha256=").unwrap_or(signature);

        let signature_bytes = hex::decode(signature)?;

        let message = format!(
            "{}{}{}",
            message_id,
            timestamp,
            String::from_utf8_lossy(body)
        );

        let mut mac = HmacSha256::new_from_slice(self.hex_encode().as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(message.as_bytes());

        mac.verify_slice(&signature_bytes)?;

        Ok(())
    }
}

impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, concat!(stringify!(Secret), "([redacted])"))
    }
}

impl Default for Secret {
    fn default() -> Self {
        Self::new_random_bytes()
    }
}
