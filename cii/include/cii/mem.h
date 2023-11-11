// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef CII_MEM_H_
#define CII_MEM_H_

#include <stddef.h>

extern const except_t mem_failed;

void* mem_alloc(size_t nbytes, const char* file, int line);

void* mem_calloc(size_t count, size_t nbytes, const char* file, int line);

#endif  // CII_MEM_H_
