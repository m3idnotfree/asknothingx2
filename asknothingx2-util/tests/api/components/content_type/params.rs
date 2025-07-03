use proptest::{
    prelude::{prop, Strategy},
    prop_compose, prop_oneof,
};

#[derive(Debug, Clone)]
pub enum Params {
    Charset(String),
    Boundary(String),
    Version(String),
    Any(String, String),
}

impl Params {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        let key = key.into();
        let a = "charset".to_string();
        let b = "boundary".to_string();
        let c = "version".to_string();
        match key {
            a => Self::Charset(value.into()),
            b => Self::Boundary(value.into()),
            c => Self::Version(value.into()),
            _ => Self::Any(key, value.into()),
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            Self::Charset(_) => "charset",
            Self::Boundary(_) => "boundary",
            Self::Version(_) => "version",
            Self::Any(s, _) => s,
        }
    }

    pub fn expected_key(&self) -> &str {
        self.as_str()
    }

    pub fn expected_value(&self) -> &str {
        match self {
            Self::Charset(v) => v,
            Self::Boundary(v) => v,
            Self::Version(v) => v,
            Self::Any(_, v) => v,
        }
    }
}

prop_compose! {
    /// input, expected_pairs
    pub fn parameter_strategy()(
        parameter in prop_oneof![
            normal_parameter_strategy(),
            quoted_parameter_strategy()
    ]) -> (String, Vec<Params>) { parameter }
}

prop_compose! {
    /// input, expected_pairs
    fn normal_parameter_strategy()(
        parameter in prop::sample::select(vec![
            // ===== CHARSET PARAMETERS =====
            vec![("charset", "utf-8")],
            vec![("charset", "UTF-8")],
            vec![("charset", "iso-8859-1")],
            vec![("charset", "ascii")],
            vec![("charset", "utf-16")],
            vec![("charset", "windows-1252")],

            // ===== BOUNDARY PARAMETERS =====
            vec![("boundary", "----WebKitFormBoundary7MA4YWxkTrZu0gW")],
            vec![("boundary", "----formdata-ng-boundary-123456")],
            vec![("boundary", "simple-boundary")],
            vec![("boundary", "abc123")],
            vec![("boundary", "----asknothingx2-multipart-boundary")],

            // ===== VERSION PARAMETERS =====
            vec![("version", "1.0")],
            vec![("version", "1.1")],
            vec![("version", "2.0")],

            // ===== QUALITY PARAMETERS =====
            vec![("q", "0.8")],
            vec![("q", "0.9")],
            vec![("q", "1.0")],

            // ===== ENCODING PARAMETERS =====
            vec![("encoding", "gzip")],
            vec![("encoding", "deflate")],
            vec![("encoding", "br")],

            // ===== MULTIPLE PARAMETERS =====
            vec![("charset", "utf-8"), ("boundary", "----WebKit123")],
            vec![("charset", "UTF-8"), ("version", "1.0")],
            vec![("boundary", "simple"), ("encoding", "gzip")],
            vec![("charset", "utf-8"), ("q", "0.9"), ("version", "1.1")],

            // ===== EMPTY (no parameters) =====
            vec![],
        ]).prop_map(|params| {

            let input = params.iter()
                .map(|(name, value)| format!("{name}={value}"))
                .collect::<Vec<_>>()
                .join("; ");
            let expected_pairs: Vec<Params> = params.into_iter()
                .map(|(k, v)| Params::new(k,v))
                .collect();
            (input, expected_pairs)
           })
    ) -> (String, Vec<Params>) { parameter }
}

prop_compose! {
    /// input, expected_pairs
    fn quoted_parameter_strategy()(
        quoted_parameter in prop::sample::select(vec![
            // ===== QUOTED CHARSET =====
            vec![("charset", "\"utf-8\"")],
            vec![("charset", "'UTF-8'")],
            vec![("charset", "\"iso-8859-1\"")],
            // ===== QUOTED BOUNDARY =====
            vec![("boundary", "\"----WebKit123\"")],
            vec![("boundary", "'simple-boundary'")],
            // ===== MIXED QUOTED/UNQUOTED =====
            vec![("charset", "utf-8"), ("boundary", "\"----WebKit123\"")],
            vec![("charset", "\"UTF-8\""), ("version", "1.0")],
            // ===== EMPTY QUOTED VALUES =====
            vec![("empty", "\"\"")],
            vec![("empty", "''")],
            // ===== QUOTED WITH SPECIAL CHARS =====
            vec![("boundary", "\"----WebKit-Form-Boundary-123/456\"")],
            vec![("charset", "\"utf-8; special=value\"")], // Nested parameters
            // ===== EMPTY (no parameters) =====
            vec![],

        ])
        .prop_map(|params| {
            let input = params.iter()
                .map(|(name, value)| format!("{name}={value}"))
                .collect::<Vec<_>>()
                .join("; ");
            let expected_pairs: Vec<Params> = params.into_iter()
                .map(|(k, v)| Params::new(k, v))
                .collect();
            (input, expected_pairs)

        })
    ) -> (String, Vec<Params>) { quoted_parameter }
}
