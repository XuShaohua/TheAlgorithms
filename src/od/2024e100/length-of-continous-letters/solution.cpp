// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <cassert>

#include <algorithm>
#include <iostream>
#include <unordered_map>
#include <vector>

int main() {
  // 读取输入
  std::string s;
  std::getline(std::cin, s);
  int k = 0;
  std::cin >> k;

  // 先遍历字符串, 分隔出连续相同字符, 然后统计其个数, 存放到计数字典中
  char current_char = 'A';
  int current_char_count = 0;
  std::unordered_map<char, int> char_dict;
  for (char chr : s) {
    // 如果当前的字符与上个字符不相同
    if (current_char != chr) {
      // 保存到字典中
      if (current_char_count > 0) {
        // 如果该字符在字典中已经存在, 则只保存最大连续数
        auto iter = char_dict.find(current_char);
        if (iter != char_dict.end()) {
          iter->second = std::max(iter->second, current_char_count);
        } else {
          char_dict.emplace(current_char, current_char_count);
        }
      }

      // 重置上个字符及其计数
      current_char = chr;
      current_char_count = 1;
    } else {
      current_char_count += 1;
    }
  }

  // 处理最后一个字符
  if (current_char_count > 0) {
    // 如果该字符在字典中已经存在, 则只保存最大连续数
    auto iter = char_dict.find(current_char);
    if (iter != char_dict.end()) {
      iter->second = std::max(iter->second, current_char_count);
    } else {
      char_dict.emplace(current_char, current_char_count);
    }
  }

  // 将字典转换成列表
  std::vector<std::tuple<int, char>> word_list;
  for (const auto tuple : char_dict) {
    word_list.emplace_back(std::get<1>(tuple), std::get<0>(tuple));
  }
  // 基于最大连续数进行排序, 从高到低
  std::sort(word_list.begin(), word_list.end(),
      [](const std::tuple<int, char>& a, const std::tuple<int, char>& b) {
        return std::get<0>(a) > std::get<0>(b);
  });
  //for (const auto& item : word_list) {
  //  std::cout << std::get<1>(item) << ":" << std::get<0>(item) << std::endl;
  //}

  // 并找到第 k 个字符, 注意下标从0开始计数, 而k是从1开始的
  if (k <= word_list.size()) {
    std::cout << std::get<0>(word_list[k - 1]) << std::endl;
  } else {
    std::cout << "-1" << std::endl;
  }

  return 0;
}
