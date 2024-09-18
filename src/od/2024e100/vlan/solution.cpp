// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <algorithm>
#include <iostream>
#include <string>
#include <sstream>
#include <unordered_set>
#include <vector>

int main() {
  // 读取输入
  // 解析当前所有的 VLAN ID, 并存储到集合或者数组中
  // 然后移除指定的 ID
  // 最后新剩下的 ID 格式化输出

  std::string line;
  std::getline(std::cin, line);

  std::vector<int> id_list;
  std::stringstream ss(line);

  std::getline(std::cin, line);
  const int removed_id = std::stoi(line);

  while (std::getline(ss, line, ',')) {
    std::cout << "line: " << line << std::endl;

    const int index = line.find('-');
    if (index == std::string::npos) {
      const int vlan_id = std::stoi(line);
      id_list.push_back(vlan_id);
    } else {
      const std::string start = line.substr(0, index);
      const int start_id = std::stoi(start);
      const std::string end = line.substr(index + 1);
      const int end_id = std::stoi(end);
      for (int vlan_id = start_id; vlan_id <= end_id; ++vlan_id) {
        id_list.push_back(vlan_id);
      }
    }
  }

  // 排序
  std::sort(id_list.begin(), id_list.end());

  // 移除指定的 ID
  auto removed_id_iter = std::find(id_list.begin(), id_list.end(), removed_id);
  if (removed_id_iter != id_list.end()) {
    id_list.erase(removed_id_iter);
  }

  int start_id = -1;
  int last_id = -1;
  std::vector<std::string> out;
  const size_t kBufLen = 64;
  char buf[kBufLen + 1];

  for (int vlan_id : id_list) {
    if (last_id + 1 == vlan_id) {
      // 连续 ID
      last_id = vlan_id;
    } else {
      // 重置连续 ID
      if (last_id == -1) {
        // pass
      } else if (last_id == start_id) {
        // 单个值
        out.push_back(std::to_string(start_id));
      } else {
        // 范围
        const int s_len = snprintf(buf, kBufLen, "%d-%d", start_id, last_id);
        out.emplace_back(buf, s_len);
      }
      start_id = vlan_id;
      last_id = vlan_id;
    }
  }

  // 处理最后一个元素
  if (last_id == start_id) {
    // 单个值
    out.push_back(std::to_string(start_id));
  } else{ 
    // 范围
    const int s_len = snprintf(buf, kBufLen, "%d-%d", start_id, last_id);
    out.emplace_back(buf, s_len);
  }

  for (int index = 0; index + 1 < out.size(); ++index) {
    std::cout << out[index] << ",";
  }
  std::cout << out[out.size() - 1] << std::endl;

  return 0;
}
