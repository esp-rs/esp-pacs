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
    region4_addr_start: REGION4_ADDR_START,
    region4_addr_end: REGION4_ADDR_END,
    region4_pms_attr: REGION4_PMS_ATTR,
    region5_addr_start: REGION5_ADDR_START,
    region5_addr_end: REGION5_ADDR_END,
    region5_pms_attr: REGION5_PMS_ATTR,
    region6_addr_start: REGION6_ADDR_START,
    region6_addr_end: REGION6_ADDR_END,
    region6_pms_attr: REGION6_PMS_ATTR,
    region7_addr_start: REGION7_ADDR_START,
    region7_addr_end: REGION7_ADDR_END,
    region7_pms_attr: REGION7_PMS_ATTR,
    region8_addr_start: REGION8_ADDR_START,
    region8_addr_end: REGION8_ADDR_END,
    region8_pms_attr: REGION8_PMS_ATTR,
    region9_addr_start: REGION9_ADDR_START,
    region9_addr_end: REGION9_ADDR_END,
    region9_pms_attr: REGION9_PMS_ATTR,
    region10_addr_start: REGION10_ADDR_START,
    region10_addr_end: REGION10_ADDR_END,
    region10_pms_attr: REGION10_PMS_ATTR,
    region11_addr_start: REGION11_ADDR_START,
    region11_addr_end: REGION11_ADDR_END,
    region11_pms_attr: REGION11_PMS_ATTR,
    region12_addr_start: REGION12_ADDR_START,
    region12_addr_end: REGION12_ADDR_END,
    region12_pms_attr: REGION12_PMS_ATTR,
    region13_addr_start: REGION13_ADDR_START,
    region13_addr_end: REGION13_ADDR_END,
    region13_pms_attr: REGION13_PMS_ATTR,
    region14_addr_start: REGION14_ADDR_START,
    region14_addr_end: REGION14_ADDR_END,
    region14_pms_attr: REGION14_PMS_ATTR,
    region15_addr_start: REGION15_ADDR_START,
    region15_addr_end: REGION15_ADDR_END,
    region15_pms_attr: REGION15_PMS_ATTR,
    func_ctrl: FUNC_CTRL,
    m0_status: M0_STATUS,
    m0_status_clr: M0_STATUS_CLR,
    m0_exception_info0: M0_EXCEPTION_INFO0,
    m0_exception_info1: M0_EXCEPTION_INFO1,
    m1_status: M1_STATUS,
    m1_status_clr: M1_STATUS_CLR,
    m1_exception_info0: M1_EXCEPTION_INFO0,
    m1_exception_info1: M1_EXCEPTION_INFO1,
    m2_status: M2_STATUS,
    m2_status_clr: M2_STATUS_CLR,
    m2_exception_info0: M2_EXCEPTION_INFO0,
    m2_exception_info1: M2_EXCEPTION_INFO1,
    m3_status: M3_STATUS,
    m3_status_clr: M3_STATUS_CLR,
    m3_exception_info0: M3_EXCEPTION_INFO0,
    m3_exception_info1: M3_EXCEPTION_INFO1,
    int_en: INT_EN,
    clock_gate: CLOCK_GATE,
    _reserved68: [u8; 0x06ec],
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
    #[doc = "0x34 - Region address register"]
    #[inline(always)]
    pub const fn region4_addr_start(&self) -> &REGION4_ADDR_START {
        &self.region4_addr_start
    }
    #[doc = "0x38 - Region address register"]
    #[inline(always)]
    pub const fn region4_addr_end(&self) -> &REGION4_ADDR_END {
        &self.region4_addr_end
    }
    #[doc = "0x3c - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region4_pms_attr(&self) -> &REGION4_PMS_ATTR {
        &self.region4_pms_attr
    }
    #[doc = "0x40 - Region address register"]
    #[inline(always)]
    pub const fn region5_addr_start(&self) -> &REGION5_ADDR_START {
        &self.region5_addr_start
    }
    #[doc = "0x44 - Region address register"]
    #[inline(always)]
    pub const fn region5_addr_end(&self) -> &REGION5_ADDR_END {
        &self.region5_addr_end
    }
    #[doc = "0x48 - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region5_pms_attr(&self) -> &REGION5_PMS_ATTR {
        &self.region5_pms_attr
    }
    #[doc = "0x4c - Region address register"]
    #[inline(always)]
    pub const fn region6_addr_start(&self) -> &REGION6_ADDR_START {
        &self.region6_addr_start
    }
    #[doc = "0x50 - Region address register"]
    #[inline(always)]
    pub const fn region6_addr_end(&self) -> &REGION6_ADDR_END {
        &self.region6_addr_end
    }
    #[doc = "0x54 - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region6_pms_attr(&self) -> &REGION6_PMS_ATTR {
        &self.region6_pms_attr
    }
    #[doc = "0x58 - Region address register"]
    #[inline(always)]
    pub const fn region7_addr_start(&self) -> &REGION7_ADDR_START {
        &self.region7_addr_start
    }
    #[doc = "0x5c - Region address register"]
    #[inline(always)]
    pub const fn region7_addr_end(&self) -> &REGION7_ADDR_END {
        &self.region7_addr_end
    }
    #[doc = "0x60 - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region7_pms_attr(&self) -> &REGION7_PMS_ATTR {
        &self.region7_pms_attr
    }
    #[doc = "0x64 - Region address register"]
    #[inline(always)]
    pub const fn region8_addr_start(&self) -> &REGION8_ADDR_START {
        &self.region8_addr_start
    }
    #[doc = "0x68 - Region address register"]
    #[inline(always)]
    pub const fn region8_addr_end(&self) -> &REGION8_ADDR_END {
        &self.region8_addr_end
    }
    #[doc = "0x6c - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region8_pms_attr(&self) -> &REGION8_PMS_ATTR {
        &self.region8_pms_attr
    }
    #[doc = "0x70 - Region address register"]
    #[inline(always)]
    pub const fn region9_addr_start(&self) -> &REGION9_ADDR_START {
        &self.region9_addr_start
    }
    #[doc = "0x74 - Region address register"]
    #[inline(always)]
    pub const fn region9_addr_end(&self) -> &REGION9_ADDR_END {
        &self.region9_addr_end
    }
    #[doc = "0x78 - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region9_pms_attr(&self) -> &REGION9_PMS_ATTR {
        &self.region9_pms_attr
    }
    #[doc = "0x7c - Region address register"]
    #[inline(always)]
    pub const fn region10_addr_start(&self) -> &REGION10_ADDR_START {
        &self.region10_addr_start
    }
    #[doc = "0x80 - Region address register"]
    #[inline(always)]
    pub const fn region10_addr_end(&self) -> &REGION10_ADDR_END {
        &self.region10_addr_end
    }
    #[doc = "0x84 - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region10_pms_attr(&self) -> &REGION10_PMS_ATTR {
        &self.region10_pms_attr
    }
    #[doc = "0x88 - Region address register"]
    #[inline(always)]
    pub const fn region11_addr_start(&self) -> &REGION11_ADDR_START {
        &self.region11_addr_start
    }
    #[doc = "0x8c - Region address register"]
    #[inline(always)]
    pub const fn region11_addr_end(&self) -> &REGION11_ADDR_END {
        &self.region11_addr_end
    }
    #[doc = "0x90 - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region11_pms_attr(&self) -> &REGION11_PMS_ATTR {
        &self.region11_pms_attr
    }
    #[doc = "0x94 - Region address register"]
    #[inline(always)]
    pub const fn region12_addr_start(&self) -> &REGION12_ADDR_START {
        &self.region12_addr_start
    }
    #[doc = "0x98 - Region address register"]
    #[inline(always)]
    pub const fn region12_addr_end(&self) -> &REGION12_ADDR_END {
        &self.region12_addr_end
    }
    #[doc = "0x9c - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region12_pms_attr(&self) -> &REGION12_PMS_ATTR {
        &self.region12_pms_attr
    }
    #[doc = "0xa0 - Region address register"]
    #[inline(always)]
    pub const fn region13_addr_start(&self) -> &REGION13_ADDR_START {
        &self.region13_addr_start
    }
    #[doc = "0xa4 - Region address register"]
    #[inline(always)]
    pub const fn region13_addr_end(&self) -> &REGION13_ADDR_END {
        &self.region13_addr_end
    }
    #[doc = "0xa8 - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region13_pms_attr(&self) -> &REGION13_PMS_ATTR {
        &self.region13_pms_attr
    }
    #[doc = "0xac - Region address register"]
    #[inline(always)]
    pub const fn region14_addr_start(&self) -> &REGION14_ADDR_START {
        &self.region14_addr_start
    }
    #[doc = "0xb0 - Region address register"]
    #[inline(always)]
    pub const fn region14_addr_end(&self) -> &REGION14_ADDR_END {
        &self.region14_addr_end
    }
    #[doc = "0xb4 - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region14_pms_attr(&self) -> &REGION14_PMS_ATTR {
        &self.region14_pms_attr
    }
    #[doc = "0xb8 - Region address register"]
    #[inline(always)]
    pub const fn region15_addr_start(&self) -> &REGION15_ADDR_START {
        &self.region15_addr_start
    }
    #[doc = "0xbc - Region address register"]
    #[inline(always)]
    pub const fn region15_addr_end(&self) -> &REGION15_ADDR_END {
        &self.region15_addr_end
    }
    #[doc = "0xc0 - Region access authority attribute register"]
    #[inline(always)]
    pub const fn region15_pms_attr(&self) -> &REGION15_PMS_ATTR {
        &self.region15_pms_attr
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
    #[doc = "0xd8 - M1 status register"]
    #[inline(always)]
    pub const fn m1_status(&self) -> &M1_STATUS {
        &self.m1_status
    }
    #[doc = "0xdc - M1 status clear register"]
    #[inline(always)]
    pub const fn m1_status_clr(&self) -> &M1_STATUS_CLR {
        &self.m1_status_clr
    }
    #[doc = "0xe0 - M1 exception_info0 register"]
    #[inline(always)]
    pub const fn m1_exception_info0(&self) -> &M1_EXCEPTION_INFO0 {
        &self.m1_exception_info0
    }
    #[doc = "0xe4 - M1 exception_info1 register"]
    #[inline(always)]
    pub const fn m1_exception_info1(&self) -> &M1_EXCEPTION_INFO1 {
        &self.m1_exception_info1
    }
    #[doc = "0xe8 - M2 status register"]
    #[inline(always)]
    pub const fn m2_status(&self) -> &M2_STATUS {
        &self.m2_status
    }
    #[doc = "0xec - M2 status clear register"]
    #[inline(always)]
    pub const fn m2_status_clr(&self) -> &M2_STATUS_CLR {
        &self.m2_status_clr
    }
    #[doc = "0xf0 - M2 exception_info0 register"]
    #[inline(always)]
    pub const fn m2_exception_info0(&self) -> &M2_EXCEPTION_INFO0 {
        &self.m2_exception_info0
    }
    #[doc = "0xf4 - M2 exception_info1 register"]
    #[inline(always)]
    pub const fn m2_exception_info1(&self) -> &M2_EXCEPTION_INFO1 {
        &self.m2_exception_info1
    }
    #[doc = "0xf8 - M3 status register"]
    #[inline(always)]
    pub const fn m3_status(&self) -> &M3_STATUS {
        &self.m3_status
    }
    #[doc = "0xfc - M3 status clear register"]
    #[inline(always)]
    pub const fn m3_status_clr(&self) -> &M3_STATUS_CLR {
        &self.m3_status_clr
    }
    #[doc = "0x100 - M3 exception_info0 register"]
    #[inline(always)]
    pub const fn m3_exception_info0(&self) -> &M3_EXCEPTION_INFO0 {
        &self.m3_exception_info0
    }
    #[doc = "0x104 - M3 exception_info1 register"]
    #[inline(always)]
    pub const fn m3_exception_info1(&self) -> &M3_EXCEPTION_INFO1 {
        &self.m3_exception_info1
    }
    #[doc = "0x108 - APM interrupt enable register"]
    #[inline(always)]
    pub const fn int_en(&self) -> &INT_EN {
        &self.int_en
    }
    #[doc = "0x10c - clock gating register"]
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
#[doc = "REGION_FILTER_EN (rw) register accessor: Region filter enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region_filter_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region_filter_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region_filter_en`] module"]
pub type REGION_FILTER_EN = crate::Reg<region_filter_en::REGION_FILTER_EN_SPEC>;
#[doc = "Region filter enable register"]
pub mod region_filter_en;
#[doc = "REGION0_ADDR_START (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region0_addr_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region0_addr_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region0_addr_start`] module"]
pub type REGION0_ADDR_START = crate::Reg<region0_addr_start::REGION0_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region0_addr_start;
#[doc = "REGION0_ADDR_END (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region0_addr_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region0_addr_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region0_addr_end`] module"]
pub type REGION0_ADDR_END = crate::Reg<region0_addr_end::REGION0_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region0_addr_end;
#[doc = "REGION0_PMS_ATTR (rw) register accessor: Region access authority attribute register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region0_pms_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region0_pms_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region0_pms_attr`] module"]
pub type REGION0_PMS_ATTR = crate::Reg<region0_pms_attr::REGION0_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region0_pms_attr;
#[doc = "REGION1_ADDR_START (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region1_addr_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region1_addr_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region1_addr_start`] module"]
pub type REGION1_ADDR_START = crate::Reg<region1_addr_start::REGION1_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region1_addr_start;
#[doc = "REGION1_ADDR_END (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region1_addr_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region1_addr_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region1_addr_end`] module"]
pub type REGION1_ADDR_END = crate::Reg<region1_addr_end::REGION1_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region1_addr_end;
#[doc = "REGION1_PMS_ATTR (rw) register accessor: Region access authority attribute register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region1_pms_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region1_pms_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region1_pms_attr`] module"]
pub type REGION1_PMS_ATTR = crate::Reg<region1_pms_attr::REGION1_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region1_pms_attr;
#[doc = "REGION2_ADDR_START (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region2_addr_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region2_addr_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region2_addr_start`] module"]
pub type REGION2_ADDR_START = crate::Reg<region2_addr_start::REGION2_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region2_addr_start;
#[doc = "REGION2_ADDR_END (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region2_addr_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region2_addr_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region2_addr_end`] module"]
pub type REGION2_ADDR_END = crate::Reg<region2_addr_end::REGION2_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region2_addr_end;
#[doc = "REGION2_PMS_ATTR (rw) register accessor: Region access authority attribute register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region2_pms_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region2_pms_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region2_pms_attr`] module"]
pub type REGION2_PMS_ATTR = crate::Reg<region2_pms_attr::REGION2_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region2_pms_attr;
#[doc = "REGION3_ADDR_START (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region3_addr_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region3_addr_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region3_addr_start`] module"]
pub type REGION3_ADDR_START = crate::Reg<region3_addr_start::REGION3_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region3_addr_start;
#[doc = "REGION3_ADDR_END (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region3_addr_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region3_addr_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region3_addr_end`] module"]
pub type REGION3_ADDR_END = crate::Reg<region3_addr_end::REGION3_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region3_addr_end;
#[doc = "REGION3_PMS_ATTR (rw) register accessor: Region access authority attribute register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region3_pms_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region3_pms_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region3_pms_attr`] module"]
pub type REGION3_PMS_ATTR = crate::Reg<region3_pms_attr::REGION3_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region3_pms_attr;
#[doc = "REGION4_ADDR_START (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region4_addr_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region4_addr_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region4_addr_start`] module"]
pub type REGION4_ADDR_START = crate::Reg<region4_addr_start::REGION4_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region4_addr_start;
#[doc = "REGION4_ADDR_END (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region4_addr_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region4_addr_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region4_addr_end`] module"]
pub type REGION4_ADDR_END = crate::Reg<region4_addr_end::REGION4_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region4_addr_end;
#[doc = "REGION4_PMS_ATTR (rw) register accessor: Region access authority attribute register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region4_pms_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region4_pms_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region4_pms_attr`] module"]
pub type REGION4_PMS_ATTR = crate::Reg<region4_pms_attr::REGION4_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region4_pms_attr;
#[doc = "REGION5_ADDR_START (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region5_addr_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region5_addr_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region5_addr_start`] module"]
pub type REGION5_ADDR_START = crate::Reg<region5_addr_start::REGION5_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region5_addr_start;
#[doc = "REGION5_ADDR_END (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region5_addr_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region5_addr_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region5_addr_end`] module"]
pub type REGION5_ADDR_END = crate::Reg<region5_addr_end::REGION5_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region5_addr_end;
#[doc = "REGION5_PMS_ATTR (rw) register accessor: Region access authority attribute register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region5_pms_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region5_pms_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region5_pms_attr`] module"]
pub type REGION5_PMS_ATTR = crate::Reg<region5_pms_attr::REGION5_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region5_pms_attr;
#[doc = "REGION6_ADDR_START (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region6_addr_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region6_addr_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region6_addr_start`] module"]
pub type REGION6_ADDR_START = crate::Reg<region6_addr_start::REGION6_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region6_addr_start;
#[doc = "REGION6_ADDR_END (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region6_addr_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region6_addr_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region6_addr_end`] module"]
pub type REGION6_ADDR_END = crate::Reg<region6_addr_end::REGION6_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region6_addr_end;
#[doc = "REGION6_PMS_ATTR (rw) register accessor: Region access authority attribute register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region6_pms_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region6_pms_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region6_pms_attr`] module"]
pub type REGION6_PMS_ATTR = crate::Reg<region6_pms_attr::REGION6_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region6_pms_attr;
#[doc = "REGION7_ADDR_START (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region7_addr_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region7_addr_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region7_addr_start`] module"]
pub type REGION7_ADDR_START = crate::Reg<region7_addr_start::REGION7_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region7_addr_start;
#[doc = "REGION7_ADDR_END (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region7_addr_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region7_addr_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region7_addr_end`] module"]
pub type REGION7_ADDR_END = crate::Reg<region7_addr_end::REGION7_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region7_addr_end;
#[doc = "REGION7_PMS_ATTR (rw) register accessor: Region access authority attribute register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region7_pms_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region7_pms_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region7_pms_attr`] module"]
pub type REGION7_PMS_ATTR = crate::Reg<region7_pms_attr::REGION7_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region7_pms_attr;
#[doc = "REGION8_ADDR_START (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region8_addr_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region8_addr_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region8_addr_start`] module"]
pub type REGION8_ADDR_START = crate::Reg<region8_addr_start::REGION8_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region8_addr_start;
#[doc = "REGION8_ADDR_END (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region8_addr_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region8_addr_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region8_addr_end`] module"]
pub type REGION8_ADDR_END = crate::Reg<region8_addr_end::REGION8_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region8_addr_end;
#[doc = "REGION8_PMS_ATTR (rw) register accessor: Region access authority attribute register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region8_pms_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region8_pms_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region8_pms_attr`] module"]
pub type REGION8_PMS_ATTR = crate::Reg<region8_pms_attr::REGION8_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region8_pms_attr;
#[doc = "REGION9_ADDR_START (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region9_addr_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region9_addr_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region9_addr_start`] module"]
pub type REGION9_ADDR_START = crate::Reg<region9_addr_start::REGION9_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region9_addr_start;
#[doc = "REGION9_ADDR_END (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region9_addr_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region9_addr_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region9_addr_end`] module"]
pub type REGION9_ADDR_END = crate::Reg<region9_addr_end::REGION9_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region9_addr_end;
#[doc = "REGION9_PMS_ATTR (rw) register accessor: Region access authority attribute register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region9_pms_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region9_pms_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region9_pms_attr`] module"]
pub type REGION9_PMS_ATTR = crate::Reg<region9_pms_attr::REGION9_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region9_pms_attr;
#[doc = "REGION10_ADDR_START (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region10_addr_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region10_addr_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region10_addr_start`] module"]
pub type REGION10_ADDR_START = crate::Reg<region10_addr_start::REGION10_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region10_addr_start;
#[doc = "REGION10_ADDR_END (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region10_addr_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region10_addr_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region10_addr_end`] module"]
pub type REGION10_ADDR_END = crate::Reg<region10_addr_end::REGION10_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region10_addr_end;
#[doc = "REGION10_PMS_ATTR (rw) register accessor: Region access authority attribute register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region10_pms_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region10_pms_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region10_pms_attr`] module"]
pub type REGION10_PMS_ATTR = crate::Reg<region10_pms_attr::REGION10_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region10_pms_attr;
#[doc = "REGION11_ADDR_START (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region11_addr_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region11_addr_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region11_addr_start`] module"]
pub type REGION11_ADDR_START = crate::Reg<region11_addr_start::REGION11_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region11_addr_start;
#[doc = "REGION11_ADDR_END (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region11_addr_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region11_addr_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region11_addr_end`] module"]
pub type REGION11_ADDR_END = crate::Reg<region11_addr_end::REGION11_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region11_addr_end;
#[doc = "REGION11_PMS_ATTR (rw) register accessor: Region access authority attribute register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region11_pms_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region11_pms_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region11_pms_attr`] module"]
pub type REGION11_PMS_ATTR = crate::Reg<region11_pms_attr::REGION11_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region11_pms_attr;
#[doc = "REGION12_ADDR_START (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region12_addr_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region12_addr_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region12_addr_start`] module"]
pub type REGION12_ADDR_START = crate::Reg<region12_addr_start::REGION12_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region12_addr_start;
#[doc = "REGION12_ADDR_END (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region12_addr_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region12_addr_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region12_addr_end`] module"]
pub type REGION12_ADDR_END = crate::Reg<region12_addr_end::REGION12_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region12_addr_end;
#[doc = "REGION12_PMS_ATTR (rw) register accessor: Region access authority attribute register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region12_pms_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region12_pms_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region12_pms_attr`] module"]
pub type REGION12_PMS_ATTR = crate::Reg<region12_pms_attr::REGION12_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region12_pms_attr;
#[doc = "REGION13_ADDR_START (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region13_addr_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region13_addr_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region13_addr_start`] module"]
pub type REGION13_ADDR_START = crate::Reg<region13_addr_start::REGION13_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region13_addr_start;
#[doc = "REGION13_ADDR_END (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region13_addr_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region13_addr_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region13_addr_end`] module"]
pub type REGION13_ADDR_END = crate::Reg<region13_addr_end::REGION13_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region13_addr_end;
#[doc = "REGION13_PMS_ATTR (rw) register accessor: Region access authority attribute register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region13_pms_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region13_pms_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region13_pms_attr`] module"]
pub type REGION13_PMS_ATTR = crate::Reg<region13_pms_attr::REGION13_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region13_pms_attr;
#[doc = "REGION14_ADDR_START (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region14_addr_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region14_addr_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region14_addr_start`] module"]
pub type REGION14_ADDR_START = crate::Reg<region14_addr_start::REGION14_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region14_addr_start;
#[doc = "REGION14_ADDR_END (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region14_addr_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region14_addr_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region14_addr_end`] module"]
pub type REGION14_ADDR_END = crate::Reg<region14_addr_end::REGION14_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region14_addr_end;
#[doc = "REGION14_PMS_ATTR (rw) register accessor: Region access authority attribute register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region14_pms_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region14_pms_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region14_pms_attr`] module"]
pub type REGION14_PMS_ATTR = crate::Reg<region14_pms_attr::REGION14_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region14_pms_attr;
#[doc = "REGION15_ADDR_START (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region15_addr_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region15_addr_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region15_addr_start`] module"]
pub type REGION15_ADDR_START = crate::Reg<region15_addr_start::REGION15_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region15_addr_start;
#[doc = "REGION15_ADDR_END (rw) register accessor: Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region15_addr_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region15_addr_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region15_addr_end`] module"]
pub type REGION15_ADDR_END = crate::Reg<region15_addr_end::REGION15_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region15_addr_end;
#[doc = "REGION15_PMS_ATTR (rw) register accessor: Region access authority attribute register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region15_pms_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region15_pms_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region15_pms_attr`] module"]
pub type REGION15_PMS_ATTR = crate::Reg<region15_pms_attr::REGION15_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region15_pms_attr;
#[doc = "FUNC_CTRL (rw) register accessor: PMS function control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_ctrl`] module"]
pub type FUNC_CTRL = crate::Reg<func_ctrl::FUNC_CTRL_SPEC>;
#[doc = "PMS function control register"]
pub mod func_ctrl;
#[doc = "M0_STATUS (r) register accessor: M0 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m0_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m0_status`] module"]
pub type M0_STATUS = crate::Reg<m0_status::M0_STATUS_SPEC>;
#[doc = "M0 status register"]
pub mod m0_status;
#[doc = "M0_STATUS_CLR (w) register accessor: M0 status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m0_status_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m0_status_clr`] module"]
pub type M0_STATUS_CLR = crate::Reg<m0_status_clr::M0_STATUS_CLR_SPEC>;
#[doc = "M0 status clear register"]
pub mod m0_status_clr;
#[doc = "M0_EXCEPTION_INFO0 (r) register accessor: M0 exception_info0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m0_exception_info0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m0_exception_info0`] module"]
pub type M0_EXCEPTION_INFO0 = crate::Reg<m0_exception_info0::M0_EXCEPTION_INFO0_SPEC>;
#[doc = "M0 exception_info0 register"]
pub mod m0_exception_info0;
#[doc = "M0_EXCEPTION_INFO1 (r) register accessor: M0 exception_info1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m0_exception_info1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m0_exception_info1`] module"]
pub type M0_EXCEPTION_INFO1 = crate::Reg<m0_exception_info1::M0_EXCEPTION_INFO1_SPEC>;
#[doc = "M0 exception_info1 register"]
pub mod m0_exception_info1;
#[doc = "M1_STATUS (r) register accessor: M1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1_status`] module"]
pub type M1_STATUS = crate::Reg<m1_status::M1_STATUS_SPEC>;
#[doc = "M1 status register"]
pub mod m1_status;
#[doc = "M1_STATUS_CLR (w) register accessor: M1 status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m1_status_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1_status_clr`] module"]
pub type M1_STATUS_CLR = crate::Reg<m1_status_clr::M1_STATUS_CLR_SPEC>;
#[doc = "M1 status clear register"]
pub mod m1_status_clr;
#[doc = "M1_EXCEPTION_INFO0 (r) register accessor: M1 exception_info0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1_exception_info0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1_exception_info0`] module"]
pub type M1_EXCEPTION_INFO0 = crate::Reg<m1_exception_info0::M1_EXCEPTION_INFO0_SPEC>;
#[doc = "M1 exception_info0 register"]
pub mod m1_exception_info0;
#[doc = "M1_EXCEPTION_INFO1 (r) register accessor: M1 exception_info1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1_exception_info1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1_exception_info1`] module"]
pub type M1_EXCEPTION_INFO1 = crate::Reg<m1_exception_info1::M1_EXCEPTION_INFO1_SPEC>;
#[doc = "M1 exception_info1 register"]
pub mod m1_exception_info1;
#[doc = "M2_STATUS (r) register accessor: M2 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2_status`] module"]
pub type M2_STATUS = crate::Reg<m2_status::M2_STATUS_SPEC>;
#[doc = "M2 status register"]
pub mod m2_status;
#[doc = "M2_STATUS_CLR (w) register accessor: M2 status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m2_status_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2_status_clr`] module"]
pub type M2_STATUS_CLR = crate::Reg<m2_status_clr::M2_STATUS_CLR_SPEC>;
#[doc = "M2 status clear register"]
pub mod m2_status_clr;
#[doc = "M2_EXCEPTION_INFO0 (r) register accessor: M2 exception_info0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2_exception_info0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2_exception_info0`] module"]
pub type M2_EXCEPTION_INFO0 = crate::Reg<m2_exception_info0::M2_EXCEPTION_INFO0_SPEC>;
#[doc = "M2 exception_info0 register"]
pub mod m2_exception_info0;
#[doc = "M2_EXCEPTION_INFO1 (r) register accessor: M2 exception_info1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2_exception_info1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2_exception_info1`] module"]
pub type M2_EXCEPTION_INFO1 = crate::Reg<m2_exception_info1::M2_EXCEPTION_INFO1_SPEC>;
#[doc = "M2 exception_info1 register"]
pub mod m2_exception_info1;
#[doc = "M3_STATUS (r) register accessor: M3 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3_status`] module"]
pub type M3_STATUS = crate::Reg<m3_status::M3_STATUS_SPEC>;
#[doc = "M3 status register"]
pub mod m3_status;
#[doc = "M3_STATUS_CLR (w) register accessor: M3 status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m3_status_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3_status_clr`] module"]
pub type M3_STATUS_CLR = crate::Reg<m3_status_clr::M3_STATUS_CLR_SPEC>;
#[doc = "M3 status clear register"]
pub mod m3_status_clr;
#[doc = "M3_EXCEPTION_INFO0 (r) register accessor: M3 exception_info0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3_exception_info0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3_exception_info0`] module"]
pub type M3_EXCEPTION_INFO0 = crate::Reg<m3_exception_info0::M3_EXCEPTION_INFO0_SPEC>;
#[doc = "M3 exception_info0 register"]
pub mod m3_exception_info0;
#[doc = "M3_EXCEPTION_INFO1 (r) register accessor: M3 exception_info1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3_exception_info1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3_exception_info1`] module"]
pub type M3_EXCEPTION_INFO1 = crate::Reg<m3_exception_info1::M3_EXCEPTION_INFO1_SPEC>;
#[doc = "M3 exception_info1 register"]
pub mod m3_exception_info1;
#[doc = "INT_EN (rw) register accessor: APM interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_en`] module"]
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
#[doc = "APM interrupt enable register"]
pub mod int_en;
#[doc = "CLOCK_GATE (rw) register accessor: clock gating register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "clock gating register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: Version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version register"]
pub mod date;
