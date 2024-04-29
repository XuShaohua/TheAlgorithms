# 0075. 颜色分类 Sort Colors

[问题描述](../problems/0075.sort-colors/content.html)

## 靠拢型双指针

靠拢型双指针是一种常用方法, 这个解法是它的变体, 叫[DNF](../../two-pointers/close-up.md).

```rust
{{#include src/main.rs:7:40 }}
```

## 排序法

各种常见的排序算法都可以, 比如括入排序, 选择排序. 因为这毕竟是一个排序题.

```rust
{{#include src/main.rs:47:56 }}
```