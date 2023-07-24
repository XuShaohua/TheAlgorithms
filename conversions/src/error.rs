// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BinaryError {
    OutOfRange,
    NonBinaryValue,
}
