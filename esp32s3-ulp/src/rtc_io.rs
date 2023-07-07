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
    pub rtc_gpio_enable_w1tc: RTC_GPIO_ENABLE_W1TC,
    #[doc = "0x18 - RTC GPIO 0 ~ 21 interrupt status"]
    pub rtc_gpio_status: RTC_GPIO_STATUS,
    #[doc = "0x1c - One set RTC GPIO 0 ~ 21 interrupt status"]
    pub rtc_gpio_status_w1ts: RTC_GPIO_STATUS_W1TS,
    #[doc = "0x20 - One clear RTC GPIO 0 ~ 21 interrupt status"]
    pub rtc_gpio_status_w1tc: RTC_GPIO_STATUS_W1TC,
    #[doc = "0x24 - RTC GPIO input data"]
    pub rtc_gpio_in: RTC_GPIO_IN,
    #[doc = "0x28 - configure RTC GPIO0"]
    pub rtc_gpio_pin0: RTC_GPIO_PIN0,
    #[doc = "0x2c - configure RTC GPIO1"]
    pub rtc_gpio_pin1: RTC_GPIO_PIN1,
    #[doc = "0x30 - configure RTC GPIO2"]
    pub rtc_gpio_pin2: RTC_GPIO_PIN2,
    #[doc = "0x34 - configure RTC GPIO3"]
    pub rtc_gpio_pin3: RTC_GPIO_PIN3,
    #[doc = "0x38 - configure RTC GPIO4"]
    pub rtc_gpio_pin4: RTC_GPIO_PIN4,
    #[doc = "0x3c - configure RTC GPIO5"]
    pub rtc_gpio_pin5: RTC_GPIO_PIN5,
    #[doc = "0x40 - configure RTC GPIO6"]
    pub rtc_gpio_pin6: RTC_GPIO_PIN6,
    #[doc = "0x44 - configure RTC GPIO7"]
    pub rtc_gpio_pin7: RTC_GPIO_PIN7,
    #[doc = "0x48 - configure RTC GPIO8"]
    pub rtc_gpio_pin8: RTC_GPIO_PIN8,
    #[doc = "0x4c - configure RTC GPIO9"]
    pub rtc_gpio_pin9: RTC_GPIO_PIN9,
    #[doc = "0x50 - configure RTC GPIO10"]
    pub rtc_gpio_pin10: RTC_GPIO_PIN10,
    #[doc = "0x54 - configure RTC GPIO11"]
    pub rtc_gpio_pin11: RTC_GPIO_PIN11,
    #[doc = "0x58 - configure RTC GPIO12"]
    pub rtc_gpio_pin12: RTC_GPIO_PIN12,
    #[doc = "0x5c - configure RTC GPIO13"]
    pub rtc_gpio_pin13: RTC_GPIO_PIN13,
    #[doc = "0x60 - configure RTC GPIO14"]
    pub rtc_gpio_pin14: RTC_GPIO_PIN14,
    #[doc = "0x64 - configure RTC GPIO15"]
    pub rtc_gpio_pin15: RTC_GPIO_PIN15,
    #[doc = "0x68 - configure RTC GPIO16"]
    pub rtc_gpio_pin16: RTC_GPIO_PIN16,
    #[doc = "0x6c - configure RTC GPIO17"]
    pub rtc_gpio_pin17: RTC_GPIO_PIN17,
    #[doc = "0x70 - configure RTC GPIO18"]
    pub rtc_gpio_pin18: RTC_GPIO_PIN18,
    #[doc = "0x74 - configure RTC GPIO19"]
    pub rtc_gpio_pin19: RTC_GPIO_PIN19,
    #[doc = "0x78 - configure RTC GPIO20"]
    pub rtc_gpio_pin20: RTC_GPIO_PIN20,
    #[doc = "0x7c - configure RTC GPIO21"]
    pub rtc_gpio_pin21: RTC_GPIO_PIN21,
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
    _reserved59: [u8; 0x0110],
    #[doc = "0x1fc - version"]
    pub date: DATE,
}
#[doc = "RTC_GPIO_OUT (rw) register accessor: an alias for `Reg<RTC_GPIO_OUT_SPEC>`"]
pub type RTC_GPIO_OUT = crate::Reg<rtc_gpio_out::RTC_GPIO_OUT_SPEC>;
#[doc = "RTC GPIO 0 ~ 21 output data register"]
pub mod rtc_gpio_out;
#[doc = "RTC_GPIO_OUT_W1TS (w) register accessor: an alias for `Reg<RTC_GPIO_OUT_W1TS_SPEC>`"]
pub type RTC_GPIO_OUT_W1TS = crate::Reg<rtc_gpio_out_w1ts::RTC_GPIO_OUT_W1TS_SPEC>;
#[doc = "one set RTC GPIO output data"]
pub mod rtc_gpio_out_w1ts;
#[doc = "RTC_GPIO_OUT_W1TC (w) register accessor: an alias for `Reg<RTC_GPIO_OUT_W1TC_SPEC>`"]
pub type RTC_GPIO_OUT_W1TC = crate::Reg<rtc_gpio_out_w1tc::RTC_GPIO_OUT_W1TC_SPEC>;
#[doc = "one clear RTC GPIO output data"]
pub mod rtc_gpio_out_w1tc;
#[doc = "RTC_GPIO_ENABLE (rw) register accessor: an alias for `Reg<RTC_GPIO_ENABLE_SPEC>`"]
pub type RTC_GPIO_ENABLE = crate::Reg<rtc_gpio_enable::RTC_GPIO_ENABLE_SPEC>;
#[doc = "Configure RTC GPIO output enable"]
pub mod rtc_gpio_enable;
#[doc = "RTC_GPIO_ENABLE_W1TS (w) register accessor: an alias for `Reg<RTC_GPIO_ENABLE_W1TS_SPEC>`"]
pub type RTC_GPIO_ENABLE_W1TS = crate::Reg<rtc_gpio_enable_w1ts::RTC_GPIO_ENABLE_W1TS_SPEC>;
#[doc = "one set RTC GPIO output enable"]
pub mod rtc_gpio_enable_w1ts;
#[doc = "RTC_GPIO_ENABLE_W1TC (w) register accessor: an alias for `Reg<RTC_GPIO_ENABLE_W1TC_SPEC>`"]
pub type RTC_GPIO_ENABLE_W1TC = crate::Reg<rtc_gpio_enable_w1tc::RTC_GPIO_ENABLE_W1TC_SPEC>;
#[doc = "one clear RTC GPIO output enable"]
pub mod rtc_gpio_enable_w1tc;
#[doc = "RTC_GPIO_STATUS (rw) register accessor: an alias for `Reg<RTC_GPIO_STATUS_SPEC>`"]
pub type RTC_GPIO_STATUS = crate::Reg<rtc_gpio_status::RTC_GPIO_STATUS_SPEC>;
#[doc = "RTC GPIO 0 ~ 21 interrupt status"]
pub mod rtc_gpio_status;
#[doc = "RTC_GPIO_STATUS_W1TS (w) register accessor: an alias for `Reg<RTC_GPIO_STATUS_W1TS_SPEC>`"]
pub type RTC_GPIO_STATUS_W1TS = crate::Reg<rtc_gpio_status_w1ts::RTC_GPIO_STATUS_W1TS_SPEC>;
#[doc = "One set RTC GPIO 0 ~ 21 interrupt status"]
pub mod rtc_gpio_status_w1ts;
#[doc = "RTC_GPIO_STATUS_W1TC (w) register accessor: an alias for `Reg<RTC_GPIO_STATUS_W1TC_SPEC>`"]
pub type RTC_GPIO_STATUS_W1TC = crate::Reg<rtc_gpio_status_w1tc::RTC_GPIO_STATUS_W1TC_SPEC>;
#[doc = "One clear RTC GPIO 0 ~ 21 interrupt status"]
pub mod rtc_gpio_status_w1tc;
#[doc = "RTC_GPIO_IN (r) register accessor: an alias for `Reg<RTC_GPIO_IN_SPEC>`"]
pub type RTC_GPIO_IN = crate::Reg<rtc_gpio_in::RTC_GPIO_IN_SPEC>;
#[doc = "RTC GPIO input data"]
pub mod rtc_gpio_in;
#[doc = "RTC_GPIO_PIN0 (rw) register accessor: an alias for `Reg<RTC_GPIO_PIN0_SPEC>`"]
pub type RTC_GPIO_PIN0 = crate::Reg<rtc_gpio_pin0::RTC_GPIO_PIN0_SPEC>;
#[doc = "configure RTC GPIO0"]
pub mod rtc_gpio_pin0;
#[doc = "RTC_GPIO_PIN1 (rw) register accessor: an alias for `Reg<RTC_GPIO_PIN1_SPEC>`"]
pub type RTC_GPIO_PIN1 = crate::Reg<rtc_gpio_pin1::RTC_GPIO_PIN1_SPEC>;
#[doc = "configure RTC GPIO1"]
pub mod rtc_gpio_pin1;
#[doc = "RTC_GPIO_PIN2 (rw) register accessor: an alias for `Reg<RTC_GPIO_PIN2_SPEC>`"]
pub type RTC_GPIO_PIN2 = crate::Reg<rtc_gpio_pin2::RTC_GPIO_PIN2_SPEC>;
#[doc = "configure RTC GPIO2"]
pub mod rtc_gpio_pin2;
#[doc = "RTC_GPIO_PIN3 (rw) register accessor: an alias for `Reg<RTC_GPIO_PIN3_SPEC>`"]
pub type RTC_GPIO_PIN3 = crate::Reg<rtc_gpio_pin3::RTC_GPIO_PIN3_SPEC>;
#[doc = "configure RTC GPIO3"]
pub mod rtc_gpio_pin3;
#[doc = "RTC_GPIO_PIN4 (rw) register accessor: an alias for `Reg<RTC_GPIO_PIN4_SPEC>`"]
pub type RTC_GPIO_PIN4 = crate::Reg<rtc_gpio_pin4::RTC_GPIO_PIN4_SPEC>;
#[doc = "configure RTC GPIO4"]
pub mod rtc_gpio_pin4;
#[doc = "RTC_GPIO_PIN5 (rw) register accessor: an alias for `Reg<RTC_GPIO_PIN5_SPEC>`"]
pub type RTC_GPIO_PIN5 = crate::Reg<rtc_gpio_pin5::RTC_GPIO_PIN5_SPEC>;
#[doc = "configure RTC GPIO5"]
pub mod rtc_gpio_pin5;
#[doc = "RTC_GPIO_PIN6 (rw) register accessor: an alias for `Reg<RTC_GPIO_PIN6_SPEC>`"]
pub type RTC_GPIO_PIN6 = crate::Reg<rtc_gpio_pin6::RTC_GPIO_PIN6_SPEC>;
#[doc = "configure RTC GPIO6"]
pub mod rtc_gpio_pin6;
#[doc = "RTC_GPIO_PIN7 (rw) register accessor: an alias for `Reg<RTC_GPIO_PIN7_SPEC>`"]
pub type RTC_GPIO_PIN7 = crate::Reg<rtc_gpio_pin7::RTC_GPIO_PIN7_SPEC>;
#[doc = "configure RTC GPIO7"]
pub mod rtc_gpio_pin7;
#[doc = "RTC_GPIO_PIN8 (rw) register accessor: an alias for `Reg<RTC_GPIO_PIN8_SPEC>`"]
pub type RTC_GPIO_PIN8 = crate::Reg<rtc_gpio_pin8::RTC_GPIO_PIN8_SPEC>;
#[doc = "configure RTC GPIO8"]
pub mod rtc_gpio_pin8;
#[doc = "RTC_GPIO_PIN9 (rw) register accessor: an alias for `Reg<RTC_GPIO_PIN9_SPEC>`"]
pub type RTC_GPIO_PIN9 = crate::Reg<rtc_gpio_pin9::RTC_GPIO_PIN9_SPEC>;
#[doc = "configure RTC GPIO9"]
pub mod rtc_gpio_pin9;
#[doc = "RTC_GPIO_PIN10 (rw) register accessor: an alias for `Reg<RTC_GPIO_PIN10_SPEC>`"]
pub type RTC_GPIO_PIN10 = crate::Reg<rtc_gpio_pin10::RTC_GPIO_PIN10_SPEC>;
#[doc = "configure RTC GPIO10"]
pub mod rtc_gpio_pin10;
#[doc = "RTC_GPIO_PIN11 (rw) register accessor: an alias for `Reg<RTC_GPIO_PIN11_SPEC>`"]
pub type RTC_GPIO_PIN11 = crate::Reg<rtc_gpio_pin11::RTC_GPIO_PIN11_SPEC>;
#[doc = "configure RTC GPIO11"]
pub mod rtc_gpio_pin11;
#[doc = "RTC_GPIO_PIN12 (rw) register accessor: an alias for `Reg<RTC_GPIO_PIN12_SPEC>`"]
pub type RTC_GPIO_PIN12 = crate::Reg<rtc_gpio_pin12::RTC_GPIO_PIN12_SPEC>;
#[doc = "configure RTC GPIO12"]
pub mod rtc_gpio_pin12;
#[doc = "RTC_GPIO_PIN13 (rw) register accessor: an alias for `Reg<RTC_GPIO_PIN13_SPEC>`"]
pub type RTC_GPIO_PIN13 = crate::Reg<rtc_gpio_pin13::RTC_GPIO_PIN13_SPEC>;
#[doc = "configure RTC GPIO13"]
pub mod rtc_gpio_pin13;
#[doc = "RTC_GPIO_PIN14 (rw) register accessor: an alias for `Reg<RTC_GPIO_PIN14_SPEC>`"]
pub type RTC_GPIO_PIN14 = crate::Reg<rtc_gpio_pin14::RTC_GPIO_PIN14_SPEC>;
#[doc = "configure RTC GPIO14"]
pub mod rtc_gpio_pin14;
#[doc = "RTC_GPIO_PIN15 (rw) register accessor: an alias for `Reg<RTC_GPIO_PIN15_SPEC>`"]
pub type RTC_GPIO_PIN15 = crate::Reg<rtc_gpio_pin15::RTC_GPIO_PIN15_SPEC>;
#[doc = "configure RTC GPIO15"]
pub mod rtc_gpio_pin15;
#[doc = "RTC_GPIO_PIN16 (rw) register accessor: an alias for `Reg<RTC_GPIO_PIN16_SPEC>`"]
pub type RTC_GPIO_PIN16 = crate::Reg<rtc_gpio_pin16::RTC_GPIO_PIN16_SPEC>;
#[doc = "configure RTC GPIO16"]
pub mod rtc_gpio_pin16;
#[doc = "RTC_GPIO_PIN17 (rw) register accessor: an alias for `Reg<RTC_GPIO_PIN17_SPEC>`"]
pub type RTC_GPIO_PIN17 = crate::Reg<rtc_gpio_pin17::RTC_GPIO_PIN17_SPEC>;
#[doc = "configure RTC GPIO17"]
pub mod rtc_gpio_pin17;
#[doc = "RTC_GPIO_PIN18 (rw) register accessor: an alias for `Reg<RTC_GPIO_PIN18_SPEC>`"]
pub type RTC_GPIO_PIN18 = crate::Reg<rtc_gpio_pin18::RTC_GPIO_PIN18_SPEC>;
#[doc = "configure RTC GPIO18"]
pub mod rtc_gpio_pin18;
#[doc = "RTC_GPIO_PIN19 (rw) register accessor: an alias for `Reg<RTC_GPIO_PIN19_SPEC>`"]
pub type RTC_GPIO_PIN19 = crate::Reg<rtc_gpio_pin19::RTC_GPIO_PIN19_SPEC>;
#[doc = "configure RTC GPIO19"]
pub mod rtc_gpio_pin19;
#[doc = "RTC_GPIO_PIN20 (rw) register accessor: an alias for `Reg<RTC_GPIO_PIN20_SPEC>`"]
pub type RTC_GPIO_PIN20 = crate::Reg<rtc_gpio_pin20::RTC_GPIO_PIN20_SPEC>;
#[doc = "configure RTC GPIO20"]
pub mod rtc_gpio_pin20;
#[doc = "RTC_GPIO_PIN21 (rw) register accessor: an alias for `Reg<RTC_GPIO_PIN21_SPEC>`"]
pub type RTC_GPIO_PIN21 = crate::Reg<rtc_gpio_pin21::RTC_GPIO_PIN21_SPEC>;
#[doc = "configure RTC GPIO21"]
pub mod rtc_gpio_pin21;
#[doc = "RTC_DEBUG_SEL (rw) register accessor: an alias for `Reg<RTC_DEBUG_SEL_SPEC>`"]
pub type RTC_DEBUG_SEL = crate::Reg<rtc_debug_sel::RTC_DEBUG_SEL_SPEC>;
#[doc = "configure rtc debug"]
pub mod rtc_debug_sel;
#[doc = "TOUCH_PAD0 (rw) register accessor: an alias for `Reg<TOUCH_PAD0_SPEC>`"]
pub type TOUCH_PAD0 = crate::Reg<touch_pad0::TOUCH_PAD0_SPEC>;
#[doc = "configure RTC PAD0"]
pub mod touch_pad0;
#[doc = "TOUCH_PAD1 (rw) register accessor: an alias for `Reg<TOUCH_PAD1_SPEC>`"]
pub type TOUCH_PAD1 = crate::Reg<touch_pad1::TOUCH_PAD1_SPEC>;
#[doc = "configure RTC PAD1"]
pub mod touch_pad1;
#[doc = "TOUCH_PAD2 (rw) register accessor: an alias for `Reg<TOUCH_PAD2_SPEC>`"]
pub type TOUCH_PAD2 = crate::Reg<touch_pad2::TOUCH_PAD2_SPEC>;
#[doc = "configure RTC PAD2"]
pub mod touch_pad2;
#[doc = "TOUCH_PAD3 (rw) register accessor: an alias for `Reg<TOUCH_PAD3_SPEC>`"]
pub type TOUCH_PAD3 = crate::Reg<touch_pad3::TOUCH_PAD3_SPEC>;
#[doc = "configure RTC PAD3"]
pub mod touch_pad3;
#[doc = "TOUCH_PAD4 (rw) register accessor: an alias for `Reg<TOUCH_PAD4_SPEC>`"]
pub type TOUCH_PAD4 = crate::Reg<touch_pad4::TOUCH_PAD4_SPEC>;
#[doc = "configure RTC PAD4"]
pub mod touch_pad4;
#[doc = "TOUCH_PAD5 (rw) register accessor: an alias for `Reg<TOUCH_PAD5_SPEC>`"]
pub type TOUCH_PAD5 = crate::Reg<touch_pad5::TOUCH_PAD5_SPEC>;
#[doc = "configure RTC PAD5"]
pub mod touch_pad5;
#[doc = "TOUCH_PAD6 (rw) register accessor: an alias for `Reg<TOUCH_PAD6_SPEC>`"]
pub type TOUCH_PAD6 = crate::Reg<touch_pad6::TOUCH_PAD6_SPEC>;
#[doc = "configure RTC PAD6"]
pub mod touch_pad6;
#[doc = "TOUCH_PAD7 (rw) register accessor: an alias for `Reg<TOUCH_PAD7_SPEC>`"]
pub type TOUCH_PAD7 = crate::Reg<touch_pad7::TOUCH_PAD7_SPEC>;
#[doc = "configure RTC PAD7"]
pub mod touch_pad7;
#[doc = "TOUCH_PAD8 (rw) register accessor: an alias for `Reg<TOUCH_PAD8_SPEC>`"]
pub type TOUCH_PAD8 = crate::Reg<touch_pad8::TOUCH_PAD8_SPEC>;
#[doc = "configure RTC PAD8"]
pub mod touch_pad8;
#[doc = "TOUCH_PAD9 (rw) register accessor: an alias for `Reg<TOUCH_PAD9_SPEC>`"]
pub type TOUCH_PAD9 = crate::Reg<touch_pad9::TOUCH_PAD9_SPEC>;
#[doc = "configure RTC PAD9"]
pub mod touch_pad9;
#[doc = "TOUCH_PAD10 (rw) register accessor: an alias for `Reg<TOUCH_PAD10_SPEC>`"]
pub type TOUCH_PAD10 = crate::Reg<touch_pad10::TOUCH_PAD10_SPEC>;
#[doc = "configure RTC PAD10"]
pub mod touch_pad10;
#[doc = "TOUCH_PAD11 (rw) register accessor: an alias for `Reg<TOUCH_PAD11_SPEC>`"]
pub type TOUCH_PAD11 = crate::Reg<touch_pad11::TOUCH_PAD11_SPEC>;
#[doc = "configure RTC PAD11"]
pub mod touch_pad11;
#[doc = "TOUCH_PAD12 (rw) register accessor: an alias for `Reg<TOUCH_PAD12_SPEC>`"]
pub type TOUCH_PAD12 = crate::Reg<touch_pad12::TOUCH_PAD12_SPEC>;
#[doc = "configure RTC PAD12"]
pub mod touch_pad12;
#[doc = "TOUCH_PAD13 (rw) register accessor: an alias for `Reg<TOUCH_PAD13_SPEC>`"]
pub type TOUCH_PAD13 = crate::Reg<touch_pad13::TOUCH_PAD13_SPEC>;
#[doc = "configure RTC PAD13"]
pub mod touch_pad13;
#[doc = "TOUCH_PAD14 (rw) register accessor: an alias for `Reg<TOUCH_PAD14_SPEC>`"]
pub type TOUCH_PAD14 = crate::Reg<touch_pad14::TOUCH_PAD14_SPEC>;
#[doc = "configure RTC PAD14"]
pub mod touch_pad14;
#[doc = "XTAL_32P_PAD (rw) register accessor: an alias for `Reg<XTAL_32P_PAD_SPEC>`"]
pub type XTAL_32P_PAD = crate::Reg<xtal_32p_pad::XTAL_32P_PAD_SPEC>;
#[doc = "configure RTC PAD15"]
pub mod xtal_32p_pad;
#[doc = "XTAL_32N_PAD (rw) register accessor: an alias for `Reg<XTAL_32N_PAD_SPEC>`"]
pub type XTAL_32N_PAD = crate::Reg<xtal_32n_pad::XTAL_32N_PAD_SPEC>;
#[doc = "configure RTC PAD16"]
pub mod xtal_32n_pad;
#[doc = "PAD_DAC1 (rw) register accessor: an alias for `Reg<PAD_DAC1_SPEC>`"]
pub type PAD_DAC1 = crate::Reg<pad_dac1::PAD_DAC1_SPEC>;
#[doc = "configure RTC PAD17"]
pub mod pad_dac1;
#[doc = "PAD_DAC2 (rw) register accessor: an alias for `Reg<PAD_DAC2_SPEC>`"]
pub type PAD_DAC2 = crate::Reg<pad_dac2::PAD_DAC2_SPEC>;
#[doc = "configure RTC PAD18"]
pub mod pad_dac2;
#[doc = "RTC_PAD19 (rw) register accessor: an alias for `Reg<RTC_PAD19_SPEC>`"]
pub type RTC_PAD19 = crate::Reg<rtc_pad19::RTC_PAD19_SPEC>;
#[doc = "configure RTC PAD19"]
pub mod rtc_pad19;
#[doc = "RTC_PAD20 (rw) register accessor: an alias for `Reg<RTC_PAD20_SPEC>`"]
pub type RTC_PAD20 = crate::Reg<rtc_pad20::RTC_PAD20_SPEC>;
#[doc = "configure RTC PAD20"]
pub mod rtc_pad20;
#[doc = "RTC_PAD21 (rw) register accessor: an alias for `Reg<RTC_PAD21_SPEC>`"]
pub type RTC_PAD21 = crate::Reg<rtc_pad21::RTC_PAD21_SPEC>;
#[doc = "configure RTC PAD21"]
pub mod rtc_pad21;
#[doc = "EXT_WAKEUP0 (rw) register accessor: an alias for `Reg<EXT_WAKEUP0_SPEC>`"]
pub type EXT_WAKEUP0 = crate::Reg<ext_wakeup0::EXT_WAKEUP0_SPEC>;
#[doc = "configure EXT0 wakeup"]
pub mod ext_wakeup0;
#[doc = "XTL_EXT_CTR (rw) register accessor: an alias for `Reg<XTL_EXT_CTR_SPEC>`"]
pub type XTL_EXT_CTR = crate::Reg<xtl_ext_ctr::XTL_EXT_CTR_SPEC>;
#[doc = "configure gpio pd XTAL"]
pub mod xtl_ext_ctr;
#[doc = "SAR_I2C_IO (rw) register accessor: an alias for `Reg<SAR_I2C_IO_SPEC>`"]
pub type SAR_I2C_IO = crate::Reg<sar_i2c_io::SAR_I2C_IO_SPEC>;
#[doc = "configure rtc i2c mux"]
pub mod sar_i2c_io;
#[doc = "TOUCH_CTRL (rw) register accessor: an alias for `Reg<TOUCH_CTRL_SPEC>`"]
pub type TOUCH_CTRL = crate::Reg<touch_ctrl::TOUCH_CTRL_SPEC>;
#[doc = "configure touch pad bufmode"]
pub mod touch_ctrl;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "version"]
pub mod date;
