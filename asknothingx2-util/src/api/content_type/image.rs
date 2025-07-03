define_mime_type! {
    pub enum Image {
        Apng => {
            const: APNG_STR,
            mime: "image/apng",
            extensions: ["apng"],
        },
        Jpeg => {
            const: JPEG_STR,
            mime: "image/jpeg",
            extensions: ["jpg", "jpeg", "jpe", "jfif"],
            aliases: ["image/jpg"]
        },
        Png => {
            const: PNG_STR,
            mime: "image/png",
            extensions: ["png"],
        },
        Gif => {
            const: GIF_STR,
            mime: "image/gif",
            extensions: ["gif"],
        },
        Webp => {
            const: WEBP_STR,
            mime: "image/webp",
            extensions: ["webp"],
        },
        SvgXml => {
            const: SVG_XML_STR,
            mime: "image/svg+xml",
            extensions: ["svg", "svgz"],
            aliases: ["image/svg"]
        },
        Tiff => {
            const: TIFF_STR,
            mime: "image/tiff",
            extensions: ["tif", "tiff"],
            aliases: ["image/tif"]
        },
        Bmp => {
            const: BMP_STR,
            mime: "image/bmp",
            extensions: ["bmp"],
        },
        Icon => {
            const: ICON_STR,
            mime: "image/x-icon",
            extensions: ["ico"],
            aliases: ["image/ico", "image/x-ico"]
        },
        Avif => {
            const: AVIF_STR,
            mime: "image/avif",
            extensions: ["avif"],
        },
        Heic => {
            const: HEIC_STR,
            mime: "image/heic",
            extensions: ["heic", "heif"],
        },
        Cgm => {
            const: CGM_STR,
            mime: "image/cgm",
            extensions: ["cgm"],
        },
        Ief => {
            const: IEF_STR,
            mime: "image/ief",
            extensions: ["ief"],
        },
        G3fax => {
            const: G3FAX_STR,
            mime: "image/g3fax",
            extensions: ["g3"],
        },
        PrsBtif => {
            const: PRS_BTIF_STR,
            mime: "image/prs.btif",
            extensions: ["btif"],
        },
        VndDjvu => {
            const: VND_DJVU_STR,
            mime: "image/vnd.djvu",
            extensions: ["djv", "djvu"],
        },
        VndDwg => {
            const: VND_DWG_STR,
            mime: "image/vnd.dwg",
            extensions: ["dwg"],
        },
        VndDxf => {
            const: VND_DXF_STR,
            mime: "image/vnd.dxf",
            extensions: ["dxf"],
        },
        VndFastbidsheet => {
            const: VND_FASTBIDSHEET_STR,
            mime: "image/vnd.fastbidsheet",
            extensions: ["fbs"],
        },
        VndFpx => {
            const: VND_FPX_STR,
            mime: "image/vnd.fpx",
            extensions: ["fpx"],
        },
        VndFst => {
            const: VND_FST_STR,
            mime: "image/vnd.fst",
            extensions: ["fst"],
        },
        VndNetFpx => {
            const: VND_NET_FPX_STR,
            mime: "image/vnd.net-fpx",
            extensions: ["npx"],
        },
        VndWapWbmp => {
            const: VND_WAP_WBMP_STR,
            mime: "image/vnd.wap.wbmp",
            extensions: ["wbmp"],
        },
        VndXiff => {
            const: VND_XIFF_STR,
            mime: "image/vnd.xiff",
            extensions: ["xif"],
        },
        VndMsModi => {
            const: VND_MS_MODI_STR,
            mime: "image/vnd.ms-modi",
            extensions: ["mdi"],
        },
        XAdobeDng => {
            const: X_ADOBE_DNG_STR,
            mime: "image/x-adobe-dng",
            extensions: ["dng"],
        },
        XCanonCr2 => {
            const: X_CANON_CR2_STR,
            mime: "image/x-canon-cr2",
            extensions: ["cr2"],
        },
        XCanonCrw => {
            const: X_CANON_CRW_STR,
            mime: "image/x-canon-crw",
            extensions: ["crw"],
        },
        XCmuRaster => {
            const: X_CMU_RASTER_STR,
            mime: "image/x-cmu-raster",
            extensions: ["ras"],
        },
        XCmx => {
            const: X_CMX_STR,
            mime: "image/x-cmx",
            extensions: ["cmx"],
        },
        XEpsonErf => {
            const: X_EPSON_ERF_STR,
            mime: "image/x-epson-erf",
            extensions: ["erf"],
        },
        XFreehand => {
            const: X_FREEHAND_STR,
            mime: "image/x-freehand",
            extensions: ["fh", "fh4", "fh5", "fh7", "fhc"],
        },
        XFujiRaf => {
            const: X_FUJI_RAF_STR,
            mime: "image/x-fuji-raf",
            extensions: ["raf"],
        },
        XIcns => {
            const: X_ICNS_STR,
            mime: "image/x-icns",
            extensions: ["icns"],
        },
        XKodakDcr => {
            const: X_KODAK_DCR_STR,
            mime: "image/x-kodak-dcr",
            extensions: ["dcr"],
        },
        XKodakK25 => {
            const: X_KODAK_K25_STR,
            mime: "image/x-kodak-k25",
            extensions: ["k25"],
        },
        XKodakKdc => {
            const: X_KODAK_KDC_STR,
            mime: "image/x-kodak-kdc",
            extensions: ["kdc"],
        },
        XMinoltaMrw => {
            const: X_MINOLTA_MRW_STR,
            mime: "image/x-minolta-mrw",
            extensions: ["mrw"],
        },
        XNikonNef => {
            const: X_NIKON_NEF_STR,
            mime: "image/x-nikon-nef",
            extensions: ["nef"],
        },
        XOlympusOrf => {
            const: X_OLYMPUS_ORF_STR,
            mime: "image/x-olympus-orf",
            extensions: ["orf"],
        },
        XPanasonicRaw => {
            const: X_PANASONIC_RAW_STR,
            mime: "image/x-panasonic-raw",
            extensions: ["raw", "rw2", "rwl"],
        },
        XPcx => {
            const: X_PCX_STR,
            mime: "image/x-pcx",
            extensions: ["pcx"],
        },
        XPentaxPef => {
            const: X_PENTAX_PEF_STR,
            mime: "image/x-pentax-pef",
            extensions: ["pef", "ptx"],
        },
        XPict => {
            const: X_PICT_STR,
            mime: "image/x-pict",
            extensions: ["pct", "pic"],
        },
        XPortableAnymap => {
            const: X_PORTABLE_ANYMAP_STR,
            mime: "image/x-portable-anymap",
            extensions: ["pnm"],
        },
        XPortableBitmap => {
            const: X_PORTABLE_BITMAP_STR,
            mime: "image/x-portable-bitmap",
            extensions: ["pbm"],
        },
        XPortableGraymap => {
            const: X_PORTABLE_GRAYMAP_STR,
            mime: "image/x-portable-graymap",
            extensions: ["pgm"],
        },
        XPortablePixmap => {
            const: X_PORTABLE_PIXMAP_STR,
            mime: "image/x-portable-pixmap",
            extensions: ["ppm"],
        },
        XRgb => {
            const: X_RGB_STR,
            mime: "image/x-rgb",
            extensions: ["rgb"],
        },
        XSigmaX3f => {
            const: X_SIGMA_X3F_STR,
            mime: "image/x-sigma-x3f",
            extensions: ["x3f"],
        },
        XSonyArw => {
            const: X_SONY_ARW_STR,
            mime: "image/x-sony-arw",
            extensions: ["arw"],
        },
        XSonySr2 => {
            const: X_SONY_SR2_STR,
            mime: "image/x-sony-sr2",
            extensions: ["sr2"],
        },
        XSonySrf => {
            const: X_SONY_SRF_STR,
            mime: "image/x-sony-srf",
            extensions: ["srf"],
        },
        XXbitmap => {
            const: X_XBITMAP_STR,
            mime: "image/x-xbitmap",
            extensions: ["xbm"],
        },
        XXpixmap => {
            const: X_XPIXMAP_STR,
            mime: "image/x-xpixmap",
            extensions: ["xpm"],
        },
        XXwindowdump => {
            const: X_XWINDOWDUMP_STR,
            mime: "image/x-xwindowdump",
            extensions: ["xwd"],
        },
    }
}
