#!/usr/bin/env python3

import sys

# Brute force
def main():
    num_guess = int(input().strip())

    # 读取输入, (guess_num, xAyB)
    guess_list = [tuple(input().split()) for _ in range(num_guess)]


    # 记录当前所有匹配的整数
    # 当它为 1 时, 表示猜中了
    # 当它大于1 时, 表示可能有多个数值符合, 所以无法确定真正的数值
    valid_count = 0
    ans = ""

    # 暴力猜测, 生成 0000-9999 范围内所有的整数
    # 然后计算它的模式 xAyB 是否跟给定的模式一致, 如果一致那就是这个整数
    # 如果不一致, 就去遍历下一个
    for num in range(10000):
        # 将整数转换成四位的字符串
        num_str = F"{num:04d}"

        is_valid = True

        # 遍历每一次猜测
        for guess_num, pattern in guess_list:
            # 数值和位置都正确
            expected_pos_matches = int(pattern[0])
            # 只有数值正确而位置错误的数值
            expected_digit_matches = int(pattern[2])

            # 用于记录数字和位置都相同的个数
            pos_matches = 0

            # 存储每个数字出现的次数
            guess_arr = [0] * 10
            num_arr = [0] * 10

            for i in range(len(guess_num)):
                # 遍历每个位上的数值
                guess_digit = int(guess_num[i])
                num_digit = int(num_str[i])

                if guess_digit == num_digit:
                    # 位置和数值都相等
                    pos_matches += 1
                else:
                    # 记录该数值出现的次数
                    guess_arr[guess_digit] += 1
                    num_arr[num_digit] += 1

            # 接下来计算数字相同但位置不同
            digit_matches = sum(min(guess_arr[i], num_arr[i]) for i in range(10))

            # 结果不符, 不再遍历下个猜测的数字
            if pos_matches != expected_pos_matches or digit_matches != expected_digit_matches:
                is_valid = False
                break


        if is_valid:
            valid_count += 1
            ans = num_str

            # 符合条件的数值比较多, 不再猜了
            if valid_count > 1:
                break

    # 判断结果
    if valid_count == 1:
        print(ans)
    else:
        print("NA")


if __name__ == "__main__":
    main()
