# 快慢型双指针

即访问数组(或者链表)时, 使用两个索引(或指针), 而不是通常的一个索引.

这两个指针, 分别称为快指针 (fast pointer) 和慢指针 (slow pointer).

快指针, 用于从0到n依次遍历整个数组, 即每次循环 `fast += 1`, 访问下一个元素

慢指针用于让数组中元素实现某个特定条件最高位索引,
比如件条可以是[元素不重复](../leetcode/0026.remove-duplicates-from-sorted-array/index.md).
当满足条件后, 要对数组做什么样的调整, 比如交换元素或者移除元素, 然后 `slow += 1`, 移动慢指针指向下一个元素.
当条件不满足时, 慢指针不动.

具体的过程看下图:

![fast-slow](assets/fast-slow.svg)

## 相关问题

- [0026. 删除有序数组中的重复项 Remove Duplicates from Sorted Array](../leetcode/0026.remove-duplicates-from-sorted-array/index.md)
