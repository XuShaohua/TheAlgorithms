// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "list.h"

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct list_node_s list_node_t;
struct list_node_s {
  struct list_node_s* next;
  element_type value;
};

struct list_s {
  size_t length;
  list_node_t* head;
};

static list_node_t* list_node_at(list_t* list, size_t position) {
  if (position >= list->length) {
    fprintf(stderr, "Index out of range: %ld, len: %ld\n", position, list->length);
    return NULL;
  }
  list_node_t* node = list->head;
  for (size_t i = 0; i < position && node != NULL; ++i) {
    node = node->next;
  }
  return node;
}

list_t* list_new(size_t size) {
  list_t* list = malloc(sizeof(list_t));
  assert(list != NULL);
  list->length = size;
  list->head = NULL;
  for (size_t i = 0; i < size; ++i) {
    list_node_t* node = malloc(sizeof(list_node_t));
    assert(node != NULL);
    node->next = list->head;
    node->value = 0;
    list->head = node;
  }
  return list;
}

void list_free(list_t* list) {
  list->length = 0;
  while (list->head != NULL) {
    list_node_t* node = list->head;
    list->head = list->head->next;
    free(node);
  }
  free(list);
}

element_type list_value(list_t* list, size_t position) {
  if (position >= list->length) {
    fprintf(stderr, "Index out of range: %ld, len: %ld\n", position, list->length);
    return 0;
  }

  list_node_t* node = list_node_at(list, position);
  assert(node != NULL);
  return node->value;
}

void list_set_value(list_t* list, size_t position, element_type new_value) {
  if (position >= list->length) {
    fprintf(stderr, "Index out of range: %ld, len: %ld\n", position, list->length);
    return;
  }

  list_node_t* node = list_node_at(list, position);
  assert(node != NULL);
  node->value = new_value;
}

ssize_t list_index_of(list_t* list, element_type value) {
  list_node_t* node = list->head;
  for (size_t i = 0; i < list->length; ++i) {
    if (node->value == value) {
      return (ssize_t) i;
    }
    node = node->next;
  }
  return -1;
}

void list_append(list_t* list, element_type value) {
  list_insert(list, list->length, value);
}

void list_prepend(list_t* list, element_type value) {
  list_insert(list, 0, value);
}

void list_insert(list_t* list, size_t position, element_type value) {
  list_node_t* new_node = malloc(sizeof(list_node_t));
  assert(new_node != NULL);
  new_node->value = value;

  if (position == 0) {
    new_node->next = list->head;
    list->head = new_node;
  } else {
    list_node_t* node = list_node_at(list, position - 1);
    assert(node != NULL);
    new_node->next = node->next;
    node->next = new_node;
  }
  list->length += 1;
}

void list_delete(list_t* list, size_t position) {
  if (position >= list->length) {
    fprintf(stderr, "Index out of range: %ld, len: %ld", position, list->length);
    return;
  }

  if (position == 0) {
    list_node_t* node = list->head;
    list->head = list->head->next;
    free(node);
  } else {
    list_node_t* node = list_node_at(list, position - 1);
    assert(node != NULL);
    list_node_t* delete_node = node->next;
    node->next = delete_node->next;
    free(delete_node);
  }
  list->length -= 1;
}

size_t list_length(list_t* list) {
  return list->length;
}

void list_debug_print(list_t* list) {
  for (list_node_t* node = list->head; node != NULL; node = node->next) {
    printf("%d, ", node->value);
  }
  printf("\n");
}