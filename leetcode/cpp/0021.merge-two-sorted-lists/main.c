// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <stdio.h>
#include <assert.h>
#include <stdlib.h>

struct ListNode {
  int val;
  struct ListNode *next;
};

struct ListNode* listFromArray(int arr[], int len) {
  struct ListNode* list = NULL;
  for (int i = len - 1; i >= 0; --i) {
    struct ListNode* node = malloc(sizeof(struct ListNode));
    assert(node != NULL);
    node->val = arr[i];
    node->next = list;
    list = node;
  }
  return list;
}

static
struct ListNode* listReverse(struct ListNode* list) {
  assert(list != NULL);
  struct ListNode* prev = NULL;
  struct ListNode* next = NULL;
  while (list != NULL) {
    next = list->next;
    list->next = prev;
    prev = list;
    list = next;
  }
  return prev;
}

struct ListNode* mergeTwoLists(struct ListNode* l1, struct ListNode* l2) {
  if (l1 == NULL) {
    return l2;
  }
  if (l2 == NULL) {
    return l1;
  }
  l1 = listReverse(l1);
  l2 = listReverse(l2);

  struct ListNode* l3 = NULL;
  struct ListNode* node = NULL;

  while (l1 != NULL && l2 != NULL) {
    if (l1->val >= l2->val) {
      node = l1;
      l1 = l1->next;
    } else {
      node = l2;
      l2 = l2->next;
    }
    node->next = l3;
    l3 = node;
  }
  while (l1 != NULL) {
    node = l1;
    l1 = l1->next;
    node->next = l3;
    l3 = node;
  }
  while (l2 != NULL) {
    node = l2;
    l2 = l2->next;
    node->next = l3;
    l3 = node;
  }

  return l3;
}

int main() {
  int arr1[] = {1, 2, 4};
  int arr2[] = {1, 3, 4};
  struct ListNode* l1 = listFromArray(arr1, sizeof(arr1) / sizeof(arr1[0]));
  struct ListNode* l2 = listFromArray(arr2, sizeof(arr2) / sizeof(arr2[0]));
  struct ListNode* l3 = mergeTwoLists(l1, l2);
  while (l3 != NULL) {
    printf("%d, ", l3->val);
    l3 = l3->next;
  }
  printf("\n");

  return 0;
}
