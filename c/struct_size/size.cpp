
#include <cassert>
#include <cstdio>

class Empty {
};

int main(void) {
  printf("sizeof empty struct in C++: %ld\n", sizeof(class Empty));

  Empty* p1 = new Empty;
  Empty* p2 = new Empty;
  assert(p1 != p2);

  return 0;
}
