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
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    fn from_slice_inner(slice: &[i32], index: usize) -> Option<Rc<RefCell<TreeNode>>> {
        // 数组为空, 或者索引无效
        if slice.is_empty() || index >= slice.len() {
            return None;
        }

        // 节点为空
        let val = slice[index];
        if val == i32::MAX {
            return None;
        }

        let left = Self::from_slice_inner(slice, 2 * index + 1);
        let right = Self::from_slice_inner(slice, 2 * index + 2);

        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    /// 从层序遍历的数组构造二叉树
    #[must_use]
    pub fn from_slice(slice: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        Self::from_slice_inner(slice, 0)
    }

    /// 以层序遍历的方式返回二叉树
    pub fn level_traverse(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return Vec::new();
        }

        // 使用队列来缓存当前层
        let mut queue = VecDeque::new();
        queue.push_back(root);

        let mut order = Vec::new();
        while !queue.is_empty() {
            let len = queue.len();
            for _ in 0..len {
                // 访问队列头部的元素
                let curr: Option<Rc<RefCell<TreeNode>>> = queue.pop_front().unwrap();
                if curr.is_none() {
                    order.push(i32::MAX);
                    continue;
                }
                let curr = curr.unwrap();
                order.push(curr.borrow().val);

                // 如果左子节点不为空, 就让它入队列.
                if let Some(left) = &curr.borrow().left {
                    queue.push_back(Some(Rc::clone(left)));
                };
                // 如果右子节点不为空, 就让它入队列.
                if let Some(right) = &curr.borrow().right {
                    queue.push_back(Some(Rc::clone(right)));
                };
            }
        }

        order
    }
}

// Recursive
pub fn preorder_traversal1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn preorder(root: Option<Rc<RefCell<TreeNode>>>, list: &mut Vec<i32>) {
        // 如果根节点为空, 则忽略并直接返回
        if let Some(root) = root {
            // 先访问根节点
            list.push(root.borrow().val);

            // 然后递归遍历左子树
            preorder(root.borrow_mut().left.take(), list);

            // 最后递归遍历右子树
            preorder(root.borrow_mut().right.take(), list);
        }
    }

    let mut list = Vec::new();
    preorder(root, &mut list);

    list
}

// Stack
// 使用本地的栈代替函数调用栈, 实现递归访问.
pub fn preorder_traversal2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    // 如果二叉树根节点为空, 直接返回.
    if root.is_none() {
        return Vec::new();
    }

    let mut list = Vec::new();
    let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![root];

    // 一直循环, 直到栈为空.
    while !stack.is_empty() {
        // 弹出根节点
        if let Some(Some(node)) = stack.pop() {
            // 访问根节点
            list.push(node.borrow().val);

            // 右子树先入栈
            if node.borrow().right.is_some() {
                stack.push(node.borrow_mut().right.take());
            };

            // 左子树后入栈
            if node.borrow().left.is_some() {
                stack.push(node.borrow_mut().left.take());
            }
        }
    }

    list
}

pub type SolutionFn = fn(Option<Rc<RefCell<TreeNode>>>) -> Vec<i32>;

fn check_solution(func: SolutionFn) {
    // TODO(Shaohua): 跳过空白的根节点
    //let root = TreeNode::from_slice(&[1, i32::MAX, 2, 3]);
    let root = TreeNode::from_slice(&[1, i32::MAX, 2, i32::MAX, i32::MAX, 3]);
    println!("root: {root:#?}");
    println!("root: {:?}", TreeNode::level_traverse(root.clone()));
    let expected = vec![1, 2, 3];
    assert_eq!(func(root), expected);

    let root = TreeNode::from_slice(&[]);
    let expected = vec![];
    assert_eq!(func(root), expected);

    let root = TreeNode::from_slice(&[1]);
    let expected = vec![1];
    assert_eq!(func(root), expected);
}

fn main() {
    check_solution(preorder_traversal1);
    check_solution(preorder_traversal2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, preorder_traversal1, preorder_traversal2};

    #[test]
    fn test_preorder_traversal1() {
        check_solution(preorder_traversal1);
    }

    #[test]
    fn test_preorder_traversal2() {
        check_solution(preorder_traversal2);
    }
}
