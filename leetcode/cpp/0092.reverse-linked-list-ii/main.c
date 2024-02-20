// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

struct ListNode {
  int val;
  struct ListNode* next;
};

static struct ListNode* listNew(int val) {
  struct ListNode* list = malloc(sizeof(struct ListNode));
  assert(list != NULL);
  list->val = val;
  list->next = NULL;
  return list;
}

static struct ListNode* listFromArray(int arr[], int len) {
  assert(len >= 0);
  struct ListNode* list = NULL;
  for (int i = len - 1; i >= 0; --i) {
    struct ListNode* node = listNew(arr[i]);
    node->next = list;
    list = node;
  }
  return list;
}

static void listFree(struct ListNode* list) {
  assert(list != NULL);
  while (list != NULL) {
    struct ListNode* node = list;
    list = list->next;
    free(node);
  }
}

static struct ListNode* reverseBetween(struct ListNode* head, int left, int right) {
  assert(head != NULL);
  assert(left >= 1 && left <= right);
  if (left == right) {
    return head;
  }
  left -= 1;
  right -= 1;

  struct ListNode* range_prev = NULL;
  struct ListNode* range_start = NULL;
  struct ListNode* range_end = NULL;
  struct ListNode* curr = head;
  struct ListNode* prev = NULL;
  struct ListNode* next = NULL;

  int index = 0;
  while (index < left) {
    range_prev = curr;
    curr = curr->next;
    index += 1;
  }

  range_start = curr;

  while (index < right) {
    // switch direction
    next = curr->next;
    curr->next = prev;
    prev = curr;
    curr = next;
    index += 1;
  }
  range_end = curr;
  if (range_prev == NULL) {
    head = range_end;
  } else {
    range_prev->next = range_end;
  }
  range_start->next = range_end->next;
  range_end->next = prev;

  return head;
}

static void checkSolution(int arr[], int len, int left, int right) {
  struct ListNode* list = listFromArray(arr, len);
  struct ListNode* reversed = reverseBetween(list, left, right);
  list = reversed;
  while (reversed != NULL) {
    printf("%d, ", reversed->val);
    reversed = reversed->next;
  }
  printf("\n");
  listFree(list);
}

int main() {
  int arr1[] = {1, 2, 3, 4, 5}; 
  checkSolution(arr1, sizeof(arr1) / sizeof(arr1[0]), 2, 4);

  int arr2[] = {5}; 
  checkSolution(arr2, sizeof(arr2) / sizeof(arr2[0]), 1, 1);

  int arr3[] = {3, 5}; 
  checkSolution(arr3, sizeof(arr3) / sizeof(arr3[0]), 1, 2);

  return 0;
}
