# 快速排序 Quicksort

## 快速排序的特点

- 最好情况的时间复杂度是 `O(n log(n))`, 最差情况的时间复杂度是 `O(n^2)`, 平均情况下的时间复杂度是 `O(n log(n))`
- 最好情况下的空间复杂度是 `O(log(n))`, 最差情况下的空间复杂度是 `O(n)`, 因为是递归调用
- 不是稳定排序 (stable sort)
- 比归并排序 (merge sort) 要快, 不需要一个额外的数组来保存中间值

## 当元素较少时, 使用插入排序

## 选择中位数 (median) 作为基准值 pivot

## 随机选择一个元素作为基准值 pivot

## 原地分隔数组 (in-place partition)

## 参考

- [Quicksort](https://en.wikipedia.org/wiki/Quicksort)