#![cfg(feature = "c-reference")]

use crate::chip::Ym3438;

#[allow(non_snake_case)]
extern "C" {
    pub fn OPN2_Reset(chip: *mut Ym3438);
    pub fn OPN2_SetChipType(chip_type: u32);
    pub fn OPN2_Clock(chip: *mut Ym3438, buffer: *mut i16);
    pub fn OPN2_Write(chip: *mut Ym3438, port: u32, data: u8);
    pub fn OPN2_SetTestPin(chip: *mut Ym3438, value: u32);
    pub fn OPN2_ReadTestPin(chip: *mut Ym3438) -> u32;
    pub fn OPN2_ReadIRQPin(chip: *mut Ym3438) -> u32;
    pub fn OPN2_Read(chip: *mut Ym3438, port: u32) -> u8;

    // Function accessor for comparison tests
    pub fn OPN2_DoRegWriteTest(chip: *mut Ym3438);

    // Table accessors for comparison tests
    pub fn OPN2_GetLogsinRom(i: u32) -> u16;
    pub fn OPN2_GetExpRom(i: u32) -> u16;
    pub fn OPN2_GetFnNote(i: u32) -> u32;
    pub fn OPN2_GetEgStephi(row: u32, col: u32) -> u32;
    pub fn OPN2_GetEgAmShift(i: u32) -> u8;
    pub fn OPN2_GetPgDetune(i: u32) -> u32;
    pub fn OPN2_GetPgLfoSh1(row: u32, col: u32) -> u32;
    pub fn OPN2_GetPgLfoSh2(row: u32, col: u32) -> u32;
    pub fn OPN2_GetOpOffset(i: u32) -> u32;
    pub fn OPN2_GetChOffset(i: u32) -> u32;
    pub fn OPN2_GetLfoCycles(i: u32) -> u32;
    pub fn OPN2_GetFmAlgorithm(src: u32, row: u32, slot: u32) -> u32;
}
