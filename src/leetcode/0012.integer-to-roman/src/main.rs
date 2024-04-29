// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[must_use]
#[inline]
pub const fn get_more_roman_symbols(num: i32) -> &'static str {
    match num {
        3000 => "MMM",
        2000 => "MM",
        1000 => "M",
        900 => "CM",
        800 => "DCCC",
        700 => "DCC",
        600 => "DC",
        500 => "D",
        400 => "CD",
        300 => "CCC",
        200 => "CC",
        100 => "C",
        90 => "XC",
        80 => "LXXX",
        70 => "LXX",
        60 => "LX",
        50 => "L",
        40 => "XL",
        30 => "XXX",
        20 => "XX",
        10 => "X",
        9 => "IX",
        8 => "VIII",
        7 => "VII",
        6 => "VI",
        5 => "V",
        4 => "IV",
        3 => "III",
        2 => "II",
        1 => "I",
        _ => "-",
    }
}

// 模式匹配
pub fn int_to_roman1(num: i32) -> String {
    assert!((0..=3999).contains(&num));

    let mut num = num;
    let mut quot;
    // 最大支持1000级的
    let mut factor = 1000;
    let mut out = String::new();

    while num > 0 || factor > 0 {
        // 计算商
        quot = num / factor;
        // 构造这个位的数值.
        let val = quot * factor;

        if val > 0 {
            // 根据数值, 拿到对应的字符串
            out += get_more_roman_symbols(val);
        }

        // 计算余数, 准备下一次循环使用
        num %= factor;
        factor /= 10;
    }

    out
}

#[must_use]
#[inline]
pub const fn get_roman_symbol(num: i32) -> char {
    match num {
        1000 => 'M',
        500 => 'D',
        100 => 'C',
        50 => 'L',
        10 => 'X',
        5 => 'V',
        1 => 'I',
        _ => '-',
    }
}

#[must_use]
pub fn get_roman_symbols_manual(quot: i32, factor: i32) -> String {
    match quot {
        0 => String::new(),
        val @ 1..=3 => {
            let one = get_roman_symbol(factor);
            let mut s = String::new();
            // [one]
            // [one, one]
            // [one, one, one]
            for _i in 0..val {
                s.push(one);
            }
            s
        }
        4 => {
            let five = get_roman_symbol(5 * factor);
            let one = get_roman_symbol(factor);
            let mut s = String::new();
            // [one, five]
            s.push(one);
            s.push(five);
            s
        }
        5 => get_roman_symbol(5 * factor).to_string(),
        val @ 6..=8 => {
            let five = get_roman_symbol(5 * factor);
            let one = get_roman_symbol(factor);
            let mut s = String::new();
            // [five, one]
            // [five, one, one]
            // [five, one, one, one]
            s.push(five);
            for _i in 0..(val - 5) {
                s.push(one);
            }
            s
        }
        9 => {
            let ten = get_roman_symbol(10 * factor);
            let one = get_roman_symbol(factor);
            let mut s = String::new();
            // [one, ten]
            s.push(one);
            s.push(ten);
            s
        }
        _ => unimplemented!(),
    }
}

// 部分模式匹配
// 剩下的手动计算, 性能较差
pub fn int_to_roman2(num: i32) -> String {
    assert!((0..=3999).contains(&num));

    let mut num = num;
    let mut quot;
    // 最大支持1000级的
    let mut factor = 1000;
    let mut out = String::new();

    while num > 0 || factor > 0 {
        // 计算商
        quot = num / factor;

        if quot > 0 {
            // 构造这个位的数值对应的罗马数字.
            out += &get_roman_symbols_manual(quot, factor);
        }

        // 计算余数, 准备下一次循环使用
        num %= factor;
        factor /= 10;
    }

    out
}

pub type SolutionFn = fn(i32) -> String;

fn check_solution(func: SolutionFn) {
    const RECORDS: &[(i32, &str)] = &[(3, "III"), (58, "LVIII"), (1994, "MCMXCIV"), (9, "IX")];
    for record in RECORDS {
        assert_eq!(func(record.0).as_str(), record.1);
    }
}

fn main() {
    check_solution(int_to_roman1);
    check_solution(int_to_roman2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, int_to_roman1, int_to_roman2};

    #[test]
    fn test_solution1() {
        check_solution(int_to_roman1);
    }

    #[test]
    fn test_solution2() {
        check_solution(int_to_roman2);
    }
}
