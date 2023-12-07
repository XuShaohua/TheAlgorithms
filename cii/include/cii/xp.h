// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef CII_XP_H_
#define CII_XP_H_

#include <stdint.h>
#include <stddef.h>

typedef uint8_t xp_t;

/**
 * Implements z = x + y + carry.
 *
 * Returns the carry-out of the most significant digit.
 * If one is returned, x + y + carry does not fit into n-digits.
 *
 * @param z
 * @param x
 * @param y
 * @param n
 * @param carry just be zero or one.
 * @return
 */
extern int xp_add(xp_t* z, const xp_t* x, const xp_t* y, size_t n, int carry);

/**
 * Implements z = x - y - borrow;
 *
 * Returns the borrow-out of the most significant digit.
 * If one is returned, y > x.
 *
 * @param z
 * @param x
 * @param y
 * @param n
 * @param borrow must be zero or one.
 * @return
 */
extern int xp_sub(xp_t* z, const xp_t* x, const xp_t* y, size_t n, int borrow);

/**
 * Implements z = z + x * y.
 *
 * Where x has n digits and y has m digits. z must be large enough to hold n + m
 * digits.
 *
 * Returns the carry-out of the most significant digit.
 *
 * @param z
 * @param x
 * @param n
 * @param y
 * @param m
 * @return
 */
extern int xp_mul(xp_t* z, const xp_t* x, size_t n, const xp_t* y, size_t m);

/**
 * Implements division.
 *
 * quot = x / y, quot and x has n digits.
 * rem = x mod y, rem and y has m digits.
 *
 * If y is zero, quot and rem is unchanged and returns zero.
 * Else it returns one.
 *
 * |tmp| must be able to hold at least n+m+2 digits.
 *
 * @param quot
 * @param rem
 * @param tmp
 * @param x
 * @param n
 * @param y
 * @param m
 * @return
 */
extern int xp_div(xp_t* quot, xp_t* rem, xp_t* tmp,
                  const xp_t* x, size_t n, const xp_t* y, size_t m);

/**
 * Implement addition.
 *
 * z[0..n-1] = x + y.
 *
 * Returns the carry-out of the most significant digit.
 *
 * @param z
 * @param x
 * @param n
 * @param y
 * @return
 */
extern uint8_t xp_sum(xp_t* z, const xp_t* x, size_t n, uint8_t y);

extern uint8_t xp_diff(xp_t* z, const xp_t* x, size_t n, uint8_t y);

/**
 * The returned carry-out can be as large as 2^8-1.
 * @param z
 * @param x
 * @param n
 * @param y
 * @return
 */
extern uint8_t xp_product(xp_t* z, const xp_t* x, size_t n, uint8_t y);

extern uint8_t xp_quotient(xp_t* z, const xp_t* x, size_t n, uint8_t y);

/**
 * Set z[0..n-1] to ~x + carry and returns the most significant digit.
 * @param z
 * @param x
 * @param n
 * @param carry
 * @return
 */
extern int xp_neg(xp_t* z, const xp_t* x, size_t n, int carry);

/**
 * If x < y, return a value less than zero.
 * If x == y, returns zero.
 * If x > y, returns a value greater than zero.
 *
 * @param x
 * @param y
 * @param n
 * @return
 */
extern int xp_cmp(const xp_t* x, const xp_t* y, size_t n);

/**
 * Assigned z[0..n-1] to x shifted left s bits.
 * @param z
 * @param n
 * @param x
 * @param m
 * @param s
 * @param fill must be zero or one.
 */
extern void xp_shift_left(xp_t* z, size_t n, const xp_t* x, size_t m,
                          int s, int fill);

extern void xp_shift_right(xp_t* z, size_t n, const xp_t* x, size_t m,
                           int s, int fill);

#endif  // CII_XP_H_
