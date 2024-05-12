// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

const BUCKET_LEN: usize = 1_000_000 + 1;
const MAX_KEY: i32 = 1_000_000;

#[derive(Debug, Clone)]
pub struct MyHashSet {
    bucket: [bool; BUCKET_LEN],
}

impl Default for MyHashSet {
    fn default() -> Self {
        Self::new()
    }
}

impl MyHashSet {
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self {
            bucket: [false; BUCKET_LEN],
        }
    }

    /// Inserts the value key into the HashSet.
    pub fn add(&mut self, key: i32) {
        assert!((0..=MAX_KEY).contains(&key));
        self.bucket[key as usize] = true;
    }

    /// Removes the value key in the HashSet. If key does not exist in the HashSet, do nothing.
    pub fn remove(&mut self, key: i32) {
        assert!((0..=MAX_KEY).contains(&key));
        self.bucket[key as usize] = false;
    }

    /// Returns whether the value key exists in the HashSet or not.
    #[must_use]
    pub fn contains(&self, key: i32) -> bool {
        assert!((0..=MAX_KEY).contains(&key));
        self.bucket[key as usize]
    }
}

fn check_solution() {
    let mut my_hash_set = MyHashSet::new();
    my_hash_set.add(1); // set = [1]
    my_hash_set.add(2); // set = [1, 2]
    assert!(my_hash_set.contains(1)); // return True
    assert!(!my_hash_set.contains(3)); // return False, (not found)
    my_hash_set.add(2); // set = [1, 2]
    assert!(my_hash_set.contains(2)); // return True
    my_hash_set.remove(2); // set = [1]
    assert!(!my_hash_set.contains(2)); // return False, (already removed)
}

fn main() {
    check_solution();
}

#[cfg(test)]
mod tests {
    use super::check_solution;

    #[test]
    fn test_hashset() {
        check_solution();
    }
}
