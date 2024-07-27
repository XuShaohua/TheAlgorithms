# 0303. 区域和检索 - 数组不可变 Range Sum Query - Immutable

[问题描述](https://leetcode.com/problems/range-sum-query-immutable)

这个问题就是要使用 [前缀和数组 prefix sum array](../../array/prefix-sum-array.md), 详细的内容可以参阅前文的介绍.

实现方法也很直接:

```rust
{{#include src/main.rs:5:34}}
```

算法的时间复杂度是 `O(1)`, 空间复杂度是 `O(n)`.

