# 第k个排列

## 题目描述

给定参数n, 从1到n会有n个整数: 1,2,3,…,n,这n个数字共有n!种排列.

按大小顺序升序列出所有排列的情况, 并一一标记,

当n=3时, 所有排列如下:

"123", "132" "213" "231" "312" "321"

给定n和k, 返回第k个排列.

### 输入描述

- 输入两行, 第一行为n, 第二行为k
- 给定n的范围是 [1,9], 给定k的范围是 [1,n!]

### 输出描述

输出排在第k位置的数字.

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

### C++

```cpp
{{#include solution.cpp:5:}}
```
