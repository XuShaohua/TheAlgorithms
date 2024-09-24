// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <queue>
#include <unordered_map>
#include <vector>

class Solution {
 public:
  std::vector<int> topKFrequent(std::vector<int>& nums, int k) {
    // 先计算各数值出现的频率
    // (number, freq)
    std::unordered_map<int, size_t> freqs;
    for (int num: nums) {
      freqs[num] += 1;
    }

    // 再将它们存入到 priority_queue, 它是个最大堆.
    // (freq, number), 以降序排列
    std::priority_queue<std::pair<size_t, int>> queue;
    for (const auto& pair : freqs) {
      queue.emplace(pair.second, pair.first);
    }

    // 最后导出为 vector
    std::vector<int> out;
    while (!queue.empty() && out.size() < k) {
      out.emplace_back(queue.top().second);
      queue.pop();
    }

    return out;
  }
};

int main() {
  return 0;
}
