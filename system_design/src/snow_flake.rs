// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::ops::Sub;
use std::time::{Duration, SystemTime, SystemTimeError};

const TWITTER_EPOCH: u64 = 1_288_834_974_657;

/// Generate global uuid based on SnowFlake algorithm from twitter.
#[derive(Debug, Clone)]
pub struct SnowFlake {
    twitter_epoch: Duration,
    now: u64,
    next_seq_no: u64,
    data_center_id: u64,
    machine_id: u64,
}

impl SnowFlake {
    /// Create a new instance of snow flake object.
    ///
    /// Note that only the least 5-bits of `data_center_id` and `machine_id` are used.
    pub fn new(data_center_id: u8, machine_id: u8) -> Result<Self, SystemTimeError> {
        let twitter_epoch = Duration::from_millis(TWITTER_EPOCH);
        let now = Self::timestamp_millis(twitter_epoch)?;
        Ok(Self {
            twitter_epoch,
            now,
            next_seq_no: 0,
            data_center_id: data_center_id as u64,
            machine_id: machine_id as u64,
        })
    }

    #[must_use]
    fn timestamp_millis(twitter_epoch: Duration) -> Result<u64, SystemTimeError> {
        let now = SystemTime::now();
        let duration = now
            .duration_since(SystemTime::UNIX_EPOCH)?
            .sub(twitter_epoch);
        Ok(duration.as_millis() as u64)
    }

    #[must_use]
    pub fn generate_id(&mut self) -> Result<u64, SystemTimeError> {
        let now = Self::timestamp_millis(self.twitter_epoch)?;
        debug_assert!(now >= self.now);

        if now == self.now {
            self.next_seq_no += 1;
            let id = self.next_id();
            Ok(id)
        } else {
            self.now = now;
            self.next_seq_no = 0;
            Ok(self.next_id())
        }
    }

    // | 1-bit | 41-bits   | 5-bits       | 5-bits    | 12-bits          |
    // | 0     | timestamp | datacenter id| machine id| sequence number  |
    #[must_use]
    #[inline]
    fn next_id(&self) -> u64 {
        println!("now: {}, seq no: {}", self.now, self.next_seq_no);
        self.now << (64 - 41)
            | (self.data_center_id >> 3 << 17)
            | (self.machine_id >> 3 << 12)
            | (self.next_seq_no >> 4)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::SnowFlake;

    #[test]
    fn test_generate_id() {
        let mut sf = SnowFlake::new(1, 2).unwrap();
        let mut set = HashSet::new();
        let id_count: usize = 20000;
        for _i in 0..id_count {
            let id = sf.generate_id().unwrap();
            set.insert(id);
            println!("id: {id}, 0b{id:0b}");
        }
        assert_eq!(set.len(), id_count);
    }
}
