# 前缀和数组 Prefix Sum Array

什么是前缀和数组? `prefix_sum_array[i] = prefix_sum_array[i - 1] + arr[i]`,

上面的定义不好理解的话, 我们再看一下例子, 原数组是 `arr[] = [1, 2, 3, 4, 5];`, 则前缀和数组就是:
`prefix_sum = [1, 3, 6, 10, 15];`.

前缀和数组的算法倒是蛮简单, 如下所示:

```rust
{{#include assets/prefix_sum.rs:7:23}}
```

该算法的时间复杂度是 `O(n)`, 空间复杂度是 `O(n)`.

这种算法思想主要是用于缓存某些需要频繁计算的过程, 以空间换取时间.

## 前缀和数组的应用

给定一个数组 `arr`, 计算 `arr[l]` 与 `arr[r]` 之间的所有元素之和.

频繁的计算数组的部分连续项之和时, 每次计算都要从头算. 我们可以用前缀和数组, 
这样每次计算时可以立即得到结果.

有下面的公式:

```text
arr[left..=right].sum() = prefix_sum_array[right] - prefix_sum_array[left - 1];
```

算法实现如下:

```rust
{{#include assets/sum-of-slice.rs:5:}}
```
