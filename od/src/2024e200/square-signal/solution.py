#!/usr/bin/env python3
# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

def main():
    # 读取输入
    # 然后使用双指针法遍历所有的输入信号
    # 方波条件:
    # 1. 以0开头, 以0结尾
    # 2. 0和1交替出现

    ZERO = "0"
    ONE = "1"

    signals = input()
    assert 3 <= len(signals) <= 1024
    assert signals[0] == ZERO and signals[-1] == ZERO

    last_continous_signal = "-1"

    stack = []
    # 遍历输入信号
    for char in signals:
        # 如果栈为空
        if len(stack) == 0:
            # 那第一个信号需要是 "0"
            # 如果不是"0", 就什么都不做
            if char == ZERO:
                print("stack bottom:", char)
                stack.append(char)
            continue

        # 如果栈顶的信号与当前信号相同, 则说明出现了冲突
        if stack[-1] == char:
            # 检查当前栈中是不是有效的连续交替信号
            # 如果是, 就把它更新到结果中
            if stack[-1] == ZERO:
                # 至少是 "010"
                if len(stack) >= 3 and len(stack) > len(last_continous_signal):
                    last_continous_signal = "".join(stack)
            elif len(stack) >= 4:
                # 至少是 "0101"
                stack.pop()
                assert stack[-1] == ZERO
                if len(stack) > len(last_continous_signal):
                    last_continous_signal = "".join(stack)

            # 最后将栈顶清空, 如果当前元素是 "0", 就把它入栈; 否则什么也不做
            stack.clear()
            if char == ZERO:
                stack.append(char)
            continue
        else:
            # 没有出现相同信号, 将新的信号入栈即可
            stack.append(char)
    
    # 输出结果
    print(last_continous_signal)

if __name__ == "__main__":
    main()
