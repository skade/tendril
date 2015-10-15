// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(feature = "unstable", feature(core, nonzero, unsafe_no_drop_flag, filling_drop))]
#![cfg_attr(test, deny(warnings))]

#[macro_use] extern crate mac;
extern crate futf;
extern crate encoding;

pub use tendril::{Tendril, ByteTendril, StrTendril, SliceExt, ReadExt, SubtendrilError};
pub use tendril::{SendTendril, Atomicity, Atomic, NonAtomic};
pub use fmt::Format;

pub mod fmt;
pub mod stream;

mod util;
mod buf32;
mod tendril;

static OFLOW: &'static str = "tendril: overflow in buffer arithmetic";
