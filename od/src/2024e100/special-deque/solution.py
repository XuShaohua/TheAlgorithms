#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

from collections import deque

def main():
    # 读取输入, number 是整数n, 对队列的操作次数是 number * 2
    number = int(input())

    # 创建双端队列
    queue = deque()
    # 是否依照顺序删除
    in_order = True
    # 调整操作的次数
    adjust_times = 0

    # 遍历所有的整数, 依次检查对它的操作
    for i in range(number * 2):
        # 解析每次输入
        parts = input().split()
        op = parts[0]
        if op == "head":
            # 如果此时队列不为空, 说明这个插入导致了无序
            if len(queue) > 0:
                in_order = False
            # 从头部插入整数
            queue.appendleft(int(parts[2]))
        elif op == "tail":
            # 从尾部插入整数
            queue.append(int(parts[2]))
        elif op == "remove":
            # 如果队列为空, 忽略它
            if len(queue) == 0:
                continue
            # 如果不按顺序插入, 则需要调整一次
            if not in_order:
                adjust_times += 1
                in_order = True
            # 从头部移除整数
            queue.popleft()

        else:
            # 无效输入
            pass

    # 输出结果
    print(adjust_times)

if __name__ == "__main__":
    main()
