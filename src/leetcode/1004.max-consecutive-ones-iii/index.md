
# 1004. 最大连续1的个数 III Max Consecutive Ones III

[问题描述](https://leetcode.com/problems/max-consecutive-ones-iii)

## 滑动窗口

这个问题跟之前的胡杨林补种一样, 用滑动窗口来解决:
- 窗口右侧经过0时, 计数加1
- 当窗口区间内的0的个数大于k 时, 把窗口左侧向右移, 直到窗口范围内的0的个数不大于k
- 然后更新最大的连续为1的个数


```rust
{{#include src/main.rs:5:37}}
```
