// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <cassert>

#include <sstream>
#include <string>
#include <vector>

std::string simplify_path(std::string path) {
  // 创建栈, 用于存放每个目录名.
  std::vector<std::string> stack;
  std::stringstream ss(path);
  std::string folder_name;
  while (std::getline(ss, folder_name, '/')) {
    if (folder_name.empty()) {
      // 忽略空白
    } else if (folder_name == ".") {
      // 忽略当前目录
    } else if (folder_name == "..") {
      // 返回上个目录
      if (!stack.empty()) {
        stack.pop_back();
      }
    } else {
      stack.emplace_back(folder_name);
    }
  }

  std::string real_path;
  if (stack.empty()) {
    // 目录以 "/" 开头
    real_path += "/";
  } else {
    for (const auto& folder_name : stack) {
      real_path = real_path + "/" + folder_name;
    }
  }

  return real_path;
}

void check_solution() {
  const auto s1 = simplify_path("/home/");
  assert(s1 == "/home");

  const auto s2 = simplify_path("/home//foo/");
  assert(s2 == "/home/foo");

  const auto s3 = simplify_path("/home/user/Documents/../Pictures");
  assert(s3 == "/home/user/Pictures");

  const auto s4 = simplify_path("/../");
  assert(s4 == "/");

  const auto s5 = simplify_path("/.../a/../b/c/../d/./");
  assert(s5 == "/.../b/d");
}

int main() {
  check_solution();
  return 0;
}
