#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Device IN endpoints 1-6
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
    ///0x00 -
    #[inline(always)]
    pub const fn diepctl(&self) -> &DIEPCTL {
        &self.diepctl
    }
    ///0x08 -
    #[inline(always)]
    pub const fn diepint(&self) -> &DIEPINT {
        &self.diepint
    }
    ///0x10 -
    #[inline(always)]
    pub const fn dieptsiz(&self) -> &DIEPTSIZ {
        &self.dieptsiz
    }
    ///0x14 -
    #[inline(always)]
    pub const fn diepdma(&self) -> &DIEPDMA {
        &self.diepdma
    }
    ///0x18 -
    #[inline(always)]
    pub const fn dtxfsts(&self) -> &DTXFSTS {
        &self.dtxfsts
    }
    ///0x1c -
    #[inline(always)]
    pub const fn diepdmab(&self) -> &DIEPDMAB {
        &self.diepdmab
    }
}
/**DIEPCTL (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`diepctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@diepctl`] module*/
pub type DIEPCTL = crate::Reg<diepctl::DIEPCTL_SPEC>;
///
pub mod diepctl;
pub use crate::usb0::in_ep0::diepint;
pub use crate::usb0::in_ep0::DIEPINT;
/**DIEPTSIZ (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dieptsiz`] module*/
pub type DIEPTSIZ = crate::Reg<dieptsiz::DIEPTSIZ_SPEC>;
///
pub mod dieptsiz;
pub use crate::usb0::in_ep0::diepdma;
pub use crate::usb0::in_ep0::diepdmab;
pub use crate::usb0::in_ep0::dtxfsts;
pub use crate::usb0::in_ep0::DIEPDMA;
pub use crate::usb0::in_ep0::DIEPDMAB;
pub use crate::usb0::in_ep0::DTXFSTS;
