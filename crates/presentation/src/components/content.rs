use super::component::Component;

pub struct ContentComponent {
    title: String,
    content: String,
}

impl ContentComponent {
    pub fn new(title: String, content: String) -> Self {
        Self { title, content }
    }
}

impl Component for ContentComponent {
    fn render(&self) -> String {
        if self.content.is_empty() {
            return self.title.clone();
        }
        format!("{}\n\n{}", self.title, self.content)
    }
}
