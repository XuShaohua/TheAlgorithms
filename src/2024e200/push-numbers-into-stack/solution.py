#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

def main():
    # 读取输入, 并将它们转换成正整数
    # 然后创建一个空的栈
    # 遍历所有的正整数, 依次入栈
    # 每次入栈时做以下检查:
    # 1. 如果 n1=n2, 则它们全出栈, 然后自动入栈 2 * n1
    # 2. 如果 n1=sum(n2, n3..), 则它们全出栈, 然后自动入栈 2 * n1
    # 最后打印栈中剩下的整数

    numbers = list(map(int, input().split()))
    stack = []

    for number in numbers:
        # 检查规则1
        if stack and stack[-1] == number:
            stack[-1] += number
            continue

        top_sum = 0
        will_append = True

        # 从栈顶开始求和
        for i in range(len(stack) - 1, -1, -1):
            top_sum += stack[i]
            if top_sum > number:
                # 不满足
                break
            elif top_sum == number:
                # 满足规则2
                for j in range(len(stack) - 1, i - 1, -1):
                    stack.pop()
                stack.append(number * 2)
                will_append = False
                break

        if will_append:
            # 如果上面的规则不满足, 就把该整数入栈
            stack.append(number)
    # 打印结果
    s = " ".join(map(str, reversed(stack)))
    print(s)

if __name__ == "__main__":
    main()
