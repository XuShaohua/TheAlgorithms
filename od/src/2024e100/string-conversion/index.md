# 字符串变换最小字符串

## 题目描述

给定一个字符串s, 最多只能进行一次变换, 返回变换后能得到的最小字符串 (按照字典序进行比较).

变换规则: 交换字符串中任意两个不同位置的字符.

### 输入描述

一串小写字母组成的字符串s.

备注:

- s是都是小写字符组成
- 1 <= s.length <= 1000

### 输出描述

一串小写字母组成的字符串s.

### 示例1

输入:

```text
{{#include assets/input1.txt}}
```

输出:

```text
{{#include assets/output1.txt}}
```

说明: abcdef已经是最小字符串, 不需要交换.

### 示例2

输入:

```text
{{#include assets/input2.txt}}
```

输出:

```text
{{#include assets/output2.txt}}
```

说明: a和b进行位置交换, 可以得到最小字符串.

## 题解