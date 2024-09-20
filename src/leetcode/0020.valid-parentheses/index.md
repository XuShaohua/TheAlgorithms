# 0020. 有效的括号 Valid Parentheses

[问题描述](https://leetcode.com/problems/valid-parentheses)

这个问题使用栈来解决, 思路有两种:

- 遇到左侧的括号, 就把它入栈; 遇到右侧括号, 就将栈顶元素出栈, 并比对它们是否成对
- 遇到左侧的括号, 就把与它对应的右侧括号入栈; 遇到右侧括号, 就将栈顶元素出栈, 并判断它们是否相同

## 代码实现

### Rust

```rust
{{#include src/main.rs:31:47}}
```

### C++

```cpp
{{#include main.cpp:33:57}}
```