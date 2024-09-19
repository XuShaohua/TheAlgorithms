# 数组拼接

## 题目描述

现在有多组整数数组, 需要将它们合并成一个新的数组.

合并规则, 从每个数组里按顺序取出固定长度的内容合并到新的数组中, 取完的内容会删除掉,
如果该行不足固定长度或者已经为空, 则直接取出剩余部分的内容放到新的数组中, 继续下一行.

### 输入描述

- 第一行是每次读取的固定长度, 0<长度<10
- 第二行是整数数组的数目, 0<数目<1000
- 第3-n行是需要合并的数组, 不同的数组用回车换行分隔, 数组内部用逗号分隔, 最大不超过100个元素

### 输出描述

输出一个新的数组, 用逗号分隔.

### 示例1

输入:

```text
{{#include assets/input1.txt}}
```

输出:

```text
{{#include assets/output1.txt}}
```

说明:

1. 获得长度3和数组数目2
2. 先遍历第一行, 获得2,5,6
3. 再遍历第二行, 获得1,7,4
4. 再循环回到第一行, 获得7,9,5
5. 再遍历第二行, 获得3,4
6. 再回到第一行, 获得7, 按顺序拼接成最终结果

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