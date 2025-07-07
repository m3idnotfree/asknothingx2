use std::{fmt, io, path::PathBuf, time::Duration};


#[derive(Debug, thiserror::Error)]
pub enum StreamError {
    #[error("IO error during {operation}: {source}")]
    Io {
        operation: String,
        #[source]
        source: io::Error,
    },

    #[error("File {operation} error for '{path}': {source}")]
    File {
        path: PathBuf,
        operation: FileOperation,
        #[source]
        source: io::Error,
    },

    #[error("Network {operation} error for '{address}': {source}")]
    Network {
        address: String,
        operation: NetworkOperation,
        #[source]
        source: io::Error,
    },

    #[error("Process {operation} error for '{command}': {source}")]
    Process {
        command: String,
        operation: ProcessOperation,
        #[source]
        source: io::Error,
    },

    #[error("Stream {operation} error: {source}")]
    Stream {
        operation: StreamOperation,
        #[source]
        source: io::Error,
    },

    #[error("Codec '{codec_name}' error{}: {source}", position_display(.position))]
    Codec {
        codec_name: String,
        position: Option<u64>,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("{limit_type} limit exceeded: {actual} > {max_allowed}")]
    Limit {
        limit_type: LimitType,
        actual: u64,
        max_allowed: u64,
    },

    #[error("Timeout during '{operation}' after {duration:?}")]
    Timeout {
        operation: String,
        duration: Duration,
    },

    #[error("HTTP error: {source}")]
    Http {
        #[source]
        source: reqwest::Error,
    },

    #[error("Content type detection failed{}{}: {reason}", 
        path_display(.path), 
        extension_display(.extension)
    )]
    ContentType {
        path: Option<PathBuf>,
        extension: Option<String>,
        reason: String,
    },

    #[error("Failed to convert from '{from_type}' to '{to_type}': {reason}")]
    Conversion {
        from_type: String,
        to_type: String,
        reason: String,
    },

    #[error("{message}")]
    Custom {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("JSON error: {source}")]
    Json {
        #[source]
        source: serde_json::Error,
    },

    #[error("URL parsing error: {source}")]
    Url {
        #[source]
        source: url::ParseError,
    },

    #[error("UTF-8 conversion error: {source}")]
    Utf8 {
        #[source]
        source: std::str::Utf8Error,
    },

    #[error("Stream ended unexpectedly")]
    EndOfStream,

    #[error("Stream operation was cancelled")]
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileOperation {
    Open,
    Read,
    Write,
    Seek,
    Metadata,
    Create,
    Delete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkOperation {
    Connect,
    Read,
    Write,
    Bind,
    Listen,
    Accept,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessOperation {
    Spawn,
    Wait,
    Kill,
    ReadStdout,
    ReadStderr,
    WriteStdin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamOperation {
    Read,
    Write,
    Transform,
    Buffer,
    Flush,
    Close,
    Yield,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LimitType {
    FileSize,
    StreamLength,
    BufferSize,
    ChunkSize,
    Duration,
    Bandwidth,
}


impl fmt::Display for FileOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Display for NetworkOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Display for ProcessOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Display for StreamOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Display for LimitType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FileOperation {
    pub const fn as_str(&self) -> &'static str {
        match self {
            FileOperation::Open => "open",
            FileOperation::Read => "read",
            FileOperation::Write => "write",
            FileOperation::Seek => "seek",
            FileOperation::Metadata => "metadata",
            FileOperation::Create => "create",
            FileOperation::Delete => "delete",
        }
    }
}

impl NetworkOperation {
    pub const fn as_str(&self) -> &'static str {
        match self {
            NetworkOperation::Connect => "connect",
            NetworkOperation::Read => "read",
            NetworkOperation::Write => "write",
            NetworkOperation::Bind => "bind",
            NetworkOperation::Listen => "listen",
            NetworkOperation::Accept => "accept",
        }
    }
}

impl ProcessOperation {
    pub const fn as_str(&self) -> &'static str {
        match self {
            ProcessOperation::Spawn => "spawn",
            ProcessOperation::Wait => "wait",
            ProcessOperation::Kill => "kill",
            ProcessOperation::ReadStdout => "read_stdout",
            ProcessOperation::ReadStderr => "read_stderr",
            ProcessOperation::WriteStdin => "write_stdin",
        }
    }
}

impl StreamOperation {
    pub const fn as_str(&self) -> &'static str {
        match self {
            StreamOperation::Read => "read",
            StreamOperation::Write => "write",
            StreamOperation::Transform => "transform",
            StreamOperation::Buffer => "buffer",
            StreamOperation::Flush => "flush",
            StreamOperation::Close => "close",
            StreamOperation::Yield => "yield",
        }
    }
}

impl LimitType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            LimitType::FileSize => "file size",
            LimitType::StreamLength => "stream length",
            LimitType::BufferSize => "buffer size",
            LimitType::ChunkSize => "chunk size",
            LimitType::Duration => "duration",
            LimitType::Bandwidth => "bandwidth",
        }
    }
}


fn position_display(position: &Option<u64>) -> String {
    match position {
        Some(pos) => format!(" at position {pos}" ),
        None => String::new(),
    }
}

fn path_display(path: &Option<PathBuf>) -> String {
    match path {
        Some(p) => format!(" for '{}'", p.display()),
        None => String::new(),
    }
}

fn extension_display(extension: &Option<String>) -> String {
    match extension {
        Some(ext) => format!(" (extension: {ext})" ),
        None => String::new(),
    }
}

impl From<std::io::Error> for StreamError {
    fn from(err: std::io::Error) -> Self {
        StreamError::Io {
            operation: "unknown".to_string(),
            source: err,
        }
    }
}

impl From<reqwest::Error> for StreamError {
    fn from(err: reqwest::Error) -> Self {
        StreamError::Http { source: err }
    }
}

impl From<serde_json::Error> for StreamError {
    fn from(err: serde_json::Error) -> Self {
        StreamError::Json { source: err }
    }
}

impl From<url::ParseError> for StreamError {
    fn from(err: url::ParseError) -> Self {
        StreamError::Url { source: err }
    }
}

impl From<std::str::Utf8Error> for StreamError {
    fn from(err: std::str::Utf8Error) -> Self {
        StreamError::Utf8 { source: err }
    }
}


impl StreamError {
    pub fn io_error<O: Into<String>>(operation: O, source: std::io::Error) -> Self {
        Self::Io {
            operation: operation.into(),
            source,
        }
    }

    pub fn file_error<P: Into<PathBuf>>(
        path: P,
        operation: FileOperation,
        source: std::io::Error,
    ) -> Self {
        Self::File {
            path: path.into(),
            operation,
            source,
        }
    }

    pub fn network_error<A: Into<String> >(
        address: A,
        operation: NetworkOperation,
        source: io::Error,
    ) -> Self {
        Self::Network {
            address: address.into(),
            operation,
            source ,
        }
    }

    pub fn timeout_error<O: Into<String>>(operation: O, duration: Duration) -> Self {
        Self::Timeout {
            operation: operation.into(),
            duration,
        }
    }

    pub fn custom<M: Into<String>>(message: M) -> Self {
        Self::Custom {
            message: message.into(),
            source: None,
        }
    }

    pub fn size_limit_error(limit_type: LimitType, actual: u64, max_allowed: u64) -> Self {
        Self::Limit {
            limit_type,
            actual,
            max_allowed,
        }
    }

    pub fn is_recoverable(&self) -> bool {
        match self {
            StreamError::Timeout { .. } => true,
            StreamError::Network { .. } => true,
            StreamError::Http { source } => source.is_timeout() || source.is_connect(),
            StreamError::Io { source, .. } => {
                matches!(
                    source.kind(),
                    std::io::ErrorKind::Interrupted
                        | std::io::ErrorKind::WouldBlock
                        | std::io::ErrorKind::TimedOut
                )
            }
            StreamError::Limit { .. } => false,
            StreamError::Cancelled => true,
            _ => false,
        }
    }

    pub fn is_temporary(&self) -> bool {
        match self {
            StreamError::Timeout { .. } => true,
            StreamError::Network { .. } => true,
            StreamError::Http { source } => {
                source.is_timeout() 
                    || source.is_connect() 
                    || source.status().is_some_and( |s| s.is_server_error())
            }
            StreamError::Io { source, .. } => {
                matches!(
                    source.kind(),
                    std::io::ErrorKind::Interrupted
                        | std::io::ErrorKind::WouldBlock
                        | std::io::ErrorKind::TimedOut
                        | std::io::ErrorKind::ConnectionRefused
                        | std::io::ErrorKind::ConnectionReset
                )
            }
            StreamError::Cancelled => true,
            _ => false,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum HeaderError {
    #[error("Invalid header name '{name}': {reason}")]
    InvalidHeaderName { name: String, reason: String },

    #[error("Invalid header value for '{name}' = '{value}': {reason}")]
    InvalidHeaderValue { 
        name: String, 
        value: String, 
        reason: String 
    },

    #[error("Encoding failed for header '{name}': {reason}")]
    EncodingFailed { name: String, reason: String },
    #[error("Header value contains invalid UTF-8: {reason}")]
    InvalidUtf8 { 
        reason: String 
    },
}
