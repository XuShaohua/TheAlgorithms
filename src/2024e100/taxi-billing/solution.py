#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

def main():
    # 读取输入
    line = input().strip()

    # 考虑使用9进制
    # 遍历所有的输入字符, 将它转换成数字
    real_num = 0
    for char in line:
        digit = int(char)
        # 原先的数字跳过了4, 我们把它还原
        if digit > 4:
            digit -= 1
        # 将9进制转换成10进制
        real_num = real_num * 9 + digit
    print(real_num)

if __name__ == "__main__":
    main()
