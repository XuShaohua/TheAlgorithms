// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::BTreeMap;

// Hashmap
// 使用字典计数
pub fn maximum_population1(logs: Vec<Vec<i32>>) -> i32 {
    debug_assert!(!logs.is_empty());
    // 因为要得到最早的年份, 我们使用 BTreeMap 来保证年份有序.
    let mut map = BTreeMap::new();

    for log in logs {
        for year in log[0]..log[1] {
            *map.entry(year).or_default() += 1;
        }
    }

    let mut max_count = 0;
    let mut max_year = 0;
    for (year, count) in map {
        if count > max_count {
            max_count = count;
            max_year = year;
        }
    }
    max_year
}

// 计数
// 使用数组代替字典
pub fn maximum_population2(logs: Vec<Vec<i32>>) -> i32 {
    const MAX_YEARS: usize = 100;
    debug_assert!(!logs.is_empty());
    let start_year: usize = 1950;
    let mut timeline = [0; MAX_YEARS];

    for log in logs {
        for year in log[0]..log[1] {
            timeline[year as usize - start_year] += 1;
        }
    }

    let mut max_count = 0;
    let mut max_year = 0;
    for (year, &count) in timeline.iter().enumerate() {
        if count > max_count {
            max_count = count;
            max_year = year;
        }
    }
    (max_year + start_year) as i32
}

// Prefix Sum
pub fn maximum_population3(logs: Vec<Vec<i32>>) -> i32 {
    let start_year = 1950;
    let end_year = 2050;
    let no_years = end_year - start_year + 1;

    // 构造数组, 用于记录每年增减的人数,
    // 为构造前缀和数组做准备
    let mut alive = vec![0; no_years as usize];
    for log in logs {
        let start_year_index = (log[0] - start_year) as usize;
        let end_year_index = (log[1] - start_year) as usize;
        alive[start_year_index] += 1;
        alive[end_year_index] -= 1;
    }

    // 构造前缀和数组
    let mut prefix_sum = vec![0; alive.len()];
    prefix_sum[0] = alive[0];
    for i in 1..alive.len() {
        prefix_sum[i] = prefix_sum[i - 1] + alive[i];
    }

    // 遍历前缀和数组, 找到最多的人所在的年份
    let mut max_count = 0;
    let mut max_year = 0;
    for (index, count) in prefix_sum.into_iter().enumerate() {
        if count > max_count {
            max_count = count;
            max_year = index as i32 + start_year;
        }
    }
    max_year
}

pub type SolutionFn = fn(Vec<Vec<i32>>) -> i32;

fn check_solution(func: SolutionFn) {
    let logs = vec![vec![1993, 1999], vec![2000, 2010]];
    assert_eq!(func(logs), 1993);

    let logs = vec![vec![1950, 1961], vec![1960, 1971], vec![1970, 1981]];
    assert_eq!(func(logs), 1960);
}

fn main() {
    check_solution(maximum_population1);
    check_solution(maximum_population2);
    check_solution(maximum_population3);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, maximum_population1, maximum_population2, maximum_population3};

    #[test]
    fn test_maximum_population1() {
        check_solution(maximum_population1);
    }

    #[test]
    fn test_maximum_population2() {
        check_solution(maximum_population2);
    }

    #[test]
    fn test_maximum_population3() {
        check_solution(maximum_population3);
    }
}
