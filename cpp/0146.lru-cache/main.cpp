// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <cassert>
#include <cstdio>

#include <algorithm>
#include <list>
#include <unordered_map>

class LRUCache {
 public:
  explicit LRUCache(int capacity) : capacity_(capacity) {
    iter_map_.reserve(capacity);
  }
  ~LRUCache() {
    iter_map_.clear();
    items_.clear();
  }

  int get(int key) {
    const auto iter = iter_map_.find(key);
    if (iter == iter_map_.end()) {
      return -1;
    }

    // found, change order in list if needed.
    const int value = iter->second->second;
    if (iter->second != items_.begin()) {
      items_.erase(iter->second);
      const auto new_iter = items_.emplace(items_.begin(), std::make_pair(key, value));
      iter_map_.emplace(key, new_iter);
    }
    return value;
  }

  void put(int key, int value) {
    printf("put(%d, %d)\n", key, value);
    auto old_iter = iter_map_.find(key);
    if (old_iter == iter_map_.end()) {
      // not found, check capacity.
      if (iter_map_.size() >= capacity_) {
        const int front_key = items_.back().first;
        items_.pop_back();
        iter_map_.erase(front_key);
      }

      // push to front of list.
      const auto new_iter = items_.emplace(items_.begin(), std::make_pair(key, value));
      iter_map_.emplace(key, new_iter);
    } else {
      // found, update value
      old_iter->second->second = value;
      if (old_iter->second != items_.begin()) {
        // change order in list if needed.
        items_.erase(old_iter->second);
        const auto new_iter = items_.emplace(items_.begin(), std::make_pair(key, value));
        iter_map_.emplace(key, new_iter);
      }
    }
  }

  void debug_print() const {
    printf("iters: ");
    for (const auto& item: iter_map_) {
      printf("%d, ", item.first);
    }
    printf("items: ");
    for (const auto& item: items_) {
      printf("(%d, %d), ", item.first, item.second);
    }
    printf("\n");
  }

 private:
  using Items = std::list<std::pair<int, int>>;
  using ItemIter = std::list<std::pair<int, int>>::iterator;

  int capacity_;
  // (key, value) pair
  Items items_;
  // key -> list iterator
  std::unordered_map<int, ItemIter> iter_map_;
};

void checkSolution() {
  LRUCache cache(2);
  cache.put(1, 1);            // cache is {1=1}
  cache.debug_print();
  cache.put(2, 2);            // cache is {1=1, 2=2}
  cache.debug_print();
  assert(cache.get(1) == 1);  // return 1
  cache.debug_print();
  cache.put(3, 3);            // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
  cache.debug_print();
  assert(cache.get(2) == -1); // returns -1 (not found)
  cache.put(4, 4);            // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
  cache.debug_print();
  assert(cache.get(1) == -1); // return -1 (not found)
  assert(cache.get(3) == 3);  // return 3
  assert(cache.get(4) == 4);  // return 4
}

int main() {
  checkSolution();
  return 0;
}
