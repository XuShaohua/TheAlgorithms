#!/usr/bin/env python3

def coinChange(coins: List[int], amount: int) -> int:
    def dp(n):
        # 基本常量
        INITIAL_COUNT = 2 ** 30
        INVALID_COUNT = -1
        # 初始条件
        if n == 0:
            # 恰好兑换完
            return 0
        if n < 0:
            # 说明没有合适的币值去完成兑换
            return INVALID_COUNT

        # 初始的最小兑换次数要足够大
        min_exchange_count = INITIAL_COUNT

        # 依次遍历每一种币值, 并使用其中的币值换一次
        # 找出最小的兑换次数
        for coin in coins:
            # 递归调用, 用当前的币值兑换一次
            exchange_count = dp(n - coin) + 1
            if exchange_count != INVALID_COUNT:
                min_exchange_count = min(min_exchange_count, exchange_count)

        # 最后返回最小值
        if min_exchange_count == INITIAL_COUNT:
            return INVALID_COUNT
        else:
            return min_exchange_count

    # 调用 dp(), 递归获得最小次数
    return dp(amount)

def main():
    pass

if __name__ == "__main__":
    main()
