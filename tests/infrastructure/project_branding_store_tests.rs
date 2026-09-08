use ephact::{
    application::ports::outbound::ProjectBrandingStorePort,
    infrastructure::project_branding_store::CargoProjectBrandingStore,
};

#[test]
fn read_project_branding_returns_cargo_metadata_and_custom_emblem() {
    let store = CargoProjectBrandingStore;

    let branding = store.read_project_branding().unwrap();

    assert_eq!(branding.name().as_str(), env!("CARGO_PKG_NAME"));
    assert_eq!(
        branding.description().as_str(),
        env!("CARGO_PKG_DESCRIPTION")
    );
    assert_eq!(branding.version().as_str(), env!("CARGO_PKG_VERSION"));
    assert_eq!(
        branding.emblem().as_str(),
        include_str!("../../assets/project_emblem.txt").trim_end()
    );
}
