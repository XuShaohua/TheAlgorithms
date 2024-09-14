#!/usr/bin/env python3

# Prefix sum array
def solution(nums):
    if len(nums) == 0:
        return []

    prefix_product = [1]
    for i in range(1, len(nums)):
        prefix_product.append(nums[i - 1] * prefix_product[i - 1])

    postfix_product = [1]
    length = len(nums)
    for i in range(1, len(nums)):
        postfix_product.append(nums[length - i] * postfix_product[i - 1])
    postfix_product = list(reversed(postfix_product))

    ans = []
    for i in range(len(nums)):
        ans.append(prefix_product[i] * postfix_product[i])
    return ans

# Prefix sum array
# 空间复杂度是 O(1)
def solution2(nums):
    nums_len = len(nums)
    ans = [1 for _ in range(len(nums))]

    # forward
    product = 1
    for i in range(nums_len):
        ans[i] *= product
        product *= nums[i]

    # backward
    product = 1
    for i in range(nums_len - 1, -1, -1):
        ans[i] *= product
        product *= nums[i]

    return ans

def main():
    nums = [1,2,3,4]
    ret = solution2(nums)
    assert ret == [24,12,8,6], "Invalid ret list"

    nums = [-1,1,0,-3,3]
    ret = solution2(nums)
    assert ret == [0,0,9,0,0], "Invalid ret list2"

if __name__ == "__main__":
    main()
