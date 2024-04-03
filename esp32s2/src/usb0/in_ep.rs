#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Device IN endpoints 1-6"]
pub struct IN_EP {
    diepctl: DIEPCTL,
    _reserved1: [u8; 0x04],
    diepint: DIEPINT,
    _reserved2: [u8; 0x04],
    dieptsiz: DIEPTSIZ,
    diepdma: DIEPDMA,
    dtxfsts: DTXFSTS,
    diepdmab: DIEPDMAB,
}
impl IN_EP {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn diepctl(&self) -> &DIEPCTL {
        &self.diepctl
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn diepint(&self) -> &DIEPINT {
        &self.diepint
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn dieptsiz(&self) -> &DIEPTSIZ {
        &self.dieptsiz
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn diepdma(&self) -> &DIEPDMA {
        &self.diepdma
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn dtxfsts(&self) -> &DTXFSTS {
        &self.dtxfsts
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn diepdmab(&self) -> &DIEPDMAB {
        &self.diepdmab
    }
}
#[doc = "DIEPCTL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl`] module"]
pub type DIEPCTL = crate::Reg<diepctl::DIEPCTL_SPEC>;
#[doc = ""]
pub mod diepctl;
#[doc = "DIEPINT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint`] module"]
pub type DIEPINT = crate::Reg<diepint::DIEPINT_SPEC>;
#[doc = ""]
pub mod diepint;
#[doc = "DIEPTSIZ (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz`] module"]
pub type DIEPTSIZ = crate::Reg<dieptsiz::DIEPTSIZ_SPEC>;
#[doc = ""]
pub mod dieptsiz;
#[doc = "DIEPDMA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma`] module"]
pub type DIEPDMA = crate::Reg<diepdma::DIEPDMA_SPEC>;
#[doc = ""]
pub mod diepdma;
#[doc = "DIEPDMAB (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdmab::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdmab`] module"]
pub type DIEPDMAB = crate::Reg<diepdmab::DIEPDMAB_SPEC>;
#[doc = ""]
pub mod diepdmab;
#[doc = "DTXFSTS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts`] module"]
pub type DTXFSTS = crate::Reg<dtxfsts::DTXFSTS_SPEC>;
#[doc = ""]
pub mod dtxfsts;
