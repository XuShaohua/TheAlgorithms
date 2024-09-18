#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

import sys

def main():
    # 先读取输入
    parts = input().split()
    assert len(parts) == 2
    # 菜的个数
    n = int(parts[0])
    # 手速
    m = int(parts[1])

    # 放菜的策略, 每个菜可以食用的时间点
    times = []
    for line in sys.stdin.readlines():
        start, delay = list(map(int, line.split()))
        time = start + delay
        times.append(time)
    assert len(times) == n

    # 每个时间点可以吃的菜的个数, 初始化为0
    # 注意, 时间点是从1开始计数的, 我们这里要从0开始
    food_nums = [0] * (max(times) + 1)
    for time in times:
        # 这个时间点有菜
        food_nums[time] += 1

    # 记录每种策略下可以吃到的菜的数量, 最后从中选择最大值就行
    eat_food = []

    # DFS 查找当前时间点可以吃的菜的数量
    def dfs(time, current_food):
        # 超过最大时间点, 后面没菜了, 终止递归
        if time >= len(food_nums):
            # 当前策略下吃到的所有的菜
            eat_food.append(current_food)
            return
        elif food_nums[time] > 0:
            # 当前时间点有菜, 有两个策略:
            # 1. 直接吃, 然后等待m秒
            dfs(time + m, current_food + 1)
            # 2. 不吃, 去到下个时间点吃
            dfs(time + 1, current_food)
        else:
            # 当前时间点没有菜, 去下个时间点
            dfs(time + 1, current_food)

    # 从第1个时间点开始进行递归搜索
    dfs(1, 0)
    
    # 打印可以吃到的最多的菜
    print(max(eat_food))

if __name__ == "__main__":
    main()
