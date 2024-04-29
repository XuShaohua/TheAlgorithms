# 0026. 删除有序数组中的重复项 Remove Duplicates from Sorted Array

[问题描述](../problems/0026.remove-duplicates-from-sorted-array/content.html)

要注意的一点是, 这个数组已经是有序的了.

## 方法1, 快慢型双指针法

快慢型双指针法的详细说明, 可以参考[这里](../../two-pointers/fast-slow.md).

![two-pointers](../../two-pointers/assets/fast-slow.svg)

```rust
{{#include src/main.rs:24:38 }}
```

## 方法2, 使用 Vec 自带的去重方法

`Vec::dedup()` 就是用来去重的, 如果它已经是排序的了, 可以去掉所有重复元素.

```rust
{{#include src/main.rs:40:44 }}
```

## 相关问题

- [0080. 删除排序数组中的重复项 II Remove Duplicates from Sorted Array II](../0080.remove-duplicates-from-sorted-array-ii/index.md)