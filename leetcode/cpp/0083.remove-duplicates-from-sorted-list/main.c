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

static void listFree(struct ListNode* list) {
  while (list != NULL) {
    struct ListNode* tmp = list;
    list = list->next;
    free(tmp);
  }
}

static struct ListNode* listFromArray(int arr[], int len) {
  assert(len >= 0);
  struct ListNode* list;
  for (int i = len - 1; i >= 0; --i) {
    struct ListNode* node = listNew(arr[i]);
    node->next = list;
    list = node;
  }
  return list;
}

static struct ListNode* deleteDuplicates(struct ListNode* head) {
  if (head == NULL) {
    return NULL;
  }

  struct ListNode* current = head;
  struct ListNode* previous = NULL;
  while (current != NULL) {
    if (previous != NULL && previous->val == current->val) {
      // Remove current node.
      previous->next = current->next;
      struct ListNode* tmp = current;
      current = current->next;
      free(tmp);
    } else {
      previous = current;
      current = current->next;
    }
  }

  return head;
}

static void checkSolution(int arr[], int len) {
  struct ListNode* list = listFromArray(arr, len);
  struct ListNode* list_uniq = deleteDuplicates(list);
  while (list_uniq != NULL) {
    printf("%d, ", list_uniq->val);
    list_uniq = list_uniq->next;
  }
  printf("\n");

  listFree(list_uniq);
}

int main() {
  int arr1[] = {1, 1, 2};
  checkSolution(arr1, sizeof(arr1) / sizeof(arr1[0]));

  int arr2[] = {1, 1, 2, 3, 3};
  checkSolution(arr2, sizeof(arr2) / sizeof(arr2[0]));
  return 0;
}
