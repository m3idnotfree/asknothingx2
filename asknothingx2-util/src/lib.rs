#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "api")]
pub mod api;
#[cfg(feature = "oauth")]
pub mod oauth;
#[cfg(feature = "serde")]
pub mod serde;
