define_mime_type! {
    pub enum Font {
        Woff => {
            const: WOFF_STR,
            mime: "font/woff",
            extensions: ["woff"],
            aliases: ["application/font-woff"]
        },
        Woff2 => {
            const: WOFF2_STR,
            mime: "font/woff2",
            extensions: ["woff2"],
            aliases: ["application/font-woff2"]
        },
        Otf => {
            const: OTF_STR,
            mime: "font/otf",
            extensions: ["otf"],
        },
        Ttf => {
            const: TTF_STR,
            mime: "font/ttf",
            extensions: ["ttf"],
        },
        ApplicationXFontBdf => {
            const: APPLICATION_X_FONT_BDF_STR,
            mime: "application/x-font-bdf",
            extensions: ["bdf"],
        },
        ApplicationXFontGhostscript => {
            const: APPLICATION_X_FONT_GHOSTSCRIPT_STR,
            mime: "application/x-font-ghostscript",
            extensions: ["gsf"],
        },
        ApplicationXFontLinuxPsf => {
            const: APPLICATION_X_FONT_LINUX_PSF_STR,
            mime: "application/x-font-linux-psf",
            extensions: ["psf"],
        },
        ApplicationXFontOtf => {
            const: APPLICATION_X_FONT_OTF_STR,
            mime: "application/x-font-otf",
            extensions: [],
        },
        ApplicationXFontPcf => {
            const: APPLICATION_X_FONT_PCF_STR,
            mime: "application/x-font-pcf",
            extensions: ["pcf"],
        },
        ApplicationXFontSnf => {
            const: APPLICATION_X_FONT_SNF_STR,
            mime: "application/x-font-snf",
            extensions: ["snf"],
        },
        ApplicationXFontTtf => {
            const: APPLICATION_X_FONT_TTF_STR,
            mime: "application/x-font-ttf",
            extensions: [],
            aliases: ["application/x-font-truetype"]
        },
        ApplicationXFontType1 => {
            const: APPLICATION_X_FONT_TYPE1_STR,
            mime: "application/x-font-type1",
            extensions: ["pfa", "pfb"],
        },
        ApplicationVndMsFontobject => {
            const: APPLICATION_VND_MS_FONTOBJECT_STR,
            mime: "application/vnd.ms-fontobject",
            extensions: ["eot"],
        },
    }
}
