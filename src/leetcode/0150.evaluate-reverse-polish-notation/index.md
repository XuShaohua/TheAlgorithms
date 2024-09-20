# 0150. 逆波兰表达式求值 Evaluate Reverse Polish Notation

[问题描述](https://leetcode.com/problems/evaluate-reverse-polish-notation)

这个问题是解析后缀表达式, 可以用栈来完成.

但要注意操作符对应的左右两侧的数值顺序.

## 代码实现

### Rust

```rust
{{#include src/main.rs:41:85}}
```

### C++

```cpp
{{#include main.cpp:5:}}
```