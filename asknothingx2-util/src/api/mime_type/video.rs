define_mime_type! {
    pub enum Video {
        Mp4 => {
            const: MP4_STR,
            mime: "video/mp4",
            extensions: ["mp4", "m4v"],
        },
        Mpeg => {
            const: MPEG_STR,
            mime: "video/mpeg",
            extensions: ["mpeg", "mpg", "mpe", "m1v", "m2v"],
        },
        Ogg => {
            const: OGG_STR,
            mime: "video/ogg",
            extensions: ["ogv"],
        },
        Webm => {
            const: WEBM_STR,
            mime: "video/webm",
            extensions: ["webm"],
        },
        Quicktime => {
            const: QUICKTIME_STR,
            mime: "video/quicktime",
            extensions: ["mov", "qt"],
        },
        XMsvideo => {
            const: X_MSVIDEO_STR,
            mime: "video/x-msvideo",
            extensions: ["avi"],
            aliases: ["video/avi"]
        },
        XFlv => {
            const: X_FLV_STR,
            mime: "video/x-flv",
            extensions: ["flv"],
        },
        XMatroska => {
            const: X_MATROSKA_STR,
            mime: "video/x-matroska",
            extensions: ["mkv"],
        },
        XMsAsf => {
            const: X_MS_ASF_STR,
            mime: "video/x-ms-asf",
            extensions: ["asf"],
        },
        XMsWm => {
            const: X_MS_WM_STR,
            mime: "video/x-ms-wm",
            extensions: ["wm"],
        },
        XMsWmv => {
            const: X_MS_WMV_STR,
            mime: "video/x-ms-wmv",
            extensions: ["wmv"],
        },
        XMsWmx => {
            const: X_MS_WMX_STR,
            mime: "video/x-ms-wmx",
            extensions: ["wmx"],
        },
        XMsWvx => {
            const: X_MS_WVX_STR,
            mime: "video/x-ms-wvx",
            extensions: ["wvx"],
        },
        XSgiMovie => {
            const: X_SGI_MOVIE_STR,
            mime: "video/x-sgi-movie",
            extensions: ["movie"],
        },
        XF4v => {
            const: X_F4V_STR,
            mime: "video/x-f4v",
            extensions: ["f4v"],
        },
        XFli => {
            const: X_FLI_STR,
            mime: "video/x-fli",
            extensions: ["fli"],
        },
        XM4v => {
            const: X_M4V_STR,
            mime: "video/x-m4v",
            extensions: [],
        },
        Video3gpp => {
            const: VIDEO_3GPP_STR,
            mime: "video/3gpp",
            extensions: ["3gp"],
        },
        Video3gpp2 => {
            const: VIDEO_3GPP2_STR,
            mime: "video/3gpp2",
            extensions: ["3g2"],
        },
        H261 => {
            const: H261_STR,
            mime: "video/h261",
            extensions: ["h261"],
        },
        H263 => {
            const: H263_STR,
            mime: "video/h263",
            extensions: ["h263"],
        },
        H264 => {
            const: H264_STR,
            mime: "video/h264",
            extensions: ["h264"],
        },
        Jpeg => {
            const: JPEG_STR,
            mime: "video/jpeg",
            extensions: ["jpgv"],
        },
        Jpm => {
            const: JPM_STR,
            mime: "video/jpm",
            extensions: ["jpm", "jpgm"],
        },
        Mj2 => {
            const: MJ2_STR,
            mime: "video/mj2",
            extensions: ["mj2", "mjp2"],
        },
        Mp2t => {
            const: MP2T_STR,
            mime: "video/mp2t",
            extensions: ["ts"],
        },
        VndFvt => {
            const: VND_FVT_STR,
            mime: "video/vnd.fvt",
            extensions: ["fvt"],
        },
        VndMpegurl => {
            const: VND_MPEGURL_STR,
            mime: "video/vnd.mpegurl",
            extensions: ["m3u8"],
        },
        VndMsPlayready => {
            const: VND_MS_PLAYREADY_STR,
            mime: "video/vnd.ms-playready.media.pyv",
            extensions: ["pyv"],
        },
        VndVivo => {
            const: VND_VIVO_STR,
            mime: "video/vnd.vivo",
            extensions: ["viv"],
        },
    }
}
