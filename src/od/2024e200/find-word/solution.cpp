// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <cassert>

#include <array>
#include <iostream>
#include <vector>

bool dfs(const std::vector<std::vector<char>>& table,
         std::vector<std::vector<bool>>& visited,
         std::vector<std::pair<int, int>>& path,
         const std::string& word, int word_index,
         int row, int column) {
  const int rows = table.size();
  const int columns = rows;
  // 结束查找:
  // 1. 检查坐标的边界, 如果不在 table 内
  // 2. 或者已经访问过了
  // 3. 或者单词中的字母与在表格中的不一致
  if (row < 0 || row >= rows || column < 0 || column >= columns || 
      visited[row][column] || word[word_index] != table[row][column]) {
    return false;
  }

  // 将当前路径添加到 path 中
  path.emplace_back(row, column);
  // 并标记该节点已经访问过
  visited[row][column] = true;
  // 如果单词中的所有字母都被找到了, 就返回
  if (word_index + 1 == word.size()) {
    return true;
  }
    
  // 定义查找的四个方向
  const std::array<std::array<int, 2>, 4> directions = {
      std::array<int, 2>{1, 0}, {-1, 0}, {0, 1}, {0, -1}
  };

  // 遍历所有可能的方向, 进行深入查找
  for (const std::array<int, 2> dir : directions) {
    const int row1 = row + dir[0];
    const int column1 = column + dir[1];
    // 去找单词中的下一个字母
    const bool found = dfs(table, visited, path, word, word_index + 1, row1, column1);
    // 如果在该方向找到了字符串, 就直接返回
    if (found) {
      return true;
    }
  }

  // 没有找到, 当前前位置从经过的路径中移除
  path.pop_back();
  // 并将该坐标从被访问记录中移除
  visited[row][column] = false;
  return false;
}

void solution() {
  // 读取输入
  int rows = 0;
  std::cin >> rows;
  // 换行符
  std::string line;
  std::cin >> line;
  // 读取所有字符表
  std::vector<std::vector<char>> table;
  for (int row = 0; row < rows; ++row) {
    std::vector<char> row_chars(rows);
    for (char& c : row_chars) {
      std::cin >> c;
    }
    // 换行符
    std::cin >> line;
    table.emplace_back(row_chars);
  }
  // 读取单词
  std::string word;
  std::cin >> word;

  // 用于标记已经访问过的字符, N * N
  std::vector<std::vector<bool>> visited(rows, std::vector<bool>(rows));
  // 用于存储访问路径
  std::vector<std::pair<int, int>> path;

  const int columns = rows;
  // 遍历所有字符
  for (int row = 0; row < rows; ++row) {
    for (int column = 0; column < columns; ++column) {
      // 如果当前单元格的字符等于单词的第一个字母
      if (word[0] == table[row][column]) {
        // 使用DFS查找字符串
        const bool found = dfs(table, visited, path, word, 0, row, column);
        if (found) {
          // 找到了, 就返回结果
          for (const auto& pair : path) {
            std::cout << pair.first << "," << pair.second << std::endl;
          }
          return;
        }
      }
    }
  }

  std::cout << "N" << std::endl;
}

int main() {
  // FIXME(Shaohua): Result invalid
  solution();
  return 0;
}
