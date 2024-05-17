// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::cell::RefCell;
use std::collections::VecDeque;
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

// 层序遍历
pub fn is_symmetric1(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut queue = VecDeque::new();
    queue.push_back(root);

    while !queue.is_empty() {
        let len = queue.len();
        let mut values = Vec::with_capacity(len);

        for _i in 0..len {
            let node = queue.pop_front();
            if let Some(Some(node_rc)) = node {
                // 访问当前节点
                let val: i32 = node_rc.borrow().val;
                values.push(val);

                // 左子节点入队列
                queue.push_back(node_rc.borrow_mut().left.take());
                // 右子节点入队列
                queue.push_back(node_rc.borrow_mut().right.take());
            } else {
                // 空节点
                values.push(i32::MAX);
            }
        }

        // 镜像数组, 比较是否相等
        for (num1, num2) in values.iter().zip(values.iter().rev()) {
            if num1 != num2 {
                return false;
            }
        }
    }
    true
}

// Recursive
pub fn is_symmetric2(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn inner(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (left, right) {
            // 左右子树都不为空
            (Some(left_rc), Some(right_rc)) => {
                let left_val: i32 = left_rc.borrow().val;
                let right_val: i32 = right_rc.borrow().val;
                // 左右两个子树的节点值不相等.
                if left_val != right_val {
                    return false;
                }

                // 递归判断子树
                // left->left == right->right && left->right == right->left
                inner(
                    left_rc.borrow_mut().left.take(),
                    right_rc.borrow_mut().right.take(),
                ) && inner(
                    left_rc.borrow_mut().right.take(),
                    right_rc.borrow_mut().left.take(),
                )
            }
            (Some(_), None) | (None, Some(_)) => {
                // 左右子树只有一个节点为空
                false
            }
            (None, None) => {
                // 左右子树都为空
                true
            }
        }
    }

    if let Some(root_rc) = root {
        let left = root_rc.borrow_mut().left.take();
        let right = root_rc.borrow_mut().right.take();
        inner(left, right)
    } else {
        true
    }
}

pub type SolutionFn = fn(Option<Rc<RefCell<TreeNode>>>) -> bool;

fn check_solution(func: SolutionFn) {
    let root = TreeNode::from_slice(&[1, 2, 2, 3, 4, 4, 3]);
    assert!(func(root));

    let root = TreeNode::from_slice(&[1, 2, 2, i32::MAX, 3, i32::MAX, 3]);
    assert!(!func(root));
}

fn main() {
    check_solution(is_symmetric1);
    check_solution(is_symmetric2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, is_symmetric1};

    #[test]
    fn test_is_symmetric1() {
        check_solution(is_symmetric1);
    }
}
