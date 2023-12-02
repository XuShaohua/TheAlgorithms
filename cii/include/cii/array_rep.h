// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef CII_ARRAY_REP_H_
#define CII_ARRAY_REP_H_

#include <stddef.h>

struct array_s {
  size_t length;
  size_t element_size;
  char* buffer;
};

extern void array_rep_init(struct array_s* array, size_t length, size_t elem_size, void* arp);

#endif  // CII_ARRAY_REP_H_
