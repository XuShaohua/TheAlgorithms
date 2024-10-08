# 周末爬山

## 题目描述

周末小明准备去爬山锻炼, 0代表平地, 山的高度使用1到9来表示, 小明每次爬山或下山高度只能相差k及k以内, 每次只能上下左右一个方向上移动一格,
小明从左上角(0,0)位置出发.

### 输入描述

- 第一行输入m n k (空格分隔)
    - 代表m*n的二维山地图, k为小明每次爬山或下山高度差的最大值
- 然后接下来输入山地图, 一共m行n列, 均以空格分隔. 取值范围:
    - 0 < m ≤ 500
    - 0< n ≤ 500
    - 0 < k < 5

备注: 所有用例输入均为正确格式, 且在取值范围内, 考生不需要考虑不合法的输入格式.

### 输出描述

请问小明能爬到的最高峰多高, 到该最高峰的最短步数, 输出以空格分隔.

同高度的山峰输出较短步数.

如果没有可以爬的山峰, 则高度和步数都返回0.

### 示例1

输入:

```text
{{#include assets/input1.txt}}
```

输出:

```text
{{#include assets/output1.txt}}
```

说明: 根据山地图可知, 能爬到的最高峰在(0,2)位置, 高度为2, 最短路径为 (0,0)-(0,1)-(0,2), 最短步数为2.

### 示例2

输入:

```text
{{#include assets/input2.txt}}
```

输出:

```text
{{#include assets/output2.txt}}
```

说明: 根据山地图可知, 每次爬山距离3, 无法爬到山峰上, 步数为0.

## 题解