// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Bruteforce
pub fn count_good_triplets1(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
    let len: usize = arr.len();
    let mut count: i32 = 0;

    // 三层嵌套循环
    for i in 0..(len - 2) {
        for j in (i + 1)..(len - 1) {
            for k in (j + 1)..len {
                if (arr[i] - arr[j]).abs() <= a
                    && (arr[j] - arr[k]).abs() <= b
                    && (arr[i] - arr[k]).abs() <= c
                {
                    count += 1;
                }
            }
        }
    }
    count
}

// 优化循环
pub fn count_good_triplets2(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
    let len: usize = arr.len();
    let mut count: i32 = 0;

    // 三层嵌套循环
    for i in 0..(len - 2) {
        for j in (i + 1)..(len - 1) {
            if (arr[i] - arr[j]).abs() > a {
                continue;
            }

            for k in (j + 1)..len {
                if (arr[j] - arr[k]).abs() <= b && (arr[i] - arr[k]).abs() <= c {
                    count += 1;
                }
            }
        }
    }
    count
}

pub type SolutionFn = fn(Vec<i32>, i32, i32, i32) -> i32;

fn check_solution(func: SolutionFn) {
    let arr = vec![3, 0, 1, 1, 9, 7];
    assert_eq!(func(arr, 7, 2, 3), 4);

    let arr = vec![1, 1, 2, 2, 3];
    assert_eq!(func(arr, 0, 0, 1), 0);
}

fn main() {
    check_solution(count_good_triplets1);
    check_solution(count_good_triplets2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, count_good_triplets1, count_good_triplets2};

    #[test]
    fn test_count_good_triplets1() {
        check_solution(count_good_triplets1);
    }

    #[test]
    fn test_count_good_triplets2() {
        check_solution(count_good_triplets2);
    }
}
