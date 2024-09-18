#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

from itertools import permutations

def main():
    # 读取输入
    # 建立数字到字母的映射关系
    # 就可以得到输入数字所有可能的字符串
    # 然后去掉被屏蔽的字符串, 就得到了结果
    num_str = input()
    assert 0 < len(num_str) < 5
    filter_str = input()

    num_digits = list(map(int, num_str))
    digit_mapping = ["abc", "def", "ghi", "jkl", "mno", "pqr", "st", "uv", "wx", "yz"]
    assert len(digit_mapping) == 10

    # 生成数字到字母的映射
    letters = [digit_mapping[digit] for digit in num_digits]

    def dfs(letters, letter_index, path, result, visited):
        # 如果所有的字母都访问过了, 那就返回
        if letter_index >= len(letters):
            # 将当前路径上的字母加入到结果中
            substr = "".join(path)
            # 过滤字符串
            if filter_str not in substr:
                result.append(substr)
            return
        # 遍历当前索引位置的所有字母
        for char in letters[letter_index]:
            # 如果
            if char not in visited:
                path.append(char)
                visited.add(char)
                # 访问下一个字母组合
                dfs(letters, letter_index + 1, path, result, visited)
                # 将字母从临时路径中移除
                path.pop()
                # 将字母从访问过的记录中移除
                visited.remove(char)

    # 存放最后的结果
    result = []
    # 临时存放经过的路径
    path = []
    # 标记已访问过的字母
    visited = set()
    dfs(letters, 0, path, result, visited)

    # 打印结果
    print(",".join(result), ",", sep="")

if __name__ == "__main__":
    main()
