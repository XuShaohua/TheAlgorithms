// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include <assert.h>
#include <stddef.h>
#include <math.h>

static double sum_of_polynomial(const double* arr, size_t arr_len, double x) {
  double p = 0.0;
  for (size_t i = 0; i < arr_len; ++i) {
    p += arr[i] * pow(x, (double)i);
  }
  return p;
}

static double sum_of_polynomial2(const double* arr, size_t arr_len, double x) {
  if (arr_len == 0) {
    return 0.0;
  }

  double p = 0.0;
  for (int i = arr_len - 1; i >= 0; --i) {
    p = arr[i] + x * p;
  }

  return p;
}

int main(void) {
  const size_t kArrayLen = 9;
  const double arr[] = {1, 2, 3, 4, 5, 6, 7, 8, 9};
  const double x = 12.4;
  assert(sum_of_polynomial(arr, kArrayLen, x) == 5418501173.194776);
  assert(sum_of_polynomial2(arr, kArrayLen, x) == 5418501173.1947775);

  return 0;
}