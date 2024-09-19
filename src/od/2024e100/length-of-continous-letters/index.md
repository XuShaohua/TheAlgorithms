# 连续字母长度

## 题目描述

给定一个字符串, 只包含大写字母, 求在包含同一字母的子串中, 长度第 k 长的子串的长度, 相同字母只取最长的那个子串.

### 输入描述

第一行有一个子串 `(1<长度<=100)`, 只包含大写字母.

### 输出描述

输出连续出现次数第k多的字母的次数.

### 示例1

输入:

```text
{{#include assets/input1.txt}}
```

输出:

```text
{{#include assets/output1.txt}}
```

说明:

- 同一字母连续出现的最多的是A和H, 四次
- 第二多的是H, 3次, 但是H已经存在4个连续的, 故不考虑
- 下个最长子串是BB, 所以最终答案应该输出2

### 示例2

输入:

```text
{{#include assets/input2.txt}}
```

输出:

```text
{{#include assets/output2.txt}}
```

说明:

- 同一字母连续出现的最多的是A, 三次
- 第二多的还是A, 两次, 但A已经存在最大连续次数三次, 故不考虑
- 下个最长子串是B, 所以输出1

### 示例3

输入:

```text
{{#include assets/input3.txt}}
```

输出:

```text
{{#include assets/output3.txt}}
```

### 示例4

输入:

```text
{{#include assets/input4.txt}}
```

输出:

```text
{{#include assets/output4.txt}}
```

## 题解

### Python

```python
{{#include solution.py:6:}}
```

### Rust

```rust
{{#include src/main.rs:5:}}
```

### C++

```cpp
{{#include solution.cpp:5:}}
```
