// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use stack::array_stack::{ArrayStack, StackError};

fn main() {
    let mut stack: ArrayStack<i32> = ArrayStack::with_capacity(4);
    assert!(stack.is_empty());
    let ret = stack.push(10);
    assert!(ret.is_ok());
    let ret = stack.push(20);
    assert!(ret.is_ok());
    let ret = stack.push(30);
    assert!(ret.is_ok());
    let ret = stack.push(40);
    assert!(ret.is_ok());

    let ret = stack.push(50);
    assert_eq!(ret, Err(StackError::StackFull));

    assert!(stack.is_full());
    assert_eq!(stack.len(), 4);

    let ret = stack.pop();
    assert_eq!(ret, Ok(40));
    assert!(!stack.is_full());
    let ret = stack.pop();
    assert_eq!(ret, Ok(30));
    let ret = stack.pop();
    assert_eq!(ret, Ok(20));
    let ret = stack.pop();
    assert_eq!(ret, Ok(10));

    assert!(!stack.is_full());
    assert!(stack.is_empty());

    let ret = stack.pop();
    assert_eq!(ret, Err(StackError::StackEmpty));
}