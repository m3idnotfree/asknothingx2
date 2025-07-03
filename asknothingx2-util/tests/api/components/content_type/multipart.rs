use asknothingx2_util::api::content_type::Multipart;

define_tests!(Multipart);

prop_compose! {
    pub fn base_stragety()(
        s in prop_oneof![
            Just(Multipart::FormData),
            Just(Multipart::ByteRanges),
            Just(Multipart::Mixed),
            Just(Multipart::Alternative),
    ]) -> Multipart { s }
}
