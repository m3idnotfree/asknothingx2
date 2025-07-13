use std::{fmt, io, pin::Pin};

use bytes::Bytes;
use reqwest::{multipart, Body, RequestBuilder};

use crate::api::error::{self, Error};

pub enum RequestBody {
    Static(&'static str),
    String(String),
    Bytes(Bytes),
    Vec(Vec<u8>),
    Form(Vec<(String, String)>),
    Multipart(reqwest::multipart::Form),
    Json(serde_json::Value),
    Custom(Body),
    Empty,

    #[cfg(feature = "stream")]
    File(tokio::fs::File),
    #[cfg(feature = "stream")]
    BufferedFile {
        file: tokio::fs::File,
        buffer_size: usize,
    },
    #[cfg(feature = "stream")]
    AsyncRead(Pin<Box<dyn tokio::io::AsyncRead + Send + Sync>>),
    #[cfg(feature = "stream")]
    TcpStream(tokio::net::TcpStream),
    #[cfg(feature = "stream")]
    ProcessOutput {
        command: String,
        child: tokio::process::Child,
        stdout: Pin<Box<dyn tokio::io::AsyncRead + Send + Sync>>,
    },

    #[cfg(feature = "stream")]
    Stream(Pin<Box<dyn futures_util::Stream<Item = Result<Bytes, Error>> + Send + Sync>>),
    #[cfg(feature = "stream")]
    IoStream(
        Pin<Box<dyn futures_util::Stream<Item = Result<Bytes, std::io::Error>> + Send + Sync>>,
    ),
    #[cfg(feature = "stream")]
    BytesIterator(Vec<Bytes>),
    #[cfg(feature = "stream")]
    CodecReader {
        reader: Pin<Box<dyn tokio::io::AsyncRead + Send + Sync>>,
        codec_type: CodecType,
    },
}

impl RequestBody {
    pub const fn from_static(s: &'static str) -> Self {
        Self::Static(s)
    }

    pub fn from_string(s: impl Into<String>) -> Self {
        Self::String(s.into())
    }

    pub fn from_bytes(bytes: Bytes) -> Self {
        Self::Bytes(bytes)
    }

    pub fn from_vec(vec: Vec<u8>) -> Self {
        Self::Vec(vec)
    }

    pub fn from_json(value: serde_json::Value) -> Self {
        Self::Json(value)
    }

    pub fn from_json_serializable<T: serde::Serialize>(value: &T) -> Result<Self, Error> {
        let json_value =
            serde_json::to_value(value).map_err(error::serialization::json_generate)?;
        Ok(Self::Json(json_value))
    }

    pub fn from_form(form: Vec<(String, String)>) -> Self {
        Self::Form(form)
    }

    pub fn from_form_pairs<I, K, V>(pairs: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        let form: Vec<(String, String)> = pairs
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        Self::Form(form)
    }

    pub fn from_multipart(form: multipart::Form) -> Self {
        Self::Multipart(form)
    }

    pub const fn empty() -> Self {
        Self::Empty
    }

    pub fn from_reqwest_body(body: Body) -> Self {
        Self::Custom(body)
    }

    #[cfg(feature = "stream")]
    pub fn from_file(file: tokio::fs::File) -> Self {
        Self::File(file)
    }

    #[cfg(feature = "stream")]
    pub fn from_file_buffered(file: tokio::fs::File, buffer_size: usize) -> Self {
        Self::BufferedFile { file, buffer_size }
    }

    #[cfg(feature = "stream")]
    pub fn from_async_read<R>(reader: R) -> Self
    where
        R: tokio::io::AsyncRead + Send + Sync + 'static,
    {
        Self::AsyncRead(Box::pin(reader))
    }

    #[cfg(feature = "stream")]
    pub async fn from_file_path<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Error> {
        use tokio::fs::File;

        let path_ref = path.as_ref();
        let file = File::open(path_ref).await.map_err(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                error::io::file_not_found(path_ref)
            } else if e.kind() == io::ErrorKind::PermissionDenied {
                error::io::permission_denied(path_ref)
            } else {
                error::io::operation(e)
            }
        })?;
        Ok(Self::from_file(file))
    }

    #[cfg(feature = "stream")]
    pub async fn from_file_path_buffered<P: AsRef<std::path::Path>>(
        path: P,
        buffer_size: usize,
    ) -> Result<Self, Error> {
        use tokio::fs::File;

        if buffer_size == 0 {
            return Err(error::config::invalid("buffer size cannot be zero"));
        }

        let path_ref = path.as_ref();
        let file = File::open(path_ref).await.map_err(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                error::io::file_not_found(path_ref)
            } else if e.kind() == io::ErrorKind::PermissionDenied {
                error::io::permission_denied(path_ref)
            } else {
                error::io::operation(e)
            }
        })?;
        Ok(Self::from_file_buffered(file, buffer_size))
    }

    #[cfg(feature = "stream")]
    pub fn from_tcp_stream(tcp: tokio::net::TcpStream) -> Self {
        Self::TcpStream(tcp)
    }

    #[cfg(feature = "stream")]
    pub fn from_command_output(mut command: tokio::process::Command) -> Result<Self, Error> {
        use std::{io, process::Stdio};

        let command_str = format!("{command:?}");
        let mut child = command
            .stdout(Stdio::piped())
            .spawn()
            .map_err(error::process::spawn)?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| error::process::spawn(io::Error::other("failed to capture stdout")))?;

        Ok(Self::ProcessOutput {
            command: command_str,
            child,
            stdout: Box::pin(stdout),
        })
    }

    #[cfg(feature = "stream")]
    pub fn from_stream<S>(stream: S) -> Self
    where
        S: futures_util::Stream<Item = Result<Bytes, Error>> + Send + Sync + 'static,
    {
        Self::Stream(Box::pin(stream))
    }

    #[cfg(feature = "stream")]
    pub fn from_io_stream<S>(stream: S) -> Self
    where
        S: futures_util::Stream<Item = Result<Bytes, std::io::Error>> + Send + Sync + 'static,
    {
        Self::IoStream(Box::pin(stream))
    }

    #[cfg(feature = "stream")]
    pub fn from_bytes_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Bytes>,
    {
        Self::BytesIterator(iter.into_iter().collect())
    }

    #[cfg(feature = "stream")]
    pub fn from_framed_read<R>(reader: R, codec_type: CodecType) -> Self
    where
        R: tokio::io::AsyncRead + Send + Sync + 'static,
    {
        Self::CodecReader {
            reader: Box::pin(reader),
            codec_type,
        }
    }

    #[cfg(feature = "stream")]
    pub fn from_bytes_framed<R>(reader: R) -> Self
    where
        R: tokio::io::AsyncRead + Send + Sync + 'static,
    {
        Self::from_framed_read(reader, CodecType::Bytes)
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            Self::Static(s) => s.is_empty(),
            Self::String(s) => s.is_empty(),
            Self::Bytes(b) => b.is_empty(),
            Self::Vec(v) => v.is_empty(),
            Self::Form(f) => f.is_empty(),
            Self::Json(j) => j.is_null(),
            #[cfg(feature = "stream")]
            Self::BytesIterator(bytes) => bytes.is_empty(),
            _ => false,
        }
    }

    pub fn content_length(&self) -> Option<u64> {
        match self {
            Self::Empty => Some(0),
            Self::Static(s) => Some(s.len() as u64),
            Self::String(s) => Some(s.len() as u64),
            Self::Bytes(b) => Some(b.len() as u64),
            Self::Vec(v) => Some(v.len() as u64),
            Self::Json(j) => serde_json::to_string(j).ok().map(|s| s.len() as u64),
            Self::Form(f) => {
                let encoded = serde_urlencoded::to_string(f).ok()?;
                Some(encoded.len() as u64)
            }
            #[cfg(feature = "stream")]
            Self::BytesIterator(bytes) => Some(bytes.iter().map(|b| b.len() as u64).sum()),
            _ => None,
        }
    }

    pub fn has_known_length(&self) -> bool {
        self.content_length().is_some()
    }

    pub fn body_type(&self) -> &'static str {
        match self {
            Self::Static(_) => "static",
            Self::String(_) => "string",
            Self::Bytes(_) => "bytes",
            Self::Vec(_) => "vector",
            Self::Json(_) => "json",
            Self::Form(_) => "form",
            Self::Multipart(_) => "multipart",
            Self::Custom(_) => "custom",
            Self::Empty => "empty",
            #[cfg(feature = "stream")]
            Self::File(_) => "file",
            #[cfg(feature = "stream")]
            Self::BufferedFile { .. } => "buffered_file",
            #[cfg(feature = "stream")]
            Self::AsyncRead(_) => "async_read",
            #[cfg(feature = "stream")]
            Self::TcpStream(_) => "tcp_stream",
            #[cfg(feature = "stream")]
            Self::ProcessOutput { .. } => "process_output",
            #[cfg(feature = "stream")]
            Self::Stream(_) => "stream",
            #[cfg(feature = "stream")]
            Self::IoStream(_) => "io_stream",
            #[cfg(feature = "stream")]
            Self::BytesIterator(_) => "bytes_iterator",
            #[cfg(feature = "stream")]
            Self::CodecReader { .. } => "codec_reader",
        }
    }

    pub fn into_reqwest_body(self, client: RequestBuilder) -> Result<RequestBuilder, Error> {
        let builder = match self {
            Self::Static(s) => client.body(s),
            Self::String(s) => client.body(s),
            Self::Bytes(b) => client.body(b),
            Self::Vec(v) => client.body(v),
            Self::Json(j) => {
                let json_string =
                    serde_json::to_string(&j).map_err(error::serialization::json_generate)?;
                client.body(json_string)
            }
            Self::Form(f) => {
                let form_string =
                    serde_urlencoded::to_string(&f).map_err(error::serialization::url_encode)?;
                client.body(form_string)
            }
            Self::Multipart(m) => client.multipart(m),
            Self::Custom(b) => client.body(b),
            Self::Empty => client.body(""),

            #[cfg(feature = "stream")]
            Self::File(file) => {
                use futures_util::TryStreamExt;
                use tokio_util::io::ReaderStream;

                let stream = ReaderStream::new(file);
                let stream = stream.map_err(error::io::operation);
                client.body(Body::wrap_stream(stream))
            }

            #[cfg(feature = "stream")]
            Self::BufferedFile { file, buffer_size } => {
                use futures_util::TryStreamExt;
                use tokio::io::BufReader;
                use tokio_util::io::ReaderStream;

                let buffered_reader = BufReader::with_capacity(buffer_size, file);
                let stream = ReaderStream::new(buffered_reader);
                let stream = stream.map_err(error::io::operation);
                client.body(Body::wrap_stream(stream))
            }

            #[cfg(feature = "stream")]
            Self::AsyncRead(reader) => {
                use futures_util::TryStreamExt;
                use tokio_util::io::ReaderStream;

                let stream = ReaderStream::new(reader);
                let stream = stream.map_err(error::io::operation);
                client.body(Body::wrap_stream(stream))
            }

            #[cfg(feature = "stream")]
            Self::TcpStream(tcp) => {
                use futures_util::TryStreamExt;
                use tokio_util::io::ReaderStream;

                let stream = ReaderStream::new(tcp);
                let stream = stream.map_err(error::network::connect);
                client.body(Body::wrap_stream(stream))
            }

            #[cfg(feature = "stream")]
            Self::ProcessOutput { stdout, .. } => {
                use futures_util::TryStreamExt;
                use tokio_util::io::ReaderStream;

                let stream = ReaderStream::new(stdout);
                let stream = stream.map_err(error::process::spawn);
                client.body(Body::wrap_stream(stream))
            }

            #[cfg(feature = "stream")]
            Self::Stream(s) => client.body(Body::wrap_stream(s)),
            #[cfg(feature = "stream")]
            Self::IoStream(stream) => {
                use futures_util::TryStreamExt;

                let stream = stream.map_err(error::io::operation);
                client.body(Body::wrap_stream(stream))
            }

            #[cfg(feature = "stream")]
            Self::BytesIterator(bytes) => {
                use futures_util::stream;

                let iter_stream = stream::iter(bytes.into_iter().map(Ok::<Bytes, Error>));
                client.body(Body::wrap_stream(iter_stream))
            }

            #[cfg(feature = "stream")]
            Self::CodecReader { reader, codec_type } => {
                use futures_util::{StreamExt, TryStreamExt};
                use std::io;
                use tokio_util::codec::{BytesCodec, FramedRead, LinesCodec, LinesCodecError};

                match codec_type {
                    CodecType::Bytes => {
                        let framed = FramedRead::new(reader, BytesCodec::new());
                        let stream = framed.map_err(error::io::operation);
                        client.body(Body::wrap_stream(stream))
                    }
                    CodecType::Lines => {
                        let framed = FramedRead::new(reader, LinesCodec::new());
                        let stream = framed.map(|result| match result {
                            Ok(line) => Ok(Bytes::from(line + "\n")),
                            Err(lines_error) => {
                                let io_error = match lines_error {
                                    LinesCodecError::MaxLineLengthExceeded => {
                                        io::Error::new(io::ErrorKind::InvalidData, "Line too long")
                                    }
                                    LinesCodecError::Io(e) => e,
                                };
                                Err(error::io::operation(io_error))
                            }
                        });
                        client.body(Body::wrap_stream(stream))
                    }
                    CodecType::Json => {
                        let framed = FramedRead::new(reader, LinesCodec::new());
                        let stream = framed.map(|result| {
                            result
                                .map(|line| Bytes::from(line + "\n"))
                                .map_err(|e| error::io::operation(io::Error::other(e)))
                        });
                        client.body(Body::wrap_stream(stream))
                    }
                    CodecType::Custom(name) => {
                        let framed = FramedRead::new(reader, BytesCodec::new());
                        let stream = framed.map_err(move |e| {
                            error::io::operation(io::Error::other(format!(
                                "Custom codec '{name}' error: {e}",
                            )))
                        });
                        client.body(Body::wrap_stream(stream))
                    }
                }
            }
        };
        Ok(builder)
    }
}

impl fmt::Display for RequestBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty { .. } => write!(f, "empty body"),

            Self::Static(s) => write!(f, "static ({} bytes)", s.len()),
            Self::String(s) => write!(f, "string ({} bytes)", s.len()),
            Self::Bytes(b) => write!(f, "bytes ({} bytes)", b.len()),
            Self::Vec(v) => write!(f, "binary ({} bytes)", v.len()),
            Self::Json(_) => write!(f, "JSON"),
            Self::Form(data) => write!(f, "form ({} fields)", data.len()),
            Self::Multipart(_) => write!(f, "multipart"),
            Self::Custom(_) => write!(f, "custom"),

            #[cfg(feature = "stream")]
            Self::File { .. } => write!(f, "file stream"),
            #[cfg(feature = "stream")]
            Self::BufferedFile { buffer_size, .. } => write!(f, "buffered file ({buffer_size}B)"),
            #[cfg(feature = "stream")]
            Self::AsyncRead(..) => write!(f, "async reader"),
            #[cfg(feature = "stream")]
            Self::TcpStream(_) => write!(f, "TCP stream"),
            #[cfg(feature = "stream")]
            Self::ProcessOutput { command, .. } => {
                write!(f, "process output: {}", truncate_command(command))
            }
            #[cfg(feature = "stream")]
            Self::Stream { .. } => write!(f, "stream"),
            #[cfg(feature = "stream")]
            Self::IoStream { .. } => write!(f, "I/O stream"),
            #[cfg(feature = "stream")]
            Self::BytesIterator(chunks) => {
                let total: usize = chunks.iter().map(|b| b.len()).sum();
                write!(
                    f,
                    "bytes iterator ({} chunks, {} bytes)",
                    chunks.len(),
                    total
                )
            }
            #[cfg(feature = "stream")]
            Self::CodecReader { codec_type, .. } => write!(f, "{codec_type} codec"),
        }
    }
}

impl fmt::Debug for RequestBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Static(s) => f
                .debug_tuple("Static")
                .field(&format!("{}B: {}", s.len(), safe_debug_preview(s)))
                .finish(),

            Self::String(s) => f
                .debug_tuple("String")
                .field(&format!("{}B/{} cap", s.len(), s.capacity()))
                .finish(),

            Self::Bytes(b) => f
                .debug_tuple("Bytes")
                .field(&format!("{}B", b.len()))
                .finish(),

            Self::Vec(v) => f
                .debug_tuple("Vec")
                .field(&format!("{}B/{} cap", v.len(), v.capacity()))
                .finish(),
            Self::Json(j) => {
                let preview = serde_json::to_string(j)
                    .map(|s| safe_debug_preview(&s))
                    .unwrap_or_else(|_| "[invalid]".to_string());
                f.debug_tuple("Json").field(&preview).finish()
            }

            Self::Form(data) => {
                let safe_form: Vec<_> = data
                    .iter()
                    .take(3)
                    .map(|(k, v)| {
                        (if is_sensitive_key(k) {
                            (k.as_str(), "[REDACTED]")
                        } else {
                            (k.as_str(), truncate_str(v, 20))
                        },)
                    })
                    .collect();

                if data.len() > 3 {
                    f.debug_tuple("Form")
                        .field(&format!("{:?}... ({} total)", safe_form, data.len()))
                        .finish()
                } else {
                    f.debug_tuple("Form").field(&safe_form).finish()
                }
            }
            Self::Multipart(_) => f.debug_tuple("Multipart").field(&"[multipart]").finish(),
            Self::Custom(_) => f.debug_tuple("Custom").field(&"[reqwest::Body]").finish(),
            Self::Empty => f.debug_tuple("Empty").finish(),

            #[cfg(feature = "stream")]
            Self::File(_) => f.debug_tuple("File").field(&"[file]").finish(),
            #[cfg(feature = "stream")]
            Self::BufferedFile {
                file: _,
                buffer_size,
            } => f
                .debug_tuple("BufferedFile")
                .field(&format!("{buffer_size}B buffer"))
                .finish(),
            #[cfg(feature = "stream")]
            Self::AsyncRead(_) => f.debug_tuple("AsyncRead").field(&"[reader]").finish(),
            #[cfg(feature = "stream")]
            Self::TcpStream(_) => f.debug_tuple("TcpStream").field(&"[tcp]").finish(),
            #[cfg(feature = "stream")]
            Self::ProcessOutput { command, .. } => f
                .debug_tuple("ProcessOutput")
                .field(&truncate_str(command, 40))
                .finish(),
            #[cfg(feature = "stream")]
            Self::Stream(_) => f.debug_tuple("Stream").field(&"[stream]").finish(),
            #[cfg(feature = "stream")]
            Self::IoStream(_) => f.debug_tuple("IoStream").field(&"[io_stream]").finish(),
            #[cfg(feature = "stream")]
            Self::BytesIterator(chunks) => {
                let total: usize = chunks.iter().map(|b| b.len()).sum();
                f.debug_tuple("BytesIterator")
                    .field(&format!("{total} chunks, {total}B"))
                    .finish()
            }
            #[cfg(feature = "stream")]
            Self::CodecReader { codec_type, .. } => {
                f.debug_tuple("CodecReader").field(codec_type).finish()
            }
        }
    }
}

#[cfg(feature = "stream")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodecType {
    Bytes,
    Lines,
    Json,
    Custom(String),
}

#[cfg(feature = "stream")]
impl fmt::Display for CodecType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodecType::Bytes => write!(f, "bytes"),
            CodecType::Lines => write!(f, "lines"),
            CodecType::Json => write!(f, "json"),
            CodecType::Custom(name) => write!(f, "{name}"),
        }
    }
}

fn safe_debug_preview(s: &str) -> String {
    if s.len() <= 100 {
        format!("{s:?}")
    } else {
        format!("{:?}... (+{} more chars)", &s[..100], s.len() - 100)
    }
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len])
    }
}

fn truncate_str(s: &str, max_len: usize) -> &str {
    if s.len() <= max_len {
        s
    } else {
        &s[..max_len]
    }
}

fn truncate_command(cmd: &str) -> String {
    truncate_string(cmd, 80)
}

fn is_sensitive_key(key: &str) -> bool {
    let key_lower = key.to_lowercase();
    key_lower.contains("password")
        || key_lower.contains("token")
        || key_lower.contains("secret")
        || key_lower.contains("auth")
}
