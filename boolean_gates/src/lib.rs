// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Boolean algebra
//!
//! [Logic gate](https://en.wikipedia.org/wiki/Logic_gate)
//! [Boolean algebra](https://en.wikipedia.org/wiki/Boolean_algebra)
//! [Math of boolean algebra](https://plato.stanford.edu/entries/boolalg-math/)
//! [Logic gates in python](https://www.geeksforgeeks.org/logic-gates-in-python/)

#![deny(
    warnings,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic
)]

pub mod and_gate;
pub mod nand_gate;
pub mod nor_gate;
pub mod not_gate;
pub mod or_gate;
pub mod xnor_gate;
pub mod xor_gate;
