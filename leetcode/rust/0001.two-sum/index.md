# 0001. 两数之和 Two Sum

[问题描述](../problems/0001.two-sum/content.html)

## 方法1, Brute Force

这个方法比较直接, 就是遍历数组, 并遍历后面的每个元素, 求两者之和是否与 `target` 相等.

![brute-force](assets/brute-force.svg)

因为有两层遍历, 这个方法的时间复杂度是 `O(n^2)`.

```rust
{{#include src/main.rs:5:16 }}
```

## 方法2, 哈稀表

同样是需要遍历整个数组, 我们可以使用哈稀表缓存一下访问过的元素, 以加快查找元素的时间.
这个哈稀表用于记录元素值到它在数组中的索引值之间的关系.

但对于从哈稀表中查找, 我们可以进行一下优化, 即, 查找与当前元素之和为 `target` 的值, 如果找到, 就可以返回了.

![hash-table](assets/hash-table.svg)

这个方法的时间复杂度是 `O(nlogn)`.

```rust
{{#include src/main.rs:18:34 }}
```