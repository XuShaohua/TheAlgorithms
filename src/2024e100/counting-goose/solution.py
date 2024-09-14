#!/usr/bin/env python3

import sys

def solution():
    # 读取输入
    string = input().strip()

    # 记录每个 "quack" 叫声在字符串在的位置, 包括起始和结束
    quack_pairs = []
    # 记录 "q" 字符在字符串中的位置
    q_index = []
    # 记录 u/a/c 三个字符在字符串中出现的次数
    u_count = 0
    a_count = 0
    c_count = 0

    # 先遍历字符串, 找出所有的 "quack" 字符串
    for i in range(len(string)):
        char = string[i]

        if char == "q":
            # 把字符 "q" 所在位置存储起来
            q_index.append(i)
        elif char == "u":
            # 如果 有足够多的字符 "q", 就将字符"u"的计数加1
            if len(q_index) > u_count:
                u_count += 1
        elif char == "a":
            # 如果 有足够多的字符 "u", 就将字符"a"的计数加1
            if u_count > a_count:
                a_count += 1
        elif char == "c":
            # 如果 有足够多的字符 "a", 就将字符"c"的计数加1
            if a_count > c_count:
                c_count += 1
        elif char == "k":
            # 如果有字符 "c", 就说明可以组成一个有效的叫声
            if c_count > 0:
                # 记录下当前的叫声, 包括起始点和结束点
                quack_pairs.append((q_index.pop(), i))
                # 同时减去字符计数
                u_count -= 1
                a_count -= 1
                c_count -= 1
        else:
            # 无效字符
            print("Invalid char", char)
            return -1

    # 没有找到有效的叫声
    if len(quack_pairs) == 0:
        print("quack_paris is empty")
        return -1

    # 接下来, 找出重叠的 "quack" 字符串有多少个, 并取它们的最大值
    max_quack_count = 1
    for i in range(len(quack_pairs)):
        # 以当前叫声为起点, 找出所有重叠的叫声
        current_count = 1
        for j in range(i + 1, len(quack_pairs)):
            # 如果有重叠, 计数就加1
            if quack_pairs[i][1] >= quack_pairs[j][0]:
                current_count += 1
        # 更新最大重叠的叫声数
        max_quack_count = max(max_quack_count, current_count)

    return max_quack_count

def main():
    count = solution()
    print(count)

if __name__ == "__main__":
    main()
