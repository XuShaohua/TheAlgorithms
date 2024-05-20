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
pub fn build_tree1(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn inner(inorder: &[i32], postorder: &[i32], n: usize) -> Option<Rc<RefCell<TreeNode>>> {
        // 遇到了空节点, 终止递归
        if n == 0 {
            return None;
        }

        debug_assert!(inorder.len() == postorder.len());

        // 寻找根节点
        let mut k = 0;
        while k < n && inorder[k] != postorder[n - 1] {
            k += 1;
        }
        debug_assert!(k < n);

        let val: i32 = inorder[k];

        // inorder: 左子树 - 根节点 - 右子树
        // postorder: 左子树 - 右子树 - 根节点
        let left = inner(&inorder[0..k], &postorder[0..k], k);
        let right = inner(&inorder[(k + 1)..n], &postorder[k..(n - 1)], n - k - 1);

        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
    inner(&inorder, &postorder, postorder.len())
}

pub type SolutionFn = fn(Vec<i32>, Vec<i32>) -> Option<Rc<RefCell<TreeNode>>>;

fn check_solution(func: SolutionFn) {
    let inorder = vec![9, 3, 15, 20, 7];
    let postorder = vec![9, 15, 7, 20, 3];
    let expected = TreeNode::from_slice(&[3, 9, 20, i32::MAX, i32::MAX, 15, 7]);
    assert_eq!(func(inorder, postorder), expected);

    let inorder = vec![-1];
    let postorder = vec![-1];
    let expected = TreeNode::from_slice(&[-1]);
    assert_eq!(func(inorder, postorder), expected);
}

fn main() {
    check_solution(build_tree1);
}

#[cfg(test)]
mod tests {
    use super::{build_tree1, check_solution};

    #[test]
    fn test_build_tree1() {
        check_solution(build_tree1);
    }
}
