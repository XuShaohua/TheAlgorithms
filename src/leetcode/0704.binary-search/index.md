# 二分查找 Binary Search

[问题描述](https://leetcode.com/problems/binary-search)

这个题目是最基础的二分查找法.

```rust
{{#include src/main.rs:5:36}}
```

## 二分查找法之排除法

这种是二分查找法的变体, 与上面的方法不同在于边界值的范围, 这里使用的是 `左闭右开区间`.

```rust
{{#include src/main.rs:38:68}}
```

## 标准库中自带的二分查找法实现

标准库的 `slice::binary_search()` 及其变体函数, 就实现了经典的二分查找法, 性能比较好:

```rust
{{#include src/main.rs:70:77}}
```