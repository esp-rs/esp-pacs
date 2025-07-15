#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster CPU%s, containing CORE_?_DEBUG_MODE, CORE_?_INTR_CLR, CORE_?_INTR_ENA, CORE_?_INTR_RAW, CORE_?_LASTPC_BEFORE_EXCEPTION, CORE_?_MONTR_ENA, CORE_?_RCD_EN, CORE_?_RCD_PDEBUGPC, CORE_?_RCD_PDEBUGSP, CORE_?_SP_MAX, CORE_?_SP_MIN, CORE_?_SP_PC"]
pub struct CPU {
    montr_ena: MONTR_ENA,
    intr_raw: INTR_RAW,
    intr_ena: INTR_ENA,
    intr_clr: INTR_CLR,
    sp_min: SP_MIN,
    sp_max: SP_MAX,
    sp_pc: SP_PC,
    rcd_en: RCD_EN,
    rcd_pdebugpc: RCD_PDEBUGPC,
    rcd_pdebugsp: RCD_PDEBUGSP,
    lastpc_before_exception: LASTPC_BEFORE_EXCEPTION,
    debug_mode: DEBUG_MODE,
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
    #[doc = "0x10 - stack min value"]
    #[inline(always)]
    pub const fn sp_min(&self) -> &SP_MIN {
        &self.sp_min
    }
    #[doc = "0x14 - stack max value"]
    #[inline(always)]
    pub const fn sp_max(&self) -> &SP_MAX {
        &self.sp_max
    }
    #[doc = "0x18 - stack monitor pc status register"]
    #[inline(always)]
    pub const fn sp_pc(&self) -> &SP_PC {
        &self.sp_pc
    }
    #[doc = "0x1c - record enable configuration register"]
    #[inline(always)]
    pub const fn rcd_en(&self) -> &RCD_EN {
        &self.rcd_en
    }
    #[doc = "0x20 - record status regsiter"]
    #[inline(always)]
    pub const fn rcd_pdebugpc(&self) -> &RCD_PDEBUGPC {
        &self.rcd_pdebugpc
    }
    #[doc = "0x24 - record status regsiter"]
    #[inline(always)]
    pub const fn rcd_pdebugsp(&self) -> &RCD_PDEBUGSP {
        &self.rcd_pdebugsp
    }
    #[doc = "0x28 - cpu status register"]
    #[inline(always)]
    pub const fn lastpc_before_exception(&self) -> &LASTPC_BEFORE_EXCEPTION {
        &self.lastpc_before_exception
    }
    #[doc = "0x2c - cpu status register"]
    #[inline(always)]
    pub const fn debug_mode(&self) -> &DEBUG_MODE {
        &self.debug_mode
    }
}
#[doc = "DEBUG_MODE (r) register accessor: cpu status register\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_mode`] module"]
pub type DEBUG_MODE = crate::Reg<debug_mode::DEBUG_MODE_SPEC>;
#[doc = "cpu status register"]
pub mod debug_mode;
#[doc = "INTR_CLR (w) register accessor: core0 monitor interrupt clr register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_clr`] module"]
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
#[doc = "LASTPC_BEFORE_EXCEPTION (r) register accessor: cpu status register\n\nYou can [`read`](crate::Reg::read) this register and get [`lastpc_before_exception::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lastpc_before_exception`] module"]
pub type LASTPC_BEFORE_EXCEPTION =
    crate::Reg<lastpc_before_exception::LASTPC_BEFORE_EXCEPTION_SPEC>;
#[doc = "cpu status register"]
pub mod lastpc_before_exception;
#[doc = "MONTR_ENA (rw) register accessor: core0 monitor enable configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`montr_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`montr_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@montr_ena`] module"]
pub type MONTR_ENA = crate::Reg<montr_ena::MONTR_ENA_SPEC>;
#[doc = "core0 monitor enable configuration register"]
pub mod montr_ena;
#[doc = "RCD_EN (rw) register accessor: record enable configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcd_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcd_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcd_en`] module"]
pub type RCD_EN = crate::Reg<rcd_en::RCD_EN_SPEC>;
#[doc = "record enable configuration register"]
pub mod rcd_en;
#[doc = "RCD_PDEBUGPC (r) register accessor: record status regsiter\n\nYou can [`read`](crate::Reg::read) this register and get [`rcd_pdebugpc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcd_pdebugpc`] module"]
pub type RCD_PDEBUGPC = crate::Reg<rcd_pdebugpc::RCD_PDEBUGPC_SPEC>;
#[doc = "record status regsiter"]
pub mod rcd_pdebugpc;
#[doc = "RCD_PDEBUGSP (r) register accessor: record status regsiter\n\nYou can [`read`](crate::Reg::read) this register and get [`rcd_pdebugsp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcd_pdebugsp`] module"]
pub type RCD_PDEBUGSP = crate::Reg<rcd_pdebugsp::RCD_PDEBUGSP_SPEC>;
#[doc = "record status regsiter"]
pub mod rcd_pdebugsp;
#[doc = "SP_MAX (rw) register accessor: stack max value\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_max::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sp_max::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sp_max`] module"]
pub type SP_MAX = crate::Reg<sp_max::SP_MAX_SPEC>;
#[doc = "stack max value"]
pub mod sp_max;
#[doc = "SP_MIN (rw) register accessor: stack min value\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_min::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sp_min::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sp_min`] module"]
pub type SP_MIN = crate::Reg<sp_min::SP_MIN_SPEC>;
#[doc = "stack min value"]
pub mod sp_min;
#[doc = "SP_PC (r) register accessor: stack monitor pc status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_pc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sp_pc`] module"]
pub type SP_PC = crate::Reg<sp_pc::SP_PC_SPEC>;
#[doc = "stack monitor pc status register"]
pub mod sp_pc;
