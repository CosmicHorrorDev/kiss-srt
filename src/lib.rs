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

// NOTE: No `Default` because empty `text` is considered invalid, so it would be a silly default
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
