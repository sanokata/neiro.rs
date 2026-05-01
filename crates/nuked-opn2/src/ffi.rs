use crate::chip::Ym3438;

#[allow(non_snake_case)]
extern "C" {
    /// OPN2_Reset
    pub fn OPN2_Reset(chip: *mut Ym3438);

    /// OPN2_SetChipType
    pub fn OPN2_SetChipType(chip_type: u32);

    /// OPN2_Clock
    pub fn OPN2_Clock(chip: *mut Ym3438, buffer: *mut i16);

    /// OPN2_Write
    pub fn OPN2_Write(chip: *mut Ym3438, port: u32, data: u8);

    /// OPN2_SetTestPin
    pub fn OPN2_SetTestPin(chip: *mut Ym3438, value: u32);

    /// OPN2_ReadTestPin
    pub fn OPN2_ReadTestPin(chip: *mut Ym3438) -> u32;

    /// OPN2_ReadIRQPin
    pub fn OPN2_ReadIRQPin(chip: *mut Ym3438) -> u32;

    /// OPN2_Read
    pub fn OPN2_Read(chip: *mut Ym3438, port: u32) -> u8;
}
