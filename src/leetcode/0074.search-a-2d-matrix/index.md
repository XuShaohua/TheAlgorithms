# 0074. 搜索二维矩阵 Search a 2D Matrix

[问题描述](https://leetcode.com/problems/search-a-2d-matrix)

这个题目适合使用二分查找法. 但在解题之外, 我们先试试暴力法.

## 暴力法

把所有的元素拷贝一遍到新的数组中, 然后使用二分查找法:

```rust
{{#include src/main.rs:7:16}}
```

真正运行时, 这个方法其实并不算太慢. 时间复杂度是 `O(m n)`, 空间复杂度是 `O(m n)`.

## 二分查找法

考虑题目中的条件:

- 每行数据是递增的
- 每列数据也是递增的

根据上面的条件, 我们的解决思路是:

1. 先使用二分查找法遍列第一列, 找到 `target` 可能位于哪一行
2. 然后再利用二分查找法遍历该行, 确定 `target` 是否存在于当前行

但要注意一下细节:

1. 在查找行时, 我们使用 `while top < bottom` 的条件判断, 该循环终止的条件是 `left == right`
2. 而且计算 middle 值时, 使用的是 `let middle = left + (right - left + 1) / 2`, 这样的话,
   中间值会偏下一位, 但因为之后的条件, `bottom = middle - 1`, 这样才不会进行死循环
3. 在查找行数据时, 我们使用 `while left <= right` 的条件, 该循环的终止条件是, 找到了 `target`,
   或者 `left > right` 没有找到

代码实现如下:

```rust
{{#include src/main.rs:18:64}}
```

算法的时间复杂度是 `O(log(m n))`, 空间复杂度是 `O(1)`.