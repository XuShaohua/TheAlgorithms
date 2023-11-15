// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "stack.h"

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct stack_node_s stack_node_t;
struct stack_node_s {
  stack_node_t* next;
  element_type value;
};

struct stack_s {
  size_t length;
  stack_node_t* head;
};

Stack* stack_new(size_t size) {
  (void) size;
  Stack* stack = malloc(sizeof(Stack));
  assert(stack != NULL);
  stack->length = 0;
  stack->head = NULL;
  return stack;
}

void stack_free(Stack* stack) {
  while (stack->head != NULL) {
    stack_node_t* node = stack->head;
    stack->head = stack->head->next;
    free(node);
  }
  stack->head = NULL;
  free(stack);
}

bool stack_is_full(Stack* stack) {
  (void) stack;
  return false;
}

bool stack_is_empty(Stack* stack) {
  return stack->length == 0;
}

size_t stack_size(Stack* stack) {
  return stack->length;
}

void stack_push(Stack* stack, element_type value) {
  assert(!stack_is_full(stack));
  stack->length += 1;
  stack_node_t* node = malloc(sizeof(stack_node_t));
  assert(node != NULL);
  node->value = value;
  node->next = stack->head;
  stack->head = node;
}

element_type stack_pop(Stack* stack) {
  assert(!stack_is_empty(stack));
  stack->length -= 1;
  stack_node_t* node = stack->head;
  stack->head = node->next;
  element_type value = node->value;
  free(node);
  return value;
}

element_type stack_top(Stack* stack) {
  assert(!stack_is_empty(stack));
  return stack->head->value;
}