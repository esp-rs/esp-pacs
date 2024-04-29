#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster TRGT%s, containing TARGET?_HI, TARGET?_LO"]
pub struct TRGT {
    hi: HI,
    lo: LO,
}
impl TRGT {
    #[doc = "0x00 - System timer target 0, high 32 bits"]
    #[inline(always)]
    pub const fn hi(&self) -> &HI {
        &self.hi
    }
    #[doc = "0x04 - System timer target 0, low 32 bits"]
    #[inline(always)]
    pub const fn lo(&self) -> &LO {
        &self.lo
    }
}
#[doc = "HI (rw) register accessor: System timer target 0, high 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hi`] module"]
pub type HI = crate::Reg<hi::HI_SPEC>;
#[doc = "System timer target 0, high 32 bits"]
pub mod hi;
#[doc = "LO (rw) register accessor: System timer target 0, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo`] module"]
pub type LO = crate::Reg<lo::LO_SPEC>;
#[doc = "System timer target 0, low 32 bits"]
pub mod lo;
