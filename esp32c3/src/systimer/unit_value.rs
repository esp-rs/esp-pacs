#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Cluster UNIT%s_VALUE, containing UNIT?_VALUE_HI, UNIT?_VALUE_LO
pub struct UNIT_VALUE {
    hi: HI,
    lo: LO,
}
impl UNIT_VALUE {
    ///0x00 - SYSTIMER_UNIT0_VALUE_HI.
    #[inline(always)]
    pub const fn hi(&self) -> &HI {
        &self.hi
    }
    ///0x04 - SYSTIMER_UNIT0_VALUE_LO.
    #[inline(always)]
    pub const fn lo(&self) -> &LO {
        &self.lo
    }
}
/**HI (r) register accessor: SYSTIMER_UNIT0_VALUE_HI.

You can [`read`](crate::generic::Reg::read) this register and get [`hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hi`] module*/
pub type HI = crate::Reg<hi::HI_SPEC>;
///SYSTIMER_UNIT0_VALUE_HI.
pub mod hi;
/**LO (r) register accessor: SYSTIMER_UNIT0_VALUE_LO.

You can [`read`](crate::generic::Reg::read) this register and get [`lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lo`] module*/
pub type LO = crate::Reg<lo::LO_SPEC>;
///SYSTIMER_UNIT0_VALUE_LO.
pub mod lo;
