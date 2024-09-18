# 简易压缩算法/一种字符串压缩表示的解压

## 题目描述

有一种简易压缩算法: 针对全部为小写英文字母组成的字符串, 将其中连续超过两个相同字母的部分压缩为连续个数加该字母
其他部分保持原样不变.

例如字符串 `aaabbccccd` 经过压缩变成字符串 `3abb4cd`.

请您编写解压函数,根据输入的字符串,判断其是否为合法压缩过的字符串.

- 若输入合法则输出解压缩后的字符串
- 否则输出字符串!error来报告错误

### 输入描述

- 输入一行, 为一个 ASCII 字符串
- 长度不超过100字符
- 用例保证输出的字符串长度也不会超过100字符串

### 输出描述

若判断输入为合法的经过压缩后的字符串, 则输出压缩前的字符串.
若输入不合法, 则输出字符串 `!error`.

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