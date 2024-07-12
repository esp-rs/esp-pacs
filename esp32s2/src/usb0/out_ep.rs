#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Device OUT endpoints 1-6"]
pub struct OUT_EP {
    doepctl: DOEPCTL,
    _reserved1: [u8; 0x04],
    doepint: DOEPINT,
    _reserved2: [u8; 0x04],
    doeptsiz: DOEPTSIZ,
    doepdma: DOEPDMA,
    _reserved4: [u8; 0x04],
    doepdmab: DOEPDMAB,
}
impl OUT_EP {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn doepctl(&self) -> &DOEPCTL {
        &self.doepctl
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn doepint(&self) -> &DOEPINT {
        &self.doepint
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn doeptsiz(&self) -> &DOEPTSIZ {
        &self.doeptsiz
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn doepdma(&self) -> &DOEPDMA {
        &self.doepdma
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn doepdmab(&self) -> &DOEPDMAB {
        &self.doepdmab
    }
}
#[doc = "DOEPCTL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl`] module"]
pub type DOEPCTL = crate::Reg<doepctl::DOEPCTL_SPEC>;
#[doc = ""]
pub mod doepctl;
pub use crate::usb0::out_ep0::doepint;
pub use crate::usb0::out_ep0::DOEPINT;
#[doc = "DOEPTSIZ (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz`] module"]
pub type DOEPTSIZ = crate::Reg<doeptsiz::DOEPTSIZ_SPEC>;
#[doc = ""]
pub mod doeptsiz;
pub use crate::usb0::out_ep0::doepdma;
pub use crate::usb0::out_ep0::doepdmab;
pub use crate::usb0::out_ep0::DOEPDMA;
pub use crate::usb0::out_ep0::DOEPDMAB;
