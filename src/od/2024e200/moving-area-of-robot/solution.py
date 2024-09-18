#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

import sys

def main():
    # 读取网格的行列值
    parts = input().split()
    rows = int(parts[0])
    columns = int(parts[1])
    assert 1 <= rows <= 150
    assert 1 <= columns <= 150
    # 读取网格中的数值
    grid = []
    for line in sys.stdin.readlines():
        grid.append(list(map(int, line.split())))
        assert len(grid[-1]) == columns
    assert len(grid) == rows
    print(grid)

    # 标记已经访问过了的节点
    visited = [[False] * columns for _row in range(rows)]

    # 四个移动的方向
    directions = ((1, 0), (-1, 0), (0, 1), (0, -1))

    def dfs(grid, visited, x, y):
        if visited[x][y]:
            return 0
        # 先标记当前节点已经访问过
        visited[x][y] = True

        move_range = 1

        # 遍历四个可能的移动方向
        for dx, dy in directions:
            x1 = x + dx
            y1 = y + dy
            # 判断新的节点是否满足条件
            # 1. 在矩形范围内移动
            # 2. 新的节点未被访问过
            # 3. 两个节点上的值相差小于等于1
            if 0 <= x1 < rows and 0 <= y1 < columns and \
                    not visited[x1][y1] and abs(grid[x][y] - grid[x1][y1]) <= 1:
                # 递归访问新的节点
                move_range += dfs(grid, visited, x1, y1)
                #print("move from:", x, y, ", to :", x1, y1)

        # 返回最大能访问的节点数
        return move_range

    # 遍历所有的格子, 找到最大的移动范围
    max_range = 0
    for i in range(rows):
        for j in range(columns):
            # 使用DFS方法, 尝试向四个方向移动
            move_range = dfs(grid, visited, i, j)
            max_range = max(max_range, move_range)

    print(max_range)

if __name__ == "__main__":
    main()
