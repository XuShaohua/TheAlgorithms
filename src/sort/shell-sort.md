# 希尔排序 Shell Sort

接下面的几节介绍几个不常用的排序算法.

本节介绍的希尔排序 (shell sort) 是插入排序的 (insertion sort) 的变体.

插入排序的一个问题是, 将元素 `k` 移动到左侧排序好的数组中的位置时, 通常还要移动元素 `k` 左侧的元素,
而移动元素的成本比较高. 希尔排序对这个过程做了优化, 以减少移动元素的次数.

希尔排序将数组拆解成由 `h` 个元素组成的小数组, 依次降低h间隔的值, 直到其为1, 这样就减少了元素交换的次数.

## 希尔排序的步骤

1. 初始化间隔值 `h = len / 3`
2. 使用插入排序法, 将 `arr[h..]` 与 `arr[..h]` 间的元素进行排序, 使用插入排序法, 但两个待比较的元素的间隔是 `h`,
   而不是默认的 `1`, 这一步很重要, 它有助于减少元素的移动次数
3. 减少间隔值, `h /= 3`, 重复上面的步骤, 直到最后一个循环 `h = 1`

这里的 `h` 值是由大到小变化的, 就是说, 每次移动的步长是h, 就是为了减少元素被移动的次数.
当 h = 1 时, 整个序列就完成排序了.

## 希尔排序的实现

```rust
{{#include assets/shell_sort.rs:5:34}}
```

## 希尔排序的特点

- 最差情况下的时间复杂度 `O(n^2)`, 空间复杂度是 `O(1)`
- 最好情竞下的时间复杂度是 `Ω(n log(n))`
- 比插入排序快
- 与插入排序不同的时, 希尔排序适合大中型的数组, 对于任意顺序的数组也有效