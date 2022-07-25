use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Error {
    pub line: usize,
    pub kind: ErrorKind,
}

impl Error {
    pub(crate) fn invalid_id(line: usize) -> Self {
        Self {
            line,
            kind: ErrorKind::InvalidId,
        }
    }

    pub(crate) fn invalid_ts_line(line: usize) -> Self {
        Self {
            line,
            kind: ErrorKind::InvalidTimestampLine,
        }
    }

    pub(crate) fn invalid_ts_start(line: usize) -> Self {
        Self {
            line,
            kind: ErrorKind::InvalidTimestampStart,
        }
    }

    pub(crate) fn invalid_ts_divider(line: usize) -> Self {
        Self {
            line,
            kind: ErrorKind::InvalidTimestampDivider,
        }
    }

    pub(crate) fn invalid_ts_end(line: usize) -> Self {
        Self {
            line,
            kind: ErrorKind::InvalidTimestampEnd,
        }
    }

    pub(crate) fn ts_end_before_start(line: usize) -> Self {
        Self {
            line,
            kind: ErrorKind::TimestampEndBeforeStart,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} on line {}", self.kind, self.line)
    }
}

impl std::error::Error for Error {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    InvalidId,
    InvalidTimestampLine,
    InvalidTimestampStart,
    InvalidTimestampDivider,
    InvalidTimestampEnd,
    TimestampEndBeforeStart,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidId => "Invalid ID-marker",
            Self::InvalidTimestampLine => "Invalid timestamp line",
            Self::InvalidTimestampStart => "Invalid starting timestamp",
            Self::InvalidTimestampDivider => "Invalid timestamp divider",
            Self::InvalidTimestampEnd => "Invalid ending timestamp",
            Self::TimestampEndBeforeStart => "End timestamp is before start",
        })
    }
}
