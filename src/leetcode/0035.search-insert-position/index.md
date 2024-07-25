# 0035. 搜索插入位置 Search Insert Position

[问题描述](https://leetcode.com/problems/search-insert-position)

这个使用二分查找法.

```rust
{{#include src/main.rs:5:38}}
```

另外, 标准库中的 `slice::binary_search()` 函数, 也可以查找目标元素 `target` 的期望位置:

```rust
{{#include src/main.rs:40:46}}
```