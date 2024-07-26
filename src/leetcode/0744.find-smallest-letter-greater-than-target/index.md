# 0744. 寻找比目标字母大的最小字母 Find Smallest Letter Greater Than Target

[问题描述](https://leetcode.com/problems/find-smallest-letter-greater-than-target/description/)

这个可以用二分查找法, 它要找到比目标元素 `target` 大的第一个元素.

## 二分查找法

思路是排除法, 因为左侧的元素较小, 优先排除左侧部分. 步骤如下:

1. 创建两个指针, 分别指向数组的左右两侧
2. 开始二分查找, 计算中间节点 middle 的值, 并判断:
    1. 如果 `letters[middle] >= target`, 说明 `[left..middle]` 区间的字符较小, 将 left 指针向右移,
       令 `left = middle + 1`
    2. 否则, 说明 `[middle..right]` 的字符都是比 `target` 大的, 将 right 指针左移, 令 `right = middle`
3. 当 `left == right` 时, 终止循环, 确认一下 left 位置的字符是不是目标

```rust
{{#include src/main.rs:5:28}}
```

时间复杂度是 `O(log(n))`.