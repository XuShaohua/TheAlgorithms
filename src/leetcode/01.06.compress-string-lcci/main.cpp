// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <cassert>

#include <string>
#include <vector>

class Solution {
 public:
  // 双指针法遍历字符串
  static std::string compressString(std::string S) {
    std::string out;
    const int len = S.size();

    // 遍历所有字符
    int left = 0;
    while (left < len) {
      // 注意边界值
      int right = left;
      while (right < len && S[left] == S[right]) {
        right += 1;
      }
      // 拼接字符串
      out += S[left];
      out += std::to_string(right - left);

      // 将左侧边界向右移
      left = right;
    }
    // 判断压缩后的字符串是不是变短了
    if (out.size() < S.size()) {
      return out;
    } else {
      return S;
    }
  }

  // 空间复杂度 O(1)
  static std::string compressString2(std::string S) {
    // 首先遍历字符串, 找出字符的最大重复次数 max_count
    // 如果 max_count == 2, 说明不必要压缩它, 直接返回.
    //
    // 然后对字符串进行原地压缩
    // 要注意边界值
    const int len = S.size();
    const size_t kBufLen = 10;
    char buf[kBufLen];

    // TODO(Shaohua):

    return S;
  }
};

void checkSolution() {
  {
   const std::string s1 = "aabcccccaaa";
   const std::string expected = "a2b1c5a3";
   const std::string s2 = Solution::compressString(s1);
   assert(s2 == expected);
  }

  {
    const std::string s1 = "abbccd";
    const std::string expected = "abbccd";
    const std::string s2 = Solution::compressString(s1);
    assert(s2 == expected);
  }
}

int main() {
  checkSolution();
  return 0;
}
