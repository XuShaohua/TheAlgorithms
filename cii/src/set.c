// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "cii/set.h"

#include <stdint.h>
#include <limits.h>

#include "cii/arith.h"
#include "cii/assert.h"
#include "cii/mem.h"

struct member_s {
  struct member_s* link;
  const void* member;
};

struct set_s {
  size_t length;
  size_t bucket_size;
  int64_t timestamp;
  cmp_func cmp;
  hash_func hash;
  struct member_s** buckets;
};

static int cmp_atom(const void* x, const void* y) {
  return x != y;
}

static size_t hash_atom(const void* x) {
  return (unsigned long )x >> 2;
}

static void set_add_member(set_t* set, size_t hash_index, const void* member) {
  struct member_s* p = NULL;
  NEW(p);
  p->member = member;
  p->link = set->buckets[hash_index];
  set->length += 1;
  set->timestamp += 1;
}

static set_t* set_copy(set_t* s, size_t size_hint) {
  assert(s != NULL);
  set_t* set = set_new(size_hint, s->cmp, s->hash);
  for (size_t i = 0; i < s->bucket_size; ++i) {
    for (struct member_s* p = s->buckets[i]; p != NULL; p = p->link) {
      const size_t hash_index = (*set->hash)(p->member) % set->bucket_size;
      set_add_member(set, hash_index, p->member);
    }
  }

  return set;
}

set_t* set_new(size_t size_hint, cmp_func cmp, hash_func hash) {
  const size_t kPrimes[] = {
      509, 509, 1021, 2053, 4093, 8191, 16381, 32771, 65521, INT_MAX
  };
  size_t size_prime = kPrimes[0];
  for (int i = 1; kPrimes[i] < size_hint; ++i) {
    size_prime = kPrimes[i - 1];
  }
  set_t* set = ALLOC(sizeof(set_t) + size_prime * sizeof(set->buckets[0]));
  assert(set != NULL);
  set->length = 0;
  set->bucket_size = size_prime;
  set->timestamp = 0;
  set->cmp = cmp ? cmp : cmp_atom;
  set->hash = hash ? hash : hash_atom;
  set->buckets = (struct member_s**)(set + 1);
  for (size_t i = 0; i < set->bucket_size; ++i) {
    set->buckets[i] = NULL;
  }

  return set;
}

bool set_member(set_t* set, const void* member) {
  assert(set != NULL);
  assert(member != NULL);

  struct member_s* p;
  const size_t hash_index = (*set->hash)(member) % set->bucket_size;
  for (p = set->buckets[hash_index]; p != NULL; p = p->link) {
    if ((*set->cmp)(p->member, member) == 0) {
      break;
    }
  }

  return p != NULL;
}

void set_put(set_t* set, const void* member) {
  assert(set != NULL);
  assert(member != NULL);

  struct member_s* p;
  const size_t hash_index = (*set->hash)(member) % set->bucket_size;
  for (p = set->buckets[hash_index]; p != NULL; p = p->link) {
    if ((*set->cmp)(p->member, member) == 0) {
      break;
    }
  }

  if (p == NULL) {
    // Add the new member into set.
    set_add_member(set, hash_index, member);
  } else {
    p->member = member;
  }
  set->timestamp += 1;
}

void* set_remove(set_t* set, const void* member) {
  assert(set != NULL);
  assert(member != NULL);

  struct member_s** pp;
  const size_t hash_index = (*set->hash)(member) % set->bucket_size;
  for (pp = &set->buckets[hash_index]; *pp != NULL; *pp = (*pp)->link) {
    if ((*set->cmp)((*pp)->member, member) == 0) {
      struct member_s* p = *pp;
      *pp = p->link;
      member = p->member;
      FREE(p);
      set->length -= 1;
      return (void*)member;
    }
  }

  return NULL;
}

size_t set_length(set_t* set) {
  assert(set != NULL);
  return set->length;
}

void set_free(set_t** set) {
  assert(set != NULL && *set != NULL);
  if (set_length(*set) > 0) {
    for (size_t i = 0; i < (*set)->bucket_size; ++i) {
      struct member_s* p = NULL;
      struct member_s* q = NULL;
      for (p = (*set)->buckets[i]; p != NULL; p = q) {
        q = p->link;
        FREE(p);
      }
    }
  }
  FREE(*set);
}

void set_map(set_t* set,
             void apply(const void* member, void* user_data),
             void* user_data) {
  assert(set != NULL);
  assert(apply != NULL);

  if (set_length(set) > 0) {
    uint64_t timestamp = set->timestamp;
    for (size_t i = 0; i < set->bucket_size; ++i) {
      struct member_s* p = NULL;
      for (p = set->buckets[i]; p != NULL; p = p->link) {
        apply(p->member, user_data);
        assert(timestamp == set->timestamp);
      }
    }
  }
}

void** set_to_array(set_t* set, void* end) {
  assert(set != NULL);

  void** array = ALLOC((set->length + 1) * sizeof(*array));
  size_t index = 0;
  for (size_t i = 0; i < set->bucket_size; ++i ) {
    struct member_s* p = NULL;
    for (p = set->buckets[i]; p != NULL; p = p->link) {
      array[index] = (void*) p->member;
      index += 1;
    }
  }
  array[index] = end;
  return array;
}

set_t* set_union(set_t* s, set_t* t) {
  if (s == NULL) {
    assert(t != NULL);
    return set_copy(t, t->bucket_size);
  } else if (t == NULL) {
    assert(s != NULL);
    return set_copy(s, s->bucket_size);
  } else {
    set_t* set = set_copy(s, arith_max(s->bucket_size, t->bucket_size));
    assert(s->cmp == t->cmp && s->hash == t->hash);
    // Copy members in t to set.
    for (size_t i = 0; i < t->bucket_size; ++i) {
      for (struct member_s* p = t->buckets[i]; p != NULL; p = p->link) {
        set_put(set, p->member);
      }
    }
    return set;
  }
}

set_t* set_intersection(set_t* s, set_t* t) {
  if (s == NULL) {
    assert(t != NULL);
    return set_new(t->bucket_size, t->cmp, t->hash);
  } else if (t == NULL) {
    return set_new(s->bucket_size, s->cmp, s->hash);
  } else if (s->length < t->length) {
    return set_intersection(t, s);
  } else {
    assert(s->cmp == t->cmp && s->hash == t->hash);
    set_t* set = set_new(arith_min(s->bucket_size, t->bucket_size), s->cmp, s->hash);
    for (size_t i = 0; i < t->bucket_size; ++i) {
      for (struct member_s* p = t->buckets[i]; p != NULL; p = p->link) {
        if (set_member(s, p->member)) {
          const size_t hash_index = (*set->hash)(p->member) % set->bucket_size;
          set_add_member(set, hash_index, p->member);
        }
      }
    }
    return set;
  }
}

set_t* set_minus(set_t* s, set_t* t) {
  if (t == NULL) {
    assert(s != NULL);
    return set_new(s->bucket_size, s->cmp, s->hash);
  } else if (s == NULL) {
    return set_copy(t, t->bucket_size);
  } else {
    assert(s->cmp == t->cmp && s->hash == t->hash);
    set_t* set = set_new(arith_min(s->bucket_size, t->bucket_size), s->cmp, s->hash);
    for (size_t i = 0; i < t->bucket_size; ++i) {
      for (struct member_s* p = t->buckets[i]; p != NULL; p = p->link) {
        if (!set_member(s, p->member)) {
          const size_t hash_index = (*set->hash)(p->member) % set->bucket_size;
          set_add_member(set, hash_index, p->member);
        }
      }
    }
    return set;
  }
}

set_t* set_diff(set_t* s, set_t* t) {
  if (s == NULL) {
    assert(t != NULL);
    return set_copy(t, t->bucket_size);
  } else if (t == NULL) {
    return set_copy(s, s->bucket_size);
  } else {
    assert(s->cmp == t->cmp && s->hash == t->hash);
    set_t* set = set_new(arith_min(s->bucket_size, t->bucket_size), s->cmp, s->hash);

    for (size_t i = 0; i < t->bucket_size; ++i) {
      for (struct member_s* p = t->buckets[i]; p != NULL; p = p->link) {
        if (!set_member(s, p->member)) {
          const size_t hash_index = (*set->hash)(p->member) % set->bucket_size;
          set_add_member(set, hash_index, p->member);
        }
      }
    }

    {
      set_t* tmp = t;
      t = s;
      s = tmp;
    }

    for (size_t i = 0; i < t->bucket_size; ++i) {
      for (struct member_s* p = t->buckets[i]; p != NULL; p = p->link) {
        if (!set_member(s, p->member)) {
          const size_t hash_index = (*set->hash)(p->member) % set->bucket_size;
          set_add_member(set, hash_index, p->member);
        }
      }
    }

    return set;
  }
}