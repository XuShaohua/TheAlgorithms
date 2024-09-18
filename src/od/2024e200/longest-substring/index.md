# 寻找符合要求的最长子串

## 题目描述

给定一个字符串s, 找出这样一个子串:

- 该子串中任意一个字符最多出现2次
- 该子串不包含指定某个字符

请你找出满足该条件的最长子串的长度.

### 输入描述

- 第一行:要求不包含的指定字符, 为单个字符, 取值范围[0-9a-zA-Z]
- 第二行：字符串s, 每个字符范围[0-9a-zA-Z], 长度范围[1, 10000]

### 输出描述

- 第一行: 要求不包含的指定字符, 为单个字符, 取值范围[0-9a-zA-Z]
- 第二行: 字符串s, 每个字符范围[0-9a-zA-Z], 长度范围[1, 10000]

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