// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <assert.h>
#include <stdlib.h>
#include <stdio.h>

int removeDuplicates(int* nums, int numsSize) {
  int* slow = nums;
  int* fast = nums;

  for (int i = 0; i < numsSize; ++i) {
    // Skip duplicate elements
    while (i+1 < numsSize && fast[i] == fast[i+1]) {
      i += 1;
    }
    *slow = fast[i];
    slow += 1;
  }

  return slow - nums;
}

void checkSolution(int* nums, int numsSize) {
  int k = removeDuplicates(nums, numsSize);
  for (int i = 0; i < k; ++i) {
    printf("%d, ", nums[i]);
  }
  printf("\n");
}

int main() {
  int arr1[] = {0,0,1,1,1,2,2,3,3,4};
  checkSolution(arr1, sizeof(arr1) / sizeof(arr1[0]));

  int arr2[] = {1, 1, 2};
  checkSolution(arr2, sizeof(arr2) / sizeof(arr2[0]));

  return 0;
}
