#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Dedicated GPIO directive output register"]
    pub out_drt: OUT_DRT,
    #[doc = "0x04 - Dedicated GPIO mask output register"]
    pub out_msk: OUT_MSK,
    #[doc = "0x08 - Dedicated GPIO individual output register"]
    pub out_idv: OUT_IDV,
    #[doc = "0x0c - Dedicated GPIO output status register"]
    pub out_scan: OUT_SCAN,
    #[doc = "0x10 - Dedicated GPIO output mode selection register"]
    pub out_cpu: OUT_CPU,
    #[doc = "0x14 - Dedicated GPIO input delay configuration register"]
    pub in_dly: IN_DLY,
    #[doc = "0x18 - Dedicated GPIO input status register"]
    pub in_scan: IN_SCAN,
    #[doc = "0x1c - Dedicated GPIO interrupts generation mode register"]
    pub intr_rcgn: INTR_RCGN,
    #[doc = "0x20 - Raw interrupt status"]
    pub intr_raw: INTR_RAW,
    #[doc = "0x24 - Interrupt enable bits"]
    pub intr_rls: INTR_RLS,
    #[doc = "0x28 - Masked interrupt status"]
    pub intr_st: INTR_ST,
    #[doc = "0x2c - Interrupt clear bits"]
    pub intr_clr: INTR_CLR,
}
#[doc = "OUT_DRT (w) register accessor: Dedicated GPIO directive output register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_drt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_drt`] module"]
pub type OUT_DRT = crate::Reg<out_drt::OUT_DRT_SPEC>;
#[doc = "Dedicated GPIO directive output register"]
pub mod out_drt;
#[doc = "OUT_MSK (w) register accessor: Dedicated GPIO mask output register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_msk::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_msk`] module"]
pub type OUT_MSK = crate::Reg<out_msk::OUT_MSK_SPEC>;
#[doc = "Dedicated GPIO mask output register"]
pub mod out_msk;
#[doc = "OUT_IDV (w) register accessor: Dedicated GPIO individual output register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_idv::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_idv`] module"]
pub type OUT_IDV = crate::Reg<out_idv::OUT_IDV_SPEC>;
#[doc = "Dedicated GPIO individual output register"]
pub mod out_idv;
#[doc = "OUT_SCAN (r) register accessor: Dedicated GPIO output status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_scan::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_scan`] module"]
pub type OUT_SCAN = crate::Reg<out_scan::OUT_SCAN_SPEC>;
#[doc = "Dedicated GPIO output status register"]
pub mod out_scan;
#[doc = "OUT_CPU (rw) register accessor: Dedicated GPIO output mode selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_cpu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_cpu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_cpu`] module"]
pub type OUT_CPU = crate::Reg<out_cpu::OUT_CPU_SPEC>;
#[doc = "Dedicated GPIO output mode selection register"]
pub mod out_cpu;
#[doc = "IN_DLY (rw) register accessor: Dedicated GPIO input delay configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_dly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in_dly`] module"]
pub type IN_DLY = crate::Reg<in_dly::IN_DLY_SPEC>;
#[doc = "Dedicated GPIO input delay configuration register"]
pub mod in_dly;
#[doc = "IN_SCAN (r) register accessor: Dedicated GPIO input status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_scan::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in_scan`] module"]
pub type IN_SCAN = crate::Reg<in_scan::IN_SCAN_SPEC>;
#[doc = "Dedicated GPIO input status register"]
pub mod in_scan;
#[doc = "INTR_RCGN (rw) register accessor: Dedicated GPIO interrupts generation mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_rcgn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_rcgn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intr_rcgn`] module"]
pub type INTR_RCGN = crate::Reg<intr_rcgn::INTR_RCGN_SPEC>;
#[doc = "Dedicated GPIO interrupts generation mode register"]
pub mod intr_rcgn;
#[doc = "INTR_RAW (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intr_raw`] module"]
pub type INTR_RAW = crate::Reg<intr_raw::INTR_RAW_SPEC>;
#[doc = "Raw interrupt status"]
pub mod intr_raw;
#[doc = "INTR_RLS (rw) register accessor: Interrupt enable bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_rls::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_rls::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intr_rls`] module"]
pub type INTR_RLS = crate::Reg<intr_rls::INTR_RLS_SPEC>;
#[doc = "Interrupt enable bits"]
pub mod intr_rls;
#[doc = "INTR_ST (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intr_st`] module"]
pub type INTR_ST = crate::Reg<intr_st::INTR_ST_SPEC>;
#[doc = "Masked interrupt status"]
pub mod intr_st;
#[doc = "INTR_CLR (w) register accessor: Interrupt clear bits\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intr_clr`] module"]
pub type INTR_CLR = crate::Reg<intr_clr::INTR_CLR_SPEC>;
#[doc = "Interrupt clear bits"]
pub mod intr_clr;
