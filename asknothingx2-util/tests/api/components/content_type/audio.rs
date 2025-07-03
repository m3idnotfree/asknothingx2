use asknothingx2_util::api::content_type::Audio;

define_tests!(Audio);

prop_compose! {
    fn base_stragety()(
        s in prop_oneof![
            Just(Audio::Mpeg),
            Just(Audio::Mp4),
            Just(Audio::Ogg),
            Just(Audio::Webm),
            Just(Audio::Wav),
            Just(Audio::Flac),
            Just(Audio::Aac),
            Just(Audio::Aiff),
            Just(Audio::Basic),
            Just(Audio::Midi),
            Just(Audio::Opus),
            Just(Audio::VndDigitalWinds),
            Just(Audio::VndDts),
            Just(Audio::VndDtsHd),
            Just(Audio::VndLucentVoice),
            Just(Audio::VndMsPlayready),
            Just(Audio::VndNueraEcelp4800),
            Just(Audio::VndNueraEcelp7470),
            Just(Audio::VndNueraEcelp9600),
            Just(Audio::XMatroska),
            Just(Audio::XMpegurl),
            Just(Audio::XMsWax),
            Just(Audio::XMsWma),
            Just(Audio::XPnRealaudio),
            Just(Audio::XPnRealaudioPlugin),
    ])->Audio { s }
}
