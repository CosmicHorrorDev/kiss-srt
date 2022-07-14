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

const BAD_STARTING_TS: &str = "
1
00:00:00.000 ---> 00:01:23,456
Oh no!
The starting timestamp uses a '.' instead of a ',' :(
";

#[test]
fn bad_ts() {
    let err = kiss_srt::from_str(BAD_STARTING_TS).unwrap_err();
    assert_eq!(
        err,
        kiss_srt::Error {
            line: 3,
            kind: kiss_srt::error::ErrorKind::InvalidTimestampStart
        }
    );
}
