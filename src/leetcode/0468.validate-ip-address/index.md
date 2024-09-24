# 0468. 验证IP地址 Validate IP Address

[问题描述](https://leetcode.com/problems/validate-ip-address)

这个是处理字符串的问题.

步骤大致如下:
- 先检查是不是 ipv4, 如果是, 直接返回
  - 先以 `.` 将字符串分隔成多个部分, parts
  - parts 数组的长度应该是 4
  - 检查每一部分字符串
    - 长度在 [1..3] 之间
    - 如果以 `0` 为前缀的话, 只能包含 `0`
    - 检查里面的字符, 只能包含  `0-9` 这10 个字符, 可以用 `std::isdigit(c)`
    - 将它转换成整数, 数值范围是 [0..255]
- 再检查是不是 ipv6, 如果是, 就返回
  - 以 `:` 将字符串分隔成多个部分, parts
  - parts 数组的长度是 8
  - 检查每一部分字符串
    - 字符串长度是 [1..4] 之间
    - 检查里面的字符, 只能包含  0-9, a-f, A-F这些字符, 可以用 `std::is_xdigit(c)`
    - 不需要把它再转换成整数
- 返回 `Neither`

以下是代码实现:

## Rust

```rust
{{#include src/main.rs:5:82}}
```

## C++

```cpp
{{#include main.cpp:5:100}}
```
