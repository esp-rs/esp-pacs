#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC GPIO 0 ~ 21 output data register"]
    pub rtc_gpio_out: RTC_GPIO_OUT,
    #[doc = "0x04 - one set RTC GPIO output data"]
    pub rtc_gpio_out_w1ts: RTC_GPIO_OUT_W1TS,
    #[doc = "0x08 - one clear RTC GPIO output data"]
    pub rtc_gpio_out_w1tc: RTC_GPIO_OUT_W1TC,
    #[doc = "0x0c - Configure RTC GPIO output enable"]
    pub rtc_gpio_enable: RTC_GPIO_ENABLE,
    #[doc = "0x10 - one set RTC GPIO output enable"]
    pub rtc_gpio_enable_w1ts: RTC_GPIO_ENABLE_W1TS,
    #[doc = "0x14 - one clear RTC GPIO output enable"]
    pub enable_w1tc: ENABLE_W1TC,
    #[doc = "0x18 - RTC GPIO 0 ~ 21 interrupt status"]
    pub rtc_gpio_status: RTC_GPIO_STATUS,
    #[doc = "0x1c - One set RTC GPIO 0 ~ 21 interrupt status"]
    pub rtc_gpio_status_w1ts: RTC_GPIO_STATUS_W1TS,
    #[doc = "0x20 - One clear RTC GPIO 0 ~ 21 interrupt status"]
    pub rtc_gpio_status_w1tc: RTC_GPIO_STATUS_W1TC,
    #[doc = "0x24 - RTC GPIO input data"]
    pub rtc_gpio_in: RTC_GPIO_IN,
    #[doc = "0x28..0x80 - configure RTC GPIO%s"]
    pub pin: [PIN; 22],
    #[doc = "0x80 - configure rtc debug"]
    pub rtc_debug_sel: RTC_DEBUG_SEL,
    #[doc = "0x84 - configure RTC PAD0"]
    pub touch_pad0: TOUCH_PAD0,
    #[doc = "0x88 - configure RTC PAD1"]
    pub touch_pad1: TOUCH_PAD1,
    #[doc = "0x8c - configure RTC PAD2"]
    pub touch_pad2: TOUCH_PAD2,
    #[doc = "0x90 - configure RTC PAD3"]
    pub touch_pad3: TOUCH_PAD3,
    #[doc = "0x94 - configure RTC PAD4"]
    pub touch_pad4: TOUCH_PAD4,
    #[doc = "0x98 - configure RTC PAD5"]
    pub touch_pad5: TOUCH_PAD5,
    #[doc = "0x9c - configure RTC PAD6"]
    pub touch_pad6: TOUCH_PAD6,
    #[doc = "0xa0 - configure RTC PAD7"]
    pub touch_pad7: TOUCH_PAD7,
    #[doc = "0xa4 - configure RTC PAD8"]
    pub touch_pad8: TOUCH_PAD8,
    #[doc = "0xa8 - configure RTC PAD9"]
    pub touch_pad9: TOUCH_PAD9,
    #[doc = "0xac - configure RTC PAD10"]
    pub touch_pad10: TOUCH_PAD10,
    #[doc = "0xb0 - configure RTC PAD11"]
    pub touch_pad11: TOUCH_PAD11,
    #[doc = "0xb4 - configure RTC PAD12"]
    pub touch_pad12: TOUCH_PAD12,
    #[doc = "0xb8 - configure RTC PAD13"]
    pub touch_pad13: TOUCH_PAD13,
    #[doc = "0xbc - configure RTC PAD14"]
    pub touch_pad14: TOUCH_PAD14,
    #[doc = "0xc0 - configure RTC PAD15"]
    pub xtal_32p_pad: XTAL_32P_PAD,
    #[doc = "0xc4 - configure RTC PAD16"]
    pub xtal_32n_pad: XTAL_32N_PAD,
    #[doc = "0xc8 - configure RTC PAD17"]
    pub pad_dac1: PAD_DAC1,
    #[doc = "0xcc - configure RTC PAD18"]
    pub pad_dac2: PAD_DAC2,
    #[doc = "0xd0 - configure RTC PAD19"]
    pub rtc_pad19: RTC_PAD19,
    #[doc = "0xd4 - configure RTC PAD20"]
    pub rtc_pad20: RTC_PAD20,
    #[doc = "0xd8 - configure RTC PAD21"]
    pub rtc_pad21: RTC_PAD21,
    #[doc = "0xdc - configure EXT0 wakeup"]
    pub ext_wakeup0: EXT_WAKEUP0,
    #[doc = "0xe0 - configure gpio pd XTAL"]
    pub xtl_ext_ctr: XTL_EXT_CTR,
    #[doc = "0xe4 - configure rtc i2c mux"]
    pub sar_i2c_io: SAR_I2C_IO,
    #[doc = "0xe8 - configure touch pad bufmode"]
    pub touch_ctrl: TOUCH_CTRL,
    _reserved38: [u8; 0x0110],
    #[doc = "0x1fc - version"]
    pub date: DATE,
}
#[doc = "RTC_GPIO_OUT (rw) register accessor: RTC GPIO 0 ~ 21 output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_gpio_out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_gpio_out`] module"]
pub type RTC_GPIO_OUT = crate::Reg<rtc_gpio_out::RTC_GPIO_OUT_SPEC>;
#[doc = "RTC GPIO 0 ~ 21 output data register"]
pub mod rtc_gpio_out;
#[doc = "RTC_GPIO_OUT_W1TS (w) register accessor: one set RTC GPIO output data\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_out_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_gpio_out_w1ts`] module"]
pub type RTC_GPIO_OUT_W1TS = crate::Reg<rtc_gpio_out_w1ts::RTC_GPIO_OUT_W1TS_SPEC>;
#[doc = "one set RTC GPIO output data"]
pub mod rtc_gpio_out_w1ts;
#[doc = "RTC_GPIO_OUT_W1TC (w) register accessor: one clear RTC GPIO output data\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_out_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_gpio_out_w1tc`] module"]
pub type RTC_GPIO_OUT_W1TC = crate::Reg<rtc_gpio_out_w1tc::RTC_GPIO_OUT_W1TC_SPEC>;
#[doc = "one clear RTC GPIO output data"]
pub mod rtc_gpio_out_w1tc;
#[doc = "RTC_GPIO_ENABLE (rw) register accessor: Configure RTC GPIO output enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_gpio_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_gpio_enable`] module"]
pub type RTC_GPIO_ENABLE = crate::Reg<rtc_gpio_enable::RTC_GPIO_ENABLE_SPEC>;
#[doc = "Configure RTC GPIO output enable"]
pub mod rtc_gpio_enable;
#[doc = "RTC_GPIO_ENABLE_W1TS (w) register accessor: one set RTC GPIO output enable\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_enable_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_gpio_enable_w1ts`] module"]
pub type RTC_GPIO_ENABLE_W1TS = crate::Reg<rtc_gpio_enable_w1ts::RTC_GPIO_ENABLE_W1TS_SPEC>;
#[doc = "one set RTC GPIO output enable"]
pub mod rtc_gpio_enable_w1ts;
#[doc = "ENABLE_W1TC (w) register accessor: one clear RTC GPIO output enable\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_w1tc`] module"]
pub type ENABLE_W1TC = crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>;
#[doc = "one clear RTC GPIO output enable"]
pub mod enable_w1tc;
#[doc = "RTC_GPIO_STATUS (rw) register accessor: RTC GPIO 0 ~ 21 interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_gpio_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_gpio_status`] module"]
pub type RTC_GPIO_STATUS = crate::Reg<rtc_gpio_status::RTC_GPIO_STATUS_SPEC>;
#[doc = "RTC GPIO 0 ~ 21 interrupt status"]
pub mod rtc_gpio_status;
#[doc = "RTC_GPIO_STATUS_W1TS (w) register accessor: One set RTC GPIO 0 ~ 21 interrupt status\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_status_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_gpio_status_w1ts`] module"]
pub type RTC_GPIO_STATUS_W1TS = crate::Reg<rtc_gpio_status_w1ts::RTC_GPIO_STATUS_W1TS_SPEC>;
#[doc = "One set RTC GPIO 0 ~ 21 interrupt status"]
pub mod rtc_gpio_status_w1ts;
#[doc = "RTC_GPIO_STATUS_W1TC (w) register accessor: One clear RTC GPIO 0 ~ 21 interrupt status\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_status_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_gpio_status_w1tc`] module"]
pub type RTC_GPIO_STATUS_W1TC = crate::Reg<rtc_gpio_status_w1tc::RTC_GPIO_STATUS_W1TC_SPEC>;
#[doc = "One clear RTC GPIO 0 ~ 21 interrupt status"]
pub mod rtc_gpio_status_w1tc;
#[doc = "RTC_GPIO_IN (r) register accessor: RTC GPIO input data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_gpio_in::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_gpio_in`] module"]
pub type RTC_GPIO_IN = crate::Reg<rtc_gpio_in::RTC_GPIO_IN_SPEC>;
#[doc = "RTC GPIO input data"]
pub mod rtc_gpio_in;
#[doc = "PIN (rw) register accessor: configure RTC GPIO%s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pin`] module"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = "configure RTC GPIO%s"]
pub mod pin;
#[doc = "RTC_DEBUG_SEL (rw) register accessor: configure rtc debug\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_debug_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_debug_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_debug_sel`] module"]
pub type RTC_DEBUG_SEL = crate::Reg<rtc_debug_sel::RTC_DEBUG_SEL_SPEC>;
#[doc = "configure rtc debug"]
pub mod rtc_debug_sel;
#[doc = "TOUCH_PAD0 (rw) register accessor: configure RTC PAD0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pad0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_pad0`] module"]
pub type TOUCH_PAD0 = crate::Reg<touch_pad0::TOUCH_PAD0_SPEC>;
#[doc = "configure RTC PAD0"]
pub mod touch_pad0;
#[doc = "TOUCH_PAD1 (rw) register accessor: configure RTC PAD1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pad1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_pad1`] module"]
pub type TOUCH_PAD1 = crate::Reg<touch_pad1::TOUCH_PAD1_SPEC>;
#[doc = "configure RTC PAD1"]
pub mod touch_pad1;
#[doc = "TOUCH_PAD2 (rw) register accessor: configure RTC PAD2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pad2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_pad2`] module"]
pub type TOUCH_PAD2 = crate::Reg<touch_pad2::TOUCH_PAD2_SPEC>;
#[doc = "configure RTC PAD2"]
pub mod touch_pad2;
#[doc = "TOUCH_PAD3 (rw) register accessor: configure RTC PAD3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pad3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_pad3`] module"]
pub type TOUCH_PAD3 = crate::Reg<touch_pad3::TOUCH_PAD3_SPEC>;
#[doc = "configure RTC PAD3"]
pub mod touch_pad3;
#[doc = "TOUCH_PAD4 (rw) register accessor: configure RTC PAD4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pad4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_pad4`] module"]
pub type TOUCH_PAD4 = crate::Reg<touch_pad4::TOUCH_PAD4_SPEC>;
#[doc = "configure RTC PAD4"]
pub mod touch_pad4;
#[doc = "TOUCH_PAD5 (rw) register accessor: configure RTC PAD5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pad5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_pad5`] module"]
pub type TOUCH_PAD5 = crate::Reg<touch_pad5::TOUCH_PAD5_SPEC>;
#[doc = "configure RTC PAD5"]
pub mod touch_pad5;
#[doc = "TOUCH_PAD6 (rw) register accessor: configure RTC PAD6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pad6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_pad6`] module"]
pub type TOUCH_PAD6 = crate::Reg<touch_pad6::TOUCH_PAD6_SPEC>;
#[doc = "configure RTC PAD6"]
pub mod touch_pad6;
#[doc = "TOUCH_PAD7 (rw) register accessor: configure RTC PAD7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pad7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_pad7`] module"]
pub type TOUCH_PAD7 = crate::Reg<touch_pad7::TOUCH_PAD7_SPEC>;
#[doc = "configure RTC PAD7"]
pub mod touch_pad7;
#[doc = "TOUCH_PAD8 (rw) register accessor: configure RTC PAD8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pad8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_pad8`] module"]
pub type TOUCH_PAD8 = crate::Reg<touch_pad8::TOUCH_PAD8_SPEC>;
#[doc = "configure RTC PAD8"]
pub mod touch_pad8;
#[doc = "TOUCH_PAD9 (rw) register accessor: configure RTC PAD9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pad9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_pad9`] module"]
pub type TOUCH_PAD9 = crate::Reg<touch_pad9::TOUCH_PAD9_SPEC>;
#[doc = "configure RTC PAD9"]
pub mod touch_pad9;
#[doc = "TOUCH_PAD10 (rw) register accessor: configure RTC PAD10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pad10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_pad10`] module"]
pub type TOUCH_PAD10 = crate::Reg<touch_pad10::TOUCH_PAD10_SPEC>;
#[doc = "configure RTC PAD10"]
pub mod touch_pad10;
#[doc = "TOUCH_PAD11 (rw) register accessor: configure RTC PAD11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pad11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_pad11`] module"]
pub type TOUCH_PAD11 = crate::Reg<touch_pad11::TOUCH_PAD11_SPEC>;
#[doc = "configure RTC PAD11"]
pub mod touch_pad11;
#[doc = "TOUCH_PAD12 (rw) register accessor: configure RTC PAD12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pad12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_pad12`] module"]
pub type TOUCH_PAD12 = crate::Reg<touch_pad12::TOUCH_PAD12_SPEC>;
#[doc = "configure RTC PAD12"]
pub mod touch_pad12;
#[doc = "TOUCH_PAD13 (rw) register accessor: configure RTC PAD13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pad13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_pad13`] module"]
pub type TOUCH_PAD13 = crate::Reg<touch_pad13::TOUCH_PAD13_SPEC>;
#[doc = "configure RTC PAD13"]
pub mod touch_pad13;
#[doc = "TOUCH_PAD14 (rw) register accessor: configure RTC PAD14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pad14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_pad14`] module"]
pub type TOUCH_PAD14 = crate::Reg<touch_pad14::TOUCH_PAD14_SPEC>;
#[doc = "configure RTC PAD14"]
pub mod touch_pad14;
#[doc = "XTAL_32P_PAD (rw) register accessor: configure RTC PAD15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xtal_32p_pad::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal_32p_pad::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`xtal_32p_pad`] module"]
pub type XTAL_32P_PAD = crate::Reg<xtal_32p_pad::XTAL_32P_PAD_SPEC>;
#[doc = "configure RTC PAD15"]
pub mod xtal_32p_pad;
#[doc = "XTAL_32N_PAD (rw) register accessor: configure RTC PAD16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xtal_32n_pad::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal_32n_pad::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`xtal_32n_pad`] module"]
pub type XTAL_32N_PAD = crate::Reg<xtal_32n_pad::XTAL_32N_PAD_SPEC>;
#[doc = "configure RTC PAD16"]
pub mod xtal_32n_pad;
#[doc = "PAD_DAC1 (rw) register accessor: configure RTC PAD17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_dac1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_dac1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pad_dac1`] module"]
pub type PAD_DAC1 = crate::Reg<pad_dac1::PAD_DAC1_SPEC>;
#[doc = "configure RTC PAD17"]
pub mod pad_dac1;
#[doc = "PAD_DAC2 (rw) register accessor: configure RTC PAD18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_dac2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_dac2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pad_dac2`] module"]
pub type PAD_DAC2 = crate::Reg<pad_dac2::PAD_DAC2_SPEC>;
#[doc = "configure RTC PAD18"]
pub mod pad_dac2;
#[doc = "RTC_PAD19 (rw) register accessor: configure RTC PAD19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_pad19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_pad19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_pad19`] module"]
pub type RTC_PAD19 = crate::Reg<rtc_pad19::RTC_PAD19_SPEC>;
#[doc = "configure RTC PAD19"]
pub mod rtc_pad19;
#[doc = "RTC_PAD20 (rw) register accessor: configure RTC PAD20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_pad20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_pad20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_pad20`] module"]
pub type RTC_PAD20 = crate::Reg<rtc_pad20::RTC_PAD20_SPEC>;
#[doc = "configure RTC PAD20"]
pub mod rtc_pad20;
#[doc = "RTC_PAD21 (rw) register accessor: configure RTC PAD21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_pad21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_pad21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_pad21`] module"]
pub type RTC_PAD21 = crate::Reg<rtc_pad21::RTC_PAD21_SPEC>;
#[doc = "configure RTC PAD21"]
pub mod rtc_pad21;
#[doc = "EXT_WAKEUP0 (rw) register accessor: configure EXT0 wakeup\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ext_wakeup0`] module"]
pub type EXT_WAKEUP0 = crate::Reg<ext_wakeup0::EXT_WAKEUP0_SPEC>;
#[doc = "configure EXT0 wakeup"]
pub mod ext_wakeup0;
#[doc = "XTL_EXT_CTR (rw) register accessor: configure gpio pd XTAL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xtl_ext_ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtl_ext_ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`xtl_ext_ctr`] module"]
pub type XTL_EXT_CTR = crate::Reg<xtl_ext_ctr::XTL_EXT_CTR_SPEC>;
#[doc = "configure gpio pd XTAL"]
pub mod xtl_ext_ctr;
#[doc = "SAR_I2C_IO (rw) register accessor: configure rtc i2c mux\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_i2c_io::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_i2c_io::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar_i2c_io`] module"]
pub type SAR_I2C_IO = crate::Reg<sar_i2c_io::SAR_I2C_IO_SPEC>;
#[doc = "configure rtc i2c mux"]
pub mod sar_i2c_io;
#[doc = "TOUCH_CTRL (rw) register accessor: configure touch pad bufmode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_ctrl`] module"]
pub type TOUCH_CTRL = crate::Reg<touch_ctrl::TOUCH_CTRL_SPEC>;
#[doc = "configure touch pad bufmode"]
pub mod touch_ctrl;
#[doc = "DATE (rw) register accessor: version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "version"]
pub mod date;
