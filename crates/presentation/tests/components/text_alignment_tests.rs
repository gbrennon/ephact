#[cfg(test)]
mod tests {
    use ephact::presentation::components::text_alignment::TextAlignment;

    #[test]
    fn centers_visible_text_using_display_width() {
        assert_eq!(
            TextAlignment::Center.line("   Title    ", 17),
            "      Title      "
        );
    }

    #[test]
    fn centers_odd_remaining_space_with_extra_trailing_padding() {
        assert_eq!(
            TextAlignment::Center.line("Title", 18),
            "      Title       "
        );
    }

    #[test]
    fn keeps_start_aligned_text_at_the_leading_edge() {
        assert_eq!(TextAlignment::Start.line("Title", 17), "Title            ");
    }
}
