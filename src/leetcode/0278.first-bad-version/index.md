# 第一个错误的版本 First Bad Version

[问题描述](https://leetcode.com/problems/)

这个问题是很简单的二分查找问题.

要查找顺序序列中的一个分界值, 分界值的左侧都是正常版本, 而右侧都是有问题的版本.

要注意二分的边界情况:

```rust
{{#include src/main.rs:19:36}}
```

时间复杂度是 `O(log(n))`, 空间复杂度是 `O(1)`.