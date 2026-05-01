//! nuked-opn2
//!
//! Non-official Pure Rust port of Nuked-OPN2.
//! Original C implementation by nukeykt.

use std::os::raw::{c_short, c_uchar, c_uint};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ym3438_t {
    pub cycles: u32,
    pub channel: u32,
    pub mol: i16,
    pub mor: i16,
    // Note: The actual struct is much larger.
    // For now, we only define the fields we might need to access immediately
    // or just treat it as an opaque blob for FFI.
    // To properly compare internal states, we will need the full definition.
    _opaque: [u8; 4096], // Rough buffer to prevent overflow if allocated in Rust
}

extern "C" {
    pub fn OPN2_Reset(chip: *mut ym3438_t);
    pub fn OPN2_SetChipType(chip_type: u32);
    pub fn OPN2_Clock(chip: *mut ym3438_t, buffer: *mut i16);
    pub fn OPN2_Write(chip: *mut ym3438_t, port: u32, data: u8);
    pub fn OPN2_SetTestPin(chip: *mut ym3438_t, value: u32);
    pub fn OPN2_ReadTestPin(chip: *mut ym3438_t) -> u32;
    pub fn OPN2_ReadIRQPin(chip: *mut ym3438_t) -> u32;
    pub fn OPN2_Read(chip: *mut ym3438_t, port: u32) -> u8;
}

pub fn init() {
    println!("nuked-opn2 (C reference) initialized");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_reset() {
        unsafe {
            let mut chip: ym3438_t = std::mem::zeroed();
            OPN2_Reset(&mut chip);
            assert_eq!(chip.cycles, 0);
        }
    }
}
