#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    out_drt: OUT_DRT,
    out_msk: OUT_MSK,
    out_idv: OUT_IDV,
    out_scan: OUT_SCAN,
    out_cpu: OUT_CPU,
    in_dly: IN_DLY,
    in_scan: IN_SCAN,
    intr_rcgn: INTR_RCGN,
    intr_raw: INTR_RAW,
    intr_rls: INTR_RLS,
    intr_st: INTR_ST,
    intr_clr: INTR_CLR,
}
impl RegisterBlock {
    ///0x00 - Dedicated GPIO directive output register
    #[inline(always)]
    pub const fn out_drt(&self) -> &OUT_DRT {
        &self.out_drt
    }
    ///0x04 - Dedicated GPIO mask output register
    #[inline(always)]
    pub const fn out_msk(&self) -> &OUT_MSK {
        &self.out_msk
    }
    ///0x08 - Dedicated GPIO individual output register
    #[inline(always)]
    pub const fn out_idv(&self) -> &OUT_IDV {
        &self.out_idv
    }
    ///0x0c - Dedicated GPIO output status register
    #[inline(always)]
    pub const fn out_scan(&self) -> &OUT_SCAN {
        &self.out_scan
    }
    ///0x10 - Dedicated GPIO output mode selection register
    #[inline(always)]
    pub const fn out_cpu(&self) -> &OUT_CPU {
        &self.out_cpu
    }
    ///0x14 - Dedicated GPIO input delay configuration register
    #[inline(always)]
    pub const fn in_dly(&self) -> &IN_DLY {
        &self.in_dly
    }
    ///0x18 - Dedicated GPIO input status register
    #[inline(always)]
    pub const fn in_scan(&self) -> &IN_SCAN {
        &self.in_scan
    }
    ///0x1c - Dedicated GPIO interrupts generation mode register
    #[inline(always)]
    pub const fn intr_rcgn(&self) -> &INTR_RCGN {
        &self.intr_rcgn
    }
    ///0x20 - Raw interrupt status
    #[inline(always)]
    pub const fn intr_raw(&self) -> &INTR_RAW {
        &self.intr_raw
    }
    ///0x24 - Interrupt enable bits
    #[inline(always)]
    pub const fn intr_rls(&self) -> &INTR_RLS {
        &self.intr_rls
    }
    ///0x28 - Masked interrupt status
    #[inline(always)]
    pub const fn intr_st(&self) -> &INTR_ST {
        &self.intr_st
    }
    ///0x2c - Interrupt clear bits
    #[inline(always)]
    pub const fn intr_clr(&self) -> &INTR_CLR {
        &self.intr_clr
    }
}
/**OUT_DRT (w) register accessor: Dedicated GPIO directive output register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_drt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_drt`] module*/
pub type OUT_DRT = crate::Reg<out_drt::OUT_DRT_SPEC>;
///Dedicated GPIO directive output register
pub mod out_drt;
/**OUT_MSK (w) register accessor: Dedicated GPIO mask output register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_msk::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_msk`] module*/
pub type OUT_MSK = crate::Reg<out_msk::OUT_MSK_SPEC>;
///Dedicated GPIO mask output register
pub mod out_msk;
/**OUT_IDV (w) register accessor: Dedicated GPIO individual output register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_idv::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_idv`] module*/
pub type OUT_IDV = crate::Reg<out_idv::OUT_IDV_SPEC>;
///Dedicated GPIO individual output register
pub mod out_idv;
/**OUT_SCAN (r) register accessor: Dedicated GPIO output status register

You can [`read`](crate::generic::Reg::read) this register and get [`out_scan::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_scan`] module*/
pub type OUT_SCAN = crate::Reg<out_scan::OUT_SCAN_SPEC>;
///Dedicated GPIO output status register
pub mod out_scan;
/**OUT_CPU (rw) register accessor: Dedicated GPIO output mode selection register

You can [`read`](crate::generic::Reg::read) this register and get [`out_cpu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_cpu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_cpu`] module*/
pub type OUT_CPU = crate::Reg<out_cpu::OUT_CPU_SPEC>;
///Dedicated GPIO output mode selection register
pub mod out_cpu;
/**IN_DLY (rw) register accessor: Dedicated GPIO input delay configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`in_dly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_dly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@in_dly`] module*/
pub type IN_DLY = crate::Reg<in_dly::IN_DLY_SPEC>;
///Dedicated GPIO input delay configuration register
pub mod in_dly;
/**IN_SCAN (r) register accessor: Dedicated GPIO input status register

You can [`read`](crate::generic::Reg::read) this register and get [`in_scan::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@in_scan`] module*/
pub type IN_SCAN = crate::Reg<in_scan::IN_SCAN_SPEC>;
///Dedicated GPIO input status register
pub mod in_scan;
/**INTR_RCGN (rw) register accessor: Dedicated GPIO interrupts generation mode register

You can [`read`](crate::generic::Reg::read) this register and get [`intr_rcgn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_rcgn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intr_rcgn`] module*/
pub type INTR_RCGN = crate::Reg<intr_rcgn::INTR_RCGN_SPEC>;
///Dedicated GPIO interrupts generation mode register
pub mod intr_rcgn;
/**INTR_RAW (r) register accessor: Raw interrupt status

You can [`read`](crate::generic::Reg::read) this register and get [`intr_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intr_raw`] module*/
pub type INTR_RAW = crate::Reg<intr_raw::INTR_RAW_SPEC>;
///Raw interrupt status
pub mod intr_raw;
/**INTR_RLS (rw) register accessor: Interrupt enable bits

You can [`read`](crate::generic::Reg::read) this register and get [`intr_rls::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_rls::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intr_rls`] module*/
pub type INTR_RLS = crate::Reg<intr_rls::INTR_RLS_SPEC>;
///Interrupt enable bits
pub mod intr_rls;
/**INTR_ST (r) register accessor: Masked interrupt status

You can [`read`](crate::generic::Reg::read) this register and get [`intr_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intr_st`] module*/
pub type INTR_ST = crate::Reg<intr_st::INTR_ST_SPEC>;
///Masked interrupt status
pub mod intr_st;
/**INTR_CLR (w) register accessor: Interrupt clear bits

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intr_clr`] module*/
pub type INTR_CLR = crate::Reg<intr_clr::INTR_CLR_SPEC>;
///Interrupt clear bits
pub mod intr_clr;
