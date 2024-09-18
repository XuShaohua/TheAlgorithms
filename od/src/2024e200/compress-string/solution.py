#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

def main():
    # 读取输入
    # 遍历字符串s中的所有字符
    # 检查输入的字符串s是否只包含数字和小写字母, 如果不是, 就报错并退出
    # 遇到数字后, 就将数字后面的字母展开
    # 如果最后一个字符是数字, 或者数字后面跟着数字, 也是无效输入
    # 如果同一个字母连续超过2次出现, 说明它没有被压缩, 也是无效输入

    s = input()
    assert 0 < len(s) <= 100

    MIN_CONTINUAS_CHARS = 3
    
    is_invalid_input = False
    last_number = -1
    # 存储所有字符
    out_chars = []
    # 记录同一个字母连续出现的次数
    last_char = ""
    last_char_count = 0

    # 遍历所有字符
    for i in range(len(s)):
        char = s[i]
        if char.isdigit():
            # 数字后面跟着另一个数字
            if last_number != -1:
                #print("duplicated num:", last_number, char)
                is_invalid_input = True
                break
            last_number = int(char)

            # 如果 last_number <= 2, 表示它不应该被压缩
            if last_number < MIN_CONTINUAS_CHARS:
                is_invalid_input = True
                break

            # 如果最后一个字符数字, 也是无效输入
            if i + 1 == len(s):
                is_invalid_input = True
                break

            # 重置连续字符出现的次数
            last_char = ""
            last_char_count = 0
        elif char.islower():
            if last_char == char:
                last_char_count += 1
            else:
                last_char = char
                last_char_count = 1

            # 先检查同一个字母连续出现的次数, 应该压缩而没有压缩
            if last_char_count >= MIN_CONTINUAS_CHARS:
                #print("last_char:", last_char, ", last_char_count:", last_char_count)
                is_invalid_input = True
                break

            if last_number == -1:
                out_chars.append(char)
            else:
                # 如果该字母左侧是一个数字, 就把它展开
                for i in range(last_number):
                    out_chars.append(char)

            # 并重置上个数字
            last_number = -1
        else:
            # 其它都是无效输入字符
            #print("other chars:", char)
            is_invalid_input = True
            break

    if is_invalid_input:
        print("!error")
    else:
        print("".join(out_chars))

if __name__ == "__main__":
    main()
