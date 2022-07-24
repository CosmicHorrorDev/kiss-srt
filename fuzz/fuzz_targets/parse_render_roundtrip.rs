#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|s: &str| {
    if let Ok(subtitles) = kiss_srt::from_str(s) {
        let rendered = kiss_srt::to_string(&subtitles);
        let reparsed = kiss_srt::from_str(&rendered).unwrap();
        assert_eq!(subtitles, reparsed);
    }
});
