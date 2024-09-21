// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <cassert>

#include <iostream>
#include <sstream>
#include <string>
#include <vector>

// 滑动窗口
void solution() {
  // 读取输入
  std::string line;
  std::cin >> line;
  std::stringstream ss(line);
  std::string part;
  std::vector<int> nums;
  while (std::getline(ss, part, ',')) {
    const int num = std::stoi(part);
    nums.push_back(num);
  }
  assert(1 <= nums.size() && nums.size() <= 200);

  int expected_sum;
  std::cin >> expected_sum;
  assert(1 <= expected_sum);

  // 最长子序的长度
  int subarray_max_len = -1;
  // 用于控制窗口左侧两侧的位置
  int left = 0;
  int right = 0;
  // 计算当前子序列的和
  int current_sum = 0;
  while (left <= right && right < nums.size()) {
    if (current_sum < expected_sum) {
      // 子序列的和太小, 将窗口右侧向右移
      current_sum += nums[right];
      right += 1;
    } else if (current_sum == expected_sum) {
      // 和相等, 计算当前的子序列长度
      const int current_length = right - left;
      subarray_max_len = std::max(subarray_max_len, current_length);
      current_sum += nums[right];
      right += 1;
    } else {
      // 子序列的和太大, 将窗口左侧向右移
      current_sum -= nums[left];
      left += 1;
    }
  }

  // 打印结果
  std::cout << subarray_max_len << std::endl;
}

int main() {
  solution();

  return 0;
}
