# 最左侧冗余覆盖子串

## 题目描述

给定两个字符串s1和s2和正整数K, 其中s1长度为n1, s2长度为n2, 在s2中选一个子串, 满足:

- 该子串长度为 n1+k
- 该子串中包含s1中全部字母
- 该子串每个字母出现次数不小于s1中对应的字母

我们称s2以长度k冗余覆盖s1, 给定s1, s2, k, 求最左侧的s2以长度k冗余覆盖s1的子串的首个元素的下标,
如果没有返回 `-1`.

### 输入描述

输入三行, 第一行为s1, 第二行为s2, 第三行为k, s1和s2只包含小写字母.

备注:

- `0 ≤ len(s1) ≤ 1000000`
- `0 ≤ len(s2) ≤ 20000000`
- `0 ≤ k ≤ 1000`

### 输出描述

最左侧的s2以长度k冗余覆盖s1的子串首个元素下标, 如果没有返回 `-1`.

### 示例1

输入:

```text
{{#include assets/input1.txt}}
```

输出:

```text
{{#include assets/output1.txt}}
```

说明: 子串aab和abc符合要求, 由于aab在abc的左侧, 因此输出aab的下标 `0`.

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