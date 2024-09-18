#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

import string

def solution():
    # 读取输入
    s1 = input()
    s2 = input()
    k = int(input())
    assert 0 < k

    # 统计s1中各个字母出现的次数
    s1_chars = dict((char, 0) for char in string.ascii_lowercase)
    for char in s1:
        s1_chars[char] += 1

    # 滑动窗口内各个字母出现的次数
    window_chars = dict((char, 0) for char in string.ascii_lowercase)
    left = 0
    right = 0
    s1_chars_left = len(s1)

    while right < len(s2):
        # 更新窗口内各个字母的次数
        right_char = s2[right]
        window_chars[right_char] += 1

        if window_chars[right_char] <= s1_chars[right_char]:
            s1_chars_left -= 1

        # 如果窗口太大, 将窗口左侧向右移
        if right - left + 1 > len(s1) + k:
            left_char = s2[left]
            if window_chars[left_char] <= s1_chars[left_char]:
                s1_chars_left += 1

            # 将左侧字符从窗口字母计数中移除
            window_chars[left_char] -= 1
            left += 1

        # 如果 s1 中没有剩下的字符待检查, 就说明找到了子串
        if s1_chars_left == 0:
            return left

        # 将窗口右侧向右移
        right += 1

    return -1

def main():
    ans = solution()
    print(ans)

if __name__ == "__main__":
    main()
