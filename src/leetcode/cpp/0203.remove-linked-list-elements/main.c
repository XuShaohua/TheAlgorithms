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

void listFree(struct ListNode* list) {
  while (list != NULL) {
    struct ListNode* tmp = list;
    list = list->next;
    free(tmp);
  }
}

struct ListNode* removeElements(struct ListNode* head, int val) {
  if (head == NULL) {
    return NULL;
  }

  struct ListNode* curr = head;
  struct ListNode* prev = NULL;

  while (curr != NULL) {
    if (curr->val == val) {
      // Remove current node.
      if (prev == NULL) {
        assert(curr == head);
        curr = curr->next;
        free(head);
        head = curr;
      } else {
        prev->next = curr->next;
        free(curr);
        curr = prev->next;
      }
    } else {
      prev = curr;
      curr = curr->next;
    }
  }
  return head;
}

struct ListNode* removeElements2(struct ListNode* head, int val) {
  if (head == NULL) {
    return NULL;
  }

  struct ListNode* curr = head->next;
  struct ListNode* prev = head;

  while (curr != NULL) {
    if (curr->val == val) {
      // Remove current node.
      prev->next = curr->next;
      free(curr);
      curr = prev->next;
    } else {
      prev = curr;
      curr = curr->next;
    }
  }
  if (head->val == val) {
    curr = head;
    head = head->next;
    free(curr);
  }
  return head;
}

void checkSolution(int arr[], int len, int val) {
  struct ListNode* list = listFromArray(arr, len);
  struct ListNode* list2 = removeElements2(list, val);
  list = list2;
  while (list != NULL) {
    printf("%d, ", list->val);
    list = list->next;
  }
  printf("\n");
  listFree(list2);
}

int main() {
  int arr[] = {1,2,6,3,4,5,6}; 
  checkSolution(arr, sizeof(arr) / sizeof(arr[0]), 6);

  return 0;
}
