# 0056. 合并区间 Merge Intervals

[问题描述](https://leetcode.com/problems/merge-intervals/)

这是一个排序题.

## 排序

因为给定的区间是无序的, 我们先以每个区间的起始点来对它进行排序, 之后再统计.

步骤如下:

1. 对 `intervals` 所有区间的起始点进行排序, `intervals.sort_by_key(|interval| interval[0])`
2. 创建一个动态数组 `ans`, 用来存储不重叠的区间
3. 遍历 `intervals` 中的所有数对, 然后合并有重叠的区间, 并将不重叠的区间存储到 `ans` 数组中
    1. 如果有重叠, 只需要更新区间的终点值即可
    2. 如果没有重叠, 则需要把之间的区间存到 `ans` 数组, 并同时更新起点和重点
4. 遍历 `ans` 数组, 统计所有不重叠区间占用的点数

```rust
{{#include src/main.rs:5:38}}
```

该算法的时间复杂度是 `O(n log(n))`, 空间复杂度是 `O(n)`.