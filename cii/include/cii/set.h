// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef CII_SET_H_
#define CII_SET_H_

#include <stddef.h>
#include <stdbool.h>

typedef struct set_s set_t;

typedef int (*cmp_func)(const void* x, const void* y);
typedef size_t (*hash_func)(const void* x);

/**
 * Allocates, initializes and returns a new set.
 *
 * If |cmp| is the null pointer, the members are assumed to be atoms; two members
 * x and y are assumed identical if x == y.
 * If |hash| is the null pointer, a suitable hash function is provided for atoms.
 *
 * May raise kMemFailed.
 *
 * @param size_hint is an estimate of the number of members the set is expected
 * to contain, accurate values of hint may improve performance.
 * @param cmp used to compare two members
 * @param hash to map members onto unsigned integer.
 * @return
 */
extern set_t* set_new(size_t size_hint, cmp_func cmp, hash_func hash);

/**
 * Deallocates set and assigns it the null pointer.
 *
 * The members are not deallocated.
 *
 * @param set
 */
extern void set_free(set_t** set);

/**
 * Returns number of items in set.
 *
 * @param set
 * @return
 */
extern size_t set_length(set_t* set);

/**
 * Check whether |member| is contained in |set|.
 *
 * @param set
 * @param member
 * @return
 */
extern bool set_member(set_t* set, const void* member);

/**
 * Adds a |member| into |set|, unless it is already there.
 *
 * May raise kMemFailed.
 *
 * @param set
 * @param member
 */
extern void set_put(set_t* set, const void* member);

/**
 * If |set| contains |member|, remove member from |set| and returns member.
 * Else returns null pointer.
 *
 * @param set
 * @param member
 * @return
 */
extern void* set_remove(set_t* set, const void* member);

/**
 * Calls |apply| for each member of set.
 *
 * @param set
 * @param apply
 * @param user_data
 */
extern void set_map(set_t* set,
                    void apply(const void* member, void* user_data),
                    void* user_data);

/**
 * Returns a pointer to an N+1 element array that holds N elements of set in an
 * arbitrary order.
 *
 * Clients must arrange to deallocate the return array.
 *
 * May raise kMemFailed.
 *
 * @param set
 * @param end is assigned to N+1 element of the array, which is often the null pointer.
 * @return
 */
extern void** set_to_array(set_t* set, void* end);

/**
 * Returns union of two sets. s + t.
 *
 * Raise runtime error if both |s| and |t| are null pointers.
 *
 * Note that both |s| and |t| share the same compare function and hash function.
 *
 * May raise kMemFailed.
 *
 * @param s
 * @param t
 * @return
 */
extern set_t* set_union(set_t* s, set_t* t);

/**
 * Return intersection of two sets. s * t.
 * @param s
 * @param t
 * @return
 */
extern set_t* set_intersection(set_t* s, set_t* t);

/**
 * Returns s - t.
 * @param s
 * @param t
 * @return
 */
extern set_t* set_minus(set_t* s, set_t* t);

/**
 * Returns s / t.
 * @param s
 * @param t
 * @return
 */
extern set_t* set_diff(set_t* s, set_t* t);

#endif //CII_SET_H_
