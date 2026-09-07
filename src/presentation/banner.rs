/// ASCII banner shown when the application starts.
///
/// Keeps the startup branding in a single, testable place: the rendering is
/// pure and `main` only decides where (and when) to print it.
const BANNER_ART: &str = r#"
 _          _    _
| |__   ___| | _| |_ _____  _
| '_ \ / _ \ |/ / __/ _ \ \ / /
| |_) |  __/   <| ||  __/\ V /
|_.__/ \___|_|\_\\__\___| \_/ "#;

const BANNER_TAGLINE: &str =
    "ephact - Runs GitHub Actions-style workflows in an ephemeral repository:
     a throwaway copy of your repository that is discarded when the run ends.";

/// Renders the startup banner: the tool name as ASCII art followed by a
/// short explanation of the ephemeral repository model.
pub fn render() -> String {
    format!("{BANNER_ART}\n{BANNER_TAGLINE}\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_includes_the_tool_name() {
        assert!(render().contains("ephact"));
    }

    #[test]
    fn render_mentions_the_ephemeral_repository() {
        let rendered = render();
        assert!(rendered.contains("ephemeral repository"));
        assert!(rendered.contains("discarded"));
    }

    #[test]
    fn render_contains_only_ascii_characters() {
        assert!(render().is_ascii());
    }

    #[test]
    fn render_is_multiline_and_terminated() {
        let rendered = render();
        assert!(rendered.lines().count() > 2);
        assert!(rendered.ends_with('\n'));
    }
}
