pub trait Component {
    fn render(&self) -> String;
}

impl<F> Component for F
where
    F: Fn() -> String,
{
    fn render(&self) -> String {
        self()
    }
}
