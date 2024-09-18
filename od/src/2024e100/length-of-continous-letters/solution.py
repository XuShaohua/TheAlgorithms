#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

import sys

def main():
    string = input().strip()
    k = int(input().strip())

    # 先遍历字符串, 分隔出连续相同字符, 然后统计其个数, 存放到计数字典中
    current_char = 'A'
    current_char_count = 0
    char_dict = dict()
    string_len = len(string)
    for char in string:
        # 如果当前的字符与上个字符不相同
        if current_char != char:
            # 保存到字典中
            if current_char_count > 0:
                # 如果该字符在字典中已经存在, 则只保存最大连续数
                last_count = char_dict.get(current_char)
                if last_count:
                    char_dict[current_char] = max(last_count, current_char_count)
                else:
                    char_dict[current_char] = current_char_count
            # 重置上个字符及其计数
            current_char = char
            current_char_count = 1
        else:
            current_char_count += 1

    # 处理最后一个字符
    if current_char_count > 0:
        # 如果该字符在字典中已经存在, 则只保存最大连续数
        last_count = char_dict.get(current_char)
        if last_count:
            char_dict[current_char] = max(last_count, current_char_count)
        else:
            char_dict[current_char] = current_char_count

    # 将字典转换成列表
    word_list = []
    for (char, count) in char_dict.items():
        word_list.append((count, char))
    # 基于最大连续数进行排序, 从高到低
    word_list.sort(key = lambda pair: pair[0], reverse = True)
    #print(word_list)

    # 并找到第 k 个字符, 注意下标从0开始计数, 而k是从1开始的
    if k <= len(word_list):
        print(word_list[k - 1][0])
    else:
        print(-1)

if __name__ == "__main__":
    main()
