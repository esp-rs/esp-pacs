#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Cluster TRGT%s, containing TARGET?_HI, TARGET?_LO
pub struct TRGT {
    hi: HI,
    lo: LO,
}
impl TRGT {
    ///0x00 - system timer comp0 value high register
    #[inline(always)]
    pub const fn hi(&self) -> &HI {
        &self.hi
    }
    ///0x04 - system timer comp0 value low register
    #[inline(always)]
    pub const fn lo(&self) -> &LO {
        &self.lo
    }
}
/**HI (rw) register accessor: system timer comp0 value high register

You can [`read`](crate::generic::Reg::read) this register and get [`hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hi`] module*/
pub type HI = crate::Reg<hi::HI_SPEC>;
///system timer comp0 value high register
pub mod hi;
/**LO (rw) register accessor: system timer comp0 value low register

You can [`read`](crate::generic::Reg::read) this register and get [`lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lo`] module*/
pub type LO = crate::Reg<lo::LO_SPEC>;
///system timer comp0 value low register
pub mod lo;
