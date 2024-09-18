#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

def main():
    # 先读取输入值
    # 种树的数量
    total = int(input())
    # 树树的数量
    dead_count = int(input())
    # 死树的位置
    dead_list = list(map(int, input().split()))
    # 补种的数量 k
    k = int(input())
    assert len(dead_list) == dead_count

    # 树的生死状态
    # 0 表示存活, 1 表示不存活
    states = [0] * total

    # 更新树的状态
    # 注意这里是从1开始计数的, 要转换成从0开始计数
    for dead in dead_list:
        states[dead - 1] = 1

    # 双指针法
    left = 0
    right = 0
    # 最大连续存活的树的数量
    max_alive = 0
    # 窗口左侧边界经过的死树的数量
    dead_left = 0
    # 窗口右侧边界经过的死树的数量
    dead_right = 0

    # 遍历所有的树
    while right < total:
        # 更新窗口右侧经过的死树的数量
        dead_right += states[right]

        # 如果窗口内死树的数量比能补种的数量多, 就将窗口左侧往右移
        while dead_right - dead_left > k:
            dead_left += states[left]
            left += 1

        # 更新最大连续活着的树的数量
        max_alive = max(max_alive, right - left + 1)
        right += 1

    print(max_alive)

if __name__ == "__main__":
    main()
