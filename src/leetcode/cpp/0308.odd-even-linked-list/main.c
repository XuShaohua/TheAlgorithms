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

struct ListNode* oddEvenList(struct ListNode* head) {
  if (head == NULL || head->next == NULL) {
    return NULL;
  }
  struct ListNode* even_head = head->next;
  struct ListNode* even = head->next;
  struct ListNode* odd = head;
  struct ListNode* old_next = NULL;

  while (odd != NULL && odd->next != NULL) {
    printf("odd: %d\n", odd->val);
    old_next = odd->next;
    odd->next = old_next->next;
    odd = old_next->next;
  }

  /*
  while (even != NULL && even->next != NULL) {
    old_next = even->next;
    even->next = old_next->next;
  }
  */

  assert(odd != NULL);
  //odd->next = even_head;
  //even->next = NULL;
  return head;
}

void checkSolution(int arr[], int len) {
  struct ListNode* list = listFromArray(arr, len);
  listDebug(list);
  list = oddEvenList(list);
  listDebug(list);
  listFree(list);
}

int main() {
  int arr1[] = {1, 2, 3, 4, 5, 6, 7};
  checkSolution(arr1, sizeof(arr1) / sizeof(arr1[0]));

  //int arr2[] = {2, 1, 3, 5, 6, 4, 7};
  //checkSolution(arr2, sizeof(arr2) / sizeof(arr2[0]));
  return 0;
}
