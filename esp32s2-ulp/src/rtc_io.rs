#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    out: OUT,
    out_w1ts: OUT_W1TS,
    out_w1tc: OUT_W1TC,
    enable: ENABLE,
    enable_w1ts: ENABLE_W1TS,
    enable_w1tc: ENABLE_W1TC,
    status: STATUS,
    status_w1ts: STATUS_W1TS,
    status_w1tc: STATUS_W1TC,
    in_: IN,
    pin: [PIN; 22],
    rtc_debug_sel: RTC_DEBUG_SEL,
    touch_pad: [TOUCH_PAD; 15],
    xtal_32p_pad: XTAL_32P_PAD,
    xtal_32n_pad: XTAL_32N_PAD,
    pad_dac1: PAD_DAC1,
    pad_dac2: PAD_DAC2,
    rtc_pad19: RTC_PAD19,
    rtc_pad20: RTC_PAD20,
    rtc_pad21: RTC_PAD21,
    ext_wakeup0: EXT_WAKEUP0,
    xtl_ext_ctr: XTL_EXT_CTR,
    sar_i2c_io: SAR_I2C_IO,
    rtc_io_touch_ctrl: RTC_IO_TOUCH_CTRL,
    _reserved24: [u8; 0x0110],
    rtc_io_date: RTC_IO_DATE,
}
impl RegisterBlock {
    ///0x00 - RTC GPIO output register
    #[inline(always)]
    pub const fn out(&self) -> &OUT {
        &self.out
    }
    ///0x04 - RTC GPIO output bit set register
    #[inline(always)]
    pub const fn out_w1ts(&self) -> &OUT_W1TS {
        &self.out_w1ts
    }
    ///0x08 - RTC GPIO output bit clear register
    #[inline(always)]
    pub const fn out_w1tc(&self) -> &OUT_W1TC {
        &self.out_w1tc
    }
    ///0x0c - RTC GPIO output enable register
    #[inline(always)]
    pub const fn enable(&self) -> &ENABLE {
        &self.enable
    }
    ///0x10 - RTC GPIO output enable bit set register
    #[inline(always)]
    pub const fn enable_w1ts(&self) -> &ENABLE_W1TS {
        &self.enable_w1ts
    }
    ///0x14 - RTC GPIO output enable bit clear register
    #[inline(always)]
    pub const fn enable_w1tc(&self) -> &ENABLE_W1TC {
        &self.enable_w1tc
    }
    ///0x18 - RTC GPIO interrupt status register
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    ///0x1c - RTC GPIO interrupt status bit set register
    #[inline(always)]
    pub const fn status_w1ts(&self) -> &STATUS_W1TS {
        &self.status_w1ts
    }
    ///0x20 - RTC GPIO interrupt status bit clear register
    #[inline(always)]
    pub const fn status_w1tc(&self) -> &STATUS_W1TC {
        &self.status_w1tc
    }
    ///0x24 - RTC GPIO input register
    #[inline(always)]
    pub const fn in_(&self) -> &IN {
        &self.in_
    }
    ///0x28..0x80 - RTC configuration for pin %s
    #[inline(always)]
    pub const fn pin(&self, n: usize) -> &PIN {
        &self.pin[n]
    }
    ///Iterator for array of:
    ///0x28..0x80 - RTC configuration for pin %s
    #[inline(always)]
    pub fn pin_iter(&self) -> impl Iterator<Item = &PIN> {
        self.pin.iter()
    }
    ///0x80 - RTC debug select register
    #[inline(always)]
    pub const fn rtc_debug_sel(&self) -> &RTC_DEBUG_SEL {
        &self.rtc_debug_sel
    }
    ///0x84..0xc0 - Touch pad %s configuration register
    #[inline(always)]
    pub const fn touch_pad(&self, n: usize) -> &TOUCH_PAD {
        &self.touch_pad[n]
    }
    ///Iterator for array of:
    ///0x84..0xc0 - Touch pad %s configuration register
    #[inline(always)]
    pub fn touch_pad_iter(&self) -> impl Iterator<Item = &TOUCH_PAD> {
        self.touch_pad.iter()
    }
    ///0xc0 - 32KHz crystal P-pad configuration register
    #[inline(always)]
    pub const fn xtal_32p_pad(&self) -> &XTAL_32P_PAD {
        &self.xtal_32p_pad
    }
    ///0xc4 - 32KHz crystal N-pad configuration register
    #[inline(always)]
    pub const fn xtal_32n_pad(&self) -> &XTAL_32N_PAD {
        &self.xtal_32n_pad
    }
    ///0xc8 - DAC1 configuration register
    #[inline(always)]
    pub const fn pad_dac1(&self) -> &PAD_DAC1 {
        &self.pad_dac1
    }
    ///0xcc - DAC2 configuration register
    #[inline(always)]
    pub const fn pad_dac2(&self) -> &PAD_DAC2 {
        &self.pad_dac2
    }
    ///0xd0 - Touch pad 19 configuration register
    #[inline(always)]
    pub const fn rtc_pad19(&self) -> &RTC_PAD19 {
        &self.rtc_pad19
    }
    ///0xd4 - Touch pad 20 configuration register
    #[inline(always)]
    pub const fn rtc_pad20(&self) -> &RTC_PAD20 {
        &self.rtc_pad20
    }
    ///0xd8 - Touch pad 21 configuration register
    #[inline(always)]
    pub const fn rtc_pad21(&self) -> &RTC_PAD21 {
        &self.rtc_pad21
    }
    ///0xdc - External wake up configuration register
    #[inline(always)]
    pub const fn ext_wakeup0(&self) -> &EXT_WAKEUP0 {
        &self.ext_wakeup0
    }
    ///0xe0 - Crystal power down enable GPIO source
    #[inline(always)]
    pub const fn xtl_ext_ctr(&self) -> &XTL_EXT_CTR {
        &self.xtl_ext_ctr
    }
    ///0xe4 - RTC I2C pad selection
    #[inline(always)]
    pub const fn sar_i2c_io(&self) -> &SAR_I2C_IO {
        &self.sar_i2c_io
    }
    ///0xe8 - Touch control register
    #[inline(always)]
    pub const fn rtc_io_touch_ctrl(&self) -> &RTC_IO_TOUCH_CTRL {
        &self.rtc_io_touch_ctrl
    }
    ///0x1fc - Version control register
    #[inline(always)]
    pub const fn rtc_io_date(&self) -> &RTC_IO_DATE {
        &self.rtc_io_date
    }
}
/**OUT (rw) register accessor: RTC GPIO output register

You can [`read`](crate::generic::Reg::read) this register and get [`out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out`] module*/
pub type OUT = crate::Reg<out::OUT_SPEC>;
///RTC GPIO output register
pub mod out;
/**OUT_W1TS (w) register accessor: RTC GPIO output bit set register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_w1ts`] module*/
pub type OUT_W1TS = crate::Reg<out_w1ts::OUT_W1TS_SPEC>;
///RTC GPIO output bit set register
pub mod out_w1ts;
/**OUT_W1TC (w) register accessor: RTC GPIO output bit clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_w1tc`] module*/
pub type OUT_W1TC = crate::Reg<out_w1tc::OUT_W1TC_SPEC>;
///RTC GPIO output bit clear register
pub mod out_w1tc;
/**ENABLE (rw) register accessor: RTC GPIO output enable register

You can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@enable`] module*/
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
///RTC GPIO output enable register
pub mod enable;
/**ENABLE_W1TS (w) register accessor: RTC GPIO output enable bit set register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@enable_w1ts`] module*/
pub type ENABLE_W1TS = crate::Reg<enable_w1ts::ENABLE_W1TS_SPEC>;
///RTC GPIO output enable bit set register
pub mod enable_w1ts;
/**ENABLE_W1TC (w) register accessor: RTC GPIO output enable bit clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@enable_w1tc`] module*/
pub type ENABLE_W1TC = crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>;
///RTC GPIO output enable bit clear register
pub mod enable_w1tc;
/**STATUS (rw) register accessor: RTC GPIO interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status`] module*/
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
///RTC GPIO interrupt status register
pub mod status;
/**STATUS_W1TS (w) register accessor: RTC GPIO interrupt status bit set register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_w1ts`] module*/
pub type STATUS_W1TS = crate::Reg<status_w1ts::STATUS_W1TS_SPEC>;
///RTC GPIO interrupt status bit set register
pub mod status_w1ts;
/**STATUS_W1TC (w) register accessor: RTC GPIO interrupt status bit clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_w1tc`] module*/
pub type STATUS_W1TC = crate::Reg<status_w1tc::STATUS_W1TC_SPEC>;
///RTC GPIO interrupt status bit clear register
pub mod status_w1tc;
/**IN (r) register accessor: RTC GPIO input register

You can [`read`](crate::generic::Reg::read) this register and get [`in_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@in_`] module*/
pub type IN = crate::Reg<in_::IN_SPEC>;
///RTC GPIO input register
pub mod in_;
/**PIN (rw) register accessor: RTC configuration for pin %s

You can [`read`](crate::generic::Reg::read) this register and get [`pin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin`] module*/
pub type PIN = crate::Reg<pin::PIN_SPEC>;
///RTC configuration for pin %s
pub mod pin;
/**RTC_DEBUG_SEL (rw) register accessor: RTC debug select register

You can [`read`](crate::generic::Reg::read) this register and get [`rtc_debug_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_debug_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtc_debug_sel`] module*/
pub type RTC_DEBUG_SEL = crate::Reg<rtc_debug_sel::RTC_DEBUG_SEL_SPEC>;
///RTC debug select register
pub mod rtc_debug_sel;
/**TOUCH_PAD (rw) register accessor: Touch pad %s configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`touch_pad::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@touch_pad`] module*/
pub type TOUCH_PAD = crate::Reg<touch_pad::TOUCH_PAD_SPEC>;
///Touch pad %s configuration register
pub mod touch_pad;
/**XTAL_32P_PAD (rw) register accessor: 32KHz crystal P-pad configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`xtal_32p_pad::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal_32p_pad::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@xtal_32p_pad`] module*/
pub type XTAL_32P_PAD = crate::Reg<xtal_32p_pad::XTAL_32P_PAD_SPEC>;
///32KHz crystal P-pad configuration register
pub mod xtal_32p_pad;
/**XTAL_32N_PAD (rw) register accessor: 32KHz crystal N-pad configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`xtal_32n_pad::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal_32n_pad::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@xtal_32n_pad`] module*/
pub type XTAL_32N_PAD = crate::Reg<xtal_32n_pad::XTAL_32N_PAD_SPEC>;
///32KHz crystal N-pad configuration register
pub mod xtal_32n_pad;
/**PAD_DAC1 (rw) register accessor: DAC1 configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pad_dac1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_dac1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pad_dac1`] module*/
pub type PAD_DAC1 = crate::Reg<pad_dac1::PAD_DAC1_SPEC>;
///DAC1 configuration register
pub mod pad_dac1;
/**PAD_DAC2 (rw) register accessor: DAC2 configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pad_dac2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_dac2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pad_dac2`] module*/
pub type PAD_DAC2 = crate::Reg<pad_dac2::PAD_DAC2_SPEC>;
///DAC2 configuration register
pub mod pad_dac2;
/**RTC_PAD19 (rw) register accessor: Touch pad 19 configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`rtc_pad19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_pad19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtc_pad19`] module*/
pub type RTC_PAD19 = crate::Reg<rtc_pad19::RTC_PAD19_SPEC>;
///Touch pad 19 configuration register
pub mod rtc_pad19;
/**RTC_PAD20 (rw) register accessor: Touch pad 20 configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`rtc_pad20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_pad20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtc_pad20`] module*/
pub type RTC_PAD20 = crate::Reg<rtc_pad20::RTC_PAD20_SPEC>;
///Touch pad 20 configuration register
pub mod rtc_pad20;
/**RTC_PAD21 (rw) register accessor: Touch pad 21 configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`rtc_pad21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_pad21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtc_pad21`] module*/
pub type RTC_PAD21 = crate::Reg<rtc_pad21::RTC_PAD21_SPEC>;
///Touch pad 21 configuration register
pub mod rtc_pad21;
/**EXT_WAKEUP0 (rw) register accessor: External wake up configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_wakeup0`] module*/
pub type EXT_WAKEUP0 = crate::Reg<ext_wakeup0::EXT_WAKEUP0_SPEC>;
///External wake up configuration register
pub mod ext_wakeup0;
/**XTL_EXT_CTR (rw) register accessor: Crystal power down enable GPIO source

You can [`read`](crate::generic::Reg::read) this register and get [`xtl_ext_ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtl_ext_ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@xtl_ext_ctr`] module*/
pub type XTL_EXT_CTR = crate::Reg<xtl_ext_ctr::XTL_EXT_CTR_SPEC>;
///Crystal power down enable GPIO source
pub mod xtl_ext_ctr;
/**SAR_I2C_IO (rw) register accessor: RTC I2C pad selection

You can [`read`](crate::generic::Reg::read) this register and get [`sar_i2c_io::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_i2c_io::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sar_i2c_io`] module*/
pub type SAR_I2C_IO = crate::Reg<sar_i2c_io::SAR_I2C_IO_SPEC>;
///RTC I2C pad selection
pub mod sar_i2c_io;
/**RTC_IO_TOUCH_CTRL (rw) register accessor: Touch control register

You can [`read`](crate::generic::Reg::read) this register and get [`rtc_io_touch_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_io_touch_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtc_io_touch_ctrl`] module*/
pub type RTC_IO_TOUCH_CTRL = crate::Reg<rtc_io_touch_ctrl::RTC_IO_TOUCH_CTRL_SPEC>;
///Touch control register
pub mod rtc_io_touch_ctrl;
/**RTC_IO_DATE (rw) register accessor: Version control register

You can [`read`](crate::generic::Reg::read) this register and get [`rtc_io_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_io_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtc_io_date`] module*/
pub type RTC_IO_DATE = crate::Reg<rtc_io_date::RTC_IO_DATE_SPEC>;
///Version control register
pub mod rtc_io_date;
