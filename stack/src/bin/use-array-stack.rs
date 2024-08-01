// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use stack::array_stack::ArrayStack;

fn main() {
    let mut stack: ArrayStack<i32> = ArrayStack::new(4);
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
    assert_eq!(ret, Err(50));

    assert_eq!(stack.capacity(), stack.len());

    let ret = stack.pop();
    assert_eq!(ret, Some(40));
    assert!(!stack.is_empty());
    let ret = stack.pop();
    assert_eq!(ret, Some(30));
    let ret = stack.pop();
    assert_eq!(ret, Some(20));
    let ret = stack.pop();
    assert_eq!(ret, Some(10));

    assert!(stack.is_empty());

    let ret = stack.pop();
    assert_eq!(ret, None);
}
