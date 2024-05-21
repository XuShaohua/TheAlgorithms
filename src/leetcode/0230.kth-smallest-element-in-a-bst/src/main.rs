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

// Bruteforce
// 遍历BST, 然后生成数组, 对它排序, 得到第k个元素
pub fn kth_smallest1(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    debug_assert!(k >= 1);

    let mut stack = vec![root];
    let mut list: Vec<i32> = Vec::new();

    // 一直循环, 直到栈为空
    while !stack.is_empty() {
        if let Some(Some(node_rc)) = stack.pop() {
            // 先访问根节点
            let val: i32 = node_rc.borrow().val;
            list.push(val);

            // 然后右子树入栈
            stack.push(node_rc.borrow_mut().right.take());
            // 最后左子树入栈
            stack.push(node_rc.borrow_mut().left.take());
        }
    }

    list.sort_unstable();
    list[(k - 1) as usize]
}

// Recursive
// 使用中序遍历的方法, 得到前k个元素, 就是要的结果.
// 因为中序遍历BST, 得到的本身就是一个有序数组, 这个是很关键的特性.
pub fn kth_smallest2(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    fn inorder(root: &Option<Rc<RefCell<TreeNode>>>, list: &mut Vec<i32>, len: usize) -> bool {
        // 根节点为空
        if let Some(root_rc) = root {
            // 先访问左子树
            if inorder(&root_rc.borrow().left, list, len) {
                return true;
            }

            let val: i32 = root_rc.borrow().val;
            list.push(val);

            if list.len() == len {
                return true;
            }

            // 最后访问右子树
            if inorder(&root_rc.borrow().right, list, len) {
                return true;
            }
        }

        false
    }

    debug_assert!(k >= 1);

    let len: usize = k as usize;
    let mut list: Vec<i32> = Vec::with_capacity(len);
    inorder(&root, &mut list, len);
    list[len - 1]
}

// TODO(Shaohua): Stack
// 使用本地栈代替函数调用栈, 实现中序遍历
//pub fn kth_smallest3(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {

pub type SolutionFn = fn(Option<Rc<RefCell<TreeNode>>>, i32) -> i32;

fn check_solution(func: SolutionFn) {
    let root = TreeNode::from_slice(&[3, 1, 4, i32::MAX, 2]);
    let k = 1;
    assert_eq!(func(root, k), 1);

    let root = TreeNode::from_slice(&[5, 3, 6, 2, 4, i32::MAX, i32::MAX, 1]);
    let k = 3;
    assert_eq!(func(root, k), 3);
}

fn main() {
    check_solution(kth_smallest1);
    check_solution(kth_smallest2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, kth_smallest1};

    #[test]
    fn test_kth_smallest1() {
        check_solution(kth_smallest1);
    }
}
