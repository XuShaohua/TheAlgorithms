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
pub fn invert_tree1(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn inner(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(root_rc) = root {
            let mut left = root_rc.borrow_mut().left.take();
            let mut right = root_rc.borrow_mut().right.take();
            inner(&mut left);
            inner(&mut right);
            root_rc.borrow_mut().left = right;
            root_rc.borrow_mut().right = left;
        }
    }

    let mut root = root;
    inner(&mut root);
    root
}

pub type SolutionFn = fn(Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>;

fn check_solution(func: SolutionFn) {
    let root = TreeNode::from_slice(&[4, 2, 7, 1, 3, 6, 9]);
    let expected = TreeNode::from_slice(&[4, 7, 2, 9, 6, 3, 1]);
    assert_eq!(func(root), expected);

    let root = TreeNode::from_slice(&[2, 1, 3]);
    let expected = TreeNode::from_slice(&[2, 3, 1]);
    assert_eq!(func(root), expected);

    let root = TreeNode::from_slice(&[]);
    let expected = TreeNode::from_slice(&[]);
    assert_eq!(func(root), expected);
}

fn main() {
    check_solution(invert_tree1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, invert_tree1};

    #[test]
    fn test_invert_tree1() {
        check_solution(invert_tree1);
    }
}
