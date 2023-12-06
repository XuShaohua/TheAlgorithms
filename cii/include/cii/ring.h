// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef CII_RING_H_
#define CII_RING_H_

#include <stddef.h>

typedef struct ring_s ring_t;

/**
 * Creates and returns an empty ring.
 *
 * @return
 */
extern ring_t* ring_new(void);

/**
 * Creates and returns a ring whose values are initialized to its non-null pointer
 * arguments.
 *
 * @param x
 * @return
 */
extern ring_t* ring_ring(void* x, ...);

/**
 * Deallocates and clear a ring.
 * @param ring
 */
extern void ring_free(ring_t** ring);

/**
 * Returns the number of values in a ring.
 *
 * @param ring
 * @return
 */
extern size_t ring_length(ring_t* ring);

/**
 * Get the value at |index| in a ring.
 *
 * It is a checked runtime error if index is out of range.
 *
 * @param ring
 * @param index
 * @return
 */
extern void* ring_get(ring_t* ring, size_t index);

/**
 * Put a new value at |index| and returns old value.
 *
 * @param ring
 * @param index
 * @param value
 * @return
 */
extern void* ring_put(ring_t* ring, size_t index, void* value);

/**
 * Add |value| at anywhere in a ring and returns |value|.
 *
 * @param ring
 * @param pos
 * @param value
 * @return
 */
extern void* ring_add(ring_t* ring, int pos, void* value);

/**
 * Add a value to lower end of ring.
 *
 * @param ring
 * @param value
 * @return
 */
extern void* ring_add_low(ring_t* ring, void* value);

/**
 * Add a value to higher end of ring.
 * @param ring
 * @param value
 * @return
 */
extern void* ring_add_high(ring_t* ring, void* value);

/**
 * Remove value at |index| and returns that value.
 *
 * @param ring
 * @param index
 * @return
 */
extern void* ring_remove(ring_t* ring, size_t index);

extern void* ring_remove_low(ring_t* ring);

extern void* ring_remove_high(ring_t* ring);

/**
 * Renumbers the ring by rotating it left or right.
 *
 * If |n| is positive, rotate to right, clockwise.
 * Else rotate to left, counterclockwise.
 *
 * It is a checked runtime error if absolute of |n| is larger than length of ring.
 *
 * @param ring
 * @param n
 */
extern void ring_rotate(ring_t* ring, int n);

#endif  // CII_RING_H_
