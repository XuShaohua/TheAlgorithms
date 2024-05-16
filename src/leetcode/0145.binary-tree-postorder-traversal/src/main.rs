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

// Recursive
pub fn postorder_traversal1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn postorder(root: Option<Rc<RefCell<TreeNode>>>, list: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }
        let root = root.unwrap();

        // 先访问左子树
        postorder(root.borrow_mut().left.take(), list);

        // 然后访问右子树
        postorder(root.borrow_mut().right.take(), list);

        // 最后访问根节点
        list.push(root.borrow().val);
    }

    let mut list = Vec::new();
    postorder(root, &mut list);
    list
}

// Stack
// 使用本地的栈代替函数调用栈, 递归遍历二叉树.
pub fn postorder_traversal2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if root.is_none() {
        return Vec::new();
    }

    let mut list = Vec::new();
    let mut stack = Vec::new();
    // 当前节点
    let mut root = root;
    // 前一个访问的节点, 用于确定当前节点的右子树是否访问完毕
    let mut prev: Option<Rc<RefCell<TreeNode>>> = None;

    // 一直循环, 直到当前节点为空, 且栈为空
    while root.is_some() || !stack.is_empty() {
        // 当前节点有左子树时, 将当前节点入栈, 递归遍历左子树
        while let Some(root_rc) = root {
            stack.push(Some(Rc::clone(&root_rc)));
            root = root_rc.borrow_mut().left.take();
        }

        // 遍历到了最左侧, 已经无子节点, 将最左侧节点出栈.
        let node = stack.pop().unwrap();
        let node_rc = node.unwrap();

        // 如果当前节点没有右子树, 或者右子树已经访问完毕, 就访问当前节点本身
        if node_rc.borrow().right.is_none() || node_rc.borrow().right == prev {
            // 访问该节点
            list.push(node_rc.borrow().val);

            // 记录当前节点
            prev = Some(node_rc);

            // 将根节点置为空
            root = None;
        } else {
            // 右子树还没访问完, 将当前节点继续压入栈中
            stack.push(Some(Rc::clone(&node_rc)));

            // 继续访问右子树, root = node.right
            root = node_rc.borrow_mut().right.take();
        }
    }

    list
}

pub type SolutionFn = fn(Option<Rc<RefCell<TreeNode>>>) -> Vec<i32>;

fn check_solution(func: SolutionFn) {
    //let root = TreeNode::from_slice(&[1, i32::MAX, 2, 3]);
    let root = TreeNode::from_slice(&[1, i32::MAX, 2, i32::MAX, i32::MAX, 3]);
    println!("root: {root:#?}");
    let expected = vec![3, 2, 1];
    assert_eq!(func(root), expected);

    let root = TreeNode::from_slice(&[]);
    let expected = vec![];
    assert_eq!(func(root), expected);

    let root = TreeNode::from_slice(&[1]);
    let expected = vec![1];
    assert_eq!(func(root), expected);
}

fn main() {
    check_solution(postorder_traversal1);
    check_solution(postorder_traversal2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, postorder_traversal1, postorder_traversal2};

    #[test]
    fn test_postorder_traversal1() {
        check_solution(postorder_traversal1);
    }

    #[test]
    fn test_postorder_traversal2() {
        check_solution(postorder_traversal2);
    }
}
