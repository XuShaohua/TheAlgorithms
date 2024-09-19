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

### Python

```python
{{#include solution.py:6:}}
```