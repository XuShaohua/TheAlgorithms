#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

import sys

def main():
    # 解析目标数组的长度 k
    k = int(input())
    num_arrays = int(input())
    # 解析所有的数组
    all_arrays = []
    for line in sys.stdin.readlines():
        all_arrays.append(list(map(int, line.split(","))))
    #print("all_arrays:", all_arrays)
    assert len(all_arrays) == num_arrays

    # 依次遍历所有数组, 取出 k 个元素
    # 如果不足k 个元素, 就取出剩下的所有元素, 然后与下面一个数组拼够 k 个元素
    # 直到所有数组里的元素都被取出
    i = 0
    ans = []
    nums_taken = k
    while True:
        current_array = all_arrays[i]
        if len(current_array) > nums_taken:
            # 取出足够的元素
            ans.extend(current_array[:nums_taken])
            all_arrays[i] = current_array[nums_taken:]
            # 去下一个数组
            i = (i + 1) % len(all_arrays)
            nums_taken = k
        else:
            # 还有几个元素等下一轮获取
            nums_taken -= len(current_array)
            # 取出所有元素
            ans.extend(current_array[:])
            # 将当前数组移除
            all_arrays.pop(i)
            if all_arrays:
                i = i % len(all_arrays)
            else:
                break

    for num in ans[:-1]:
        print(num, ",", sep="", end="")
    if ans:
        print(ans[-1])

if __name__ == "__main__":
    main()
