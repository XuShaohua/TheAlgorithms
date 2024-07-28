# 1732. 找到最高海拔 Find the Highest Altitude

[问题描述](https://leetcode.com/problems/find-the-highest-altitude)

这个问题可以使用前缀和 prefix sum 的算法, 但是是逆着来的, 给出的是前缀和数组, 我们需要得到
原先的数组.

```rust
{{#include src/main.rs:5:15}}
```

时间复杂度是 `O(n)`, 空间复杂度是 `O(1)`.