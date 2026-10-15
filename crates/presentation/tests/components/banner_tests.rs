#[cfg(test)]
mod tests {
    use ephact::{
        PROJECT_EMBLEM,
        application::dtos::responses::ShowProjectBrandingInfoResponse,
        presentation::components::{
            banner::Banner, box_component::BoxComponent, project_branding::ProjectBranding,
            terminal::Terminal,
        },
    };

    struct FakeTerminal {
        width: usize,
    }

    impl Terminal for FakeTerminal {
        fn dimensions(&self) -> (usize, usize) {
            (self.width, 40)
        }

        fn write_text(&self, _text: &str) -> std::io::Result<()> {
            Ok(())
        }

        fn read_line(&self) -> std::io::Result<String> {
            Ok(String::new())
        }
    }

    #[test]
    fn centered_banner_keeps_border_and_branding_on_one_axis() {
        let branding = ShowProjectBrandingInfoResponse::new(
            "ephact".to_owned(),
            "run workflows locally in containers".to_owned(),
            "0.4.0".to_owned(),
            PROJECT_EMBLEM.to_owned(),
        );
        let rendered = BoxComponent::centered(
            Banner::new(&branding, ProjectBranding::CLI_HINT),
            &FakeTerminal { width: 80 },
        )
        .with_title("the net is vast and infinite")
        .render();
        let lines = rendered.lines().collect::<Vec<_>>();
        let border_start = lines[0].chars().position(|glyph| glyph == '┌').unwrap();
        let border_end = lines[0]
            .chars()
            .enumerate()
            .filter_map(|(index, glyph)| (glyph == '┐').then_some(index))
            .last()
            .expect("find right border");
        let border_center = border_start + border_end;

        assert_eq!(lines[0].chars().count(), 80);
        assert!(lines.last().unwrap().contains("└"));
        assert!(lines.last().unwrap().contains("┘"));
        assert!(lines.iter().any(|line| line.contains("@----o")));
        assert!(lines.iter().any(|line| line.contains("EPHACT")));
        assert!(
            lines
                .iter()
                .any(|line| line.contains("run workflows locally in containers"))
        );
        assert!(
            lines
                .iter()
                .any(|line| line.contains(ProjectBranding::CLI_HINT))
        );
        let emblem_row = lines
            .iter()
            .position(|line| line.contains("@----o"))
            .unwrap();
        let title_row = lines
            .iter()
            .position(|line| line.contains("EPHACT"))
            .unwrap();
        let subtitle_row = lines
            .iter()
            .position(|line| line.contains(ProjectBranding::SUBTITLE))
            .unwrap();
        let hint_row = lines
            .iter()
            .position(|line| line.contains(ProjectBranding::CLI_HINT))
            .unwrap();
        assert!(emblem_row < title_row);
        assert!(title_row < subtitle_row);
        assert!(subtitle_row < hint_row);

        for line in &lines[1..lines.len() - 1] {
            let left_border = line.chars().position(|glyph| glyph == '│').unwrap();
            let right_border = line
                .chars()
                .enumerate()
                .filter_map(|(index, glyph)| (glyph == '│').then_some(index))
                .last()
                .expect("find right border");
            assert_eq!(left_border, border_start, "left border drift: {line:?}");
            assert_eq!(right_border, border_end, "right border drift: {line:?}");
            let content = line
                .chars()
                .skip(left_border + 1)
                .take(right_border - left_border - 1)
                .collect::<Vec<_>>();
            let Some(first) = content.iter().position(|glyph| !glyph.is_whitespace()) else {
                continue;
            };
            let last = content
                .iter()
                .enumerate()
                .filter_map(|(index, glyph)| (!glyph.is_whitespace()).then_some(index))
                .next_back()
                .expect("find content end");
            let content_center = first + last + 2 * (left_border + 1);
            assert!(
                content_center.abs_diff(border_center) <= 1,
                "content is off-axis: line={line:?}, content_center={content_center}, border_center={border_center}"
            );
        }
    }

    #[test]
    fn centered_banner_keeps_border_columns_at_narrow_width() {
        let branding = ShowProjectBrandingInfoResponse::new(
            "ephact".to_owned(),
            "run workflows locally in containers".to_owned(),
            "0.4.0".to_owned(),
            PROJECT_EMBLEM.to_owned(),
        );
        let rendered = BoxComponent::centered(
            Banner::new(&branding, ProjectBranding::CLI_HINT),
            &FakeTerminal { width: 40 },
        )
        .with_title("the net is vast and infinite")
        .render();
        let lines = rendered.lines().collect::<Vec<_>>();
        let border_start = lines[0].chars().position(|glyph| glyph == '┌').unwrap();
        let border_end = lines[0]
            .chars()
            .enumerate()
            .filter_map(|(index, glyph)| (glyph == '┐').then_some(index))
            .last()
            .expect("find right border");

        assert!(lines.iter().all(|line| line.chars().count() == 40));
        for line in &lines[1..lines.len() - 1] {
            assert_eq!(
                line.chars().position(|glyph| glyph == '│'),
                Some(border_start)
            );
            assert_eq!(
                line.chars()
                    .enumerate()
                    .filter_map(|(index, glyph)| (glyph == '│').then_some(index))
                    .last(),
                Some(border_end)
            );
        }
    }
}
