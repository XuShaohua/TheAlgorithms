// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#ifndef C_OBJECT_H_
#define C_OBJECT_H_

#include <stddef.h>

struct ng_object_s {
#define NG_OBJECT_PROPS \
  size_t ref_count; \
  const char* name; \
  size_t (*hash)(struct ng_object_s* obj); \
  void (*destroy)(struct ng_object_s* obj);
  NG_OBJECT_PROPS;
};
typedef struct ng_object_s ng_object_t;
#define NG_OBJECT(obj) (ng_object_t*)(obj)

extern void ng_object_init(ng_object_t* obj);

extern void ng_object_destroy(ng_object_t* obj);

extern size_t ng_object_hash_address(ng_object_t* obj);

extern void ng_object_incref(ng_object_t* obj);

extern void ng_object_decref(ng_object_t* obj);

#endif  // C_OBJECT_H_
