// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

int search(int* nums, int numsSize, int target) {
  if (numsSize == 0) {
    return -1;
  }

  int left = 0;
  int right = numsSize - 1;
  int middle;

  while (left <= right) {
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
  return -1;
}

int main() {
  return 0;
}
