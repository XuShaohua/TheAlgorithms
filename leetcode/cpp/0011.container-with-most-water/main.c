// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

int maxArea(int* height, int heightSize) {
  assert(heightSize >= 2);
  int max_area = 0;
  int left = 0;
  int right = heightSize - 1;
  int area = 0;

  while (left < right) {
    if (height[left] < height[right]) {
      area = (right - left) * height[left];
      left += 1;
    } else {
      area = (right - left) * height[right];
      right -= 1;
    }
    max_area = (max_area < area) ? area : max_area;
  }

  return max_area;
}

void checkSolution(int* height, int heightSize, int expected_area) {
  int area = maxArea(height, heightSize);
  printf("area: %d, expected: %d\n", area, expected_area);
  assert(area == expected_area);
}

int main() {
  int heigh1[] = {1, 8, 6, 2, 5, 4, 8, 3, 7};
  checkSolution(heigh1, sizeof(heigh1) / sizeof(heigh1[0]), 49);

  int height2[] = {1, 1};
  checkSolution(height2, sizeof(height2) / sizeof(height2[0]), 1);

  return 0;
}
