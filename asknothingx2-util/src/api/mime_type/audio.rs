define_mime_type! {
    pub enum Audio {
        Mpeg => {
            const: MPEG_STR,
            mime: "audio/mpeg",
            extensions: ["mp3", "mpga"],
            aliases: ["audio/mp3"]
        },
        Mp4 => {
            const: MP4_STR,
            mime: "audio/mp4",
            extensions: ["m4a", "mp4a"]
        },
        Ogg => {
            const: OGG_STR,
            mime: "audio/ogg",
            extensions: ["oga", "ogg", "spx"]
        },
        Webm => {
            const: WEBM_STR,
            mime: "audio/webm",
            extensions: ["webm"]
        },
        Wav => {
            const: WAV_STR,
            mime: "audio/wav",
            extensions: ["wav", "wave"],
            aliases: ["audio/wave", "audio/x-wav"]
        },
        Flac => {
            const: FLAC_STR,
            mime: "audio/flac",
            extensions: ["flac"]
        },
        Aac => {
            const: AAC_STR,
            mime: "audio/aac",
            extensions: ["aac"]
        },
        Aiff => {
            const: AIFF_STR,
            mime: "audio/aiff",
            extensions: ["aif", "aiff", "aifc"]
        },
        Basic => {
            const: BASIC_STR,
            mime: "audio/basic",
            extensions: ["au", "snd"]
        },
        Midi => {
            const: MIDI_STR,
            mime: "audio/midi",
            extensions: ["mid", "midi", "kar", "rmi"]
        },
        Opus => {
            const: OPUS_STR,
            mime: "audio/opus",
            extensions: ["opus"]
        },
        VndDigitalWinds => {
            const: VND_DIGITAL_WINDS_STR,
            mime: "audio/vnd.digital-winds",
            extensions: []
        },
        VndDts => {
            const: VND_DTS_STR,
            mime: "audio/vnd.dts",
            extensions: []
        },
        VndDtsHd => {
            const: VND_DTS_HD_STR,
            mime: "audio/vnd.dts.hd",
            extensions: []
        },
        VndLucentVoice => {
            const: VND_LUCENT_VOICE_STR,
            mime: "audio/vnd.lucent.voice",
            extensions: []
        },
        VndMsPlayready => {
            const: VND_MS_PLAYREADY_STR,
            mime: "audio/vnd.ms-playready.media.pya",
            extensions: []
        },
        VndNueraEcelp4800 => {
            const: VND_NUERA_ECELP4800_STR,
            mime: "audio/vnd.nuera.ecelp4800",
            extensions: []
        },
        VndNueraEcelp7470 => {
            const: VND_NUERA_ECELP7470_STR,
            mime: "audio/vnd.nuera.ecelp7470",
            extensions: []
        },
        VndNueraEcelp9600 => {
            const: VND_NUERA_ECELP9600_STR,
            mime: "audio/vnd.nuera.ecelp9600",
            extensions: []
        },
        XMatroska => {
            const: X_MATROSKA_STR,
            mime: "audio/x-matroska",
            extensions: ["mka"]
        },
        XMpegurl => {
            const: X_MPEGURL_STR,
            mime: "audio/x-mpegurl",
            extensions: ["m3u"]
        },
        XMsWax => {
            const: X_MS_WAX_STR,
            mime: "audio/x-ms-wax",
            extensions: ["wax"]
        },
        XMsWma => {
            const: X_MS_WMA_STR,
            mime: "audio/x-ms-wma",
            extensions: ["wma"]
        },
        XPnRealaudio => {
            const: X_PN_REALAUDIO_STR,
            mime: "audio/x-pn-realaudio",
            extensions: ["ra", "ram"]
        },
        XPnRealaudioPlugin => {
            const: X_PN_REALAUDIO_PLUGIN_STR,
            mime: "audio/x-pn-realaudio-plugin",
            extensions: ["rmp"]
        }
    }
}
