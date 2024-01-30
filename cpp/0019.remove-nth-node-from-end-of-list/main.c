
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

struct ListNode {
  int val;
  struct ListNode* next;
};

static struct ListNode* listNodeNew(int val) {
  struct ListNode* node = malloc(sizeof(struct ListNode));
  assert(node != NULL);
  node->val = val;
  node->next = NULL;
  return node;
}

static struct ListNode* listFromArray(int arr[], int len) {
  struct ListNode* list = NULL;
  for (int i = len - 1; i >= 0; --i) {
    struct ListNode* node = listNodeNew(arr[i]);
    node->next = list;
    list = node;
  }
  return list;
}

static int listSize(struct ListNode* list) {
  int size = 0;
  while (list != NULL) {
    size += 1;
    list = list->next;
  }
  return size;
}

static struct ListNode* removeNthFromEnd(struct ListNode* head, int n) {
  assert(head != NULL);
  assert(n >= 0);

  struct ListNode* tmp = NULL;
  int size = listSize(head);
  assert(n <= size);

  if (size == n) {
    // Remove head node.
    tmp = head;
    head = head->next;
    free(tmp);
    return head;
  }

  int index = 0;
  struct ListNode* head_clone = head;
  while (index + 1 < size - n && head != NULL) {
    head = head->next;
    index += 1;
  }
  assert(head != NULL);
  if (head->next != NULL) {
    tmp = head->next;
    head->next = head->next->next;
    free(tmp);
  }

  return head_clone;
}

static struct ListNode* removeNthFromEnd2Helper(struct ListNode* current, int n, int* count) {
  // End of list.
  if (current == NULL) {
    return NULL;
  }

  struct ListNode* next = removeNthFromEnd2Helper(current->next, n, count);
  *count += 1;
  if (*count == n) {
    free(current);
    return next;
  }

  current->next = next;
  return current;
}

static struct ListNode* removeNthFromEnd2(struct ListNode* head, int n) {
  int count = 0;
  return removeNthFromEnd2Helper(head, n, &count);
}

static struct ListNode* removeNthFromEnd3(struct ListNode* head, int n) {
  assert(n >= 0);
  if (head == NULL) {
    return NULL;
  }

  // Used for counting.
  struct ListNode* fast = head;
  struct ListNode* slow = head;

  for (int i = 0; i < n; ++i) {
    fast = fast->next;
  }
  if (fast == NULL) {
    return NULL;
  }

  while (fast->next != NULL) {
    fast = fast->next;
    slow = slow->next;
  }
  struct ListNode* tmp = slow->next;
  assert(tmp != NULL);
  slow->next = slow->next->next;
  free(tmp);
  return head;
}

static void checkSolution(int arr[], int arr_len, int target) {
  struct ListNode* l1 = listFromArray(arr, arr_len);
  //struct ListNode* l2 = removeNthFromEnd(l1, target);
  struct ListNode* l2 = removeNthFromEnd2(l1, target);
  //struct ListNode* l2 = removeNthFromEnd3(l1, target);
  while (l2 != NULL) {
    printf("%d, ", l2->val);
    l2 = l2->next;
  }
  printf("\n");
}

int main() {
  int arr1[] = {1, 2, 3, 4, 5};
  checkSolution(arr1, sizeof(arr1) / sizeof(arr1[0]), 2);

  int arr2[] = {1};
  checkSolution(arr2, sizeof(arr2) / sizeof(arr2[0]), 1);

  int arr3[] = {1, 2};
  checkSolution(arr3, sizeof(arr3) / sizeof(arr3[0]), 1);

  return 0;
}
