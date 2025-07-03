define_mime_type! {
    pub enum Multipart {
        FormData => {
            const: FORM_DATA_STR,
            mime: "multipart/form-data",
            extensions: [],
        },
        ByteRanges => {
            const: BYTE_RANGES_STR,
            mime: "multipart/byteranges",
            extensions: [],
        },
        Mixed => {
            const: MIXED_STR,
            mime: "multipart/mixed",
            extensions: [],
        },
        Alternative => {
            const: ALTERNATIVE_STR,
            mime: "multipart/alternative",
            extensions: [],
        },
    }
}
