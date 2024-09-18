#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

def main():
    # 读取输入
    k = int(input())
    assert k > 0
    s = input()

    # - 将字符串s 的第一个子串移除后, 把后面的子串从新拼合,
    # - 然后重新分隔成k长度的子串, 以"-"相连.
    # - 然后改变每个新子串中的字符大小写
    # - 最后与第一个子串结合在一起, 就是结果

    parts = s.split("-")
    first_part = parts[0]
    s2 = "".join(parts[1:])
    new_parts = []
    for i in range(0, len(s2), k):
        new_parts.append(s2[i:i+k])
    # 如果最后剩下少量的字符, 把它作为最后一个子串加进去
    if len(s2) % k != 0:
        new_parts.append(s2[i+k:])

    # 改变大小写
    for i in range(len(new_parts)):
        part = new_parts[i]

        # 先统计大小写字符出现的次数
        lowercase_count = 0
        uppercase_count = 0
        for char in part:
            if char.islower():
                lowercase_count += 1
            elif char.isupper():
                uppercase_count += 1

        # 基于规则改变大小写
        if lowercase_count > uppercase_count:
            part = part.lower()
        elif lowercase_count < uppercase_count:
            part = part.upper()
        new_parts[i] = part

    # 将第一个子串合并进来
    new_parts.insert(0, first_part)
    s3 = "-".join(new_parts)
    print(s3)

if __name__ == "__main__":
    main()
