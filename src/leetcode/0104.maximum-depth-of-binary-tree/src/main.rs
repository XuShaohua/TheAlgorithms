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
        fn from_slice_inner(slice: &[i32], index: usize) -> Option<Rc<RefCell<TreeNode>>> {
            if slice.is_empty() {
                return None;
            }
            if index >= slice.len() {
                return None;
            }
            let val: i32 = slice[index];
            if val == i32::MAX {
                return None;
            }

            let left = from_slice_inner(slice, index * 2 + 1);
            let right = from_slice_inner(slice, index * 2 + 2);
            Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
        }

        from_slice_inner(slice, 0)
    }
}

// 层序遍历
pub fn max_depth1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max_level = 0;
    let mut queue = VecDeque::new();
    queue.push_back(root);

    while !queue.is_empty() {
        let len = queue.len();
        let mut has_node = false;

        // 遍历本层的所有节点
        for _i in 0..len {
            let node = queue.pop_front().unwrap();
            if let Some(node_rc) = node {
                has_node = true;
                // 访问当前节点
                let _val: i32 = node_rc.borrow().val;
                // 将左右子节点入队列
                queue.push_back(node_rc.borrow_mut().left.take());
                queue.push_back(node_rc.borrow_mut().right.take());
            }
        }

        if has_node {
            max_level += 1;
        }
    }

    max_level
}

// Recursive
pub fn max_depth2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn inner(root: Option<Rc<RefCell<TreeNode>>>, last_level: i32) -> i32 {
        if let Some(node_rc) = root {
            let current_level: i32 = last_level + 1;
            // 递归访问左子树
            let left_level: i32 = inner(node_rc.borrow_mut().left.take(), current_level);
            // 递归访问右子树
            let right_level: i32 = inner(node_rc.borrow_mut().right.take(), current_level);

            // 计算最深的层
            left_level.max(right_level)
        } else {
            // 根节点是空节点
            last_level
        }
    }
    inner(root, 0)
}

pub type SolutionFn = fn(Option<Rc<RefCell<TreeNode>>>) -> i32;

fn check_solution(func: SolutionFn) {
    let root = TreeNode::from_slice(&[3, 9, 20, i32::MAX, i32::MAX, 15, 7]);
    assert_eq!(func(root), 3);

    let root = TreeNode::from_slice(&[1, i32::MAX, 2]);
    assert_eq!(func(root), 2);
}

fn main() {
    check_solution(max_depth1);
    check_solution(max_depth2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, max_depth1};

    #[test]
    fn test_max_depth1() {
        check_solution(max_depth1);
    }
}
