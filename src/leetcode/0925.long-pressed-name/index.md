# 0925. 长按键入 Long Pressed Name

[问题描述](https://leetcode.com/problems/long-pressed-name)

## 并行双指针

典型的[并行双指针](../../two-pointers/parallel.md)问题.

`typed` 数组与 `name` 数组不一致的情况有三种:

- 长按了, 通过 `name[index1 - 1] == typed[index2]` 来确认
- 按错了, `name[index1] != typed[index2]`
- 少按了, 在最后, `index2 != len2`

![parallel two-pointer](../../two-pointers/assets/parallel.svg)

要注意的是:

- 如果已经遍历完 `name` 数组, 要检查 `typed` 数组有没有遍历完
- 返回 `true` 的条件就是两个数组都满足条件的情况下遍历完

```rust
{{#include src/main.rs:5:44 }}
```