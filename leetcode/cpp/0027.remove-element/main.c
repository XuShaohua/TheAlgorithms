// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

int removeElement(int* nums, int numsSize, int val) {
  for (int i = 0; i < numsSize; /* empty */) {
    if (nums[i] == val) {
      // Implements swap_remove()
      nums[i] = nums[numsSize - 1];
      numsSize -= 1;
    } else {
      i += 1;
    }
  }
  return numsSize;
}

void checkSolution(int* nums, int numsSize, int val, int expected_k) {
  int k = removeElement(nums, numsSize, val);
  for (int i = 0; i < k; ++i) {
    printf("%d, ", nums[i]);
  }
  printf("\n");
  assert(k == expected_k);
}

int main() {
  int arr1[] = {0, 1, 2, 2, 3, 0, 4, 2};
  checkSolution(arr1, sizeof(arr1) / sizeof(arr1[0]), 2, 5);

  int arr2[] = {3, 2, 2, 3};
  checkSolution(arr2, sizeof(arr2) / sizeof(arr2[0]), 3, 2);

  return 0;
}
