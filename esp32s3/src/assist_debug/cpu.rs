#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster CPU%s, containing CORE_?_AREA_DRAM0_0_MAX, CORE_?_AREA_DRAM0_0_MIN, CORE_?_AREA_DRAM0_1_MAX, CORE_?_AREA_DRAM0_1_MIN, CORE_?_AREA_PC, CORE_?_AREA_PIF_0_MAX, CORE_?_AREA_PIF_0_MIN, CORE_?_AREA_PIF_1_MAX, CORE_?_AREA_PIF_1_MIN, CORE_?_AREA_SP, CORE_?_DRAM0_EXCEPTION_MONITOR_0, CORE_?_DRAM0_EXCEPTION_MONITOR_1, CORE_?_DRAM0_EXCEPTION_MONITOR_2, CORE_?_DRAM0_EXCEPTION_MONITOR_3, CORE_?_DRAM0_EXCEPTION_MONITOR_4, CORE_?_DRAM0_EXCEPTION_MONITOR_5, CORE_?_INTR_CLR, CORE_?_INTR_ENA, CORE_?_INTR_RAW, CORE_?_IRAM0_EXCEPTION_MONITOR_0, CORE_?_IRAM0_EXCEPTION_MONITOR_1, CORE_?_MONTR_ENA, CORE_?_RCD_PDEBUGDATA, CORE_?_RCD_PDEBUGENABLE, CORE_?_RCD_PDEBUGINST, CORE_?_RCD_PDEBUGLS0ADDR, CORE_?_RCD_PDEBUGLS0DATA, CORE_?_RCD_PDEBUGLS0STAT, CORE_?_RCD_PDEBUGPC, CORE_?_RCD_PDEBUGSTATUS, CORE_?_RCD_RECORDING, CORE_?_RCD_SP, CORE_?_SP_MAX, CORE_?_SP_MIN, CORE_?_SP_PC, CORE_?_SP_UNSTABLE"]
pub struct CPU {
    montr_ena: MONTR_ENA,
    intr_raw: INTR_RAW,
    intr_ena: INTR_ENA,
    intr_clr: INTR_CLR,
    area_dram0_0_min: AREA_DRAM0_0_MIN,
    area_dram0_0_max: AREA_DRAM0_0_MAX,
    area_dram0_1_min: AREA_DRAM0_1_MIN,
    area_dram0_1_max: AREA_DRAM0_1_MAX,
    area_pif_0_min: AREA_PIF_0_MIN,
    area_pif_0_max: AREA_PIF_0_MAX,
    area_pif_1_min: AREA_PIF_1_MIN,
    area_pif_1_max: AREA_PIF_1_MAX,
    area_sp: AREA_SP,
    area_pc: AREA_PC,
    sp_unstable: SP_UNSTABLE,
    sp_min: SP_MIN,
    sp_max: SP_MAX,
    sp_pc: SP_PC,
    rcd_pdebugenable: RCD_PDEBUGENABLE,
    rcd_recording: RCD_RECORDING,
    rcd_pdebuginst: RCD_PDEBUGINST,
    rcd_pdebugstatus: RCD_PDEBUGSTATUS,
    rcd_pdebugdata: RCD_PDEBUGDATA,
    rcd_pdebugpc: RCD_PDEBUGPC,
    rcd_pdebugls0stat: RCD_PDEBUGLS0STAT,
    rcd_pdebugls0addr: RCD_PDEBUGLS0ADDR,
    rcd_pdebugls0data: RCD_PDEBUGLS0DATA,
    rcd_sp: RCD_SP,
    iram0_exception_monitor_0: IRAM0_EXCEPTION_MONITOR_0,
    iram0_exception_monitor_1: IRAM0_EXCEPTION_MONITOR_1,
    dram0_exception_monitor_0: DRAM0_EXCEPTION_MONITOR_0,
    dram0_exception_monitor_1: DRAM0_EXCEPTION_MONITOR_1,
    dram0_exception_monitor_2: DRAM0_EXCEPTION_MONITOR_2,
    dram0_exception_monitor_3: DRAM0_EXCEPTION_MONITOR_3,
    dram0_exception_monitor_4: DRAM0_EXCEPTION_MONITOR_4,
    dram0_exception_monitor_5: DRAM0_EXCEPTION_MONITOR_5,
}
impl CPU {
    #[doc = "0x00 - core0 monitor enable configuration register"]
    #[inline(always)]
    pub const fn montr_ena(&self) -> &MONTR_ENA {
        &self.montr_ena
    }
    #[doc = "0x04 - core0 monitor interrupt status register"]
    #[inline(always)]
    pub const fn intr_raw(&self) -> &INTR_RAW {
        &self.intr_raw
    }
    #[doc = "0x08 - core0 monitor interrupt enable register"]
    #[inline(always)]
    pub const fn intr_ena(&self) -> &INTR_ENA {
        &self.intr_ena
    }
    #[doc = "0x0c - core0 monitor interrupt clr register"]
    #[inline(always)]
    pub const fn intr_clr(&self) -> &INTR_CLR {
        &self.intr_clr
    }
    #[doc = "0x10 - core0 dram0 region0 addr configuration register"]
    #[inline(always)]
    pub const fn area_dram0_0_min(&self) -> &AREA_DRAM0_0_MIN {
        &self.area_dram0_0_min
    }
    #[doc = "0x14 - core0 dram0 region0 addr configuration register"]
    #[inline(always)]
    pub const fn area_dram0_0_max(&self) -> &AREA_DRAM0_0_MAX {
        &self.area_dram0_0_max
    }
    #[doc = "0x18 - core0 dram0 region1 addr configuration register"]
    #[inline(always)]
    pub const fn area_dram0_1_min(&self) -> &AREA_DRAM0_1_MIN {
        &self.area_dram0_1_min
    }
    #[doc = "0x1c - core0 dram0 region1 addr configuration register"]
    #[inline(always)]
    pub const fn area_dram0_1_max(&self) -> &AREA_DRAM0_1_MAX {
        &self.area_dram0_1_max
    }
    #[doc = "0x20 - core0 PIF region0 addr configuration register"]
    #[inline(always)]
    pub const fn area_pif_0_min(&self) -> &AREA_PIF_0_MIN {
        &self.area_pif_0_min
    }
    #[doc = "0x24 - core0 PIF region0 addr configuration register"]
    #[inline(always)]
    pub const fn area_pif_0_max(&self) -> &AREA_PIF_0_MAX {
        &self.area_pif_0_max
    }
    #[doc = "0x28 - core0 PIF region1 addr configuration register"]
    #[inline(always)]
    pub const fn area_pif_1_min(&self) -> &AREA_PIF_1_MIN {
        &self.area_pif_1_min
    }
    #[doc = "0x2c - core0 PIF region1 addr configuration register"]
    #[inline(always)]
    pub const fn area_pif_1_max(&self) -> &AREA_PIF_1_MAX {
        &self.area_pif_1_max
    }
    #[doc = "0x30 - core0 area sp status register"]
    #[inline(always)]
    pub const fn area_sp(&self) -> &AREA_SP {
        &self.area_sp
    }
    #[doc = "0x34 - core0 area pc status register"]
    #[inline(always)]
    pub const fn area_pc(&self) -> &AREA_PC {
        &self.area_pc
    }
    #[doc = "0x38 - core0 sp unstable configuration register"]
    #[inline(always)]
    pub const fn sp_unstable(&self) -> &SP_UNSTABLE {
        &self.sp_unstable
    }
    #[doc = "0x3c - core0 sp region configuration regsiter"]
    #[inline(always)]
    pub const fn sp_min(&self) -> &SP_MIN {
        &self.sp_min
    }
    #[doc = "0x40 - core0 sp region configuration regsiter"]
    #[inline(always)]
    pub const fn sp_max(&self) -> &SP_MAX {
        &self.sp_max
    }
    #[doc = "0x44 - core0 sp pc status register"]
    #[inline(always)]
    pub const fn sp_pc(&self) -> &SP_PC {
        &self.sp_pc
    }
    #[doc = "0x48 - core0 pdebug configuration register"]
    #[inline(always)]
    pub const fn rcd_pdebugenable(&self) -> &RCD_PDEBUGENABLE {
        &self.rcd_pdebugenable
    }
    #[doc = "0x4c - core0 pdebug status register"]
    #[inline(always)]
    pub const fn rcd_recording(&self) -> &RCD_RECORDING {
        &self.rcd_recording
    }
    #[doc = "0x50 - core0 pdebug status register"]
    #[inline(always)]
    pub const fn rcd_pdebuginst(&self) -> &RCD_PDEBUGINST {
        &self.rcd_pdebuginst
    }
    #[doc = "0x54 - core0 pdebug status register"]
    #[inline(always)]
    pub const fn rcd_pdebugstatus(&self) -> &RCD_PDEBUGSTATUS {
        &self.rcd_pdebugstatus
    }
    #[doc = "0x58 - core0 pdebug status register"]
    #[inline(always)]
    pub const fn rcd_pdebugdata(&self) -> &RCD_PDEBUGDATA {
        &self.rcd_pdebugdata
    }
    #[doc = "0x5c - core0 pdebug status register"]
    #[inline(always)]
    pub const fn rcd_pdebugpc(&self) -> &RCD_PDEBUGPC {
        &self.rcd_pdebugpc
    }
    #[doc = "0x60 - core0 pdebug status register"]
    #[inline(always)]
    pub const fn rcd_pdebugls0stat(&self) -> &RCD_PDEBUGLS0STAT {
        &self.rcd_pdebugls0stat
    }
    #[doc = "0x64 - core0 pdebug status register"]
    #[inline(always)]
    pub const fn rcd_pdebugls0addr(&self) -> &RCD_PDEBUGLS0ADDR {
        &self.rcd_pdebugls0addr
    }
    #[doc = "0x68 - core0 pdebug status register"]
    #[inline(always)]
    pub const fn rcd_pdebugls0data(&self) -> &RCD_PDEBUGLS0DATA {
        &self.rcd_pdebugls0data
    }
    #[doc = "0x6c - core0 pdebug status register"]
    #[inline(always)]
    pub const fn rcd_sp(&self) -> &RCD_SP {
        &self.rcd_sp
    }
    #[doc = "0x70 - core0 bus busy status regsiter"]
    #[inline(always)]
    pub const fn iram0_exception_monitor_0(&self) -> &IRAM0_EXCEPTION_MONITOR_0 {
        &self.iram0_exception_monitor_0
    }
    #[doc = "0x74 - core0 bus busy status regsiter"]
    #[inline(always)]
    pub const fn iram0_exception_monitor_1(&self) -> &IRAM0_EXCEPTION_MONITOR_1 {
        &self.iram0_exception_monitor_1
    }
    #[doc = "0x78 - core0 bus busy status regsiter"]
    #[inline(always)]
    pub const fn dram0_exception_monitor_0(&self) -> &DRAM0_EXCEPTION_MONITOR_0 {
        &self.dram0_exception_monitor_0
    }
    #[doc = "0x7c - core0 bus busy status regsiter"]
    #[inline(always)]
    pub const fn dram0_exception_monitor_1(&self) -> &DRAM0_EXCEPTION_MONITOR_1 {
        &self.dram0_exception_monitor_1
    }
    #[doc = "0x80 - core0 bus busy status regsiter"]
    #[inline(always)]
    pub const fn dram0_exception_monitor_2(&self) -> &DRAM0_EXCEPTION_MONITOR_2 {
        &self.dram0_exception_monitor_2
    }
    #[doc = "0x84 - core0 bus busy status regsiter"]
    #[inline(always)]
    pub const fn dram0_exception_monitor_3(&self) -> &DRAM0_EXCEPTION_MONITOR_3 {
        &self.dram0_exception_monitor_3
    }
    #[doc = "0x88 - core0 bus busy configuration regsiter"]
    #[inline(always)]
    pub const fn dram0_exception_monitor_4(&self) -> &DRAM0_EXCEPTION_MONITOR_4 {
        &self.dram0_exception_monitor_4
    }
    #[doc = "0x8c - core0 bus busy configuration regsiter"]
    #[inline(always)]
    pub const fn dram0_exception_monitor_5(&self) -> &DRAM0_EXCEPTION_MONITOR_5 {
        &self.dram0_exception_monitor_5
    }
}
#[doc = "AREA_DRAM0_0_MAX (rw) register accessor: core0 dram0 region0 addr configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`area_dram0_0_max::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`area_dram0_0_max::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@area_dram0_0_max`] module"]
pub type AREA_DRAM0_0_MAX = crate::Reg<area_dram0_0_max::AREA_DRAM0_0_MAX_SPEC>;
#[doc = "core0 dram0 region0 addr configuration register"]
pub mod area_dram0_0_max;
#[doc = "AREA_DRAM0_0_MIN (rw) register accessor: core0 dram0 region0 addr configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`area_dram0_0_min::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`area_dram0_0_min::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@area_dram0_0_min`] module"]
pub type AREA_DRAM0_0_MIN = crate::Reg<area_dram0_0_min::AREA_DRAM0_0_MIN_SPEC>;
#[doc = "core0 dram0 region0 addr configuration register"]
pub mod area_dram0_0_min;
#[doc = "AREA_DRAM0_1_MAX (rw) register accessor: core0 dram0 region1 addr configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`area_dram0_1_max::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`area_dram0_1_max::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@area_dram0_1_max`] module"]
pub type AREA_DRAM0_1_MAX = crate::Reg<area_dram0_1_max::AREA_DRAM0_1_MAX_SPEC>;
#[doc = "core0 dram0 region1 addr configuration register"]
pub mod area_dram0_1_max;
#[doc = "AREA_DRAM0_1_MIN (rw) register accessor: core0 dram0 region1 addr configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`area_dram0_1_min::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`area_dram0_1_min::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@area_dram0_1_min`] module"]
pub type AREA_DRAM0_1_MIN = crate::Reg<area_dram0_1_min::AREA_DRAM0_1_MIN_SPEC>;
#[doc = "core0 dram0 region1 addr configuration register"]
pub mod area_dram0_1_min;
#[doc = "AREA_PC (r) register accessor: core0 area pc status register\n\nYou can [`read`](crate::Reg::read) this register and get [`area_pc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@area_pc`] module"]
pub type AREA_PC = crate::Reg<area_pc::AREA_PC_SPEC>;
#[doc = "core0 area pc status register"]
pub mod area_pc;
#[doc = "AREA_PIF_0_MAX (rw) register accessor: core0 PIF region0 addr configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`area_pif_0_max::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`area_pif_0_max::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@area_pif_0_max`] module"]
pub type AREA_PIF_0_MAX = crate::Reg<area_pif_0_max::AREA_PIF_0_MAX_SPEC>;
#[doc = "core0 PIF region0 addr configuration register"]
pub mod area_pif_0_max;
#[doc = "AREA_PIF_0_MIN (rw) register accessor: core0 PIF region0 addr configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`area_pif_0_min::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`area_pif_0_min::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@area_pif_0_min`] module"]
pub type AREA_PIF_0_MIN = crate::Reg<area_pif_0_min::AREA_PIF_0_MIN_SPEC>;
#[doc = "core0 PIF region0 addr configuration register"]
pub mod area_pif_0_min;
#[doc = "AREA_PIF_1_MAX (rw) register accessor: core0 PIF region1 addr configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`area_pif_1_max::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`area_pif_1_max::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@area_pif_1_max`] module"]
pub type AREA_PIF_1_MAX = crate::Reg<area_pif_1_max::AREA_PIF_1_MAX_SPEC>;
#[doc = "core0 PIF region1 addr configuration register"]
pub mod area_pif_1_max;
#[doc = "AREA_PIF_1_MIN (rw) register accessor: core0 PIF region1 addr configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`area_pif_1_min::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`area_pif_1_min::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@area_pif_1_min`] module"]
pub type AREA_PIF_1_MIN = crate::Reg<area_pif_1_min::AREA_PIF_1_MIN_SPEC>;
#[doc = "core0 PIF region1 addr configuration register"]
pub mod area_pif_1_min;
#[doc = "AREA_SP (r) register accessor: core0 area sp status register\n\nYou can [`read`](crate::Reg::read) this register and get [`area_sp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@area_sp`] module"]
pub type AREA_SP = crate::Reg<area_sp::AREA_SP_SPEC>;
#[doc = "core0 area sp status register"]
pub mod area_sp;
#[doc = "DRAM0_EXCEPTION_MONITOR_0 (r) register accessor: core0 bus busy status regsiter\n\nYou can [`read`](crate::Reg::read) this register and get [`dram0_exception_monitor_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dram0_exception_monitor_0`] module"]
pub type DRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<dram0_exception_monitor_0::DRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "core0 bus busy status regsiter"]
pub mod dram0_exception_monitor_0;
#[doc = "DRAM0_EXCEPTION_MONITOR_1 (r) register accessor: core0 bus busy status regsiter\n\nYou can [`read`](crate::Reg::read) this register and get [`dram0_exception_monitor_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dram0_exception_monitor_1`] module"]
pub type DRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<dram0_exception_monitor_1::DRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "core0 bus busy status regsiter"]
pub mod dram0_exception_monitor_1;
#[doc = "DRAM0_EXCEPTION_MONITOR_2 (r) register accessor: core0 bus busy status regsiter\n\nYou can [`read`](crate::Reg::read) this register and get [`dram0_exception_monitor_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dram0_exception_monitor_2`] module"]
pub type DRAM0_EXCEPTION_MONITOR_2 =
    crate::Reg<dram0_exception_monitor_2::DRAM0_EXCEPTION_MONITOR_2_SPEC>;
#[doc = "core0 bus busy status regsiter"]
pub mod dram0_exception_monitor_2;
#[doc = "DRAM0_EXCEPTION_MONITOR_3 (r) register accessor: core0 bus busy status regsiter\n\nYou can [`read`](crate::Reg::read) this register and get [`dram0_exception_monitor_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dram0_exception_monitor_3`] module"]
pub type DRAM0_EXCEPTION_MONITOR_3 =
    crate::Reg<dram0_exception_monitor_3::DRAM0_EXCEPTION_MONITOR_3_SPEC>;
#[doc = "core0 bus busy status regsiter"]
pub mod dram0_exception_monitor_3;
#[doc = "DRAM0_EXCEPTION_MONITOR_4 (r) register accessor: core0 bus busy configuration regsiter\n\nYou can [`read`](crate::Reg::read) this register and get [`dram0_exception_monitor_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dram0_exception_monitor_4`] module"]
pub type DRAM0_EXCEPTION_MONITOR_4 =
    crate::Reg<dram0_exception_monitor_4::DRAM0_EXCEPTION_MONITOR_4_SPEC>;
#[doc = "core0 bus busy configuration regsiter"]
pub mod dram0_exception_monitor_4;
#[doc = "DRAM0_EXCEPTION_MONITOR_5 (r) register accessor: core0 bus busy configuration regsiter\n\nYou can [`read`](crate::Reg::read) this register and get [`dram0_exception_monitor_5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dram0_exception_monitor_5`] module"]
pub type DRAM0_EXCEPTION_MONITOR_5 =
    crate::Reg<dram0_exception_monitor_5::DRAM0_EXCEPTION_MONITOR_5_SPEC>;
#[doc = "core0 bus busy configuration regsiter"]
pub mod dram0_exception_monitor_5;
#[doc = "INTR_CLR (rw) register accessor: core0 monitor interrupt clr register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_clr`] module"]
pub type INTR_CLR = crate::Reg<intr_clr::INTR_CLR_SPEC>;
#[doc = "core0 monitor interrupt clr register"]
pub mod intr_clr;
#[doc = "INTR_ENA (rw) register accessor: core0 monitor interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_ena`] module"]
pub type INTR_ENA = crate::Reg<intr_ena::INTR_ENA_SPEC>;
#[doc = "core0 monitor interrupt enable register"]
pub mod intr_ena;
#[doc = "INTR_RAW (r) register accessor: core0 monitor interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_raw`] module"]
pub type INTR_RAW = crate::Reg<intr_raw::INTR_RAW_SPEC>;
#[doc = "core0 monitor interrupt status register"]
pub mod intr_raw;
#[doc = "IRAM0_EXCEPTION_MONITOR_0 (r) register accessor: core0 bus busy status regsiter\n\nYou can [`read`](crate::Reg::read) this register and get [`iram0_exception_monitor_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iram0_exception_monitor_0`] module"]
pub type IRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<iram0_exception_monitor_0::IRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "core0 bus busy status regsiter"]
pub mod iram0_exception_monitor_0;
#[doc = "IRAM0_EXCEPTION_MONITOR_1 (r) register accessor: core0 bus busy status regsiter\n\nYou can [`read`](crate::Reg::read) this register and get [`iram0_exception_monitor_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iram0_exception_monitor_1`] module"]
pub type IRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<iram0_exception_monitor_1::IRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "core0 bus busy status regsiter"]
pub mod iram0_exception_monitor_1;
#[doc = "MONTR_ENA (rw) register accessor: core0 monitor enable configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`montr_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`montr_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@montr_ena`] module"]
pub type MONTR_ENA = crate::Reg<montr_ena::MONTR_ENA_SPEC>;
#[doc = "core0 monitor enable configuration register"]
pub mod montr_ena;
#[doc = "RCD_PDEBUGDATA (r) register accessor: core0 pdebug status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcd_pdebugdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcd_pdebugdata`] module"]
pub type RCD_PDEBUGDATA = crate::Reg<rcd_pdebugdata::RCD_PDEBUGDATA_SPEC>;
#[doc = "core0 pdebug status register"]
pub mod rcd_pdebugdata;
#[doc = "RCD_PDEBUGENABLE (rw) register accessor: core0 pdebug configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcd_pdebugenable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcd_pdebugenable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcd_pdebugenable`] module"]
pub type RCD_PDEBUGENABLE = crate::Reg<rcd_pdebugenable::RCD_PDEBUGENABLE_SPEC>;
#[doc = "core0 pdebug configuration register"]
pub mod rcd_pdebugenable;
#[doc = "RCD_PDEBUGINST (r) register accessor: core0 pdebug status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcd_pdebuginst::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcd_pdebuginst`] module"]
pub type RCD_PDEBUGINST = crate::Reg<rcd_pdebuginst::RCD_PDEBUGINST_SPEC>;
#[doc = "core0 pdebug status register"]
pub mod rcd_pdebuginst;
#[doc = "RCD_PDEBUGLS0ADDR (r) register accessor: core0 pdebug status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcd_pdebugls0addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcd_pdebugls0addr`] module"]
pub type RCD_PDEBUGLS0ADDR = crate::Reg<rcd_pdebugls0addr::RCD_PDEBUGLS0ADDR_SPEC>;
#[doc = "core0 pdebug status register"]
pub mod rcd_pdebugls0addr;
#[doc = "RCD_PDEBUGLS0DATA (r) register accessor: core0 pdebug status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcd_pdebugls0data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcd_pdebugls0data`] module"]
pub type RCD_PDEBUGLS0DATA = crate::Reg<rcd_pdebugls0data::RCD_PDEBUGLS0DATA_SPEC>;
#[doc = "core0 pdebug status register"]
pub mod rcd_pdebugls0data;
#[doc = "RCD_PDEBUGLS0STAT (r) register accessor: core0 pdebug status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcd_pdebugls0stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcd_pdebugls0stat`] module"]
pub type RCD_PDEBUGLS0STAT = crate::Reg<rcd_pdebugls0stat::RCD_PDEBUGLS0STAT_SPEC>;
#[doc = "core0 pdebug status register"]
pub mod rcd_pdebugls0stat;
#[doc = "RCD_PDEBUGPC (r) register accessor: core0 pdebug status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcd_pdebugpc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcd_pdebugpc`] module"]
pub type RCD_PDEBUGPC = crate::Reg<rcd_pdebugpc::RCD_PDEBUGPC_SPEC>;
#[doc = "core0 pdebug status register"]
pub mod rcd_pdebugpc;
#[doc = "RCD_PDEBUGSTATUS (r) register accessor: core0 pdebug status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcd_pdebugstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcd_pdebugstatus`] module"]
pub type RCD_PDEBUGSTATUS = crate::Reg<rcd_pdebugstatus::RCD_PDEBUGSTATUS_SPEC>;
#[doc = "core0 pdebug status register"]
pub mod rcd_pdebugstatus;
#[doc = "RCD_RECORDING (rw) register accessor: core0 pdebug status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcd_recording::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcd_recording::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcd_recording`] module"]
pub type RCD_RECORDING = crate::Reg<rcd_recording::RCD_RECORDING_SPEC>;
#[doc = "core0 pdebug status register"]
pub mod rcd_recording;
#[doc = "RCD_SP (r) register accessor: core0 pdebug status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcd_sp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcd_sp`] module"]
pub type RCD_SP = crate::Reg<rcd_sp::RCD_SP_SPEC>;
#[doc = "core0 pdebug status register"]
pub mod rcd_sp;
#[doc = "SP_MAX (rw) register accessor: core0 sp region configuration regsiter\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_max::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sp_max::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sp_max`] module"]
pub type SP_MAX = crate::Reg<sp_max::SP_MAX_SPEC>;
#[doc = "core0 sp region configuration regsiter"]
pub mod sp_max;
#[doc = "SP_MIN (rw) register accessor: core0 sp region configuration regsiter\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_min::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sp_min::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sp_min`] module"]
pub type SP_MIN = crate::Reg<sp_min::SP_MIN_SPEC>;
#[doc = "core0 sp region configuration regsiter"]
pub mod sp_min;
#[doc = "SP_PC (r) register accessor: core0 sp pc status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_pc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sp_pc`] module"]
pub type SP_PC = crate::Reg<sp_pc::SP_PC_SPEC>;
#[doc = "core0 sp pc status register"]
pub mod sp_pc;
#[doc = "SP_UNSTABLE (rw) register accessor: core0 sp unstable configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_unstable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sp_unstable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sp_unstable`] module"]
pub type SP_UNSTABLE = crate::Reg<sp_unstable::SP_UNSTABLE_SPEC>;
#[doc = "core0 sp unstable configuration register"]
pub mod sp_unstable;
