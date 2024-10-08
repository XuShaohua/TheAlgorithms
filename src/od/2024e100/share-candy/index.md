# 分糖果

## 题目描述

小明从糖果盒中随意抓一把糖果, 每次小明会取出一半的糖果分给同学们.

当糖果不能平均分配时, 小明可以选择从糖果盒中 (假设盒中糖果足够) 取出一个糖果或放回一个糖果.

小明最少需要多少次 (取出, 放回和平均分配均记一次), 能将手中糖果分至只剩一颗.

### 输入描述

抓取的糖果数 (< 10000000000)

### 输出描述

最少分至一颗糖果的次数

### 示例1

输入:

```text
{{#include assets/input1.txt}}
```

输出:

```text
{{#include assets/output1.txt}}
```

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

动态规划的思路:

- 如果是奇数个糖果, 就有两种方法: 取一个再平分;放一个再平分. 我们可以分别计算它们的结果, 再求得其中的最小值
- 如果是偶数个糖果, 就先平均分一次
- 使用缓存存储中间结果, 加快运算

### Python

```python
{{#include solution.py:6:}}
```

### C++

```cpp
{{#include solution.cpp:5:}}
```

### Rust

```rust
{{#include src/main.rs:5:}}
```