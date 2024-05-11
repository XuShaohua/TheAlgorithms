// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

const MAX_KEY: i32 = 1000_000;
const BUCKET_LEN: usize = 1000_000 + 1;
const DEFAULT_VALUE: i32 = -1;

#[derive(Debug, Clone)]
pub struct MyHashMap {
    bucket: [i32; BUCKET_LEN],
}

impl Default for MyHashMap {
    fn default() -> Self {
        Self::new()
    }
}

impl MyHashMap {
    /// Initializes the object with an empty map.
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self {
            bucket: [DEFAULT_VALUE; BUCKET_LEN],
        }
    }

    /// Inserts a (key, value) pair into the HashMap.
    ///
    /// If the key already exists in the map, update the corresponding value.
    pub fn put(&mut self, key: i32, value: i32) {
        debug_assert!((0..=MAX_KEY).contains(&key));
        self.bucket[key as usize] = value;
    }

    /// Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key.
    #[must_use]
    pub fn get(&self, key: i32) -> i32 {
        debug_assert!((0..=MAX_KEY).contains(&key));
        self.bucket[key as usize]
    }

    /// Removes the key and its corresponding value if the map contains the mapping for the key.
    pub fn remove(&mut self, key: i32) {
        debug_assert!((0..=MAX_KEY).contains(&key));
        self.bucket[key as usize] = DEFAULT_VALUE;
    }
}

fn check_solution() {
    let mut my_hash_map = MyHashMap::new();
    my_hash_map.put(1, 1); // The map is now [[1,1]]
    my_hash_map.put(2, 2); // The map is now [[1,1], [2,2]]
    assert_eq!(my_hash_map.get(1), 1); // return 1, The map is now [[1,1], [2,2]]
    assert_eq!(my_hash_map.get(3), -1); // return -1 (i.e., not found), The map is now [[1,1], [2,2]]
    my_hash_map.put(2, 1); // The map is now [[1,1], [2,1]] (i.e., update the existing value)
    assert_eq!(my_hash_map.get(2), 1); // return 1, The map is now [[1,1], [2,1]]
    my_hash_map.remove(2); // remove the mapping for 2, The map is now [[1,1]]
    assert_eq!(my_hash_map.get(2), -1); // return -1 (i.e., not found), The map is now [[1,1]]
}

fn main() {
    check_solution();
}

#[cfg(test)]
mod tests {
    use super::check_solution;

    #[test]
    fn test_use_array() {
        check_solution();
    }
}
