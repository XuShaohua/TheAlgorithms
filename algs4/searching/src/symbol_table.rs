// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[derive(Debug)]
pub struct SymbolTable<Key, Value> {
    keys: Vec<Key>,
    values: Vec<Value>,
    len: usize,
}

impl<Key, Value> SymbolTable<Key, Value>
where
    Key: PartialEq + PartialOrd + Clone,
    Value: PartialEq + PartialOrd + Clone,
{
    pub fn new() -> SymbolTable<Key, Value> {
        SymbolTable {
            keys: vec![],
            values: vec![],
            len: 0,
        }
    }

    pub fn put(&mut self, key: Key, val: Value) {
        self.keys.push(key);
        self.values.push(val);
        self.len += 1;
    }

    pub fn get(&self, _key: &Key) -> Option<Value> {
        unimplemented!()
    }

    pub fn delete(&self, _key: &Key) {
        unimplemented!()
    }

    pub fn contains(&self, key: &Key) -> bool {
        self.get(key).is_none()
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    pub fn size(&self) -> usize {
        return self.len;
    }

    pub fn keys(&self) -> Vec<Key> {
        unimplemented!()
    }

    pub fn min(&self) -> Option<Key> {
        unimplemented!()
    }

    pub fn max(&self) -> Option<Key> {
        unimplemented!()
    }

    pub fn floor(&self, _key: Key) -> Option<Key> {
        unimplemented!()
    }

    pub fn ceiling(&self, _key: Key) -> Option<Key> {
        unimplemented!()
    }

    pub fn rank(&self, _key: Key) -> usize {
        unimplemented!()
    }

    pub fn select(&self, _n: usize) -> Option<Key> {
        unimplemented!()
    }
}
