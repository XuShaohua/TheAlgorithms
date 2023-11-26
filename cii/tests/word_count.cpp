// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include <string>
#include <map>
#include <iostream>

void word_count(const char* file) {
  std::map<std::string, size_t> counts;
  std::string word;
  while (std::cin >> word) {
    counts[word] += 1;
  }

  for (const auto& w : counts) {
    std::cout << w.first << " occurs " << w.second
              << ((w.second > 1) ? "times" : "time")
              << std::endl;
  }
}

int main(int argc, char** argv) {
  for (int i = 1; i < argc; ++i) {
    word_count(argv[i]);
  }
  return 0;
}