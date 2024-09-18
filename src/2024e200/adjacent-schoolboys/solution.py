#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

def main():
    # 读取输入
    # 然后遍历二维数组中的所有节点, 找到 "M", 然后基于此, 向四个方向移动, 找到最长的连续队列
    rows, columns = map(int, input().split(","))
    assert 0 < rows and 0 < columns and rows * columns < 10000
    table = [list(input().split(",")) for _row in range(rows)]
    assert len(table[0]) == columns

    MAN = "M"
    WOMEN = "W"

    # 当前节点可以连接最多男生的数量
    count_list = []
    
    # 查找的四个方向, 向右/向下/右下角/左下角
    directions = ((1, 0), (0, 1), (1, 1), (-1, 1))

    def get_max_male_student(x, y, count_list):
        # 遍历所有方向
        for dx, dy in directions:
            x2 = x + dx
            y2 = y + dy
            # 男学生的数量
            count = 1

            # 按当前方向一直查找
            # 1. 新的坐标在队列内
            # 2. 新的位置是男生
            while 0 <= x2 < rows and 0 <= y2 < columns and table[x2][y2] == MAN:
                x2 += dx
                y2 += dy
                count += 1

            count_list.append(count)

    # 遍历所有节点
    for row in range(rows):
        for column in range(columns):
            # 如果当前节点是位男生, 就递归地找最长连续序列
            if table[row][column] == MAN:
                get_max_male_student(row, column, count_list)

    # 打印结果
    print(max(count_list))

if __name__ == "__main__":
    main()
