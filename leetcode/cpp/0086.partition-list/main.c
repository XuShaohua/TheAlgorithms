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
  struct ListNode* node = malloc(sizeof(struct ListNode));
  assert(node != NULL);
  node->val = val;
  node->next = NULL;
  return node;
}

static struct ListNode* listFromArray(int arr[], int len) {
  assert(len >= 0);
  struct ListNode* head = NULL;
  for (int i = len - 1; i >= 0; --i) {
    struct ListNode* node = listNew(arr[i]);
    node->next = head;
    head = node;
  }

  return head;
}

static void listFree(struct ListNode* head) {
  while (head != NULL) {
    struct ListNode* tmp = head;
    head = head->next;
    free(tmp);
  }
}

static struct ListNode* partition(struct ListNode* head, int x) {
  if (head == NULL) {
    return NULL;
  }

  struct ListNode* small_head = NULL;
  struct ListNode* small_tail = NULL;
  struct ListNode* large_head = NULL;
  struct ListNode* large_tail = NULL;
  struct ListNode* tmp = NULL;
  while (head != NULL) {
    tmp = head;
    if (tmp->val < x) {
      if (small_tail == NULL) {
        small_head = tmp;
        small_tail = tmp;
      } else {
        small_tail->next = tmp;
        small_tail = tmp;
      }
    } else {
      if (large_tail == NULL) {
        large_head = tmp;
        large_tail = tmp;
      } else {
        large_tail->next = tmp;
        large_tail = tmp;
      }
    }

    // Go to next node.
    head = head->next;
  }

  if (small_tail == NULL) {
    return large_head;
  }
  // Connect two parts.
  small_tail->next = large_head;
  if (large_tail != NULL) {
    large_tail->next = NULL;
  }

  return small_head;
}

static void checkSolution(int arr[], int len, int x) {
  struct ListNode* list = listFromArray(arr, len);
  struct ListNode* list2 = partition(list, x);
  list = list2;

  while (list2 != NULL) {
    printf("%d, ", list2->val);
    list2 = list2->next;
  }
  printf("\n");

  listFree(list);
}

int main() {
  int arr1[] = {1,4,3,2,5,2};
  checkSolution(arr1, sizeof(arr1) / sizeof(arr1[0]), 3);

  int arr2[] = {2, 1};
  checkSolution(arr2, sizeof(arr2) / sizeof(arr2[0]), 2);

  return 0;
}
