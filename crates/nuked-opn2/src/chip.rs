/// YM3438 (OPN2) emulator chip state.
///
/// This struct is compatible with the original C ym3438_t structure
/// using `#[repr(C)]`.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Ym3438 {
    pub cycles: u32,
    pub channel: u32,
    pub mol: i16,
    pub mor: i16,

    /* IO */
    pub write_data: u16,
    pub write_a: u8,
    pub write_d: u8,
    pub write_a_en: u8,
    pub write_d_en: u8,
    pub write_busy: u8,
    pub write_busy_cnt: u8,
    pub write_fm_address: u8,
    pub write_fm_data: u8,
    pub write_fm_mode_a: u16,
    pub address: u16,
    pub data: u8,
    pub pin_test_in: u8,
    pub pin_irq: u8,
    pub busy: u8,

    /* LFO */
    pub lfo_en: u8,
    pub lfo_freq: u8,
    pub lfo_pm: u8,
    pub lfo_am: u8,
    pub lfo_cnt: u8,
    pub lfo_inc: u8,
    pub lfo_quotient: u8,

    /* Phase generator */
    pub pg_fnum: u16,
    pub pg_block: u8,
    pub pg_kcode: u8,
    pub pg_inc: [u32; 24],
    pub pg_phase: [u32; 24],
    pub pg_reset: [u8; 24],
    pub pg_read: u32,

    /* Envelope generator */
    pub eg_cycle: u8,
    pub eg_cycle_stop: u8,
    pub eg_shift: u8,
    pub eg_shift_lock: u8,
    pub eg_timer_low_lock: u8,
    pub eg_timer: u16,
    pub eg_timer_inc: u8,
    pub eg_quotient: u16,
    pub eg_custom_timer: u8,
    pub eg_rate: u8,
    pub eg_ksv: u8,
    pub eg_inc: u8,
    pub eg_ratemax: u8,
    pub eg_sl: [u8; 2],
    pub eg_lfo_am: u8,
    pub eg_tl: [u8; 2],
    pub eg_state: [u8; 24],
    pub eg_level: [u16; 24],
    pub eg_out: [u16; 24],
    pub eg_kon: [u8; 24],
    pub eg_kon_csm: [u8; 24],
    pub eg_kon_latch: [u8; 24],
    pub eg_csm_mode: [u8; 24],
    pub eg_ssg_enable: [u8; 24],
    pub eg_ssg_pgrst_latch: [u8; 24],
    pub eg_ssg_repeat_latch: [u8; 24],
    pub eg_ssg_hold_up_latch: [u8; 24],
    pub eg_ssg_dir: [u8; 24],
    pub eg_ssg_inv: [u8; 24],
    pub eg_read: [u32; 2],
    pub eg_read_inc: u8,

    /* FM */
    pub fm_op1: [[i16; 2]; 6],
    pub fm_op2: [i16; 6],
    pub fm_out: [i16; 24],
    pub fm_mod: [u16; 24],

    /* Channel */
    pub ch_acc: [i16; 6],
    pub ch_out: [i16; 6],
    pub ch_lock: i16,
    pub ch_lock_l: u8,
    pub ch_lock_r: u8,
    pub ch_read: i16,

    /* Timer */
    pub timer_a_cnt: u16,
    pub timer_a_reg: u16,
    pub timer_a_load_lock: u8,
    pub timer_a_load: u8,
    pub timer_a_enable: u8,
    pub timer_a_reset: u8,
    pub timer_a_load_latch: u8,
    pub timer_a_overflow_flag: u8,
    pub timer_a_overflow: u8,

    pub timer_b_cnt: u16,
    pub timer_b_subcnt: u8,
    pub timer_b_reg: u16,
    pub timer_b_load_lock: u8,
    pub timer_b_load: u8,
    pub timer_b_enable: u8,
    pub timer_b_reset: u8,
    pub timer_b_load_latch: u8,
    pub timer_b_overflow_flag: u8,
    pub timer_b_overflow: u8,

    /* Register set */
    pub mode_test_21: [u8; 8],
    pub mode_test_2c: [u8; 8],
    pub mode_ch3: u8,
    pub mode_kon_channel: u8,
    pub mode_kon_operator: [u8; 4],
    pub mode_kon: [u8; 24],
    pub mode_csm: u8,
    pub mode_kon_csm: u8,
    pub dacen: u8,
    pub dacdata: i16,

    pub ks: [u8; 24],
    pub ar: [u8; 24],
    pub sr: [u8; 24],
    pub dt: [u8; 24],
    pub multi: [u8; 24],
    pub sl: [u8; 24],
    pub rr: [u8; 24],
    pub dr: [u8; 24],
    pub am: [u8; 24],
    pub tl: [u8; 24],
    pub ssg_eg: [u8; 24],

    pub fnum: [u16; 6],
    pub block: [u8; 6],
    pub kcode: [u8; 6],
    pub fnum_3ch: [u16; 6],
    pub block_3ch: [u8; 6],
    pub kcode_3ch: [u8; 6],
    pub reg_a4: u8,
    pub reg_ac: u8,
    pub connect: [u8; 6],
    pub fb: [u8; 6],
    pub pan_l: [u8; 6],
    pub pan_r: [u8; 6],
    pub ams: [u8; 6],
    pub pms: [u8; 6],
    pub status: u8,
    pub status_time: u32,
}

impl Default for Ym3438 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
