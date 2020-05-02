use chrono::{self, Datelike, NaiveDate};
use std::convert::From;

#[derive(Clone, Debug, PartialEq)]
pub struct JulianDay(u32);

impl From<NaiveDate> for JulianDay {
    fn from(date: NaiveDate) -> Self {
        let day = date.day();
        let month = date.month();
        let year = date.year() as u32;

        let a = (14 - month) / 12;
        let y = year + 4800 - a;
        let m = month + 12 * a - 3;

        let jd = day + (153 * m + 2) / 5 + y * 365 + y / 4 - y / 100 + y / 400 - 32045;

        JulianDay(jd)
    }
}

impl JulianDay {
    pub fn inner(self) -> u32 {
        let JulianDay(day) = self;
        return day;
    }

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

        NaiveDate::from_ymd(year as i32, month, day)
    }
}

#[cfg(test)]
mod tests {
    use crate::JulianDay;
    use chrono::NaiveDate;

    #[test]
    fn julian_to_naivedate() {
        let jd = JulianDay(2458898);
        let naivedate = NaiveDate::from_ymd(2020, 2, 18);
        let date = jd.to_date();

        assert_eq!(date, naivedate)
    }

    #[test]
    fn naivedate_to_julian() {
        let jd = JulianDay(2458898);
        let naivedate = NaiveDate::from_ymd(2020, 2, 18);
        let date = JulianDay::from(naivedate);

        assert_eq!(date, jd)
    }
}
