#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

def main():
    # 读取字母的行数
    rows = int(input())
    # 字母表, N * N 的二维数组
    table = []
    for _i in range(rows):
        # 读取当前行的所有字母
        table.append(list(input().split(",")))
        assert len(table[-1]) == rows
    # 读取输入的单词
    word = input()

    # 用于标记已经访问过的字符, N * N
    visited = [[False] * rows for _row in range(rows)]

    # 定义查找的四个方向
    directions = ((1, 0), (-1, 0), (0, 1), (0, -1))

    # 使用DFS搜索所有可能的位置
    def dfs(row: int, column: int, index_in_word: int, path):
        # 结束查找
        # 1. 检查坐标的边界, 如果不在 table 内
        # 2. 或者已经访问过了
        # 3. 或者单词中的字母与在表格中的不一致
        if row < 0 or row >= rows or column < 0 or column >= rows or \
                visited[row][column] or \
                word[index_in_word] != table[row][column]:
            return False
        # 将当前路径添加到 path 中
        path.append((row, column))
        # 并标记该节点已经访问过
        visited[row][column] = True
        # 如果单词中的所有字母都被找到了, 就返回
        if index_in_word + 1 == len(word):
            return True

        # 遍历所有可能的方向, 进行深入查找
        for direction in directions:
            row2 = row + direction[0]
            column2 = column + direction[1]
            # 去找单词中的下一个字母
            found = dfs(row2, column2, index_in_word + 1, path)
            # 如果在该方向找到了字符串, 就直接返回
            if found:
                return True
        # 没有找到, 当前前位置从经过的路径中移除
        path.pop()
        # 并将该坐标从被访问记录中移除
        visited[row][column] = False
        return False

    def find_string():
        # 用于存储访问路径
        path= []
        # 遍历所有的单元格
        for row in range(rows):
            for column in range(rows):
                # 如果当前单元格的字符等于单词的第一个字母
                if table[row][column] == word[0]:
                    # 使用DFS查找字符串
                    found = dfs(row, column, 0, path)
                    if found:
                        # 找到了, 就返回结果
                        positions = []
                        for row, column in path:
                            positions.append(str(row))
                            positions.append(str(column))
                        result = ",".join(positions)
                        return result

        # 没有找到合适的
        return "N"

    result = find_string()
    # 打印最后的结果
    print(result)


if __name__ == "__main__":
    main()
