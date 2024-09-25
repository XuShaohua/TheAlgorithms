// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <cassert>

#include <algorithm>
#include <iostream>
#include <vector>

void solution() {
  int n = 0;
  std::cin >> n;
  assert(n > 0);
  int k = 0;
  std::cin >> k;
  assert(k > 0);

  std::string line;
  for (int i = 1; i <= n; ++i) {
    line += i + '0';
  }

  int count = 1;
  while (count < k && std::next_permutation(line.begin(), line.end())) {
    count += 1;
  }
  std::cout << line << std::endl;
}

int main() {
  solution();
  return 0;
}
