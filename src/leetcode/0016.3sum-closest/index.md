# 0016. 最接近的三数之和 3Sum Closest

[问题描述](https://leetcode.com/problems/3sum-closest)

## 双指针法

可以使用双指针法遍历整个数组. 具体的步骤如下:
- 先对整数数组进行排序
- 然后计算特殊情况
  - 整数数组长度为3时, 直接返回前三个元素的和
  - 前三个元素的和等于 target 时, 也可以直接返回
- 遍历数组中所有元素(一直到倒数第三个元素), 作为三个数中的第一个数
  - 对于第二个和第三个数, 我们使用双指针法来遍历
  - 先计算三个元素的和 sum, 以及 sum 与 target 的差值绝对值 diff
  - 如果 sum == target, 直接返回
  - 如果 sum < target, 将双指针的左侧元素向右移一位, left += 1
  - 如果 sum > target, 将双指针的右侧元素向左移一位, right -= 1
  - 如果 diff 比目前最小的差值还小, 那就更新它

代码实现如下:

### Rust

```rust
{{#include src/main.rs:5:6}}
{{#include src/main.rs:25:67}}
```

### C++
```cpp
{{#include main.cpp:5:54}}
```
