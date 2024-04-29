// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub fn next_permutation1(nums: &mut Vec<i32>) {
    todo!()
}

pub type SolutionFn = fn(&mut Vec<i32>);

fn check_solution(func: SolutionFn) {
    todo!()
}

fn main() {
    check_solution(next_permutation1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, next_permutation1};

    #[test]
    fn test_next_permutation1() {
        check_solution(next_permutation1);
    }
}
