#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    region_filter_en: REGION_FILTER_EN,
    region0_addr_start: REGION0_ADDR_START,
    region0_addr_end: REGION0_ADDR_END,
    region0_pms_attr: REGION0_PMS_ATTR,
    region1_addr_start: REGION1_ADDR_START,
    region1_addr_end: REGION1_ADDR_END,
    region1_pms_attr: REGION1_PMS_ATTR,
    region2_addr_start: REGION2_ADDR_START,
    region2_addr_end: REGION2_ADDR_END,
    region2_pms_attr: REGION2_PMS_ATTR,
    region3_addr_start: REGION3_ADDR_START,
    region3_addr_end: REGION3_ADDR_END,
    region3_pms_attr: REGION3_PMS_ATTR,
    _reserved13: [u8; 0x90],
    func_ctrl: FUNC_CTRL,
    m0_status: M0_STATUS,
    m0_status_clr: M0_STATUS_CLR,
    m0_exception_info0: M0_EXCEPTION_INFO0,
    m0_exception_info1: M0_EXCEPTION_INFO1,
    int_en: INT_EN,
    clock_gate: CLOCK_GATE,
    _reserved20: [u8; 0x071c],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - Region filter enable register"]
    #[inline(always)]
    pub const fn region_filter_en(&self) -> &REGION_FILTER_EN {
        &self.region_filter_en
    }
    #[doc = "0x04 - Region address register"]
    #[inline(always)]
    pub const fn region0_addr_start(&self) -> &REGION0_ADDR_START {
        &self.region0_addr_start
    }
    #[doc = "0x08 - Region address register"]
    #[inline(always)]
    pub const fn region0_addr_end(&self) -> &REGION0_ADDR_END {
        &self.region0_addr_end
    }
    #[doc = "0x0c - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region0_pms_attr(&self) -> &REGION0_PMS_ATTR {
        &self.region0_pms_attr
    }
    #[doc = "0x10 - Region address register"]
    #[inline(always)]
    pub const fn region1_addr_start(&self) -> &REGION1_ADDR_START {
        &self.region1_addr_start
    }
    #[doc = "0x14 - Region address register"]
    #[inline(always)]
    pub const fn region1_addr_end(&self) -> &REGION1_ADDR_END {
        &self.region1_addr_end
    }
    #[doc = "0x18 - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region1_pms_attr(&self) -> &REGION1_PMS_ATTR {
        &self.region1_pms_attr
    }
    #[doc = "0x1c - Region address register"]
    #[inline(always)]
    pub const fn region2_addr_start(&self) -> &REGION2_ADDR_START {
        &self.region2_addr_start
    }
    #[doc = "0x20 - Region address register"]
    #[inline(always)]
    pub const fn region2_addr_end(&self) -> &REGION2_ADDR_END {
        &self.region2_addr_end
    }
    #[doc = "0x24 - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region2_pms_attr(&self) -> &REGION2_PMS_ATTR {
        &self.region2_pms_attr
    }
    #[doc = "0x28 - Region address register"]
    #[inline(always)]
    pub const fn region3_addr_start(&self) -> &REGION3_ADDR_START {
        &self.region3_addr_start
    }
    #[doc = "0x2c - Region address register"]
    #[inline(always)]
    pub const fn region3_addr_end(&self) -> &REGION3_ADDR_END {
        &self.region3_addr_end
    }
    #[doc = "0x30 - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region3_pms_attr(&self) -> &REGION3_PMS_ATTR {
        &self.region3_pms_attr
    }
    #[doc = "0xc4 - PMS function control register"]
    #[inline(always)]
    pub const fn func_ctrl(&self) -> &FUNC_CTRL {
        &self.func_ctrl
    }
    #[doc = "0xc8 - M0 status register"]
    #[inline(always)]
    pub const fn m0_status(&self) -> &M0_STATUS {
        &self.m0_status
    }
    #[doc = "0xcc - M0 status clear register"]
    #[inline(always)]
    pub const fn m0_status_clr(&self) -> &M0_STATUS_CLR {
        &self.m0_status_clr
    }
    #[doc = "0xd0 - M0 exception_info0 register"]
    #[inline(always)]
    pub const fn m0_exception_info0(&self) -> &M0_EXCEPTION_INFO0 {
        &self.m0_exception_info0
    }
    #[doc = "0xd4 - M0 exception_info1 register"]
    #[inline(always)]
    pub const fn m0_exception_info1(&self) -> &M0_EXCEPTION_INFO1 {
        &self.m0_exception_info1
    }
    #[doc = "0xd8 - APM interrupt enable register"]
    #[inline(always)]
    pub const fn int_en(&self) -> &INT_EN {
        &self.int_en
    }
    #[doc = "0xdc - clock gating register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x7fc - Version register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "REGION_FILTER_EN (rw) register accessor: Region filter enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_filter_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_filter_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region_filter_en`] module"]
pub type REGION_FILTER_EN = crate::Reg<region_filter_en::REGION_FILTER_EN_SPEC>;
#[doc = "Region filter enable register"]
pub mod region_filter_en;
#[doc = "REGION0_ADDR_START (rw) register accessor: Region address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region0_addr_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region0_addr_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region0_addr_start`] module"]
pub type REGION0_ADDR_START = crate::Reg<region0_addr_start::REGION0_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region0_addr_start;
#[doc = "REGION0_ADDR_END (rw) register accessor: Region address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region0_addr_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region0_addr_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region0_addr_end`] module"]
pub type REGION0_ADDR_END = crate::Reg<region0_addr_end::REGION0_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region0_addr_end;
#[doc = "REGION0_PMS_ATTR (rw) register accessor: Region access authority attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`region0_pms_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region0_pms_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region0_pms_attr`] module"]
pub type REGION0_PMS_ATTR = crate::Reg<region0_pms_attr::REGION0_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region0_pms_attr;
#[doc = "REGION1_ADDR_START (rw) register accessor: Region address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region1_addr_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region1_addr_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region1_addr_start`] module"]
pub type REGION1_ADDR_START = crate::Reg<region1_addr_start::REGION1_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region1_addr_start;
#[doc = "REGION1_ADDR_END (rw) register accessor: Region address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region1_addr_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region1_addr_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region1_addr_end`] module"]
pub type REGION1_ADDR_END = crate::Reg<region1_addr_end::REGION1_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region1_addr_end;
#[doc = "REGION1_PMS_ATTR (rw) register accessor: Region access authority attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`region1_pms_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region1_pms_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region1_pms_attr`] module"]
pub type REGION1_PMS_ATTR = crate::Reg<region1_pms_attr::REGION1_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region1_pms_attr;
#[doc = "REGION2_ADDR_START (rw) register accessor: Region address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region2_addr_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region2_addr_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region2_addr_start`] module"]
pub type REGION2_ADDR_START = crate::Reg<region2_addr_start::REGION2_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region2_addr_start;
#[doc = "REGION2_ADDR_END (rw) register accessor: Region address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region2_addr_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region2_addr_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region2_addr_end`] module"]
pub type REGION2_ADDR_END = crate::Reg<region2_addr_end::REGION2_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region2_addr_end;
#[doc = "REGION2_PMS_ATTR (rw) register accessor: Region access authority attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`region2_pms_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region2_pms_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region2_pms_attr`] module"]
pub type REGION2_PMS_ATTR = crate::Reg<region2_pms_attr::REGION2_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region2_pms_attr;
#[doc = "REGION3_ADDR_START (rw) register accessor: Region address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region3_addr_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region3_addr_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region3_addr_start`] module"]
pub type REGION3_ADDR_START = crate::Reg<region3_addr_start::REGION3_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region3_addr_start;
#[doc = "REGION3_ADDR_END (rw) register accessor: Region address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region3_addr_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region3_addr_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region3_addr_end`] module"]
pub type REGION3_ADDR_END = crate::Reg<region3_addr_end::REGION3_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region3_addr_end;
#[doc = "REGION3_PMS_ATTR (rw) register accessor: Region access authority attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`region3_pms_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region3_pms_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region3_pms_attr`] module"]
pub type REGION3_PMS_ATTR = crate::Reg<region3_pms_attr::REGION3_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region3_pms_attr;
#[doc = "FUNC_CTRL (rw) register accessor: PMS function control register\n\nYou can [`read`](crate::Reg::read) this register and get [`func_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_ctrl`] module"]
pub type FUNC_CTRL = crate::Reg<func_ctrl::FUNC_CTRL_SPEC>;
#[doc = "PMS function control register"]
pub mod func_ctrl;
#[doc = "M0_STATUS (r) register accessor: M0 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`m0_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m0_status`] module"]
pub type M0_STATUS = crate::Reg<m0_status::M0_STATUS_SPEC>;
#[doc = "M0 status register"]
pub mod m0_status;
#[doc = "M0_STATUS_CLR (w) register accessor: M0 status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m0_status_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m0_status_clr`] module"]
pub type M0_STATUS_CLR = crate::Reg<m0_status_clr::M0_STATUS_CLR_SPEC>;
#[doc = "M0 status clear register"]
pub mod m0_status_clr;
#[doc = "M0_EXCEPTION_INFO0 (r) register accessor: M0 exception_info0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`m0_exception_info0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m0_exception_info0`] module"]
pub type M0_EXCEPTION_INFO0 = crate::Reg<m0_exception_info0::M0_EXCEPTION_INFO0_SPEC>;
#[doc = "M0 exception_info0 register"]
pub mod m0_exception_info0;
#[doc = "M0_EXCEPTION_INFO1 (r) register accessor: M0 exception_info1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`m0_exception_info1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m0_exception_info1`] module"]
pub type M0_EXCEPTION_INFO1 = crate::Reg<m0_exception_info1::M0_EXCEPTION_INFO1_SPEC>;
#[doc = "M0 exception_info1 register"]
pub mod m0_exception_info1;
#[doc = "INT_EN (rw) register accessor: APM interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_en`] module"]
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
#[doc = "APM interrupt enable register"]
pub mod int_en;
#[doc = "CLOCK_GATE (rw) register accessor: clock gating register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "clock gating register"]
pub mod clock_gate;
pub use crate::dma::date;
pub use crate::dma::DATE;
