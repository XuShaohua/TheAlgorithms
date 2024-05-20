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
#[allow(unused_assignments)]
pub fn flatten0(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    //void flatten(struct TreeNode* root) {
    //  while (root != NULL) {
    //    if (root->left != NULL) {
    //      struct TreeNode* right_leaf = root->left;
    //      while (right_leaf->right != NULL) {
    //        right_leaf = right_leaf->right;
    //      }
    //
    //      right_leaf->right = root->right;
    //      root->right = root->left;
    //      root->left = NULL;
    //    }
    //    root = root->right;
    //  }
    //}
    while let Some(root_rc) = root {
        // 取到根节点的左子树
        let left = root_rc.borrow_mut().left.take();
        let mut right = root_rc.borrow_mut().right.take();

        if let Some(left_rc) = left {
            let mut right_leaf: Option<Rc<RefCell<TreeNode>>> = Some(Rc::clone(&left_rc));
            // 找到右侧的叶节点
            while let Some(right_leaf_rc) = right_leaf.clone() {
                let new_node = right_leaf_rc.borrow().right.clone();
                if new_node.is_none() {
                    break;
                }
                right_leaf = new_node;
            }

            // 把根节点的右子树作为右侧叶节点的右子树
            if let Some(right_leaf_rc) = right_leaf {
                right_leaf_rc.borrow_mut().right = right;
            }
            // 把根节点的左子树作为根节点的右子树
            right = Some(left_rc);
        }

        // FIXME(Shaohua): references error
        //*root = root_rc.borrow_mut().right;
    }
}

pub fn flatten1(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    fn inner(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root_rc) = root {
            let left_leaf = inner(&root_rc.borrow().left);
            let right_leaf = inner(&root_rc.borrow().right);

            // 如果左侧节点不为空, 就进行交换.
            if let Some(left_leaf_rc) = &left_leaf {
                let right = root_rc.borrow_mut().right.take();
                left_leaf_rc.borrow_mut().right = right;
                let left = root_rc.borrow_mut().left.take();
                root_rc.borrow_mut().right = left;
            }

            [right_leaf, left_leaf, Some(Rc::clone(root_rc))]
                .into_iter()
                .find(Option::is_some)
                .unwrap()
        } else {
            None
        }
    }

    inner(root);
}

pub type SolutionFn = fn(&mut Option<Rc<RefCell<TreeNode>>>);

fn check_solution(func: SolutionFn) {
    let mut root = TreeNode::from_slice(&[1, 2, 5, 3, 4, i32::MAX, 6]);
    // FIXME(Shaohua): from_slice does not support this format.
    let _expected = TreeNode::from_slice(&[
        1,
        i32::MAX,
        2,
        i32::MAX,
        3,
        i32::MAX,
        4,
        i32::MAX,
        5,
        i32::MAX,
        6,
    ]);
    func(&mut root);
    //assert_eq!(root, expected);

    let mut root = TreeNode::from_slice(&[0]);
    let expected = TreeNode::from_slice(&[0]);
    func(&mut root);
    assert_eq!(root, expected);

    let mut root = TreeNode::from_slice(&[]);
    let expected = TreeNode::from_slice(&[]);
    func(&mut root);
    assert_eq!(root, expected);
}

fn main() {
    check_solution(flatten1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, flatten1};

    #[test]
    fn test_flatten1() {
        check_solution(flatten1);
    }
}
