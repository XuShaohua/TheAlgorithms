# 1893. 检查是否区域内所有整数都被覆盖 Check if All the Integers in a Range Are Covered

[问题描述](https://leetcode.com/problems/check-if-all-the-integers-in-a-range-are-covered)

这个问题有两个思路可以处理.

## 集合 Set

使用集合来存储区间上的每个点.

步骤如下:

1. 创建集合
2. 遍历 `ranges` 数组, 将每个范围上的所有点位都存储到集合中
3. 遍历 `[left..=right]` 区间上的所有点位, 查看它们是否都在集合中

```rust
{{#include src/main.rs:5:24}}
```

该算法:

- 时间复杂度是 `O(n m)`, 其中 `n` 是范围的个数, 而 `m` 是最大的范围区间
- 空间复杂度是 `O(n)`, 其中 `n` 是范围包含的所有点位个数

## 合并区间 Merge intervals

这个方法用于计算区间重叠很方便.

其步骤如下:

1. 先对 `ranges` 数组进行排序, 依照范围的起始点
2. 构造合并区间 `intervals`
    1. 初始化区间值 start = 0, end = 0
    2. 遍历 ranges, 并判断当前区间是否能跟区间值 `[start..=end]` 拼接在一起, 判定条件是 `range[0] <= end + 1`
    3. 如果可以, 就只需要移动区间的终点值, `end = end.max(range[1])`
    4. 如果不行, 就先将当前区间 `[start..=end]` 加入到 `intervals`, 然后更新 `[start..=end]` 区间
3. 查找 `[left..=right]` 区间是否在 `intervals` 内

```rust
{{#include src/main.rs:26:61}}
```

该算法:

- 时间复杂度是 `O(n log(n))`, n 是 `ranges` 中的区间个数
- 空间复杂度是 `O(n)`, n 是 `ranges` 内不连接的区间个数