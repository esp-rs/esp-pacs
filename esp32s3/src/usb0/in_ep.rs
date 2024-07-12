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
#[doc = "DIEPCTL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl`] module"]
pub type DIEPCTL = crate::Reg<diepctl::DIEPCTL_SPEC>;
#[doc = ""]
pub mod diepctl;
pub use crate::usb0::in_ep0::diepint;
pub use crate::usb0::in_ep0::DIEPINT;
#[doc = "DIEPTSIZ (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz`] module"]
pub type DIEPTSIZ = crate::Reg<dieptsiz::DIEPTSIZ_SPEC>;
#[doc = ""]
pub mod dieptsiz;
pub use crate::usb0::in_ep0::diepdma;
pub use crate::usb0::in_ep0::diepdmab;
pub use crate::usb0::in_ep0::dtxfsts;
pub use crate::usb0::in_ep0::DIEPDMA;
pub use crate::usb0::in_ep0::DIEPDMAB;
pub use crate::usb0::in_ep0::DTXFSTS;
