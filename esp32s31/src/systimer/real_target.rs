#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster REAL_TARGET%s, containing REAL_TARGET?_LO, REAL_TARGET?_HI"]
pub struct REAL_TARGET {
    lo: LO,
    hi: HI,
}
impl REAL_TARGET {
    #[doc = "0x00 - Actual target value of COMP0, low 32 bits"]
    #[inline(always)]
    pub const fn lo(&self) -> &LO {
        &self.lo
    }
    #[doc = "0x04 - Actual target value of COMP0, high 20 bits"]
    #[inline(always)]
    pub const fn hi(&self) -> &HI {
        &self.hi
    }
}
#[doc = "LO (r) register accessor: Actual target value of COMP0, low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`lo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo`] module"]
pub type LO = crate::Reg<lo::LO_SPEC>;
#[doc = "Actual target value of COMP0, low 32 bits"]
pub mod lo;
#[doc = "HI (r) register accessor: Actual target value of COMP0, high 20 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`hi::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hi`] module"]
pub type HI = crate::Reg<hi::HI_SPEC>;
#[doc = "Actual target value of COMP0, high 20 bits"]
pub mod hi;
