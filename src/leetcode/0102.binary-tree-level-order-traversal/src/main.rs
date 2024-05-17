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
        // 递归调用
        fn from_slice_inner(slice: &[i32], index: usize) -> Option<Rc<RefCell<TreeNode>>> {
            if slice.is_empty() {
                return None;
            }

            if index >= slice.len() {
                return None;
            }

            let val: i32 = slice[index];
            // 空节点
            if val == i32::MAX {
                return None;
            }

            let left = from_slice_inner(slice, index * 2 + 1);
            let right = from_slice_inner(slice, index * 2 + 2);
            Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
        }

        from_slice_inner(slice, 0)
    }

    // 层序遍历的方式打印
    pub fn debug_print(root: Option<Rc<RefCell<TreeNode>>>) {
        let mut queue = VecDeque::new();
        queue.push_back(root);
        print!("queue: [");
        while !queue.is_empty() {
            // 遍历当前层
            let len = queue.len();
            for _i in 0..len {
                let node = queue.pop_front();
                let node = node.unwrap();
                if let Some(node_rc) = node {
                    // 访问当前节点
                    let val: i32 = node_rc.borrow().val;
                    print!("{val}, ");
                    // 左右两个子节点入队列
                    queue.push_back(node_rc.borrow_mut().left.clone().take());
                    queue.push_back(node_rc.borrow_mut().right.clone().take());
                } else {
                    print!("Nil, ");
                }
            }
        }
        println!("]");
    }
}

pub fn level_order1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut list = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(root);

    while !queue.is_empty() {
        let len = queue.len();
        let mut level = Vec::new();
        for _ in 0..len {
            let node = queue.pop_front().unwrap();
            if let Some(node_rc) = node {
                // 访问当前节点
                let val: i32 = node_rc.borrow().val;
                level.push(val);
                // 左右子节点入队列
                queue.push_back(node_rc.borrow_mut().left.take());
                queue.push_back(node_rc.borrow_mut().right.take());
            } else {
                // Nil
            }
        }
        if !level.is_empty() {
            list.push(level);
        }
    }

    list
}

pub type SolutionFn = fn(Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>>;

fn check_solution(func: SolutionFn) {
    let root = TreeNode::from_slice(&[3, 9, 20, i32::MAX, i32::MAX, 15, 7]);
    println!("root: {root:?}");
    if let Some(root_rc) = root.as_ref() {
        TreeNode::debug_print(Some(Rc::clone(root_rc)));
    }
    let expected = vec![vec![3], vec![9, 20], vec![15, 7]];
    assert_eq!(func(root), expected);

    let root = TreeNode::from_slice(&[1]);
    let expected = vec![vec![1]];
    assert_eq!(func(root), expected);

    let root = TreeNode::from_slice(&[]);
    let expected: Vec<Vec<i32>> = vec![];
    assert_eq!(func(root), expected);
}

fn main() {
    check_solution(level_order1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, level_order1};

    #[test]
    fn test_level_order1() {
        check_solution(level_order1);
    }
}
