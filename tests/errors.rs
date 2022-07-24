const BAD_STARTING_TS: &str = "
1
00:00:00.000 ---> 00:01:23,456
Oh no!
The starting timestamp uses a '.' instead of a ',' :(
";

#[test]
fn bad_start_ts() {
    let err = kiss_srt::from_str(BAD_STARTING_TS).unwrap_err();
    insta::assert_snapshot!(err.to_string());
}
