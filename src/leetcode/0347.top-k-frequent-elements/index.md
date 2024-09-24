# _`0347. 前 K 个高频元素 Top K Frequent Elements`_

[问题描述](https://leetcode.com/problems/top-k-frequent-elements)

## 优先级队列

因为要计算 top-k 的问题, 我们自然想到了优先级队列 Priority Queue.

- 同样是先用 hashmap 来统计各整数出现的频率
- 然后将它们存放到一个最大堆 heap 中, 每个元素是一个元组, 包含 (频率, 整数值) 两项, 以频率和整数值的降序来排列
- 之后将最大堆转换成数组, 并截取前 `k` 个元素即可

### Rust 实现

```rust
{{#include src/main.rs:5:35}}
```

### Python 实现

```python
{{#include main.py:6:}}
```

### C++ 实现

```cpp
{{#include main.cpp:5:35}}
```

## 手动对数值频率进行排序

上面的方法使用了最大堆来对数值出现的频率进行了排序, 但我们发现它并不是最快的算法.

- 我们可以将有相同频率的所有数值都存放在同一个数组中
- 然后用一个大的数组来存放, 以数值的频率作为元素的索引

代码实现如下:

```rust
{{#include src/main.rs:5:6}}
{{#include src/main.rs:62:93}}
```

## Quick Select

TODO

## Counting Sort

TODO