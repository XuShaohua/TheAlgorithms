// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <assert.h>
#include <limits.h>
#include <stdbool.h>
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

struct ListNode* listConnectTailTo(struct ListNode* head, int pos) {
  assert(head != NULL);
  assert(pos > 0);

  struct ListNode* curr = head;
  struct ListNode* pos_node = NULL;
  while (pos > 0) {
    curr = curr->next;
    pos -= 1;
  }
  assert(curr != NULL);
  pos_node = curr;
  while (curr->next != NULL) {
    curr = curr->next;
  }
  curr->next = pos_node;

  return head;
}

void listFree(struct ListNode* list) {
  while (list != NULL) {
    struct ListNode* tmp = list;
    list = list->next;
    free(tmp);
  }
}

bool hasCycle(struct ListNode *head) {
  struct ListNode* alert_node = listNew(INT_MIN);
  struct ListNode* tmp = NULL;
  while (head != NULL) {
    if (head->next == alert_node) {
      return true;
    }
    tmp = head;
    head = head->next;
    tmp->next = alert_node;
  }
  return false;
}

bool hasCycle2(struct ListNode *head) {
  const int kResetVal = INT_MIN;
  while (head != NULL) {
    if (head->val == kResetVal) {
      return true;
    }
    head->val = kResetVal;
    head = head->next;
  }
  return false;
}

// Floyd’s Cycle-Finding Algorithm
bool hasCycle3(struct ListNode *head) {
  struct ListNode* fast = head;
  struct ListNode* slow = head;

  while (fast != NULL && fast->next != NULL) {
    // 2 nodes forward
    fast = fast->next->next;
    // 1 node forward
    slow = slow->next;

    // Got the same node.
    if (fast == slow) {
      return true;
    }
  }
  return false;
}

void checkSolution(int arr[], int len, int pos) {
  struct ListNode* list = listFromArray(arr, len);
  struct ListNode* list2 = listConnectTailTo(list, pos);
  bool ret = hasCycle(list2);
  printf("has cycle: %d\n", ret);
}

int main() {
  int arr[] = {3, 2, 0, -4};
  checkSolution(arr, sizeof(arr) / sizeof(arr[0]), 1);

  return 0;
}
