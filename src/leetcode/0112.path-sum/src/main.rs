// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[must_use]
    #[inline]
    pub const fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    #[must_use]
    pub fn from_slice(slice: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        fn inner(slice: &[i32], index: usize) -> Option<Rc<RefCell<TreeNode>>> {
            if index >= slice.len() {
                return None;
            }
            let val: i32 = slice[index];
            if val == i32::MAX {
                return None;
            }

            let left = inner(slice, index * 2 + 1);
            let right = inner(slice, index * 2 + 2);
            Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
        }

        inner(slice, 0)
    }
}

// Recursive
// 前序遍历所有节点
pub fn has_path_sum1(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    fn inner(root: Option<Rc<RefCell<TreeNode>>>, mut sum: i32) -> bool {
        if let Some(root_rc) = root {
            // 访问当前节点
            let val: i32 = root_rc.borrow().val;
            sum += val;
            // 递归访问左子树
            let left = root_rc.borrow_mut().left.take();
            // 递归访问右子树
            let right = root_rc.borrow_mut().right.take();
            match (left, right) {
                (Some(left_rc), Some(right_rc)) => {
                    // 左右子树都不为空
                    inner(Some(left_rc), sum) || inner(Some(right_rc), sum)
                }
                (Some(left_rc), None) => {
                    // 左子树不为空
                    inner(Some(left_rc), sum)
                }
                (None, Some(right_rc)) => {
                    // 右子树不为空
                    inner(Some(right_rc), sum)
                }
                (None, None) => {
                    // 左右子树都为空, 当前节点就是叶节点
                    sum == 0
                }
            }
        } else {
            // 当前节点为空, 没有根节点到叶节点之间的路径
            false
        }
    }

    inner(root, -target_sum)
}

// Stack
// 使用本地的栈替换函数调用栈
pub fn has_path_sum2(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    let mut stack = vec![(root, -target_sum)];

    // 一直循环, 直到栈为空
    while let Some((node, sum)) = stack.pop() {
        if let Some(node_rc) = node {
            // 访问当前节点
            let val: i32 = node_rc.borrow().val;
            let new_sum: i32 = sum + val;

            // 如果左子树不为空, 就入栈
            // 如果右子树不为空, 就入栈
            let left = node_rc.borrow_mut().left.take();
            let right = node_rc.borrow_mut().right.take();
            match (left, right) {
                (Some(left_rc), Some(right_rc)) => {
                    // 左右子树都不为空
                    stack.push((Some(right_rc), new_sum));
                    stack.push((Some(left_rc), new_sum));
                }
                (Some(left_rc), None) => {
                    // 只有左子树
                    stack.push((Some(left_rc), new_sum));
                }
                (None, Some(right_rc)) => {
                    // 只有右子树
                    stack.push((Some(right_rc), new_sum));
                }
                (None, None) => {
                    // 左右子树都为空, 当前节点是叶节点
                    if new_sum == 0 {
                        return true;
                    }
                }
            }
        } else {
            // 当前节点为空
            return false;
        }
    }

    false
}

pub type SolutionFn = fn(Option<Rc<RefCell<TreeNode>>>, i32) -> bool;

fn check_solution(func: SolutionFn) {
    let root = TreeNode::from_slice(&[
        5,
        4,
        8,
        11,
        i32::MAX,
        13,
        4,
        7,
        2,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        1,
    ]);
    assert!(func(root, 22));

    let root = TreeNode::from_slice(&[1, 2, 3]);
    assert!(!func(root, 5));

    let root = TreeNode::from_slice(&[]);
    assert!(!func(root, 0));

    let root = TreeNode::from_slice(&[1, 2]);
    assert!(!func(root, 1));
}

fn main() {
    check_solution(has_path_sum1);
    check_solution(has_path_sum2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, has_path_sum1};

    #[test]
    fn test_has_path_sum1() {
        check_solution(has_path_sum1);
    }
}
