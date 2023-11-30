// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "queue.h"

#include <assert.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

struct queue_s {
  element_type* data;
  ssize_t front;
  ssize_t rear;
  size_t capacity;
  size_t size;
};

queue_t* queue_new(size_t size) {
  queue_t* queue = malloc(sizeof(queue_t));
  assert(queue != NULL);
  queue->front = -1;
  queue->rear = -1;
  queue->capacity = size;
  queue->size = 0;
  const size_t data_size = sizeof(element_type) * size;
  queue->data = malloc(data_size);
  assert(queue->data != NULL);
  memset(queue->data, 0, data_size);

  return queue;
}

void queue_free(queue_t* queue) {
  free(queue->data);
  queue->data = NULL;
  free(queue);
}

bool queue_is_full(queue_t* queue) {
  return queue->size == queue->capacity;
}

bool queue_is_empty(queue_t* queue) {
  return queue->size == 0;
}

size_t queue_size(queue_t* queue) {
  return queue->size;
}

void queue_add(queue_t* queue, element_type value) {
  assert(!queue_is_full(queue));
  queue->rear = (queue->rear + 1) % (ssize_t)queue->capacity;
  queue->data[queue->rear] = value;
  queue->size += 1;
}

element_type queue_delete(queue_t* queue) {
  assert(!queue_is_empty(queue));
  queue->front = (queue->front + 1) % (ssize_t)queue->capacity;
  queue->size -= 1;
  return queue->data[queue->front];
}

element_type queue_get(queue_t* queue) {
  assert(!queue_is_empty(queue));
  return queue->data[queue->front+1];
}

void queue_debug_print(queue_t* queue) {
  if (queue_is_empty(queue)) {
    return;
  }

  if (queue->rear > queue->front) {
    for (ssize_t i = queue->front + 1; i <= queue->rear; ++i) {
      printf("%d, ", queue->data[i]);
    }
    printf("\n");
  } else {
    for (ssize_t i = queue->front + 1; i != queue->size; ++i) {
      printf("%d, ", queue->data[i]);
    }
    for (ssize_t i = 0; i <= queue->rear; ++i) {
      printf("%d, ", queue->data[i]);
    }
    printf("\n");
  }

}