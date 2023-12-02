// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "cii/seq.h"

#include <stdarg.h>
#include <stdlib.h>
#include <string.h>

#include "cii/array.h"
#include "cii/array_rep.h"
#include "cii/assert.h"
#include "cii/mem.h"

struct seq_s {
  // The |length| holds the number of values in the sequence.
  size_t length;
  // The first value is stored in the element number |head| of the array.
  size_t head;
  // The |array| stores values.
  // Length of this array is at least |length|.
  //
  // It is used as a circle buffer.
  // If the ith value in the sequence is stored
  // in the element number array.length - 1, the ith + 1 value is stored in the
  // element zero of the array.
  array_t array;
};

seq_t* seq_new(size_t size_hint) {
  if (size_hint == 0) {
    size_hint = 16;
  }

  seq_t* seq;
  NEW(seq);
  void* arp = ALLOC(size_hint * sizeof(void*));
  array_rep_init(&seq->array, size_hint, sizeof(void*), arp);

  return seq;
}

seq_t* seq_seq(void* x, ...) {
  va_list ap;
  va_start(ap, x);
  seq_t* seq = seq_new(0);
  for (/* empty */; x != NULL; x = va_arg(ap, void*)) {
    seq_push_high(seq, x);
  }
  va_end(ap);

  return seq;
}

void seq_free(seq_t** seq) {
  assert(seq != NULL && *seq != NULL);
  assert((void*)*seq == (void*) &(*seq)->array);
  array_free((array_t**)seq);
}

size_t seq_length(seq_t* seq) {
  assert(seq != NULL);
  return seq->length;
}

static void* seq_value_at(seq_t* seq, size_t index) {
  size_t index_modulo = (seq->head + index) % seq->array.length;
  return ((void**)seq->array.buffer)[index_modulo];
}

static void* seq_set_value_at(seq_t* seq, size_t index, void* value) {
  size_t index_modulo = (seq->head + index) % seq->array.length;
  ((void**)seq->array.buffer)[index_modulo] = value;
  return value;
}

static void seq_expand(seq_t* seq) {
  size_t len = seq->length;
  array_resize(&seq->array, 2 * len);
  if (seq->head > 0) {
    // slide tail down.
    void** old = &((void**)seq->array.buffer)[seq->head];
    memcpy(old + len, old, (len - seq->head) * sizeof(void*));
    seq->head += len;
  }
}

void* seq_get(seq_t* seq, size_t index) {
  assert(seq != NULL);
  assert(index < seq->length);
  return seq_value_at(seq, index);
}

void* seq_put(seq_t* seq, size_t index, void* value) {
  assert(seq != NULL);
  assert(index < seq->length);
  void* prev = seq_value_at(seq, index);
  seq_set_value_at(seq, index, value);

  return prev;
}

void* seq_push_low(seq_t* seq, void* value) {
  assert(seq != NULL);
  if (seq->length == seq->array.length) {
    seq_expand(seq);
  }
  if (seq->head == 0) {
    seq->head = seq->array.length - 1;
  } else {
    seq->head -= 1;
  }
  size_t index = 0;
  seq->length += 1;
  return seq_set_value_at(seq, index, value);
}

void* seq_push_high(seq_t* seq, void* value) {
  assert(seq != NULL);
  if (seq->length == seq->array.length) {
    seq_expand(seq);
  }
  size_t index = seq->length;
  seq->length += 1;
  return seq_set_value_at(seq, index, value);
}

void* seq_pop_low(seq_t* seq) {
  assert(seq != NULL);
  assert(seq->length > 0);
  void* x = seq_value_at(seq, 0);
  seq->head = (seq->head + 1) % seq->array.length;
  seq->length -= 1;

  return x;
}

void* seq_pop_high(seq_t* seq) {
  assert(seq != NULL);
  assert(seq->length > 0);
  seq->length -= 1;
  size_t index = seq->length;
  return seq_value_at(seq, index);
}