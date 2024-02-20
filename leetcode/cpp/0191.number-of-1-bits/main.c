// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <stdint.h>

int hammingWeight(uint32_t n) {
  int count = 0;
  while (n != 0) {
    count += n % 2;
    n /= 2;
  }
  return count;
}

int main() {
  return 0;
}
