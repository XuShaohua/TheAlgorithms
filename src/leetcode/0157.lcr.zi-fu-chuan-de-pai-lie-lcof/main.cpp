// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <cassert>

#include <algorithm>
#include <string>
#include <vector>

class Solution {
 public:
  std::vector<std::string> goodsOrder(std::string goods) {
    std::sort(goods.begin(), goods.end());
    std::vector<std::string> ans{goods};
    while(std::next_permutation(goods.begin(), goods.end())) {
      ans.emplace_back(goods);
    }
    return ans;
  }
};


int main() {
  return 0;
}
