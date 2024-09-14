#!/usr/bin/env python3

# 遍历
def solution(nums):
    max_count = 0
    current_count = 0
    for i in range(len(nums)):
        if nums[i] == 1:
            current_count += 1
        else:
            max_count = max(max_count, current_count)
            current_count = 0

    # Check the last value
    max_count = max(max_count, current_count)
    return max_count

def main():
    nums = [1,1,0,1,1,1]
    ret = solution(nums)
    assert ret == 3, "Invalid value, expected 3"

    nums = [1,0,1,1,0,1]
    ret = solution(nums)
    assert ret == 2, "Invalid value, expected 2"

if __name__ == "__main__":
    main()
