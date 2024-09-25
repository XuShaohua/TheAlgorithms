// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <cassert>

#include <algorithm>
#include <array>
#include <iostream>
#include <string>
#include <unordered_set>
#include <vector>

class Solution {
 public:
  static bool checkInclusion(std::string s1, std::string s2) {
    // 构造s2所有可能的子串, 子串长度为 s1.size(), 并对子串进行排序
    // 然后遍历 s1 所有的排列, 看它们是否存在集合中.
    // 并对 s1 进行排序, 比较 s2 中的子串与 s1 是否相同.
    //
    // 考虑到每次构造 s2 的子串都要排序, 我们可以使用集合来减少排序次数.
    // 因为字符串中只包含小写的英文字母, 可以用数组存储字符串中每个字符的出现次数,
    // 这样也可以避免对字符串进行排序

    // 边界情况
    if (s1.size() > s2.size()) {
      return false;
    }
    std::sort(s1.begin(), s1.end());
    if (s1.size() == s2.size()) {
      std::sort(s2.begin(), s2.end());
      return s1 == s2;
    }

    const int s1_len = s1.size();
    for (int i = 0; i < s2.size() - s1_len + 1; ++i) {
      std::string sub_string = s2.substr(i, s1_len);
      std::sort(sub_string.begin(), sub_string.end());
      if (sub_string == s1) {
        return true;
      }
    }

    return false;
  }

  static constexpr size_t kNumChars = 26;

  static bool sliceEqual(const std::array<int, kNumChars>& count1,
                         const std::array<int, kNumChars>& count2) {
    for (int i = 0; i < kNumChars; ++i) {
      if (count1[i] != count2[i]) {
        return false;
      }
    }
    return true;
  }

  // 因为字符串中只包含小写的英文字母, 可以用数组存储字符串中每个字符的出现次数,
  // 这样也可以避免对字符串进行排序
  static bool checkInclusion2(std::string s1, std::string s2) {
    if (s1.size() > s2.size()) {
      return false;
    }
    // 构造 s1 的字符数组
    std::array<int, kNumChars> count1;
    for (int i = 0; i < kNumChars; ++i) {
      count1[i] = 0;
    }
    for (char c : s1) {
      int index = c - 'a';
      count1[index] += 1; 
    }

    // 构造 s2 的字符数组
    std::array<int, kNumChars> count2;
    for (int i = 0; i < kNumChars; ++i) {
      count2[i] = 0;
    }
    for (int i = 0; i < s1.size(); ++i) {
      int index = s2[i] - 'a';
      count2[index] += 1; 
    }
    if (sliceEqual(count1, count2)) {
      return true;
    }

    const int s1_len = s1.size();
    for (int i = 1; i < s2.size() - s1_len + 1; ++i) {
      int old_index = s2[i - 1] - 'a';
      int new_index = s2[i + s1_len - 1] - 'a';
      count2[old_index] -= 1;
      count2[new_index] += 1;
      if (sliceEqual(count1, count2)) {
        return true;
      }
    }

    return false;
  }
};

void checkSolution() {
  {
    std::string s1 = "a";
    std::string s2 = "ab";
    const bool out = Solution::checkInclusion2(s1, s2);
    assert(out);
  }
  {
    std::string s1 = "ab";
    std::string s2 = "eidbaooo";
    const bool out = Solution::checkInclusion2(s1, s2);
    assert(out);
  }

  {
    std::string s1= "ab";
    std::string s2 = "eidboaoo";
    const bool out = Solution::checkInclusion2(s1, s2);
    assert(!out);
  }

  {
    std::string s1= "ab";
    std::string s2 = "ba";
    const bool out = Solution::checkInclusion2(s1, s2);
    assert(out);
  }

  {
    std::string s1= "trinitrophenylmethylnitramine";
    std::string s2 = "dinitrophenylhydrazinetrinitrophenylmethylnitramine";
    const bool out = Solution::checkInclusion2(s1, s2);
    assert(out);
  }
}

int main() {
  checkSolution();
  return 0;
}
