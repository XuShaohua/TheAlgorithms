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

            let left = inner(slice, index * 2 + 1);

            let right = inner(slice, index * 2 + 2);
            Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
        }

        inner(slice, 0)
    }

    pub fn debug_print(root: Option<Rc<RefCell<TreeNode>>>) {
        let mut queue = VecDeque::new();
        queue.push_back(root);

        print!("queue: [");
        while !queue.is_empty() {
            let len = queue.len();
            for _i in 0..len {
                // 取出队头的节点
                let node = queue.pop_front();
                if let Some(Some(node_rc)) = node {
                    // 访问当前节点
                    let val: i32 = node_rc.borrow().val;
                    print!("{val}, ");
                    // 把左右子树入队列
                    queue.push_back(node_rc.borrow_mut().left.clone());
                    queue.push_back(node_rc.borrow_mut().right.clone());
                }
            }
        }
        println!("]");
    }
}

// Recursive
pub fn min_depth1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn inner(root: Option<Rc<RefCell<TreeNode>>>, last_level: i32) -> i32 {
        if let Some(node_rc) = root {
            // 当前节点
            let current_level: i32 = last_level + 1;
            // 遍历访问左子树和右子树
            let left_level: i32 = inner(node_rc.borrow_mut().left.take(), current_level);
            let right_level: i32 = inner(node_rc.borrow_mut().right.take(), current_level);
            if left_level == current_level && right_level == current_level {
                // 如果左子树右子树都是空的, 说明当前是叶节点
                current_level
            } else if left_level == current_level {
                // 左子树为空, 右子树不空, 说明当前不是叶节点
                right_level
            } else if right_level == current_level {
                // 左子树不为空, 右子树为空, 说明当前不是叶节点
                left_level
            } else {
                // 左右子树都不为空, 取最小值
                left_level.min(right_level)
            }
        } else {
            // 根节点为空
            last_level
        }
    }
    inner(root, 0)
}

// Stack
// 前序遍历, 使用本地的栈代替函数调用栈
pub fn min_depth2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut stack = vec![(root, 1_i32)];
    let mut min_level = i32::MAX;

    // 一直循环直到栈为空
    while let Some((node, level)) = stack.pop() {
        if let Some(node_rc) = node {
            // 不需要访问当前节点
            let left_node = node_rc.borrow_mut().left.take();
            let right_node = node_rc.borrow_mut().right.take();
            let next_level: i32 = level + 1;
            match (left_node, right_node) {
                (Some(left_node), Some(right_node)) => {
                    // 左右子树都不为空, 左右子树都入栈
                    stack.push((Some(right_node), next_level));
                    stack.push((Some(left_node), next_level));
                }
                (Some(left_node), None) => {
                    // 只有左子树不为空, 左子树入栈
                    stack.push((Some(left_node), next_level));
                }
                (None, Some(right_node)) => {
                    // 只有右子树不为空, 右子树入栈
                    stack.push((Some(right_node), next_level));
                }
                (None, None) => {
                    // 左右两个子树都为空, 说明当前就是叶节点
                    min_level = min_level.min(level);
                }
            }
        }
    }
    min_level
}

pub type SolutionFn = fn(Option<Rc<RefCell<TreeNode>>>) -> i32;

fn check_solution(func: SolutionFn) {
    let root = TreeNode::from_slice(&[3, 9, 20, i32::MAX, i32::MAX, 15, 7]);
    //println!("root: {root:#?}");
    //TreeNode::debug_print(root.clone());
    assert_eq!(func(root), 2);

    let root = TreeNode::from_slice(&[2, i32::MAX, 3, i32::MAX, 4, i32::MAX, 5, i32::MAX, 6]);
    //println!("root: {root:#?}");
    TreeNode::debug_print(root.clone());
    // FIXME(Shaohua): from_slice() is invalid
    //assert_eq!(func(root), 5);
    assert_eq!(func(root), 3);
}

fn main() {
    check_solution(min_depth1);
    check_solution(min_depth2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, min_depth1, min_depth2};

    #[test]
    fn test_min_depth1() {
        check_solution(min_depth1);
    }

    #[test]
    fn test_min_depth2() {
        check_solution(min_depth2);
    }
}
