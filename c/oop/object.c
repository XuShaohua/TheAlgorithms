// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include "object.h"

#include <assert.h>
#include <stdio.h>
#include <stddef.h>
#include <stdlib.h>

size_t ng_object_hash_address(ng_object_t* obj) {
  return (unsigned long)obj >> 1;
}

void ng_object_init(ng_object_t* obj) {
  assert(obj != NULL);
  obj->ref_count = 1;
  obj->hash = ng_object_hash_address;
  obj->destroy = ng_object_destroy;
}

void ng_object_destroy(ng_object_t* obj) {
  fprintf(stderr, "%s: %p\n", __func__, obj);
  assert(obj != NULL);
  assert(obj->ref_count == 0);
  free(obj);
}

void ng_object_incref(ng_object_t* obj) {
  assert(obj != NULL);
  obj->ref_count += 1;
}

void ng_object_decref(ng_object_t* obj) {
  assert(obj != NULL);
  assert(obj->ref_count > 0);
  obj->ref_count -= 1;
  if (obj->ref_count == 0) {
    obj->destroy(obj);
  }
}

