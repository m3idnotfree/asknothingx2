use asknothingx2_util::api::content_type::Font;

define_tests!(Font);

prop_compose! {
    fn base_stragety()(
        s in prop_oneof![
            Just(Font::Woff),
            Just(Font::Woff2),
            Just(Font::Otf),
            Just(Font::Ttf),
            Just(Font::ApplicationXFontBdf),
            Just(Font::ApplicationXFontGhostscript),
            Just(Font::ApplicationXFontLinuxPsf),
            Just(Font::ApplicationXFontOtf),
            Just(Font::ApplicationXFontPcf),
            Just(Font::ApplicationXFontSnf),
            Just(Font::ApplicationXFontTtf),
            Just(Font::ApplicationXFontType1),
            Just(Font::ApplicationVndMsFontobject),
    ]) -> Font { s }
}
