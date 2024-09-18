# 查找充电设备组合

## 题目描述

某个充电站, 可提供 n 个充电设备, 每个充电设备均有对应的输出功率.

任意个充电设备组合的输出功率总和, 均构成功率集合 P 的 1 个元素.

功率集合 P 的最优元素, 表示最接近充电站最大输出功率 p_max 的元素.

### 输入描述

输入为 3 行:

- 第 1 行为充电设备个数 n
- 第 2 行为每个充电设备的输出功率
- 第 3 行为充电站最大输出功率 p_max

备注:

- 充电设备个数 n > 0
- 最优元素必须小于或等于充电站最大输出功率 p_max

### 输出描述

功率集合 P 的最优元素.

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

## 题解

### Python

```python
{{#include solution.py:6:}}
```