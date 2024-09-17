#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

def main():
    # 读取输入
    # 四个点构成正方形的条件:
    # 1. 对角线成90度, 即点积为0
    # 2. 对角线长度相等
    # 两层遍历所有的坐标点, 就构造出一个对角线
    # 然后计算该对角线成90的另外的几个对角线, 是否也存在坐标点中

    num_points = int(input())
    assert 0 < num_points <= 100
    points = []
    for i in range(num_points):
        # 存储所有的坐标点
        point = tuple(map(int, input().split()))
        assert -10 <= point[0] <= 10
        assert -10 <= point[1] <= 10
        points.append(point)

    num_squares = 0
    points_set = set(points)

    # 遍历所有的点, 检查能否构成正方形
    for i in range(num_points):
        x1, y1 = points[i]
        for j in range(i + 1, num_points):
            x2, y2 = points[j]

            # 计算两个对角点
            x3, y3 = x1 - (y1 - y2), y1 + (x1 - x2)
            x4, y4 = x2 - (y1 - y2), y2 + (x1 - x2)
            p3 = (x3, y3)
            p4 = (x4, y4)
            if p3 in points_set and p4 in points_set:
                num_squares += 1

            # 计算另外两个对角点
            x5, y5 = x1 + (y1 - y2), y1 - (x1 - x2)
            x6, y6 = x2 + (y1 - y2), y2 - (x1 - x2)
            p5 = (x5, y5)
            p6 = (x6, y6)
            if p5 in points_set and p6 in points_set:
                num_squares += 1

    # 因为对角线计算了4次, 我们来去重
    print(num_squares)
    num_squares = num_squares // 4
    print(num_squares)

if __name__ == "__main__":
    main()
