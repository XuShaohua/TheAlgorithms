// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Bruteforce
pub fn contains_pattern1(arr: Vec<i32>, m: i32, k: i32) -> bool {
    debug_assert!(arr.len() >= 2);
    debug_assert!((1..=100).contains(&m));
    debug_assert!((2..=100).contains(&k));

    let len: usize = arr.len();
    let m: usize = m as usize;

    // 遍历数组
    for i in 0..(len - m) {
        let slice_i: &[i32] = &arr[i..(i + m)];
        // 每次遍历都要重置 count
        let mut count: i32 = 1;

        for j in ((i + m)..len).step_by(m) {
            if j + m > len {
                break;
            }
            let slice_j: &[i32] = &arr[j..(j + m)];
            //println!("slice i: {slice_i:?}, slice j: {slice_j:?}");
            if slice_i == slice_j {
                count += 1;
                if count == k {
                    return true;
                }
            } else {
                // 没有连续, 放弃并退出内层循环.
                break;
            }
        }
    }

    false
}

pub type SolutionFn = fn(Vec<i32>, i32, i32) -> bool;

fn check_solution(func: SolutionFn) {
    let arr = vec![1, 2, 4, 4, 4, 4];
    assert!(func(arr, 1, 3));

    let arr = vec![1, 2, 1, 2, 1, 1, 1, 3];
    assert!(func(arr, 2, 2));

    let arr = vec![1, 2, 1, 2, 1, 3];
    assert!(!func(arr, 2, 3));

    let arr = vec![2, 2];
    assert!(func(arr, 1, 2));

    let arr = vec![2, 2, 1, 2, 2, 1, 1, 1, 2, 1];
    assert!(!func(arr, 2, 2));
}

fn main() {
    check_solution(contains_pattern1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, contains_pattern1};

    #[test]
    fn test_contains_pattern1() {
        check_solution(contains_pattern1);
    }
}
