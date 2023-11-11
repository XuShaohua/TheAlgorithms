// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef CII_MEM_H_
#define CII_MEM_H_

#include <stddef.h>

#include "cii/except.h"

extern const except_t kMemFailed;

/**
 * Allocates a block of at least nbytes and returns a pointer to the first byte.
 *
 * The block is aligned on an addressing boundary that is suitable for the data
 * with the strictest alignment requirement. The contents of the block are
 * uninitialized.
 *
 * Raise kMemFailed exception if failed to allocate memory.
 * @param nbytes
 * @param file
 * @param line
 * @return pointer to the address of first byte.
 */
void* mem_alloc(size_t nbytes, const char* file, int line);

/**
 * Allocates a block large enough to hols an array of |count| elements each of
 * size |nbytes|, and returns a pointer to the first element.
 *
 * The block is aligned and is initialized to zero.
 *
 * Raise kMemFailed exception if failed to allocate memory.
 * @param count
 * @param nbytes
 * @param file
 * @param line
 * @return
 */
void* mem_calloc(size_t count, size_t nbytes, const char* file, int line);

/**
 * Deallocates the block the pointer |ptr| points to.
 *
 * Do nothing if |ptr| is null.
 * @param ptr
 * @param file
 * @param line
 */
void mem_free(void* ptr, const char* file, int line);

/**
 * Change size of the block.
 * @param ptr
 * @param nbytes
 * @param file
 * @param line
 */
void* mem_resize(void* ptr, size_t nbytes, const char* file, int line);

/**
 * Allocate an uninitialized block.
 */
#define ALLOC(nbytes) mem_alloc((nbytes), __FILE__, __LINE__)

/**
 * Allocate an array of element with zero-initialized.
 */
#define CALLOC(count, nbytes) mem_calloc((count), (nbytes), __FILE__, __LINE__)

/**
 * Allocate an uninitialized block to hold *p and set sets p to the block
 * address of that block.
 */
#define NEW(p) ((p) = ALLOC(sizeof *(p)))

/**
 * Allocate an zero-initialized block to hold *p and set sets p to the block
 * address of that block.
 */
#define NEW0(p) ((p) = CALLOC(1, sizeof *(p)))

/**
 * Deallocate memory block, and reset pointer value to null.
 */
#define FREE(ptr) ((void)(mem_free((ptr), __FILE__, __LINE__), (ptr) = 0))

#define RESIZE(ptr, nbytes) ((ptr) = mem_resize((ptr), (nbytes), __FILE__, __LINE__))

#endif  // CII_MEM_H_
