//! A stupidly simple SRT subtitles parser/renderer
//!
//! ## The Gist
//!
//! The main entrypoints into this API are [`from_str()`] and [`to_string()`]. Both of
//! those deal with a sequence of [`Subtitle`]s that can be manipulated to your
//! heart's content
//!
//! ## Quickstart
//!
//! Here's a simple example that
//!
//! 1. Parses some SRT text
//! 2. Removes the first and last entry
//! 3. Shifts all the timestamps up by 500ms
//! 4. Renders back to SRT text
//!
//! ```rust
//! const SAMPLE_SRT_TEXT: &str = "\
//! 1
//! 00:00:00,000 --> 00:00:05,250
//! Will be removed
//!
//! 2
//! 00:00:06,000 --> 00:00:10,000
//! Will be kept
//!
//! 3
//! 00:00:15,500 --> 00:00:20,750
//! Will also be removed
//! ";
//!
//! // 1. Parses some SRT text
//! let subtitles = kiss_srt::from_str(SAMPLE_SRT_TEXT).unwrap();
//! // 2. Removes the first and last entry
//! let mut middle = match subtitles.as_slice() {
//!     [_, middle @ .., _] => middle.to_owned(),
//!     _ => panic!("Needs at least two entries"),
//! };
//!
//! // 3. Shifts all the timestamps up by 500ms
//! for subtitle in &mut middle {
//!     subtitle.start += kiss_srt::Duration::from_millis(500);
//! }
//!
//! // 4. Renders back to SRT text
//! const PRUNED_AND_SHIFTED: &str = "\
//! 1
//! 00:00:06,500 --> 00:00:10,500
//! Will be kept
//! ";
//! assert_eq!(kiss_srt::to_string(&middle), PRUNED_AND_SHIFTED);
//! ```
//!
//! Let's take a moment to appreciate how simple this was
//!
//! We didn't have to worry about shifting the IDs to make it start at one again (it
//! just gets inferred from the index), and we didn't have to worry about shifting
//! both the start and end timestamp because the subtitles use a start and
//! _duration_ instead
//!
//! There is beauty in simplicity 💕

// TODO: setup github actions
mod error;
mod parse;
mod render;
mod time;

use std::fmt::Display;

pub use error::{Error, Result};
pub use parse::from_str;
pub use render::to_string;
pub use time::{Duration, Timestamp};

/// Represents a single SRT subtitle item
///
/// Composed of the `start`ing timestamp, `duration` (aka end - start), and UTF-8 `text` for the
/// subtitle. The ID is inferred as index + 1 when rendering with [`to_string`]
///
/// **⚠  WARN: This type is only misuse-resistent ⚠**
///
/// The `start` and `duration` are always valid, but
/// the `text` must not contain an empty line to avoid rendering invalid subtitles. This was a
/// deliberate tradeoff to keep rendering "infallible" while keeping the `text` easy to use
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Subtitle {
    pub start: Timestamp,
    pub duration: Duration,
    pub text: String,
}

impl Display for Subtitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let end = self.start + self.duration;
        write!(f, "{} --> {}\n{}", self.start, end, self.text)
    }
}
