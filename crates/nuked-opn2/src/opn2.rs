use crate::chip::Ym3438;
use crate::tables::{CH_OFFSET, FN_NOTE, OP_OFFSET};

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

    pub fn do_reg_write(&mut self) {
        let slot = (self.cycles % 12) as usize;
        let channel = self.channel as usize;

        if self.write_fm_data != 0 {
            // Slot registers
            if OP_OFFSET[slot] == (self.address & 0x107) as u32 {
                let slot = if self.address & 0x08 != 0 { slot + 12 } else { slot }; // OP2, OP4
                let address = self.address & 0xf0;
                match address {
                    0x30 => { // DT, MULTI
                        let m = self.data & 0x0f;
                        self.multi[slot] = if m == 0 { 1 } else { m << 1 };
                        self.dt[slot] = (self.data >> 4) & 0x07;
                    }
                    0x40 => { // TL
                        self.tl[slot] = self.data & 0x7f;
                    }
                    0x50 => { // KS, AR
                        self.ar[slot] = self.data & 0x1f;
                        self.ks[slot] = (self.data >> 6) & 0x03;
                    }
                    0x60 => { // AM, DR
                        self.dr[slot] = self.data & 0x1f;
                        self.am[slot] = (self.data >> 7) & 0x01;
                    }
                    0x70 => { // SR
                        self.sr[slot] = self.data & 0x1f;
                    }
                    0x80 => { // SL, RR
                        self.rr[slot] = self.data & 0x0f;
                        let sl = (self.data >> 4) & 0x0f;
                        self.sl[slot] = sl | ((sl + 1) & 0x10);
                    }
                    0x90 => { // SSG-EG
                        self.ssg_eg[slot] = self.data & 0x0f;
                    }
                    _ => {}
                }
            }

            // Channel registers
            if CH_OFFSET[channel] == (self.address & 0x103) as u32 {
                let address = self.address & 0xfc;
                match address {
                    0xa0 => {
                        self.fnum[channel] =
                            self.data as u16 | ((self.reg_a4 as u16 & 0x07) << 8);
                        self.block[channel] = (self.reg_a4 >> 3) & 0x07;
                        self.kcode[channel] = (self.block[channel] << 2)
                            | FN_NOTE[(self.fnum[channel] >> 7) as usize] as u8;
                    }
                    0xa4 => {
                        self.reg_a4 = self.data;
                    }
                    0xa8 => {
                        self.fnum_3ch[channel] =
                            self.data as u16 | ((self.reg_ac as u16 & 0x07) << 8);
                        self.block_3ch[channel] = (self.reg_ac >> 3) & 0x07;
                        self.kcode_3ch[channel] = (self.block_3ch[channel] << 2)
                            | FN_NOTE[(self.fnum_3ch[channel] >> 7) as usize] as u8;
                    }
                    0xac => {
                        self.reg_ac = self.data;
                    }
                    0xb0 => {
                        self.connect[channel] = self.data & 0x07;
                        self.fb[channel] = (self.data >> 3) & 0x07;
                    }
                    0xb4 => {
                        self.pms[channel] = self.data & 0x07;
                        self.ams[channel] = (self.data >> 4) & 0x03;
                        self.pan_l[channel] = (self.data >> 7) & 0x01;
                        self.pan_r[channel] = (self.data >> 6) & 0x01;
                    }
                    _ => {}
                }
            }
        }

        if self.write_a_en != 0 || self.write_d_en != 0 {
            if self.write_a_en != 0 {
                self.write_fm_data = 0;
            }
            if self.write_fm_address != 0 && self.write_d_en != 0 {
                self.write_fm_data = 1;
            }

            // Address
            if self.write_a_en != 0 {
                if (self.write_data & 0xf0) != 0x00 {
                    // FM write
                    self.address = self.write_data;
                    self.write_fm_address = 1;
                } else {
                    // SSG write
                    self.write_fm_address = 0;
                }
            }

            // FM Mode data
            if self.write_d_en != 0 && (self.write_data & 0x100) == 0 {
                match self.write_fm_mode_a {
                    0x21 => { // LSI test 1
                        for i in 0..8 {
                            self.mode_test_21[i] = ((self.write_data >> i) & 0x01) as u8;
                        }
                    }
                    0x22 => { // LFO control
                        self.lfo_en = if (self.write_data >> 3) & 0x01 != 0 { 0x7f } else { 0 };
                        self.lfo_freq = (self.write_data & 0x07) as u8;
                    }
                    0x24 => { // Timer A high byte
                        self.timer_a_reg &= 0x03;
                        self.timer_a_reg |= (self.write_data & 0xff) << 2;
                    }
                    0x25 => { // Timer A low 2 bits
                        self.timer_a_reg &= 0x3fc;
                        self.timer_a_reg |= self.write_data & 0x03;
                    }
                    0x26 => { // Timer B
                        self.timer_b_reg = self.write_data & 0xff;
                    }
                    0x27 => { // CSM, Timer control
                        self.mode_ch3 = ((self.write_data & 0xc0) >> 6) as u8;
                        self.mode_csm = (self.mode_ch3 == 2) as u8;
                        self.timer_a_load = (self.write_data & 0x01) as u8;
                        self.timer_a_enable = ((self.write_data >> 2) & 0x01) as u8;
                        self.timer_a_reset = ((self.write_data >> 4) & 0x01) as u8;
                        self.timer_b_load = ((self.write_data >> 1) & 0x01) as u8;
                        self.timer_b_enable = ((self.write_data >> 3) & 0x01) as u8;
                        self.timer_b_reset = ((self.write_data >> 5) & 0x01) as u8;
                    }
                    0x28 => { // Key on/off
                        for i in 0..4 {
                            self.mode_kon_operator[i] =
                                ((self.write_data >> (4 + i)) & 0x01) as u8;
                        }
                        self.mode_kon_channel = if (self.write_data & 0x03) == 0x03 {
                            0xff
                        } else {
                            ((self.write_data & 0x03) + ((self.write_data >> 2) & 1) * 3) as u8
                        };
                    }
                    0x2a => { // DAC data
                        self.dacdata &= 0x01;
                        self.dacdata |= ((self.write_data ^ 0x80) << 1) as i16;
                    }
                    0x2b => { // DAC enable
                        self.dacen = (self.write_data >> 7) as u8;
                    }
                    0x2c => { // LSI test 2
                        for i in 0..8 {
                            self.mode_test_2c[i] = ((self.write_data >> i) & 0x01) as u8;
                        }
                        self.dacdata &= 0x1fe;
                        self.dacdata |= self.mode_test_2c[3] as i16;
                        self.eg_custom_timer =
                            (self.mode_test_2c[7] == 0 && self.mode_test_2c[6] != 0) as u8;
                    }
                    _ => {}
                }
            }

            if self.write_a_en != 0 {
                self.write_fm_mode_a = self.write_data & 0x1ff;
            }
        }

        if self.write_fm_data != 0 {
            self.data = (self.write_data & 0xff) as u8;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // slot=0, ch=0: cycles=0, address & 0x107 = op_offset[0] = 0x000
    fn slot_chip(address: u16, data: u8) -> Ym3438 {
        let mut c = Ym3438::default();
        c.cycles = 0;
        c.write_fm_data = 1;
        c.address = address;
        c.data = data;
        c
    }

    // channel=0: channel=0, address & 0x103 = ch_offset[0] = 0x000
    fn ch_chip(address: u16, data: u8) -> Ym3438 {
        let mut c = Ym3438::default();
        c.channel = 0;
        c.write_fm_data = 1;
        c.address = address;
        c.data = data;
        c
    }

    // FM mode data write (write_d_en=1, mode address already latched)
    fn mode_chip(write_fm_mode_a: u16, write_data: u16) -> Ym3438 {
        let mut c = Ym3438::default();
        c.write_d_en = 1;
        c.write_fm_mode_a = write_fm_mode_a;
        c.write_data = write_data;
        c
    }

    #[test]
    fn slot_multi_zero_becomes_one() {
        let mut chip = slot_chip(0x030, 0x00); // MULTI=0
        chip.do_reg_write();
        assert_eq!(chip.multi[0], 1);
    }

    #[test]
    fn slot_multi_nonzero_doubled() {
        let mut chip = slot_chip(0x030, 0x05); // MULTI=5
        chip.do_reg_write();
        assert_eq!(chip.multi[0], 10);
    }

    #[test]
    fn slot_multi_op2_uses_slot_plus_12() {
        let mut chip = slot_chip(0x038, 0x03); // bit3=1 → OP2/OP4 → slot+12
        chip.do_reg_write();
        assert_eq!(chip.multi[12], 6);
        assert_eq!(chip.multi[0], 0); // slot 0 は未変更
    }

    #[test]
    fn slot_sl_max_maps_to_0x1f() {
        let mut chip = slot_chip(0x080, 0xf0); // SL=0xf
        chip.do_reg_write();
        assert_eq!(chip.sl[0], 0x1f);
    }

    #[test]
    fn slot_sl_nonmax_no_extra_bit() {
        let mut chip = slot_chip(0x080, 0xe0); // SL=0xe
        chip.do_reg_write();
        assert_eq!(chip.sl[0], 0x0e); // 0xf+1 = 0x10, 0x10 & 0x10 = 0x10 → 0x0e | 0 = 0x0e... wait
        // SL=0xe: (0xe+1)&0x10 = 0xf & 0x10 = 0x00 → sl = 0x0e
    }

    #[test]
    fn mode_csm_set_when_mode_ch3_eq_2() {
        let mut chip = mode_chip(0x27, 0x80); // mode_ch3 = 0xc0>>6 = 2
        chip.do_reg_write();
        assert_eq!(chip.mode_ch3, 2);
        assert_eq!(chip.mode_csm, 1);
    }

    #[test]
    fn mode_csm_clear_when_mode_ch3_ne_2() {
        let mut chip = mode_chip(0x27, 0x40); // mode_ch3 = 1
        chip.mode_csm = 1; // 事前にセット
        chip.do_reg_write();
        assert_eq!(chip.mode_ch3, 1);
        assert_eq!(chip.mode_csm, 0);
    }

    #[test]
    fn mode_kon_invalid_channel_0x03() {
        let mut chip = mode_chip(0x28, 0x03); // channel bits = 0x03 → invalid
        chip.do_reg_write();
        assert_eq!(chip.mode_kon_channel, 0xff);
    }

    #[test]
    fn mode_lfo_enable() {
        let mut chip = mode_chip(0x22, 0x08); // bit3=1 → enable
        chip.do_reg_write();
        assert_eq!(chip.lfo_en, 0x7f);
        assert_eq!(chip.lfo_freq, 0);
    }

    #[test]
    fn mode_lfo_disable() {
        let mut chip = mode_chip(0x22, 0x05); // bit3=0 → disable, freq=5
        chip.lfo_en = 0x7f;
        chip.do_reg_write();
        assert_eq!(chip.lfo_en, 0);
        assert_eq!(chip.lfo_freq, 5);
    }

    #[test]
    fn mode_dac_enable() {
        let mut chip = mode_chip(0x2b, 0x80); // bit7=1
        chip.do_reg_write();
        assert_eq!(chip.dacen, 1);
    }

    // C リファレンス比較テスト
    #[cfg(feature = "c-reference")]
    mod c_reference {
        use super::*;
        use crate::ffi::OPN2_DoRegWriteTest;

        fn run(setup: Ym3438) -> (Ym3438, Ym3438) {
            let mut rust = setup;
            let mut c = setup;
            rust.do_reg_write();
            unsafe { OPN2_DoRegWriteTest(&mut c); }
            (rust, c)
        }

        #[test]
        fn slot_0x30_dt_multi() {
            let (r, c) = run(slot_chip(0x030, 0x73)); // DT=7, MULTI=3
            assert_eq!(r.dt[0], c.dt[0], "dt");
            assert_eq!(r.multi[0], c.multi[0], "multi");
        }

        #[test]
        fn slot_0x30_multi_zero() {
            let (r, c) = run(slot_chip(0x030, 0x70)); // DT=7, MULTI=0
            assert_eq!(r.multi[0], c.multi[0], "multi zero edge case");
        }

        #[test]
        fn slot_0x40_tl() {
            let (r, c) = run(slot_chip(0x040, 0x5a));
            assert_eq!(r.tl[0], c.tl[0]);
        }

        #[test]
        fn slot_0x50_ks_ar() {
            let (r, c) = run(slot_chip(0x050, 0xdf));
            assert_eq!(r.ar[0], c.ar[0], "ar");
            assert_eq!(r.ks[0], c.ks[0], "ks");
        }

        #[test]
        fn slot_0x60_am_dr() {
            let (r, c) = run(slot_chip(0x060, 0x9f));
            assert_eq!(r.dr[0], c.dr[0], "dr");
            assert_eq!(r.am[0], c.am[0], "am");
        }

        #[test]
        fn slot_0x70_sr() {
            let (r, c) = run(slot_chip(0x070, 0x1f));
            assert_eq!(r.sr[0], c.sr[0]);
        }

        #[test]
        fn slot_0x80_sl_rr() {
            let (r, c) = run(slot_chip(0x080, 0xf7));
            assert_eq!(r.rr[0], c.rr[0], "rr");
            assert_eq!(r.sl[0], c.sl[0], "sl");
        }

        #[test]
        fn slot_0x80_sl_max() {
            let (r, c) = run(slot_chip(0x080, 0xf0)); // SL=0xf
            assert_eq!(r.sl[0], c.sl[0], "sl max");
        }

        #[test]
        fn slot_0x90_ssg_eg() {
            let (r, c) = run(slot_chip(0x090, 0x0f));
            assert_eq!(r.ssg_eg[0], c.ssg_eg[0]);
        }

        #[test]
        fn channel_0xa4_reg_a4() {
            let (r, c) = run(ch_chip(0x0a4, 0x3b));
            assert_eq!(r.reg_a4, c.reg_a4);
        }

        #[test]
        fn channel_0xa0_fnum_block_kcode() {
            let mut chip = ch_chip(0x0a0, 0x56);
            chip.reg_a4 = 0x23; // block=4, fnum_hi=3
            let (r, c) = run(chip);
            assert_eq!(r.fnum[0], c.fnum[0], "fnum");
            assert_eq!(r.block[0], c.block[0], "block");
            assert_eq!(r.kcode[0], c.kcode[0], "kcode");
        }

        #[test]
        fn channel_0xac_reg_ac() {
            let (r, c) = run(ch_chip(0x0ac, 0x1c));
            assert_eq!(r.reg_ac, c.reg_ac);
        }

        #[test]
        fn channel_0xa8_fnum3ch() {
            let mut chip = ch_chip(0x0a8, 0x78);
            chip.reg_ac = 0x15;
            let (r, c) = run(chip);
            assert_eq!(r.fnum_3ch[0], c.fnum_3ch[0], "fnum_3ch");
            assert_eq!(r.block_3ch[0], c.block_3ch[0], "block_3ch");
            assert_eq!(r.kcode_3ch[0], c.kcode_3ch[0], "kcode_3ch");
        }

        #[test]
        fn channel_0xb0_connect_fb() {
            let (r, c) = run(ch_chip(0x0b0, 0x3f));
            assert_eq!(r.connect[0], c.connect[0], "connect");
            assert_eq!(r.fb[0], c.fb[0], "fb");
        }

        #[test]
        fn channel_0xb4_pan_lfo() {
            let (r, c) = run(ch_chip(0x0b4, 0xf7));
            assert_eq!(r.pms[0], c.pms[0], "pms");
            assert_eq!(r.ams[0], c.ams[0], "ams");
            assert_eq!(r.pan_l[0], c.pan_l[0], "pan_l");
            assert_eq!(r.pan_r[0], c.pan_r[0], "pan_r");
        }

        #[test]
        fn mode_0x22_lfo_control() {
            let (r, c) = run(mode_chip(0x22, 0x0b)); // enable, freq=3
            assert_eq!(r.lfo_en, c.lfo_en, "lfo_en");
            assert_eq!(r.lfo_freq, c.lfo_freq, "lfo_freq");
        }

        #[test]
        fn mode_0x24_timer_a_high() {
            let (r, c) = run(mode_chip(0x24, 0xab));
            assert_eq!(r.timer_a_reg, c.timer_a_reg);
        }

        #[test]
        fn mode_0x25_timer_a_low() {
            let (r, c) = run(mode_chip(0x25, 0x03));
            assert_eq!(r.timer_a_reg, c.timer_a_reg);
        }

        #[test]
        fn mode_0x26_timer_b() {
            let (r, c) = run(mode_chip(0x26, 0xcd));
            assert_eq!(r.timer_b_reg, c.timer_b_reg);
        }

        #[test]
        fn mode_0x27_timer_control() {
            let (r, c) = run(mode_chip(0x27, 0xff));
            assert_eq!(r.mode_ch3, c.mode_ch3, "mode_ch3");
            assert_eq!(r.mode_csm, c.mode_csm, "mode_csm");
            assert_eq!(r.timer_a_load, c.timer_a_load, "timer_a_load");
            assert_eq!(r.timer_a_enable, c.timer_a_enable, "timer_a_enable");
            assert_eq!(r.timer_a_reset, c.timer_a_reset, "timer_a_reset");
            assert_eq!(r.timer_b_load, c.timer_b_load, "timer_b_load");
            assert_eq!(r.timer_b_enable, c.timer_b_enable, "timer_b_enable");
            assert_eq!(r.timer_b_reset, c.timer_b_reset, "timer_b_reset");
        }

        #[test]
        fn mode_0x28_key_on() {
            let (r, c) = run(mode_chip(0x28, 0xf1)); // ch=1, bank=0, all ops on
            assert_eq!(r.mode_kon_channel, c.mode_kon_channel, "channel");
            assert_eq!(r.mode_kon_operator, c.mode_kon_operator, "operators");
        }

        #[test]
        fn mode_0x28_key_on_bank1() {
            let (r, c) = run(mode_chip(0x28, 0xf6)); // ch=2, bank=1 → channel=5
            assert_eq!(r.mode_kon_channel, c.mode_kon_channel);
        }

        #[test]
        fn mode_0x28_key_on_invalid() {
            let (r, c) = run(mode_chip(0x28, 0x03)); // ch bits=3 → invalid
            assert_eq!(r.mode_kon_channel, c.mode_kon_channel);
        }

        #[test]
        fn mode_0x2b_dac_enable() {
            let (r, c) = run(mode_chip(0x2b, 0x80));
            assert_eq!(r.dacen, c.dacen);
        }

        #[test]
        fn mode_0x2a_dac_data() {
            let (r, c) = run(mode_chip(0x2a, 0x55));
            assert_eq!(r.dacdata, c.dacdata);
        }
    }
}
