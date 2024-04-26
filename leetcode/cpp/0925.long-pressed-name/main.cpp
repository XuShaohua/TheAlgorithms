// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <cassert>
#include <cstdio>

#include <string>

class Solution {
 public:
  static bool isLongPressedName(std::string name, std::string typed) {
    assert(!name.empty() && !typed.empty());

    const size_t m = name.size();
    const size_t n = typed.size();
    size_t i = 0;
    size_t j = 0;
    while (i < m && j < n) {
      if (name[i] == typed[j]) {
        i += 1;
        j += 1;
      } else if (i > 0 && name[i - 1] == typed[j]) {
        j += 1;
      } else {
        return false;
      }
    }

    while (j < n && name[m - 1] == typed[j]) {
      j += 1;
    }

    return i == m && j == n;
  }
};


void checkSolution() {
}

int main() {
  checkSolution();
  return 0;
}
