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

struct ListNode* listFromArray(int arr[], int len) {
  struct ListNode* list = NULL;
  for (int i = len - 1; i >= 0; --i) {
    struct ListNode* new_node = malloc(sizeof(struct ListNode));
    assert(new_node != NULL);
    new_node->val = arr[i];
    new_node->next = list;
    list = new_node;
  }
  return list;
}

struct ListNode* addTwoNumbers(struct ListNode* l1, struct ListNode* l2) {
  struct ListNode* l3 = NULL;
  struct ListNode* tail_node = NULL;
  int carry = 0;
  while (l1 != NULL || l2 != NULL || carry != 0) {
    struct ListNode* new_node = malloc(sizeof(struct ListNode));
    assert(new_node != NULL);
    int sum = carry;
    if (l1 != NULL) {
      sum += l1->val;
      l1 = l1->next;
    }
    if (l2 != NULL) {
      sum += l2->val;
      l2 = l2->next;
    }
    carry = sum / 10;
    new_node->val = sum % 10;
    new_node->next = NULL;
    if (tail_node == NULL) {
      l3 = new_node;
      tail_node = new_node;
    } else {
      tail_node->next = new_node;
      tail_node = new_node;
    }
  }
  return l3;
}

int main() {
  int arr1[] = {2, 4, 3};
  int arr2[] = {5, 6, 4};
  struct ListNode* l1 = listFromArray(arr1, 3);
  struct ListNode* l2 = listFromArray(arr2, 3);
  struct ListNode* l3 = addTwoNumbers(l1, l2);
  while (l3 != NULL) {
    printf("%d, ", l3->val);
    l3 = l3->next;
  }
  printf("\n");

  return 0;
}
