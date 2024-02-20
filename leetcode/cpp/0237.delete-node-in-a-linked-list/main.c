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
    struct ListNode* node = head;
    head = head->next;
    free(node);
  }
}

void listDebug(struct ListNode* head) {
  while (head != NULL) {
    printf("%d, ", head->val);
    head = head->next;
  }
  printf("\n");
}

void deleteNode(struct ListNode* node) {
  assert(node != NULL);
  assert(node->next != NULL);
  struct ListNode* tmp = node->next;
  node->val = tmp->val;
  node->next = tmp->next;
  free(tmp);
}

void checkSolution(int arr[], int len, int target) {
  struct ListNode* list = listFromArray(arr, len);
  struct ListNode* node = list;
  for (int i = 0; i < target; ++i) {
    node = node->next;
  }
  deleteNode(node);
  listDebug(list);
  listFree(list);
}

int main() {
  int arr[] = {4, 5, 1, 9};
  checkSolution(arr, sizeof(arr) / sizeof(arr[0]), 2);

  return 0;
}
