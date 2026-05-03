use crate::chip::Ym3438;

impl Ym3438 {
    /// Update IO signals and busy counter (port of OPN2_DoIO)
    pub fn do_io(&mut self) {
        // Write signal check
        self.write_a_en = if (self.write_a & 0x03) == 0x01 { 1 } else { 0 };
        self.write_d_en = if (self.write_d & 0x03) == 0x01 { 1 } else { 0 };
        self.write_a <<= 1;
        self.write_d <<= 1;

        // Busy counter
        self.busy = self.write_busy;
        self.write_busy_cnt = self.write_busy_cnt.wrapping_add(self.write_busy);
        self.write_busy =
            if (self.write_busy != 0 && (self.write_busy_cnt >> 5) == 0) || self.write_d_en != 0 {
                1
            } else {
                0
            };
        self.write_busy_cnt &= 0x1f;
    }
}
