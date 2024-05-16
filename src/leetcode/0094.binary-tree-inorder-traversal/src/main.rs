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

    /// 层序遍历
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
}

// Recursive
pub fn inorder_traversal1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn inorder(root: Option<Rc<RefCell<TreeNode>>>, list: &mut Vec<i32>) {
        // 如果根节点为空, 直接返回.
        if root.is_none() {
            return;
        }

        if let Some(root) = root {
            // 先递归遍历左子树
            inorder(root.borrow_mut().left.take(), list);

            // 然后访问根节点
            let val: i32 = root.borrow().val;
            list.push(val);

            // 最后递归遍历右子树
            inorder(root.borrow_mut().right.take(), list);
        }
    }

    let mut list = Vec::new();
    inorder(root, &mut list);
    list
}

// Stack
// 使用本的栈代替函数调用栈实现递归访问
pub fn inorder_traversal2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    // 二叉树为空时直接返回
    if root.is_none() {
        return Vec::new();
    }

    let mut list = Vec::new();
    let mut stack = Vec::new();
    // 当前节点
    let mut node = root;

    // 一直循环, 直到当前k节点为空且栈为空
    while node.is_some() || !stack.is_empty() {
        // 当前节点不为空时, 循环遍历左子树, 并将该子树的根节点入栈
        while let Some(node_rc) = node {
            stack.push(Some(Rc::clone(&node_rc)));
            // 访问左子树, node = node.left;
            node = node_rc.borrow_mut().left.take();
        }

        // 已经遍历到最左侧节点, 当前节点已经没有左子树, 弹出最左侧节点
        node = stack.pop().unwrap();

        if let Some(node_rc) = node {
            // 访问该节点
            list.push(node_rc.borrow().val);

            // 尝试访问该节点的右子树, node = node.right;
            node = node_rc.borrow_mut().right.take();
        }
    }

    list
}

pub type SolutionFn = fn(Option<Rc<RefCell<TreeNode>>>) -> Vec<i32>;

fn check_solution(func: SolutionFn) {
    //let root = TreeNode::from_slice(&[1, i32::MAX, 2, 3]);
    let root = TreeNode::from_slice(&[1, i32::MAX, 2, i32::MAX, i32::MAX, 3]);
    println!("root: {root:#?}");
    let expected = vec![1, 3, 2];
    assert_eq!(func(root), expected);

    let root = TreeNode::from_slice(&[]);
    let expected = vec![];
    assert_eq!(func(root), expected);

    let root = TreeNode::from_slice(&[1]);
    let expected = vec![1];
    assert_eq!(func(root), expected);
}

fn main() {
    check_solution(inorder_traversal1);
    check_solution(inorder_traversal2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, inorder_traversal1, inorder_traversal2};

    #[test]
    fn test_inorder_traversal1() {
        check_solution(inorder_traversal1);
    }

    #[test]
    fn test_inorder_traversal2() {
        check_solution(inorder_traversal2);
    }
}
