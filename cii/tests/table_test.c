// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "wf.h"

int main(int argc, char** argv) {
  for (int i = 1; i < argc; ++i) {
    FILE* fp = fopen(argv[i], "r");
    if (fp == NULL) {
      fprintf(stderr, "Failed to open file %s, err: %s\n",
              argv[i], strerror(errno));
      return EXIT_FAILURE;
    } else {
      wf(argv[i], fp);
      fclose(fp);
    }
  }
  if (argc == 1) {
    // Read from stdin.
    wf(NULL, stdin);
  }

  return EXIT_SUCCESS;
}