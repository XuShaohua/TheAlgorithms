// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef CII_BIT_H
#define CII_BIT_H

#include <stdbool.h>
#include <stddef.h>

typedef struct bit_s bit_t;

/**
 * Create a bit vector with fixed |length| and sets all the bits to zero.
 *
 * @param length
 * @return
 */
extern bit_t* bit_new(size_t length);

/**
 * Get fixed length of a bit vector.
 *
 * @param set
 * @return
 */
extern size_t bit_length(bit_t* set);

/**
 * Returns number of bits with one values in a bit vector.
 *
 * @param set
 * @return
 */
extern size_t bit_count_ones(bit_t* set);

/**
 * Returns number of bits with zero values in a bit vector.
 * @param set
 * @return
 */
extern size_t bit_count_zeros(bit_t* set);

/**
 * Deallocate and clear a bit vector.
 *
 * @param set
 */
extern void bit_free(bit_t** set);

/**
 * Check bit at |index| is set or not.
 *
 * @param set
 * @param index
 * @return
 */
extern bool bit_get(bit_t* set, size_t index);

/**
 * update bit value at |index| and returns old value.
 *
 * @param set
 * @param index
 * @param value
 * @return
 */
extern bool bit_put(bit_t* set, size_t index, bool value);

/**
 * Clear bits in range [low, high] in a bit vector.
 *
 * @param set
 * @param low
 * @param high
 */
extern void bit_clear(bit_t* set, size_t low, size_t high);

/**
 * Set bits in range [low, high] in a bit vector.
 *
 * @param set
 * @param low
 * @param high
 */
extern void bit_set(bit_t* set, size_t low, size_t high);

/**
 * Toggle bits in range [low, high] in a bit vector.
 *
 * @param set
 * @param low
 * @param high
 */
extern void bit_not(bit_t* set, size_t low, size_t high);

/**
 * Check whether bit vector s is less than t.
 *
 * @param s
 * @param t
 * @return
 */
extern bool bit_less(bit_t* s, bit_t* t);

/**
 * Check whether bit vector s equals to t.
 *
 * @param s
 * @param t
 * @return
 */
extern bool bit_equal(bit_t* s, bit_t* t);

/**
 * Check whether bit vector s is less than or equal to t.
 *
 * @param s
 * @param t
 * @return
 */
extern bool bit_less_equal(bit_t* s, bit_t* t);

/**
 * Calls |apply| for each bit in set.
 * @param s
 * @param apply
 * @param user_data
 */
extern void bit_map(bit_t* set,
                    void(*apply)(size_t index, bool is_set, void* user_data),
                    void* user_data);

/**
 * Returns the union of s and t, denoted s + t.
 * @param s
 * @param t
 * @return
 */
extern bit_t* bit_union(bit_t* s, bit_t* t);

/**
 * Returns the intersection of s and t, denoted s * t.
 * @param s
 * @param t
 * @return
 */
extern bit_t* bit_inter(bit_t* s, bit_t* t);

/**
 * Returns the difference of s and t, denoted s - t.
 * @param s
 * @param t
 * @return
 */
extern bit_t* bit_minus(bit_t* s, bit_t* t);

/**
 * Returns the symmetric difference of s and t, denoted s / t.
 * @param s
 * @param t
 * @return
 */
extern bit_t* bit_diff(bit_t* s, bit_t* t);

#endif  // CII_BIT_H
