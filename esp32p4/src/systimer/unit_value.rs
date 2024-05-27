#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Cluster UNIT%s_VALUE, containing UNIT?_VALUE_HI, UNIT?_VALUE_LO
pub struct UNIT_VALUE {
    hi: HI,
    lo: LO,
}
impl UNIT_VALUE {
    ///0x00 - system timer unit0 value high register
    #[inline(always)]
    pub const fn hi(&self) -> &HI {
        &self.hi
    }
    ///0x04 - system timer unit0 value low register
    #[inline(always)]
    pub const fn lo(&self) -> &LO {
        &self.lo
    }
}
/**HI (r) register accessor: system timer unit0 value high register

You can [`read`](crate::generic::Reg::read) this register and get [`hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hi`] module*/
pub type HI = crate::Reg<hi::HI_SPEC>;
///system timer unit0 value high register
pub mod hi;
/**LO (r) register accessor: system timer unit0 value low register

You can [`read`](crate::generic::Reg::read) this register and get [`lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lo`] module*/
pub type LO = crate::Reg<lo::LO_SPEC>;
///system timer unit0 value low register
pub mod lo;
