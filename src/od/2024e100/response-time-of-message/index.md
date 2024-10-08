# 报文响应时间

## 题目描述

IGMP 协议中, 有一个字段称作最大响应时间 (Max Response Time), HOST收到查询报文, 解折出 MaxResponseTime 字段后, 需要在
`(0，MaxResponseTime]` 时间(秒)内选取随机时间回应一个响应报文.
如果在随机时间内收到一个新的查询报文, 则会根据两者时间的大小, 选取小的一方刷新回应时间.

最大响应时间有如下计算方式:

- 当 `MaxRespCode < 128` 时, `MaxRespTime = MaxRespCode`
- 当 `MaxRespCode ≥ 128` 时, `MaxRespTime = (mant | 0x10) << (exp + 3)`

注:

- `exp` 最大响应时间的高5~7位
- `mant` 为最大响应时间的低4位

其中接收到的 `MaxRespCode` 最大值为 255, 以上出现所有字段均为无符号数.

现在我们认为 HOST 收到查询报文时, 选取的随机时间必定为最大值, 现给出 HOST 收到查询报文个数 C,
HOST 收到该报文的时间T, 以及查询报文的最大响应时间字段值 M, 请计算出 HOST 发送响应报文的时间.

### 输入描述

第一行为查询报文个数 C, 后续每行分别为 HOST 收到报文时间 T, 及最大响应时间M, 以空格分割.

### 输出描述

HOST 发送响应报文的时间.

备注: 用例确定只会发送一个响应报文, 不存在计时结束后依然收到查询报文的情况.

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

- 收到3个报文
- 第0秒收到第1个报文, 响应时间为20秒, 则要到 `0+20=20` 秒响应
- 第1秒收到第2个报文, 响应时间为10秒, 则要到 `1+10=11` 秒响应, 与上面的报文的响应时间比较获得响应时间最小为11秒
- 第8秒收到第3个报文, 响应时间为20秒, 则要到 `8+20=28` 秒响应, 与第上面的报文的响应时间比较获得响应时间最小为11秒
- 最终得到最小响应报文时间为11秒

### 示例2

输入:

```text
{{#include assets/input2.txt}}
```

输出:

```text
{{#include assets/output2.txt}}
```

说明:

- 收到2个报文
- 第0秒收到第1个报文, 响应时间为255秒, 则要到 `(15 | 0x10) << (7 + 3)= 31744` 秒响应, (mant = 15，exp = 7)
- 第200秒收到第2个报文, 响应时间为60秒, 则要到 `200+60-260` 秒响应, 与第上面的报文的响应时间比较获得响应时间最小为260秒
- 最终得到最小响应报文时间为260秒

## 题解

这个问题要搞明白如何进行位运算, 然后注意边界条件即可.

### Python

```python
{{#include solution.py:6:}}
```

### C++

```cpp
{{#include solution.cpp:5:}}
```

### Rust

```rust
{{#include src/main.rs:5:}}
```