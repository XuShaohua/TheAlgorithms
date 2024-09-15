#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

def main():
    # 读取输入
    # 遍历每个坐标点
    # 计算相邻坐标点之间形成的矩形面积
    # 然后计算所有面积之和, 就是结果
    parts = input().split()
    assert len(parts) == 2
    # 移动的坐标点数
    num_points = int(parts[0])
    # 终点E所在的X坐标
    end_x = int(parts[1])

    points = []
    last_y = 0
    # 遍历所有的输入坐标, 并计算它们的绝对坐标
    for i in range(num_points):
        parts = input().split()
        assert len(parts) == 2
        x = int(parts[0])
        offset_y = int(parts[1])
        y = last_y + offset_y
        points.append((x, y))
        last_y = y


    # 移动的总面积
    total_areas = 0
    # 出发点是原点
    last_point = (0, 0)

    # 将最后一个点也加入进来
    end_point = (end_x, 0)
    points.append(end_point)

    # 遍历所有的点
    for point in points:
        # 机器人移动方式是: 先横向移动到 point.x,  再纵向移动到 point.y
        # 矩形面积是 dx * dy
        dx = abs(point[0] - last_point[0])
        #dy = abs_int(point[1] - last_point[1])
        dy = abs(last_point[1])
        total_areas += dx * dy
        last_point = point

    print(total_areas)

if __name__ == "__main__":
    main()
