# 文本统计分析

题目描述

有一个文件, 包含以一定规则写作的文本, 请统计文件中包含的文本数量.

规则如下:

1. 文本以 `;` 分隔, 最后一条可以没有 `;`, 但空文本不能算语句, 比如 `COMMAND A; ;`只能算一条语句. 注意,
   无字符/空白字符/制表符都算作 `空` 文本
2. 文本可以跨行, 比如下面, 是一条文本, 而不是三条:

```text
COMMAND A
AND
COMMAND B;
```

3. 文本支持字符串, 字符串为成对的单引号(')或者成对的双引号(“), 字符串可能出现用转义字符()处理的单双引号
   (`"your input is""`) 和转义字符本身, 比如:

```text
COMMAND A "Say \"hello\"";
```

4. 支持注释, 可以出现在字符串之外的任意位置注释以 `–` 开头, 到换行结束, 比如:

```text
COMMAND A; --this is comment
COMMAND --comment
A AND COMMAND B;

```

注意字符串内的 `–`, 不是注释.

### 输入描述

文本文件.

### 输出描述

包含的文本数量.

### 示例1

输入:

```text
{{#include assets/input1.txt}}
```

输出:

```text
{{#include assets/output1.txt}}
```

## 题解
