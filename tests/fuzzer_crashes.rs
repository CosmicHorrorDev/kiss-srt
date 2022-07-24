fn parse_render_roundtrip(s: &str) {
    if let Ok(subtitles) = kiss_srt::from_str(s) {
        let rendered = kiss_srt::to_string(&subtitles);
        let reparsed = kiss_srt::from_str(&rendered).unwrap();
        assert_eq!(subtitles, reparsed);
    }
}

#[test]
fn crash1() {
    parse_render_roundtrip("\n1\n11:11:11,111 --> 11:11:11,111\na\n\r\r");
}
