#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

# 简单的字符串替换
def main():
    # 读取输入
    s = input()
    assert 1 < len(s) < 100000

    # 然后解析所有的括号, 并读取里面的所有字符, 并对它们进行排序, 做成一个替换表
    # 然后将它们从字符串s中移除
    # 最后使用替换表将字符串s中的字符替换成最小序的字符
    raw_chars = []
    tab_chars = []
    found_bracket = False
    for char in s:
        if char == "(":
            found_bracket = True
        elif char == ")":
            found_bracket = False
        elif found_bracket:
            tab_chars.append(char)
        else:
            raw_chars.append(char)

    raw_str = "".join(raw_chars)

    # 对替换表进行排序, 可以找出最小序的字母
    tab_chars.sort()
    if len(tab_chars) > 1:
        first_char = tab_chars[0]
        tab_dict = dict((char, first_char) for char in tab_chars[1:])
        if len(tab_dict) > 1:
            raw_str = raw_str.translate(str.maketrans(tab_dict))

    print(raw_str)

if __name__ == "__main__":
    main()
