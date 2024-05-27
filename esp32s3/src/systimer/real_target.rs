#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Cluster REAL_TARGET%s, containing REAL_TARGET?_LO, REAL_TARGET?_HI
pub struct REAL_TARGET {
    lo: LO,
    hi: HI,
}
impl REAL_TARGET {
    ///0x00 - system timer comp0 actual target value low register
    #[inline(always)]
    pub const fn lo(&self) -> &LO {
        &self.lo
    }
    ///0x04 - system timer comp0 actual target value high register
    #[inline(always)]
    pub const fn hi(&self) -> &HI {
        &self.hi
    }
}
/**LO (r) register accessor: system timer comp0 actual target value low register

You can [`read`](crate::generic::Reg::read) this register and get [`lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lo`] module*/
pub type LO = crate::Reg<lo::LO_SPEC>;
///system timer comp0 actual target value low register
pub mod lo;
/**HI (r) register accessor: system timer comp0 actual target value high register

You can [`read`](crate::generic::Reg::read) this register and get [`hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hi`] module*/
pub type HI = crate::Reg<hi::HI_SPEC>;
///system timer comp0 actual target value high register
pub mod hi;
