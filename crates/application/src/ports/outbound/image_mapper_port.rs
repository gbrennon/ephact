pub trait ImageMapperPort: Send + Sync {
    fn map(&self, platform: &str) -> String;
    fn fallback(&self) -> String;
    fn clone_box(&self) -> Box<dyn ImageMapperPort>;
}

impl Clone for Box<dyn ImageMapperPort> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
