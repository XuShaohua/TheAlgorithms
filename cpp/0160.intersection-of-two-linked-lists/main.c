// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

struct ListNode {
  int val;
  struct ListNode* next;
};

struct ListNode* listSkip(struct ListNode* head, int count) {
  while (count > 0) {
    head = head->next;
    assert(head != NULL);
    count -= 1;
  }
  return head;
}

int listLen(struct ListNode* head) {
  int count = 0;
  while (head != NULL) {
    head = head->next;
    count += 1;
  }
  return count;
}

struct ListNode* getIntersectionNode(struct ListNode* headA, struct ListNode* headB) {
  int len_a = listLen(headA);
  int len_b = listLen(headB);
  if (len_a > len_b) {
    headA = listSkip(headA, len_a - len_b);
  } else {
    headB = listSkip(headB, len_b - len_a);
  }

  while (headA != NULL) {
    if (headA == headB) {
      return headA;
    }
    headA = headA->next;
    headB = headB->next;
  }
  return NULL;
}

// Do not count length of list directly.
// Walk through two rounds instead.
struct ListNode* getIntersectionNode(struct ListNode* headA, struct ListNode* headB) {
  if (headA == NULL || headB == NULL) {
    return NULL;
  }

  struct ListNode* originHeadA = headA;
  struct ListNode* originHeadB = headB;
  bool is_a_end = false;
  bool is_b_end = false;

  while (headA != headB) {
    headA = headA->next;
    headB = headB->next;

    if (headA == NULL && !is_a_end) {
      headA = originHeadB;
      is_a_end = true;
    }

    if (headB == NULL && !is_b_end) {
      headB = originHeadA;
      is_b_end = true;
    }

    if (headA == NULL || headB == NULL) {
      return NULL;
    }
  }
  return headA;
}

int main() {
  return 0;
}
