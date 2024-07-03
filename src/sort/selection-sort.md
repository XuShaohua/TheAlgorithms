# 选择排序 Selection Sort

选择排序的逻辑很简单, 将数组分成两部分:

- 左侧部分是排序好的, 按顺序放着较小的元素
- 右侧部分是未排序的, 放着较大的元素

## 选择排序的步骤

1. 遍历数组, 找到最小的元素, 让它与最左侧元素交换
2. 遍历数组中剩下的元素, 找到最小的元素, 让它与最左侧第二位元素交换
3. 重复上面的步骤, 直到所有元素都有序排列

我们以 `arr = [9, 4, 1, 7];` 为例进行演示:

首先找到最小的元素 `1`, 把它与最左侧元素相交换:

![selection sort pass 1](assets/selection-sort-pass1.svg)

第二阶段, 找到剩下元素中最小的元素 `4`, 把它与左侧第二位相交换:

![selection sort pass 2](assets/selection-sort-pass2.svg)

第三阶段, 找到最小的元素 `7`, 把它与左侧第三个元素相交换:

![selection sort pass 3](assets/selection-sort-pass3.svg)

到达了数组的最右侧, 所有元素都已排好序.

## 选择排序的代码实现

```rust
{{#include assets/selection_sort.rs:5:28}}
```

## 递归实现选择排序

以上代码是用的迭代方式实现的选择排序, 接下来我们以递归的方式重新实现它.

```rust
{{#include assets/selection_sort.rs:30:63}}
```

## 优化选择排序

默认实现的选择排序, 在每次循环时会找到最小的元素, 然后把它放在数组的左侧部分.
每次循环时, 我们可以同时找到最大的元素, 然后把它放在数组的右侧部分. 这样的话, 每个循环就可以同时找到最小和最大的元素.

```rust
{{#include assets/selection_sort.rs:65:120 }}
```

## 选择排序支持稳定排序

默认实现的选择排序算法, 是将最小元素交换到它的目标位置, 这样的话移动元素的次数很少, 但是是不稳定排序.
为了实现稳定排序, 我们可以插入排序的方式, 将最小元素插入到目标位置, 然后将其它元素向右移动一个位置,
尽管这样一来性能比较差.

```rust
{{#include assets/selection_sort.rs:65:120 }}
```

## 选择排序的特点

1. 即使数组中的元素基本排序好, 也需要遍历所有元素并比较大小, 这种情况下效率较低, 依然需要 `N^2 / 2` 次比较操作
2. `N^2 / 2` 次比较以及 N 次交换, 平均时间复杂度是 `O(NlogN)`, 空间复杂度是 `O(1)`
3. 在所有排序算法中, 选择排序移动元素的次数最少, 每个元素最多只移动一次, 就可以移到最终位置;
   这个算法比较适合那种比较元素时的成本低, 但移动元素成本比较高的情况 (比如, 移动文件中的内容)
4. 选择排序是原地排序 (in-place sort)
5. 选择排序是 in-adaptive sort
6. 默认实现的选择排序算法是不稳定排序 (unstable), 但优化后的算法可以实现稳定排序 (stable sort)