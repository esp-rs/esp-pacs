#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub gotgctl: GOTGCTL,
    #[doc = "0x04 - "]
    pub gotgint: GOTGINT,
    #[doc = "0x08 - "]
    pub gahbcfg: GAHBCFG,
    #[doc = "0x0c - "]
    pub gusbcfg: GUSBCFG,
    #[doc = "0x10 - "]
    pub grstctl: GRSTCTL,
    #[doc = "0x14 - "]
    pub gintsts: GINTSTS,
    #[doc = "0x18 - "]
    pub gintmsk: GINTMSK,
    #[doc = "0x1c - "]
    pub grxstsr: GRXSTSR,
    #[doc = "0x20 - "]
    pub grxstsp: GRXSTSP,
    #[doc = "0x24 - "]
    pub grxfsiz: GRXFSIZ,
    #[doc = "0x28 - "]
    pub gnptxfsiz: GNPTXFSIZ,
    #[doc = "0x2c - "]
    pub gnptxsts: GNPTXSTS,
    _reserved12: [u8; 0x10],
    #[doc = "0x40 - "]
    pub gsnpsid: GSNPSID,
    #[doc = "0x44 - "]
    pub ghwcfg1: GHWCFG1,
    #[doc = "0x48 - "]
    pub ghwcfg2: GHWCFG2,
    #[doc = "0x4c - "]
    pub ghwcfg3: GHWCFG3,
    #[doc = "0x50 - "]
    pub ghwcfg4: GHWCFG4,
    _reserved17: [u8; 0x08],
    #[doc = "0x5c - "]
    pub gdfifocfg: GDFIFOCFG,
    _reserved18: [u8; 0xa0],
    #[doc = "0x100 - "]
    pub hptxfsiz: HPTXFSIZ,
    #[doc = "0x104 - "]
    pub dieptxf1: DIEPTXF1,
    #[doc = "0x108 - "]
    pub dieptxf2: DIEPTXF2,
    #[doc = "0x10c - "]
    pub dieptxf3: DIEPTXF3,
    #[doc = "0x110 - "]
    pub dieptxf4: DIEPTXF4,
    _reserved23: [u8; 0x02ec],
    #[doc = "0x400 - "]
    pub hcfg: HCFG,
    #[doc = "0x404 - "]
    pub hfir: HFIR,
    #[doc = "0x408 - "]
    pub hfnum: HFNUM,
    _reserved26: [u8; 0x04],
    #[doc = "0x410 - "]
    pub hptxsts: HPTXSTS,
    #[doc = "0x414 - "]
    pub haint: HAINT,
    #[doc = "0x418 - "]
    pub haintmsk: HAINTMSK,
    #[doc = "0x41c - "]
    pub hflbaddr: HFLBADDR,
    _reserved30: [u8; 0x20],
    #[doc = "0x440 - "]
    pub hprt: HPRT,
    _reserved31: [u8; 0xbc],
    #[doc = "0x500 - "]
    pub hcchar0: HCCHAR0,
    _reserved32: [u8; 0x04],
    #[doc = "0x508 - "]
    pub hcint0: HCINT0,
    #[doc = "0x50c - "]
    pub hcintmsk0: HCINTMSK0,
    #[doc = "0x510 - "]
    pub hctsiz0: HCTSIZ0,
    #[doc = "0x514 - "]
    pub hcdma0: HCDMA0,
    _reserved36: [u8; 0x04],
    #[doc = "0x51c - "]
    pub hcdmab0: HCDMAB0,
    #[doc = "0x520 - "]
    pub hcchar1: HCCHAR1,
    _reserved38: [u8; 0x04],
    #[doc = "0x528 - "]
    pub hcint1: HCINT1,
    #[doc = "0x52c - "]
    pub hcintmsk1: HCINTMSK1,
    #[doc = "0x530 - "]
    pub hctsiz1: HCTSIZ1,
    #[doc = "0x534 - "]
    pub hcdma1: HCDMA1,
    _reserved42: [u8; 0x04],
    #[doc = "0x53c - "]
    pub hcdmab1: HCDMAB1,
    #[doc = "0x540 - "]
    pub hcchar2: HCCHAR2,
    _reserved44: [u8; 0x04],
    #[doc = "0x548 - "]
    pub hcint2: HCINT2,
    #[doc = "0x54c - "]
    pub hcintmsk2: HCINTMSK2,
    #[doc = "0x550 - "]
    pub hctsiz2: HCTSIZ2,
    #[doc = "0x554 - "]
    pub hcdma2: HCDMA2,
    _reserved48: [u8; 0x04],
    #[doc = "0x55c - "]
    pub hcdmab2: HCDMAB2,
    #[doc = "0x560 - "]
    pub hcchar3: HCCHAR3,
    _reserved50: [u8; 0x04],
    #[doc = "0x568 - "]
    pub hcint3: HCINT3,
    #[doc = "0x56c - "]
    pub hcintmsk3: HCINTMSK3,
    #[doc = "0x570 - "]
    pub hctsiz3: HCTSIZ3,
    #[doc = "0x574 - "]
    pub hcdma3: HCDMA3,
    _reserved54: [u8; 0x04],
    #[doc = "0x57c - "]
    pub hcdmab3: HCDMAB3,
    #[doc = "0x580 - "]
    pub hcchar4: HCCHAR4,
    _reserved56: [u8; 0x04],
    #[doc = "0x588 - "]
    pub hcint4: HCINT4,
    #[doc = "0x58c - "]
    pub hcintmsk4: HCINTMSK4,
    #[doc = "0x590 - "]
    pub hctsiz4: HCTSIZ4,
    #[doc = "0x594 - "]
    pub hcdma4: HCDMA4,
    _reserved60: [u8; 0x04],
    #[doc = "0x59c - "]
    pub hcdmab4: HCDMAB4,
    #[doc = "0x5a0 - "]
    pub hcchar5: HCCHAR5,
    _reserved62: [u8; 0x04],
    #[doc = "0x5a8 - "]
    pub hcint5: HCINT5,
    #[doc = "0x5ac - "]
    pub hcintmsk5: HCINTMSK5,
    #[doc = "0x5b0 - "]
    pub hctsiz5: HCTSIZ5,
    #[doc = "0x5b4 - "]
    pub hcdma5: HCDMA5,
    _reserved66: [u8; 0x04],
    #[doc = "0x5bc - "]
    pub hcdmab5: HCDMAB5,
    #[doc = "0x5c0 - "]
    pub hcchar6: HCCHAR6,
    _reserved68: [u8; 0x04],
    #[doc = "0x5c8 - "]
    pub hcint6: HCINT6,
    #[doc = "0x5cc - "]
    pub hcintmsk6: HCINTMSK6,
    #[doc = "0x5d0 - "]
    pub hctsiz6: HCTSIZ6,
    #[doc = "0x5d4 - "]
    pub hcdma6: HCDMA6,
    _reserved72: [u8; 0x04],
    #[doc = "0x5dc - "]
    pub hcdmab6: HCDMAB6,
    #[doc = "0x5e0 - "]
    pub hcchar7: HCCHAR7,
    _reserved74: [u8; 0x04],
    #[doc = "0x5e8 - "]
    pub hcint7: HCINT7,
    #[doc = "0x5ec - "]
    pub hcintmsk7: HCINTMSK7,
    #[doc = "0x5f0 - "]
    pub hctsiz7: HCTSIZ7,
    #[doc = "0x5f4 - "]
    pub hcdma7: HCDMA7,
    _reserved78: [u8; 0x04],
    #[doc = "0x5fc - "]
    pub hcdmab7: HCDMAB7,
    _reserved79: [u8; 0x0200],
    #[doc = "0x800 - "]
    pub dcfg: DCFG,
    #[doc = "0x804 - "]
    pub dctl: DCTL,
    #[doc = "0x808 - "]
    pub dsts: DSTS,
    _reserved82: [u8; 0x04],
    #[doc = "0x810 - "]
    pub diepmsk: DIEPMSK,
    #[doc = "0x814 - "]
    pub doepmsk: DOEPMSK,
    #[doc = "0x818 - "]
    pub daint: DAINT,
    #[doc = "0x81c - "]
    pub daintmsk: DAINTMSK,
    _reserved86: [u8; 0x08],
    #[doc = "0x828 - "]
    pub dvbusdis: DVBUSDIS,
    #[doc = "0x82c - "]
    pub dvbuspulse: DVBUSPULSE,
    #[doc = "0x830 - "]
    pub dthrctl: DTHRCTL,
    #[doc = "0x834 - "]
    pub diepempmsk: DIEPEMPMSK,
    _reserved90: [u8; 0xc8],
    #[doc = "0x900 - "]
    pub diepctl0: DIEPCTL0,
    _reserved91: [u8; 0x04],
    #[doc = "0x908 - "]
    pub diepint0: DIEPINT0,
    _reserved92: [u8; 0x04],
    #[doc = "0x910 - "]
    pub dieptsiz0: DIEPTSIZ0,
    #[doc = "0x914 - "]
    pub diepdma0: DIEPDMA0,
    #[doc = "0x918 - "]
    pub dtxfsts0: DTXFSTS0,
    #[doc = "0x91c - "]
    pub diepdmab0: DIEPDMAB0,
    #[doc = "0x920 - "]
    pub diepctl1: DIEPCTL1,
    _reserved97: [u8; 0x04],
    #[doc = "0x928 - "]
    pub diepint1: DIEPINT1,
    _reserved98: [u8; 0x04],
    #[doc = "0x930 - "]
    pub dieptsiz1: DIEPTSIZ1,
    #[doc = "0x934 - "]
    pub diepdma1: DIEPDMA1,
    #[doc = "0x938 - "]
    pub dtxfsts1: DTXFSTS1,
    #[doc = "0x93c - "]
    pub diepdmab1: DIEPDMAB1,
    #[doc = "0x940 - "]
    pub diepctl2: DIEPCTL2,
    _reserved103: [u8; 0x04],
    #[doc = "0x948 - "]
    pub diepint2: DIEPINT2,
    _reserved104: [u8; 0x04],
    #[doc = "0x950 - "]
    pub dieptsiz2: DIEPTSIZ2,
    #[doc = "0x954 - "]
    pub diepdma2: DIEPDMA2,
    #[doc = "0x958 - "]
    pub dtxfsts2: DTXFSTS2,
    #[doc = "0x95c - "]
    pub diepdmab2: DIEPDMAB2,
    #[doc = "0x960 - "]
    pub diepctl3: DIEPCTL3,
    _reserved109: [u8; 0x04],
    #[doc = "0x968 - "]
    pub diepint3: DIEPINT3,
    _reserved110: [u8; 0x04],
    #[doc = "0x970 - "]
    pub dieptsiz3: DIEPTSIZ3,
    #[doc = "0x974 - "]
    pub diepdma3: DIEPDMA3,
    #[doc = "0x978 - "]
    pub dtxfsts3: DTXFSTS3,
    #[doc = "0x97c - "]
    pub diepdmab3: DIEPDMAB3,
    #[doc = "0x980 - "]
    pub diepctl4: DIEPCTL4,
    _reserved115: [u8; 0x04],
    #[doc = "0x988 - "]
    pub diepint4: DIEPINT4,
    _reserved116: [u8; 0x04],
    #[doc = "0x990 - "]
    pub dieptsiz4: DIEPTSIZ4,
    #[doc = "0x994 - "]
    pub diepdma4: DIEPDMA4,
    #[doc = "0x998 - "]
    pub dtxfsts4: DTXFSTS4,
    #[doc = "0x99c - "]
    pub diepdmab4: DIEPDMAB4,
    #[doc = "0x9a0 - "]
    pub diepctl5: DIEPCTL5,
    _reserved121: [u8; 0x04],
    #[doc = "0x9a8 - "]
    pub diepint5: DIEPINT5,
    _reserved122: [u8; 0x04],
    #[doc = "0x9b0 - "]
    pub dieptsiz5: DIEPTSIZ5,
    #[doc = "0x9b4 - "]
    pub diepdma5: DIEPDMA5,
    #[doc = "0x9b8 - "]
    pub dtxfsts5: DTXFSTS5,
    #[doc = "0x9bc - "]
    pub diepdmab5: DIEPDMAB5,
    #[doc = "0x9c0 - "]
    pub diepctl6: DIEPCTL6,
    _reserved127: [u8; 0x04],
    #[doc = "0x9c8 - "]
    pub diepint6: DIEPINT6,
    _reserved128: [u8; 0x04],
    #[doc = "0x9d0 - "]
    pub dieptsiz6: DIEPTSIZ6,
    #[doc = "0x9d4 - "]
    pub diepdma6: DIEPDMA6,
    #[doc = "0x9d8 - "]
    pub dtxfsts6: DTXFSTS6,
    #[doc = "0x9dc - "]
    pub diepdmab6: DIEPDMAB6,
    _reserved132: [u8; 0x0120],
    #[doc = "0xb00 - "]
    pub doepctl0: DOEPCTL0,
    _reserved133: [u8; 0x04],
    #[doc = "0xb08 - "]
    pub doepint0: DOEPINT0,
    _reserved134: [u8; 0x04],
    #[doc = "0xb10 - "]
    pub doeptsiz0: DOEPTSIZ0,
    #[doc = "0xb14 - "]
    pub doepdma0: DOEPDMA0,
    _reserved136: [u8; 0x04],
    #[doc = "0xb1c - "]
    pub doepdmab0: DOEPDMAB0,
    #[doc = "0xb20 - "]
    pub doepctl1: DOEPCTL1,
    _reserved138: [u8; 0x04],
    #[doc = "0xb28 - "]
    pub doepint1: DOEPINT1,
    _reserved139: [u8; 0x04],
    #[doc = "0xb30 - "]
    pub doeptsiz1: DOEPTSIZ1,
    #[doc = "0xb34 - "]
    pub doepdma1: DOEPDMA1,
    _reserved141: [u8; 0x04],
    #[doc = "0xb3c - "]
    pub doepdmab1: DOEPDMAB1,
    #[doc = "0xb40 - "]
    pub doepctl2: DOEPCTL2,
    _reserved143: [u8; 0x04],
    #[doc = "0xb48 - "]
    pub doepint2: DOEPINT2,
    _reserved144: [u8; 0x04],
    #[doc = "0xb50 - "]
    pub doeptsiz2: DOEPTSIZ2,
    #[doc = "0xb54 - "]
    pub doepdma2: DOEPDMA2,
    _reserved146: [u8; 0x04],
    #[doc = "0xb5c - "]
    pub doepdmab2: DOEPDMAB2,
    #[doc = "0xb60 - "]
    pub doepctl3: DOEPCTL3,
    _reserved148: [u8; 0x04],
    #[doc = "0xb68 - "]
    pub doepint3: DOEPINT3,
    _reserved149: [u8; 0x04],
    #[doc = "0xb70 - "]
    pub doeptsiz3: DOEPTSIZ3,
    #[doc = "0xb74 - "]
    pub doepdma3: DOEPDMA3,
    _reserved151: [u8; 0x04],
    #[doc = "0xb7c - "]
    pub doepdmab3: DOEPDMAB3,
    #[doc = "0xb80 - "]
    pub doepctl4: DOEPCTL4,
    _reserved153: [u8; 0x04],
    #[doc = "0xb88 - "]
    pub doepint4: DOEPINT4,
    _reserved154: [u8; 0x04],
    #[doc = "0xb90 - "]
    pub doeptsiz4: DOEPTSIZ4,
    #[doc = "0xb94 - "]
    pub doepdma4: DOEPDMA4,
    _reserved156: [u8; 0x04],
    #[doc = "0xb9c - "]
    pub doepdmab4: DOEPDMAB4,
    #[doc = "0xba0 - "]
    pub doepctl5: DOEPCTL5,
    _reserved158: [u8; 0x04],
    #[doc = "0xba8 - "]
    pub doepint5: DOEPINT5,
    _reserved159: [u8; 0x04],
    #[doc = "0xbb0 - "]
    pub doeptsiz5: DOEPTSIZ5,
    #[doc = "0xbb4 - "]
    pub doepdma5: DOEPDMA5,
    _reserved161: [u8; 0x04],
    #[doc = "0xbbc - "]
    pub doepdmab5: DOEPDMAB5,
    #[doc = "0xbc0 - "]
    pub doepctl6: DOEPCTL6,
    _reserved163: [u8; 0x04],
    #[doc = "0xbc8 - "]
    pub doepint6: DOEPINT6,
    _reserved164: [u8; 0x04],
    #[doc = "0xbd0 - "]
    pub doeptsiz6: DOEPTSIZ6,
    #[doc = "0xbd4 - "]
    pub doepdma6: DOEPDMA6,
    _reserved166: [u8; 0x04],
    #[doc = "0xbdc - "]
    pub doepdmab6: DOEPDMAB6,
    _reserved167: [u8; 0x0220],
    #[doc = "0xe00 - "]
    pub pcgcctl: PCGCCTL,
}
#[doc = "GOTGCTL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gotgctl`] module"]
pub type GOTGCTL = crate::Reg<gotgctl::GOTGCTL_SPEC>;
#[doc = ""]
pub mod gotgctl;
#[doc = "GOTGINT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gotgint`] module"]
pub type GOTGINT = crate::Reg<gotgint::GOTGINT_SPEC>;
#[doc = ""]
pub mod gotgint;
#[doc = "GAHBCFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gahbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gahbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gahbcfg`] module"]
pub type GAHBCFG = crate::Reg<gahbcfg::GAHBCFG_SPEC>;
#[doc = ""]
pub mod gahbcfg;
#[doc = "GUSBCFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gusbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gusbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gusbcfg`] module"]
pub type GUSBCFG = crate::Reg<gusbcfg::GUSBCFG_SPEC>;
#[doc = ""]
pub mod gusbcfg;
#[doc = "GRSTCTL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grstctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grstctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`grstctl`] module"]
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTL_SPEC>;
#[doc = ""]
pub mod grstctl;
#[doc = "GINTSTS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gintsts`] module"]
pub type GINTSTS = crate::Reg<gintsts::GINTSTS_SPEC>;
#[doc = ""]
pub mod gintsts;
#[doc = "GINTMSK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gintmsk`] module"]
pub type GINTMSK = crate::Reg<gintmsk::GINTMSK_SPEC>;
#[doc = ""]
pub mod gintmsk;
#[doc = "GRXSTSR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`grxstsr`] module"]
pub type GRXSTSR = crate::Reg<grxstsr::GRXSTSR_SPEC>;
#[doc = ""]
pub mod grxstsr;
#[doc = "GRXSTSP (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`grxstsp`] module"]
pub type GRXSTSP = crate::Reg<grxstsp::GRXSTSP_SPEC>;
#[doc = ""]
pub mod grxstsp;
#[doc = "GRXFSIZ (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`grxfsiz`] module"]
pub type GRXFSIZ = crate::Reg<grxfsiz::GRXFSIZ_SPEC>;
#[doc = ""]
pub mod grxfsiz;
#[doc = "GNPTXFSIZ (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gnptxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gnptxfsiz`] module"]
pub type GNPTXFSIZ = crate::Reg<gnptxfsiz::GNPTXFSIZ_SPEC>;
#[doc = ""]
pub mod gnptxfsiz;
#[doc = "GNPTXSTS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gnptxsts`] module"]
pub type GNPTXSTS = crate::Reg<gnptxsts::GNPTXSTS_SPEC>;
#[doc = ""]
pub mod gnptxsts;
#[doc = "GSNPSID (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gsnpsid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gsnpsid`] module"]
pub type GSNPSID = crate::Reg<gsnpsid::GSNPSID_SPEC>;
#[doc = ""]
pub mod gsnpsid;
#[doc = "GHWCFG1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghwcfg1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ghwcfg1`] module"]
pub type GHWCFG1 = crate::Reg<ghwcfg1::GHWCFG1_SPEC>;
#[doc = ""]
pub mod ghwcfg1;
#[doc = "GHWCFG2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghwcfg2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ghwcfg2`] module"]
pub type GHWCFG2 = crate::Reg<ghwcfg2::GHWCFG2_SPEC>;
#[doc = ""]
pub mod ghwcfg2;
#[doc = "GHWCFG3 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghwcfg3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ghwcfg3`] module"]
pub type GHWCFG3 = crate::Reg<ghwcfg3::GHWCFG3_SPEC>;
#[doc = ""]
pub mod ghwcfg3;
#[doc = "GHWCFG4 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghwcfg4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ghwcfg4`] module"]
pub type GHWCFG4 = crate::Reg<ghwcfg4::GHWCFG4_SPEC>;
#[doc = ""]
pub mod ghwcfg4;
#[doc = "GDFIFOCFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdfifocfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdfifocfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gdfifocfg`] module"]
pub type GDFIFOCFG = crate::Reg<gdfifocfg::GDFIFOCFG_SPEC>;
#[doc = ""]
pub mod gdfifocfg;
#[doc = "HPTXFSIZ (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hptxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hptxfsiz`] module"]
pub type HPTXFSIZ = crate::Reg<hptxfsiz::HPTXFSIZ_SPEC>;
#[doc = ""]
pub mod hptxfsiz;
#[doc = "DIEPTXF1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptxf1`] module"]
pub type DIEPTXF1 = crate::Reg<dieptxf1::DIEPTXF1_SPEC>;
#[doc = ""]
pub mod dieptxf1;
#[doc = "DIEPTXF2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptxf2`] module"]
pub type DIEPTXF2 = crate::Reg<dieptxf2::DIEPTXF2_SPEC>;
#[doc = ""]
pub mod dieptxf2;
#[doc = "DIEPTXF3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptxf3`] module"]
pub type DIEPTXF3 = crate::Reg<dieptxf3::DIEPTXF3_SPEC>;
#[doc = ""]
pub mod dieptxf3;
#[doc = "DIEPTXF4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptxf4`] module"]
pub type DIEPTXF4 = crate::Reg<dieptxf4::DIEPTXF4_SPEC>;
#[doc = ""]
pub mod dieptxf4;
#[doc = "HCFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcfg`] module"]
pub type HCFG = crate::Reg<hcfg::HCFG_SPEC>;
#[doc = ""]
pub mod hcfg;
#[doc = "HFIR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hfir`] module"]
pub type HFIR = crate::Reg<hfir::HFIR_SPEC>;
#[doc = ""]
pub mod hfir;
#[doc = "HFNUM (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfnum::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hfnum`] module"]
pub type HFNUM = crate::Reg<hfnum::HFNUM_SPEC>;
#[doc = ""]
pub mod hfnum;
#[doc = "HPTXSTS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hptxsts`] module"]
pub type HPTXSTS = crate::Reg<hptxsts::HPTXSTS_SPEC>;
#[doc = ""]
pub mod hptxsts;
#[doc = "HAINT (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`haint`] module"]
pub type HAINT = crate::Reg<haint::HAINT_SPEC>;
#[doc = ""]
pub mod haint;
#[doc = "HAINTMSK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`haintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`haintmsk`] module"]
pub type HAINTMSK = crate::Reg<haintmsk::HAINTMSK_SPEC>;
#[doc = ""]
pub mod haintmsk;
#[doc = "HFLBADDR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hflbaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hflbaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hflbaddr`] module"]
pub type HFLBADDR = crate::Reg<hflbaddr::HFLBADDR_SPEC>;
#[doc = ""]
pub mod hflbaddr;
#[doc = "HPRT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hprt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hprt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hprt`] module"]
pub type HPRT = crate::Reg<hprt::HPRT_SPEC>;
#[doc = ""]
pub mod hprt;
#[doc = "HCCHAR0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcchar0`] module"]
pub type HCCHAR0 = crate::Reg<hcchar0::HCCHAR0_SPEC>;
#[doc = ""]
pub mod hcchar0;
#[doc = "HCINT0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcint0`] module"]
pub type HCINT0 = crate::Reg<hcint0::HCINT0_SPEC>;
#[doc = ""]
pub mod hcint0;
#[doc = "HCINTMSK0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcintmsk0`] module"]
pub type HCINTMSK0 = crate::Reg<hcintmsk0::HCINTMSK0_SPEC>;
#[doc = ""]
pub mod hcintmsk0;
#[doc = "HCTSIZ0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hctsiz0`] module"]
pub type HCTSIZ0 = crate::Reg<hctsiz0::HCTSIZ0_SPEC>;
#[doc = ""]
pub mod hctsiz0;
#[doc = "HCDMA0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcdma0`] module"]
pub type HCDMA0 = crate::Reg<hcdma0::HCDMA0_SPEC>;
#[doc = ""]
pub mod hcdma0;
#[doc = "HCDMAB0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdmab0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcdmab0`] module"]
pub type HCDMAB0 = crate::Reg<hcdmab0::HCDMAB0_SPEC>;
#[doc = ""]
pub mod hcdmab0;
#[doc = "HCCHAR1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcchar1`] module"]
pub type HCCHAR1 = crate::Reg<hcchar1::HCCHAR1_SPEC>;
#[doc = ""]
pub mod hcchar1;
#[doc = "HCINT1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcint1`] module"]
pub type HCINT1 = crate::Reg<hcint1::HCINT1_SPEC>;
#[doc = ""]
pub mod hcint1;
#[doc = "HCINTMSK1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcintmsk1`] module"]
pub type HCINTMSK1 = crate::Reg<hcintmsk1::HCINTMSK1_SPEC>;
#[doc = ""]
pub mod hcintmsk1;
#[doc = "HCTSIZ1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hctsiz1`] module"]
pub type HCTSIZ1 = crate::Reg<hctsiz1::HCTSIZ1_SPEC>;
#[doc = ""]
pub mod hctsiz1;
#[doc = "HCDMA1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcdma1`] module"]
pub type HCDMA1 = crate::Reg<hcdma1::HCDMA1_SPEC>;
#[doc = ""]
pub mod hcdma1;
#[doc = "HCDMAB1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdmab1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcdmab1`] module"]
pub type HCDMAB1 = crate::Reg<hcdmab1::HCDMAB1_SPEC>;
#[doc = ""]
pub mod hcdmab1;
#[doc = "HCCHAR2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcchar2`] module"]
pub type HCCHAR2 = crate::Reg<hcchar2::HCCHAR2_SPEC>;
#[doc = ""]
pub mod hcchar2;
#[doc = "HCINT2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcint2`] module"]
pub type HCINT2 = crate::Reg<hcint2::HCINT2_SPEC>;
#[doc = ""]
pub mod hcint2;
#[doc = "HCINTMSK2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcintmsk2`] module"]
pub type HCINTMSK2 = crate::Reg<hcintmsk2::HCINTMSK2_SPEC>;
#[doc = ""]
pub mod hcintmsk2;
#[doc = "HCTSIZ2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hctsiz2`] module"]
pub type HCTSIZ2 = crate::Reg<hctsiz2::HCTSIZ2_SPEC>;
#[doc = ""]
pub mod hctsiz2;
#[doc = "HCDMA2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcdma2`] module"]
pub type HCDMA2 = crate::Reg<hcdma2::HCDMA2_SPEC>;
#[doc = ""]
pub mod hcdma2;
#[doc = "HCDMAB2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdmab2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcdmab2`] module"]
pub type HCDMAB2 = crate::Reg<hcdmab2::HCDMAB2_SPEC>;
#[doc = ""]
pub mod hcdmab2;
#[doc = "HCCHAR3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcchar3`] module"]
pub type HCCHAR3 = crate::Reg<hcchar3::HCCHAR3_SPEC>;
#[doc = ""]
pub mod hcchar3;
#[doc = "HCINT3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcint3`] module"]
pub type HCINT3 = crate::Reg<hcint3::HCINT3_SPEC>;
#[doc = ""]
pub mod hcint3;
#[doc = "HCINTMSK3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcintmsk3`] module"]
pub type HCINTMSK3 = crate::Reg<hcintmsk3::HCINTMSK3_SPEC>;
#[doc = ""]
pub mod hcintmsk3;
#[doc = "HCTSIZ3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hctsiz3`] module"]
pub type HCTSIZ3 = crate::Reg<hctsiz3::HCTSIZ3_SPEC>;
#[doc = ""]
pub mod hctsiz3;
#[doc = "HCDMA3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcdma3`] module"]
pub type HCDMA3 = crate::Reg<hcdma3::HCDMA3_SPEC>;
#[doc = ""]
pub mod hcdma3;
#[doc = "HCDMAB3 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdmab3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcdmab3`] module"]
pub type HCDMAB3 = crate::Reg<hcdmab3::HCDMAB3_SPEC>;
#[doc = ""]
pub mod hcdmab3;
#[doc = "HCCHAR4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcchar4`] module"]
pub type HCCHAR4 = crate::Reg<hcchar4::HCCHAR4_SPEC>;
#[doc = ""]
pub mod hcchar4;
#[doc = "HCINT4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcint4`] module"]
pub type HCINT4 = crate::Reg<hcint4::HCINT4_SPEC>;
#[doc = ""]
pub mod hcint4;
#[doc = "HCINTMSK4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcintmsk4`] module"]
pub type HCINTMSK4 = crate::Reg<hcintmsk4::HCINTMSK4_SPEC>;
#[doc = ""]
pub mod hcintmsk4;
#[doc = "HCTSIZ4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hctsiz4`] module"]
pub type HCTSIZ4 = crate::Reg<hctsiz4::HCTSIZ4_SPEC>;
#[doc = ""]
pub mod hctsiz4;
#[doc = "HCDMA4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcdma4`] module"]
pub type HCDMA4 = crate::Reg<hcdma4::HCDMA4_SPEC>;
#[doc = ""]
pub mod hcdma4;
#[doc = "HCDMAB4 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdmab4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcdmab4`] module"]
pub type HCDMAB4 = crate::Reg<hcdmab4::HCDMAB4_SPEC>;
#[doc = ""]
pub mod hcdmab4;
#[doc = "HCCHAR5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcchar5`] module"]
pub type HCCHAR5 = crate::Reg<hcchar5::HCCHAR5_SPEC>;
#[doc = ""]
pub mod hcchar5;
#[doc = "HCINT5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcint5`] module"]
pub type HCINT5 = crate::Reg<hcint5::HCINT5_SPEC>;
#[doc = ""]
pub mod hcint5;
#[doc = "HCINTMSK5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcintmsk5`] module"]
pub type HCINTMSK5 = crate::Reg<hcintmsk5::HCINTMSK5_SPEC>;
#[doc = ""]
pub mod hcintmsk5;
#[doc = "HCTSIZ5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hctsiz5`] module"]
pub type HCTSIZ5 = crate::Reg<hctsiz5::HCTSIZ5_SPEC>;
#[doc = ""]
pub mod hctsiz5;
#[doc = "HCDMA5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcdma5`] module"]
pub type HCDMA5 = crate::Reg<hcdma5::HCDMA5_SPEC>;
#[doc = ""]
pub mod hcdma5;
#[doc = "HCDMAB5 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdmab5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcdmab5`] module"]
pub type HCDMAB5 = crate::Reg<hcdmab5::HCDMAB5_SPEC>;
#[doc = ""]
pub mod hcdmab5;
#[doc = "HCCHAR6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcchar6`] module"]
pub type HCCHAR6 = crate::Reg<hcchar6::HCCHAR6_SPEC>;
#[doc = ""]
pub mod hcchar6;
#[doc = "HCINT6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcint6`] module"]
pub type HCINT6 = crate::Reg<hcint6::HCINT6_SPEC>;
#[doc = ""]
pub mod hcint6;
#[doc = "HCINTMSK6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcintmsk6`] module"]
pub type HCINTMSK6 = crate::Reg<hcintmsk6::HCINTMSK6_SPEC>;
#[doc = ""]
pub mod hcintmsk6;
#[doc = "HCTSIZ6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hctsiz6`] module"]
pub type HCTSIZ6 = crate::Reg<hctsiz6::HCTSIZ6_SPEC>;
#[doc = ""]
pub mod hctsiz6;
#[doc = "HCDMA6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcdma6`] module"]
pub type HCDMA6 = crate::Reg<hcdma6::HCDMA6_SPEC>;
#[doc = ""]
pub mod hcdma6;
#[doc = "HCDMAB6 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdmab6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcdmab6`] module"]
pub type HCDMAB6 = crate::Reg<hcdmab6::HCDMAB6_SPEC>;
#[doc = ""]
pub mod hcdmab6;
#[doc = "HCCHAR7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcchar7`] module"]
pub type HCCHAR7 = crate::Reg<hcchar7::HCCHAR7_SPEC>;
#[doc = ""]
pub mod hcchar7;
#[doc = "HCINT7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcint7`] module"]
pub type HCINT7 = crate::Reg<hcint7::HCINT7_SPEC>;
#[doc = ""]
pub mod hcint7;
#[doc = "HCINTMSK7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcintmsk7`] module"]
pub type HCINTMSK7 = crate::Reg<hcintmsk7::HCINTMSK7_SPEC>;
#[doc = ""]
pub mod hcintmsk7;
#[doc = "HCTSIZ7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hctsiz7`] module"]
pub type HCTSIZ7 = crate::Reg<hctsiz7::HCTSIZ7_SPEC>;
#[doc = ""]
pub mod hctsiz7;
#[doc = "HCDMA7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcdma7`] module"]
pub type HCDMA7 = crate::Reg<hcdma7::HCDMA7_SPEC>;
#[doc = ""]
pub mod hcdma7;
#[doc = "HCDMAB7 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdmab7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcdmab7`] module"]
pub type HCDMAB7 = crate::Reg<hcdmab7::HCDMAB7_SPEC>;
#[doc = ""]
pub mod hcdmab7;
#[doc = "DCFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcfg`] module"]
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
#[doc = ""]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dctl`] module"]
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
#[doc = ""]
pub mod dctl;
#[doc = "DSTS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsts`] module"]
pub type DSTS = crate::Reg<dsts::DSTS_SPEC>;
#[doc = ""]
pub mod dsts;
#[doc = "DIEPMSK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepmsk`] module"]
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSK_SPEC>;
#[doc = ""]
pub mod diepmsk;
#[doc = "DOEPMSK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepmsk`] module"]
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSK_SPEC>;
#[doc = ""]
pub mod doepmsk;
#[doc = "DAINT (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`daint`] module"]
pub type DAINT = crate::Reg<daint::DAINT_SPEC>;
#[doc = ""]
pub mod daint;
#[doc = "DAINTMSK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`daintmsk`] module"]
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSK_SPEC>;
#[doc = ""]
pub mod daintmsk;
#[doc = "DVBUSDIS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbusdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbusdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dvbusdis`] module"]
pub type DVBUSDIS = crate::Reg<dvbusdis::DVBUSDIS_SPEC>;
#[doc = ""]
pub mod dvbusdis;
#[doc = "DVBUSPULSE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbuspulse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbuspulse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dvbuspulse`] module"]
pub type DVBUSPULSE = crate::Reg<dvbuspulse::DVBUSPULSE_SPEC>;
#[doc = ""]
pub mod dvbuspulse;
#[doc = "DTHRCTL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dthrctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dthrctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dthrctl`] module"]
pub type DTHRCTL = crate::Reg<dthrctl::DTHRCTL_SPEC>;
#[doc = ""]
pub mod dthrctl;
#[doc = "DIEPEMPMSK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepempmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepempmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepempmsk`] module"]
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>;
#[doc = ""]
pub mod diepempmsk;
#[doc = "DIEPCTL0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepctl0`] module"]
pub type DIEPCTL0 = crate::Reg<diepctl0::DIEPCTL0_SPEC>;
#[doc = ""]
pub mod diepctl0;
#[doc = "DIEPINT0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepint0`] module"]
pub type DIEPINT0 = crate::Reg<diepint0::DIEPINT0_SPEC>;
#[doc = ""]
pub mod diepint0;
#[doc = "DIEPTSIZ0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptsiz0`] module"]
pub type DIEPTSIZ0 = crate::Reg<dieptsiz0::DIEPTSIZ0_SPEC>;
#[doc = ""]
pub mod dieptsiz0;
#[doc = "DIEPDMA0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepdma0`] module"]
pub type DIEPDMA0 = crate::Reg<diepdma0::DIEPDMA0_SPEC>;
#[doc = ""]
pub mod diepdma0;
#[doc = "DTXFSTS0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dtxfsts0`] module"]
pub type DTXFSTS0 = crate::Reg<dtxfsts0::DTXFSTS0_SPEC>;
#[doc = ""]
pub mod dtxfsts0;
#[doc = "DIEPDMAB0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdmab0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepdmab0`] module"]
pub type DIEPDMAB0 = crate::Reg<diepdmab0::DIEPDMAB0_SPEC>;
#[doc = ""]
pub mod diepdmab0;
#[doc = "DIEPCTL1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepctl1`] module"]
pub type DIEPCTL1 = crate::Reg<diepctl1::DIEPCTL1_SPEC>;
#[doc = ""]
pub mod diepctl1;
#[doc = "DIEPINT1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepint1`] module"]
pub type DIEPINT1 = crate::Reg<diepint1::DIEPINT1_SPEC>;
#[doc = ""]
pub mod diepint1;
#[doc = "DIEPTSIZ1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptsiz1`] module"]
pub type DIEPTSIZ1 = crate::Reg<dieptsiz1::DIEPTSIZ1_SPEC>;
#[doc = ""]
pub mod dieptsiz1;
#[doc = "DIEPDMA1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepdma1`] module"]
pub type DIEPDMA1 = crate::Reg<diepdma1::DIEPDMA1_SPEC>;
#[doc = ""]
pub mod diepdma1;
#[doc = "DTXFSTS1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dtxfsts1`] module"]
pub type DTXFSTS1 = crate::Reg<dtxfsts1::DTXFSTS1_SPEC>;
#[doc = ""]
pub mod dtxfsts1;
#[doc = "DIEPDMAB1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdmab1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepdmab1`] module"]
pub type DIEPDMAB1 = crate::Reg<diepdmab1::DIEPDMAB1_SPEC>;
#[doc = ""]
pub mod diepdmab1;
#[doc = "DIEPCTL2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepctl2`] module"]
pub type DIEPCTL2 = crate::Reg<diepctl2::DIEPCTL2_SPEC>;
#[doc = ""]
pub mod diepctl2;
#[doc = "DIEPINT2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepint2`] module"]
pub type DIEPINT2 = crate::Reg<diepint2::DIEPINT2_SPEC>;
#[doc = ""]
pub mod diepint2;
#[doc = "DIEPTSIZ2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptsiz2`] module"]
pub type DIEPTSIZ2 = crate::Reg<dieptsiz2::DIEPTSIZ2_SPEC>;
#[doc = ""]
pub mod dieptsiz2;
#[doc = "DIEPDMA2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepdma2`] module"]
pub type DIEPDMA2 = crate::Reg<diepdma2::DIEPDMA2_SPEC>;
#[doc = ""]
pub mod diepdma2;
#[doc = "DTXFSTS2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dtxfsts2`] module"]
pub type DTXFSTS2 = crate::Reg<dtxfsts2::DTXFSTS2_SPEC>;
#[doc = ""]
pub mod dtxfsts2;
#[doc = "DIEPDMAB2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdmab2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepdmab2`] module"]
pub type DIEPDMAB2 = crate::Reg<diepdmab2::DIEPDMAB2_SPEC>;
#[doc = ""]
pub mod diepdmab2;
#[doc = "DIEPCTL3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepctl3`] module"]
pub type DIEPCTL3 = crate::Reg<diepctl3::DIEPCTL3_SPEC>;
#[doc = ""]
pub mod diepctl3;
#[doc = "DIEPINT3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepint3`] module"]
pub type DIEPINT3 = crate::Reg<diepint3::DIEPINT3_SPEC>;
#[doc = ""]
pub mod diepint3;
#[doc = "DIEPTSIZ3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptsiz3`] module"]
pub type DIEPTSIZ3 = crate::Reg<dieptsiz3::DIEPTSIZ3_SPEC>;
#[doc = ""]
pub mod dieptsiz3;
#[doc = "DIEPDMA3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepdma3`] module"]
pub type DIEPDMA3 = crate::Reg<diepdma3::DIEPDMA3_SPEC>;
#[doc = ""]
pub mod diepdma3;
#[doc = "DTXFSTS3 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dtxfsts3`] module"]
pub type DTXFSTS3 = crate::Reg<dtxfsts3::DTXFSTS3_SPEC>;
#[doc = ""]
pub mod dtxfsts3;
#[doc = "DIEPDMAB3 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdmab3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepdmab3`] module"]
pub type DIEPDMAB3 = crate::Reg<diepdmab3::DIEPDMAB3_SPEC>;
#[doc = ""]
pub mod diepdmab3;
#[doc = "DIEPCTL4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepctl4`] module"]
pub type DIEPCTL4 = crate::Reg<diepctl4::DIEPCTL4_SPEC>;
#[doc = ""]
pub mod diepctl4;
#[doc = "DIEPINT4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepint4`] module"]
pub type DIEPINT4 = crate::Reg<diepint4::DIEPINT4_SPEC>;
#[doc = ""]
pub mod diepint4;
#[doc = "DIEPTSIZ4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptsiz4`] module"]
pub type DIEPTSIZ4 = crate::Reg<dieptsiz4::DIEPTSIZ4_SPEC>;
#[doc = ""]
pub mod dieptsiz4;
#[doc = "DIEPDMA4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepdma4`] module"]
pub type DIEPDMA4 = crate::Reg<diepdma4::DIEPDMA4_SPEC>;
#[doc = ""]
pub mod diepdma4;
#[doc = "DTXFSTS4 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dtxfsts4`] module"]
pub type DTXFSTS4 = crate::Reg<dtxfsts4::DTXFSTS4_SPEC>;
#[doc = ""]
pub mod dtxfsts4;
#[doc = "DIEPDMAB4 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdmab4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepdmab4`] module"]
pub type DIEPDMAB4 = crate::Reg<diepdmab4::DIEPDMAB4_SPEC>;
#[doc = ""]
pub mod diepdmab4;
#[doc = "DIEPCTL5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepctl5`] module"]
pub type DIEPCTL5 = crate::Reg<diepctl5::DIEPCTL5_SPEC>;
#[doc = ""]
pub mod diepctl5;
#[doc = "DIEPINT5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepint5`] module"]
pub type DIEPINT5 = crate::Reg<diepint5::DIEPINT5_SPEC>;
#[doc = ""]
pub mod diepint5;
#[doc = "DIEPTSIZ5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptsiz5`] module"]
pub type DIEPTSIZ5 = crate::Reg<dieptsiz5::DIEPTSIZ5_SPEC>;
#[doc = ""]
pub mod dieptsiz5;
#[doc = "DIEPDMA5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepdma5`] module"]
pub type DIEPDMA5 = crate::Reg<diepdma5::DIEPDMA5_SPEC>;
#[doc = ""]
pub mod diepdma5;
#[doc = "DTXFSTS5 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dtxfsts5`] module"]
pub type DTXFSTS5 = crate::Reg<dtxfsts5::DTXFSTS5_SPEC>;
#[doc = ""]
pub mod dtxfsts5;
#[doc = "DIEPDMAB5 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdmab5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepdmab5`] module"]
pub type DIEPDMAB5 = crate::Reg<diepdmab5::DIEPDMAB5_SPEC>;
#[doc = ""]
pub mod diepdmab5;
#[doc = "DIEPCTL6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepctl6`] module"]
pub type DIEPCTL6 = crate::Reg<diepctl6::DIEPCTL6_SPEC>;
#[doc = ""]
pub mod diepctl6;
#[doc = "DIEPINT6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepint6`] module"]
pub type DIEPINT6 = crate::Reg<diepint6::DIEPINT6_SPEC>;
#[doc = ""]
pub mod diepint6;
#[doc = "DIEPTSIZ6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptsiz6`] module"]
pub type DIEPTSIZ6 = crate::Reg<dieptsiz6::DIEPTSIZ6_SPEC>;
#[doc = ""]
pub mod dieptsiz6;
#[doc = "DIEPDMA6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepdma6`] module"]
pub type DIEPDMA6 = crate::Reg<diepdma6::DIEPDMA6_SPEC>;
#[doc = ""]
pub mod diepdma6;
#[doc = "DTXFSTS6 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dtxfsts6`] module"]
pub type DTXFSTS6 = crate::Reg<dtxfsts6::DTXFSTS6_SPEC>;
#[doc = ""]
pub mod dtxfsts6;
#[doc = "DIEPDMAB6 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdmab6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepdmab6`] module"]
pub type DIEPDMAB6 = crate::Reg<diepdmab6::DIEPDMAB6_SPEC>;
#[doc = ""]
pub mod diepdmab6;
#[doc = "DOEPCTL0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepctl0`] module"]
pub type DOEPCTL0 = crate::Reg<doepctl0::DOEPCTL0_SPEC>;
#[doc = ""]
pub mod doepctl0;
#[doc = "DOEPINT0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepint0`] module"]
pub type DOEPINT0 = crate::Reg<doepint0::DOEPINT0_SPEC>;
#[doc = ""]
pub mod doepint0;
#[doc = "DOEPTSIZ0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doeptsiz0`] module"]
pub type DOEPTSIZ0 = crate::Reg<doeptsiz0::DOEPTSIZ0_SPEC>;
#[doc = ""]
pub mod doeptsiz0;
#[doc = "DOEPDMA0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepdma0`] module"]
pub type DOEPDMA0 = crate::Reg<doepdma0::DOEPDMA0_SPEC>;
#[doc = ""]
pub mod doepdma0;
#[doc = "DOEPDMAB0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdmab0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdmab0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepdmab0`] module"]
pub type DOEPDMAB0 = crate::Reg<doepdmab0::DOEPDMAB0_SPEC>;
#[doc = ""]
pub mod doepdmab0;
#[doc = "DOEPCTL1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepctl1`] module"]
pub type DOEPCTL1 = crate::Reg<doepctl1::DOEPCTL1_SPEC>;
#[doc = ""]
pub mod doepctl1;
#[doc = "DOEPINT1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepint1`] module"]
pub type DOEPINT1 = crate::Reg<doepint1::DOEPINT1_SPEC>;
#[doc = ""]
pub mod doepint1;
#[doc = "DOEPTSIZ1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doeptsiz1`] module"]
pub type DOEPTSIZ1 = crate::Reg<doeptsiz1::DOEPTSIZ1_SPEC>;
#[doc = ""]
pub mod doeptsiz1;
#[doc = "DOEPDMA1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepdma1`] module"]
pub type DOEPDMA1 = crate::Reg<doepdma1::DOEPDMA1_SPEC>;
#[doc = ""]
pub mod doepdma1;
#[doc = "DOEPDMAB1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdmab1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdmab1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepdmab1`] module"]
pub type DOEPDMAB1 = crate::Reg<doepdmab1::DOEPDMAB1_SPEC>;
#[doc = ""]
pub mod doepdmab1;
#[doc = "DOEPCTL2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepctl2`] module"]
pub type DOEPCTL2 = crate::Reg<doepctl2::DOEPCTL2_SPEC>;
#[doc = ""]
pub mod doepctl2;
#[doc = "DOEPINT2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepint2`] module"]
pub type DOEPINT2 = crate::Reg<doepint2::DOEPINT2_SPEC>;
#[doc = ""]
pub mod doepint2;
#[doc = "DOEPTSIZ2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doeptsiz2`] module"]
pub type DOEPTSIZ2 = crate::Reg<doeptsiz2::DOEPTSIZ2_SPEC>;
#[doc = ""]
pub mod doeptsiz2;
#[doc = "DOEPDMA2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepdma2`] module"]
pub type DOEPDMA2 = crate::Reg<doepdma2::DOEPDMA2_SPEC>;
#[doc = ""]
pub mod doepdma2;
#[doc = "DOEPDMAB2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdmab2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdmab2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepdmab2`] module"]
pub type DOEPDMAB2 = crate::Reg<doepdmab2::DOEPDMAB2_SPEC>;
#[doc = ""]
pub mod doepdmab2;
#[doc = "DOEPCTL3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepctl3`] module"]
pub type DOEPCTL3 = crate::Reg<doepctl3::DOEPCTL3_SPEC>;
#[doc = ""]
pub mod doepctl3;
#[doc = "DOEPINT3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepint3`] module"]
pub type DOEPINT3 = crate::Reg<doepint3::DOEPINT3_SPEC>;
#[doc = ""]
pub mod doepint3;
#[doc = "DOEPTSIZ3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doeptsiz3`] module"]
pub type DOEPTSIZ3 = crate::Reg<doeptsiz3::DOEPTSIZ3_SPEC>;
#[doc = ""]
pub mod doeptsiz3;
#[doc = "DOEPDMA3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepdma3`] module"]
pub type DOEPDMA3 = crate::Reg<doepdma3::DOEPDMA3_SPEC>;
#[doc = ""]
pub mod doepdma3;
#[doc = "DOEPDMAB3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdmab3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdmab3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepdmab3`] module"]
pub type DOEPDMAB3 = crate::Reg<doepdmab3::DOEPDMAB3_SPEC>;
#[doc = ""]
pub mod doepdmab3;
#[doc = "DOEPCTL4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepctl4`] module"]
pub type DOEPCTL4 = crate::Reg<doepctl4::DOEPCTL4_SPEC>;
#[doc = ""]
pub mod doepctl4;
#[doc = "DOEPINT4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepint4`] module"]
pub type DOEPINT4 = crate::Reg<doepint4::DOEPINT4_SPEC>;
#[doc = ""]
pub mod doepint4;
#[doc = "DOEPTSIZ4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doeptsiz4`] module"]
pub type DOEPTSIZ4 = crate::Reg<doeptsiz4::DOEPTSIZ4_SPEC>;
#[doc = ""]
pub mod doeptsiz4;
#[doc = "DOEPDMA4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepdma4`] module"]
pub type DOEPDMA4 = crate::Reg<doepdma4::DOEPDMA4_SPEC>;
#[doc = ""]
pub mod doepdma4;
#[doc = "DOEPDMAB4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdmab4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdmab4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepdmab4`] module"]
pub type DOEPDMAB4 = crate::Reg<doepdmab4::DOEPDMAB4_SPEC>;
#[doc = ""]
pub mod doepdmab4;
#[doc = "DOEPCTL5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepctl5`] module"]
pub type DOEPCTL5 = crate::Reg<doepctl5::DOEPCTL5_SPEC>;
#[doc = ""]
pub mod doepctl5;
#[doc = "DOEPINT5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepint5`] module"]
pub type DOEPINT5 = crate::Reg<doepint5::DOEPINT5_SPEC>;
#[doc = ""]
pub mod doepint5;
#[doc = "DOEPTSIZ5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doeptsiz5`] module"]
pub type DOEPTSIZ5 = crate::Reg<doeptsiz5::DOEPTSIZ5_SPEC>;
#[doc = ""]
pub mod doeptsiz5;
#[doc = "DOEPDMA5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepdma5`] module"]
pub type DOEPDMA5 = crate::Reg<doepdma5::DOEPDMA5_SPEC>;
#[doc = ""]
pub mod doepdma5;
#[doc = "DOEPDMAB5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdmab5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdmab5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepdmab5`] module"]
pub type DOEPDMAB5 = crate::Reg<doepdmab5::DOEPDMAB5_SPEC>;
#[doc = ""]
pub mod doepdmab5;
#[doc = "DOEPCTL6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepctl6`] module"]
pub type DOEPCTL6 = crate::Reg<doepctl6::DOEPCTL6_SPEC>;
#[doc = ""]
pub mod doepctl6;
#[doc = "DOEPINT6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepint6`] module"]
pub type DOEPINT6 = crate::Reg<doepint6::DOEPINT6_SPEC>;
#[doc = ""]
pub mod doepint6;
#[doc = "DOEPTSIZ6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doeptsiz6`] module"]
pub type DOEPTSIZ6 = crate::Reg<doeptsiz6::DOEPTSIZ6_SPEC>;
#[doc = ""]
pub mod doeptsiz6;
#[doc = "DOEPDMA6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepdma6`] module"]
pub type DOEPDMA6 = crate::Reg<doepdma6::DOEPDMA6_SPEC>;
#[doc = ""]
pub mod doepdma6;
#[doc = "DOEPDMAB6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdmab6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdmab6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepdmab6`] module"]
pub type DOEPDMAB6 = crate::Reg<doepdmab6::DOEPDMAB6_SPEC>;
#[doc = ""]
pub mod doepdmab6;
#[doc = "PCGCCTL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcgcctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcgcctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcgcctl`] module"]
pub type PCGCCTL = crate::Reg<pcgcctl::PCGCCTL_SPEC>;
#[doc = ""]
pub mod pcgcctl;
