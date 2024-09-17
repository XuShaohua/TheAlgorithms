#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

def main():
    # 读取输入
    steps_str = input()
    steps = list(map(int, steps_str[1:-1].split(",")))
    assert 0 <= len(steps) <= 5000
    count = int(input())
    assert 0 < count <= 1000

    # 所谓的两步, 就是两数之和等于 count
    # 直接遍历, 或者先生成一个字典, 加快搜索
    min_index = 10 ** 9
    ans = []
    for i in range(len(steps) - 1):
        for j in range(i + 1, len(steps)):
            # 两数之和等于 count
            # 并且两数索引之和更小
            if steps[i] + steps[j] == count and i + j < min_index:
                min_index = min(min_index, i + j)
                ans = [steps[i], steps[j]]
            # 忽略掉后面的步骤
            if i + j > min_index:
                break

        # 忽略掉后面的步骤
        if i + i > min_index:
            break

    # 打印结果
    print("[%d, %d]" % (ans[0], ans[1]))

if __name__ == "__main__":
    main()
