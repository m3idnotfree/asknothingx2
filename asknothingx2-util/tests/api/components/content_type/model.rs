use asknothingx2_util::api::content_type::Model;

define_tests!(Model);

prop_compose! {
    pub fn base_stragety()(
        s in prop_oneof![
            Just(Model::Iges),
            Just(Model::Mesh),
            Just(Model::Vrml),
            Just(Model::VndDwf),
            Just(Model::VndGdl),
            Just(Model::VndGtw),
            Just(Model::VndMts),
            Just(Model::VndVtu),
    ]) -> Model { s }
}
