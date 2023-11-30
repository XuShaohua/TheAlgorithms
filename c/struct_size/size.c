
#include <stdio.h>

struct empty_s {
};

int main(void) {
  printf("sizeof empty struct in C: %ld\n", sizeof(struct empty_s));
  return 0;
}
