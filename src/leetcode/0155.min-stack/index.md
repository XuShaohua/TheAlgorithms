# 0155. 最小栈 Min Stack

[问题描述](https://leetcode.com/problems/min-stack)

这个问题可以有两种解法:

- 使用两个栈, 分别存放正常的数值, 和当前位置的最小值
- 使用一个栈, 但是栈中每个元素是一个元组, 分别存储 当前值和最小值

## 使用两个栈

### Rust

```rust
{{#include src/bin/min-stack-with-dual-vec.rs:5:63}}
```

### C++

```cpp
{{#include dual-stack.cpp:5:40}}
```

## 使用一个栈

### Rust

```rust
{{#include src/bin/min-stack-with-single-vec.rs:5:53}}
```

### C++

```cpp
{{#include single-stack.cpp:5:}}
```