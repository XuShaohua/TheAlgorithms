# 0154. 寻找旋转排序数组中的最小值 II Find Minimum in Rotated Sorted Array II

[问题描述](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/)

这个问题是 [0153](../0153.find-minimum-in-rotated-sorted-array/index.md) 的扩展.
这个表面上看起来可以直用用二分查找法, 但因为可以有相同值的元素存在, 它把问题复杂化了.

## 暴力法 Brute force

遍历数组, 计算最小值:

```rust
{{#include src/main.rs:5:8}}
```

时间复杂度是 `O(n)`.

## 二分查找法

这个要对二分查找法的命中条件做一些改进. 因为有重复元素的存在, 我们在某些条件下无法确定元素的顺序,
但可以只对有明确顺序的情况使用二分查找:

- 如果 `nums[middle] < nums[right]` 则最小值不在右侧部分
- 如果 `nums[middle] > nums[right]` 则最小值在右侧部分
- 其它情况, 一次将 `left` 右移一步, 将 `right` 左移一步, 渐渐靠近

```rust
{{#include src/main.rs:10:40}}
```

该算法的时间复杂度是 `O(log(n))`.