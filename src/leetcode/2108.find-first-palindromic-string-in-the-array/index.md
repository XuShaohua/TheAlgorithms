# 2108. 找出数组中的第一个回文字符串 Find First Palindromic String in the Array

[问题描述](../problems/2108.find-first-palindromic-string-in-the-array/content.html)

这道题与 [0125. 验证回文串 Valid Palindrome](../0125.valid-palindrome/index.md) 基本一致,
只是多了一个循环遍历, 将不再讲解.

## 靠拢型双指针

```rust
{{#include src/main.rs:5:27 }}
```

## 反转字符串

```rust
{{#include src/main.rs:29:40 }}
```

## 相关问题

- [0125. 验证回文串 Valid Palindrome](../0125.valid-palindrome/index.md)
