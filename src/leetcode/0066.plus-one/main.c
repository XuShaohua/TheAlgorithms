// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int* plusOne(int* digits, int digitsSize, int* returnSize) {
  int carry = 1;
  int new_val = 0;
  for (int i = digitsSize - 1; i >= 0; --i) {
    new_val = carry + digits[i];
    digits[i] = new_val % 10;
    carry = new_val / 10;
  }

  int* out_digits;
  if (carry != 0) {
    *returnSize = digitsSize + 1;
    out_digits = (int*)malloc(*returnSize * sizeof(int));
    assert(out_digits);
    memcpy(out_digits + 1, digits, digitsSize * sizeof(int));
    out_digits[0] = carry;
  } else {
    *returnSize = digitsSize;
    out_digits = (int*)malloc(*returnSize * sizeof(int));
    assert(out_digits);
    memcpy(out_digits, digits, digitsSize * sizeof(int));
  }
  return out_digits;
}

void checkSolution(int* digits, int digitsSize, int expectedReturnSize) {
  int returnSize;
  int* out_digits = plusOne(digits, digitsSize, &returnSize);
  assert(returnSize == expectedReturnSize);
  for (int i = 0; i < returnSize; ++i) {
    printf("%d, ", out_digits[i]);
  }
  printf("\n");
}

int main() {
  int arr1[] = {1, 2, 3};
  checkSolution(arr1, sizeof(arr1) / sizeof(arr1[0]), 3);

  int arr2[] = {4, 3, 2, 1};
  checkSolution(arr2, sizeof(arr2) / sizeof(arr2[0]), 4);

  int arr3[] = {9};
  checkSolution(arr3, sizeof(arr3) / sizeof(arr3[0]), 2);

  return 0;
}
