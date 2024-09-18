// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <algorithm>
#include <array>
#include <iostream>
#include <vector>

int main() {
  // 创建计数数组
  // 读取输入, 并统计所有字母出现的次数
  // 统计出现的最多次数
  // 然后逆向遍历所有的次数, 并打印结果

  const size_t kNumChars = 256;
  std::array<int, kNumChars> chars;
  for (int i = 0; i < kNumChars; ++i) {
    chars[i] = 0;
  }

  std::string line;
  std::getline(std::cin, line);
  for (char chr : line) {
    int index = static_cast<int>(chr);
    chars[index] += 1;
  }

  const int max_count = *std::max(chars.cbegin(), chars.cend());
  std::vector<std::string> out;
  const size_t kBufLen = 64;
  char buf[kBufLen];

  for (int count = max_count; count > 0; --count) {
    // 遍历所有的小写字母
    for (int char_index = static_cast<int>('a'); char_index <= static_cast<int>('z'); ++char_index) {
      if (chars[char_index] == count) {
        const int s_len = std::snprintf(buf, kBufLen, "%c:%d", static_cast<char>(char_index), count);
        out.emplace_back(buf, s_len);
      }
    }

    // 再遍历所有的大写字母
    for (int char_index = static_cast<int>('A'); char_index <= static_cast<int>('Z'); ++char_index) {
      if (chars[char_index] == count) {
        const int s_len = std::snprintf(buf, kBufLen, "%c:%d", static_cast<char>(char_index), count);
        out.emplace_back(buf, s_len);
      }
    }
  }

  // 打印结果
  for (int i = 0; i + 1 < out.size(); ++i) {
    std::cout << out[i] << ";";
  }
  std::cout << out[out.size() - 1] << std::endl;

  return 0;
}
