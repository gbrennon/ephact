use std::error::Error;

use ephact::{
    application::{
        dtos::ShowProjectBrandingInfoResponse,
        ports::{inbound::ShowProjectBrandingInfoPort, outbound::ProjectBrandingStorePort},
        services::show_project_branding_info_service::ShowProjectBrandingInfoService,
    },
    domain::{ProjectBranding, ProjectDescription, ProjectEmblem, ProjectName, ProjectVersion},
};

struct FakeBrandingStore {
    result: Result<ProjectBranding, String>,
}

impl ProjectBrandingStorePort for FakeBrandingStore {
    fn read_project_branding(&self) -> Result<ProjectBranding, Box<dyn Error>> {
        match &self.result {
            Ok(branding) => Ok(branding.clone()),
            Err(message) => Err(std::io::Error::other(message.as_str()).into()),
        }
    }
}

fn branding() -> ProjectBranding {
    ProjectBranding::new(
        ProjectName::new("ephact".to_string()).unwrap(),
        ProjectDescription::new("Ephemeral action runner".to_string()).unwrap(),
        ProjectVersion::new("0.1.0".to_string()).unwrap(),
        ProjectEmblem::new("shield".to_string()).unwrap(),
    )
}

#[test]
fn execute_maps_domain_branding_to_primitive_response() {
    let service = ShowProjectBrandingInfoService::new(Box::new(FakeBrandingStore {
        result: Ok(branding()),
    }));

    let response = service.execute().unwrap();

    assert_eq!(
        response,
        ShowProjectBrandingInfoResponse::new("ephact".to_string(), "Ephemeral action runner".to_string(), "0.1.0".to_string(), "shield".to_string())
    );
}

#[test]
fn execute_propagates_branding_store_failure() {
    let service = ShowProjectBrandingInfoService::new(Box::new(FakeBrandingStore {
        result: Err("branding unavailable".to_string()),
    }));

    let error = service.execute().unwrap_err();

    assert!(error.to_string().contains("branding unavailable"));
}
