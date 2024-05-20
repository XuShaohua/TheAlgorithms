// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<NodeLink>,
    pub right: Option<NodeLink>,
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
    pub fn from_slice(slice: &[i32]) -> Option<NodeLink> {
        fn inner(slice: &[i32], index: usize) -> Option<NodeLink> {
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

pub type NodeLink = Rc<RefCell<TreeNode>>;

// Recursive
// 我们假设二叉树中元素的值都是唯一的.
pub fn build_tree1(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<NodeLink> {
    fn inner(preorder: &[i32], inorder: &[i32], n: usize) -> Option<NodeLink> {
        // 递归的终止条件是, 到达了空节点.
        if n == 0 {
            return None;
        }
        let mut k = 0;
        // 确定根节点在 inorder 中的位置
        while preorder[0] != inorder[k] {
            k += 1;
        }
        debug_assert!(k < n);

        let val: i32 = inorder[k];
        // preorder: 根节点 - 左子树 - 右子树
        // inorder: 左子树 - 根节点 - 右子树
        let left = inner(&preorder[1..(k + 1)], &inorder[0..k], k);
        let right = inner(&preorder[(k + 1)..], &inorder[(k + 1)..], n - k - 1);

        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
    inner(&preorder, &inorder, inorder.len())
}

pub type SolutionFn = fn(Vec<i32>, Vec<i32>) -> Option<NodeLink>;

fn check_solution(func: SolutionFn) {
    let preorder = vec![3, 9, 20, 15, 7];
    let inorder = vec![9, 3, 15, 20, 7];
    let expected = TreeNode::from_slice(&[3, 9, 20, i32::MAX, i32::MAX, 15, 7]);
    assert_eq!(func(preorder, inorder), expected);

    let preorder = vec![-1];
    let inorder = vec![-1];
    let expected = TreeNode::from_slice(&[-1]);
    assert_eq!(func(preorder, inorder), expected);
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
