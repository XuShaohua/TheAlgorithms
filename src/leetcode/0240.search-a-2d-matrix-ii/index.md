# 0240. Search a 2D Matrix II Search a 2D Matrix II

[问题描述](https://leetcode.com/problems/search-a-2d-matrix-ii)

这个题目中, 二维数组的行和列都是有序的.

## 暴力法

首先考虑的就是直接将二维数组中的元素排序, 我们通过将所有数值移到同一个数组中, 然后再给它排序,
再利用二分查找法, 这样就比较方便了.

```rust
{{#include src/main.rs:8:19}}
```

算法的时间复杂度是 `O(n log(n))`, 空间复杂度是 `O(n)`, 其中 `n` 是二维数组中的所有元素个数.

另外, 也可以直接使用 HashSet 来存储所有的整数, 它自动排序.

```rust
{{#include src/main.rs:6:6}}

{{#include src/main.rs:21:29}}
```

## 对角线法

对于有序的二维数组, 如果我们从数组的右上角开始查找 `target`, 有三种情况:

1. 创建 `row_index` 和 `col_index`, 用于定位二维数组中的元素; 并在开始时将它定位到数组右上角
2. 当前元素等于 `target`, 就直接返回
3. 当前元素比 `target` 大, 那我们就将 `col_index -= 1`, 向左侧继续查找
4. 当前元素比 `target` 小, 那我们就将 `row_index += 1`, 去下一行继续查找
5. 直找我们遍历到二维数组的左下角终止循环, 此时 `row_index = matrix_rows - 1`, `col_index = 0`

```rust
{{#include src/main.rs:5:5}}

{{#include src/main.rs:31:64}}
```

TODO: 更新时间复杂度

## 二分查找法

这种方法要复杂一些.