// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <cassert>

#include <iostream>
#include <sstream>
#include <string>

class Solution {
 public:
  static bool isIPv4(const std::string& query) {
    // 用 `.` 来分隔各部分
    // 并判断每个部分是有效的数值
    // 数值不带有前缀0

    if (query[0] == '.' || query[query.size() - 1] == '.') {
      return false;
    }
    std::stringstream ss(query);
    std::string part;
    int part_count = 0;
    while (std::getline(ss, part, '.')) {
      part_count += 1;
      // 数值不带有前缀0
      if (part[0] == '0' && part.size() > 1) {
        return false;
      }
      if (part.size() < 1 || part.size() > 3) {
        return false;
      }
      // 判断字符的范围, 0-9
      for (char c : part) {
        if (!std::isdigit(c)) {
          return false;
        }
      }

      const int val = std::stoi(part);
      // 不是有效的整数
      if (val == 0 && part != "0") {
        return false;
      }
      // 数值范围是 0..255
      if (val < 0 || val > 255) {
        return false;
      }
    }

    // 要有4个部分
    return part_count == 4;
  }

  static bool isIPv6(const std::string& query) {
    // 使用 `:` 作为分隔符
    // 每个部分是16进制的整数, 16进制支持大小写, 最多包含4个字符
    // 可以有0作为前缀
    // 不需要考虑缩写

    if (query[0] == ':' || query[query.size() - 1] == ':') {
      return false;
    }

    std::stringstream ss(query);
    std::string part;
    int part_count = 0;
    while (std::getline(ss, part, ':')) {
      // 1-4个字符
      if (part.size() < 1 || part.size() > 4) {
        return false;
      }

      for (char c : part) {
        // 判断字符的范围, 0-9, a-f, A-F
        if (!std::isxdigit(c)) {
          return false;
        }
      }
      part_count += 1;
    }

    return part_count == 8;
  }

  static std::string validIPAddress(std::string queryIP) {
    if (isIPv4(queryIP)) {
      return "IPv4";
    } 
    if (isIPv6(queryIP)) {
      return "IPv6";
    }
    return "Neither";
  }
};

void checkSolution() {
  {
    const std::string s1 = "2001:0db8:85a3:0:0:8A2E:0370:7334:";
    const std::string expected = "Neither";
    const std::string out = Solution::validIPAddress(s1);
    assert(out == expected);
  }

  {
    const std::string s1 = "1e1.4.5.6";
    const std::string expected = "Neither";
    const std::string out = Solution::validIPAddress(s1);
    assert(out == expected);
  }
}

int main() {
  checkSolution();
  return 0;
}
