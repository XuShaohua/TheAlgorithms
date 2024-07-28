# 3028. 边界上的蚂蚁 Ant on the Boundary

[问题描述](https://leetcode.com/problems/ant-on-the-boundary)

这是一个经典的前缀和数组问题.

解题思路如下:

1. 先遍历 `nums` 数组, 构造出前缀和数组 `positions`
2. 遍历 `positions` 数组, 统计里面数值 `0` 出现的次数

该算法的实现如下:

```rust
{{#include src/main.rs:5:15}}
```

该算法的时间复杂度是 `O(n)`, 空间复杂度是 `O(n)`.

上面的算法可以做一些简化, 因为是一次性获取结果, 其是是不需要将中间值存入到 `positions` 数组的,
在遍历 `nums` 数组时可以直接更新 `zeros_count` 计数值. 算法实现如下:

```rust
{{#include src/main.rs:17:31}}
```

该算法的时间复杂度是 `O(n)`, 空间复杂度是 `O(1)`.