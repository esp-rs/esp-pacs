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
    pin0: PIN0,
    pin1: PIN1,
    pin2: PIN2,
    pin3: PIN3,
    pin4: PIN4,
    pin5: PIN5,
    pin6: PIN6,
    pin7: PIN7,
    pin8: PIN8,
    pin9: PIN9,
    pin10: PIN10,
    pin11: PIN11,
    pin12: PIN12,
    pin13: PIN13,
    pin14: PIN14,
    pin15: PIN15,
    pin16: PIN16,
    pin17: PIN17,
    pin18: PIN18,
    pin19: PIN19,
    pin20: PIN20,
    pin21: PIN21,
    rtc_debug_sel: RTC_DEBUG_SEL,
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
    touch_pad10: TOUCH_PAD10,
    touch_pad11: TOUCH_PAD11,
    touch_pad12: TOUCH_PAD12,
    touch_pad13: TOUCH_PAD13,
    touch_pad14: TOUCH_PAD14,
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
    touch_ctrl: TOUCH_CTRL,
    _reserved59: [u8; 0x0110],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - RTC GPIO 0 ~ 21 output data register
    #[inline(always)]
    pub const fn out(&self) -> &OUT {
        &self.out
    }
    ///0x04 - one set RTC GPIO output data
    #[inline(always)]
    pub const fn out_w1ts(&self) -> &OUT_W1TS {
        &self.out_w1ts
    }
    ///0x08 - one clear RTC GPIO output data
    #[inline(always)]
    pub const fn out_w1tc(&self) -> &OUT_W1TC {
        &self.out_w1tc
    }
    ///0x0c - Configure RTC GPIO output enable
    #[inline(always)]
    pub const fn enable(&self) -> &ENABLE {
        &self.enable
    }
    ///0x10 - one set RTC GPIO output enable
    #[inline(always)]
    pub const fn enable_w1ts(&self) -> &ENABLE_W1TS {
        &self.enable_w1ts
    }
    ///0x14 - one clear RTC GPIO output enable
    #[inline(always)]
    pub const fn enable_w1tc(&self) -> &ENABLE_W1TC {
        &self.enable_w1tc
    }
    ///0x18 - RTC GPIO 0 ~ 21 interrupt status
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    ///0x1c - One set RTC GPIO 0 ~ 21 interrupt status
    #[inline(always)]
    pub const fn status_w1ts(&self) -> &STATUS_W1TS {
        &self.status_w1ts
    }
    ///0x20 - One clear RTC GPIO 0 ~ 21 interrupt status
    #[inline(always)]
    pub const fn status_w1tc(&self) -> &STATUS_W1TC {
        &self.status_w1tc
    }
    ///0x24 - RTC GPIO input data
    #[inline(always)]
    pub const fn in_(&self) -> &IN {
        &self.in_
    }
    ///0x28 - configure RTC GPIO0
    #[inline(always)]
    pub const fn pin0(&self) -> &PIN0 {
        &self.pin0
    }
    ///0x2c - configure RTC GPIO1
    #[inline(always)]
    pub const fn pin1(&self) -> &PIN1 {
        &self.pin1
    }
    ///0x30 - configure RTC GPIO2
    #[inline(always)]
    pub const fn pin2(&self) -> &PIN2 {
        &self.pin2
    }
    ///0x34 - configure RTC GPIO3
    #[inline(always)]
    pub const fn pin3(&self) -> &PIN3 {
        &self.pin3
    }
    ///0x38 - configure RTC GPIO4
    #[inline(always)]
    pub const fn pin4(&self) -> &PIN4 {
        &self.pin4
    }
    ///0x3c - configure RTC GPIO5
    #[inline(always)]
    pub const fn pin5(&self) -> &PIN5 {
        &self.pin5
    }
    ///0x40 - configure RTC GPIO6
    #[inline(always)]
    pub const fn pin6(&self) -> &PIN6 {
        &self.pin6
    }
    ///0x44 - configure RTC GPIO7
    #[inline(always)]
    pub const fn pin7(&self) -> &PIN7 {
        &self.pin7
    }
    ///0x48 - configure RTC GPIO8
    #[inline(always)]
    pub const fn pin8(&self) -> &PIN8 {
        &self.pin8
    }
    ///0x4c - configure RTC GPIO9
    #[inline(always)]
    pub const fn pin9(&self) -> &PIN9 {
        &self.pin9
    }
    ///0x50 - configure RTC GPIO10
    #[inline(always)]
    pub const fn pin10(&self) -> &PIN10 {
        &self.pin10
    }
    ///0x54 - configure RTC GPIO11
    #[inline(always)]
    pub const fn pin11(&self) -> &PIN11 {
        &self.pin11
    }
    ///0x58 - configure RTC GPIO12
    #[inline(always)]
    pub const fn pin12(&self) -> &PIN12 {
        &self.pin12
    }
    ///0x5c - configure RTC GPIO13
    #[inline(always)]
    pub const fn pin13(&self) -> &PIN13 {
        &self.pin13
    }
    ///0x60 - configure RTC GPIO14
    #[inline(always)]
    pub const fn pin14(&self) -> &PIN14 {
        &self.pin14
    }
    ///0x64 - configure RTC GPIO15
    #[inline(always)]
    pub const fn pin15(&self) -> &PIN15 {
        &self.pin15
    }
    ///0x68 - configure RTC GPIO16
    #[inline(always)]
    pub const fn pin16(&self) -> &PIN16 {
        &self.pin16
    }
    ///0x6c - configure RTC GPIO17
    #[inline(always)]
    pub const fn pin17(&self) -> &PIN17 {
        &self.pin17
    }
    ///0x70 - configure RTC GPIO18
    #[inline(always)]
    pub const fn pin18(&self) -> &PIN18 {
        &self.pin18
    }
    ///0x74 - configure RTC GPIO19
    #[inline(always)]
    pub const fn pin19(&self) -> &PIN19 {
        &self.pin19
    }
    ///0x78 - configure RTC GPIO20
    #[inline(always)]
    pub const fn pin20(&self) -> &PIN20 {
        &self.pin20
    }
    ///0x7c - configure RTC GPIO21
    #[inline(always)]
    pub const fn pin21(&self) -> &PIN21 {
        &self.pin21
    }
    ///0x80 - configure rtc debug
    #[inline(always)]
    pub const fn rtc_debug_sel(&self) -> &RTC_DEBUG_SEL {
        &self.rtc_debug_sel
    }
    ///0x84 - configure RTC PAD0
    #[inline(always)]
    pub const fn touch_pad0(&self) -> &TOUCH_PAD0 {
        &self.touch_pad0
    }
    ///0x88 - configure RTC PAD1
    #[inline(always)]
    pub const fn touch_pad1(&self) -> &TOUCH_PAD1 {
        &self.touch_pad1
    }
    ///0x8c - configure RTC PAD2
    #[inline(always)]
    pub const fn touch_pad2(&self) -> &TOUCH_PAD2 {
        &self.touch_pad2
    }
    ///0x90 - configure RTC PAD3
    #[inline(always)]
    pub const fn touch_pad3(&self) -> &TOUCH_PAD3 {
        &self.touch_pad3
    }
    ///0x94 - configure RTC PAD4
    #[inline(always)]
    pub const fn touch_pad4(&self) -> &TOUCH_PAD4 {
        &self.touch_pad4
    }
    ///0x98 - configure RTC PAD5
    #[inline(always)]
    pub const fn touch_pad5(&self) -> &TOUCH_PAD5 {
        &self.touch_pad5
    }
    ///0x9c - configure RTC PAD6
    #[inline(always)]
    pub const fn touch_pad6(&self) -> &TOUCH_PAD6 {
        &self.touch_pad6
    }
    ///0xa0 - configure RTC PAD7
    #[inline(always)]
    pub const fn touch_pad7(&self) -> &TOUCH_PAD7 {
        &self.touch_pad7
    }
    ///0xa4 - configure RTC PAD8
    #[inline(always)]
    pub const fn touch_pad8(&self) -> &TOUCH_PAD8 {
        &self.touch_pad8
    }
    ///0xa8 - configure RTC PAD9
    #[inline(always)]
    pub const fn touch_pad9(&self) -> &TOUCH_PAD9 {
        &self.touch_pad9
    }
    ///0xac - configure RTC PAD10
    #[inline(always)]
    pub const fn touch_pad10(&self) -> &TOUCH_PAD10 {
        &self.touch_pad10
    }
    ///0xb0 - configure RTC PAD11
    #[inline(always)]
    pub const fn touch_pad11(&self) -> &TOUCH_PAD11 {
        &self.touch_pad11
    }
    ///0xb4 - configure RTC PAD12
    #[inline(always)]
    pub const fn touch_pad12(&self) -> &TOUCH_PAD12 {
        &self.touch_pad12
    }
    ///0xb8 - configure RTC PAD13
    #[inline(always)]
    pub const fn touch_pad13(&self) -> &TOUCH_PAD13 {
        &self.touch_pad13
    }
    ///0xbc - configure RTC PAD14
    #[inline(always)]
    pub const fn touch_pad14(&self) -> &TOUCH_PAD14 {
        &self.touch_pad14
    }
    ///0xc0 - configure RTC PAD15
    #[inline(always)]
    pub const fn xtal_32p_pad(&self) -> &XTAL_32P_PAD {
        &self.xtal_32p_pad
    }
    ///0xc4 - configure RTC PAD16
    #[inline(always)]
    pub const fn xtal_32n_pad(&self) -> &XTAL_32N_PAD {
        &self.xtal_32n_pad
    }
    ///0xc8 - configure RTC PAD17
    #[inline(always)]
    pub const fn pad_dac1(&self) -> &PAD_DAC1 {
        &self.pad_dac1
    }
    ///0xcc - configure RTC PAD18
    #[inline(always)]
    pub const fn pad_dac2(&self) -> &PAD_DAC2 {
        &self.pad_dac2
    }
    ///0xd0 - configure RTC PAD19
    #[inline(always)]
    pub const fn rtc_pad19(&self) -> &RTC_PAD19 {
        &self.rtc_pad19
    }
    ///0xd4 - configure RTC PAD20
    #[inline(always)]
    pub const fn rtc_pad20(&self) -> &RTC_PAD20 {
        &self.rtc_pad20
    }
    ///0xd8 - configure RTC PAD21
    #[inline(always)]
    pub const fn rtc_pad21(&self) -> &RTC_PAD21 {
        &self.rtc_pad21
    }
    ///0xdc - configure EXT0 wakeup
    #[inline(always)]
    pub const fn ext_wakeup0(&self) -> &EXT_WAKEUP0 {
        &self.ext_wakeup0
    }
    ///0xe0 - configure gpio pd XTAL
    #[inline(always)]
    pub const fn xtl_ext_ctr(&self) -> &XTL_EXT_CTR {
        &self.xtl_ext_ctr
    }
    ///0xe4 - configure rtc i2c mux
    #[inline(always)]
    pub const fn sar_i2c_io(&self) -> &SAR_I2C_IO {
        &self.sar_i2c_io
    }
    ///0xe8 - configure touch pad bufmode
    #[inline(always)]
    pub const fn touch_ctrl(&self) -> &TOUCH_CTRL {
        &self.touch_ctrl
    }
    ///0x1fc - version
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**OUT (rw) register accessor: RTC GPIO 0 ~ 21 output data register

You can [`read`](crate::generic::Reg::read) this register and get [`out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out`] module*/
pub type OUT = crate::Reg<out::OUT_SPEC>;
///RTC GPIO 0 ~ 21 output data register
pub mod out;
/**OUT_W1TS (w) register accessor: one set RTC GPIO output data

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_w1ts`] module*/
pub type OUT_W1TS = crate::Reg<out_w1ts::OUT_W1TS_SPEC>;
///one set RTC GPIO output data
pub mod out_w1ts;
/**OUT_W1TC (w) register accessor: one clear RTC GPIO output data

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_w1tc`] module*/
pub type OUT_W1TC = crate::Reg<out_w1tc::OUT_W1TC_SPEC>;
///one clear RTC GPIO output data
pub mod out_w1tc;
/**ENABLE (rw) register accessor: Configure RTC GPIO output enable

You can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@enable`] module*/
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
///Configure RTC GPIO output enable
pub mod enable;
/**ENABLE_W1TS (w) register accessor: one set RTC GPIO output enable

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@enable_w1ts`] module*/
pub type ENABLE_W1TS = crate::Reg<enable_w1ts::ENABLE_W1TS_SPEC>;
///one set RTC GPIO output enable
pub mod enable_w1ts;
/**ENABLE_W1TC (w) register accessor: one clear RTC GPIO output enable

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@enable_w1tc`] module*/
pub type ENABLE_W1TC = crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>;
///one clear RTC GPIO output enable
pub mod enable_w1tc;
/**STATUS (rw) register accessor: RTC GPIO 0 ~ 21 interrupt status

You can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status`] module*/
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
///RTC GPIO 0 ~ 21 interrupt status
pub mod status;
/**STATUS_W1TS (w) register accessor: One set RTC GPIO 0 ~ 21 interrupt status

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_w1ts`] module*/
pub type STATUS_W1TS = crate::Reg<status_w1ts::STATUS_W1TS_SPEC>;
///One set RTC GPIO 0 ~ 21 interrupt status
pub mod status_w1ts;
/**STATUS_W1TC (w) register accessor: One clear RTC GPIO 0 ~ 21 interrupt status

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_w1tc`] module*/
pub type STATUS_W1TC = crate::Reg<status_w1tc::STATUS_W1TC_SPEC>;
///One clear RTC GPIO 0 ~ 21 interrupt status
pub mod status_w1tc;
/**IN (r) register accessor: RTC GPIO input data

You can [`read`](crate::generic::Reg::read) this register and get [`in_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@in_`] module*/
pub type IN = crate::Reg<in_::IN_SPEC>;
///RTC GPIO input data
pub mod in_;
/**PIN0 (rw) register accessor: configure RTC GPIO0

You can [`read`](crate::generic::Reg::read) this register and get [`pin0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin0`] module*/
pub type PIN0 = crate::Reg<pin0::PIN0_SPEC>;
///configure RTC GPIO0
pub mod pin0;
/**PIN1 (rw) register accessor: configure RTC GPIO1

You can [`read`](crate::generic::Reg::read) this register and get [`pin1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin1`] module*/
pub type PIN1 = crate::Reg<pin1::PIN1_SPEC>;
///configure RTC GPIO1
pub mod pin1;
/**PIN2 (rw) register accessor: configure RTC GPIO2

You can [`read`](crate::generic::Reg::read) this register and get [`pin2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin2`] module*/
pub type PIN2 = crate::Reg<pin2::PIN2_SPEC>;
///configure RTC GPIO2
pub mod pin2;
/**PIN3 (rw) register accessor: configure RTC GPIO3

You can [`read`](crate::generic::Reg::read) this register and get [`pin3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin3`] module*/
pub type PIN3 = crate::Reg<pin3::PIN3_SPEC>;
///configure RTC GPIO3
pub mod pin3;
/**PIN4 (rw) register accessor: configure RTC GPIO4

You can [`read`](crate::generic::Reg::read) this register and get [`pin4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin4`] module*/
pub type PIN4 = crate::Reg<pin4::PIN4_SPEC>;
///configure RTC GPIO4
pub mod pin4;
/**PIN5 (rw) register accessor: configure RTC GPIO5

You can [`read`](crate::generic::Reg::read) this register and get [`pin5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin5`] module*/
pub type PIN5 = crate::Reg<pin5::PIN5_SPEC>;
///configure RTC GPIO5
pub mod pin5;
/**PIN6 (rw) register accessor: configure RTC GPIO6

You can [`read`](crate::generic::Reg::read) this register and get [`pin6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin6`] module*/
pub type PIN6 = crate::Reg<pin6::PIN6_SPEC>;
///configure RTC GPIO6
pub mod pin6;
/**PIN7 (rw) register accessor: configure RTC GPIO7

You can [`read`](crate::generic::Reg::read) this register and get [`pin7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin7`] module*/
pub type PIN7 = crate::Reg<pin7::PIN7_SPEC>;
///configure RTC GPIO7
pub mod pin7;
/**PIN8 (rw) register accessor: configure RTC GPIO8

You can [`read`](crate::generic::Reg::read) this register and get [`pin8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin8`] module*/
pub type PIN8 = crate::Reg<pin8::PIN8_SPEC>;
///configure RTC GPIO8
pub mod pin8;
/**PIN9 (rw) register accessor: configure RTC GPIO9

You can [`read`](crate::generic::Reg::read) this register and get [`pin9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin9`] module*/
pub type PIN9 = crate::Reg<pin9::PIN9_SPEC>;
///configure RTC GPIO9
pub mod pin9;
/**PIN10 (rw) register accessor: configure RTC GPIO10

You can [`read`](crate::generic::Reg::read) this register and get [`pin10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin10`] module*/
pub type PIN10 = crate::Reg<pin10::PIN10_SPEC>;
///configure RTC GPIO10
pub mod pin10;
/**PIN11 (rw) register accessor: configure RTC GPIO11

You can [`read`](crate::generic::Reg::read) this register and get [`pin11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin11`] module*/
pub type PIN11 = crate::Reg<pin11::PIN11_SPEC>;
///configure RTC GPIO11
pub mod pin11;
/**PIN12 (rw) register accessor: configure RTC GPIO12

You can [`read`](crate::generic::Reg::read) this register and get [`pin12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin12`] module*/
pub type PIN12 = crate::Reg<pin12::PIN12_SPEC>;
///configure RTC GPIO12
pub mod pin12;
/**PIN13 (rw) register accessor: configure RTC GPIO13

You can [`read`](crate::generic::Reg::read) this register and get [`pin13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin13`] module*/
pub type PIN13 = crate::Reg<pin13::PIN13_SPEC>;
///configure RTC GPIO13
pub mod pin13;
/**PIN14 (rw) register accessor: configure RTC GPIO14

You can [`read`](crate::generic::Reg::read) this register and get [`pin14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin14`] module*/
pub type PIN14 = crate::Reg<pin14::PIN14_SPEC>;
///configure RTC GPIO14
pub mod pin14;
/**PIN15 (rw) register accessor: configure RTC GPIO15

You can [`read`](crate::generic::Reg::read) this register and get [`pin15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin15`] module*/
pub type PIN15 = crate::Reg<pin15::PIN15_SPEC>;
///configure RTC GPIO15
pub mod pin15;
/**PIN16 (rw) register accessor: configure RTC GPIO16

You can [`read`](crate::generic::Reg::read) this register and get [`pin16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin16`] module*/
pub type PIN16 = crate::Reg<pin16::PIN16_SPEC>;
///configure RTC GPIO16
pub mod pin16;
/**PIN17 (rw) register accessor: configure RTC GPIO17

You can [`read`](crate::generic::Reg::read) this register and get [`pin17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin17`] module*/
pub type PIN17 = crate::Reg<pin17::PIN17_SPEC>;
///configure RTC GPIO17
pub mod pin17;
/**PIN18 (rw) register accessor: configure RTC GPIO18

You can [`read`](crate::generic::Reg::read) this register and get [`pin18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin18`] module*/
pub type PIN18 = crate::Reg<pin18::PIN18_SPEC>;
///configure RTC GPIO18
pub mod pin18;
/**PIN19 (rw) register accessor: configure RTC GPIO19

You can [`read`](crate::generic::Reg::read) this register and get [`pin19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin19`] module*/
pub type PIN19 = crate::Reg<pin19::PIN19_SPEC>;
///configure RTC GPIO19
pub mod pin19;
/**PIN20 (rw) register accessor: configure RTC GPIO20

You can [`read`](crate::generic::Reg::read) this register and get [`pin20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin20`] module*/
pub type PIN20 = crate::Reg<pin20::PIN20_SPEC>;
///configure RTC GPIO20
pub mod pin20;
/**PIN21 (rw) register accessor: configure RTC GPIO21

You can [`read`](crate::generic::Reg::read) this register and get [`pin21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin21`] module*/
pub type PIN21 = crate::Reg<pin21::PIN21_SPEC>;
///configure RTC GPIO21
pub mod pin21;
/**RTC_DEBUG_SEL (rw) register accessor: configure rtc debug

You can [`read`](crate::generic::Reg::read) this register and get [`rtc_debug_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_debug_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtc_debug_sel`] module*/
pub type RTC_DEBUG_SEL = crate::Reg<rtc_debug_sel::RTC_DEBUG_SEL_SPEC>;
///configure rtc debug
pub mod rtc_debug_sel;
/**TOUCH_PAD0 (rw) register accessor: configure RTC PAD0

You can [`read`](crate::generic::Reg::read) this register and get [`touch_pad0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@touch_pad0`] module*/
pub type TOUCH_PAD0 = crate::Reg<touch_pad0::TOUCH_PAD0_SPEC>;
///configure RTC PAD0
pub mod touch_pad0;
/**TOUCH_PAD1 (rw) register accessor: configure RTC PAD1

You can [`read`](crate::generic::Reg::read) this register and get [`touch_pad1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@touch_pad1`] module*/
pub type TOUCH_PAD1 = crate::Reg<touch_pad1::TOUCH_PAD1_SPEC>;
///configure RTC PAD1
pub mod touch_pad1;
/**TOUCH_PAD2 (rw) register accessor: configure RTC PAD2

You can [`read`](crate::generic::Reg::read) this register and get [`touch_pad2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@touch_pad2`] module*/
pub type TOUCH_PAD2 = crate::Reg<touch_pad2::TOUCH_PAD2_SPEC>;
///configure RTC PAD2
pub mod touch_pad2;
/**TOUCH_PAD3 (rw) register accessor: configure RTC PAD3

You can [`read`](crate::generic::Reg::read) this register and get [`touch_pad3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@touch_pad3`] module*/
pub type TOUCH_PAD3 = crate::Reg<touch_pad3::TOUCH_PAD3_SPEC>;
///configure RTC PAD3
pub mod touch_pad3;
/**TOUCH_PAD4 (rw) register accessor: configure RTC PAD4

You can [`read`](crate::generic::Reg::read) this register and get [`touch_pad4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@touch_pad4`] module*/
pub type TOUCH_PAD4 = crate::Reg<touch_pad4::TOUCH_PAD4_SPEC>;
///configure RTC PAD4
pub mod touch_pad4;
/**TOUCH_PAD5 (rw) register accessor: configure RTC PAD5

You can [`read`](crate::generic::Reg::read) this register and get [`touch_pad5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@touch_pad5`] module*/
pub type TOUCH_PAD5 = crate::Reg<touch_pad5::TOUCH_PAD5_SPEC>;
///configure RTC PAD5
pub mod touch_pad5;
/**TOUCH_PAD6 (rw) register accessor: configure RTC PAD6

You can [`read`](crate::generic::Reg::read) this register and get [`touch_pad6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@touch_pad6`] module*/
pub type TOUCH_PAD6 = crate::Reg<touch_pad6::TOUCH_PAD6_SPEC>;
///configure RTC PAD6
pub mod touch_pad6;
/**TOUCH_PAD7 (rw) register accessor: configure RTC PAD7

You can [`read`](crate::generic::Reg::read) this register and get [`touch_pad7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@touch_pad7`] module*/
pub type TOUCH_PAD7 = crate::Reg<touch_pad7::TOUCH_PAD7_SPEC>;
///configure RTC PAD7
pub mod touch_pad7;
/**TOUCH_PAD8 (rw) register accessor: configure RTC PAD8

You can [`read`](crate::generic::Reg::read) this register and get [`touch_pad8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@touch_pad8`] module*/
pub type TOUCH_PAD8 = crate::Reg<touch_pad8::TOUCH_PAD8_SPEC>;
///configure RTC PAD8
pub mod touch_pad8;
/**TOUCH_PAD9 (rw) register accessor: configure RTC PAD9

You can [`read`](crate::generic::Reg::read) this register and get [`touch_pad9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@touch_pad9`] module*/
pub type TOUCH_PAD9 = crate::Reg<touch_pad9::TOUCH_PAD9_SPEC>;
///configure RTC PAD9
pub mod touch_pad9;
/**TOUCH_PAD10 (rw) register accessor: configure RTC PAD10

You can [`read`](crate::generic::Reg::read) this register and get [`touch_pad10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@touch_pad10`] module*/
pub type TOUCH_PAD10 = crate::Reg<touch_pad10::TOUCH_PAD10_SPEC>;
///configure RTC PAD10
pub mod touch_pad10;
/**TOUCH_PAD11 (rw) register accessor: configure RTC PAD11

You can [`read`](crate::generic::Reg::read) this register and get [`touch_pad11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@touch_pad11`] module*/
pub type TOUCH_PAD11 = crate::Reg<touch_pad11::TOUCH_PAD11_SPEC>;
///configure RTC PAD11
pub mod touch_pad11;
/**TOUCH_PAD12 (rw) register accessor: configure RTC PAD12

You can [`read`](crate::generic::Reg::read) this register and get [`touch_pad12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@touch_pad12`] module*/
pub type TOUCH_PAD12 = crate::Reg<touch_pad12::TOUCH_PAD12_SPEC>;
///configure RTC PAD12
pub mod touch_pad12;
/**TOUCH_PAD13 (rw) register accessor: configure RTC PAD13

You can [`read`](crate::generic::Reg::read) this register and get [`touch_pad13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@touch_pad13`] module*/
pub type TOUCH_PAD13 = crate::Reg<touch_pad13::TOUCH_PAD13_SPEC>;
///configure RTC PAD13
pub mod touch_pad13;
/**TOUCH_PAD14 (rw) register accessor: configure RTC PAD14

You can [`read`](crate::generic::Reg::read) this register and get [`touch_pad14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@touch_pad14`] module*/
pub type TOUCH_PAD14 = crate::Reg<touch_pad14::TOUCH_PAD14_SPEC>;
///configure RTC PAD14
pub mod touch_pad14;
/**XTAL_32P_PAD (rw) register accessor: configure RTC PAD15

You can [`read`](crate::generic::Reg::read) this register and get [`xtal_32p_pad::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal_32p_pad::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@xtal_32p_pad`] module*/
pub type XTAL_32P_PAD = crate::Reg<xtal_32p_pad::XTAL_32P_PAD_SPEC>;
///configure RTC PAD15
pub mod xtal_32p_pad;
/**XTAL_32N_PAD (rw) register accessor: configure RTC PAD16

You can [`read`](crate::generic::Reg::read) this register and get [`xtal_32n_pad::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal_32n_pad::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@xtal_32n_pad`] module*/
pub type XTAL_32N_PAD = crate::Reg<xtal_32n_pad::XTAL_32N_PAD_SPEC>;
///configure RTC PAD16
pub mod xtal_32n_pad;
/**PAD_DAC1 (rw) register accessor: configure RTC PAD17

You can [`read`](crate::generic::Reg::read) this register and get [`pad_dac1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_dac1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pad_dac1`] module*/
pub type PAD_DAC1 = crate::Reg<pad_dac1::PAD_DAC1_SPEC>;
///configure RTC PAD17
pub mod pad_dac1;
/**PAD_DAC2 (rw) register accessor: configure RTC PAD18

You can [`read`](crate::generic::Reg::read) this register and get [`pad_dac2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_dac2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pad_dac2`] module*/
pub type PAD_DAC2 = crate::Reg<pad_dac2::PAD_DAC2_SPEC>;
///configure RTC PAD18
pub mod pad_dac2;
/**RTC_PAD19 (rw) register accessor: configure RTC PAD19

You can [`read`](crate::generic::Reg::read) this register and get [`rtc_pad19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_pad19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtc_pad19`] module*/
pub type RTC_PAD19 = crate::Reg<rtc_pad19::RTC_PAD19_SPEC>;
///configure RTC PAD19
pub mod rtc_pad19;
/**RTC_PAD20 (rw) register accessor: configure RTC PAD20

You can [`read`](crate::generic::Reg::read) this register and get [`rtc_pad20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_pad20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtc_pad20`] module*/
pub type RTC_PAD20 = crate::Reg<rtc_pad20::RTC_PAD20_SPEC>;
///configure RTC PAD20
pub mod rtc_pad20;
/**RTC_PAD21 (rw) register accessor: configure RTC PAD21

You can [`read`](crate::generic::Reg::read) this register and get [`rtc_pad21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_pad21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtc_pad21`] module*/
pub type RTC_PAD21 = crate::Reg<rtc_pad21::RTC_PAD21_SPEC>;
///configure RTC PAD21
pub mod rtc_pad21;
/**EXT_WAKEUP0 (rw) register accessor: configure EXT0 wakeup

You can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_wakeup0`] module*/
pub type EXT_WAKEUP0 = crate::Reg<ext_wakeup0::EXT_WAKEUP0_SPEC>;
///configure EXT0 wakeup
pub mod ext_wakeup0;
/**XTL_EXT_CTR (rw) register accessor: configure gpio pd XTAL

You can [`read`](crate::generic::Reg::read) this register and get [`xtl_ext_ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtl_ext_ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@xtl_ext_ctr`] module*/
pub type XTL_EXT_CTR = crate::Reg<xtl_ext_ctr::XTL_EXT_CTR_SPEC>;
///configure gpio pd XTAL
pub mod xtl_ext_ctr;
/**SAR_I2C_IO (rw) register accessor: configure rtc i2c mux

You can [`read`](crate::generic::Reg::read) this register and get [`sar_i2c_io::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_i2c_io::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sar_i2c_io`] module*/
pub type SAR_I2C_IO = crate::Reg<sar_i2c_io::SAR_I2C_IO_SPEC>;
///configure rtc i2c mux
pub mod sar_i2c_io;
/**TOUCH_CTRL (rw) register accessor: configure touch pad bufmode

You can [`read`](crate::generic::Reg::read) this register and get [`touch_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@touch_ctrl`] module*/
pub type TOUCH_CTRL = crate::Reg<touch_ctrl::TOUCH_CTRL_SPEC>;
///configure touch pad bufmode
pub mod touch_ctrl;
/**DATE (rw) register accessor: version

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///version
pub mod date;
