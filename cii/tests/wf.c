// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "wf.h"

#include <ctype.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>

#include "cii/assert.h"
#include "cii/atom.h"
#include "cii/mem.h"
#include "cii/table.h"
#include "get_word.h"

int first(int c) {
  return isalpha(c);
}

int rest(int c) {
  return isalpha(c) || c == '_';
}

int compare(const void* x, const void* y) {
  return strcmp(*(char**)x, *(char**) y);
}

void wf(const char* filename, FILE* fp) {
  const size_t kBufLen = 128;
  char buf[kBufLen];

  table_t* table = table_new(0, NULL, NULL);
  assert(table != NULL);

  while (get_word(fp, buf, kBufLen, first, rest)) {
    for (int i = 0; buf[i] != '\0'; ++i) {
      buf[i] = (char)tolower(buf[i]);
    }
    const char* word = atom_string(buf);
    int* count = table_get(table, word);
    if (count != NULL) {
      *count += 1;
    } else {
      NEW(count);
      *count = 1;
      table_put(table, word, count);
    }
  }

  // Print the words.
  if (filename != NULL) {
    printf("%s:\n", filename);
  }
  {
    void** array = table_to_array(table, NULL);
    assert(array != NULL);
    qsort(array, table_length(table), 2 * sizeof(*array), compare);
//    for (int i = 0; array[i] != NULL; i + 2) {
//      printf("%d\t%s\n",
//             *(int*) array[i+1],
//             (char*) array[i]);
//    }
    FREE(array);
  }

  // Deallocate the entries and the table.
  table_free(&table);
}
