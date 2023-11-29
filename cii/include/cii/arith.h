// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef CII_ARITH_H_
#define CII_ARITH_H_

/**
 * Returns the maximum of two integers.
 * @param x
 * @param y
 * @return
 */
extern int arith_max(int x, int y);

/**
 * Returns the minimum of two integers.
 * @param x
 * @param y
 * @return
 */
extern int arith_min(int x, int y);

/**
 * Returns quotient of dividing |x| by |y|.
 *
 * When |x| and |y| are both positive or both negative, this function is equals
 * to x / y.
 * @param x
 * @param y
 * @return
 */
extern int arith_div(int x, int y);

/**
 * Returns remainder of dividing x by y.
 *
 * When |x| and |y| are both positive or both negative, this function is equals
 * x % y.
 *
 * @param x
 * @param y
 * @return
 */
extern int arith_mod(int x, int y);

extern int arith_ceiling(int x, int y);

extern int arith_floor(int x, int y);

#endif  // CII_ARITH_H_
