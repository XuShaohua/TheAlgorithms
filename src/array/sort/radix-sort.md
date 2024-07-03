# 基数排序 Radix Sort

前文介绍的几种排序算法都是基于比较元素之间的关系(comparison based), 这对于像字符串或者其它自定义数据类型也是有效的,
只需要实现 `PartialOrd` 即可, 具有通用性.

接下来两节介绍的排序算法都是基于元素的数值大小, 而不是比较关系 (non-comparison based), 它们只适合比较数值元素.