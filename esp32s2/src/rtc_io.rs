#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC GPIO output register"]
    pub rtc_gpio_out: RTC_GPIO_OUT,
    #[doc = "0x04 - RTC GPIO output bit set register"]
    pub rtc_gpio_out_w1ts: RTC_GPIO_OUT_W1TS,
    #[doc = "0x08 - RTC GPIO output bit clear register"]
    pub rtc_gpio_out_w1tc: RTC_GPIO_OUT_W1TC,
    #[doc = "0x0c - RTC GPIO output enable register"]
    pub rtc_gpio_enable: RTC_GPIO_ENABLE,
    #[doc = "0x10 - RTC GPIO output enable bit set register"]
    pub rtc_gpio_enable_w1ts: RTC_GPIO_ENABLE_W1TS,
    #[doc = "0x14 - RTC GPIO output enable bit clear register"]
    pub enable_w1tc: ENABLE_W1TC,
    #[doc = "0x18 - RTC GPIO interrupt status register"]
    pub rtc_gpio_status: RTC_GPIO_STATUS,
    #[doc = "0x1c - RTC GPIO interrupt status bit set register"]
    pub rtc_gpio_status_w1ts: RTC_GPIO_STATUS_W1TS,
    #[doc = "0x20 - RTC GPIO interrupt status bit clear register"]
    pub rtc_gpio_status_w1tc: RTC_GPIO_STATUS_W1TC,
    #[doc = "0x24 - RTC GPIO input register"]
    pub rtc_gpio_in: RTC_GPIO_IN,
    #[doc = "0x28..0x80 - RTC configuration for pin %s"]
    pub pin: [PIN; 22],
    #[doc = "0x80 - RTC debug select register"]
    pub rtc_debug_sel: RTC_DEBUG_SEL,
    #[doc = "0x84..0xc0 - Touch pad %s configuration register"]
    pub touch_pad: [TOUCH_PAD; 15],
    #[doc = "0xc0 - 32KHz crystal P-pad configuration register"]
    pub xtal_32p_pad: XTAL_32P_PAD,
    #[doc = "0xc4 - 32KHz crystal N-pad configuration register"]
    pub xtal_32n_pad: XTAL_32N_PAD,
    #[doc = "0xc8 - DAC1 configuration register"]
    pub pad_dac1: PAD_DAC1,
    #[doc = "0xcc - DAC2 configuration register"]
    pub pad_dac2: PAD_DAC2,
    #[doc = "0xd0 - Touch pad 19 configuration register"]
    pub rtc_pad19: RTC_PAD19,
    #[doc = "0xd4 - Touch pad 20 configuration register"]
    pub rtc_pad20: RTC_PAD20,
    #[doc = "0xd8 - Touch pad 21 configuration register"]
    pub rtc_pad21: RTC_PAD21,
    #[doc = "0xdc - External wake up configuration register"]
    pub ext_wakeup0: EXT_WAKEUP0,
    #[doc = "0xe0 - Crystal power down enable GPIO source"]
    pub xtl_ext_ctr: XTL_EXT_CTR,
    #[doc = "0xe4 - RTC I2C pad selection"]
    pub sar_i2c_io: SAR_I2C_IO,
    #[doc = "0xe8 - Touch control register"]
    pub rtc_io_touch_ctrl: RTC_IO_TOUCH_CTRL,
    _reserved24: [u8; 0x0110],
    #[doc = "0x1fc - Version control register"]
    pub rtc_io_date: RTC_IO_DATE,
}
#[doc = "RTC_GPIO_OUT (rw) register accessor: an alias for `Reg<RTC_GPIO_OUT_SPEC>`"]
pub type RTC_GPIO_OUT = crate::Reg<rtc_gpio_out::RTC_GPIO_OUT_SPEC>;
#[doc = "RTC GPIO output register"]
pub mod rtc_gpio_out;
#[doc = "RTC_GPIO_OUT_W1TS (w) register accessor: an alias for `Reg<RTC_GPIO_OUT_W1TS_SPEC>`"]
pub type RTC_GPIO_OUT_W1TS = crate::Reg<rtc_gpio_out_w1ts::RTC_GPIO_OUT_W1TS_SPEC>;
#[doc = "RTC GPIO output bit set register"]
pub mod rtc_gpio_out_w1ts;
#[doc = "RTC_GPIO_OUT_W1TC (w) register accessor: an alias for `Reg<RTC_GPIO_OUT_W1TC_SPEC>`"]
pub type RTC_GPIO_OUT_W1TC = crate::Reg<rtc_gpio_out_w1tc::RTC_GPIO_OUT_W1TC_SPEC>;
#[doc = "RTC GPIO output bit clear register"]
pub mod rtc_gpio_out_w1tc;
#[doc = "RTC_GPIO_ENABLE (rw) register accessor: an alias for `Reg<RTC_GPIO_ENABLE_SPEC>`"]
pub type RTC_GPIO_ENABLE = crate::Reg<rtc_gpio_enable::RTC_GPIO_ENABLE_SPEC>;
#[doc = "RTC GPIO output enable register"]
pub mod rtc_gpio_enable;
#[doc = "RTC_GPIO_ENABLE_W1TS (w) register accessor: an alias for `Reg<RTC_GPIO_ENABLE_W1TS_SPEC>`"]
pub type RTC_GPIO_ENABLE_W1TS = crate::Reg<rtc_gpio_enable_w1ts::RTC_GPIO_ENABLE_W1TS_SPEC>;
#[doc = "RTC GPIO output enable bit set register"]
pub mod rtc_gpio_enable_w1ts;
#[doc = "ENABLE_W1TC (w) register accessor: an alias for `Reg<ENABLE_W1TC_SPEC>`"]
pub type ENABLE_W1TC = crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>;
#[doc = "RTC GPIO output enable bit clear register"]
pub mod enable_w1tc;
#[doc = "RTC_GPIO_STATUS (rw) register accessor: an alias for `Reg<RTC_GPIO_STATUS_SPEC>`"]
pub type RTC_GPIO_STATUS = crate::Reg<rtc_gpio_status::RTC_GPIO_STATUS_SPEC>;
#[doc = "RTC GPIO interrupt status register"]
pub mod rtc_gpio_status;
#[doc = "RTC_GPIO_STATUS_W1TS (w) register accessor: an alias for `Reg<RTC_GPIO_STATUS_W1TS_SPEC>`"]
pub type RTC_GPIO_STATUS_W1TS = crate::Reg<rtc_gpio_status_w1ts::RTC_GPIO_STATUS_W1TS_SPEC>;
#[doc = "RTC GPIO interrupt status bit set register"]
pub mod rtc_gpio_status_w1ts;
#[doc = "RTC_GPIO_STATUS_W1TC (w) register accessor: an alias for `Reg<RTC_GPIO_STATUS_W1TC_SPEC>`"]
pub type RTC_GPIO_STATUS_W1TC = crate::Reg<rtc_gpio_status_w1tc::RTC_GPIO_STATUS_W1TC_SPEC>;
#[doc = "RTC GPIO interrupt status bit clear register"]
pub mod rtc_gpio_status_w1tc;
#[doc = "RTC_GPIO_IN (r) register accessor: an alias for `Reg<RTC_GPIO_IN_SPEC>`"]
pub type RTC_GPIO_IN = crate::Reg<rtc_gpio_in::RTC_GPIO_IN_SPEC>;
#[doc = "RTC GPIO input register"]
pub mod rtc_gpio_in;
#[doc = "PIN (rw) register accessor: an alias for `Reg<PIN_SPEC>`"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = "RTC configuration for pin %s"]
pub mod pin;
#[doc = "RTC_DEBUG_SEL (rw) register accessor: an alias for `Reg<RTC_DEBUG_SEL_SPEC>`"]
pub type RTC_DEBUG_SEL = crate::Reg<rtc_debug_sel::RTC_DEBUG_SEL_SPEC>;
#[doc = "RTC debug select register"]
pub mod rtc_debug_sel;
#[doc = "TOUCH_PAD (rw) register accessor: an alias for `Reg<TOUCH_PAD_SPEC>`"]
pub type TOUCH_PAD = crate::Reg<touch_pad::TOUCH_PAD_SPEC>;
#[doc = "Touch pad %s configuration register"]
pub mod touch_pad;
#[doc = "XTAL_32P_PAD (rw) register accessor: an alias for `Reg<XTAL_32P_PAD_SPEC>`"]
pub type XTAL_32P_PAD = crate::Reg<xtal_32p_pad::XTAL_32P_PAD_SPEC>;
#[doc = "32KHz crystal P-pad configuration register"]
pub mod xtal_32p_pad;
#[doc = "XTAL_32N_PAD (rw) register accessor: an alias for `Reg<XTAL_32N_PAD_SPEC>`"]
pub type XTAL_32N_PAD = crate::Reg<xtal_32n_pad::XTAL_32N_PAD_SPEC>;
#[doc = "32KHz crystal N-pad configuration register"]
pub mod xtal_32n_pad;
#[doc = "PAD_DAC1 (rw) register accessor: an alias for `Reg<PAD_DAC1_SPEC>`"]
pub type PAD_DAC1 = crate::Reg<pad_dac1::PAD_DAC1_SPEC>;
#[doc = "DAC1 configuration register"]
pub mod pad_dac1;
#[doc = "PAD_DAC2 (rw) register accessor: an alias for `Reg<PAD_DAC2_SPEC>`"]
pub type PAD_DAC2 = crate::Reg<pad_dac2::PAD_DAC2_SPEC>;
#[doc = "DAC2 configuration register"]
pub mod pad_dac2;
#[doc = "RTC_PAD19 (rw) register accessor: an alias for `Reg<RTC_PAD19_SPEC>`"]
pub type RTC_PAD19 = crate::Reg<rtc_pad19::RTC_PAD19_SPEC>;
#[doc = "Touch pad 19 configuration register"]
pub mod rtc_pad19;
#[doc = "RTC_PAD20 (rw) register accessor: an alias for `Reg<RTC_PAD20_SPEC>`"]
pub type RTC_PAD20 = crate::Reg<rtc_pad20::RTC_PAD20_SPEC>;
#[doc = "Touch pad 20 configuration register"]
pub mod rtc_pad20;
#[doc = "RTC_PAD21 (rw) register accessor: an alias for `Reg<RTC_PAD21_SPEC>`"]
pub type RTC_PAD21 = crate::Reg<rtc_pad21::RTC_PAD21_SPEC>;
#[doc = "Touch pad 21 configuration register"]
pub mod rtc_pad21;
#[doc = "EXT_WAKEUP0 (rw) register accessor: an alias for `Reg<EXT_WAKEUP0_SPEC>`"]
pub type EXT_WAKEUP0 = crate::Reg<ext_wakeup0::EXT_WAKEUP0_SPEC>;
#[doc = "External wake up configuration register"]
pub mod ext_wakeup0;
#[doc = "XTL_EXT_CTR (rw) register accessor: an alias for `Reg<XTL_EXT_CTR_SPEC>`"]
pub type XTL_EXT_CTR = crate::Reg<xtl_ext_ctr::XTL_EXT_CTR_SPEC>;
#[doc = "Crystal power down enable GPIO source"]
pub mod xtl_ext_ctr;
#[doc = "SAR_I2C_IO (rw) register accessor: an alias for `Reg<SAR_I2C_IO_SPEC>`"]
pub type SAR_I2C_IO = crate::Reg<sar_i2c_io::SAR_I2C_IO_SPEC>;
#[doc = "RTC I2C pad selection"]
pub mod sar_i2c_io;
#[doc = "RTC_IO_TOUCH_CTRL (rw) register accessor: an alias for `Reg<RTC_IO_TOUCH_CTRL_SPEC>`"]
pub type RTC_IO_TOUCH_CTRL = crate::Reg<rtc_io_touch_ctrl::RTC_IO_TOUCH_CTRL_SPEC>;
#[doc = "Touch control register"]
pub mod rtc_io_touch_ctrl;
#[doc = "RTC_IO_DATE (rw) register accessor: an alias for `Reg<RTC_IO_DATE_SPEC>`"]
pub type RTC_IO_DATE = crate::Reg<rtc_io_date::RTC_IO_DATE_SPEC>;
#[doc = "Version control register"]
pub mod rtc_io_date;
