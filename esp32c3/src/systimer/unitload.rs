#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Cluster UNIT%sLOAD, containing UNIT?_LOAD_HI, UNIT?_LOAD_LO
pub struct UNITLOAD {
    hi: HI,
    lo: LO,
}
impl UNITLOAD {
    ///0x00 - SYSTIMER_UNIT0_LOAD_HI.
    #[inline(always)]
    pub const fn hi(&self) -> &HI {
        &self.hi
    }
    ///0x04 - SYSTIMER_UNIT0_LOAD_LO.
    #[inline(always)]
    pub const fn lo(&self) -> &LO {
        &self.lo
    }
}
/**HI (rw) register accessor: SYSTIMER_UNIT0_LOAD_HI.

You can [`read`](crate::generic::Reg::read) this register and get [`hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hi`] module*/
pub type HI = crate::Reg<hi::HI_SPEC>;
///SYSTIMER_UNIT0_LOAD_HI.
pub mod hi;
/**LO (rw) register accessor: SYSTIMER_UNIT0_LOAD_LO.

You can [`read`](crate::generic::Reg::read) this register and get [`lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lo`] module*/
pub type LO = crate::Reg<lo::LO_SPEC>;
///SYSTIMER_UNIT0_LOAD_LO.
pub mod lo;
