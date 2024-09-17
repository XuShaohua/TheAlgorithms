#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

import itertools

def main():
    def power(n: int) -> int:
        assert n > 0
        p = 1
        for i in range(1, n + 1):
            p *= i
        return p

    # 读取输入
    # 生成所有的排列
    # 找到第k个排列
    n = int(input())
    assert 1 <= n <= 9
    k = int(input())
    power_n = power(n)
    assert 1 <= k <= power_n

    # 生成所有的数字
    nums = tuple(i for i in range(1, n + 1))
    # 生成排列的迭代器
    it = itertools.permutations(nums)
    # 注意, k是从1开始计数的
    for i in range(k):
        ans = next(it)

    # 打印结果
    s = "".join(map(str, ans))
    print(s)

if __name__ == "__main__":
    main()
