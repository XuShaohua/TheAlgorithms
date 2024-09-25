#!/usr/bin/env python3

def countSubstrings(s: str) -> int:
    # 回文字符串的一个特性是中间对齐, 我们可以遍历所有字符, 以它为中间, 尝试
    # 向左右两侧扩展, 检查它们是否相等, 如果相等, 就说明这个 substring 是回文的.

    palindrome_count = 0 

    def twoSideEquals(s: str, left: int, right: int) -> bool:
        # 先检查两个索引值是否有效, 再检查它们指向的字符是否相等
        if left >= 0 and right < len(s) and s[left] == s[right]:
            return True
        else:
            return False

    # middle 是 substring 的中间节点
    for middle in range(len(s)):
        # 如果 substring 中包含奇数个字符, 那就以 (middle) 作为中间对齐节点
        left = middle
        right = middle
        while twoSideEquals(s, left, right):
            palindrome_count += 1
            left -= 1
            right += 1

        # 如果 substring 中包含偶数个字符, 那就以 (middle, middle+1) 作为中间对齐节点
        left = middle
        right = middle + 1
        while twoSideEquals(s, left, right):
            palindrome_count += 1
            left -= 1
            right += 1

    # 返回结果
    return palindrome_count

def main():
    s1 = "abc"
    assert(countSubstrings(s1) == 3)

    s2 = "aaa"
    assert(countSubstrings(s2) == 6)

if __name__ == "__main__":
    main()
