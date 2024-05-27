// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

fn is_ugly_number(mut n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    while n != 0 && n % 2 == 0 {
        n /= 2;
    }
    while n != 0 && n % 3 == 0 {
        n /= 3;
    }
    while n != 0 && n % 5 == 0 {
        n /= 5;
    }
    n == 1
}

// Bruteforce
// Timedout
pub fn nth_ugly_number1(n: i32) -> i32 {
    debug_assert!((1..=1690).contains(&n));
    let mut ugly_numbers: Vec<i32> = Vec::with_capacity(1691);
    ugly_numbers.push(1);
    let n: usize = n as usize;
    let mut last2: i32 = 0;
    let mut last3: i32 = 0;
    let mut last5: i32 = 0;
    let mut last_num: i32 = 0;
    while ugly_numbers.len() < n {
        // 找到这一组中的最小值.
        let mut tuples = [(2, last2 + 2), (3, last3 + 3), (5, last5 + 5)];
        tuples.sort_by_key(|pair| pair.1);
        // 更新索引.
        match tuples[0].0 {
            2 => last2 += 2,
            3 => last3 += 3,
            5 => last5 += 5,
            _ => (),
        }
        // 如果是 ugly number, 就加到数组中.
        let curr = tuples[0].1;
        if last_num == curr {
            continue;
        }
        if is_ugly_number(curr) {
            ugly_numbers.push(curr);
            last_num = curr;
        }
    }

    ugly_numbers[n - 1]
}

pub fn nth_ugly_number2(n: i32) -> i32 {
    debug_assert!((1..=1690).contains(&n));

    const MAX_N: usize = 1691;

    #[must_use]
    #[inline(always)]
    const fn min(a: i32, b: i32) -> i32 {
        if a < b {
            a
        } else {
            b
        }
    }

    const fn generate_ugly_list() -> [i32; MAX_N] {
        let mut ugly_numbers = [0; MAX_N];
        ugly_numbers[0] = 1;
        let mut index2: usize = 0;
        let mut index3: usize = 0;
        let mut index5: usize = 0;

        let mut i: usize = 1;
        while i < MAX_N {
            let next2: i32 = ugly_numbers[index2] * 2;
            let next3: i32 = ugly_numbers[index3] * 3;
            let next5: i32 = ugly_numbers[index5] * 5;
            //let next_ugly: i32 = next2.min(next3.min(next5));
            let next_ugly: i32 = min(next2, min(next3, next5));
            if next_ugly == next2 {
                index2 += 1;
            }
            if next_ugly == next3 {
                index3 += 1;
            }
            if next_ugly == next5 {
                index5 += 1;
            }
            ugly_numbers[i] = next_ugly;
            i += 1;
        }
        ugly_numbers
    }

    const UGLY_NUMBERS: [i32; MAX_N] = generate_ugly_list();
    //println!("ugly numbers: {:?}", &UGLY_NUMBERS[..10]);
    UGLY_NUMBERS[n as usize - 1]
}

pub type SolutionFn = fn(i32) -> i32;

fn check_solution(func: SolutionFn, is_slow: bool) {
    assert_eq!(func(1), 1);
    assert_eq!(func(10), 12);
    assert_eq!(func(20), 36);
    assert_eq!(func(100), 1536);
    assert_eq!(func(500), 937_500);
    if !is_slow {
        assert_eq!(func(1000), 51_200_000);
        assert_eq!(func(1352), 402_653_184);
        assert_eq!(func(1690), 2_123_366_400);
    }
}

fn main() {
    //check_solution(nth_ugly_number1, true);
    check_solution(nth_ugly_number2, false);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, nth_ugly_number1, nth_ugly_number2};

    #[test]
    fn test_nth_ugly_number1() {
        check_solution(nth_ugly_number1, true);
    }

    #[test]
    fn test_nth_ugly_number2() {
        check_solution(nth_ugly_number2, false);
    }
}
