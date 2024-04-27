# 0011. 盛最多水的容器 Container With Most Water

[问题描述](../problems/0011.container-with-most-water/content.html)

这是一个典型的[靠拢型双指针问题](../../two-pointers/close-up.md).

```rust
{{#include src/main.rs:5:31 }}
```

## 针对靠拢型双指针做一点优化

上面提到的代码实现是经典的写法, 但这里, 还可以针对里面的实现细节做一些优化.
具体来说就是, 判断当前指针的下个元素的值如果大不于当前值, 那就可以跳过下个元素,
因为它的面积值一定比当前根据两个指针计算的面积值要小.

```rust
{{#include src/main.rs:33:68 }}
```