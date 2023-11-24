#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    _reserved0: [u8; 0x14],
    rtc_state1: RTC_STATE1,
    _reserved1: [u8; 0x18],
    rtc_store0: RTC_STORE0,
    _reserved2: [u8; 0x34],
    rtc_gpio_out: RTC_GPIO_OUT,
    _reserved3: [u8; 0x08],
    rtc_gpio_enable: RTC_GPIO_ENABLE,
    _reserved4: [u8; 0x14],
    rtc_gpio_in_data: RTC_GPIO_IN_DATA,
    rtc_gpio_conf: RTC_GPIO_CONF,
    _reserved6: [u8; 0x0c],
    pad_xpd_dcdc_conf: PAD_XPD_DCDC_CONF,
}
impl RegisterBlock {
    #[doc = "0x14 - RTC_STATE1"]
    #[inline(always)]
    pub const fn rtc_state1(&self) -> &RTC_STATE1 {
        &self.rtc_state1
    }
    #[doc = "0x30 - RTC_STORE0"]
    #[inline(always)]
    pub const fn rtc_store0(&self) -> &RTC_STORE0 {
        &self.rtc_store0
    }
    #[doc = "0x68 - RTC_GPIO_OUT"]
    #[inline(always)]
    pub const fn rtc_gpio_out(&self) -> &RTC_GPIO_OUT {
        &self.rtc_gpio_out
    }
    #[doc = "0x74 - RTC_GPIO_ENABLE"]
    #[inline(always)]
    pub const fn rtc_gpio_enable(&self) -> &RTC_GPIO_ENABLE {
        &self.rtc_gpio_enable
    }
    #[doc = "0x8c - RTC_GPIO_IN_DATA"]
    #[inline(always)]
    pub const fn rtc_gpio_in_data(&self) -> &RTC_GPIO_IN_DATA {
        &self.rtc_gpio_in_data
    }
    #[doc = "0x90 - RTC_GPIO_CONF"]
    #[inline(always)]
    pub const fn rtc_gpio_conf(&self) -> &RTC_GPIO_CONF {
        &self.rtc_gpio_conf
    }
    #[doc = "0xa0 - PAD_XPD_DCDC_CONF"]
    #[inline(always)]
    pub const fn pad_xpd_dcdc_conf(&self) -> &PAD_XPD_DCDC_CONF {
        &self.pad_xpd_dcdc_conf
    }
}
#[doc = "RTC_STORE0 (rw) register accessor: RTC_STORE0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_store0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_store0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_store0`] module"]
pub type RTC_STORE0 = crate::Reg<rtc_store0::RTC_STORE0_SPEC>;
#[doc = "RTC_STORE0"]
pub mod rtc_store0;
#[doc = "RTC_STATE1 (rw) register accessor: RTC_STATE1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_state1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_state1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_state1`] module"]
pub type RTC_STATE1 = crate::Reg<rtc_state1::RTC_STATE1_SPEC>;
#[doc = "RTC_STATE1"]
pub mod rtc_state1;
#[doc = "PAD_XPD_DCDC_CONF (rw) register accessor: PAD_XPD_DCDC_CONF\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_xpd_dcdc_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_xpd_dcdc_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_xpd_dcdc_conf`] module"]
pub type PAD_XPD_DCDC_CONF = crate::Reg<pad_xpd_dcdc_conf::PAD_XPD_DCDC_CONF_SPEC>;
#[doc = "PAD_XPD_DCDC_CONF"]
pub mod pad_xpd_dcdc_conf;
#[doc = "RTC_GPIO_CONF (rw) register accessor: RTC_GPIO_CONF\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_gpio_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_gpio_conf`] module"]
pub type RTC_GPIO_CONF = crate::Reg<rtc_gpio_conf::RTC_GPIO_CONF_SPEC>;
#[doc = "RTC_GPIO_CONF"]
pub mod rtc_gpio_conf;
#[doc = "RTC_GPIO_ENABLE (rw) register accessor: RTC_GPIO_ENABLE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_gpio_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_gpio_enable`] module"]
pub type RTC_GPIO_ENABLE = crate::Reg<rtc_gpio_enable::RTC_GPIO_ENABLE_SPEC>;
#[doc = "RTC_GPIO_ENABLE"]
pub mod rtc_gpio_enable;
#[doc = "RTC_GPIO_IN_DATA (rw) register accessor: RTC_GPIO_IN_DATA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_gpio_in_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_in_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_gpio_in_data`] module"]
pub type RTC_GPIO_IN_DATA = crate::Reg<rtc_gpio_in_data::RTC_GPIO_IN_DATA_SPEC>;
#[doc = "RTC_GPIO_IN_DATA"]
pub mod rtc_gpio_in_data;
#[doc = "RTC_GPIO_OUT (rw) register accessor: RTC_GPIO_OUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_gpio_out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_gpio_out`] module"]
pub type RTC_GPIO_OUT = crate::Reg<rtc_gpio_out::RTC_GPIO_OUT_SPEC>;
#[doc = "RTC_GPIO_OUT"]
pub mod rtc_gpio_out;
