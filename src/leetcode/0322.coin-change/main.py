#!/usr/bin/env python3

def coinChange(coins: List[int], amount: int) -> int:
    def dp(n):
        # 基本情况
        INIT_COUNT = 2 ** 10
        INVALID_COUNT = -1
        if n == 0:
            return 0
        if n < 0:
            return INVALID_COUNT
        min_exchange_count = INIT_COUNT

        # 依次遍历每一种币值, 并使用其中的币值换一次
        # 找出最小的兑换次数
        for coin in coins:
            # 递归调用, 用当前的币值兑换一次
            exchange_count = dp(n - coin) + 1
            if exchange_count != INVALID_COUNT:
                min_exchange_count = min(min_exchange_count, exchange_count)

        # 最后返回最小值
        if min_exchange_count == INIT_COUNT:
            return INVALID_COUNT
        else:
            return min_exchange_count

    # 调用 dp(), 递归获得最小次数
    return dp(amount)

def main():
    pass

if __name__ == "__main__":
    main()
