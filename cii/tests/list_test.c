// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include "cii/list.h"

static void print_node(void** value, void* cl) {
  char** str = (char**)value;
  FILE* fp = cl;
  fprintf(fp, "node: %s\n", *str);
}

int main() {
  list_t* p1 = list_list(NULL);
  list_t* p2 = list_list("Atom", "Mem", "Arena", "List", NULL);

  list_t* p3 = list_push(NULL, "List");
  p3 = list_push(p3, "Arena");
  p3 = list_push(p3, "Mem");
  p3 = list_push(p3, "Atom");

  list_t* p4 = list_append(NULL, list_list("Except", NULL));

  p3 = list_reverse(p3);

  list_t* p5 = list_copy(p2);
  list_map(p5, print_node, stdout);

  char** array_p2 = (char**)list_to_array(p2, NULL);
  for (int i = 0; array_p2[i] != NULL; ++i) {
    printf("[%d] %s\n", i, array_p2[i]);
  }

  list_free(&p2);
  list_free(&p3);
  list_free(&p4);
  list_free(&p5);

  free(array_p2);
}