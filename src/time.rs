use std::{
    cmp,
    convert::TryFrom,
    fmt::Display,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

const MAX_HOURS: u64 = 100;
const MINUTES_PER_HOUR: u64 = 60;
const SECONDS_PER_MINUTE: u64 = 60;
const MILLIS_PER_SECOND: u64 = 1_000;

pub type Duration = Timestamp;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp(u64);

impl Timestamp {
    pub const MAX: Self =
        Self(MAX_HOURS * MINUTES_PER_HOUR * SECONDS_PER_MINUTE * MILLIS_PER_SECOND - 1);

    pub fn from_millis(total_millis: u64) -> Self {
        Self(total_millis)
    }

    pub fn new(hours: u8, minutes: u8, seconds: u8, millis: u16) -> Option<Self> {
        let hours = u64::from(hours);
        let minutes = u64::from(minutes);
        let seconds = u64::from(seconds);
        let millis = u64::from(millis);

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

    pub fn hours(&self) -> u8 {
        u8::try_from(self.total_hours()).expect("Hour should be within 0..100")
    }

    pub fn minutes(&self) -> u8 {
        let minutes = self.total_minutes() % MINUTES_PER_HOUR;
        u8::try_from(minutes).expect("Range is 0..60")
    }

    pub fn seconds(&self) -> u8 {
        let seconds = self.total_seconds() % SECONDS_PER_MINUTE;
        u8::try_from(seconds).expect("Range is 0..60")
    }

    pub fn millis(&self) -> u16 {
        let millis = self.total_millis() % MILLIS_PER_SECOND;
        u16::try_from(millis).expect("Range is 0..1_000")
    }

    pub fn total_hours(&self) -> u64 {
        self.total_minutes() / MINUTES_PER_HOUR
    }

    pub fn total_minutes(&self) -> u64 {
        self.total_seconds() / SECONDS_PER_MINUTE
    }

    pub fn total_seconds(&self) -> u64 {
        self.total_millis() / MILLIS_PER_SECOND
    }

    pub fn total_millis(&self) -> u64 {
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
                let millis = if millis >= 0.0 { millis as u64 } else { 0 };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_timestamp_is_max() {
        assert_eq!(Timestamp::MAX.to_string(), "99:59:59,999");
        // Saturates to max
        assert_eq!(Timestamp::MAX + Timestamp::from_millis(1), Timestamp::MAX);
    }

    #[test]
    fn min_timestamp_is_min() {
        let min = Timestamp::default();
        assert_eq!(min.to_string(), "00:00:00,000");
        // Saturates to min
        assert_eq!(min - Timestamp::from_millis(1), min);
    }

    #[test]
    fn float_shenanigans() {
        // Floats out of bounds will saturate
        assert_eq!(Timestamp::MAX * 1.1, Timestamp::MAX);
        assert_eq!(Timestamp::from_millis(1) * -1.0, Timestamp::default());
        assert_eq!(Timestamp::from_millis(1) * f32::INFINITY, Timestamp::MAX);
        assert_eq!(
            Timestamp::from_millis(1) * f32::NEG_INFINITY,
            Timestamp::default()
        );

        // Floats in bounds are fine
        assert_eq!(
            Timestamp::from_millis(1) * 100.0,
            Timestamp::from_millis(100)
        );
        assert_eq!(Timestamp::from_millis(10) * 0.1, Timestamp::from_millis(1));
    }
}
