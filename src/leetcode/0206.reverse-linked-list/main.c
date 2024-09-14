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

struct ListNode* listNew(int val) {
  struct ListNode* node = malloc(sizeof(struct ListNode));
  assert(node != NULL);
  node->val = val;
  node->next = NULL;
  return node;
}

struct ListNode* listFromArray(int arr[], int len) {
  struct ListNode* list = NULL;
  for (int i = len - 1; i >= 0; --i) {
    struct ListNode* node = listNew(arr[i]);
    node->next = list;
    list = node;
  }
  return list;
}

void listFree(struct ListNode* head) {
  while (head != NULL) {
    struct ListNode* tmp = head;
    head = head->next;
    free(tmp);
  }
}

void listDebug(struct ListNode* head) {
  while (head != NULL) {
    printf("%d, ", head->val);
    head = head->next;
  }
  printf("\n");
}

struct ListNode* reverseList(struct ListNode* head) {
  if (head == NULL) {
    return NULL;
  }
  struct ListNode* prev = NULL;
  struct ListNode* next = NULL;
  while (head != NULL) {
    next = head->next;
    head->next = prev;
    prev = head;
    head = next;
  }
  return prev;
}

void checkSolution(int arr[], int len) {
  struct ListNode* list = listFromArray(arr, len);
  listDebug(list);
  list = reverseList(list);
  listDebug(list);
  listFree(list);
}

int main() {
  int arr1[] = {1, 2, 3, 4, 5};
  checkSolution(arr1, sizeof(arr1) / sizeof(arr1[0]));

  int arr2[] = {1, 2};
  checkSolution(arr2, sizeof(arr2) / sizeof(arr2[0]));

  return 0;
}
