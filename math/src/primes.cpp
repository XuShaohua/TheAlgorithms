
#include <cassert>

#include <vector>

std::vector<int> get_primes(int n) {
  assert(n > 0);
  std::vector<bool> primes(true, n + 1);
  for (int i = 2; i < n + 1; ++i) {
    if (!primes[i]) {
      continue;
    }
    for (int j = i + 1; j < n + 1; ++j) {
      if (j % i == 0) {
        primes[j] = false;
      }
    }
  }

  return {};
}
