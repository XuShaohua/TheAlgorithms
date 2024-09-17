#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

import math

def main():
    # 先读取输入
    # 得到一个 N*N 的二维数组
    array = list(map(int, input().split(",")))

    AFFECTED = 1
    UNAFFECTED = 0

    # 然后检查感染区域, 如果全是0或者全是1, 就返回-1
    # 否则就模拟每天的感染情况
    all_affected = all(x == AFFECTED for x in array)
    all_unaffected= all(x == UNAFFECTED for x in array)
    if all_affected or all_unaffected:
        print(-1)
        return

    # 计算二维数组的边界
    rows = int(math.sqrt(len(array)))
    columns = rows
    assert rows * rows == len(array)
    assert 1 <= rows < 200

    # 将一维数组转换成二维数组, 方便进行定位
    matrix = [array[row * columns: (row + 1) * columns] for row in range(rows)]

    # 可能的感染方向
    directions = ((1, 0), (-1, 0), (0, 1), (0, -1))

    # 创建新一天的感染快照, 注意这里使用深拷贝, 解除两个数组间的关联
    snapshot = matrix[:]
    # 持续感染的天数
    days = 0
    # 计算还未被感染的区域数量
    unaffected_remains = len(array) - len(list(x for x in array if x == UNAFFECTED))

    # 持续感染, 直到扩散到所有区域
    while unaffected_remains > 0:
        for row in range(rows):
            for col in range(columns):
                if matrix[row][col] == UNAFFECTED:
                    # 当前区域尚未被感染, 等待被感染
                    pass
                else:
                    # 去感染相邻四周
                    for dx, dy in directions:
                        row2 = row + dx
                        col2 = col + dy
                        # 检查新区域的有效性, 它是否已被感染
                        if 0 <= row2 < rows and 0 <= col2 < columns and snapshot[row2][col2] == UNAFFECTED:
                            snapshot[row2][col2] = AFFECTED
                            unaffected_remains -= 1
        # 更新当天的感染情况
        days += 1
        # 注意这里是深拷贝
        matrix = snapshot[:]

    print(days)

if __name__ == "__main__":
    main()
