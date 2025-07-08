use std::fmt;

use base64::{engine::general_purpose, Engine as _};
use http::HeaderValue;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum AuthScheme<'a> {
    /// Basic authentication - RFC 7617
    /// Credentials: username:password encoded in base64
    /// Security: LOW - credentials are easily decoded
    /// Usage: Simple username/password authentication
    Basic {
        username: &'a str,
        password: &'a str,
    },

    /// Bearer token authentication - RFC 6750  
    /// Credentials: Token (usually JWT or OAuth 2.0 access token)
    /// Security: HIGH - depends on token implementation
    /// Usage: OAuth 2.0, JWT, API tokens
    Bearer { token: &'a str },

    /// Digest authentication - RFC 7616
    /// Credentials: Hashed challenge-response using MD5/SHA
    /// Security: MEDIUM - prevents password transmission, vulnerable to rainbow tables
    /// Usage: Enhanced security over Basic auth
    Digest(DigestBuilder<'a>),

    /// HTTP Origin-Bound Authentication - RFC 7486
    /// Credentials: Digital signature-based
    /// Security: HIGH - not vulnerable to phishing attacks
    /// Usage: Advanced security scenarios, no password storage needed
    HOBA {
        result: String, // Contains "kid"."challenge"."nonce"."sig"
    },

    /// Mutual authentication - RFC 8120
    /// Credentials: Bidirectional authentication
    /// Security: HIGH - both client and server authenticate each other
    /// Usage: High-security environments, certificate-based auth
    Mutual { credentials: &'a str },

    /// Negotiate/NTLM authentication - RFC 4559
    /// Credentials: SPNEGO for Kerberos/NTLM
    /// Security: HIGH - enterprise-grade authentication
    /// Usage: Windows domain authentication, SSO
    Negotiate { token: &'a str },

    /// VAPID authentication - RFC 8292
    /// Credentials: Voluntary Application Server Identification
    /// Security: MEDIUM - for web push notifications
    /// Usage: Web push services, contact information verification
    Vapid {
        public_key: &'a str,
        subject: &'a str,
        signature: String,
    },

    /// SCRAM authentication - RFC 7804
    /// Credentials: SASL mechanisms (SHA-1, SHA-256)
    /// Security: HIGH - salted challenge-response
    /// Usage: Database authentication, secure challenge-response
    Scram {
        variant: SCRAMVariant,
        credentials: String,
    },

    /// AWS Signature Version 4 - AWS documentation
    /// Credentials: HMAC-SHA256 signature
    /// Security: HIGH - signed requests with access keys
    /// Usage: AWS API authentication
    Aws4HmacSha256 {
        access_key: &'a str,
        signature: String,
        region: &'a str,
        service: &'a str,
        date: String,
    },

    /// Custom authentication scheme
    /// Credentials: User-defined
    /// Security: Varies
    /// Usage: Proprietary or non-standard auth schemes
    Custom {
        scheme: &'a str,
        credentials: &'a str,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SCRAMVariant {
    SHA1,
    SHA256,
}

impl<'a> AuthScheme<'a> {
    pub fn basic(username: &'a str, password: &'a str) -> Self {
        Self::Basic { username, password }
    }

    pub fn bearer(token: &'a str) -> Self {
        Self::Bearer { token }
    }

    pub fn digest(digest: DigestBuilder<'a>) -> Self {
        Self::Digest(digest)
    }

    pub fn hoba(result: impl Into<String>) -> Self {
        Self::HOBA {
            result: result.into(),
        }
    }

    pub fn mutual(credentials: &'a str) -> Self {
        Self::Mutual { credentials }
    }

    pub fn negotiate(token: &'a str) -> Self {
        Self::Negotiate { token }
    }

    pub fn vapid(public_key: &'a str, subject: &'a str, signature: impl Into<String>) -> Self {
        Self::Vapid {
            public_key,
            subject,
            signature: signature.into(),
        }
    }

    pub fn scram(variant: SCRAMVariant, credentials: impl Into<String>) -> Self {
        Self::Scram {
            variant,
            credentials: credentials.into(),
        }
    }

    pub fn aws4_hmac_sha256(
        access_key: &'a str,
        signature: impl Into<String>,
        region: &'a str,
        service: &'a str,
        date: impl Into<String>,
    ) -> Self {
        Self::Aws4HmacSha256 {
            access_key,
            signature: signature.into(),
            region,
            service,
            date: date.into(),
        }
    }

    pub fn custom(scheme: &'a str, credentials: &'a str) -> Self {
        Self::Custom {
            scheme,
            credentials,
        }
    }

    pub fn to_header_value(self) -> Result<HeaderValue, AuthError> {
        let auth_string = match self {
            AuthScheme::Basic { username, password } => {
                let credentials = format!("{username}:{password}");
                let encoded = general_purpose::STANDARD.encode(credentials);
                format!("Basic {encoded}")
            }
            AuthScheme::Bearer { token } => format!("Bearer {token}"),
            AuthScheme::Digest(digest) => digest.build(),
            AuthScheme::HOBA { result } => format!("HOBA result=\"{result}\""),
            AuthScheme::Mutual { credentials } => format!("Mutual {credentials}"),
            AuthScheme::Negotiate { token } => format!("Negotiate {token}"),
            AuthScheme::Vapid {
                public_key,
                subject,
                signature,
            } => format!("VAPID k={public_key}, a={subject}, s={signature}"),
            AuthScheme::Scram {
                variant,
                credentials,
            } => {
                let scheme_name = match variant {
                    SCRAMVariant::SHA1 => "SCRAM-SHA-1",
                    SCRAMVariant::SHA256 => "SCRAM-SHA-256",
                };
                format!("{scheme_name} {credentials}")
            }
            AuthScheme::Aws4HmacSha256 {
                access_key,
                signature,
                region,
                service,
                date,
            } =>  format!("AWS4-HMAC-SHA256 Credential={access_key}/{date}/{region}/{service}/aws4_request, SignedHeaders=host;x-amz-date, Signature={signature}"),
            AuthScheme::Custom {
                scheme,
                credentials,
            } => 
                format!("{scheme} {credentials}"),
           
        };

        HeaderValue::from_str(&auth_string)
            .map_err(|e| AuthError::InvalidHeaderValue(e.to_string()))
    }

    pub fn scheme_name(&self) -> &str {
        match self {
            AuthScheme::Basic { .. } => "Basic",
            AuthScheme::Bearer { .. } => "Bearer",
            AuthScheme::Digest { .. } => "Digest",
            AuthScheme::HOBA { .. } => "HOBA",
            AuthScheme::Mutual { .. } => "Mutual",
            AuthScheme::Negotiate { .. } => "Negotiate",
            AuthScheme::Vapid { .. } => "VAPID",
            AuthScheme::Scram { variant, .. } => match variant {
                SCRAMVariant::SHA1 => "SCRAM-SHA-1",
                SCRAMVariant::SHA256 => "SCRAM-SHA-256",
            },
            AuthScheme::Aws4HmacSha256 { .. } => "AWS4-HMAC-SHA256",
            AuthScheme::Custom { scheme, .. } => scheme,
        }
    }
}

impl<'a> fmt::Display for AuthScheme<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthScheme::Basic { username, .. } => write!(f, "Basic (user: {username})"),
            AuthScheme::Bearer { .. } => write!(f, "Bearer token"),
            AuthScheme::Digest(digest) => write!(f, "{digest}"),
            AuthScheme::HOBA { .. } => write!(f, "HOBA"),
            AuthScheme::Mutual { .. } => write!(f, "Mutual"),
            AuthScheme::Negotiate { .. } => write!(f, "Negotiate"),
            AuthScheme::Vapid { subject, .. } => write!(f, "VAPID ({subject})"),
            AuthScheme::Scram { variant, .. } => write!(f, "SCRAM-{variant:?}"),
            AuthScheme::Aws4HmacSha256 {
                region, service, ..
            } => write!(f, "AWS4 ({region}/{service})"),
            AuthScheme::Custom { scheme, .. } => write!(f, "Custom ({scheme})"),
        }
    }
}

impl<'a> fmt::Debug for AuthScheme<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthScheme::Basic { username, password: _ } => f
                .debug_struct("Basic")
                .field("username", username)
                .field("password", &"[REDACTED]")
                .finish(),
            
            AuthScheme::Bearer { token: _ } => f
                .debug_struct("Bearer")
                .field("token", &"[REDACTED]")
                .finish(),
            
            AuthScheme::Digest(digest) => f
                .debug_tuple("Digest")
                .field(digest)
                .finish(),
            
            AuthScheme::HOBA { result: _ } => f
                .debug_struct("HOBA")
                .field("result", &"[REDACTED]")
                .finish(),
            
            AuthScheme::Mutual { credentials: _ } => f
                .debug_struct("Mutual")
                .field("credentials", &"[REDACTED]")
                .finish(),
            
            AuthScheme::Negotiate { token: _ } => f
                .debug_struct("Negotiate")
                .field("token", &"[REDACTED]")
                .finish(),
            
            AuthScheme::Vapid { 
                public_key: _, 
                subject, 
                signature: _ 
            } => f
                .debug_struct("Vapid")
                .field("public_key", &"[REDACTED]")
                .field("subject", subject)
                .field("signature", &"[REDACTED]")
                .finish(),
            
            AuthScheme::Scram { 
                variant, 
                credentials: _ 
            } => f
                .debug_struct("Scram")
                .field("variant", variant)
                .field("credentials", &"[REDACTED]")
                .finish(),
            
            AuthScheme::Aws4HmacSha256 {
                access_key:_,
                signature: _,
                region,
                service,
                date,
            } => f
                .debug_struct("Aws4HmacSha256")
                .field("access_key", &"[REDACTED]")
                .field("signature", &"[REDACTED]")
                .field("region", region)
                .field("service", service)
                .field("date", date)
                .finish(),
            
            AuthScheme::Custom { 
                scheme, 
                credentials: _ 
            } => f
                .debug_struct("Custom")
                .field("scheme", scheme)
                .field("credentials", &"[REDACTED]")
                .finish(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct DigestBuilder<'a> {
    username: &'a str,
    realm: &'a str,
    nonce: &'a str,
    uri: &'a str,
    response: &'a str,

    algorithm: Option<&'a str>,
    cnonce: Option<&'a str>,
    opaque: Option<&'a str>,
    qop: Option<&'a str>,
    nc: Option<&'a str>,
}

impl<'a> DigestBuilder<'a> {
    pub fn new(
        username: &'a str,
        realm: &'a str,
        nonce: &'a str,
        uri: &'a str,
        response: &'a str,
    ) -> Self {
        Self {
            username,
            realm,
            nonce,
            uri,
            response,
            algorithm: None,
            cnonce: None,
            opaque: None,
            qop: None,
            nc: None,
        }
    }

    pub fn algorithm(mut self, algorithm: &'a str) -> Self {
        self.algorithm = Some(algorithm);
        self
    }
    pub fn cnonce(mut self, cnonce: &'a str) -> Self {
        self.cnonce = Some(cnonce);
        self
    }
    pub fn opaque(mut self, opaque: &'a str) -> Self {
        self.opaque = Some(opaque);
        self
    }
    pub fn qop(mut self, qop: &'a str) -> Self {
        self.qop = Some(qop);
        self
    }
    pub fn nc(mut self, nc: &'a str) -> Self {
        self.nc = Some(nc);
        self
    }

    pub fn build(self) -> String {
        let Self {
            username,
            realm,
            nonce,
            uri,
            response,
            algorithm,
            cnonce,
            opaque,
            qop,
            nc,
        } = self;
        let mut parts = vec![
            format!("username=\"{}\"", username),
            format!("realm=\"{}\"", realm),
            format!("nonce=\"{}\"", nonce),
            format!("uri=\"{}\"", uri),
            format!("response=\"{}\"", response),
        ];

        if let Some(alg) = algorithm {
            parts.push(format!("algorithm={alg}"));
        }
        if let Some(cn) = cnonce {
            parts.push(format!("cnonce=\"{cn}\""));
        }
        if let Some(op) = opaque {
            parts.push(format!("opaque=\"{op}\""));
        }
        if let Some(q) = qop {
            parts.push(format!("qop={q}"));
        }
        if let Some(n) = nc {
            parts.push(format!("nc={n}"));
        }

        format!("Digest {}", parts.join(", "))
    }
}

impl fmt::Display for DigestBuilder<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Digest (user: {}, realm: {})", self.username, self.realm)
    }
}

impl<'a> fmt::Debug for DigestBuilder<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DigestBuilder")
            .field("username", &self.username)
            .field("realm", &self.realm)
            .field("nonce", &"[REDACTED]")
            .field("uri", &self.uri)
            .field("response", &"[REDACTED]")
            .field("algorithm", &self.algorithm)
            .field("cnonce", &self.cnonce.map(|_| "[REDACTED]"))
            .field("opaque", &self.opaque.map(|_| "[REDACTED]"))
            .field("qop", &self.qop)
            .field("nc", &self.nc)
            .finish()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(String),
}
