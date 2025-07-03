#![cfg(feature = "api")]

use asknothingx2_util::api::content_type::*;
use std::str::FromStr;

// ===== BASIC PARSING TESTS =====

#[test]
fn test_all_application_types() {
    let test_cases = vec![
        ("application/json", Application::Json),
        ("application/xml", Application::Xml),
        ("application/pdf", Application::Pdf),
        ("application/zip", Application::Zip),
        ("application/gzip", Application::Gzip),
        ("application/octet-stream", Application::OctetStream),
        (
            "application/x-www-form-urlencoded",
            Application::FormUrlEncoded,
        ),
        ("application/postscript", Application::Postscript),
        ("application/rtf", Application::Rtf),
        ("application/atom+xml", Application::AtomXml),
        ("application/rss+xml", Application::RssXml),
        ("application/soap+xml", Application::SoapXml),
        ("application/xhtml+xml", Application::XhtmlXml),
        ("application/xslt+xml", Application::XsltXml),
        ("application/yaml", Application::Yaml),
        ("application/wasm", Application::Wasm),
        // Microsoft Office
        ("application/msword", Application::MsWord),
        ("application/vnd.ms-excel", Application::MsExcel),
        ("application/vnd.ms-powerpoint", Application::MsPowerpoint),
        ("application/vnd.ms-project", Application::MsProject),
        ("application/vnd.ms-works", Application::MsWorks),
        ("application/vnd.visio", Application::MsVisio),
        ("application/onenote", Application::MsOneNote),
        // Office Open XML
        (
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            Application::VndOpenXmlWordDoc,
        ),
        (
            "application/vnd.openxmlformats-officedocument.wordprocessingml.template",
            Application::VndOpenXmlWordTemplate,
        ),
        (
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            Application::VndOpenXmlSpreadsheet,
        ),
        (
            "application/vnd.openxmlformats-officedocument.spreadsheetml.template",
            Application::VndOpenXmlSpreadsheetTemplate,
        ),
        (
            "application/vnd.openxmlformats-officedocument.presentationml.presentation",
            Application::VndOpenXmlPresentation,
        ),
        (
            "application/vnd.openxmlformats-officedocument.presentationml.template",
            Application::VndOpenXmlPresentationTemplate,
        ),
        // OpenDocument
        (
            "application/vnd.oasis.opendocument.text",
            Application::VndOasisText,
        ),
        (
            "application/vnd.oasis.opendocument.spreadsheet",
            Application::VndOasisSpreadsheet,
        ),
        (
            "application/vnd.oasis.opendocument.presentation",
            Application::VndOasisPresentation,
        ),
        (
            "application/vnd.oasis.opendocument.graphics",
            Application::VndOasisGraphics,
        ),
        (
            "application/vnd.oasis.opendocument.formula",
            Application::VndOasisFormula,
        ),
        (
            "application/vnd.oasis.opendocument.database",
            Application::VndOasisDatabase,
        ),
        // Compression
        ("application/x-7z-compressed", Application::X7zCompressed),
        ("application/x-rar-compressed", Application::XRarCompressed),
        ("application/x-tar", Application::XTar),
        ("application/x-bzip2", Application::XBzip2),
        ("application/x-ace-compressed", Application::XAceCompressed),
        ("application/x-stuffit", Application::XStuffit),
        (
            "application/vnd.debian.binary-package",
            Application::VndDebian,
        ),
        ("application/vnd.rar", Application::VndRar),
        // Programming
        ("application/java-archive", Application::JavaArchive),
        (
            "application/java-serialized-object",
            Application::JavaSerializedObject,
        ),
        ("application/java-vm", Application::JavaVm),
        ("application/x-shellscript", Application::XShellScript),
        ("application/x-perl", Application::XPerl),
        ("application/x-tcl", Application::XTcl),
        ("application/x-python", Application::XPython),
        ("application/x-ruby", Application::XRuby),
        // Ebooks
        ("application/epub+zip", Application::EpubZip),
        ("application/vnd.amazon.ebook", Application::VndAmazonEbook),
        (
            "application/x-mobipocket-ebook",
            Application::XMobipocketEbook,
        ),
        ("application/vnd.ms-htmlhelp", Application::VndMsHtmlhelp),
        // Data
        ("application/vnd.sqlite3", Application::VndSqlite3),
        ("application/x-netcdf", Application::XNetcdf),
        ("application/x-hdf", Application::XHdf),
        ("application/mbox", Application::VndMbox),
        // Adobe
        (
            "application/vnd.adobe.air-application-installer-package+zip",
            Application::VndAdobeAir,
        ),
        ("application/vnd.adobe.xdp+xml", Application::VndAdobeXdp),
        ("application/vnd.adobe.xfdf", Application::VndAdobeXfdf),
        ("image/vnd.adobe.photoshop", Application::VndAdobePhotoshop),
        // Google
        (
            "application/vnd.google-earth.kml+xml",
            Application::VndGoogleEarthKml,
        ),
        (
            "application/vnd.google-earth.kmz",
            Application::VndGoogleEarthKmz,
        ),
        // Others
        (
            "application/vnd.apple.installer+xml",
            Application::VndAppleInstaller,
        ),
        (
            "application/vnd.android.package-archive",
            Application::VndAndroidPackage,
        ),
        ("application/vnd.dwg", Application::VndAutocadDwg),
        ("application/vnd.dxf", Application::VndAutocadDxf),
    ];

    for (input, expected) in test_cases {
        let result = Application::from_str(input).unwrap();
        assert_eq!(result, expected, "Failed to parse Application: {input}");

        let content_type = ContentType::from_str(input).unwrap();
        assert_eq!(content_type, ContentType::Application(expected));
    }
}

#[test]
fn test_all_audio_types() {
    let test_cases = vec![
        ("audio/mpeg", Audio::Mpeg),
        ("audio/mp4", Audio::Mp4),
        ("audio/ogg", Audio::Ogg),
        ("audio/webm", Audio::Webm),
        ("audio/wav", Audio::Wav),
        ("audio/flac", Audio::Flac),
        ("audio/aac", Audio::Aac),
        ("audio/aiff", Audio::Aiff),
        ("audio/basic", Audio::Basic),
        ("audio/midi", Audio::Midi),
        ("audio/opus", Audio::Opus),
        ("audio/vnd.digital-winds", Audio::VndDigitalWinds),
        ("audio/vnd.dts", Audio::VndDts),
        ("audio/vnd.dts.hd", Audio::VndDtsHd),
        ("audio/vnd.lucent.voice", Audio::VndLucentVoice),
        ("audio/vnd.ms-playready.media.pya", Audio::VndMsPlayready),
        ("audio/vnd.nuera.ecelp4800", Audio::VndNueraEcelp4800),
        ("audio/vnd.nuera.ecelp7470", Audio::VndNueraEcelp7470),
        ("audio/vnd.nuera.ecelp9600", Audio::VndNueraEcelp9600),
        ("audio/x-matroska", Audio::XMatroska),
        ("audio/x-mpegurl", Audio::XMpegurl),
        ("audio/x-ms-wax", Audio::XMsWax),
        ("audio/x-ms-wma", Audio::XMsWma),
        ("audio/x-pn-realaudio", Audio::XPnRealaudio),
        ("audio/x-pn-realaudio-plugin", Audio::XPnRealaudioPlugin),
        // Aliases
        ("audio/mp3", Audio::Mpeg),
        ("audio/wave", Audio::Wav),
        ("audio/x-wav", Audio::Wav),
    ];

    for (input, expected) in test_cases {
        let result = Audio::from_str(input).unwrap();
        assert_eq!(result, expected, "Failed to parse Audio: {input}");

        let content_type = ContentType::from_str(input).unwrap();
        assert_eq!(content_type, ContentType::Audio(expected));
    }
}

#[test]
fn test_all_image_types() {
    let test_cases = vec![
        ("image/apng", Image::Apng),
        ("image/jpeg", Image::Jpeg),
        ("image/png", Image::Png),
        ("image/gif", Image::Gif),
        ("image/webp", Image::Webp),
        ("image/svg+xml", Image::SvgXml),
        ("image/tiff", Image::Tiff),
        ("image/bmp", Image::Bmp),
        ("image/x-icon", Image::Icon),
        ("image/avif", Image::Avif),
        ("image/heic", Image::Heic),
        ("image/cgm", Image::Cgm),
        ("image/ief", Image::Ief),
        ("image/g3fax", Image::G3fax),
        ("image/prs.btif", Image::PrsBtif),
        ("image/vnd.djvu", Image::VndDjvu),
        ("image/vnd.dwg", Image::VndDwg),
        ("image/vnd.dxf", Image::VndDxf),
        ("image/vnd.fastbidsheet", Image::VndFastbidsheet),
        ("image/vnd.fpx", Image::VndFpx),
        ("image/vnd.fst", Image::VndFst),
        ("image/vnd.net-fpx", Image::VndNetFpx),
        ("image/vnd.wap.wbmp", Image::VndWapWbmp),
        ("image/vnd.xiff", Image::VndXiff),
        ("image/vnd.ms-modi", Image::VndMsModi),
        // RAW formats
        ("image/x-adobe-dng", Image::XAdobeDng),
        ("image/x-canon-cr2", Image::XCanonCr2),
        ("image/x-canon-crw", Image::XCanonCrw),
        ("image/x-epson-erf", Image::XEpsonErf),
        ("image/x-fuji-raf", Image::XFujiRaf),
        ("image/x-kodak-dcr", Image::XKodakDcr),
        ("image/x-kodak-k25", Image::XKodakK25),
        ("image/x-kodak-kdc", Image::XKodakKdc),
        ("image/x-minolta-mrw", Image::XMinoltaMrw),
        ("image/x-nikon-nef", Image::XNikonNef),
        ("image/x-olympus-orf", Image::XOlympusOrf),
        ("image/x-panasonic-raw", Image::XPanasonicRaw),
        ("image/x-pentax-pef", Image::XPentaxPef),
        ("image/x-sigma-x3f", Image::XSigmaX3f),
        ("image/x-sony-arw", Image::XSonyArw),
        ("image/x-sony-sr2", Image::XSonySr2),
        ("image/x-sony-srf", Image::XSonySrf),
        // Other formats
        ("image/x-cmu-raster", Image::XCmuRaster),
        ("image/x-cmx", Image::XCmx),
        ("image/x-freehand", Image::XFreehand),
        ("image/x-icns", Image::XIcns),
        ("image/x-pcx", Image::XPcx),
        ("image/x-pict", Image::XPict),
        ("image/x-portable-anymap", Image::XPortableAnymap),
        ("image/x-portable-bitmap", Image::XPortableBitmap),
        ("image/x-portable-graymap", Image::XPortableGraymap),
        ("image/x-portable-pixmap", Image::XPortablePixmap),
        ("image/x-rgb", Image::XRgb),
        ("image/x-xbitmap", Image::XXbitmap),
        ("image/x-xpixmap", Image::XXpixmap),
        ("image/x-xwindowdump", Image::XXwindowdump),
        // Aliases
        ("image/jpg", Image::Jpeg),
        ("image/svg", Image::SvgXml),
        ("image/ico", Image::Icon),
        ("image/x-ico", Image::Icon),
        ("image/tif", Image::Tiff),
    ];

    for (input, expected) in test_cases {
        let result = Image::from_str(input).unwrap();
        assert_eq!(result, expected, "Failed to parse Image: {input}");

        let content_type = ContentType::from_str(input).unwrap();
        assert_eq!(content_type, ContentType::Image(expected));
    }
}

#[test]
fn test_all_text_types() {
    let test_cases = vec![
        ("text/plain", Text::Plain),
        ("text/html", Text::Html),
        ("text/css", Text::Css),
        ("text/javascript", Text::Javascript),
        ("text/csv", Text::Csv),
        ("text/xml", Text::Xml),
        ("text/markdown", Text::Markdown),
        ("text/calendar", Text::Calendar),
        ("text/richtext", Text::Richtext),
        ("text/sgml", Text::Sgml),
        ("text/tab-separated-values", Text::TabSeparatedValues),
        ("text/troff", Text::Troff),
        ("text/uri-list", Text::UriList),
        ("text/x-vcard", Text::VCard),
        ("text/x-vcalendar", Text::VCalendar),
        ("text/x-setext", Text::Setext),
        ("text/x-uuencode", Text::Uuencode),
        ("text/x-asm", Text::Asm),
        ("text/x-c", Text::C),
        ("text/x-fortran", Text::Fortran),
        ("text/x-java-source", Text::JavaSource),
        ("text/x-pascal", Text::Pascal),
        ("text/x-python", Text::Python),
        // Aliases
        ("text/x-markdown", Text::Markdown),
        ("text/rtf", Text::Richtext),
    ];

    for (input, expected) in test_cases {
        let result = Text::from_str(input).unwrap();
        assert_eq!(result, expected, "Failed to parse Text: {input}");

        let content_type = ContentType::from_str(input).unwrap();
        assert_eq!(content_type, ContentType::Text(expected));
    }
}

#[test]
fn test_all_video_types() {
    let test_cases = vec![
        ("video/mp4", Video::Mp4),
        ("video/mpeg", Video::Mpeg),
        ("video/ogg", Video::Ogg),
        ("video/webm", Video::Webm),
        ("video/quicktime", Video::Quicktime),
        ("video/x-msvideo", Video::XMsvideo),
        ("video/x-flv", Video::XFlv),
        ("video/x-matroska", Video::XMatroska),
        ("video/x-ms-asf", Video::XMsAsf),
        ("video/x-ms-wm", Video::XMsWm),
        ("video/x-ms-wmv", Video::XMsWmv),
        ("video/x-ms-wmx", Video::XMsWmx),
        ("video/x-ms-wvx", Video::XMsWvx),
        ("video/x-sgi-movie", Video::XSgiMovie),
        ("video/x-f4v", Video::XF4v),
        ("video/x-fli", Video::XFli),
        ("video/x-m4v", Video::XM4v),
        ("video/3gpp", Video::Video3gpp),
        ("video/3gpp2", Video::Video3gpp2),
        ("video/h261", Video::H261),
        ("video/h263", Video::H263),
        ("video/h264", Video::H264),
        ("video/jpeg", Video::Jpeg),
        ("video/jpm", Video::Jpm),
        ("video/mj2", Video::Mj2),
        ("video/mp2t", Video::Mp2t),
        ("video/vnd.fvt", Video::VndFvt),
        ("video/vnd.mpegurl", Video::VndMpegurl),
        ("video/vnd.ms-playready.media.pyv", Video::VndMsPlayready),
        ("video/vnd.vivo", Video::VndVivo),
        // Aliases
        ("video/avi", Video::XMsvideo),
    ];

    for (input, expected) in test_cases {
        let result = Video::from_str(input).unwrap();
        assert_eq!(result, expected, "Failed to parse Video: {input}");

        let content_type = ContentType::from_str(input).unwrap();
        assert_eq!(content_type, ContentType::Video(expected));
    }
}

#[test]
fn test_smaller_categories() {
    // Chemical
    let chemical_cases = vec![
        ("chemical/x-cdx", Chemical::XCdx),
        ("chemical/x-cif", Chemical::XCif),
        ("chemical/x-cml", Chemical::XCml),
        ("chemical/x-csml", Chemical::XCsml),
        ("chemical/x-xyz", Chemical::XXyz),
    ];

    for (input, expected) in chemical_cases {
        let result = Chemical::from_str(input).unwrap();
        assert_eq!(result, expected, "Failed to parse Chemical: {input}");
    }

    // Font
    let font_cases = vec![
        ("font/woff", Font::Woff),
        ("font/woff2", Font::Woff2),
        ("font/otf", Font::Otf),
        ("font/ttf", Font::Ttf),
        ("application/x-font-bdf", Font::ApplicationXFontBdf),
        (
            "application/x-font-ghostscript",
            Font::ApplicationXFontGhostscript,
        ),
        (
            "application/x-font-linux-psf",
            Font::ApplicationXFontLinuxPsf,
        ),
        ("application/x-font-otf", Font::ApplicationXFontOtf),
        ("application/x-font-pcf", Font::ApplicationXFontPcf),
        ("application/x-font-snf", Font::ApplicationXFontSnf),
        ("application/x-font-ttf", Font::ApplicationXFontTtf),
        ("application/x-font-type1", Font::ApplicationXFontType1),
        (
            "application/vnd.ms-fontobject",
            Font::ApplicationVndMsFontobject,
        ),
        // Aliases
        ("application/font-woff", Font::Woff),
        ("application/font-woff2", Font::Woff2),
        ("application/x-font-truetype", Font::ApplicationXFontTtf),
    ];

    for (input, expected) in font_cases {
        let result = Font::from_str(input).unwrap();
        assert_eq!(result, expected, "Failed to parse Font: {input}");
    }

    // Message
    let message_cases = vec![
        ("message/rfc822", Message::Rfc822),
        ("message/partial", Message::Partial),
        ("message/external-body", Message::ExternalBody),
    ];

    for (input, expected) in message_cases {
        let result = Message::from_str(input).unwrap();
        assert_eq!(result, expected, "Failed to parse Message: {input}");
    }

    // Model
    let model_cases = vec![
        ("model/iges", Model::Iges),
        ("model/mesh", Model::Mesh),
        ("model/vrml", Model::Vrml),
        ("model/vnd.dwf", Model::VndDwf),
        ("model/vnd.gdl", Model::VndGdl),
        ("model/vnd.gtw", Model::VndGtw),
        ("model/vnd.mts", Model::VndMts),
        ("model/vnd.vtu", Model::VndVtu),
    ];

    for (input, expected) in model_cases {
        let result = Model::from_str(input).unwrap();
        assert_eq!(result, expected, "Failed to parse Model: {input}");
    }

    // Multipart
    let multipart_cases = vec![
        ("multipart/form-data", Multipart::FormData),
        ("multipart/byteranges", Multipart::ByteRanges),
        ("multipart/mixed", Multipart::Mixed),
        ("multipart/alternative", Multipart::Alternative),
    ];

    for (input, expected) in multipart_cases {
        let result = Multipart::from_str(input).unwrap();
        assert_eq!(result, expected, "Failed to parse Multipart: {input}");
    }
}

// ===== CASE INSENSITIVITY TESTS =====

#[test]
fn test_case_insensitive_parsing() {
    let test_cases = vec![
        ("application/json", "APPLICATION/JSON"),
        ("application/json", "Application/Json"),
        ("application/json", "application/JSON"),
        ("text/html", "TEXT/HTML"),
        ("text/html", "Text/Html"),
        ("image/jpeg", "IMAGE/JPEG"),
        ("image/jpeg", "Image/Jpeg"),
        ("audio/mpeg", "AUDIO/MPEG"),
        ("video/mp4", "VIDEO/MP4"),
        ("multipart/form-data", "MULTIPART/FORM-DATA"),
        ("multipart/form-data", "Multipart/Form-Data"),
    ];

    for (original, case_variant) in test_cases {
        let original_result = ContentType::from_str(original).unwrap();
        let variant_result = ContentType::from_str(case_variant).unwrap();
        assert_eq!(
            original_result, variant_result,
            "Case insensitive parsing failed: '{original}' vs '{case_variant}'"
        );
    }
}

// ===== PARAMETER HANDLING TESTS =====

#[test]
fn test_parameter_handling() {
    let test_cases = vec![
        // Basic parameters
        (
            "application/json; charset=utf-8",
            ContentType::Application(Application::Json),
        ),
        ("text/html; charset=UTF-8", ContentType::Text(Text::Html)),
        (
            "text/plain; charset=iso-8859-1",
            ContentType::Text(Text::Plain),
        ),
        // Multiple parameters
        (
            "multipart/form-data; boundary=----WebKit123; charset=utf-8",
            ContentType::Multipart(Multipart::FormData),
        ),
        (
            "application/json; charset=utf-8; version=1.0",
            ContentType::Application(Application::Json),
        ),
        // Quoted parameters
        (
            "application/json; charset=\"utf-8\"",
            ContentType::Application(Application::Json),
        ),
        (
            "multipart/form-data; boundary=\"----WebKit123\"",
            ContentType::Multipart(Multipart::FormData),
        ),
        (
            "text/plain; charset='utf-8'",
            ContentType::Text(Text::Plain),
        ),
        // Whitespace variations
        (
            "application/json;charset=utf-8",
            ContentType::Application(Application::Json),
        ),
        (
            "application/json ; charset=utf-8",
            ContentType::Application(Application::Json),
        ),
        (
            "application/json; charset = utf-8",
            ContentType::Application(Application::Json),
        ),
        (
            "application/json ; charset = utf-8 ",
            ContentType::Application(Application::Json),
        ),
    ];

    for (input, expected) in test_cases {
        let result = ContentType::from_str(input).unwrap();
        assert_eq!(result, expected, "Parameter handling failed for: {input}");
    }
}

#[test]
fn test_parameter_extraction() {
    let test_cases = vec![
        ("application/json; charset=utf-8", Some("utf-8")),
        ("application/json; charset=\"utf-8\"", Some("utf-8")),
        ("application/json; charset='UTF-8'", Some("UTF-8")),
        (
            "text/html; charset=iso-8859-1; version=1.0",
            Some("iso-8859-1"),
        ),
        ("application/json", None),
        ("text/plain; encoding=gzip", None), // charset not present
    ];

    for (input, expected_charset) in test_cases {
        let actual = ContentType::extract_charset(input);
        assert_eq!(
            actual, expected_charset,
            "Charset extraction failed for: {input}"
        );
    }

    let boundary_cases = vec![
        (
            "multipart/form-data; boundary=----WebKit123",
            Some("----WebKit123"),
        ),
        (
            "multipart/form-data; boundary=\"----WebKit123\"",
            Some("----WebKit123"),
        ),
        (
            "multipart/form-data; charset=utf-8; boundary=simple",
            Some("simple"),
        ),
        ("application/json", None),
    ];

    for (input, expected_boundary) in boundary_cases {
        let actual = ContentType::extract_boundary(input);
        assert_eq!(
            actual, expected_boundary,
            "Boundary extraction failed for: {input}"
        );
    }
}

// ===== ERROR CASES =====

#[test]
fn test_invalid_mime_types() {
    let invalid_cases = vec![
        "",                       // Empty
        "   ",                    // Whitespace only
        "invalid",                // No slash
        "application/",           // Missing subtype
        "/json",                  // Missing type
        "/",                      // Just slash
        "application//json",      // Double slash
        "application/json/extra", // Too many parts
        "тип/подтип",             // Non-ASCII
        "type\x00/subtype",       // Null byte
        "type/sub\x00type",       // Null byte in subtype
    ];

    for invalid_input in invalid_cases {
        let result = ContentType::from_str(invalid_input);
        assert!(
            result.is_err(),
            "Should reject invalid MIME type: '{invalid_input}'"
        );
    }
}

#[test]
fn test_malformed_parameters() {
    let malformed_cases = vec![
        "text/plain; =value",          // Missing parameter name
        "text/plain; name=",           // Missing parameter value
        "text/plain; =",               // Missing both
        "text/plain;;",                // Double semicolon
        "text/plain; ;",               // Empty parameter
        "text/plain;name=value;",      // Trailing semicolon
        "text/plain; name value",      // Missing equals
        "text/plain; name==value",     // Double equals
        "text/plain; charset=\"utf-8", // Unclosed quote
        "text/plain; charset='utf-8",  // Unclosed single quote
    ];

    for malformed_input in malformed_cases {
        // These should still parse the base content type successfully
        let result = ContentType::from_str(malformed_input);
        // Most should succeed since we only parse the base type
        if result.is_ok() {
            assert_eq!(result.unwrap(), ContentType::Text(Text::Plain));
        }
    }
}

#[test]
fn test_unsupported_content_types() {
    let unsupported_cases = vec![
        "unknown/type",
        "fake/subtype",
        "application/nonexistent",
        "text/unknown",
        "image/fake",
    ];

    for unsupported_input in unsupported_cases {
        let result = ContentType::from_str(unsupported_input);
        // Should create Custom variant
        assert!(result.is_ok());
        if let Ok(ContentType::Custom(custom)) = result {
            assert_eq!(custom, unsupported_input);
        } else {
            panic!("Expected Custom variant for: {unsupported_input}");
        }
    }
}

// ===== ROUND-TRIP TESTS =====

#[test]
fn test_round_trip_serialization() {
    let test_types = vec![
        ContentType::Application(Application::Json),
        ContentType::Text(Text::Html),
        ContentType::Image(Image::Jpeg),
        ContentType::Audio(Audio::Mpeg),
        ContentType::Video(Video::Mp4),
        ContentType::Multipart(Multipart::FormData),
        ContentType::Font(Font::Woff),
        ContentType::Chemical(Chemical::XCdx),
        ContentType::Message(Message::Rfc822),
        ContentType::Model(Model::Iges),
    ];

    for content_type in test_types {
        let serialized = content_type.to_string();
        let deserialized = ContentType::from_str(&serialized).unwrap();
        assert_eq!(
            content_type, deserialized,
            "Round-trip failed for: {content_type:?}"
        );
    }
}

// ===== HEADER VALUE CONVERSION TESTS =====

#[test]
fn test_header_value_conversion() {
    let test_cases = vec![
        "application/json",
        "text/html; charset=utf-8",
        "multipart/form-data; boundary=----WebKit123",
        "image/jpeg",
        "audio/mpeg",
        "video/mp4",
    ];

    for input in test_cases {
        let content_type = ContentType::from_str(input).unwrap();
        let header_value = content_type.as_header_value();
        let from_header = ContentType::from_header_value(&header_value).unwrap();

        // Note: parameter information might be lost in this conversion
        // so we only check the main type matches
        assert_eq!(
            content_type.as_str().split(';').next(),
            from_header.as_str().split(';').next(),
            "HeaderValue conversion failed for: {input}"
        );
    }
}

// ===== EXTENSION-BASED DETECTION TESTS =====

#[test]
fn test_extension_detection() {
    let test_cases = vec![
        // Application
        ("json", Some(ContentType::Application(Application::Json))),
        ("pdf", Some(ContentType::Application(Application::Pdf))),
        ("zip", Some(ContentType::Application(Application::Zip))),
        (
            "docx",
            Some(ContentType::Application(Application::VndOpenXmlWordDoc)),
        ),
        (
            "xlsx",
            Some(ContentType::Application(Application::VndOpenXmlSpreadsheet)),
        ),
        // Text
        ("txt", Some(ContentType::Text(Text::Plain))),
        ("html", Some(ContentType::Text(Text::Html))),
        ("css", Some(ContentType::Text(Text::Css))),
        ("js", Some(ContentType::Text(Text::Javascript))),
        ("csv", Some(ContentType::Text(Text::Csv))),
        ("md", Some(ContentType::Text(Text::Markdown))),
        // Image
        ("jpg", Some(ContentType::Image(Image::Jpeg))),
        ("png", Some(ContentType::Image(Image::Png))),
        ("gif", Some(ContentType::Image(Image::Gif))),
        ("svg", Some(ContentType::Image(Image::SvgXml))),
        ("webp", Some(ContentType::Image(Image::Webp))),
        // Audio
        ("mp3", Some(ContentType::Audio(Audio::Mpeg))),
        ("wav", Some(ContentType::Audio(Audio::Wav))),
        ("flac", Some(ContentType::Audio(Audio::Flac))),
        // Video
        ("mp4", Some(ContentType::Video(Video::Mp4))),
        ("avi", Some(ContentType::Video(Video::XMsvideo))),
        ("mov", Some(ContentType::Video(Video::Quicktime))),
        // Unknown
        ("unknown", None),
        ("fake", None),
        ("", None),
    ];

    for (extension, expected) in test_cases {
        let result = ContentType::from_extension(extension);
        assert_eq!(
            result, expected,
            "Extension detection failed for: {extension}"
        );
    }
}

#[test]
fn test_filename_detection() {
    let test_cases = vec![
        (
            "document.pdf",
            Some(ContentType::Application(Application::Pdf)),
        ),
        ("image.jpg", Some(ContentType::Image(Image::Jpeg))),
        ("style.css", Some(ContentType::Text(Text::Css))),
        ("video.mp4", Some(ContentType::Video(Video::Mp4))),
        ("audio.mp3", Some(ContentType::Audio(Audio::Mpeg))),
        (
            "archive.zip",
            Some(ContentType::Application(Application::Zip)),
        ),
        ("no_extension", None),
        ("file.", None),
        ("", None),
    ];

    for (filename, expected) in test_cases {
        let result = ContentType::from_filename(filename);
        assert_eq!(
            result, expected,
            "Filename detection failed for: {filename}"
        );
    }
}

// ===== UTILITY METHOD TESTS =====

#[test]
fn test_content_type_categories() {
    // Test is_text
    assert!(ContentType::Text(Text::Plain).is_text());
    assert!(ContentType::Text(Text::Html).is_text());
    assert!(!ContentType::Application(Application::Json).is_text());

    // Test is_image
    assert!(ContentType::Image(Image::Jpeg).is_image());
    assert!(ContentType::Image(Image::Png).is_image());
    assert!(!ContentType::Text(Text::Plain).is_image());

    // Test is_media
    assert!(ContentType::Audio(Audio::Mpeg).is_media());
    assert!(ContentType::Video(Video::Mp4).is_media());
    assert!(!ContentType::Text(Text::Plain).is_media());

    // Test is_multipart
    assert!(ContentType::Multipart(Multipart::FormData).is_multipart());
    assert!(!ContentType::Text(Text::Plain).is_multipart());
}

#[test]
fn test_custom_content_types() {
    let custom_input = "application/vnd.custom+json";
    let result = ContentType::from_str(custom_input).unwrap();

    if let ContentType::Custom(custom) = result {
        assert_eq!(custom, custom_input);
    } else {
        panic!("Expected Custom variant for unknown content type");
    }
}

// ===== PERFORMANCE TESTS =====

#[test]
fn test_parsing_performance() {
    let test_inputs = vec![
        "application/json",
        "text/html",
        "image/jpeg",
        "audio/mpeg",
        "video/mp4",
    ];

    let start = std::time::Instant::now();

    for _ in 0..1000 {
        for input in &test_inputs {
            let _ = ContentType::from_str(input);
        }
    }

    let duration = start.elapsed();

    // Should parse 5000 content types quickly
    assert!(
        duration.as_millis() < 100,
        "Parsing performance too slow: {duration:?}"
    );
}
