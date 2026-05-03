//! nuked-opn2
//!
//! Non-official Pure Rust port of Nuked-OPN2.
//! Original C implementation by nukeykt.

pub mod chip;
#[cfg(feature = "c-reference")]
pub mod ffi;
pub mod opn2;
pub mod tables;
#[cfg(all(test, feature = "c-reference"))]
mod tests;

pub use chip::Ym3438;

pub fn init() {
    println!("nuked-opn2 initialized");
}
