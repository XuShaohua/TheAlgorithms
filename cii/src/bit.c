// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "cii/bit.h"

#include <stdarg.h>
#include <string.h>
#include <stdint.h>

#include "cii/assert.h"
#include "cii/mem.h"

struct bit_s {
  // The |length| field gives the number of bits in the vector.
  size_t length;

  // |bytes| points to at least [length / 8] bytes.
  // The bits are accessed by indexing bytes.
  uint8_t* bytes;
  size_t* words;
};

// Bytes per word.
static size_t bpw() {
  return 8 * sizeof(size_t);
}

// Computes the number of words needed for a bit vector of length bits.
static size_t num_words(size_t length) {
  return ((length + bpw() - 1) & (~(bpw() - 1))) / bpw();
}

bit_t* bit_new(size_t length) {
  bit_t* set;
  NEW(set);
  if (length > 0) {
    set->words = CALLOC(num_words(length), sizeof(size_t));
  } else {
    set->words = NULL;
  }
  set->length = length;
  set->bytes = (uint8_t*) set->words;

  return set;
}

void bit_free(bit_t** set) {
  assert(set != NULL && *set != NULL);
  FREE((*set)->words);
  FREE(*set);
}

size_t bit_length(bit_t* set) {
  assert(set != NULL);
  return set->length;
}

size_t bit_count_ones(bit_t* set) {
  assert(set != NULL);
  size_t length = 0;
  const uint8_t kCount[] = {
      0,1,1,2,1,2,2,3,1,2,2,3,2,3,3,4
  };
  const size_t n_words = num_words(set->length);
  for (size_t i = 0; i < n_words; ++i) {
    uint8_t c = set->bytes[i];
    length += kCount[c & 0x0F] + kCount[c >> 4];
  }

  return length;
}

size_t bit_count_zeros(bit_t* set) {
  assert(set != NULL);
  return bit_length(set) - bit_count_ones(set);
}

static bool bit_get_bit(bit_t* set, size_t index) {
  return (set->bytes[ index / 8] >> (index % 8)) & 1;
}

bool bit_get(bit_t* set, size_t index) {
  assert(set != NULL);
  assert(index < set->length);
  return bit_get_bit(set, index);
}

bool bit_put(bit_t* set, size_t index, bool value) {
  assert(set != NULL);
  assert(index < set->length);
  const bool prev = bit_get_bit(set, index);
  if (value) {
    set->bytes[index / 8] |= 1 << (index % 8);
  } else {
    set->bytes[index / 8] &= ~(1 << (index % 8));
  }
  return prev;
}

// Most significant byte mask.
static const uint8_t kMsbMask[] = {
    0xFF, 0xFE, 0xFC, 0xF8,
    0xF0, 0xE0, 0xC0, 0x80
};

// Least significant byte mask.
static const uint8_t kLsbMask[] = {
    0x01, 0x03, 0x07, 0x0F,
    0x1F, 0x3F, 0x7F, 0xFF
};

void bit_clear(bit_t* set, size_t low, size_t high) {
  assert(set != NULL);
  assert(high < set->length);
  assert(low <= high);

  if (low / 8 < high / 8) {
    // Set the most significant bits in byte low/8.
    set->bytes[low / 8] &= ~kMsbMask[low % 8];

    // Set all the bits in bytes low/8+1..high/8-1.
    {
      for (size_t i = low / 8 + 1; i < high / 8; ++i) {
        set->bytes[i] = 0x00;
      }
    }

    // Set the least significant bits in byte high/8.
    set->bytes[high / 8] &= ~kLsbMask[high % 8];
  } else {
    // Set bits low%8..high%8 in byte low/8.
    set->bytes[low / 8] &= ~(kMsbMask[low % 8] & kLsbMask[high % 8]);
  }
}

void bit_set(bit_t* set, size_t low, size_t high) {
  assert(set != NULL);
  assert(high < set->length);
  assert(low <= high);

  if (low / 8 < high / 8) {
    // Set the most significant bits in byte low/8.
    set->bytes[low / 8] |= kMsbMask[low % 8];

    // Set all the bits in bytes low/8+1..high/8-1.
    {
      for (size_t i = low / 8 + 1; i < high / 8; ++i) {
        set->bytes[i] = 0xFF;
      }
    }

    // Set the least significant bits in byte high/8.
    set->bytes[high / 8] |= kLsbMask[high % 8];
  } else {
    // Set bits low%8..high%8 in byte low/8.
    set->bytes[low / 8] |= (kMsbMask[low % 8] & kLsbMask[high % 8]);
  }
}

void bit_not(bit_t* set, size_t low, size_t high) {
  assert(set != NULL);
  assert(high < set->length);
  assert(low <= high);

  if (low / 8 < high / 8) {
    // Set the most significant bits in byte low/8.
    set->bytes[low / 8] ^= kMsbMask[low % 8];

    // Set all the bits in bytes low/8+1..high/8-1.
    {
      for (size_t i = low / 8 + 1; i < high / 8; ++i) {
        set->bytes[i] ^= 0xFF;
      }
    }

    // Set the least significant bits in byte high/8.
    set->bytes[high / 8] ^= kLsbMask[high % 8];
  } else {
    // Set bits low%8..high%8 in byte low/8.
    set->bytes[low / 8] ^= (kMsbMask[low % 8] & kLsbMask[high % 8]);
  }
}

void bit_map(bit_t* set,
             void(*apply)(size_t index, bool is_set, void* user_data),
             void* user_data) {
  assert(set != NULL);
  assert(apply != NULL);

  for (size_t i = 0; i < set->length; ++i) {
    const bool bit = bit_get_bit(set, i);
    apply(i, bit, user_data);
  }
}