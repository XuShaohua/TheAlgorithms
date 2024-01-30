// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <stdio.h>
#include <stdlib.h>
#include <assert.h>

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
    struct ListNode* tmp = list;
    list = list->next;
    free(tmp);
  }
}

static int listSize(struct ListNode* list) {
  assert(list != NULL);
  int len = 0;
  while (list != NULL) {
    len += 1;
    list = list->next;
  }
  return len;
}

struct ListNode* rotateRight(struct ListNode* head, int k) {
  if (head == NULL) {
    return NULL;
  }

  assert(k >= 0);

  //const int len = listSize(head);
  int len = 1;
  struct ListNode* tail = head;
  while (tail != NULL && tail->next != NULL) {
    tail = tail->next;
    len += 1;
  }

  k = len - (k % len);
  if (k == 0) {
    return head;
  }

  // Connect tail to head.
  tail->next = head;

  // Count node from head to tail.
  while (k > 0) {
    head = head->next;
    tail = tail->next;
    k -= 1;
  }
  // Break the ring at k point.
  tail->next = NULL;

  return head;
}

struct ListNode* rotateRight2(struct ListNode* head, int k) {
  if (head == NULL) {
    return NULL;
  }
  assert(k >= 0);

  struct ListNode* fast = head;
  struct ListNode* slow = head;
  // FIXME(Shaohua): k might > len
  while (k > 0) {
    fast = fast->next;
    k -= 1;
  }

  while (fast != NULL && fast->next != NULL) {
    fast = fast->next;
    slow = slow->next;
  }
  // Connect tail to head node.
  fast->next = head;
  // Now slow is the new tail node.
  head = slow->next;
  slow->next = NULL;

  return head;
}

static void checkResult(int arr[], int len, int k) {
  struct ListNode* head = listFromArray(arr, len);
  struct ListNode* new_head = rotateRight(head, k);
  //struct ListNode* new_head = rotateRight2(head, k);
  head = new_head;
  while (head != NULL) {
    printf("%d, ", head->val);
    head = head->next;
  }
  printf("\n");
  free(new_head);
}

int main() {
  int arr1[] = {1,2,3,4,5}; 
  checkResult(arr1, sizeof(arr1) / sizeof(arr1[0]), 2);

  int arr2[] = {1,2}; 
  checkResult(arr2, sizeof(arr2) / sizeof(arr2[0]), 0);

  int arr3[] = {1};
  checkResult(arr3, sizeof(arr3) / sizeof(arr3[0]), 10);

  return 0;
}
