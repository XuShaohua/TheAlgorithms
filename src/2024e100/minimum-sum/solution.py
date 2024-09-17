#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

import sys

def main():
    # 提取 array1
    part1 = input().split()
    size1 = int(part1[0])
    array1 = list(map(int, part1[1:]))
    assert len(array1) == size1
    assert 0 < size1 <= 100

    # 提取 array2
    part2 = input().split()
    size2 = int(part2[0])
    array2 = list(map(int, part2[1:]))
    assert len(array2) == size2
    assert 0 < size2 <= 100

    # 提取整数 k
    k = int(input())

    # Brute force
    # 取得所有的数对
    all_pairs = []
    for num1 in array1:
        for num2 in array2:
            all_pairs.append(num1 + num2)

    # 以升序排序数对
    all_pairs.sort()

    # 取出前 k 对数对的和, 并计算其总和
    ans = sum(all_pairs[:k])
    print(ans)

if __name__ == "__main__":
    main()
