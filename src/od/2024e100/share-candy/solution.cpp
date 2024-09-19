// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <iostream>
#include <unordered_map>

// 缓存 + 递归
int get_minimum_times(int num, std::unordered_map<int, int>& cache) {
  const auto iter = cache.find(num);
  if (iter != cache.end()) {
    return iter->second;
  }

  if (num % 2 == 0) {
    // 如果是偶数个
    // 平均分一次
    const int times = 1 + get_minimum_times(num / 2, cache);
    cache[num] = times;
    return times;
  } else {
    // 如果是奇数个, 有两种方式:
    // 取一个
    const int times1 = 1 + get_minimum_times(num + 1, cache);
    // 放一个
    const int times2 = 1 + get_minimum_times(num - 1, cache);
            
    // 求它们的最小值
    const int min_times = std::min(times1, times2);
    cache[num] = min_times;
    return min_times;
  }
}

int main() {
  std::unordered_map<int, int> cache;
  cache[1] = 0;

  int num_candies = 0;
  std::cin >> num_candies;

  const int times = get_minimum_times(num_candies, cache);
  std::cout << times << std::endl;

  return 0;
}
