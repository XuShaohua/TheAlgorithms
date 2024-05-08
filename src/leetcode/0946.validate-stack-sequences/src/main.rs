// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Stack
pub fn validate_stack_sequences1(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
    assert_eq!(pushed.len(), popped.len());
    assert!(!pushed.is_empty());

    let mut stack: Vec<i32> = Vec::new();
    let mut pop_index: usize = 0;

    // 先遍历 pushed 数组.
    for num in &pushed {
        // 如果与 popped 当前元素相等, 则不必入栈, 直接将 popped 的索引移到下一位.
        if Some(num) == popped.get(pop_index) {
            pop_index += 1;

            // 贪心, 检查栈中剩下的元素, 能不能被出栈.
            while pop_index < popped.len() && !stack.is_empty() {
                if popped.get(pop_index) == stack.last() {
                    // 相等则出栈, 同时移动索引.
                    pop_index += 1;
                    let _ = stack.pop();
                } else {
                    // 不相等, 则立即中止循环.
                    break;
                }
            }
        } else {
            // 如果不相等, 则入栈.
            stack.push(*num);
        }
    }

    // 遍历 popped 数组中剩下的元素, 看它们与出栈后的元素是否相等.
    while pop_index < popped.len() {
        // 比较popped 中的元素与栈顶元素是否相等
        if popped.get(pop_index) != stack.pop().as_ref() {
            return false;
        }
        pop_index += 1;
    }

    stack.is_empty()
}

// Stack
// 优化栈的操作, 遍历 popped 数组.
pub fn validate_stack_sequences2(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
    assert_eq!(pushed.len(), popped.len());
    assert!(!pushed.is_empty());

    let mut stack: Vec<i32> = Vec::with_capacity(pushed.len());
    let mut push_iter = pushed.into_iter();

    // 先遍历 popped 数组.
    for pop_num in &popped {
        // 如果当前的栈顶与当前的 pop_num 不相等, 则有必要再入栈一些新的整数.
        while Some(pop_num) != stack.last() {
            if let Some(push_num) = push_iter.next() {
                // 入栈
                stack.push(push_num);
            } else {
                // 如果所有元素都已经入栈, 仍然找不到相等的元素可以出栈, 那就说明 popped
                // 不是一个有效的出栈序列.
                return false;
            }
        }
        // pop_num 与栈顶相等, 可以出栈了.
        let _stack_pop = stack.pop();
    }
    true
}

pub type SolutionFn = fn(Vec<i32>, Vec<i32>) -> bool;

fn check_solution(func: SolutionFn) {
    let pushed = vec![1, 2, 3, 4, 5];
    let popped = vec![4, 5, 3, 2, 1];
    assert!(func(pushed, popped));

    let pushed = vec![1, 2, 3, 4, 5];
    let popped = vec![4, 3, 5, 1, 2];
    assert!(!func(pushed, popped));

    let pushed = vec![2, 1, 0];
    let popped = vec![1, 2, 0];
    assert!(func(pushed, popped));
}

fn main() {
    check_solution(validate_stack_sequences1);
    check_solution(validate_stack_sequences2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, validate_stack_sequences1, validate_stack_sequences2};

    #[test]
    fn test_validate_stack_sequences1() {
        check_solution(validate_stack_sequences1);
    }

    #[test]
    fn test_validate_stack_sequences2() {
        check_solution(validate_stack_sequences2);
    }
}
