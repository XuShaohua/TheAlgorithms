// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef CII_TABLE_H_
#define CII_TABLE_H_

#include <stdint.h>
#include <stddef.h>

typedef struct table_s table_t;

/**
 * Allocate a new table.
 *
 * May raise kMemFailed.
 *
 * @param hint is an estimate of the number of entries that the new table
 * is expected to hold.
 * @param cmp compares x and y values and returns an integer less than zero,
 * equal to zero, or greater than zero. If cmp is the null function pointer,
 * keys are assumed to be atoms, and two keys x and y are equal if x == y.
 * @param hash returns a hash number fo key. If cmp(x, y) == 0, then hash(x) == hash(y).
 * If hash is the null function pointer, the keys in the new table are assumed
 * to be atoms and the implementation of table provides a suitable hash function.
 * @return a pointer to new table.
 */
extern table_t* table_new(size_t hint,
                         int cmp(const void* x, const void* y),
                         size_t hash(const void* key));

/**
 * Deallocate a table and set it to the null pointer.
 *
 * It does not deallocate the keys or values.
 *
 * @param table shall not be null pointer.
 */
extern void table_free(table_t* table);

/**
 * Get number of key-value pairs in table.
 *
 * @param table
 * @return
 */
extern size_t table_length(table_t* table);

/**
 * Add a new key-value pair.
 *
 * If table already holds key, |value| overrides the previous value, and returns
 * the previous value.
 * Else, key-value pair are added to table, which grows by one entry, and returns
 * the null pointer.
 *
 * It may raise kMemFailed error.
 *
 * @param table
 * @param key
 * @param value
 * @return
 */
extern void* table_put(table_t* table, const void* key, void* value);

/**
 * Fetch the value associated with a key in the table.
 *
 * Returns null if the table does not hold the key.
 *
 * @param table
 * @param key
 * @return
 */
extern void* table_get(table_t* table, const void* key);

/**
 * Remove a key-value pair.
 *
 * If table holds the key, remove key-value pair from table, which thus shrinks
 * by one entry, and returns the removed value.
 * Else, it returns the null pointer.
 *
 * @param table
 * @param key
 * @return
 */
extern void* table_remove(table_t* table, const void* key);

/**
 * Traverse the table in an unspecified order and calls |apply| function to
 * every key-value pair.
 *
 * Note that |value| is passed to |apply| by pointer, so it may be changed within
 * that function.
 *
 * This method can also be used to deallocate keys or values before freeing the table.
 * @param table
 * @param apply
 * @param user_data
 */
extern void table_map(table_t* table,
                      void apply(const void* key, void** value, void* user_data),
                      void* user_data);

/**ap
 * Given a table with N key-value pairs, this method builds an array with 2N+1
 * elements and returns a pointer to the first element.
 *
 * The keys and values alternate, with keys appearing in the even-numbered index
 * and associated values follow by.
 *
 * The last element, at index 2N, is assigned |end|.
 *
 * Note that this method may raise kMemFailed error.
 * Clients must deallocate the array it returns.
 *
 * @param table
 * @param end
 * @return
 */
extern void** table_to_array(table_t* table, void* end);

#endif  // CII_TABLE_H_
