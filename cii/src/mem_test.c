// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "mem_test.h"

#include <string.h>

#include "cii/assert.h"
#include "cii/except.h"
#include "cii/mem.h"

#define MEM_TEST_MAX_DESC 2048
static descriptor_t* g_hash_table[MEM_TEST_MAX_DESC];

// dummy descriptor.
static descriptor_t g_free_list = { &g_free_list };

/**
 * Treat the block address as a bit pattern, shifts it right three bits, and
 * reduces it modulo the size of hash table.
 * @param ptr
 * @return
 */
static size_t ptr_hash(const void* ptr) {
  return (size_t)ptr >> 3 & (sizeof(g_hash_table)/sizeof(g_hash_table[0]) - 1);
}

descriptor_t* find_node(const void* ptr) {
  const size_t h = ptr_hash(ptr);
  descriptor_t* bp = g_hash_table[h];
  while (bp != NULL && bp->ptr != ptr) {
    bp = bp->link;
  }
  return bp;
}