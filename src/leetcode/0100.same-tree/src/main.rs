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
pub fn is_same_tree1(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (Some(p_rc), Some(q_rc)) => {
            if p_rc.borrow().val != q_rc.borrow().val {
                return false;
            }
            return is_same_tree1(p_rc.borrow_mut().left.take(), q_rc.borrow_mut().left.take())
                && is_same_tree1(
                    p_rc.borrow_mut().right.take(),
                    q_rc.borrow_mut().right.take(),
                );
        }
        (None, None) => true,
        _ => false,
    }
}

pub type SolutionFn = fn(Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>) -> bool;

fn check_solution(func: SolutionFn) {
    let p = TreeNode::from_slice(&[1, 2, 3]);
    let q = TreeNode::from_slice(&[1, 2, 3]);
    assert!(func(p, q));

    let p = TreeNode::from_slice(&[1, 2]);
    let q = TreeNode::from_slice(&[1, i32::MAX, 2]);
    assert!(!func(p, q));

    let p = TreeNode::from_slice(&[1, 2, 1]);
    let q = TreeNode::from_slice(&[1, 1, 2]);
    assert!(!func(p, q));
}

fn main() {
    check_solution(is_same_tree1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, is_same_tree1};

    #[test]
    fn test_is_same_tree1() {
        check_solution(is_same_tree1);
    }
}
