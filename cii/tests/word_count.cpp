// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include <string>
#include <map>
#include <iostream>
#include <fstream>

void word_count(const char* file) {
  std::map<std::string, size_t> counts;
  std::string word;

  if (file != nullptr) {
    std::fstream fstream;
    fstream.open(file);
    while (fstream >> word) {
      ++counts[word];
    }
  } else {
    while (std::cin >> word) {
      ++counts[word];
    }
  }

  for (const auto& w : counts) {
    std::cout << w.first << " occurs " << w.second
              << ((w.second > 1) ? " times" : " time")
              << std::endl;
  }
}

int main(int argc, char** argv) {
  for (int i = 1; i < argc; ++i) {
    word_count(argv[i]);
  }
  if (argc == 1) {
    word_count(nullptr);
  }
  return 0;
}