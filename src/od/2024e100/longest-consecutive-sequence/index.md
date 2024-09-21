# 最长连续子序列

## 题目描述

有N个正整数组成的一个序列, 给定整数sum, 求长度最长的连续子序列, 使他们的和等于sum, 返回此子序列的长度.

如果没有满足要求的序列, 返回`-1`.

### 输入描述

- 第一行输入是: N个正整数组成的一个序列
- 第二行输入是: 给定整数sum

### 输出描述

最长的连续子序列的长度

备注:

- 输入序列仅由数字和英文逗号构成, 数字之间采用英文逗号分隔
- 序列长度: `1 <= N <= 200`
- 输入序列不考虑异常情况

### 示例1

输入:

```text
{{#include assets/input1.txt}}
```

输出:

```text
{{#include assets/output1.txt}}
```

说明: 1,2,3和4, 2两个序列均能满足要求, 所以最长的连续序列为1,2,3, 因此结果为3.

### 示例2

输入:

```text
{{#include assets/input2.txt}}
```

输出:

```text
{{#include assets/output2.txt}}
```

## 题解

### Python

```python
{{#include solution.py:6:}}
```

### Rust

```rust
{{#include src/main.rs:5:54}}
```

### C++

```cpp
{{#include solution.cpp:5:57}}
```