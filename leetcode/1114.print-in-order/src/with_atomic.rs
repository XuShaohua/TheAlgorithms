// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#![allow(dead_code)]

use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::thread;

struct Foo {
    counter: AtomicI32,
}

fn print_first() {
    print!("first");
}

fn print_second() {
    print!("second");
}

fn print_third() {
    print!("third");
}

impl Foo {
    #[must_use]
    pub fn new() -> Self {
        Self {
            counter: AtomicI32::new(0),
        }
    }

    pub fn first(&self) {
        print_first();
        self.counter.store(1, Ordering::SeqCst);
    }

    pub fn second(&self) {
        while self.counter.load(Ordering::SeqCst) != 1 {
            // empty
        }
        print_second();
        self.counter.store(2, Ordering::SeqCst);
    }

    pub fn third(&self) {
        while self.counter.load(Ordering::SeqCst) != 2 {
            // empty
        }
        print_third();
    }
}

pub fn run() {
    let foo = Arc::new(Foo::new());

    let a = {
        let foo = foo.clone();
        thread::spawn(move || {
            foo.first();
        })
    };
    let b = {
        let foo = foo.clone();
        thread::spawn(move || {
            foo.second();
        })
    };
    let c = {
        thread::spawn(move || {
            foo.third();
        })
    };
    let _ = a.join();
    let _ = b.join();
    let _ = c.join();
}