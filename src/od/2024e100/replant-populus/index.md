# 补种未成活胡杨

## 题目描述

近些年来, 我国防沙治沙取得显著成果. 某沙漠新种植N棵胡杨 (编号1-N), 排成一排.

一个月后, 有M棵胡杨未能成活.

现可补种胡杨K棵, 请问如何补种 (只能补种, 不能新种), 可以得到最多的连续胡杨树?

### 输入描述

- N 总种植数量, 1 <= N <= 100000
- M 未成活胡杨数量, M 个空格分隔的数, 按编号从小到大排列, 1 <= M <= N
- K 最多可以补种的数量, 0 <= K <= M

### 输出描述

最多的连续胡杨棵树.

### 示例1

输入:

```text
{{#include assets/input1.txt}}
```

输出:

```text
{{#include assets/output1.txt}}
```

说明: 补种到2或4结果一样, 最多的连续胡杨棵树都是3.

### 示例2

输入:

```text
{{#include assets/input2.txt}}
```

输出:

```text
{{#include assets/output2.txt}}
```

说明: 种第7棵树, 最多连续胡杨树棵数位6 (5, 6, 7, 8, 9, 10)

## 题解

### Python

```python
{{#include solution.py:6:}}
```