define_mime_type! {
    pub enum Message {
        Rfc822 => {
            const: RFC822_STR,
            mime: "message/rfc822",
            extensions: ["eml", "mime"],
        },
        Partial => {
            const: PARTIAL_STR,
            mime: "message/partial",
            extensions: [],
        },
        ExternalBody => {
            const: EXTERNAL_BODY_STR,
            mime: "message/external-body",
            extensions: [],
        },
    }
}
