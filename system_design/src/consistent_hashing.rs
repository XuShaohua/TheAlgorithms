// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::btree_map::Entry::{Occupied, Vacant};
use std::collections::BTreeMap;
use std::fmt;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::Bound;

pub const SLOT_MAX: u16 = 3600;

#[derive(Debug, Clone)]
pub struct ConsistentHash {
    nodes: BTreeMap<u16, String>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ConsistentHashError {
    OutOfRange,
    SlotConflict,
}

impl fmt::Display for ConsistentHashError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::OutOfRange => "slot out of range",
            Self::SlotConflict => "slot conflict, another node on the same slot",
        };
        write!(f, "{}", s)
    }
}

impl std::error::Error for ConsistentHashError {}

impl ConsistentHash {
    pub fn new(nodes: Vec<(String, Vec<u16>)>) -> Result<Self, ConsistentHashError> {
        let mut node_map = BTreeMap::new();
        for (node_name, slots) in nodes {
            for slot in slots {
                Self::check_slot(slot)?;
                node_map.insert(slot, node_name.clone());
            }
        }
        Ok(Self { nodes: node_map })
    }

    fn check_slot(slot: u16) -> Result<(), ConsistentHashError> {
        if slot >= SLOT_MAX {
            Err(ConsistentHashError::OutOfRange)
        } else {
            Ok(())
        }
    }

    pub fn add_node(
        &mut self,
        node_name: String,
        slots: &[u16],
    ) -> Result<(), ConsistentHashError> {
        for &slot in slots {
            Self::check_slot(slot)?;
            match self.nodes.entry(slot) {
                Vacant(vacant) => {
                    vacant.insert(node_name.clone());
                }
                Occupied(_occupied) => return Err(ConsistentHashError::SlotConflict),
            }
        }
        Ok(())
    }

    pub fn remove_node(&mut self, node_name: &str) {
        let mut slots = Vec::new();
        for (&slot, name) in &self.nodes {
            if name == node_name {
                slots.push(slot);
            }
        }
        for slot in slots {
            self.nodes.remove(&slot);
        }
    }

    #[must_use]
    pub fn get_node<T: Hash>(&self, obj: T) -> Option<&str> {
        let mut s = DefaultHasher::new();
        obj.hash(&mut s);
        let obj_hash = s.finish();
        // Map object hash to slots.
        let slot = (obj_hash % SLOT_MAX as u64) as u16;

        let cursor = self.nodes.lower_bound(Bound::Included(&slot));
        if let Some((&next_slot, node_name)) = cursor.peek_next() {
            debug_assert!(next_slot >= slot);
            Some(node_name)
        } else if let Some((&first_slot, node_name)) = self.nodes.first_key_value() {
            // If slot is out of range, get the first node on the circle.
            debug_assert!(first_slot < slot);
            Some(node_name)
        } else {
            None
        }
    }
}
