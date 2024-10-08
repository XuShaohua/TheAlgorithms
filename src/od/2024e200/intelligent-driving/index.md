# 智能驾驶

## 题目描述

有一辆汽车需要从 m * n 的地图左上角起点开往地图的右下角终点, 去往每一个地区都需要消耗一定的油量, 加油站可进行加油.

请你计算汽车确保从从起点到达终点时所需的最少初始油量.

说明:

- 智能汽车可以上下左右四个方向移动
- 地图上的数字取值是 0 或 -1 或 正整数
    - -1, 表示加油站, 可以加满油, 汽车的油箱容量最大为100；
    - 0: 表示这个地区是障碍物, 汽车不能通过
    - 正整数: 表示汽车走过这个地区的耗油量
- 如果汽车无论如何都无法到达终点, 则返回 -1

### 输入描述

- 第一行为两个数字, M, N, 表示地图的大小为 M * N, 0 < M,N ≤ 200
- 后面一个 M * N 的矩阵, 其中的值是 0 或 -1 或正整数, 加油站的总数不超过 200 个

### 输出描述

如果汽车无论如何都无法到达终点, 则返回 -1.
如果汽车可以到达终点, 则返回最少的初始油量.

### 示例1

输入:

```text
{{#include assets/input1.txt}}
```

输出:

```text
{{#include assets/output1.txt}}
```

说明: 行走的路线为 `右→下`

### 示例2

输入:

```text
{{#include assets/input2.txt}}
```

输出:

```text
{{#include assets/output2.txt}}
```

说明: 行走的路线为 `右→右→下→下→下→右`

### 示例3

输入:

```text
{{#include assets/input3.txt}}
```

输出:

```text
{{#include assets/output3.txt}}
```

说明: 行走的路线为 `下→下→下→右→右→上→上→上→右→右→下→下→下`

### 示例4

输入:

```text
{{#include assets/input4.txt}}
```

输出:

```text
{{#include assets/output4.txt}}
```

说明: 无论如何都无法到达终点.

## 题解