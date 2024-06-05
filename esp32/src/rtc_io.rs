#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
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
    pin: [PIN; 18],
    rtc_debug_sel: RTC_DEBUG_SEL,
    dig_pad_hold: DIG_PAD_HOLD,
    hall_sens: HALL_SENS,
    sensor_pads: SENSOR_PADS,
    adc_pad: ADC_PAD,
    pad_dac: [PAD_DAC; 2],
    xtal_32k_pad: XTAL_32K_PAD,
    touch_cfg: TOUCH_CFG,
    touch_pad0: TOUCH_PAD0,
    touch_pad1: TOUCH_PAD1,
    touch_pad2: TOUCH_PAD2,
    touch_pad3: TOUCH_PAD3,
    touch_pad4: TOUCH_PAD4,
    touch_pad5: TOUCH_PAD5,
    touch_pad6: TOUCH_PAD6,
    touch_pad7: TOUCH_PAD7,
    touch_pad8: TOUCH_PAD8,
    touch_pad9: TOUCH_PAD9,
    ext_wakeup0: EXT_WAKEUP0,
    xtl_ext_ctr: XTL_EXT_CTR,
    sar_i2c_io: SAR_I2C_IO,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn out(&self) -> &OUT {
        &self.out
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn out_w1ts(&self) -> &OUT_W1TS {
        &self.out_w1ts
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn out_w1tc(&self) -> &OUT_W1TC {
        &self.out_w1tc
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn enable(&self) -> &ENABLE {
        &self.enable
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn enable_w1ts(&self) -> &ENABLE_W1TS {
        &self.enable_w1ts
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn enable_w1tc(&self) -> &ENABLE_W1TC {
        &self.enable_w1tc
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn status_w1ts(&self) -> &STATUS_W1TS {
        &self.status_w1ts
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn status_w1tc(&self) -> &STATUS_W1TC {
        &self.status_w1tc
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn in_(&self) -> &IN {
        &self.in_
    }
    #[doc = "0x28..0x70 - "]
    #[inline(always)]
    pub const fn pin(&self, n: usize) -> &PIN {
        &self.pin[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x28..0x70 - "]
    #[inline(always)]
    pub fn pin_iter(&self) -> impl Iterator<Item = &PIN> {
        self.pin.iter()
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn rtc_debug_sel(&self) -> &RTC_DEBUG_SEL {
        &self.rtc_debug_sel
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn dig_pad_hold(&self) -> &DIG_PAD_HOLD {
        &self.dig_pad_hold
    }
    #[doc = "0x78 - "]
    #[inline(always)]
    pub const fn hall_sens(&self) -> &HALL_SENS {
        &self.hall_sens
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn sensor_pads(&self) -> &SENSOR_PADS {
        &self.sensor_pads
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn adc_pad(&self) -> &ADC_PAD {
        &self.adc_pad
    }
    #[doc = "0x84..0x8c - "]
    #[inline(always)]
    pub const fn pad_dac(&self, n: usize) -> &PAD_DAC {
        &self.pad_dac[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x84..0x8c - "]
    #[inline(always)]
    pub fn pad_dac_iter(&self) -> impl Iterator<Item = &PAD_DAC> {
        self.pad_dac.iter()
    }
    #[doc = "0x84 - PAD_DAC1"]
    #[inline(always)]
    pub const fn pad_dac1(&self) -> &PAD_DAC {
        self.pad_dac(0)
    }
    #[doc = "0x88 - PAD_DAC2"]
    #[inline(always)]
    pub const fn pad_dac2(&self) -> &PAD_DAC {
        self.pad_dac(1)
    }
    #[doc = "0x8c - "]
    #[inline(always)]
    pub const fn xtal_32k_pad(&self) -> &XTAL_32K_PAD {
        &self.xtal_32k_pad
    }
    #[doc = "0x90 - "]
    #[inline(always)]
    pub const fn touch_cfg(&self) -> &TOUCH_CFG {
        &self.touch_cfg
    }
    #[doc = "0x94 - "]
    #[inline(always)]
    pub const fn touch_pad0(&self) -> &TOUCH_PAD0 {
        &self.touch_pad0
    }
    #[doc = "0x98 - "]
    #[inline(always)]
    pub const fn touch_pad1(&self) -> &TOUCH_PAD1 {
        &self.touch_pad1
    }
    #[doc = "0x9c - "]
    #[inline(always)]
    pub const fn touch_pad2(&self) -> &TOUCH_PAD2 {
        &self.touch_pad2
    }
    #[doc = "0xa0 - "]
    #[inline(always)]
    pub const fn touch_pad3(&self) -> &TOUCH_PAD3 {
        &self.touch_pad3
    }
    #[doc = "0xa4 - "]
    #[inline(always)]
    pub const fn touch_pad4(&self) -> &TOUCH_PAD4 {
        &self.touch_pad4
    }
    #[doc = "0xa8 - "]
    #[inline(always)]
    pub const fn touch_pad5(&self) -> &TOUCH_PAD5 {
        &self.touch_pad5
    }
    #[doc = "0xac - "]
    #[inline(always)]
    pub const fn touch_pad6(&self) -> &TOUCH_PAD6 {
        &self.touch_pad6
    }
    #[doc = "0xb0 - "]
    #[inline(always)]
    pub const fn touch_pad7(&self) -> &TOUCH_PAD7 {
        &self.touch_pad7
    }
    #[doc = "0xb4 - "]
    #[inline(always)]
    pub const fn touch_pad8(&self) -> &TOUCH_PAD8 {
        &self.touch_pad8
    }
    #[doc = "0xb8 - "]
    #[inline(always)]
    pub const fn touch_pad9(&self) -> &TOUCH_PAD9 {
        &self.touch_pad9
    }
    #[doc = "0xbc - "]
    #[inline(always)]
    pub const fn ext_wakeup0(&self) -> &EXT_WAKEUP0 {
        &self.ext_wakeup0
    }
    #[doc = "0xc0 - "]
    #[inline(always)]
    pub const fn xtl_ext_ctr(&self) -> &XTL_EXT_CTR {
        &self.xtl_ext_ctr
    }
    #[doc = "0xc4 - "]
    #[inline(always)]
    pub const fn sar_i2c_io(&self) -> &SAR_I2C_IO {
        &self.sar_i2c_io
    }
    #[doc = "0xc8 - "]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "OUT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out`] module"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = ""]
pub mod out;
#[doc = "OUT_W1TS (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_w1ts`] module"]
pub type OUT_W1TS = crate::Reg<out_w1ts::OUT_W1TS_SPEC>;
#[doc = ""]
pub mod out_w1ts;
#[doc = "OUT_W1TC (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_w1tc`] module"]
pub type OUT_W1TC = crate::Reg<out_w1tc::OUT_W1TC_SPEC>;
#[doc = ""]
pub mod out_w1tc;
#[doc = "ENABLE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`] module"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = ""]
pub mod enable;
#[doc = "ENABLE_W1TS (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_w1ts`] module"]
pub type ENABLE_W1TS = crate::Reg<enable_w1ts::ENABLE_W1TS_SPEC>;
#[doc = ""]
pub mod enable_w1ts;
#[doc = "ENABLE_W1TC (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_w1tc`] module"]
pub type ENABLE_W1TC = crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>;
#[doc = ""]
pub mod enable_w1tc;
#[doc = "STATUS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = ""]
pub mod status;
#[doc = "STATUS_W1TS (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_w1ts`] module"]
pub type STATUS_W1TS = crate::Reg<status_w1ts::STATUS_W1TS_SPEC>;
#[doc = ""]
pub mod status_w1ts;
#[doc = "STATUS_W1TC (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_w1tc`] module"]
pub type STATUS_W1TC = crate::Reg<status_w1tc::STATUS_W1TC_SPEC>;
#[doc = ""]
pub mod status_w1tc;
#[doc = "IN (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_`] module"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = ""]
pub mod in_;
#[doc = "PIN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin`] module"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = ""]
pub mod pin;
#[doc = "RTC_DEBUG_SEL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_debug_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_debug_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_debug_sel`] module"]
pub type RTC_DEBUG_SEL = crate::Reg<rtc_debug_sel::RTC_DEBUG_SEL_SPEC>;
#[doc = ""]
pub mod rtc_debug_sel;
#[doc = "DIG_PAD_HOLD (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dig_pad_hold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dig_pad_hold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dig_pad_hold`] module"]
pub type DIG_PAD_HOLD = crate::Reg<dig_pad_hold::DIG_PAD_HOLD_SPEC>;
#[doc = ""]
pub mod dig_pad_hold;
#[doc = "HALL_SENS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hall_sens::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hall_sens::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hall_sens`] module"]
pub type HALL_SENS = crate::Reg<hall_sens::HALL_SENS_SPEC>;
#[doc = ""]
pub mod hall_sens;
#[doc = "SENSOR_PADS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sensor_pads::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sensor_pads::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sensor_pads`] module"]
pub type SENSOR_PADS = crate::Reg<sensor_pads::SENSOR_PADS_SPEC>;
#[doc = ""]
pub mod sensor_pads;
#[doc = "ADC_PAD (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_pad::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_pad::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_pad`] module"]
pub type ADC_PAD = crate::Reg<adc_pad::ADC_PAD_SPEC>;
#[doc = ""]
pub mod adc_pad;
#[doc = "PAD_DAC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_dac::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_dac::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_dac`] module"]
pub type PAD_DAC = crate::Reg<pad_dac::PAD_DAC_SPEC>;
#[doc = ""]
pub mod pad_dac;
#[doc = "XTAL_32K_PAD (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xtal_32k_pad::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal_32k_pad::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtal_32k_pad`] module"]
pub type XTAL_32K_PAD = crate::Reg<xtal_32k_pad::XTAL_32K_PAD_SPEC>;
#[doc = ""]
pub mod xtal_32k_pad;
#[doc = "TOUCH_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_cfg`] module"]
pub type TOUCH_CFG = crate::Reg<touch_cfg::TOUCH_CFG_SPEC>;
#[doc = ""]
pub mod touch_cfg;
#[doc = "TOUCH_PAD0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pad0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_pad0`] module"]
pub type TOUCH_PAD0 = crate::Reg<touch_pad0::TOUCH_PAD0_SPEC>;
#[doc = ""]
pub mod touch_pad0;
#[doc = "TOUCH_PAD1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pad1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_pad1`] module"]
pub type TOUCH_PAD1 = crate::Reg<touch_pad1::TOUCH_PAD1_SPEC>;
#[doc = ""]
pub mod touch_pad1;
#[doc = "TOUCH_PAD2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pad2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_pad2`] module"]
pub type TOUCH_PAD2 = crate::Reg<touch_pad2::TOUCH_PAD2_SPEC>;
#[doc = ""]
pub mod touch_pad2;
#[doc = "TOUCH_PAD3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pad3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_pad3`] module"]
pub type TOUCH_PAD3 = crate::Reg<touch_pad3::TOUCH_PAD3_SPEC>;
#[doc = ""]
pub mod touch_pad3;
#[doc = "TOUCH_PAD4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pad4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_pad4`] module"]
pub type TOUCH_PAD4 = crate::Reg<touch_pad4::TOUCH_PAD4_SPEC>;
#[doc = ""]
pub mod touch_pad4;
#[doc = "TOUCH_PAD5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pad5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_pad5`] module"]
pub type TOUCH_PAD5 = crate::Reg<touch_pad5::TOUCH_PAD5_SPEC>;
#[doc = ""]
pub mod touch_pad5;
#[doc = "TOUCH_PAD6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pad6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_pad6`] module"]
pub type TOUCH_PAD6 = crate::Reg<touch_pad6::TOUCH_PAD6_SPEC>;
#[doc = ""]
pub mod touch_pad6;
#[doc = "TOUCH_PAD7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pad7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_pad7`] module"]
pub type TOUCH_PAD7 = crate::Reg<touch_pad7::TOUCH_PAD7_SPEC>;
#[doc = ""]
pub mod touch_pad7;
#[doc = "TOUCH_PAD8 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pad8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_pad8`] module"]
pub type TOUCH_PAD8 = crate::Reg<touch_pad8::TOUCH_PAD8_SPEC>;
#[doc = ""]
pub mod touch_pad8;
#[doc = "TOUCH_PAD9 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pad9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_pad9`] module"]
pub type TOUCH_PAD9 = crate::Reg<touch_pad9::TOUCH_PAD9_SPEC>;
#[doc = ""]
pub mod touch_pad9;
#[doc = "EXT_WAKEUP0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_wakeup0`] module"]
pub type EXT_WAKEUP0 = crate::Reg<ext_wakeup0::EXT_WAKEUP0_SPEC>;
#[doc = ""]
pub mod ext_wakeup0;
#[doc = "XTL_EXT_CTR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xtl_ext_ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtl_ext_ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtl_ext_ctr`] module"]
pub type XTL_EXT_CTR = crate::Reg<xtl_ext_ctr::XTL_EXT_CTR_SPEC>;
#[doc = ""]
pub mod xtl_ext_ctr;
#[doc = "SAR_I2C_IO (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_i2c_io::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_i2c_io::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_i2c_io`] module"]
pub type SAR_I2C_IO = crate::Reg<sar_i2c_io::SAR_I2C_IO_SPEC>;
#[doc = ""]
pub mod sar_i2c_io;
#[doc = "DATE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
