extern crate chrono;

use self::chrono::NaiveDate;
use chrono::{DateTime, Datelike, Local};
use std::fmt::{Display, Formatter};
use std::{cmp::*, ops::Sub};

/// Date structure, which contains day, month and year.
/// It's a copyable type like i32 (no move).
/// You can clone, debug and compare as == / !=

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Date {
    pub(crate) day: u8,
    pub(crate) month: u8,
    pub(crate) year: u16,
}

impl PartialOrd for Date {
    /// Dates can be compared as >, <, >=, <=
    /// (as it works in real world)

    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return if self.year < other.year {
            Some(Ordering::Less)
        } else if self.year > other.year {
            Some(Ordering::Greater)
        } else {
            if self.month < other.month {
                Some(Ordering::Less)
            } else if self.month > other.month {
                Some(Ordering::Greater)
            } else {
                self.day.partial_cmp(&other.day)
            }
        };
    }
}

impl Ord for Date {
    /// Dates can be compared as >, <, >=, <=
    /// (as it works in real world)

    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Sub for Date {
    /// years
    type Output = u16;

    /// Difference between two dates in years (>= 0)

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let max = max(self, rhs);
        let min = min(self, rhs);

        return max.year
            - min.year
            - if max.month > min.month {
                0
            } else if max.month < min.month {
                1
            } else if max.day < min.day {
                1
            } else {
                0
            };
    }
}

impl From<DateTime<Local>> for Date {
    /// Constructor from chrono's library dates

    #[inline]
    fn from(date: DateTime<Local>) -> Self {
        Date {
            day: date.day() as u8,
            month: date.month() as u8,
            year: date.year() as u16,
        }
    }
}

impl From<NaiveDate> for Date {
    /// Constructor from chrono's library dates

    #[inline]
    fn from(date: NaiveDate) -> Self {
        Date {
            day: date.day() as u8,
            month: date.month() as u8,
            year: date.year() as u16,
        }
    }
}

impl From<Date> for String {
    /// Converts date to string

    #[inline]
    fn from(d: Date) -> Self {
        d.to_string()
    }
}

impl Display for Date {
    /// Displays date as string.
    ///
    /// D/M/Y

    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Date {
    /// Constructs date. If date params are wrong,
    /// It will return Err.

    #[inline]
    pub(crate) fn new(new_day: u8, new_month: u8, new_year: u16) -> std::result::Result<Self, ()> {
        let date = Date {
            day: new_day,
            month: new_month,
            year: new_year,
        };

        return if date.correct() { Ok(date) } else { Err(()) };
    }

    /// Checks if date is correct
    /// according to real world

    #[inline]
    pub(crate) fn correct(&self) -> bool {
        return if self.month > 12 || self.month == 0 || self.day == 0 {
            false
        } else {
            const DAYS: [u8; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

            if self.month != 2 {
                unsafe { self.day <= *DAYS.get_unchecked(self.month as usize) }
            } else if self.year % 4 == 0 && self.year % 100 != 0 || self.year % 400 == 0 {
                unsafe { self.day <= *DAYS.get_unchecked(2) + 1 }
            } else {
                unsafe { self.day <= *DAYS.get_unchecked(2) }
            }
        };
    }

    /// Converts date to string

    #[inline]
    pub(crate) fn to_string(&self) -> String {
        format!("{}/{}/{}", self.day, self.month, self.year)
    }
}
