#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

import string


def main():
    # 首先解析出所有的单词, 忽略掉空格以及标点等, 并生成一个集合, 以备查询
    #   1. 区分大小写
    #   2. 如果有'的话, 会被作为两个单词
    #
    # 然后将前缀字符串在集合中查找, 找到以此开头的;
    # 当然, 如果考虑速度的话, 可以将单词存放列表, 并排序, 这样就可以用二分法;
    # 或者生成一个字典树 trie tree, 查找前缀的速度会更快.

    sentence = input()
    prefix = input()

    # 使用转换表, 将所有的标点都转换成空格,
    # 然后以空格为分隔, 得到所有的单词
    sentence = sentence.translate(str.maketrans(string.punctuation, " " * len(string.punctuation)))
    words = set(sentence.split())
    #print("dict:", words)

    ans = []
    for word in words:
        if word.startswith(prefix):
            ans.append(word)

    ans.sort()
    if ans:
        print(" ".join(ans))
    else:
        print(prefix)


if __name__ == "__main__":
    main()
