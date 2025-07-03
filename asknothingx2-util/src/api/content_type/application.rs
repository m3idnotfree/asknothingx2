define_mime_type! {
    pub enum Application {
        Json => {
            const: JSON_STR,
            mime: "application/json",
            extensions: ["json"],
        },
        Xml => {
            const: XML_STR,
            mime: "application/xml",
            extensions: ["xml"],
        },
        Pdf => {
            const: PDF_STR,
            mime: "application/pdf",
            extensions: ["pdf"],
        },
        Zip => {
            const: ZIP_STR,
            mime: "application/zip",
            extensions: ["zip"],
        },
        Gzip => {
            const: GZIP_STR,
            mime: "application/gzip",
            extensions: ["gz", "tgz"],
        },
        OctetStream => {
            const: OCTET_STREAM_STR,
            mime: "application/octet-stream",
            extensions: [],
        },
        FormUrlEncoded => {
            const: FORM_URL_ENCODED_STR,
            mime: "application/x-www-form-urlencoded",
            extensions: [],
        },
        Postscript => {
            const: POSTSCRIPT_STR,
            mime: "application/postscript",
            extensions: ["ps", "eps", "ai"],
        },
        Rtf => {
            const: RTF_STR,
            mime: "application/rtf",
            extensions: ["rtf"],
        },
        AtomXml => {
            const: ATOM_XML_STR,
            mime: "application/atom+xml",
            extensions: ["atom"],
        },
        RssXml => {
            const: RSS_XML_STR,
            mime: "application/rss+xml",
            extensions: ["rss"],
        },
        SoapXml => {
            const: SOAP_XML_STR,
            mime: "application/soap+xml",
            extensions: [],
        },
        XhtmlXml => {
            const: XHTML_XML_STR,
            mime: "application/xhtml+xml",
            extensions: ["xhtml"],
        },
        XsltXml => {
            const: XSLT_XML_STR,
            mime: "application/xslt+xml",
            extensions: ["xsl", "xslt"],
        },
        Yaml => {
            const: YAML_STR,
            mime: "application/yaml",
            extensions: ["yaml", "yml"],
        },
        Wasm => {
            const: WASM_STR,
            mime: "application/wasm",
            extensions: ["wasm"],
        },
        // Microsoft Office
        MsWord => {
            const: MS_WORD_STR,
            mime: "application/msword",
            extensions: ["doc", "dot"],
        },
        MsExcel => {
            const: MS_EXCEL_STR,
            mime: "application/vnd.ms-excel",
            extensions: ["xls", "xla", "xlb", "xlc", "xlm", "xlt", "xlw"],
        },
        MsPowerpoint => {
            const: MS_POWERPOINT_STR,
            mime: "application/vnd.ms-powerpoint",
            extensions: ["ppt", "pot", "ppa", "pps", "pwz"],
        },
        MsProject => {
            const: MS_PROJECT_STR,
            mime: "application/vnd.ms-project",
            extensions: ["mpp", "mpt"],
        },
        MsWorks => {
            const: MS_WORKS_STR,
            mime: "application/vnd.ms-works",
            extensions: ["wcm", "wdb", "wks", "wps"],
        },
        MsVisio => {
            const: MS_VISIO_STR,
            mime: "application/vnd.visio",
            extensions: ["vsd", "vss", "vst", "vsw"],
        },
        MsOneNote => {
            const: MS_ONENOTE_STR,
            mime: "application/onenote",
            extensions: ["one", "onepkg", "onetmp", "onetoc", "onetoc2"],
        },
        // Office Open XML (newer Office formats)
        VndOpenXmlWordDoc => {
            const: VND_OPENXML_WORD_DOC_STR,
            mime: "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            extensions: ["docx"],
        },
        VndOpenXmlWordTemplate => {
            const: VND_OPENXML_WORD_TEMPLATE_STR,
            mime: "application/vnd.openxmlformats-officedocument.wordprocessingml.template",
            extensions: ["dotx"],
        },
        VndOpenXmlSpreadsheet => {
            const: VND_OPENXML_SPREADSHEET_STR,
            mime: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            extensions: ["xlsx"],
        },
        VndOpenXmlSpreadsheetTemplate => {
            const: VND_OPENXML_SPREADSHEET_TEMPLATE_STR,
            mime: "application/vnd.openxmlformats-officedocument.spreadsheetml.template",
            extensions: ["xltx"],
        },
        VndOpenXmlPresentation => {
            const: VND_OPENXML_PRESENTATION_STR,
            mime: "application/vnd.openxmlformats-officedocument.presentationml.presentation",
            extensions: ["pptx"],
        },
        VndOpenXmlPresentationTemplate => {
            const: VND_OPENXML_PRESENTATION_TEMPLATE_STR,
            mime: "application/vnd.openxmlformats-officedocument.presentationml.template",
            extensions: ["potx"],
        },
        // OpenDocument formats
        VndOasisText => {
            const: VND_OASIS_TEXT_STR,
            mime: "application/vnd.oasis.opendocument.text",
            extensions: ["odt"],
        },
        VndOasisSpreadsheet => {
            const: VND_OASIS_SPREADSHEET_STR,
            mime: "application/vnd.oasis.opendocument.spreadsheet",
            extensions: ["ods"],
        },
        VndOasisPresentation => {
            const: VND_OASIS_PRESENTATION_STR,
            mime: "application/vnd.oasis.opendocument.presentation",
            extensions: ["odp"],
        },
        VndOasisGraphics => {
            const: VND_OASIS_GRAPHICS_STR,
            mime: "application/vnd.oasis.opendocument.graphics",
            extensions: ["odg"],
        },
        VndOasisFormula => {
            const: VND_OASIS_FORMULA_STR,
            mime: "application/vnd.oasis.opendocument.formula",
            extensions: ["odf"],
        },
        VndOasisDatabase => {
            const: VND_OASIS_DATABASE_STR,
            mime: "application/vnd.oasis.opendocument.database",
            extensions: ["odb"],
        },
        // Compression and archives
        X7zCompressed => {
            const: X_7Z_COMPRESSED_STR,
            mime: "application/x-7z-compressed",
            extensions: ["7z"],
        },
        XRarCompressed => {
            const: X_RAR_COMPRESSED_STR,
            mime: "application/x-rar-compressed",
            extensions: ["rar"],
        },
        XTar => {
            const: X_TAR_STR,
            mime: "application/x-tar",
            extensions: ["tar"],
        },
        XBzip2 => {
            const: X_BZIP2_STR,
            mime: "application/x-bzip2",
            extensions: ["bz2", "boz"],
        },
        XAceCompressed => {
            const: X_ACE_COMPRESSED_STR,
            mime: "application/x-ace-compressed",
            extensions: ["ace"],
        },
        XStuffit => {
            const: X_STUFFIT_STR,
            mime: "application/x-stuffit",
            extensions: ["sit", "sitx"],
        },
        VndDebian => {
            const: VND_DEBIAN_STR,
            mime: "application/vnd.debian.binary-package",
            extensions: ["deb", "udeb"],
        },
        VndRar => {
            const: VND_RAR_STR,
            mime: "application/vnd.rar",
            extensions: [],
        },
        // Programming and development
        JavaArchive => {
            const: JAVA_ARCHIVE_STR,
            mime: "application/java-archive",
            extensions: ["jar"],
        },
        JavaSerializedObject => {
            const: JAVA_SERIALIZED_OBJECT_STR,
            mime: "application/java-serialized-object",
            extensions: ["ser"],
        },
        JavaVm => {
            const: JAVA_VM_STR,
            mime: "application/java-vm",
            extensions: ["class"],
        },
        XShellScript => {
            const: X_SHELL_SCRIPT_STR,
            mime: "application/x-shellscript",
            extensions: ["sh"],
        },
        XPerl => {
            const: X_PERL_STR,
            mime: "application/x-perl",
            extensions: ["pl", "pm"],
        },
        XTcl => {
            const: X_TCL_STR,
            mime: "application/x-tcl",
            extensions: ["tcl"],
        },
        XPython => {
            const: X_PYTHON_STR,
            mime: "application/x-python",
            extensions: ["py", "pyc", "pyo", "pyd"],
        },
        XRuby => {
            const: X_RUBY_STR,
            mime: "application/x-ruby",
            extensions: ["rb"],
        },
        // Ebooks and documents
        EpubZip => {
            const: EPUB_ZIP_STR,
            mime: "application/epub+zip",
            extensions: ["epub"],
        },
        VndAmazonEbook => {
            const: VND_AMAZON_EBOOK_STR,
            mime: "application/vnd.amazon.ebook",
            extensions: ["azw"],
        },
        XMobipocketEbook => {
            const: X_MOBIPOCKET_EBOOK_STR,
            mime: "application/x-mobipocket-ebook",
            extensions: ["mobi", "prc"],
        },
        VndMsHtmlhelp => {
            const: VND_MS_HTMLHELP_STR,
            mime: "application/vnd.ms-htmlhelp",
            extensions: ["chm"],
        },
        // Database and data
        VndSqlite3 => {
            const: VND_SQLITE3_STR,
            mime: "application/vnd.sqlite3",
            extensions: ["sqlite", "sqlite3", "db"],
        },
        XNetcdf => {
            const: X_NETCDF_STR,
            mime: "application/x-netcdf",
            extensions: ["nc", "cdf"],
        },
        XHdf => {
            const: X_HDF_STR,
            mime: "application/x-hdf",
            extensions: ["hdf"],
        },
        VndMbox => {
            const: VND_MBOX_STR,
            mime: "application/mbox",
            extensions: ["mbox"],
        },
        // Adobe formats
        VndAdobeAir => {
            const: VND_ADOBE_AIR_STR,
            mime: "application/vnd.adobe.air-application-installer-package+zip",
            extensions: ["air"],
        },
        VndAdobeXdp => {
            const: VND_ADOBE_XDP_STR,
            mime: "application/vnd.adobe.xdp+xml",
            extensions: ["xdp"],
        },
        VndAdobeXfdf => {
            const: VND_ADOBE_XFDF_STR,
            mime: "application/vnd.adobe.xfdf",
            extensions: ["xfdf"],
        },
        VndAdobePhotoshop => {
            const: VND_ADOBE_PHOTOSHOP_STR,
            mime: "image/vnd.adobe.photoshop",
            extensions: ["psd"],
        },
        // Google formats
        VndGoogleEarthKml => {
            const: VND_GOOGLE_EARTH_KML_STR,
            mime: "application/vnd.google-earth.kml+xml",
            extensions: ["kml"],
        },
        VndGoogleEarthKmz => {
            const: VND_GOOGLE_EARTH_KMZ_STR,
            mime: "application/vnd.google-earth.kmz",
            extensions: ["kmz"],
        },
        // Apple formats
        VndAppleInstaller => {
            const: VND_APPLE_INSTALLER_STR,
            mime: "application/vnd.apple.installer+xml",
            extensions: ["mpkg"],
        },
        // Android
        VndAndroidPackage => {
            const: VND_ANDROID_PACKAGE_STR,
            mime: "application/vnd.android.package-archive",
            extensions: ["apk"],
        },
        // CAD and engineering
        VndAutocadDwg => {
            const: VND_AUTOCAD_DWG_STR,
            mime: "application/vnd.dwg",
            extensions: ["dwg"],
        },
        VndAutocadDxf => {
            const: VND_AUTOCAD_DXF_STR,
            mime: "application/vnd.dxf",
            extensions: ["dxf"],
        },
    }
}
