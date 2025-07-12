define_mime_type! {
    pub enum Text {
        Plain => {
            const: PLAIN_STR,
            mime: "text/plain",
            extensions: ["txt"],
        },
        Html => {
            const: HTML_STR,
            mime: "text/html",
            extensions: ["html", "htm"],
        },
        Css => {
            const: CSS_STR,
            mime: "text/css",
            extensions: ["css"],
        },
        Javascript => {
            const: JAVASCRIPT_STR,
            mime: "text/javascript",
            extensions: ["js", "mjs"],
        },
        Csv => {
            const: CSV_STR,
            mime: "text/csv",
            extensions: ["csv"],
        },
        Xml => {
            const: XML_STR,
            mime: "text/xml",
            extensions: ["xml"],
        },
        Markdown => {
            const: MARKDOWN_STR,
            mime: "text/markdown",
            extensions: ["md", "markdown"],
            aliases: ["text/x-markdown"]
        },
        Calendar => {
            const: CALENDAR_STR,
            mime: "text/calendar",
            extensions: ["ics"],
        },
        Richtext => {
            const: RICHTEXT_STR,
            mime: "text/richtext",
            extensions: ["rtx"],
            aliases: ["text/rtf"]
        },
        Sgml => {
            const: SGML_STR,
            mime: "text/sgml",
            extensions: ["sgml", "sgm"],
        },
        TabSeparatedValues => {
            const: TAB_SEPARATED_VALUES_STR,
            mime: "text/tab-separated-values",
            extensions: ["tsv"],
        },
        Troff => {
            const: TROFF_STR,
            mime: "text/troff",
            extensions: ["tr", "roff", "man", "me", "ms"],
        },
        UriList => {
            const: URI_LIST_STR,
            mime: "text/uri-list",
            extensions: ["uri", "uris", "urls"],
        },
        VCard => {
            const: VCARD_STR,
            mime: "text/x-vcard",
            extensions: ["vcf", "vcard"],
        },
        VCalendar => {
            const: VCALENDAR_STR,
            mime: "text/x-vcalendar",
            extensions: ["vcs"],
        },
        Setext => {
            const: SETEXT_STR,
            mime: "text/x-setext",
            extensions: ["etx"],
        },
        Uuencode => {
            const: UUENCODE_STR,
            mime: "text/x-uuencode",
            extensions: ["uu"],
        },
        Asm => {
            const: ASM_STR,
            mime: "text/x-asm",
            extensions: ["s", "asm"],
        },
        C => {
            const: C_STR,
            mime: "text/x-c",
            extensions: ["c", "cc", "cxx", "cpp", "h", "hh", "dic"],
        },
        Fortran => {
            const: FORTRAN_STR,
            mime: "text/x-fortran",
            extensions: ["f", "for", "f77", "f90"],
        },
        JavaSource => {
            const: JAVA_SOURCE_STR,
            mime: "text/x-java-source",
            extensions: ["java"],
        },
        Pascal => {
            const: PASCAL_STR,
            mime: "text/x-pascal",
            extensions: ["p", "pas"],
        },
        Python => {
            const: PYTHON_STR,
            mime: "text/x-python",
            extensions: ["py"],
        },
    }
}
