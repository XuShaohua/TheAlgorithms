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
// 前缀树, 得到数组, 转换成字符串, 然后判断是不是子串
pub fn is_subtree1(
    root: Option<Rc<RefCell<TreeNode>>>,
    sub_root: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    fn to_string(root: Option<Rc<RefCell<TreeNode>>>, list: &mut String) {
        if let Some(root_rc) = root {
            // 访问当前节点
            let val: i32 = root_rc.borrow().val;
            // 转换成字符
            let val_u32: u32 = (val + 4000) as u32;
            let chr: char = char::from_u32(val_u32).unwrap_or_default();
            list.push(chr);
            // 如果左子树不为空, 就访问左子树
            to_string(root_rc.borrow_mut().left.take(), list);
            // 如果右子树不为空, 就访问右子树
            to_string(root_rc.borrow_mut().right.take(), list);
        } else {
            // 如果当前节点为空, 就插入 ' '
            list.push(' ');
        }
    }
    let mut root_str = String::with_capacity(8000);
    let mut sub_str = String::with_capacity(4000);
    to_string(root, &mut root_str);
    to_string(sub_root, &mut sub_str);
    root_str.contains(&sub_str)
}

// Recursive
pub fn is_subtree2(
    root: Option<Rc<RefCell<TreeNode>>>,
    sub_root: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    fn is_same_tree(
        root: &Option<Rc<RefCell<TreeNode>>>,
        sub_root: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root, sub_root) {
            (Some(root_rc), Some(sub_root_rc)) => {
                if root_rc.borrow().val != sub_root_rc.borrow().val {
                    return false;
                }

                return is_same_tree(&root_rc.borrow().left, &sub_root_rc.borrow().left)
                    && is_same_tree(&root_rc.borrow().right, &sub_root_rc.borrow().right);
            }
            (None, None) => true,
            _ => false,
        }
    }

    fn inner(
        root: &Option<Rc<RefCell<TreeNode>>>,
        sub_root: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if sub_root.is_none() {
            return true;
        }
        if is_same_tree(root, sub_root) {
            return true;
        }

        if let Some(root_rc) = root {
            if inner(&root_rc.borrow().left, sub_root) || inner(&root_rc.borrow().right, sub_root) {
                return true;
            }
        }
        false
    }

    inner(&root, &sub_root)
}

pub type SolutionFn = fn(Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>) -> bool;

fn check_solution(func: SolutionFn) {
    let root = TreeNode::from_slice(&[3, 4, 5, 1, 2]);
    let sub_root = TreeNode::from_slice(&[4, 1, 2]);
    assert!(func(root, sub_root));

    let root = TreeNode::from_slice(&[3, 4, 5, 1, 2, i32::MAX, i32::MAX, i32::MAX, i32::MAX, 0]);
    let sub_root = TreeNode::from_slice(&[4, 1, 2]);
    assert!(!func(root, sub_root));
}

fn main() {
    check_solution(is_subtree1);
    check_solution(is_subtree2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, is_subtree1, is_subtree2};

    #[test]
    fn test_is_subtree1() {
        check_solution(is_subtree1);
    }

    #[test]
    fn test_is_subtree2() {
        check_solution(is_subtree2);
    }
}
