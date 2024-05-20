// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#![deny(
    warnings,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic
)]
#![allow(clippy::module_name_repetitions)]

mod bubble_sort;
mod counting_sort;
mod double_sort;
mod gnome_sort;
mod insertion_sort;
mod merge_sort;
mod odd_even_sort;
mod quick_sort;
mod selection_sort;
mod shaker_sort;
mod shell_sort;
pub mod util;

pub use bubble_sort::bubble_sort;
pub use counting_sort::{counting_sort, counting_sort_generic};
pub use double_sort::double_sort;
pub use gnome_sort::gnome_sort;
pub use insertion_sort::{insertion_sort, insertion_sort_vanilla};
pub use merge_sort::merge_sort;
pub use odd_even_sort::odd_even_sort;
pub use quick_sort::quick_sort;
pub use selection_sort::selection_sort;
pub use shaker_sort::shaker_sort;
pub use shell_sort::shell_sort;
