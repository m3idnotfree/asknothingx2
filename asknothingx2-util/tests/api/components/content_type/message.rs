use asknothingx2_util::api::content_type::Message;

define_tests!(Message);

prop_compose! {
    pub fn base_stragety()(
        s in prop_oneof![
            Just(Message::Rfc822),
            Just(Message::Partial),
            Just(Message::ExternalBody),
    ]) -> Message { s }
}
