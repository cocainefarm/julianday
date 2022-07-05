//! Julian day is the continuous count of days since the beginning of the Julian Period.
//! This crate implements a method to convert a JulianDay to and from the chrono's NaiveDate.
//!
//! # Example
//!
//! ```
//! use julianday::JulianDay;
//! use chrono::NaiveDate;
//!
//! fn main() {
//!     let naivedate = NaiveDate::from_ymd(2020, 2, 18);
//!     let julianday = JulianDay::from(naivedate);
//!
//!     let date = julianday.to_date();
//! }
//! ```
//!  
//!  Modified Julian Days are translated Julian Days, starting on November 17 1858 at midnight
//!
//!  # Example
//!  ```
//!  use chrono::NaiveDate;
//!  use julianday::{JulianDay, ModifiedJulianDay};
//!  
//!  fn main() {
//!     let date = NaiveDate::from_ymd(1858, 11, 17);
//!     let mjd = ModifiedJulianDay::from(date);
//!     let jd = JulianDay::new(2400001);
//!     assert_eq!(mjd, ModifiedJulianDay::new(0));
//!  }
//!  ```

use chrono::{self, Datelike, NaiveDate};
use std::convert::From;

#[derive(Clone, Debug, PartialEq)]
pub struct JulianDay(i32);

impl From<NaiveDate> for JulianDay {
    /// Get a JulianDay from a NaiveDate
    fn from(date: NaiveDate) -> Self {
        let day = date.day();
        let month = date.month();
        let year = date.year() as u32;

        let a = (14 - month) / 12;
        let y = year + 4800 - a;
        let m = month + 12 * a - 3;

        let jd = day + (153 * m + 2) / 5 + y * 365 + y / 4 - y / 100 + y / 400 - 32045;

        JulianDay(jd as i32)
    }
}

impl JulianDay {
    /// Construct a new JulianDay
    pub fn new(day: i32) -> Self {
        JulianDay(day)
    }

    /// Get the value of JulianDay as i32
    pub fn inner(self) -> i32 {
        let JulianDay(day) = self;
        return day;
    }

    /// Convert a JulianDay to a NaiveDate
    pub fn to_date(self) -> NaiveDate {
        let jd = self.inner();

        let a = jd + 32044;
        let b = (4 * a + 3) / 146097;
        let c = a - (b * 146097) / 4;

        let d = (4 * c + 3) / 1461;
        let e = c - (1461 * d) / 4;
        let m = (5 * e + 2) / 153;

        let day = e - (153 * m + 2) / 5 + 1;
        let month = m + 3 - 12 * (m / 10);
        let year = b * 100 + d - 4800 + m / 10;

        NaiveDate::from_ymd(year as i32, month as u32, day as u32)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModifiedJulianDay (i32);

impl From<NaiveDate> for ModifiedJulianDay {
    /// Get a ModifiedJulianDay from a NaiveDate
    fn from (date: NaiveDate) -> Self {
        let jd : JulianDay = date.into();
        ModifiedJulianDay(jd.inner() - 2400001)
    }
}

impl ModifiedJulianDay {
    pub fn new (day: i32) -> Self {
        ModifiedJulianDay(day)
    }

    pub fn inner (self) -> i32 {
        let ModifiedJulianDay(day) = self;
        return day;
    }

    pub fn to_date (self) -> NaiveDate {
        let jd = JulianDay(self.inner() + 2400001); 
        jd.to_date()
    }
}

#[cfg(test)]
mod tests {
    use crate::{JulianDay, ModifiedJulianDay};
    use chrono::NaiveDate;

    #[test]
    fn julian_to_naivedate() {
        let jd = JulianDay::new(2458898);
        let naivedate = NaiveDate::from_ymd(2020, 2, 18);
        let date = jd.to_date();

        assert_eq!(date, naivedate)
    }

    #[test]
    fn naivedate_to_julian() {
        let jd = JulianDay::new(2458898);
        let naivedate = NaiveDate::from_ymd(2020, 2, 18);
        let date = JulianDay::from(naivedate);

        assert_eq!(date, jd)
    }

    #[test]
    fn mjd_to_naivedate() {
        let mjd = ModifiedJulianDay::new(58897);
        let naivedate = NaiveDate::from_ymd(2020, 2, 18);
        let date = mjd.to_date();
        assert_eq!(date, naivedate);
        let mjd = ModifiedJulianDay::new(57005);
        let naivedate = NaiveDate::from_ymd(2014, 12, 14);
        assert_eq!(mjd.to_date(), naivedate);
    }

    #[test]
    fn naivedate_to_mjd() {
        let date = NaiveDate::from_ymd(2014, 12, 14);
        let mjd = ModifiedJulianDay::from(date);
        assert_eq!(mjd, ModifiedJulianDay(57005));
        let date = NaiveDate::from_ymd(1858, 11, 17);
        let mjd = ModifiedJulianDay::from(date);
        assert_eq!(mjd, ModifiedJulianDay(0));
        let date = NaiveDate::from_ymd(1858, 11, 18);
        let mjd = ModifiedJulianDay::from(date);
        assert_eq!(mjd, ModifiedJulianDay(1));
        let date = NaiveDate::from_ymd(1858, 11, 16);
        let mjd = ModifiedJulianDay::from(date);
        assert_eq!(mjd, ModifiedJulianDay(-1));
    }
}
