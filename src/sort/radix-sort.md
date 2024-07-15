# 基数排序 Radix Sort

前文介绍的几种排序算法都是基于比较元素之间的关系(comparison based), 这对于像字符串或者其它自定义数据类型也是有效的,
只需要实现 `PartialOrd` 即可, 具有通用性.

接下来两节介绍的排序算法都是基于元素的数值大小, 而不是比较关系 (non-comparison based), 它们只适合整数和定长的字符串.

基数排序是一种线性排序算法 (linear sorting). 它基于数值的整数每一位来排序, 直到整个数组变得有序.

根据排序的方向可以划分为最低位基数排序 (Least Significant Digit, LSD) 和是高位基数排序 (Most Significant Digit, MSD).