#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster UNIT%s_VALUE, containing UNIT?_VALUE_HI, UNIT?_VALUE_LO"]
pub struct UNIT_VALUE {
    hi: HI,
    lo: LO,
}
impl UNIT_VALUE {
    #[doc = "0x00 - System timer value, high 32 bits"]
    #[inline(always)]
    pub const fn hi(&self) -> &HI {
        &self.hi
    }
    #[doc = "0x04 - System timer value, low 32 bits"]
    #[inline(always)]
    pub const fn lo(&self) -> &LO {
        &self.lo
    }
}
#[doc = "HI (r) register accessor: System timer value, high 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hi`] module"]
pub type HI = crate::Reg<hi::HI_SPEC>;
#[doc = "System timer value, high 32 bits"]
pub mod hi;
#[doc = "LO (r) register accessor: System timer value, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo`] module"]
pub type LO = crate::Reg<lo::LO_SPEC>;
#[doc = "System timer value, low 32 bits"]
pub mod lo;
