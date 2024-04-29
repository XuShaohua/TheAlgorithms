# 0532.数组中的数对 K-diff Pairs in an Array

[问题描述](../problems/0532.k-diff-pairs-in-an-array/content.html)

这是一个查找问题, 先思考一下处理查找问题的常用手段:

- 哈稀表或者 HashSet
- BitSet
- 排序后二分查找
- 排序后快慢型双指针遍历

## 哈稀表 Hash Table

使用哈稀表 HashMap 来统计整数值及其次数; 用集合 HashSet 来存放的有序数对, 并去掉重复的.

![hash-table](../0001.two-sum/assets/hash-table.svg)

这种方法可以支持无序数组.

```rust
{{#include src/main.rs:8:42 }}
```

## 二分查找法 Binary Search

基本的思路是:

- 先给数组排序
- 开始遍历数组
- 根据题目条件, 确定目标的元素的值; 使用二分查找法搜索目标元素
- 再根据要求, 判断目标元素是否合适, 比如两者索引值不能相同

```rust
{{#include src/main.rs:133:181 }}
```

## 快慢型双指针 Fast-Slow Two Pointers

[这个方法](../../two-pointers/fast-slow.md)的效率是最高的, 也最节省内存.

解决问题之前依然要先给数组排序.

![fast-slow two-pointers](../../two-pointers/assets/fast-slow.svg)

这个题目中, 双指针的命中条件是 `nums[fast] - nums[slow] = k;`, 只需要围绕这个核心条件做判断即可.

```rust
{{#include src/main.rs:183:233 }}
```

## 相关问题

- [0349. 两个数组的交集 Intersection of Two Arrays](../0349.intersection-of-two-arrays/index.md)
