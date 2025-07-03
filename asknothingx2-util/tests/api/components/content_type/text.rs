use asknothingx2_util::api::content_type::Text;

define_tests!(Text);

prop_compose! {
    pub fn base_stragety()(
        s in prop_oneof![
            Just(Text::Plain),
            Just(Text::Html),
            Just(Text::Css),
            Just(Text::Javascript),
            Just(Text::Csv),
            Just(Text::Xml),
            Just(Text::Markdown),
            Just(Text::Calendar),
            Just(Text::Richtext),
            Just(Text::Sgml),
            Just(Text::TabSeparatedValues),
            Just(Text::Troff),
            Just(Text::UriList),
            Just(Text::VCard),
            Just(Text::VCalendar),
            Just(Text::Setext),
            Just(Text::Uuencode),
            Just(Text::Asm),
            Just(Text::C),
            Just(Text::Fortran),
            Just(Text::JavaSource),
            Just(Text::Pascal),
            Just(Text::Python),
    ]) -> Text { s }
}
