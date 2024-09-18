#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

def main():
    # 读取输入
    first_word_index = int(input())
    word_count = int(input())
    assert 0 <= first_word_index < word_count
    words = []
    for i in range(word_count):
        words.append(input())

    s = words[first_word_index]
    # 将第一个单词从字典中去除, 然后重整字典
    words.pop(first_word_index)

    # 开始接龙, 如果字典已经空了, 就终止
    while words:
        next_char = s[-1]
        last_word_index = -1
        last_word = ""

        # 遍历字典, 找到以 next_char 开头的单词
        for i in range(len(words)):
            word = words[i]
            # 将当前词更新为 last_word 的条件有:
            # - 当前词的长度比上个词长
            # - 或者当前词的字典序小于上个词
            if word.startswith(next_char):
                if len(word) > len(last_word) or (len(word) == len(last_word) and word < last_word):
                    last_word = word
                    last_word_index = i
        # 没有找到合适的单词, 终止循环
        if last_word_index == -1:
            break
    
        # 接龙, 并将该单词从字典中移除
        s += last_word
        words.pop(last_word_index)

    print(s)

if __name__ == "__main__":
    main()
