# 加一 Plus One

[问题描述](https://leetcode.com/problems/plus-one)

这个题目比较简单, 就是用数组来模拟任意精度的整数相加, 数组中的每个元素代表了十进制整数值中的每个位.
这个问题唯一要操作的是进位置 carry, 从右向左依次将数组中的每个元素与 carry 相加, 然后把结果
写回到该元素, 如果产生了新的进位, 就把它写回到 carry.

```rust
{{#include src/main.rs:5:10}}
```