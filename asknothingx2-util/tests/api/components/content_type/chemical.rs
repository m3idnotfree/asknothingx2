use asknothingx2_util::api::content_type::Chemical;

define_tests!(Chemical);

prop_compose! {
    pub fn base_stragety()(
        s in prop_oneof![
            Just(Chemical::XCdx),
            Just(Chemical::XCif),
            Just(Chemical::XCml),
            Just(Chemical::XCsml),
            Just(Chemical::XXyz),
    ]) -> Chemical { s }
}
