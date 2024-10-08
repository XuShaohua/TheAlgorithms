# 字符统计及重排

## 题目描述

给出一个仅包含字母的字符串, 不包含空格, 统计字符串中各个字母（区分大小写）出现的次数,

并按照字母出现次数从大到小的顺序. 输出各个字母及其出现次数.

如果次数相同, 按照自然顺序进行排序, 且小写字母在大写字母之前.

### 输入描述

输入一行, 为一个仅包含字母的字符串.

### 输出描述

按照字母出现次数从大到小的顺序输出各个字母和字母次数, 用英文分号分隔, 注意末尾的分号.

字母和次数间用英文冒号分隔.

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

说明: b的出现个数比a多，故b排在a之前

## 题解

### Python

```python
{{#include solution.py:6:}}
```

### Rust

```rust
{{#include src/main.rs:5:}}
```

### C++

```C++
{{#include solution.cpp:6:}}
```