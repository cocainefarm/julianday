[![mirror](https://img.shields.io/badge/mirror-gitlab-orange)](https://gitlab.com/cocainefarm/julianday)
[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/cocainefarm/julianday)
[![crates.io](https://img.shields.io/crates/v/julianday.svg)](https://crates.io/crates/julianday)
[![Rust](https://github.com/cocainefarm/julianday/actions/workflows/rust.yml/badge.svg)](https://github.com/cocainefarm/julianday/actions/workflows/rust.yml)
[![docs.rs](https://docs.rs/julianday/badge.svg)](https://docs.rs/julianday)

Julian day is the continuous count of days since the beginning of the Julian Period.
This crate implements a method to convert a JulianDay to and from the chrono's NaiveDate.

## Julian Day (`JD`)

```rust
use chrono::NaiveDate;
use julianday::JulianDay;

fn main() {
    let naivedate = NaiveDate::from_ymd(2020, 2, 18);
    let julianday = JulianDay::from(naivedate);
    let date = julianday.to_date();
}
```

## Modified Julian Day (`MJD`)

```rust
use chrono::NaiveDate;
use julianday::ModifiedJulianDay;

fn main() {
    let naivedate = NaiveDate::from_ymd(2020, 2, 18);
    let mjd = ModifiedJulianDay::from(naivedate);
    let date = mjd.to_date();
}
```
