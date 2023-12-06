// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "cii/ring.h"

#include <stdarg.h>
#include <stdlib.h>
#include <string.h>

#include "cii/assert.h"
#include "cii/mem.h"

/**
 * Double linked list to hold the value of the ring.
 */
struct node_s {
  struct node_s* left;
  struct node_s* right;
  void* value;
};
typedef struct node_s node_t;

struct ring_s {
  size_t length;
  // |head| points to the zero indexed node.
  node_t* head;
};

ring_t* ring_new(void) {
  ring_t* ring;
  NEW(ring);
  ring->length = 0;
  ring->head = NULL;

  return ring;
}

ring_t* ring_ring(void* x, ...) {
  ring_t* ring = ring_new();

  va_list ap;
  va_start(ap, x);
  for (/* empty */ ; x != NULL; x = va_arg(ap, void*)) {
    ring_add_high(ring, x);
  }

  va_end(ap);

  return ring;
}

// First deallocates the node structures.
// Then deallocates the head.
void ring_free(ring_t** ring) {
  assert(ring != NULL && *ring != NULL);
  if ((*ring)->head != NULL) {
    node_t* p = (*ring)->head;
    node_t* q = NULL;
    for (size_t i = 0; i < (*ring)->length; ++i) {
      q = p->right;
      FREE(p);
      p = q;
    }
  }

  FREE(*ring);
}

size_t ring_length(ring_t* ring) {
  assert(ring != NULL);
  return ring->length;
}

node_t* ring_get_node(ring_t* ring, size_t index) {
  node_t* p = ring->head;
  if (index <= ring->length / 2) {
    for (size_t i = 0; i < index; ++i) {
      p = p->right;
    }
  } else {
    for (size_t i = ring->length - index; i > 0; --i) {
      p = p->left;
    }
  }
  return p;
}

void* ring_get(ring_t* ring, size_t index) {
  assert(ring != NULL);
  assert(index < ring->length);

  node_t* p = ring_get_node(ring, index);
  return p->value;
}

void* ring_put(ring_t* ring, size_t index, void* value) {
  assert(ring != NULL);
  assert(index < ring->length);

  node_t* p = ring_get_node(ring, index);
  void* prev = p->value;
  p->value = value;
  return prev;
}

void* ring_add_low(ring_t* ring, void* value) {
  assert(ring != NULL);
  ring_add_high(ring, value);
  // Rotate the ring one value to the right.
  ring->head = ring->head->left;
  return value;
}

void* ring_add_high(ring_t* ring, void* value) {
  assert(ring != NULL);
  node_t* p;
  NEW(p);
  if (ring->head != NULL) {
    node_t* q = ring->head;
    // Insert p to the left of q.
    p->left = q->left;
    p->right = q;
    q->left->right = p;
    q->left = p;
  } else {
    // p is the only value in ring.
    p->left = p;
    p->right = p;
    ring->head = p;
  }
  ring->length += 1;
  return value;
}

void* ring_add(ring_t* ring, int pos, void* value) {
  assert(ring != NULL);
  assert(pos >= -ring->length && pos <= ring->length + 1);

  if (pos == 1 || pos == -ring->length) {
    return ring_add_low(ring, value);
  } else if (pos == 0 || pos == ring->length + 1) {
    return ring_add_high(ring, value);
  }

  size_t index = (pos < 0) ? (pos + ring->length) : (pos - 1);
  node_t* q = ring_get_node(ring, index);
  node_t* p;
  NEW(p);
  // Insert p to the left of q.
  p->left = q->left;
  p->right = q;
  q->left->right = p;
  q->left = p;

  p->value = value;
  ring->length += 1;
  return value;
}

static void ring_remove_node(ring_t* ring, node_t* q) {
  // Delete node q.
  q->left->right = q->right;
  q->right->left = q->left;
  FREE(q);
  ring->length -= 1;
  if (ring->length == 0) {
    ring->head = NULL;
  }
}

void* ring_remove(ring_t* ring, size_t index) {
  assert(ring != NULL);
  assert(ring->length > 0);
  assert(index < ring->length);
  node_t* q = ring_get_node(ring, index);
  if (index == 0) {
    ring->head = ring->head->right;
  }
  void* value = q->value;
  ring_remove_node(ring, q);

  return value;
}

void* ring_remove_low(ring_t* ring) {
  assert(ring != NULL);
  assert(ring->length > 0);
  // Rotate the ring one value right.
  ring->head = ring->head->right;
  return ring_remove_high(ring);
}

void* ring_remove_high(ring_t* ring) {
  assert(ring != NULL);
  assert(ring->length > 0);

  node_t* q = ring->head->left;
  void* value = q->value;
  ring_remove_node(ring, q);

  return value;
}

void ring_rotate(ring_t* ring, int n) {
  assert(ring != NULL);
  assert(ring->length > 0);
  assert(abs(n) <= ring->length);
  size_t index;
  if (n >= 0) {
    index = n % ring->length;
  } else {
    index = n + ring->length;
  }

  node_t* q = ring_get_node(ring, index);
  assert(q != NULL);
  ring->head = q;
}