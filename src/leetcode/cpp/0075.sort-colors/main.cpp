// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <cassert>
#include <cstdio>

#include <algorithm>
#include <vector>

class Solution {
 public:
  static void sortColors(std::vector<int>& nums) {
    int low = 0;
    int mid = 0;
    int high = nums.size() - 1;

    while (mid <= high) {
      if (nums[mid] == 0) {
        std::swap(nums[low], nums[mid]);
        low += 1;
        mid += 1;
      } else if (nums[mid] == 2) {
        std::swap(nums[mid], nums[high]);
        high -= 1;
      } else {
        mid += 1;
      }
    }
  }

};


void checkSolution() {
  //std::vector<int> vec = {2,0,2,1,1,0};
  //std::vector<int> vec = {2,0,1};
  std::vector<int> vec = {1, 2, 0};
  Solution::sortColors(vec);
  for (int i : vec) {
    printf("%d, ", i);
  }
  printf("\n");
}

int main() {
  checkSolution();
  return 0;
}
