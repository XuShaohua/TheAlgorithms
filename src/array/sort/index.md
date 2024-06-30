# 排序 Sorting

所谓的数组排序, 就是按照相同的顺序将数组中所有元素依次排好.

排序算法的特点:

- 稳定排序 (stable sort): 排序相同值的元素时, 会保持它们在数组中的原有顺序
- 不稳定排序 (unstable sort): 排序相同值的元素时, 会打乱它们在数组中的原有顺序
- adaptive sort: 能利用输入数组的已有顺序, 如果输入的是基本已排序好的数组, 排序效率更高
- non-adaptive sort: 即使输入的数组已基本有序, 仍然需要固定的步骤完成排序工作, 所以排序效率较低
- 原地排序 (in-place sort): 不需要额外的内存空间, 只在原先的数组上进行排序; 当然, 在交换元素时用到的一个临时变量不算在内

## 参考

- [Sorting algorithm](https://en.wikipedia.org/wiki/Sorting_algorithm)