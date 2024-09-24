// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <cassert>

int gcd(int a, int b) {
  assert(a >= b);
  if (b == 0) {
    return a;
  } else {
    return gcd(b, a % b);
  }
} 

int lcm(int a, int b) {
  assert(a >= b);
  return a / gcd(a, b) * b;
}
