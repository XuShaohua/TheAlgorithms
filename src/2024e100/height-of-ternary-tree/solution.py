#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

class TreeNode:
    def __init__(self, value):
        self.value = value
        self.left = None
        self.middle = None
        self.right = None

    # 向当前节点的子节点插入新的值
    # 如果当前节点的子节点不存在, 就创建它
    # 如果对应的子节点已经存在, 就在子节点中完成插入操作
    def insert(self, value):
        if self.value - value > 500:
            # 去左子节点
            if self.left:
                self.left.insert(value)
            else:
                self.left = TreeNode(value)
        elif value - self.value > 500:
            # 去右子节点
            if self.right:
                self.right.insert(value)
            else:
                self.right = TreeNode(value)
        else:
            # 去中间子节点
            if self.middle:
                self.middle.insert(value)
            else:
                self.middle = TreeNode(value)

# 构造三叉搜索树
def build_tree(array: list[int]) -> TreeNode:
    # 创建根节点
    root = TreeNode(array[0])
    for value in array[1:]:
        root.insert(value)
    return root

# 递归访问所有节点
def tree_height(root: TreeNode) -> int:
    if not root:
        return 0
    left_height = tree_height(root.left)
    middle_height = tree_height(root.middle)
    right_height = tree_height(root.right)
    child_height = max(left_height, middle_height, right_height)
    return 1 + child_height

def main():
    # 读取输入
    # 读取节点个数
    num_nodes = int(input())
    assert 1 <= num_nodes <= 10000
    # 读取所有节点的值, 存放到数组
    array = list(map(int, input().split()))
    assert len(array) == num_nodes

    # 构造三叉树
    root = build_tree(array)

    # 获取树的最大高度
    height = tree_height(root)

    # 打印结果
    print(height)

if __name__ == "__main__":
    main()
