// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <cassert>

#include <iostream>
#include <vector>

int solution(int n) {
  // 解题思路:
  // 1. 先计算 1..n 之间的所有质数
  // 2. 创建一个数组, 用于存放每个质因数的最大出现次数
  // 3. 遍历 1..n 之间所有整数, 进行因式分解, 然后更新最大质因数数组
  // 4. 将最大质因数数组中的值相乘, 即可得到结果.

  // 得到质数
  const int max_num = n + 1;
  std::vector<bool> primes(max_num, true);
  for (int i = 2; i <= n; ++i) {
    if (!primes[i]) {
      continue;
    }
    for (int j = i + 1; j <= n; ++j) {
      if (j % i == 0) {
        primes[j] = false;
      }
    }
  }

  std::vector<int> max_factors(max_num, 0);
  // 遍历所有整数
  for (int num = 2; num <= n; ++num) {
    // 遍历所有质数
    int rem = num;
    for (int j = 2; j <= num; ++j) {
      if (!primes[j]) {
        continue;
      }

      // 确定当前质数作为质因数的次数.
      int factor_count = 0;
      while (rem % j == 0) {
        rem /= j;
        factor_count += 1;
      }

      // 更新质因数的最大个数
      if (factor_count > max_factors[j]) {
        max_factors[j] = factor_count;
      }

      // 当前整数 num 已经被除尽了
      if (rem == 0) {
        break;
      }
    }
  }

  // 最后, 计算最大质因数的积.
  int ans = 1;
  const int kModular = 987654321;
  for (int num = 2; num <= n; ++num) {
    if (max_factors[num] > 0) {
      for (int i = 0; i < max_factors[num]; ++i) {
        ans = ans * num % kModular;
      }
    }
  }

  return ans;
}

void check_solution() {
  assert(solution(3) == 6);
}

int main() {
  check_solution();
  return 0;
}
