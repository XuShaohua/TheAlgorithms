// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

// Binary search
int searchInsert(int* nums, int numsSize, int target) {
  if (nums[numsSize - 1] < target) {
    return numSize;
  }

  int left = 0;
  int right = numsSize - 1;
  int middle = 0;
  while (left < right) {
    middle = left + (right - left) / 2;
    if (nums[middle] == target) {
      return middle;
    }
    if (nums[middle] > target) {
      right = middle - 1;
    } else {
      left = middle + 1;
    }
  }
  return right;
}

void checkSolution(int* nums, int numsSize, int target, int expected) {
  int out = searchInsert(nums, numsSize, target);
  printf("out: %d, expected: %d\n", out, expected);
  assert(out == expected);
}
int main() {
  int arr1[] = {1, 3, 5, 6};
  checkSolution(arr1, sizeof(arr1) / sizeof(arr1[0]), 5, 2);

  int arr2[] = {1, 3, 5, 6};
  checkSolution(arr2, sizeof(arr2) / sizeof(arr2[0]), 2, 1);

  int arr3[] = {1, 3, 5, 6};
  checkSolution(arr3, sizeof(arr3) / sizeof(arr3[0]), 7, 4);

  return 0;
}
