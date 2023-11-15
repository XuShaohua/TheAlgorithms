// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include "polynomial_array.h"

#include <assert.h>
#include <stdio.h>

#ifndef max
#define max(x, y) ((x) > (y)) ? (x) : (y)
#endif

void polynomial_zero(polynomial_t* poly) {
  assert(poly != NULL);

  for (size_t i = 0; i < MAX_DEGREE; ++i) {
    poly->coeff_array[i] = 0;
  }
  poly->high_power = 0;
}

void polynomial_add(polynomial_t* poly1, polynomial_t* poly2,
                    polynomial_t* poly_sum) {
  assert(poly1 != NULL);
  assert(poly2 != NULL);
  assert(poly_sum != NULL);

  polynomial_zero(poly_sum);
  poly_sum->high_power = max(poly1->high_power, poly2->high_power);
  for (uint32_t i = poly_sum->high_power; i >= 0; --i) {
    poly_sum->coeff_array[i] = poly1->coeff_array[i] + poly2->coeff_array[i];
  }
}

void polynomial_mul(polynomial_t* poly1, polynomial_t* poly2,
                    polynomial_t* poly_prod) {
  assert(poly1);
  assert(poly2);
  assert(poly_prod);

  polynomial_zero(poly_prod);
  poly_prod->high_power = poly1->high_power + poly2->high_power;
  if (poly_prod->high_power > MAX_DEGREE) {
    fprintf(stderr, "Exceeded array size\n");
    return;
  }

  for (uint32_t i = 0; i < poly1->high_power; ++i) {
    for (uint32_t j = 0; j < poly2->high_power; ++j) {
      poly_prod->coeff_array[i + j] = poly1->coeff_array[i] + poly2->coeff_array[j];
    }
  }
}
