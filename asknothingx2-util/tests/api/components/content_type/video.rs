use asknothingx2_util::api::content_type::Video;

define_tests!(Video);

prop_compose! {
    pub fn base_stragety()(
        s in prop_oneof![
            Just(Video::Mp4),
            Just(Video::Mpeg),
            Just(Video::Ogg),
            Just(Video::Webm),
            Just(Video::Quicktime),
            Just(Video::XMsvideo),
            Just(Video::XFlv),
            Just(Video::XMatroska),
            Just(Video::H261),
            Just(Video::H263),
            Just(Video::H264),
            Just(Video::Jpeg),
            Just(Video::Jpm),
            Just(Video::Mj2),
            Just(Video::Mp2t),
            Just(Video::XMsAsf),
            Just(Video::XMsWm),
            Just(Video::XMsWmv),
            Just(Video::XMsWmx),
            Just(Video::XMsWvx),
            Just(Video::XSgiMovie),
            Just(Video::XF4v),
            Just(Video::XFli),
            Just(Video::XM4v),
            Just(Video::Video3gpp),
            Just(Video::Video3gpp2),
            Just(Video::VndFvt),
            Just(Video::VndMpegurl),
            Just(Video::VndMsPlayready),
            Just(Video::VndVivo),
    ]) -> Video { s }
}
