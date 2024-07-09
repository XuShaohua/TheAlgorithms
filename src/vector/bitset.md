# 位图 BitSet

BitSet 又称为 bit map, bit array, bit mask 或者 bit vector, 是一个数组结构, 里面只存储单个的比特, 每一个比特可以表示两个状态.
它是一种很简单的集合数据结构.

## 位图的基本操作

## 位图的集合操作

## 位图的实现

```rust
{{#include assets/bitset.rs:5:236}}
```

## 位图的应用

### 对整数数组快速排序并去重

如果数组中都是整数, 而且数字间比较连续, 比较稠密, 数值的范围也是确定的, 并且要移除重复的元素,
那就可以考虑使用位图来进行快速排序和去重.

使用比特位的下标作为整数值, 这样的话只需要一个比特位就可以表示一个整数, 与 `Vec<i32>` 等存储结构相比,
位图可以显著地节省存储空间.

```rust, no_run
{{#include assets/bitset-sort.rs:5:42}}
```

时间复杂度是 `O(n)`, 空间复杂度是 `O(n)`.

## 参考

- [Bit array](https://en.wikipedia.org/wiki/Bit_array)
- [std::bitset](https://en.cppreference.com/w/cpp/utility/bitset)