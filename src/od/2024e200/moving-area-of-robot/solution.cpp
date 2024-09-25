// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <cassert>

#include <array>
#include <iostream>
#include <string>
#include <vector>

int dfs_visit(const std::vector<std::vector<int>>& grid,
              std::vector<std::vector<bool>>& visited,
              int x, int y, int rows, int columns) {
  if (visited[x][y]) {
    return 0;
  }
  // 先标记当前节点状态
  int move_range = 1;
  visited[x][y] = true;

  const std::array<std::array<int, 2>, 4> directions = 
    {std::array<int, 2>{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
  // 开始准备向四个方向访问
  for (const auto dir : directions) {
    int x1 = x + dir[0];
    int y1 = y + dir[1];
    // 判断新的节点是否满足条件
    // 1. 在矩形范围内移动
    // 2. 新的节点未被访问过
    // 3. 两个节点上的值相差小于等于1

    if (0 <= x1 && x1 < rows && 0 <= y1 && y1 < columns && !visited[x1][y1] && 
        std::abs(grid[x][y] - grid[x1][y1]) <= 1) {
      // 递归访问新的节点
      move_range += dfs_visit(grid, visited, x1, y1, rows, columns);
    }
  }

  // 返回移动次数
  return move_range;
}

void solution() {
  // 读取行与列
  int rows = 0;
  int columns = 0;
  std::cin >> rows >> columns;
  assert(rows > 0 && columns > 0);
  // 换行符
  std::string line;
  std::getline(std::cin, line);

  // 读取二维数组
  std::vector<std::vector<int>> grid;
  for (int row = 0; row < rows; ++row) {
    std::vector<int> nums(columns);
    for (int& num : nums) {
      std::cin >> num;
    }
    grid.emplace_back(nums);
    // 换行符
    std::getline(std::cin, line);
  }

  // 标记已经访问过了的节点
  std::vector<std::vector<bool>> visited(rows, std::vector<bool>(columns, false));

  int max_move_range = 0;

  // 接下来遍历所有的节点, 找到最大的访问范围.
  for (int row = 0; row < rows; ++row) {
    for (int column = 0; column < columns; ++column) {
      // 如果该节点已经访问过, 就忽略
      if (!visited[row][column]) {
        const int move_range = dfs_visit(grid, visited, row, column, rows, columns);
        max_move_range = std::max(max_move_range, move_range);
      }
    }
  }

  // 打印结果
  std::cout << max_move_range << std::endl;
}

int main() {
  solution();
  return 0;
}
