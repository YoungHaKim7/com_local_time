//!
//! SystemTime { tv_sec: 1717411455, tv_nsec: 637180000 }
//!
//! # 컴퓨터 시간만으로 우리나라 시간 및 다른 나라 시간 구하기
//!
//! # Example
//! ```rs
//! let sys_time = SystemTime::now();
//! println!("{}", sys_time);
//! ```
//!
//! # Result
//! ```bash
//! SystemTime { tv_sec: 1717411455, tv_nsec: 637180000 }
//! ```
//!
//! <hr>
//!
//! # Example
//! ```rs
//! let sys_local_time = com_local_time::localtime();
//! ```
//! <hr>
//!
//! # Result
//!```bash
//!2024년 6월 3일 금요일, 19: 41: 19
//!```
//! <hr>
//!
//! ```rs
//! use std::{ io::Write,
//!     thread::sleep,
//!     time::{SystemTime, UNIX_EPOCH},
//! };
//!
//! const SEC_IN_MIN: u64 = 60;
//! const SEC_IN_HOUR: u64 = 60 * SEC_IN_MIN;
//! const SEC_IN_DAY: u64 = 24 * SEC_IN_HOUR;
//! const SEC_IN_YEAR: u64 = 365 * SEC_IN_DAY;
//! const SEC_IN_LEAP_YEAR: u64 = 366 * SEC_IN_DAY;
//!
//! fn is_leap_year(year: u64) -> bool {
//!     (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
//! }

//! fn main() {
//!    loop {
//!        let total_sec: u64 = SystemTime::now()
//!            .duration_since(UNIX_EPOCH)
//!            .expect("Time went backwards")
//!            .as_secs();
//!
//!        let mut remaining_sec = total_sec;
//!
//!        // Calculate the current year
//!        let mut year = 1970;
//!        while remaining_sec >= SEC_IN_YEAR {
//!            let year_seconds = if is_leap_year(year) {
//!                SEC_IN_LEAP_YEAR
//!            } else {
//!                SEC_IN_YEAR
//!            };
//!            if remaining_sec < year_seconds {
//!                break;
//!            }
//!            remaining_sec -= year_seconds;
//!            year += 1;
//!        }
//!
//!        // Calculate the current month and day
//!        const DAYS_IN_MONTH: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
//!        const DAYS_IN_LEAP_MONTH: [u64; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
//!        const DAYS_OF_WEEK: [&str; 7] = ["월", "화", "수", "목", "금", "토", "일"];
//!
//!        let days_in_month = if is_leap_year(year) {
//!            &DAYS_IN_LEAP_MONTH
//!        } else {
//!            &DAYS_IN_MONTH
//!        };
//!
//!        let mut month = 0;
//!        let mut days = remaining_sec / SEC_IN_DAY;
//!        remaining_sec %= SEC_IN_DAY;
//!
//!        while month < 12 && days >= days_in_month[month] {
//!            days -= days_in_month[month];
//!            month += 1;
//!        }
//!
//!        // Calculate the current hour, minute, and second
//!        let hours = remaining_sec / SEC_IN_HOUR;
//!        remaining_sec %= SEC_IN_HOUR;
//!        let minutes = remaining_sec / SEC_IN_MIN;
//!        let seconds = remaining_sec % SEC_IN_MIN;
//!
//!        let day_of_week_cal = (days + 2) % 7;
//!        let week_cal = DAYS_OF_WEEK[day_of_week_cal as usize];
//!
//!        print!(
//!            "\r{}년 {}월 {}일 {}요일, {}: {}: {}",
//!            year,
//!            month + 1,
//!            days + 1,
//!            week_cal,
//!            hours + 9,
//!            minutes,
//!            seconds,
//!        );
//!        std::io::stdout().flush().unwrap();
//!        sleep(std::time::Duration::from_secs(1));
//!    }
//!}
//!
//!
//!```
//!
//!# Result
//!
//!```bash
//!2024년 6월 3일 금요일, 19: 41: 19
//!```

use std::{
    io::Write,
    thread::sleep,
    time::{SystemTime, UNIX_EPOCH},
};

const SEC_IN_MIN: u64 = 60;
const SEC_IN_HOUR: u64 = 60 * SEC_IN_MIN;
const SEC_IN_DAY: u64 = 24 * SEC_IN_HOUR;
const SEC_IN_YEAR: u64 = 365 * SEC_IN_DAY;
const SEC_IN_LEAP_YEAR: u64 = 366 * SEC_IN_DAY;

fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

pub fn localtime() {
    let total_sec: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let mut remaining_sec = total_sec;

    // Calculate the current year
    let mut year = 1970;
    while remaining_sec >= SEC_IN_YEAR {
        let year_seconds = if is_leap_year(year) {
            SEC_IN_LEAP_YEAR
        } else {
            SEC_IN_YEAR
        };
        if remaining_sec < year_seconds {
            break;
        }
        remaining_sec -= year_seconds;
        year += 1;
    }

    // Calculate the current month and day
    const DAYS_IN_MONTH: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    const DAYS_IN_LEAP_MONTH: [u64; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    const DAYS_OF_WEEK: [&str; 7] = ["월", "화", "수", "목", "금", "토", "일"];

    let days_in_month = if is_leap_year(year) {
        &DAYS_IN_LEAP_MONTH
    } else {
        &DAYS_IN_MONTH
    };

    let mut month = 0;
    let mut days = remaining_sec / SEC_IN_DAY;
    remaining_sec %= SEC_IN_DAY;

    while month < 12 && days >= days_in_month[month] {
        days -= days_in_month[month];
        month += 1;
    }

    // Calculate the current hour, minute, and second
    let hours = remaining_sec / SEC_IN_HOUR;
    remaining_sec %= SEC_IN_HOUR;
    let minutes = remaining_sec / SEC_IN_MIN;
    let seconds = remaining_sec % SEC_IN_MIN;

    let day_of_week_cal = (days + 2) % 7;
    let week_cal = DAYS_OF_WEEK[day_of_week_cal as usize];

    print!(
        "\r{}년 {}월 {}일 {}요일, {}: {}: {}",
        year,
        month + 1,
        days + 1,
        week_cal,
        hours + 9,
        minutes,
        seconds,
    );
    std::io::stdout().flush().unwrap();
    sleep(std::time::Duration::from_secs(1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_time() {
        let result = localtime();
        println!("{:?}", result);
    }

    #[test]
    fn system_time_now() {
        let result = SystemTime::now();
        println!("{:?}", result);
    }
}
