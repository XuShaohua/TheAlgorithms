# 分苹果

## 题目描述

A, B两个人把苹果分为两堆, A希望按照他的计算规则等分苹果, 他的计算规则是按照二进制加法计算,
并且不计算进位 `12+5=9` (1100 + 0101 = 9).
B的计算规则是十进制加法, 包括正常进位, B希望在满足A的情况下获取苹果重量最多.

输入苹果的数量和每个苹果重量, 输出满足A的情况下B获取的苹果总重量.

如果无法满足A的要求, 输出 `-1`.

数据范围:

- 1 <= 总苹果数量 <= 20000
- 1 <= 每个苹果重量 <= 10000

### 输入描述

- 输入第一行是苹果数量, 3
- 输入第二行是每个苹果重量, 3 5 6

### 输出描述

输出第一行是B获取的苹果总重量 11.

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

### Python

```python
{{#include solution.py:6:}}
```