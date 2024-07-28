// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Prefix sum
pub fn largest_altitude1(gain: Vec<i32>) -> i32 {
    debug_assert!(!gain.is_empty());
    let mut last_altitude = 0;
    let mut highest = 0;
    for altitude in gain {
        last_altitude += altitude;
        highest = highest.max(last_altitude);
    }
    highest
}

pub type SolutionFn = fn(Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let gain = vec![-5, 1, 5, 0, -7];
    assert_eq!(func(gain), 1);

    let gain = vec![-4, -3, -2, -1, 4, 3, 2];
    assert_eq!(func(gain), 0);
}

fn main() {
    check_solution(largest_altitude1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, largest_altitude1};

    #[test]
    fn test_largest_altitude1() {
        check_solution(largest_altitude1);
    }
}
