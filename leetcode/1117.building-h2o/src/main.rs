// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct H2O {
    water: String,
    // true -> hydrogen
    // false -> oxygen
    mutex: Mutex<bool>,
    cvar: Condvar,
}

fn print_atom(atom: char) {
    print!("{atom}");
}

impl H2O {
    const OXYGEN: char = 'O';
    const HYDROGEN: char = 'H';
    const RELEASE_HYDROGEN: bool = true;

    pub fn new(water: &str) -> Self {
        Self {
            water: water.to_owned(),
            mutex: Mutex::new(Self::RELEASE_HYDROGEN),
            cvar: Condvar::new(),
        }
    }

    pub fn oxygen(&self) {
        let mut index = 0;
        loop {
            let mut mutex = self.mutex.lock().unwrap();
            while *mutex == Self::RELEASE_HYDROGEN {
                mutex = self.cvar.wait(mutex).unwrap();
            }

            let mut count = 0;
            while index < self.water.len() && count < 1 {
                if self.water.chars().nth(index) == Some(Self::OXYGEN) {
                    print_atom(Self::OXYGEN);
                    count += 1;
                }
                index += 1;
            }

            if count == 0 {
                return;
            }
            assert_eq!(count, 1);
            *mutex = Self::RELEASE_HYDROGEN;
            self.cvar.notify_all();
        }
    }

    pub fn hydrogen(&self) {
        let mut index = 0;
        let len = self.water.len();
        loop {
            let mut mutex = self.mutex.lock().unwrap();
            while *mutex != Self::RELEASE_HYDROGEN {
                mutex = self.cvar.wait(mutex).unwrap();
            }

            let mut count = 0;
            while index < len && count < 2 {
                if self.water.chars().nth(index) == Some(Self::HYDROGEN) {
                    print_atom(Self::HYDROGEN);
                    count += 1;
                }
                index += 1;
            }

            if count == 0 {
                *mutex = !Self::RELEASE_HYDROGEN;
                self.cvar.notify_all();
                return;
            }
            assert_eq!(count, 2);
            *mutex = !Self::RELEASE_HYDROGEN;
            self.cvar.notify_all();
        }
    }
}

fn main() {
    let s = "HOH";
    let s = "OOHHHH";
    let h2o = Arc::new(H2O::new(s));
    let a = {
        let h2o = Arc::clone(&h2o);
        thread::spawn(move || {
            h2o.oxygen();
        })
    };
    let b = {
        thread::spawn(move || {
            h2o.hydrogen();
        })
    };
    let _ = a.join();
    let _ = b.join();
}
