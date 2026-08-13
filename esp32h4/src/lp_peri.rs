#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    clk_en: CLK_EN,
    reset_en: RESET_EN,
    _reserved2: [u8; 0x08],
    lp_peri_pms_conf: LP_PERI_PMS_CONF,
    lp_peri_pms_exception_info: LP_PERI_PMS_EXCEPTION_INFO,
    peri_pms_int_en: PERI_PMS_INT_EN,
    _reserved5: [u8; 0x04],
    interrupt_source: INTERRUPT_SOURCE,
    _reserved6: [u8; 0x03d8],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - configure peri in lp system clk enable"]
    #[inline(always)]
    pub const fn clk_en(&self) -> &CLK_EN {
        &self.clk_en
    }
    #[doc = "0x04 - configure peri in lp system reset enable"]
    #[inline(always)]
    pub const fn reset_en(&self) -> &RESET_EN {
        &self.reset_en
    }
    #[doc = "0x10 - LP Peripherals PMS configuration register"]
    #[inline(always)]
    pub const fn lp_peri_pms_conf(&self) -> &LP_PERI_PMS_CONF {
        &self.lp_peri_pms_conf
    }
    #[doc = "0x14 - LP Peripherals PMS exception info record register"]
    #[inline(always)]
    pub const fn lp_peri_pms_exception_info(&self) -> &LP_PERI_PMS_EXCEPTION_INFO {
        &self.lp_peri_pms_exception_info
    }
    #[doc = "0x18 - APM interrupt enable register"]
    #[inline(always)]
    pub const fn peri_pms_int_en(&self) -> &PERI_PMS_INT_EN {
        &self.peri_pms_int_en
    }
    #[doc = "0x20 - record the lp cpu interrupt"]
    #[inline(always)]
    pub const fn interrupt_source(&self) -> &INTERRUPT_SOURCE {
        &self.interrupt_source
    }
    #[doc = "0x3fc - version register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CLK_EN (rw) register accessor: configure peri in lp system clk enable\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_en`] module"]
pub type CLK_EN = crate::Reg<clk_en::CLK_EN_SPEC>;
#[doc = "configure peri in lp system clk enable"]
pub mod clk_en;
#[doc = "RESET_EN (rw) register accessor: configure peri in lp system reset enable\n\nYou can [`read`](crate::Reg::read) this register and get [`reset_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_en`] module"]
pub type RESET_EN = crate::Reg<reset_en::RESET_EN_SPEC>;
#[doc = "configure peri in lp system reset enable"]
pub mod reset_en;
#[doc = "LP_PERI_PMS_CONF (w) register accessor: LP Peripherals PMS configuration register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_peri_pms_conf::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_peri_pms_conf`] module"]
pub type LP_PERI_PMS_CONF = crate::Reg<lp_peri_pms_conf::LP_PERI_PMS_CONF_SPEC>;
#[doc = "LP Peripherals PMS configuration register"]
pub mod lp_peri_pms_conf;
#[doc = "LP_PERI_PMS_EXCEPTION_INFO (r) register accessor: LP Peripherals PMS exception info record register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_peri_pms_exception_info::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_peri_pms_exception_info`] module"]
pub type LP_PERI_PMS_EXCEPTION_INFO =
    crate::Reg<lp_peri_pms_exception_info::LP_PERI_PMS_EXCEPTION_INFO_SPEC>;
#[doc = "LP Peripherals PMS exception info record register"]
pub mod lp_peri_pms_exception_info;
#[doc = "PERI_PMS_INT_EN (rw) register accessor: APM interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_pms_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_pms_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_pms_int_en`] module"]
pub type PERI_PMS_INT_EN = crate::Reg<peri_pms_int_en::PERI_PMS_INT_EN_SPEC>;
#[doc = "APM interrupt enable register"]
pub mod peri_pms_int_en;
#[doc = "INTERRUPT_SOURCE (r) register accessor: record the lp cpu interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_source::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_source`] module"]
pub type INTERRUPT_SOURCE = crate::Reg<interrupt_source::INTERRUPT_SOURCE_SPEC>;
#[doc = "record the lp cpu interrupt"]
pub mod interrupt_source;
pub use crate::dma::{date, DATE};
