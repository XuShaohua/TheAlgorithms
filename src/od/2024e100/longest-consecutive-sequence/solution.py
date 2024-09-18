#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

# 滑动窗口
def main():
    nums = list(map(int, input().split(",")))
    expected_sum = int(input())
    assert 1 <= len(nums) <= 200
    assert 1 <= expected_sum

    # 最长子序的长度
    subarray_max_len = -1
    # 用于控制窗口左侧两侧的位置
    left = 0
    right = 0
    # 计算当前子序列的和
    current_sum = 0

    while left <= right and right < len(nums):
        if current_sum < expected_sum:
            # 子序列的和太小, 将窗口右侧向右移
            current_sum += nums[right]
            right += 1
        elif current_sum == expected_sum:
            # 和相等, 计算当前的子序列长度
            current_length = right - left
            subarray_max_len = max(subarray_max_len, current_length)
            current_sum += nums[right]
            right += 1
        else:
            # 子序列的和太大, 将窗口左侧向右移
            current_sum -= nums[left]
            left += 1

    print(subarray_max_len)


if __name__ == "__main__":
    main()
