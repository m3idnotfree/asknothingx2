define_mime_type! {
    pub enum Model {
        Iges => {
            const: IGES_STR,
            mime: "model/iges",
            extensions: ["igs", "iges"],
        },
        Mesh => {
            const: MESH_STR,
            mime: "model/mesh",
            extensions: ["msh", "mesh"],
        },
        Vrml => {
            const: VRML_STR,
            mime: "model/vrml",
            extensions: ["wrl", "vrml"],
        },
        VndDwf => {
            const: VND_DWF_STR,
            mime: "model/vnd.dwf",
            extensions: ["dwf"],
        },
        VndGdl => {
            const: VND_GDL_STR,
            mime: "model/vnd.gdl",
            extensions: ["gdl"],
        },
        VndGtw => {
            const: VND_GTW_STR,
            mime: "model/vnd.gtw",
            extensions: ["gtw"],
        },
        VndMts => {
            const: VND_MTS_STR,
            mime: "model/vnd.mts",
            extensions: ["mts"],
        },
        VndVtu => {
            const: VND_VTU_STR,
            mime: "model/vnd.vtu",
            extensions: ["vtu"],
        },
    }
}
