#![doc = include_str!("../README.md")]
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
