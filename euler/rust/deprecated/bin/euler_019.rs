// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

/// Problem:
///
/// You are given the following information, but you may prefer to do
/// some research for yourself.
///
///     * 1 Jan 1900 was a Monday.
///     * Thirty days has September,
///       April, June and November.
///       All the rest have thirty-one,
///       Saving February alone,
///       Which has twenty-eight, rain or shine.
///       And on leap years, twenty-nine.
///    * A leap year occurs on any year evenly divisible by 4, but not on
///      a century unless it is divisible by 400.
///
/// How many Sundays fell on the first of the month during the twentieth
/// century (1 Jan 1901 to 31 Dec 2000)?

type Year = u32;
type Month = u8;
type Day = u8;

fn method1() -> u64 {
    const MONTH_DAYS: [Month; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let check_leap_year = |year: Year| year % 400 == 0 || (year % 4 == 0 && year % 100 != 0);

    let mut year: Year = 1900;
    let mut month: Month = 0; // January
    let mut day: Day = 1; // zero-day
    let mut weekday: u8 = 1; // Monday
    let mut is_leap_year = check_leap_year(year);
    let mut month_days = MONTH_DAYS[month as usize];

    let mut count = 0;

    loop {
        if weekday == 7 && day == 1 && year > 1900 {
            count += 1;
        }

        day += 1;
        weekday %= 7;
        weekday += 1;

        if month_days < day {
            // goto next month.
            day = 1;
            month += 1;
        }

        if month == 12 {
            day = 1;
            month = 0;
            year += 1;
            is_leap_year = check_leap_year(year);
        }

        if day == 1 {
            month_days = MONTH_DAYS[month as usize];
            // Handle leap years.
            if month == 1 && is_leap_year {
                month_days += 1;
            }
        }

        if year == 2001 {
            break;
        }
    }

    count
}

fn main() {
    println!("method1: {}", method1());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 171));
}
