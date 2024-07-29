# 2574. 左右元素和的差值 Left and Right Sum Differences

[问题描述](https://leetcode.com/problems/left-and-right-sum-differences)

这是一个简单的前缀和 prefix sum 问题.

## Prefix sum

处理思路如下:

1. 计算 `left_sum`, 从左到右遍历原数组, 并计算前缀和, `left_sum[i + 1] = left_sum[i] + nums[i]`
2. 计算 `right_sum` 从左到右遍历原数组, 并计算前缀和, `right_sum[i - 1] = right_sum[i] + nums[i]`
3. 计算遍历两个数组, 并计算 `(left_sum[i] - right_sum[i]).abs()`, 就得到了答案

```rust
{{#include src/main.rs:5:31}}
```

该算法的时间复杂度是 `O(n)`, 空间复杂度是 `O(n)`.