#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

from collections import deque

def main():
    # 读取行列值
    rows, columns = list(map(int, input().split()))

    horse_matrix = []
    for row in range(rows):
        parts = input().split()
        for column in range(columns):
            if parts[column] != ".":
                # 马可以移动的最大步数
                steps = int(parts[column])
                horse_matrix.append((row, column, steps))

    # 可以移动的方向
    directions = ((-1, -2), (-1, 2), (1, -2), (1, 2), (-2, -1), (-2, 1), (2, -1), (2, 1))
    INITIAL_STEPS = 10 ** 9

    def dfs(row, column, x, y, max_move_steps, dist, visited):
        if row == x and column == y:
            return dist

        # 访问所有的方向
        for dx, dy in directions:
            x2 = x + dx
            y2 = y + dy
            # 检查新的坐标位置是否有效, 是否访问过
            if 0 <= x2 < rows and 0 <= y2 < columns and dist < max_move_steps and (x2, y2) not in visited:
                visited.add((x2, y2))
                steps = dfs(row, column, x2, y2, max_move_steps, dist + 1, visited)
                if steps > -1:
                    return steps

        return -1

    min_steps = INITIAL_STEPS

    # 遍历每个位置
    for row in range(rows):
        for column in range(columns):
            # 所有马移动的总步数
            total_steps = 0
            possible_move = True

            # 遍历每只马
            for x, y, move_steps in horse_matrix:
                visited = set()
                steps = dfs(row, column, x, y, move_steps, 0, visited)
                print("steps:", steps)
                if steps > -1:
                    total_steps += steps
                    break
                else:
                    possible_move = False
            if possible_move:
                print("total steps:", total_steps)
                min_steps = min(min_steps, total_steps)

    print("min_steps:", min_steps)

if __name__ == "__main__":
    main()
