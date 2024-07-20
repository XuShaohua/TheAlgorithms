# 旋转数组 Rotate Array

[问题描述](https://leetcode.com/problems/rotate-array/)

## 三次反转法

操作过程如下:

1. 将 `arr[k..n]` 进行反转
2. 将 `arr[0..k]` 进行反转
3. 将 `arr[..]` 进行反转

这个方法是在原地操作的, 其时间复杂度是 `O(n)`, 空间复杂度是 `O(1)`.

```rust
{{#include src/main.rs:31:49}}
```

## 使用时时数组

操作过程如下:

1. 创建辅助数组
2. 将 `arr[(len - k)..]` 复制到辅助数组
3. 将 `arr[..(len - k)]` 复制到辅助数组
4. 将辅助数组中的内容与目标数组交换, 通过 `mem::swap()`

这个方法不是在原地操作的, 其时间复杂度是 `O(n)`, 空间复杂度是 `O(n)`.

```rust
{{#include src/main.rs:51:66}}
```

## 参考

- [旋转数组](../../array/rotate.md)
