define_mime_type! {
    pub enum Chemical {
        XCdx => {
            const: X_CDX_STR,
            mime: "chemical/x-cdx",
            extensions: ["cdx"],
        },
        XCif => {
            const: X_CIF_STR,
            mime: "chemical/x-cif",
            extensions: ["cif"],
        },
        XCml => {
            const: X_CML_STR,
            mime: "chemical/x-cml",
            extensions: ["cml"],
        },
        XCsml => {
            const: X_CSML_STR,
            mime: "chemical/x-csml",
            extensions: ["csml"],
        },
        XXyz => {
            const: X_XYZ_STR,
            mime: "chemical/x-xyz",
            extensions: ["xyz"],
        },
    }
}
