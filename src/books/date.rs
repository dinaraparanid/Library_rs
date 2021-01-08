use chrono::{DateTime, Datelike, Utc};
use std::cmp::Ordering;

/// Date structure, which contains day, month and year.
/// It's a copyable type like i32 (no move).
/// You can clone, debug and compare as == / !=

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Date {
    pub(crate) day: u8,
    pub(crate) month: u8,
    pub(crate) year: u16,
}

/// Dates can be compared as >, <, >=, <=
/// (as it works in real world)

impl PartialOrd for Date {
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

/// Dates can be compared as >, <, >=, <=
/// (as it works in real world)

impl Ord for Date {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// Constructor from chrono's library dates

impl From<DateTime<Utc>> for Date {
    fn from(date: DateTime<Utc>) -> Self {
        Date::new(date.day() as u8, date.month() as u8, date.year() as u16).unwrap()
    }
}

impl Date {
    /// Constructs date. If date params are wrong,
    /// It will return Err.

    #[inline]
    pub fn new(new_day: u8, new_month: u8, new_year: u16) -> std::result::Result<Self, ()> {
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
    pub fn correct(&self) -> bool {
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
}
