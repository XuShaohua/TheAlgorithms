// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub fn solution1() {
    todo!();
}

pub type SolutionFn = fn();

fn check_solution(func: SolutionFn) {
    todo!();
}

fn main() {
    check_solution(solution1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, solution1};

    #[test]
    fn test_solution1() {
        check_solution(solution1);
    }
}
