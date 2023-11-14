// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#ifndef WEISS_C03_POLYNOMIAL_ARRAY_H_
#define WEISS_C03_POLYNOMIAL_ARRAY_H_

#include <stddef.h>
#include <stdint.h>

#define MAX_DEGREE 32

struct polynomial_s {
  int32_t coeff_array[MAX_DEGREE + 1];
  uint32_t high_power;
};

typedef struct polynomial_s polynomial_t;

/**
 * Reset all fields in polynomial to zero.
 *
 * @param poly
 */
extern void polynomial_zero(polynomial_t* poly);

/**
 * Add two polynomials.
 *
 * @param poly1
 * @param poly2
 * @param poly_sum
 */
extern void polynomial_add(polynomial_t* poly1, polynomial_t* poly2,
                           polynomial_t* poly_sum);

extern void polynomial_mul(polynomial_t* poly1, polynomial_t* poly2,
                           polynomial_t* poly_prod);

#endif  // WEISS_C03_POLYNOMIAL_ARRAY_H_
