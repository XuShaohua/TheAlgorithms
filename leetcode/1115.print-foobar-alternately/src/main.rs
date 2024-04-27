// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

use std::hint;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

struct FooBar {
    n: i32,
    toggle: AtomicBool,
}

fn print_foo() {
    print!("foo");
}
fn print_bar() {
    print!("bar");
}

impl FooBar {
    pub fn new(n: i32) -> Self {
        Self {
            n,
            toggle: AtomicBool::new(false),
        }
    }
    pub fn foo(&self) {
        for _i in 0..self.n {
            while self.toggle.load(Ordering::SeqCst) {
                hint::spin_loop();
            }
            print_foo();
            self.toggle.store(true, Ordering::SeqCst);
        }
    }

    pub fn bar(&self) {
        for _i in 0..self.n {
            while !self.toggle.load(Ordering::SeqCst) {
                hint::spin_loop();
            }
            print_bar();
            self.toggle.store(false, Ordering::SeqCst);
        }
    }
}

fn main() {
    let n = 100;
    let foobar = Arc::new(FooBar::new(n));
    let a = {
        let foobar = foobar.clone();
        thread::spawn(move || {
            foobar.foo();
        })
    };
    let b = thread::spawn(move || {
        foobar.bar();
    });
    let _ = a.join();
    let _ = b.join();
}
