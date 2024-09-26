
# 0005. 最长回文子串 Longest Palindromic Substring

[问题描述](https://leetcode.com/problems/longest-palindromic-substring)

这个问题, 要充分考虑回文字符串的特性:
- 如果回文字符串有奇数个字符, 那就以中间那个字符为分界, 左右两侧完全相同
- 如果回文字符串有偶数个字符, 那中间两个字符相同, 并且它们左右两侧也完全相同

基于这个特性, 可以利用双指针法来找出回文子串, 具体步骤是:
- 先遍历字符串中的每个字符
- 并以它为中心字符 (middle), 利用远离型双指针, 分别向左右两侧扩张, 并且 s[left] == s[right]
- 接下来考虑偶数个字符的情况, 以 (middle, middle + 1) 为中心, 利用远离型双指针, 分别向左右两侧扩张
- 通过这个循环, 就可以找出字符串 s 中的所有回文子串
- 然后保存下来最长的那个子串, 既是最终的结果

代码实现如下:

## Rust

```rust
{{#include src/main.rs:5:52}}
```

## Python

```python
{{#include main.py:6:}}
```
