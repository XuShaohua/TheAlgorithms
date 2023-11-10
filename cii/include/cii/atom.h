// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef CII_ATOM_H_
#define CII_ATOM_H_

/**
 * An atom is a pointer to a unique, immutable sequence of zero or more
 * arbitrary bytes. Most atoms are null-terminated strings, but a pointer to
 * any sequence of bytes can be an atom.
 *
 * There is only a single occurrence(instance) of any atom, which is why it's
 * called an atom.
 *
 * Two atoms are identical if they point to the same location. Comparing two byte
 * sequences for equality by simply comparing pointers is one of the advantages
 * of atoms. Another advantage is that using atoms saves space because there's
 * only one occurrence of each sequence.
 *
 * Once an atom is created, it exists for the duration of the client's execution.
 * An atom is always terminated with a null character.
 */

#include <stdint.h>
#include <stddef.h>

/**
 * It adds a copy of the sequence to the table of atoms, if necessary, and
 * returns the pointer to that atom.
 *
 * Never returns null pointer.
 *
 * @param str sequence of bytes, shall not be null pointer.
 * @param len number of bytes
 * @return the atom, which is a pointer to the copy of the sequence in the atom
 * table.
 */
const char* atom_new(const char* str, size_t len);

/**
 * Like |atom_new|, it caters to the common use of character strings as atoms.
 *
 * Adds a copy of that string to the atom table, if necessary, and returns the
 * pointer to that atom.
 * @param str null-terminated string
 * @return the atom, which is a pointer to the copy of the sequence in the atom
 * table.
 */
const char* atom_string(const char* str);

/**
 * Returns the atom for the string representation of the integer |n|.
 * @param n integer value to be represented as string.
 * @return the atom, which is a pointer to the copy of the integer in the atom
 * table.
 */
const char* atom_int(int64_t n);

/**
 * Returns length of its atom argument.
 *
 * Will raise error if |str| is not a pointer to an atom.
 *
 * @param str pointer to an atom.
 * @return the string length of an atom.
 */
size_t atom_length(const char* str);

#endif  // CII_ATOM_H_
