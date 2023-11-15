// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "queue.h"

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct queue_node_s queue_node_t;
struct queue_node_s {
  element_type value;
  queue_node_t* next;
};

struct queue_s {
  queue_node_t* front;
  queue_node_t* rear;
  size_t size;
};

queue_t* queue_new(size_t size) {
  (void) size;
  queue_t* queue = malloc(sizeof(queue_t));
  assert(queue != NULL);
  queue->front = NULL;
  queue->rear = NULL;
  queue->size = 0;
  return queue;
}

void queue_free(queue_t* queue) {
  for (queue_node_t* node = queue->front; node != queue->rear; /* */) {
    queue_node_t* next_node = node->next;
    free(node);
    node = next_node;
  }

  free(queue);
}

bool queue_is_full(queue_t* queue) {
  (void) queue;
  return false;
}

bool queue_is_empty(queue_t* queue) {
  return queue->size == 0;
}

size_t queue_size(queue_t* queue) {
  return queue->size;
}

void queue_add(queue_t* queue, element_type value) {
  queue->size += 1;
  queue_node_t* new_node = malloc(sizeof(queue_node_t));
  assert(new_node != NULL);
  new_node->value = value;
  new_node->next = NULL;
  if (queue->rear == NULL) {
    queue->rear = new_node;
    queue->front = new_node;
  } else {
    queue->rear->next = new_node;
    queue->rear = new_node;
  }
}

element_type queue_delete(queue_t* queue) {
  assert(!queue_is_empty(queue));
  queue_node_t* new_rear = queue->front;
  if (queue->front == queue->rear) {
    // When last element is deleted, rear ponter needs to be reset too.
    queue->rear = NULL;
  }
  queue->front = queue->front->next;
  free(new_rear);
  queue->size -= 1;
}

element_type queue_get(queue_t* queue) {
  assert(!queue_is_empty(queue));
  return queue->front->value;
}

void queue_debug_print(queue_t* queue) {
  if (queue_is_empty(queue)) {
    return;
  }

  for (queue_node_t* ptr = queue->front; ptr != NULL; ptr = ptr->next) {
    printf("%d, ", ptr->value);
  }
  printf("\n");
}
