# 1422. 分割字符串的最大得分 Maximum Score After Splitting a String

[问题描述](https://leetcode.com/problems/maximum-score-after-splitting-a-string)

## 暴力法

这个用于验证思路, 直接遍历数组, 计算每个分隔位点的数值, 求得其中最大的那个.

```rust
{{#include src/main.rs:5:27}}
```

这个算法的时间复杂度是 `O(n^2)`, 因为存在两层遍历数组的操作; 空间复杂度是 `O(1)`.

## 前缀和 Prefix sum

这个方法, 先计算好每个分隔位点之前的 `0` 的个数以及分隔位点之后的 `1` 的个数,
然后计算其中最大的那个组合.

具体步骤是:

1. 创建 `count_zeros` 数组, 用于存放从前向后字符 `0` 出现的次数之和
2. 从前向后遍历字符串, 统计出字符 `0` 出现的次数之和, 并存入 `count_zeros` 数组
3. 创建 `count_ones` 数组, 用于存放从后向前字符 `1` 出现的次数之和
4. 从后向前遍历字符串, 统计出字符 `1` 出现的次数之和, 并存入 `count_ones` 数组
5. 将 `count_ones` 数组反转, 方便后面的计算
6. 遍历计数数组 `(cont_ones, count_zeros)`, 找出最大的那个组合

```rust
{{#include src/main.rs:29:72}}
```

算法的时间复杂度是 `O(n)`, 空间复杂度是 `O(n)`, 因为引入了两个辅助数组.