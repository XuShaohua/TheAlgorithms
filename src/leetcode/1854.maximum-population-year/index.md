# 1854. 人口最多的年份 Maximum Population Year

[问题描述](https://leetcode.com/problems/maximum-population-year)

这是一个计数的问题, 首先想到的就是字典计数.

## BTreeMap

计数步骤如下:

1. 创建字典, 为了找到相同生存人数的最小年份, 我们使用 BTreeMap 来保证年份的有序.
2. 遍历 `logs` 中的所有记录, 并遍历每条记录的出生到去世间的所有年份, 将本年度加入到字典中
3. 遍历有序字典, 找到有最大生存人数的那一年

```rust
{{#include src/main.rs:5:29}}
```

这个算法的时间复杂度是 `O(n * m)`, 其中 `n` 是人数, `m` 是生存的年份.
空间复杂度是 `O(n)`, 其中 `n` 是 `logs` 的年份范围.

## 计数数组

因为年份跨度比较小, 只有100年, 我们可以用栈上的数组来代替 BTreeMap, 其它步骤没有太多变化.

```rust
{{#include src/main.rs:31:54}}
```

这个算法的时间复杂度是 `O(n * m)`, 其中 `n` 是人数, `m` 是生存的年份.
空间复杂度是 `O(n)`, 其中 `n` 是 `logs` 的年份范围.

## 前缀和

这个有些不好考虑, 在构造前缀和数组之前, 我们先构造一个辅助数组. 整个解决步骤如下:

1. 创建 `alive` 辅助数组
2. 遍历 `logs` 数组
    1. 当一个人出生时, 将 alive 中出生所在年份计数加1, `alive[start_year] += 1`
    2. 当一个人去世时, 将 alive 中去世所在年份计数减1, `alive[end_year] -= 1`
3. 创建 `prefix_sum` 前缀和数组, 通过遍历 `alive` 数组
4. 最后遍历 `prefix_sum` 数组, 找到生存人数最多的那个年份

```rust
{{#include src/main.rs:56:89}}
```

这个算法的时间复杂度是 `O(n)`, 空间复杂度是 `O(n)`, 其中 `n` 是 `logs` 的年份范围.