#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

class TreeNode:

    def __init__(self, value: int, left=None, right=None):
        self.value = value
        self.left = left
        self.right = right

def build_tree(preorder, inorder, length) -> TreeNode:
    # 没有更多节点
    if length == 0:
        return None

    # preorder[0] 是根节点, 现在确定根节点在 inorder 中的位置
    k = 0
    while preorder[0] != inorder[k]:
        k += 1
    # preorder 序列的内容,  | root | left | right |
    # inorder 序列的内容, | left | root | right |, 左子树有 k 个节点

    # 构造二叉树
    node = TreeNode(preorder[0])
    node.left = build_tree(preorder[1: k + 1], inorder[: k], k)
    # 注意要考虑根节点占用一个位置
    node.right = build_tree(preorder[k + 1: ], inorder[k + 1: ], length - k - 1)
    return node

# 以中序遍历的方式递归访问二叉树
def inorder_traversal(tree: TreeNode, out: list[int]):
    if not tree:
        return
    inorder_traversal(tree.left)
    out.append(tree.value)
    inorder_traversal(tree.right)

def main():
    # 读取序列
    inorder = list(map(int, input().split()))
    preorder = list(map(int, input().split()))
    assert len(inorder) == len(preorder)

    # 我们假定二叉树中的每个节点的值都是唯一的

    # 以前序遍历序列和中序遍历序列来构造原来的二叉树
    tree = build_tree(preorder, inorder, len(preorder))

    out = []
    inorder_traversal(tree, out)

if __name__ == "__main__":
    main()
