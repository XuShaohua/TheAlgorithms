// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <cassert>

#include <iostream>
#include <sstream>
#include <string>
#include <sstream>

class Solution {
 public:
  static bool isIPv4(const std::string& query) {
    // 用 `.` 来分隔各部分
    // 并判断每个部分是有效的数值
    // 数值不带有前缀0

    if (query.empty() || query[0] == '.' || query[query.size() - 1] == '.') {
      return false;
    }
    int part_count = 0;
    std::stringstream ss(query);
    std::string part;
    while (std::getline(ss, part, '.')) {
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

      size_t pos = 0;
      const int val = std::stoi(part, &pos);
      // 不是有效的整数
      if (pos != part.size()) {
        //return false;
      }

      // 数值范围是 0..255
      if (val < 0 || val > 255) {
        return false;
      }

      part_count += 1;
    }

    // 要有4个部分
    return part_count == 4;
  }

  static bool isIPv6(const std::string& query) {
    // 使用 `:` 作为分隔符
    // 每个部分是16进制的整数, 16进制支持大小写, 最多包含4个字符
    // 可以有0作为前缀
    // 不需要考虑缩写

    if (query.empty() || query[0] == ':' || query[query.size() - 1] == ':') {
      return false;
    }

    int part_count = 0;
    std::stringstream ss(query);
    std::string part;
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
    const std::string s1 = "172.16.254.1";
    const std::string expected = "IPv4";
    const std::string out = Solution::validIPAddress(s1);
    assert(out == expected);
  }
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
