// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef CII_SRC_MEM_TEST_H_
#define CII_SRC_MEM_TEST_H_

#include <stdlib.h>

struct descriptor_s {
  // Forms a list of free blocks.
  // Its value is null if this block is allocated and nonnull if it's free.
  struct descriptor_s* free;
  // Linked list in the bucket with same hash index in hash table.
  struct descriptor_s* link;
  // Address of memory block.
  const void* ptr;
  // Size of memory block.
  size_t nbytes;
  // (file, line) pair is the location coordinates of code.
  const char* file;
  int line;
};

typedef struct descriptor_s descriptor_t;

/**
 * Given an address, searches for its descriptor.
 *
 * It returns either a pointer to the descriptor or the null pointer.
 * @param ptr
 * @return
 */
descriptor_t* find_node(const void* ptr);

#endif //CII_SRC_MEM_TEST_H_
