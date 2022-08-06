use std::{
    cmp,
    convert::TryFrom,
    fmt::Display,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

const MAX_HOURS: u32 = 100;
const MINUTES_PER_HOUR: u32 = 60;
const SECONDS_PER_MINUTE: u32 = 60;
const MILLIS_PER_SECOND: u32 = 1_000;

/// A `Duration` behaves the same as a [`Timestamp`]
pub type Duration = Timestamp;

/// Represents an SRT timestamp
///
/// ```
/// # use kiss_srt::Timestamp;
/// let ts = Timestamp::new(12, 34, 56, 789)
///     .expect("timestamp is within range");
///
/// assert_eq!(ts.to_string(), "12:34:56,789");
///
/// // Check each specific component
/// assert_eq!(ts.hours(), 12);
/// assert_eq!(ts.minutes(), 34);
/// assert_eq!(ts.seconds(), 56);
/// assert_eq!(ts.millis(), 789);
///
/// // Check totals for each component
/// assert_eq!(ts.total_hours(), 12);
/// assert_eq!(ts.total_minutes(), 12 * 60 + 34);
/// assert_eq!(ts.total_seconds(), (12 * 60 + 34) * 60 + 56);
/// assert_eq!(ts.total_millis(), ((12 * 60 + 34) * 60 + 56) * 1_000 + 789);
/// ```
///
/// Operations for addition, subtraction, and multiplication are all supported. All of these
/// operations will saturate either down to `Timestamp::from_millis(0)` aka [`Timestamp::default()`]
/// or up to [`Timestamp::MAX`]
///
/// ```
/// # use kiss_srt::{Duration, Timestamp};
/// // Simple addition
/// let half_sec = Timestamp::from_millis(500);
/// let sec_and_a_half = half_sec + Duration::from_millis(1_000);
///
/// // Saturating a value to 0
/// let mut saturating_to_zero = Timestamp::from_millis(1_000);
/// saturating_to_zero -= Duration::from_millis(500);
/// assert_eq!(saturating_to_zero, half_sec);
/// saturating_to_zero -= Duration::from_millis(1_000);
/// assert_eq!(saturating_to_zero, Timestamp::default());
///
/// // Scaling a timestamp
/// assert_eq!(half_sec * 1.1, Timestamp::from_millis(550));
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp(u32);

impl Timestamp {
    /// The max possible timestamp
    ///
    /// ```
    /// # use kiss_srt::Timestamp;
    /// assert_eq!(Timestamp::MAX.to_string(), "99:59:59,999");
    /// ```
    pub const MAX: Self =
        Self(MAX_HOURS * MINUTES_PER_HOUR * SECONDS_PER_MINUTE * MILLIS_PER_SECOND - 1);

    /// Attempts to construct a timestamp returning `None` when above `Timestamp::MAX`
    ///
    /// ```
    /// # use kiss_srt::Timestamp;
    /// // Within the range
    /// assert!(Timestamp::checked_from_millis(1_234).is_some());
    /// // Just outside of the range
    /// assert!(
    ///     Timestamp::checked_from_millis(
    ///         Timestamp::MAX.total_millis() + 1
    ///     ).is_none()
    /// );
    /// ```
    pub fn checked_from_millis(total_millis: u32) -> Option<Self> {
        if Self(total_millis) <= Self::MAX {
            Some(Self(total_millis))
        } else {
            None
        }
    }

    /// Constructs a timestamp saturating to `Timestamp::MAX`
    ///
    /// ```
    /// # use kiss_srt::Timestamp;
    /// // Within the range
    /// assert_eq!(Timestamp::from_millis(1_234).total_millis(), 1_234);
    /// // Saturates to `Timestamp::MAX`
    /// assert_eq!(
    ///     Timestamp::from_millis(
    ///         Timestamp::MAX.total_millis() + 1
    ///     ),
    ///     Timestamp::MAX,
    /// );
    /// ```
    pub fn from_millis(total_millis: u32) -> Self {
        // Saturates the value to max
        cmp::min(Self(total_millis), Self::MAX)
    }

    /// Attempts to contruct a timestamp using the provided components
    ///
    /// Returns `None` if **any** of the components are outside the following ranges
    ///
    /// | Component | Range |
    /// |:---:|:---:|
    /// | `hours` | `0..100` |
    /// | `minutes` | `0..60` |
    /// | `seconds` | `0..60` |
    /// | `millis` | `0..1_000` |
    ///
    /// ```
    /// # use kiss_srt::Timestamp;
    /// assert_eq!(
    ///     Timestamp::new(12, 34, 56, 789)
    ///         .expect("timestamp is within range")
    ///         .to_string(),
    ///     "12:34:56,789",
    /// );
    /// // 60 minutes is outside of the accepted range
    /// assert!(Timestamp::new(0, 60, 0, 0).is_none());
    /// ```
    pub fn new(hours: u8, minutes: u8, seconds: u8, millis: u16) -> Option<Self> {
        let hours = u32::from(hours);
        let minutes = u32::from(minutes);
        let seconds = u32::from(seconds);
        let millis = u32::from(millis);

        if hours >= MAX_HOURS
            || minutes >= MINUTES_PER_HOUR
            || seconds >= SECONDS_PER_MINUTE
            || millis >= MILLIS_PER_SECOND
        {
            None
        } else {
            let total_minutes = hours * MINUTES_PER_HOUR + minutes;
            let total_seconds = total_minutes * SECONDS_PER_MINUTE + seconds;
            let total_millis = total_seconds * MILLIS_PER_SECOND + millis;

            Some(Self(total_millis))
        }
    }

    /// Returns just the hours component
    pub fn hours(&self) -> u8 {
        u8::try_from(self.total_hours()).expect("Hour should be within 0..100")
    }

    /// Returns just the minutes component
    pub fn minutes(&self) -> u8 {
        let minutes = self.total_minutes() % MINUTES_PER_HOUR;
        u8::try_from(minutes).expect("Range is 0..60")
    }

    /// Returns just the seconds component
    pub fn seconds(&self) -> u8 {
        let seconds = self.total_seconds() % SECONDS_PER_MINUTE;
        u8::try_from(seconds).expect("Range is 0..60")
    }

    /// Returns just the millis component
    pub fn millis(&self) -> u16 {
        let millis = self.total_millis() % MILLIS_PER_SECOND;
        u16::try_from(millis).expect("Range is 0..1_000")
    }

    /// Returns the total number of hours
    pub fn total_hours(&self) -> u32 {
        self.total_minutes() / MINUTES_PER_HOUR
    }

    /// Returns the total number of minutes
    pub fn total_minutes(&self) -> u32 {
        self.total_seconds() / SECONDS_PER_MINUTE
    }

    /// Returns the total number of seconds
    pub fn total_seconds(&self) -> u32 {
        self.total_millis() / MILLIS_PER_SECOND
    }

    /// Returns the total number of millis
    pub fn total_millis(&self) -> u32 {
        self.0
    }
}

impl Add for Timestamp {
    type Output = Self;

    fn add(self, other: Duration) -> Self {
        cmp::min(Self(self.0 + other.0), Timestamp::MAX)
    }
}

impl AddAssign for Timestamp {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Timestamp {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.saturating_sub(rhs.0))
    }
}

impl SubAssign for Timestamp {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

// TODO: I really don't know how to cleanly deal with floats -> ints so this likely still has
// issues
macro_rules! gen_mul_float_traits {
    ($t:ty) => {
        impl Mul<$t> for Timestamp {
            type Output = Self;

            fn mul(self, rhs: $t) -> Self::Output {
                let millis = self.0 as $t * rhs;
                let millis = if millis >= 0.0 { millis as u32 } else { 0 };
                cmp::min(Self(millis), Timestamp::MAX)
            }
        }

        impl MulAssign<$t> for Timestamp {
            fn mul_assign(&mut self, rhs: $t) {
                *self = *self * rhs;
            }
        }
    };
}

gen_mul_float_traits!(f32);
gen_mul_float_traits!(f64);

impl Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02},{:03}",
            self.hours(),
            self.minutes(),
            self.seconds(),
            self.millis()
        )
    }
}
