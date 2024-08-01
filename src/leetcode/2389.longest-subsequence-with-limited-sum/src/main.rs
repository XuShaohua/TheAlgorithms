// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub fn answer_queries1(_nums: Vec<i32>, _queries: Vec<i32>) -> Vec<i32> {
    todo!();
}

pub type SolutionFn = fn(Vec<i32>, Vec<i32>) -> Vec<i32>;

fn check_solution(_func: SolutionFn) {
    todo!();
}

fn main() {
    check_solution(answer_queries1);
}

#[cfg(test)]
mod tests {
    use super::{answer_queries1, check_solution};

    #[test]
    fn test_answer_queries1() {
        check_solution(answer_queries1);
    }
}
