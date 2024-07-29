# 2848. 与车相交的点 Points That Intersect With Cars

[问题描述](https://leetcode.com/problems/points-that-intersect-with-cars)

这个问题的解法就比较多了.

## 哈稀表

确切来说是 HashSet, 我们用集合来统计每辆车占用的点位, 最后计算集合中点的个数就行.

操作步骤如下:

1. 创建统计用的集合
2. 遍历所有车辆, 将每个车辆占用的点位区间上的所有点, 都加入到集合中
3. 集合的长度就是所有的点位数

```rust
{{#include src/main.rs:7:18}}
```

该算法的时间复杂度是 `O(n)`, 空间复杂度是 `O(n)`.

## Bitset

这个是对上述方法的优化, 因为给定的点数比较少, 我们也可以直接使用 Bitset 来统计点位数,
为了实现简单, 我们直接使用 `[false; 101]` 来作为 bitset.

```rust
{{#include src/main.rs:20:31}}
```

该算法的时间复杂度是 `O(n)`, 空间复杂度是 `O(n)`.

## 合并区间 Merge intervals

这个方法是 [0056. Merge intervals](../0056.merge-intervals/index.md) 的解法.

因为给定的区间是无序的, 我们先以每个区间的起始点来对它进行排序, 之后再统计.

步骤如下:

1. 对所有车辆所占用的区间的起始点进行排序
2. 创建一个动态数组 `intervals`, 用来存储不重叠的区间
3. 遍历 `nums` 中的所有数对, 然后合并有重叠的区间, 并将不重叠的区间存储到 `intervals` 数组中
    1. 如果有重叠, 只需要更新区间的终点值即可
    2. 如果没有重叠, 则需要把之间的区间存到 `intervals` 数组, 并同时更新起点和重点
4. 遍历 `intervals` 数组, 统计所有不重叠区间占用的点数

```rust
{{#include src/main.rs:51:80}}
```