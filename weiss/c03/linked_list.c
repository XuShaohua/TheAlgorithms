// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include "linked_list.h"

#include <assert.h>
#include <stdlib.h>

bool list_is_empty(node_t* list) {
  assert(list != NULL);
  return list->next == NULL;
}

bool list_is_last(node_t* list, node_t* current) {
  assert(list != NULL);
  assert(current != NULL);
  return current->next == NULL;
}

node_t* list_find(node_t* list, element_type value) {
  assert(list != NULL);

  while ((list != NULL) && (list->value != value)) {
    list = list->next;
  }
  return list;
}

node_t* list_find_previous(node_t* list, element_type value) {
  assert(list != NULL);

  if (list->value == value) {
    return list;
  }

  while (list->next != NULL) {
    if (list->next->value == value) {
      return list;
    }
    list = list->next;
  }
  return NULL;
}

bool list_delete(node_t** list, element_type value) {
  assert(list != NULL);

  node_t* prev = list_find_previous(*list, value);
  if (prev == NULL) {
    // Not found.
    return false;
  }

  if (prev == *list) {
    // Delete the first node.
    *list = prev->next;
    free(prev);
  } else {
    node_t* tmp = prev->next;
    prev->next = tmp->next;
    free(tmp);
  }
  return true;
}
