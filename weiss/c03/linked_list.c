// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include "linked_list.h"

#include <assert.h>
#include <stdio.h>
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

  // Delete the first node.
  if ((*list)->value == value) {
    node_t* tmp = *list;
    *list = tmp->next;
    free(tmp);
    return true;
  }

  node_t* prev = list_find_previous(*list, value);
  if (prev == NULL) {
    // Not found.
    return false;
  }

  node_t* tmp = prev->next;
  prev->next = tmp->next;
  free(tmp);
  return true;
}

node_t* list_insert(node_t** list, node_t* position, element_type value) {
  assert(list != NULL);

  node_t* new_node = (node_t*) malloc(sizeof(node_t));
  assert(new_node != NULL);
  new_node->value = value;

  if (position == NULL) {
    new_node->next = *list;
    *list = new_node;
  } else {
    new_node->next = position->next;
    position->next = new_node;
  }

  return new_node;
}

void list_clear(node_t** list) {
  assert(list != NULL);
  while (*list != NULL) {
    node_t* tmp = *list;
    *list = tmp->next;
    free(tmp);
  }
}

void list_map(node_t* list, void apply(node_t* node, void* user_data), void* user_data) {
  assert(list != NULL);
  while (list != NULL) {
    apply(list, user_data);
    list = list->next;
  }
}

size_t list_length(node_t* list) {
  assert(list != NULL);

  size_t count = 0;
  while (list != NULL) {
    count += 1;
    list = list->next;
  }
  return count;
}

static void print_node(node_t* node, void*) {
  printf("%d, ", node->value);
}

void list_debug_print(node_t* list) {
  list_map(list, print_node, NULL);
  printf("\n");
}
