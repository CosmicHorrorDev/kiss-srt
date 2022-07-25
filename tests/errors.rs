macro_rules! snapshot_parse_err {
    ( $( ($fn_name:ident, $text:expr ) ),* $(,)? ) => {
        $(
        #[test]
        fn $fn_name() {
            let err = kiss_srt::from_str($text).unwrap_err();
            ::insta::assert_snapshot!(err.to_string());
        }
        )*
    };
}

const BAD_START_TS: &str = "\
1
00:00:00.000 --> 00:01:23,456
Oh no!
The starting timestamp uses a '.' instead of a ',' :(
";

const INVALID_TS_DIGIT: &str = "\
1
00:00:00,000 --> 11:11:1l,111
There is an 'l' instead of a 'l' in the end ts
";

const BAD_ID: &str = "\
bad id
00:00:00,000 --> 11:11:11,111
'bad id' is a bad id
";

const TS_END_BEFORE_START: &str = "\
1
12:34:56,789 --> 12:34:56,788
Text with time travel
";

const TS_OUT_OF_BOUNDS: &str = "\
1
00:60:00,000 --> 11:11:11,111
60 minutes is invalid. It should just be an hour
";

const MISSING_TS_LINE: &str = "\
1
";

const INVALID_TS_DIVIDER: &str = "\
1
00:00:00,000 ---> 11:11:11,111
The timestamp divider has one too many '-'
";

snapshot_parse_err!(
    (bad_start_ts, BAD_START_TS),
    (invalid_ts_digit, INVALID_TS_DIGIT),
    (bad_id, BAD_ID),
    (ts_end_before_start, TS_END_BEFORE_START),
    (ts_out_of_bounds, TS_OUT_OF_BOUNDS),
    (missing_ts_line, MISSING_TS_LINE),
    (invalid_ts_divider, INVALID_TS_DIVIDER),
);
