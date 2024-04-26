// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <cassert>
#include <cstdio>

#include <vector>

class Solution {
 public:
  static
  bool canPlaceFlowers(std::vector<int>& flowerbed, int n) {
    if (n == 0) {
      return true;
    }

    flowerbed.insert(flowerbed.begin(), 0);
    flowerbed.push_back(0);
    int i = 1;
    for (i = 1; i < flowerbed.size() - 1; ++i) {
      if (flowerbed.at(i) == 1) {
        i += 1;
      } else {
        if (flowerbed.at(i - 1) == 0 && flowerbed.at(i + 1) == 0) {
          flowerbed[i] = 1;
          n -= 1;
          if (n == 0) {
            break;
          }
        }
      }
    }

    return n == 0;
  }
};


void checkSolution() {
  {
    std::vector<int> flowerbed = {1,0,0,0,1};
    assert(Solution::canPlaceFlowers(flowerbed, 1));
  }

  {
    std::vector<int> flowerbed = {1,0,0,0,1};
    assert(!Solution::canPlaceFlowers(flowerbed, 2));
  }

  {
    std::vector<int> flowerbed = {1,0,0,0,0,1};
    assert(!Solution::canPlaceFlowers(flowerbed, 2));
  }

  {
    std::vector<int> flowerbed = {0,1,0,1,0,1,0,0};
    assert(Solution::canPlaceFlowers(flowerbed, 1));
  }

  {
    std::vector<int> flowerbed = {1,0,0,1};
    assert(!Solution::canPlaceFlowers(flowerbed, 1));
  }

  {
    std::vector<int> flowerbed = {0,0,1,0,0};
    assert(Solution::canPlaceFlowers(flowerbed, 1));
  }
}

int main() {
  checkSolution();
  return 0;
}
