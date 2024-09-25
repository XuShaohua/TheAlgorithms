#!/usr/bin/env python3

def longestPalindrome(s: str) -> str:
    # 跟据回文字符串的特性

    def twoSidesEqual(s: str, left: int, right: int) -> bool:
        # 先检查两个索引值的有效性, 再比较它们指向的字符是否相等.
        if left >= 0 and right < len(s) and s[left] == s[right]:
            return True
        else:
            return False

    longest_palindrome_len = 0
    longest_palindrome_start = 0

    # 遍历所有字符
    for middle in range(len(s)):
        # 以 (middle) 为对称点, substring 有奇数个字符
        left = middle
        right = middle
        while twoSidesEqual(s, left, right):
            length = right - left + 1
            if longest_palindrome_len < length
                longest_palindrome_len = length
                longest_palindrome_start = left
            left -= 1
            right += 1

        # 以 (middle, middle+1) 作为对称点, substring 有偶数个字符
        left = middle
        right = middle + 1
        while twoSidesEqual(s, left, right):
            length = right - left + 1
            if longest_palindrome_len < length
                longest_palindrome_len = length
                longest_palindrome_start = left
            left -= 1
            right += 1

    # 返回结果
    return s[longest_palindrome_start:longest_palindrome_start+longest_palindrome_len]

def main():
    s1 = "babad"
    out1 = longestPalindrome(s1)
    assert(out1 == "bab")

    s2 = "cbbd"
    out2 = longestPalindrome(s2)
    assert(out2 == "bb")

if __name__ == "__main__":
    main()
