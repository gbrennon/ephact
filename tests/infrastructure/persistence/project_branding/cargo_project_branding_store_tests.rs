#[cfg(test)]
mod tests {
    use ephact::{
        application::ports::outbound::ProjectBrandingStorePort,
        infrastructure::persistence::CargoProjectBrandingStore,
    };

    #[test]
    fn read_project_branding_returns_cargo_metadata_and_custom_emblem() {
        let store = CargoProjectBrandingStore::new();

        let branding = store
            .read_project_branding()
            .expect("project branding should be read");

        assert_eq!(branding.name().as_str(), env!("CARGO_PKG_NAME"));
        assert_eq!(
            branding.description().as_str(),
            env!("CARGO_PKG_DESCRIPTION")
        );
        assert_eq!(branding.version().as_str(), env!("CARGO_PKG_VERSION"));
        assert_eq!(
            branding.emblem().as_str(),
            include_str!("../../../../assets/project_emblem.txt").trim_end()
        );
    }
}
