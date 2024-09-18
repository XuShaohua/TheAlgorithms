// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::io::{stdin, BufRead};

pub struct TreeNode {
    value: i32,
    left: Option<Box<Self>>,
    middle: Option<Box<Self>>,
    right: Option<Box<Self>>,
}

impl TreeNode {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            left: None,
            middle: None,
            right: None,
        }
    }

    fn boxed_new(value: i32) -> Option<Box<Self>> {
        Some(Box::new(Self {
            value,
            left: None,
            middle: None,
            right: None,
        }))
    }

    // 向当前节点的子节点插入新的值
    // 如果当前节点的子节点不存在, 就创建它
    // 如果对应的子节点已经存在, 就在子节点中完成插入操作
    pub fn insert(&mut self, value: i32) {
        if self.value - value > 500 {
            // 去左子节点
            if let Some(left) = &mut self.left {
                left.insert(value);
            } else {
                self.left = Self::boxed_new(value);
            }
        } else if value - self.value > 500 {
            // 去右子节点
            if let Some(right) = &mut self.right {
                right.insert(value);
            } else {
                self.right = TreeNode::boxed_new(value);
            }
        } else {
            // 去中间子节点
            if let Some(middle) = &mut self.middle {
                middle.insert(value);
            } else {
                self.middle = TreeNode::boxed_new(value);
            }
        }
    }
}

// 构造三叉搜索树
fn build_tree(array: &[i32]) -> TreeNode {
    assert!(!array.is_empty());

    // 创建根节点
    let mut root = TreeNode::new(array[0]);
    for &value in &array[1..] {
        root.insert(value);
    }
    root
}

// 递归访问所有节点
fn tree_height(root: &Option<Box<TreeNode>>) -> usize {
    if let Some(root) = root.as_ref() {
        let left_height = tree_height(&root.left);
        let middle_height = tree_height(&root.middle);
        let right_height = tree_height(&root.right);
        let child_height = left_height.max(middle_height.max(right_height));
        1 + child_height
    } else {
        0
    }
}

fn main() {
    // 读取输入
    // 读取节点个数
    let mut line = String::new();
    let ret = stdin().lock().read_line(&mut line);
    assert!(ret.is_ok());
    let num_nodes: usize = line.trim().parse().unwrap();
    assert!(1 <= num_nodes && num_nodes <= 10000);

    // 读取所有节点的值, 存放到数组
    line.clear();
    let ret = stdin().lock().read_line(&mut line);
    assert!(ret.is_ok());
    let array: Vec<i32> = line
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    assert_eq!(array.len(), num_nodes);

    // 构造三叉树
    let root = build_tree(&array);
    let root = Some(Box::new(root));

    // 获取树的最大高度
    let height: usize = tree_height(&root);

    // 打印结果
    println!("{height}");
}
