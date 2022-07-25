const SANITY: &str = "
1
00:00:00,000 --> 00:01:23,456
This is some sample
text

2
00:02:34,567 --> 00:03:00,000
3
00:00:00,000 --> 11:11:11,111
^^ Wow. That looks a lot like a subtitle, but it isn't
";

#[test]
fn sanity() {
    let subtitles = kiss_srt::from_str(SANITY).unwrap();
    insta::assert_snapshot!(kiss_srt::to_string(&subtitles));
}

#[test]
fn mutations() {
    let subtitles = kiss_srt::from_str(SANITY).unwrap();

    let increase: Vec<_> = subtitles
        .clone()
        .into_iter()
        .map(|mut sub| {
            sub.start += kiss_srt::Duration::from_millis(100);
            sub
        })
        .collect();
    insta::assert_snapshot!(kiss_srt::to_string(&increase));

    let decrease: Vec<_> = subtitles
        .clone()
        .into_iter()
        .map(|mut sub| {
            sub.start -= kiss_srt::Duration::from_millis(100);
            sub
        })
        .collect();
    insta::assert_snapshot!(kiss_srt::to_string(&decrease));

    let scaled_up: Vec<_> = subtitles
        .clone()
        .into_iter()
        .map(|mut sub| {
            sub.start *= 1.1;
            sub.duration *= 1.1;
            sub
        })
        .collect();
    insta::assert_snapshot!(kiss_srt::to_string(&scaled_up));

    let mut removed = subtitles.clone();
    let _ = removed.remove(0);
    insta::assert_snapshot!(kiss_srt::to_string(&removed));
}

const EMPTY_TEXT_SECTION: &str = "\
1
00:00:00,000 --> 11:11:11,111

";

#[test]
fn empty_text_section() {
    let subtitles = kiss_srt::from_str(EMPTY_TEXT_SECTION).unwrap();
    insta::assert_snapshot!(kiss_srt::to_string(&subtitles));
}
