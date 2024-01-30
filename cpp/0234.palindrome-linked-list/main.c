// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <assert.h>
#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>

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

bool isPalindrome(struct ListNode* head) {
  if (head == NULL) {
    return true;
  }

  // 1. Reverse left half of list.
  struct ListNode* slow = head;
  struct ListNode* fast = head;
  struct ListNode* next = NULL;
  struct ListNode* prev = NULL;

  while (fast != NULL && fast->next != NULL) {
    slow = slow->next;
    fast = fast->next->next;

    // Reverse via slow node.
    next = head->next;
    head->next = prev;
    prev = head;
    head = next;
  }

  if (fast != NULL) {
    slow = slow->next;
  }
  head = prev;

  // 2. Compare from middle.
  while (slow != NULL) {
    if (slow->val != head->val) {
      return false;
    }
    head = head->next;
    slow = slow->next;
  }
  return true;
}

void checkSolution(int arr[], int len) {
  struct ListNode* list = listFromArray(arr, len);
  listDebug(list);
  printf("is palindrome: %d\n", isPalindrome(list));
  listFree(list);
}

int main() {
  int arr1[] = {1, 2, 2, 1};
  checkSolution(arr1, sizeof(arr1) / sizeof(arr1[0]));

  int arr2[] = {1, 2,};
  checkSolution(arr2, sizeof(arr2) / sizeof(arr2[0]));

  return 0;
}
