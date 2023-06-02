#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    _reserved0: [u8; 0x14],
    #[doc = "0x14 - RTC_STATE1"]
    pub rtc_state1: RTC_STATE1,
    _reserved1: [u8; 0x18],
    #[doc = "0x30 - RTC_STORE0"]
    pub rtc_store0: RTC_STORE0,
    _reserved2: [u8; 0x34],
    #[doc = "0x68 - RTC_GPIO_OUT"]
    pub rtc_gpio_out: RTC_GPIO_OUT,
    _reserved3: [u8; 0x08],
    #[doc = "0x74 - RTC_GPIO_ENABLE"]
    pub rtc_gpio_enable: RTC_GPIO_ENABLE,
    _reserved4: [u8; 0x14],
    #[doc = "0x8c - RTC_GPIO_IN_DATA"]
    pub rtc_gpio_in_data: RTC_GPIO_IN_DATA,
    #[doc = "0x90 - RTC_GPIO_CONF"]
    pub rtc_gpio_conf: RTC_GPIO_CONF,
    _reserved6: [u8; 0x0c],
    #[doc = "0xa0 - PAD_XPD_DCDC_CONF"]
    pub pad_xpd_dcdc_conf: PAD_XPD_DCDC_CONF,
}
#[doc = "RTC_STORE0 (rw) register accessor: an alias for `Reg<RTC_STORE0_SPEC>`"]
pub type RTC_STORE0 = crate::Reg<rtc_store0::RTC_STORE0_SPEC>;
#[doc = "RTC_STORE0"]
pub mod rtc_store0;
#[doc = "RTC_STATE1 (rw) register accessor: an alias for `Reg<RTC_STATE1_SPEC>`"]
pub type RTC_STATE1 = crate::Reg<rtc_state1::RTC_STATE1_SPEC>;
#[doc = "RTC_STATE1"]
pub mod rtc_state1;
#[doc = "PAD_XPD_DCDC_CONF (rw) register accessor: an alias for `Reg<PAD_XPD_DCDC_CONF_SPEC>`"]
pub type PAD_XPD_DCDC_CONF = crate::Reg<pad_xpd_dcdc_conf::PAD_XPD_DCDC_CONF_SPEC>;
#[doc = "PAD_XPD_DCDC_CONF"]
pub mod pad_xpd_dcdc_conf;
#[doc = "RTC_GPIO_CONF (rw) register accessor: an alias for `Reg<RTC_GPIO_CONF_SPEC>`"]
pub type RTC_GPIO_CONF = crate::Reg<rtc_gpio_conf::RTC_GPIO_CONF_SPEC>;
#[doc = "RTC_GPIO_CONF"]
pub mod rtc_gpio_conf;
#[doc = "RTC_GPIO_ENABLE (rw) register accessor: an alias for `Reg<RTC_GPIO_ENABLE_SPEC>`"]
pub type RTC_GPIO_ENABLE = crate::Reg<rtc_gpio_enable::RTC_GPIO_ENABLE_SPEC>;
#[doc = "RTC_GPIO_ENABLE"]
pub mod rtc_gpio_enable;
#[doc = "RTC_GPIO_IN_DATA (rw) register accessor: an alias for `Reg<RTC_GPIO_IN_DATA_SPEC>`"]
pub type RTC_GPIO_IN_DATA = crate::Reg<rtc_gpio_in_data::RTC_GPIO_IN_DATA_SPEC>;
#[doc = "RTC_GPIO_IN_DATA"]
pub mod rtc_gpio_in_data;
#[doc = "RTC_GPIO_OUT (rw) register accessor: an alias for `Reg<RTC_GPIO_OUT_SPEC>`"]
pub type RTC_GPIO_OUT = crate::Reg<rtc_gpio_out::RTC_GPIO_OUT_SPEC>;
#[doc = "RTC_GPIO_OUT"]
pub mod rtc_gpio_out;
