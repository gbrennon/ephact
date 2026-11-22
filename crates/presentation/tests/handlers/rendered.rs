use ephact::{
    application::dtos::responses::RunSummaryResponse, presentation::handlers::RunHandler,
};

pub struct Rendered {
    text: String,
}

impl Rendered {
    pub fn of(summary: &RunSummaryResponse) -> Self {
        Self {
            text: RunHandler::render(summary),
        }
    }

    pub fn line(&self, index: usize) -> &str {
        self.text.lines().nth(index).unwrap()
    }

    pub fn lines(&self) -> std::str::Lines<'_> {
        self.text.lines()
    }
}
