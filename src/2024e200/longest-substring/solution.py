#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

from collections import defaultdict

def main():
    # 先读取输入值
    ignored_char = input()
    s = input()
    assert len(ignored_char) == 1
    assert 1 <= len(s) <= 10000

    # 使用滑动窗口遍历字符串s中的所有字符
    # 使用字典来统计各个字符出现的次数
    # 当窗口右侧向右移动时, 将新的字符加到到计数字典中
    # 当窗口左侧向右移动时, 将左侧旧的字符从计数字典中减1
    # 判定条件有两个:
    #   1. 不能出现 ignored_char
    #   2. 窗口中同一个字符最大出现次数是2次
    
    # 窗口中各字符的计数
    window_chars = defaultdict(int)

    left = 0
    right = 0
    substring_max_len = 0
    # 窗口中同一个字符最大出现次数是2次
    char_max_presents = 2

    # 遍历字符串s
    while right < len(s):
        right_char = s[right]

        # 如果窗口右侧出现了禁止的字符, 就说明这个窗口要被终止了
        if right_char == ignored_char:
            substring_max_len = max(substring_max_len, right - left)
            # 将左右指针都移动到下一个字符
            right += 1
            left = right
            # 重置计数字典
            window_chars.clear()
            continue

        # 将当前字符加入到计数字典中
        window_chars[right_char] += 1

        # 如果当前字符出现次数超过 2 次, 就需要把窗口左侧向右移动
        if window_chars[right_char] > char_max_presents:
            substring_max_len = max(substring_max_len, right - left)
            left_char = s[left]
            while left_char != right_char:
                window_chars[left_char] -= 1
                left += 1
                left_char = s[left]

        # 最后, 将窗口右侧向右移
        right += 1

    # 最后一个子串
    substring_max_len = max(substring_max_len, right - left)

    print(substring_max_len)

if __name__ == "__main__":
    main()
