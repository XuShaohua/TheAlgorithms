#!/usr/bin/env python3

def solution(nums):
    # 遍历求和
    # x + n + x = sum
    nums_sum = sum(nums)

    left_sum = 0
    for i in range(len(nums)):
        if left_sum * 2 + nums[i] == nums_sum:
            return i
        left_sum += nums[i]
    return -1

def main():
    nums = [1, 7, 3, 6, 5, 6]
    ret = solution(nums)
    assert ret == 3, "Invalid value, expected 3"

    nums = [1, 2, 3]
    ret = solution(nums)
    assert ret == -1, "Invalid value, expected -1"

    nums = [2, 1, -1]
    ret = solution(nums)
    assert ret == 0, "Invalid value, expected 0"

if __name__ == "__main__":
    main()
