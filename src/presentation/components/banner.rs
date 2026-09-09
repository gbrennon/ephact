use figlet_rs::FIGlet;

use super::component::Component;
use crate::application::dtos::ShowProjectBrandingInfoResponse;

pub struct Banner<'a> {
    response: &'a ShowProjectBrandingInfoResponse,
}

impl<'a> Banner<'a> {
    pub fn new(response: &'a ShowProjectBrandingInfoResponse) -> Self {
        Self { response }
    }
}

impl Component for Banner<'_> {
    fn render(&self) -> String {
        let font = FIGlet::standard().expect("the bundled standard FIGlet font must be valid");
        let ascii_name = font
            .convert(&self.response.name())
            .expect("the crate name must be renderable by the bundled standard FIGlet font");

        format!(
            "{}\n\n{}\n\n{}\n\nVersion {}",
            ascii_name.as_str().trim_end(),
            self.response.emblem(),
            self.response.description(),
            self.response.version()
        )
    }
}
