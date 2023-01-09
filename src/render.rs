use std::fmt::Write;

use crate::Subtitle;

/// Renders the `subtitles` to the SRT text representation
///
/// ```
/// # use kiss_srt::{time::{Duration, Timestamp}, Subtitle};
/// let subtitles = vec![
///     Subtitle {
///         start: Timestamp::from_millis(0),
///         duration: Duration::from_millis(5_000),
///         text: String::from("Sample text"),
///     },
/// ];
/// const TEXT: &str = "\
/// 1
/// 00:00:00,000 --> 00:00:05,000
/// Sample text
/// ";
///
/// assert_eq!(kiss_srt::to_string(&subtitles), TEXT);
/// ```
pub fn to_string(subtitles: &[Subtitle]) -> String {
    let mut it = (1..).zip(subtitles.iter());

    it.next()
        .map(|(i, subtitle)| {
            let mut rendered = format!("{}\n{}\n", i, subtitle);

            for (i, subtitle) in it {
                write!(rendered, "\n{}\n{}\n", i, subtitle).expect("OOM ;-;");
            }
            rendered
        })
        .unwrap_or_default()
}
