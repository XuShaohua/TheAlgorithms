# x 的平方根 Sqrt(x)

[问题描述](https://leetcode.com/problems/sqrtx)

这个比较适合二分查找法, 比较容易理解.

```rust
{{#include src/main.rs:7:32}}
```

下面方法是二分查找法的另一个写法, 其边界值的判定条件跟上述方法有所差别:

```rust
{{#include src/main.rs:34:50}}
```