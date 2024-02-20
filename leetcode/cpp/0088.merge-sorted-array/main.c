// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

void merge(int* nums1, int nums1Size, int m, int* nums2, int nums2Size, int n) {
  // Merge from end of nums1
  int i1 = m - 1;
  int i2 = n - 1;
  int index = m + n - 1;
  while (i1 >= 0 && i2 >= 0) {
    if (nums1[i1] > nums2[i2]) {
      nums1[index] = nums1[i1];
      i1 -= 1;
    } else {
      nums1[index] = nums2[i2];
      i2 -= 1;
    }
    index -= 1;
  }
  while (i1 >= 0) {
    nums1[index] = nums1[i1];
    index -= 1;
    i1 -= 1;
  }
  while (i2 >= 0) {
    nums1[index] = nums2[i2];
    index -= 1;
    i2 -= 1;
  }
}

int main() {
  return 0;
}
