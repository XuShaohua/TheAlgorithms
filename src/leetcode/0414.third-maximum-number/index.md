# 0414. 第三大的数 Third Maximum Number

[问题描述](https://leetcode.com/problems/two-sum)

## 方法1, Brute Force

这个方法比较直接, 就是先对数组排序, 并移除重复的元素.

因为有线性排序, 这个方法的时间复杂度是 `O(n log(n))`.

```rust
{{#include src/main.rs:5:16 }}
```

## 方法2, 

这个方法的时间复杂度是 `O(n)`.

```rust
{{#include src/main.rs:18:34 }}
```
