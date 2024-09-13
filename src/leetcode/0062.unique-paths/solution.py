
class Solution:
    # 动态规划
    def uniquePaths(self, rows: int, columns: int) -> int:
        # 创建二维数组, 用于记录每个坐标点可能的路径数量
        dp = [[0 for _column in range(columns)] for _row in range(rows)]

        # 横向移动的方法只有一种
        for column in range(columns):
            dp[0][column] = 1

        # 纵向移动的方法只有一种
        for row in range(rows):
            dp[row][0] = 1

        for row in range(1, rows):
            for column in range(1, columns):
                # 观察规律, 找到以下关系
                dp[row][column] = dp[row][column - 1] + dp[row - 1][column]

        return dp[rows - 1][columns - 1]
