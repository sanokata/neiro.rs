//! nuked-opn2
//!
//! Non-official Pure Rust port of Nuked-OPN2.
//! Original C implementation by nukeykt.

pub mod chip;
pub mod ffi;
#[cfg(test)]
mod tests;

pub use chip::Ym3438;

pub fn init() {
    println!("nuked-opn2 initialized");
}
