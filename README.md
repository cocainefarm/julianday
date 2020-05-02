Julian day is the continuous count of days since the beginning of the Julian Period.
This crate implements a method to convert a JulianDay to and from the chrono's NaiveDate.

# Example

```
use julianday::JulianDay;
use chrono::NaiveDate;

fn main() {
    let naivedate = NaiveDate::from_ymd(2020, 2, 18);
    let julianday = JulianDay::from(naivedate);

    let date = julianday.to_date();
}
```
