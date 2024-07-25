# 两数之和 II - 输入有序数组 Two Sum II - Input Array Is Sorted

[问题描述](https://leetcode.com/problems/two-sum-ii-input-array-is-sorted)

## 靠拢型双指针

典型的双指针问题, 就不过多介绍了, 详细的分析看[这里](../../two-pointers/close-up.md).

```rust
{{#include src/main.rs:7:27 }}
```

## 二分查找法

因为数组已经是排好序的了, 也可以先遍历数组, 并用二分查找法找到其和为 `target` 的对应的元素.

要注意的是, 二分查找法对于有很多的重复元素时, 需要做一下优化, 我们在这里并没有手动实现二分查找法,
而只是调用了 Rust 的 `slice::binary_search()` 方法.

```rust
{{#include src/main.rs:29:40 }}
```