#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

def solution1():
    # 创建计数数组
    # 读取输入, 并统计所有字母出现的次数
    # 统计出现的最多次数
    # 然后逆向遍历所有的次数, 并打印结果
    chars = [0] * 256
    for char in input().strip():
        index = ord(char)
        chars[index] += 1

    max_count = max(chars)
    out = []
    for count in range(max_count, 0, -1):
        # 遍历所有的小写字母
        for char_index in range(ord('a'), ord('z') + 1):
            if chars[char_index] == count:
                out.append("{}:{}".format(chr(char_index), count))
        
        # 再遍历所有的大写字母
        for char_index in range(ord('A'), ord('Z') + 1):
            if chars[char_index] == count:
                out.append("{}:{}".format(chr(char_index), count))

    # 打印结果
    s = ";".join(out)
    print(s)

def solution2():
    # 读取输入
    # 创建计数字典
    # 统计所有字母出现的次数
    # 然后转换成数组, 并对其进行排序

    chars = dict()
    for char in input().strip():
        if char not in chars:
            chars[char] = 0
        chars[char] += 1
    print(chars_list)

    # 打印结果
    s = ";".join(F"{char}:{count}" for (char, count) in chars_list)
    print(s)

def main():
    solution1()

if __name__ == "__main__":
    main()
