// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::VecDeque;

// Stack
pub fn count_students1(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
    assert_eq!(students.len(), sandwiches.len());
    assert!(!students.is_empty());

    let mut queue: VecDeque<i32> = VecDeque::new();
    let mut sandwiches = sandwiches;
    sandwiches.reverse();

    // 遍历所有的学生.
    for student in students {
        if Some(&student) == sandwiches.last() {
            // 找到了喜欢吃的, 直接离开
            let _ = sandwiches.pop();
        } else {
            // 入队列.
            queue.push_back(student);
        }
    }

    let mut sandwich_count = sandwiches.len();
    let mut index = 0;
    while let Some(student) = queue.pop_front() {
        if Some(&student) == sandwiches.last() {
            // 找到了合适的, 走吧.
            let _ = sandwiches.pop();
        } else {
            // 继续入队列.
            queue.push_back(student);
        }
        index += 1;
        // 循环一圈之后, 三明治没有减少.
        if index > sandwich_count {
            if sandwich_count == sandwiches.len() {
                break;
            } else {
                // 进入下一次循环
                index = 0;
                sandwich_count = sandwiches.len();
            }
        }
    }
    queue.len() as i32
}

pub type SolutionFn = fn(Vec<i32>, Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let students = vec![1, 1, 0, 0];
    let sandwiches = vec![0, 1, 0, 1];
    assert_eq!(func(students, sandwiches), 0);

    let students = vec![1, 1, 1, 0, 0, 1];
    let sandwiches = vec![1, 0, 0, 0, 1, 1];
    assert_eq!(func(students, sandwiches), 3);
}

fn main() {
    check_solution(count_students1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, count_students1};

    #[test]
    fn test_count_students1() {
        check_solution(count_students1);
    }
}
