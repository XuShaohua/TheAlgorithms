# 构成正方形的数量

## 题目描述

输入N个互不相同的二维整数坐标, 求这N个坐标可以构成的正方形数量. 内积为零的的两个向量垂直.

### 输入描述

第一行输入为N, N代表坐标数量, N为正整数. N <= 100

之后的 K 行输入为坐标x y以空格分隔, x, y为整数, -10 <= x, y <= 10.

### 输出描述

输出可以构成的正方形数量.

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