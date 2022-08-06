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

#[test]
fn ts_trailing_bytes() {
    assert!(kiss_srt::from_str("1\n00:00:00,000 --> 11:11:11,111A\nTrailing 'A'\n").is_err())
}
