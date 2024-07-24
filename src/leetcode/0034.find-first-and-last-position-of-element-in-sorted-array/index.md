# 在排序数组中查找元素的第一个和最后一个位置 Find First and Last Position of Element in Sorted Array

[问题描述](https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array)

这个问题适合用二分查找法.

## 基本的二分查找法

分两步来解这个问题:

1. 使用二分查找法确定 `target` 存在于数组中, 如果不存在就直接返回; 如果存在, 就得到了它的索引值 `middle`
2. 然后使用线性查找法, 分别从 `middle` 开始向数组的左右两端查找与 `target` 元素相等的

最后的实现代码如下所示:

```rust
{{#include src/main.rs:5:48}}
```

## 使用 slice::binary_search

这个是对上述方法的简化, 使用标准库中自带的二分查找算法:

```rust
{{#include src/main.rs:50:77}}
```

## 两次二分查找

上面的算法, 对于查找与 `target` 相等的元素时, 使用了两次线性查找, 用于确定左右边界.
这一步时间复杂度是 `O(k)`, 其中的 `k` 是与 `target` 相等的元素的个数.

我们可以使用两次二分查找法, 取代线性查找法, 直接确定左右边界.

```rust
{{#include src/main.rs:79:146}}
```