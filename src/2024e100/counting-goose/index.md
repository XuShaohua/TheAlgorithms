# 数大雁

## 题目描述

一群大雁往南飞, 给定一个字符串记录地面上的游客听到的大雁叫声, 请给出叫声最少由几只大雁发出.

具体的:

1. 大雁发出的完整叫声为"quack", 因为有多只大雁同一时间嘎嘎作响, 所以字符串中可能会混合多个"quack"
2. 大雁会依次完整发出"quack", 即字符串中 'q', 'u', 'a', 'c', 'k' 这5个字母按顺序完整存在才能计数为一只大雁;
   如果不完整或者没有按顺序则不予计数
3. 如果字符串不是由 'q', 'u', 'a', 'c', 'k' 字符组合而成, 或者没有找到一只大雁, 请返回 `-1`

### 输入描述

一个字符串, 包含大雁 quack 的叫声, `1 <= 字符串长度 <= 1000`, 字符串中的字符只有 'q', 'u', 'a', 'c', 'k'.

### 输出描述

大雁的数量

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

### 示例3

输入:

```text
{{#include assets/input3.txt}}
```

输出:

```text
{{#include assets/output3.}}
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
{{#include solution.py:5:}}
```