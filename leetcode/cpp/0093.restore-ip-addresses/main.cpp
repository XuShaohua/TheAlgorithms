// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <cassert>
#include <cstdio>

#include <algorithm>
#include <string>
#include <vector>

class Solution {
 public:
  static bool isValidIpSector(const std::string& s) {
    if (s.empty()) {
      return false;
    }
    if (s.size() > 3) {
      return false;
    }
    // Leading zero
    if (s.size() > 1 && s[0] == '0') {
      return false;
    }
    int val = std::stoi(s);
    return val >=0 && val <= 255;
  }

  static
  std::vector<std::string> restoreIpAddresses(std::string s) {
    std::vector<std::string> out;
    for (int i = 1; i <= 3 && i < s.size(); ++i) {
      const std::string& first = s.substr(0, i);
      if (!isValidIpSector(first)) {
        continue;
      }
      for (int j = 1; j <= 3 && i + j < s.size(); ++j) {
        const std::string& second = s.substr(i, j);
        if (!isValidIpSector(second)) {
          continue;
        }

        for (int k = 1; k <= 3 && i + j + k < s.size(); ++k) {
          const std::string& third = s.substr(i + j, k);
          const std::string& forth = s.substr(i + j + k);
          if (isValidIpSector(third) && isValidIpSector(forth)) {
            out.emplace_back(first + "." + second + "." + third + "." + forth);
          }
        }
      }
    }
    return out;
  }
};

void checkSolution(std::string s, std::vector<std::string> exp_out) {
  std::vector<std::string> out = Solution::restoreIpAddresses(s);
  printf("size: %d, exp: %d\n", out.size(), exp_out.size());
  assert(out.size() == exp_out.size());
  std::sort(out.begin(), out.end());
  std::sort(exp_out.begin(), exp_out.end());
  for (int i = 0; i < out.size(); ++i) {
    assert(out[i] == exp_out[i]);
  }
}

int main() {
  checkSolution("25525511135", {"255.255.11.135", "255.255.111.35"});
  checkSolution("101023", {
      "1.0.10.23",
      "1.0.102.3",
      "10.1.0.23",
      "10.10.2.3",
      "101.0.2.3"
      });

  return 0;
}
