#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    gotgctl: GOTGCTL,
    gotgint: GOTGINT,
    gahbcfg: GAHBCFG,
    gusbcfg: GUSBCFG,
    grstctl: GRSTCTL,
    gintsts: GINTSTS,
    gintmsk: GINTMSK,
    grxstsr: GRXSTSR,
    grxstsp: GRXSTSP,
    grxfsiz: GRXFSIZ,
    gnptxfsiz: GNPTXFSIZ,
    gnptxsts: GNPTXSTS,
    _reserved12: [u8; 0x10],
    gsnpsid: GSNPSID,
    ghwcfg1: GHWCFG1,
    ghwcfg2: GHWCFG2,
    ghwcfg3: GHWCFG3,
    ghwcfg4: GHWCFG4,
    _reserved17: [u8; 0x08],
    gdfifocfg: GDFIFOCFG,
    _reserved18: [u8; 0xa0],
    hptxfsiz: HPTXFSIZ,
    dieptxf1: DIEPTXF1,
    dieptxf2: DIEPTXF2,
    dieptxf3: DIEPTXF3,
    dieptxf4: DIEPTXF4,
    _reserved23: [u8; 0x02ec],
    hcfg: HCFG,
    hfir: HFIR,
    hfnum: HFNUM,
    _reserved26: [u8; 0x04],
    hptxsts: HPTXSTS,
    haint: HAINT,
    haintmsk: HAINTMSK,
    hflbaddr: HFLBADDR,
    _reserved30: [u8; 0x20],
    hprt: HPRT,
    _reserved31: [u8; 0xbc],
    hcchar0: HCCHAR0,
    _reserved32: [u8; 0x04],
    hcint0: HCINT0,
    hcintmsk0: HCINTMSK0,
    hctsiz0: HCTSIZ0,
    hcdma0: HCDMA0,
    _reserved36: [u8; 0x04],
    hcdmab0: HCDMAB0,
    hcchar1: HCCHAR1,
    _reserved38: [u8; 0x04],
    hcint1: HCINT1,
    hcintmsk1: HCINTMSK1,
    hctsiz1: HCTSIZ1,
    hcdma1: HCDMA1,
    _reserved42: [u8; 0x04],
    hcdmab1: HCDMAB1,
    hcchar2: HCCHAR2,
    _reserved44: [u8; 0x04],
    hcint2: HCINT2,
    hcintmsk2: HCINTMSK2,
    hctsiz2: HCTSIZ2,
    hcdma2: HCDMA2,
    _reserved48: [u8; 0x04],
    hcdmab2: HCDMAB2,
    hcchar3: HCCHAR3,
    _reserved50: [u8; 0x04],
    hcint3: HCINT3,
    hcintmsk3: HCINTMSK3,
    hctsiz3: HCTSIZ3,
    hcdma3: HCDMA3,
    _reserved54: [u8; 0x04],
    hcdmab3: HCDMAB3,
    hcchar4: HCCHAR4,
    _reserved56: [u8; 0x04],
    hcint4: HCINT4,
    hcintmsk4: HCINTMSK4,
    hctsiz4: HCTSIZ4,
    hcdma4: HCDMA4,
    _reserved60: [u8; 0x04],
    hcdmab4: HCDMAB4,
    hcchar5: HCCHAR5,
    _reserved62: [u8; 0x04],
    hcint5: HCINT5,
    hcintmsk5: HCINTMSK5,
    hctsiz5: HCTSIZ5,
    hcdma5: HCDMA5,
    _reserved66: [u8; 0x04],
    hcdmab5: HCDMAB5,
    hcchar6: HCCHAR6,
    _reserved68: [u8; 0x04],
    hcint6: HCINT6,
    hcintmsk6: HCINTMSK6,
    hctsiz6: HCTSIZ6,
    hcdma6: HCDMA6,
    _reserved72: [u8; 0x04],
    hcdmab6: HCDMAB6,
    hcchar7: HCCHAR7,
    _reserved74: [u8; 0x04],
    hcint7: HCINT7,
    hcintmsk7: HCINTMSK7,
    hctsiz7: HCTSIZ7,
    hcdma7: HCDMA7,
    _reserved78: [u8; 0x04],
    hcdmab7: HCDMAB7,
    _reserved79: [u8; 0x0200],
    dcfg: DCFG,
    dctl: DCTL,
    dsts: DSTS,
    _reserved82: [u8; 0x04],
    diepmsk: DIEPMSK,
    doepmsk: DOEPMSK,
    daint: DAINT,
    daintmsk: DAINTMSK,
    _reserved86: [u8; 0x08],
    dvbusdis: DVBUSDIS,
    dvbuspulse: DVBUSPULSE,
    dthrctl: DTHRCTL,
    diepempmsk: DIEPEMPMSK,
    _reserved90: [u8; 0xc8],
    diepctl0: DIEPCTL0,
    _reserved91: [u8; 0x04],
    diepint0: DIEPINT0,
    _reserved92: [u8; 0x04],
    dieptsiz0: DIEPTSIZ0,
    diepdma0: DIEPDMA0,
    dtxfsts0: DTXFSTS0,
    diepdmab0: DIEPDMAB0,
    diepctl1: DIEPCTL1,
    _reserved97: [u8; 0x04],
    diepint1: DIEPINT1,
    _reserved98: [u8; 0x04],
    dieptsiz1: DIEPTSIZ1,
    diepdma1: DIEPDMA1,
    dtxfsts1: DTXFSTS1,
    diepdmab1: DIEPDMAB1,
    diepctl2: DIEPCTL2,
    _reserved103: [u8; 0x04],
    diepint2: DIEPINT2,
    _reserved104: [u8; 0x04],
    dieptsiz2: DIEPTSIZ2,
    diepdma2: DIEPDMA2,
    dtxfsts2: DTXFSTS2,
    diepdmab2: DIEPDMAB2,
    diepctl3: DIEPCTL3,
    _reserved109: [u8; 0x04],
    diepint3: DIEPINT3,
    _reserved110: [u8; 0x04],
    dieptsiz3: DIEPTSIZ3,
    diepdma3: DIEPDMA3,
    dtxfsts3: DTXFSTS3,
    diepdmab3: DIEPDMAB3,
    diepctl4: DIEPCTL4,
    _reserved115: [u8; 0x04],
    diepint4: DIEPINT4,
    _reserved116: [u8; 0x04],
    dieptsiz4: DIEPTSIZ4,
    diepdma4: DIEPDMA4,
    dtxfsts4: DTXFSTS4,
    diepdmab4: DIEPDMAB4,
    diepctl5: DIEPCTL5,
    _reserved121: [u8; 0x04],
    diepint5: DIEPINT5,
    _reserved122: [u8; 0x04],
    dieptsiz5: DIEPTSIZ5,
    diepdma5: DIEPDMA5,
    dtxfsts5: DTXFSTS5,
    diepdmab5: DIEPDMAB5,
    diepctl6: DIEPCTL6,
    _reserved127: [u8; 0x04],
    diepint6: DIEPINT6,
    _reserved128: [u8; 0x04],
    dieptsiz6: DIEPTSIZ6,
    diepdma6: DIEPDMA6,
    dtxfsts6: DTXFSTS6,
    diepdmab6: DIEPDMAB6,
    _reserved132: [u8; 0x0120],
    doepctl0: DOEPCTL0,
    _reserved133: [u8; 0x04],
    doepint0: DOEPINT0,
    _reserved134: [u8; 0x04],
    doeptsiz0: DOEPTSIZ0,
    doepdma0: DOEPDMA0,
    _reserved136: [u8; 0x04],
    doepdmab0: DOEPDMAB0,
    doepctl1: DOEPCTL1,
    _reserved138: [u8; 0x04],
    doepint1: DOEPINT1,
    _reserved139: [u8; 0x04],
    doeptsiz1: DOEPTSIZ1,
    doepdma1: DOEPDMA1,
    _reserved141: [u8; 0x04],
    doepdmab1: DOEPDMAB1,
    doepctl2: DOEPCTL2,
    _reserved143: [u8; 0x04],
    doepint2: DOEPINT2,
    _reserved144: [u8; 0x04],
    doeptsiz2: DOEPTSIZ2,
    doepdma2: DOEPDMA2,
    _reserved146: [u8; 0x04],
    doepdmab2: DOEPDMAB2,
    doepctl3: DOEPCTL3,
    _reserved148: [u8; 0x04],
    doepint3: DOEPINT3,
    _reserved149: [u8; 0x04],
    doeptsiz3: DOEPTSIZ3,
    doepdma3: DOEPDMA3,
    _reserved151: [u8; 0x04],
    doepdmab3: DOEPDMAB3,
    doepctl4: DOEPCTL4,
    _reserved153: [u8; 0x04],
    doepint4: DOEPINT4,
    _reserved154: [u8; 0x04],
    doeptsiz4: DOEPTSIZ4,
    doepdma4: DOEPDMA4,
    _reserved156: [u8; 0x04],
    doepdmab4: DOEPDMAB4,
    doepctl5: DOEPCTL5,
    _reserved158: [u8; 0x04],
    doepint5: DOEPINT5,
    _reserved159: [u8; 0x04],
    doeptsiz5: DOEPTSIZ5,
    doepdma5: DOEPDMA5,
    _reserved161: [u8; 0x04],
    doepdmab5: DOEPDMAB5,
    doepctl6: DOEPCTL6,
    _reserved163: [u8; 0x04],
    doepint6: DOEPINT6,
    _reserved164: [u8; 0x04],
    doeptsiz6: DOEPTSIZ6,
    doepdma6: DOEPDMA6,
    _reserved166: [u8; 0x04],
    doepdmab6: DOEPDMAB6,
    _reserved167: [u8; 0x0220],
    pcgcctl: PCGCCTL,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn gotgctl(&self) -> &GOTGCTL {
        &self.gotgctl
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn gotgint(&self) -> &GOTGINT {
        &self.gotgint
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn gahbcfg(&self) -> &GAHBCFG {
        &self.gahbcfg
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn gusbcfg(&self) -> &GUSBCFG {
        &self.gusbcfg
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn grstctl(&self) -> &GRSTCTL {
        &self.grstctl
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn gintsts(&self) -> &GINTSTS {
        &self.gintsts
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn gintmsk(&self) -> &GINTMSK {
        &self.gintmsk
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn grxstsr(&self) -> &GRXSTSR {
        &self.grxstsr
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn grxstsp(&self) -> &GRXSTSP {
        &self.grxstsp
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn grxfsiz(&self) -> &GRXFSIZ {
        &self.grxfsiz
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn gnptxfsiz(&self) -> &GNPTXFSIZ {
        &self.gnptxfsiz
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn gnptxsts(&self) -> &GNPTXSTS {
        &self.gnptxsts
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn gsnpsid(&self) -> &GSNPSID {
        &self.gsnpsid
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn ghwcfg1(&self) -> &GHWCFG1 {
        &self.ghwcfg1
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn ghwcfg2(&self) -> &GHWCFG2 {
        &self.ghwcfg2
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn ghwcfg3(&self) -> &GHWCFG3 {
        &self.ghwcfg3
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn ghwcfg4(&self) -> &GHWCFG4 {
        &self.ghwcfg4
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn gdfifocfg(&self) -> &GDFIFOCFG {
        &self.gdfifocfg
    }
    #[doc = "0x100 - "]
    #[inline(always)]
    pub const fn hptxfsiz(&self) -> &HPTXFSIZ {
        &self.hptxfsiz
    }
    #[doc = "0x104 - "]
    #[inline(always)]
    pub const fn dieptxf1(&self) -> &DIEPTXF1 {
        &self.dieptxf1
    }
    #[doc = "0x108 - "]
    #[inline(always)]
    pub const fn dieptxf2(&self) -> &DIEPTXF2 {
        &self.dieptxf2
    }
    #[doc = "0x10c - "]
    #[inline(always)]
    pub const fn dieptxf3(&self) -> &DIEPTXF3 {
        &self.dieptxf3
    }
    #[doc = "0x110 - "]
    #[inline(always)]
    pub const fn dieptxf4(&self) -> &DIEPTXF4 {
        &self.dieptxf4
    }
    #[doc = "0x400 - "]
    #[inline(always)]
    pub const fn hcfg(&self) -> &HCFG {
        &self.hcfg
    }
    #[doc = "0x404 - "]
    #[inline(always)]
    pub const fn hfir(&self) -> &HFIR {
        &self.hfir
    }
    #[doc = "0x408 - "]
    #[inline(always)]
    pub const fn hfnum(&self) -> &HFNUM {
        &self.hfnum
    }
    #[doc = "0x410 - "]
    #[inline(always)]
    pub const fn hptxsts(&self) -> &HPTXSTS {
        &self.hptxsts
    }
    #[doc = "0x414 - "]
    #[inline(always)]
    pub const fn haint(&self) -> &HAINT {
        &self.haint
    }
    #[doc = "0x418 - "]
    #[inline(always)]
    pub const fn haintmsk(&self) -> &HAINTMSK {
        &self.haintmsk
    }
    #[doc = "0x41c - "]
    #[inline(always)]
    pub const fn hflbaddr(&self) -> &HFLBADDR {
        &self.hflbaddr
    }
    #[doc = "0x440 - "]
    #[inline(always)]
    pub const fn hprt(&self) -> &HPRT {
        &self.hprt
    }
    #[doc = "0x500 - "]
    #[inline(always)]
    pub const fn hcchar0(&self) -> &HCCHAR0 {
        &self.hcchar0
    }
    #[doc = "0x508 - "]
    #[inline(always)]
    pub const fn hcint0(&self) -> &HCINT0 {
        &self.hcint0
    }
    #[doc = "0x50c - "]
    #[inline(always)]
    pub const fn hcintmsk0(&self) -> &HCINTMSK0 {
        &self.hcintmsk0
    }
    #[doc = "0x510 - "]
    #[inline(always)]
    pub const fn hctsiz0(&self) -> &HCTSIZ0 {
        &self.hctsiz0
    }
    #[doc = "0x514 - "]
    #[inline(always)]
    pub const fn hcdma0(&self) -> &HCDMA0 {
        &self.hcdma0
    }
    #[doc = "0x51c - "]
    #[inline(always)]
    pub const fn hcdmab0(&self) -> &HCDMAB0 {
        &self.hcdmab0
    }
    #[doc = "0x520 - "]
    #[inline(always)]
    pub const fn hcchar1(&self) -> &HCCHAR1 {
        &self.hcchar1
    }
    #[doc = "0x528 - "]
    #[inline(always)]
    pub const fn hcint1(&self) -> &HCINT1 {
        &self.hcint1
    }
    #[doc = "0x52c - "]
    #[inline(always)]
    pub const fn hcintmsk1(&self) -> &HCINTMSK1 {
        &self.hcintmsk1
    }
    #[doc = "0x530 - "]
    #[inline(always)]
    pub const fn hctsiz1(&self) -> &HCTSIZ1 {
        &self.hctsiz1
    }
    #[doc = "0x534 - "]
    #[inline(always)]
    pub const fn hcdma1(&self) -> &HCDMA1 {
        &self.hcdma1
    }
    #[doc = "0x53c - "]
    #[inline(always)]
    pub const fn hcdmab1(&self) -> &HCDMAB1 {
        &self.hcdmab1
    }
    #[doc = "0x540 - "]
    #[inline(always)]
    pub const fn hcchar2(&self) -> &HCCHAR2 {
        &self.hcchar2
    }
    #[doc = "0x548 - "]
    #[inline(always)]
    pub const fn hcint2(&self) -> &HCINT2 {
        &self.hcint2
    }
    #[doc = "0x54c - "]
    #[inline(always)]
    pub const fn hcintmsk2(&self) -> &HCINTMSK2 {
        &self.hcintmsk2
    }
    #[doc = "0x550 - "]
    #[inline(always)]
    pub const fn hctsiz2(&self) -> &HCTSIZ2 {
        &self.hctsiz2
    }
    #[doc = "0x554 - "]
    #[inline(always)]
    pub const fn hcdma2(&self) -> &HCDMA2 {
        &self.hcdma2
    }
    #[doc = "0x55c - "]
    #[inline(always)]
    pub const fn hcdmab2(&self) -> &HCDMAB2 {
        &self.hcdmab2
    }
    #[doc = "0x560 - "]
    #[inline(always)]
    pub const fn hcchar3(&self) -> &HCCHAR3 {
        &self.hcchar3
    }
    #[doc = "0x568 - "]
    #[inline(always)]
    pub const fn hcint3(&self) -> &HCINT3 {
        &self.hcint3
    }
    #[doc = "0x56c - "]
    #[inline(always)]
    pub const fn hcintmsk3(&self) -> &HCINTMSK3 {
        &self.hcintmsk3
    }
    #[doc = "0x570 - "]
    #[inline(always)]
    pub const fn hctsiz3(&self) -> &HCTSIZ3 {
        &self.hctsiz3
    }
    #[doc = "0x574 - "]
    #[inline(always)]
    pub const fn hcdma3(&self) -> &HCDMA3 {
        &self.hcdma3
    }
    #[doc = "0x57c - "]
    #[inline(always)]
    pub const fn hcdmab3(&self) -> &HCDMAB3 {
        &self.hcdmab3
    }
    #[doc = "0x580 - "]
    #[inline(always)]
    pub const fn hcchar4(&self) -> &HCCHAR4 {
        &self.hcchar4
    }
    #[doc = "0x588 - "]
    #[inline(always)]
    pub const fn hcint4(&self) -> &HCINT4 {
        &self.hcint4
    }
    #[doc = "0x58c - "]
    #[inline(always)]
    pub const fn hcintmsk4(&self) -> &HCINTMSK4 {
        &self.hcintmsk4
    }
    #[doc = "0x590 - "]
    #[inline(always)]
    pub const fn hctsiz4(&self) -> &HCTSIZ4 {
        &self.hctsiz4
    }
    #[doc = "0x594 - "]
    #[inline(always)]
    pub const fn hcdma4(&self) -> &HCDMA4 {
        &self.hcdma4
    }
    #[doc = "0x59c - "]
    #[inline(always)]
    pub const fn hcdmab4(&self) -> &HCDMAB4 {
        &self.hcdmab4
    }
    #[doc = "0x5a0 - "]
    #[inline(always)]
    pub const fn hcchar5(&self) -> &HCCHAR5 {
        &self.hcchar5
    }
    #[doc = "0x5a8 - "]
    #[inline(always)]
    pub const fn hcint5(&self) -> &HCINT5 {
        &self.hcint5
    }
    #[doc = "0x5ac - "]
    #[inline(always)]
    pub const fn hcintmsk5(&self) -> &HCINTMSK5 {
        &self.hcintmsk5
    }
    #[doc = "0x5b0 - "]
    #[inline(always)]
    pub const fn hctsiz5(&self) -> &HCTSIZ5 {
        &self.hctsiz5
    }
    #[doc = "0x5b4 - "]
    #[inline(always)]
    pub const fn hcdma5(&self) -> &HCDMA5 {
        &self.hcdma5
    }
    #[doc = "0x5bc - "]
    #[inline(always)]
    pub const fn hcdmab5(&self) -> &HCDMAB5 {
        &self.hcdmab5
    }
    #[doc = "0x5c0 - "]
    #[inline(always)]
    pub const fn hcchar6(&self) -> &HCCHAR6 {
        &self.hcchar6
    }
    #[doc = "0x5c8 - "]
    #[inline(always)]
    pub const fn hcint6(&self) -> &HCINT6 {
        &self.hcint6
    }
    #[doc = "0x5cc - "]
    #[inline(always)]
    pub const fn hcintmsk6(&self) -> &HCINTMSK6 {
        &self.hcintmsk6
    }
    #[doc = "0x5d0 - "]
    #[inline(always)]
    pub const fn hctsiz6(&self) -> &HCTSIZ6 {
        &self.hctsiz6
    }
    #[doc = "0x5d4 - "]
    #[inline(always)]
    pub const fn hcdma6(&self) -> &HCDMA6 {
        &self.hcdma6
    }
    #[doc = "0x5dc - "]
    #[inline(always)]
    pub const fn hcdmab6(&self) -> &HCDMAB6 {
        &self.hcdmab6
    }
    #[doc = "0x5e0 - "]
    #[inline(always)]
    pub const fn hcchar7(&self) -> &HCCHAR7 {
        &self.hcchar7
    }
    #[doc = "0x5e8 - "]
    #[inline(always)]
    pub const fn hcint7(&self) -> &HCINT7 {
        &self.hcint7
    }
    #[doc = "0x5ec - "]
    #[inline(always)]
    pub const fn hcintmsk7(&self) -> &HCINTMSK7 {
        &self.hcintmsk7
    }
    #[doc = "0x5f0 - "]
    #[inline(always)]
    pub const fn hctsiz7(&self) -> &HCTSIZ7 {
        &self.hctsiz7
    }
    #[doc = "0x5f4 - "]
    #[inline(always)]
    pub const fn hcdma7(&self) -> &HCDMA7 {
        &self.hcdma7
    }
    #[doc = "0x5fc - "]
    #[inline(always)]
    pub const fn hcdmab7(&self) -> &HCDMAB7 {
        &self.hcdmab7
    }
    #[doc = "0x800 - "]
    #[inline(always)]
    pub const fn dcfg(&self) -> &DCFG {
        &self.dcfg
    }
    #[doc = "0x804 - "]
    #[inline(always)]
    pub const fn dctl(&self) -> &DCTL {
        &self.dctl
    }
    #[doc = "0x808 - "]
    #[inline(always)]
    pub const fn dsts(&self) -> &DSTS {
        &self.dsts
    }
    #[doc = "0x810 - "]
    #[inline(always)]
    pub const fn diepmsk(&self) -> &DIEPMSK {
        &self.diepmsk
    }
    #[doc = "0x814 - "]
    #[inline(always)]
    pub const fn doepmsk(&self) -> &DOEPMSK {
        &self.doepmsk
    }
    #[doc = "0x818 - "]
    #[inline(always)]
    pub const fn daint(&self) -> &DAINT {
        &self.daint
    }
    #[doc = "0x81c - "]
    #[inline(always)]
    pub const fn daintmsk(&self) -> &DAINTMSK {
        &self.daintmsk
    }
    #[doc = "0x828 - "]
    #[inline(always)]
    pub const fn dvbusdis(&self) -> &DVBUSDIS {
        &self.dvbusdis
    }
    #[doc = "0x82c - "]
    #[inline(always)]
    pub const fn dvbuspulse(&self) -> &DVBUSPULSE {
        &self.dvbuspulse
    }
    #[doc = "0x830 - "]
    #[inline(always)]
    pub const fn dthrctl(&self) -> &DTHRCTL {
        &self.dthrctl
    }
    #[doc = "0x834 - "]
    #[inline(always)]
    pub const fn diepempmsk(&self) -> &DIEPEMPMSK {
        &self.diepempmsk
    }
    #[doc = "0x900 - "]
    #[inline(always)]
    pub const fn diepctl0(&self) -> &DIEPCTL0 {
        &self.diepctl0
    }
    #[doc = "0x908 - "]
    #[inline(always)]
    pub const fn diepint0(&self) -> &DIEPINT0 {
        &self.diepint0
    }
    #[doc = "0x910 - "]
    #[inline(always)]
    pub const fn dieptsiz0(&self) -> &DIEPTSIZ0 {
        &self.dieptsiz0
    }
    #[doc = "0x914 - "]
    #[inline(always)]
    pub const fn diepdma0(&self) -> &DIEPDMA0 {
        &self.diepdma0
    }
    #[doc = "0x918 - "]
    #[inline(always)]
    pub const fn dtxfsts0(&self) -> &DTXFSTS0 {
        &self.dtxfsts0
    }
    #[doc = "0x91c - "]
    #[inline(always)]
    pub const fn diepdmab0(&self) -> &DIEPDMAB0 {
        &self.diepdmab0
    }
    #[doc = "0x920 - "]
    #[inline(always)]
    pub const fn diepctl1(&self) -> &DIEPCTL1 {
        &self.diepctl1
    }
    #[doc = "0x928 - "]
    #[inline(always)]
    pub const fn diepint1(&self) -> &DIEPINT1 {
        &self.diepint1
    }
    #[doc = "0x930 - "]
    #[inline(always)]
    pub const fn dieptsiz1(&self) -> &DIEPTSIZ1 {
        &self.dieptsiz1
    }
    #[doc = "0x934 - "]
    #[inline(always)]
    pub const fn diepdma1(&self) -> &DIEPDMA1 {
        &self.diepdma1
    }
    #[doc = "0x938 - "]
    #[inline(always)]
    pub const fn dtxfsts1(&self) -> &DTXFSTS1 {
        &self.dtxfsts1
    }
    #[doc = "0x93c - "]
    #[inline(always)]
    pub const fn diepdmab1(&self) -> &DIEPDMAB1 {
        &self.diepdmab1
    }
    #[doc = "0x940 - "]
    #[inline(always)]
    pub const fn diepctl2(&self) -> &DIEPCTL2 {
        &self.diepctl2
    }
    #[doc = "0x948 - "]
    #[inline(always)]
    pub const fn diepint2(&self) -> &DIEPINT2 {
        &self.diepint2
    }
    #[doc = "0x950 - "]
    #[inline(always)]
    pub const fn dieptsiz2(&self) -> &DIEPTSIZ2 {
        &self.dieptsiz2
    }
    #[doc = "0x954 - "]
    #[inline(always)]
    pub const fn diepdma2(&self) -> &DIEPDMA2 {
        &self.diepdma2
    }
    #[doc = "0x958 - "]
    #[inline(always)]
    pub const fn dtxfsts2(&self) -> &DTXFSTS2 {
        &self.dtxfsts2
    }
    #[doc = "0x95c - "]
    #[inline(always)]
    pub const fn diepdmab2(&self) -> &DIEPDMAB2 {
        &self.diepdmab2
    }
    #[doc = "0x960 - "]
    #[inline(always)]
    pub const fn diepctl3(&self) -> &DIEPCTL3 {
        &self.diepctl3
    }
    #[doc = "0x968 - "]
    #[inline(always)]
    pub const fn diepint3(&self) -> &DIEPINT3 {
        &self.diepint3
    }
    #[doc = "0x970 - "]
    #[inline(always)]
    pub const fn dieptsiz3(&self) -> &DIEPTSIZ3 {
        &self.dieptsiz3
    }
    #[doc = "0x974 - "]
    #[inline(always)]
    pub const fn diepdma3(&self) -> &DIEPDMA3 {
        &self.diepdma3
    }
    #[doc = "0x978 - "]
    #[inline(always)]
    pub const fn dtxfsts3(&self) -> &DTXFSTS3 {
        &self.dtxfsts3
    }
    #[doc = "0x97c - "]
    #[inline(always)]
    pub const fn diepdmab3(&self) -> &DIEPDMAB3 {
        &self.diepdmab3
    }
    #[doc = "0x980 - "]
    #[inline(always)]
    pub const fn diepctl4(&self) -> &DIEPCTL4 {
        &self.diepctl4
    }
    #[doc = "0x988 - "]
    #[inline(always)]
    pub const fn diepint4(&self) -> &DIEPINT4 {
        &self.diepint4
    }
    #[doc = "0x990 - "]
    #[inline(always)]
    pub const fn dieptsiz4(&self) -> &DIEPTSIZ4 {
        &self.dieptsiz4
    }
    #[doc = "0x994 - "]
    #[inline(always)]
    pub const fn diepdma4(&self) -> &DIEPDMA4 {
        &self.diepdma4
    }
    #[doc = "0x998 - "]
    #[inline(always)]
    pub const fn dtxfsts4(&self) -> &DTXFSTS4 {
        &self.dtxfsts4
    }
    #[doc = "0x99c - "]
    #[inline(always)]
    pub const fn diepdmab4(&self) -> &DIEPDMAB4 {
        &self.diepdmab4
    }
    #[doc = "0x9a0 - "]
    #[inline(always)]
    pub const fn diepctl5(&self) -> &DIEPCTL5 {
        &self.diepctl5
    }
    #[doc = "0x9a8 - "]
    #[inline(always)]
    pub const fn diepint5(&self) -> &DIEPINT5 {
        &self.diepint5
    }
    #[doc = "0x9b0 - "]
    #[inline(always)]
    pub const fn dieptsiz5(&self) -> &DIEPTSIZ5 {
        &self.dieptsiz5
    }
    #[doc = "0x9b4 - "]
    #[inline(always)]
    pub const fn diepdma5(&self) -> &DIEPDMA5 {
        &self.diepdma5
    }
    #[doc = "0x9b8 - "]
    #[inline(always)]
    pub const fn dtxfsts5(&self) -> &DTXFSTS5 {
        &self.dtxfsts5
    }
    #[doc = "0x9bc - "]
    #[inline(always)]
    pub const fn diepdmab5(&self) -> &DIEPDMAB5 {
        &self.diepdmab5
    }
    #[doc = "0x9c0 - "]
    #[inline(always)]
    pub const fn diepctl6(&self) -> &DIEPCTL6 {
        &self.diepctl6
    }
    #[doc = "0x9c8 - "]
    #[inline(always)]
    pub const fn diepint6(&self) -> &DIEPINT6 {
        &self.diepint6
    }
    #[doc = "0x9d0 - "]
    #[inline(always)]
    pub const fn dieptsiz6(&self) -> &DIEPTSIZ6 {
        &self.dieptsiz6
    }
    #[doc = "0x9d4 - "]
    #[inline(always)]
    pub const fn diepdma6(&self) -> &DIEPDMA6 {
        &self.diepdma6
    }
    #[doc = "0x9d8 - "]
    #[inline(always)]
    pub const fn dtxfsts6(&self) -> &DTXFSTS6 {
        &self.dtxfsts6
    }
    #[doc = "0x9dc - "]
    #[inline(always)]
    pub const fn diepdmab6(&self) -> &DIEPDMAB6 {
        &self.diepdmab6
    }
    #[doc = "0xb00 - "]
    #[inline(always)]
    pub const fn doepctl0(&self) -> &DOEPCTL0 {
        &self.doepctl0
    }
    #[doc = "0xb08 - "]
    #[inline(always)]
    pub const fn doepint0(&self) -> &DOEPINT0 {
        &self.doepint0
    }
    #[doc = "0xb10 - "]
    #[inline(always)]
    pub const fn doeptsiz0(&self) -> &DOEPTSIZ0 {
        &self.doeptsiz0
    }
    #[doc = "0xb14 - "]
    #[inline(always)]
    pub const fn doepdma0(&self) -> &DOEPDMA0 {
        &self.doepdma0
    }
    #[doc = "0xb1c - "]
    #[inline(always)]
    pub const fn doepdmab0(&self) -> &DOEPDMAB0 {
        &self.doepdmab0
    }
    #[doc = "0xb20 - "]
    #[inline(always)]
    pub const fn doepctl1(&self) -> &DOEPCTL1 {
        &self.doepctl1
    }
    #[doc = "0xb28 - "]
    #[inline(always)]
    pub const fn doepint1(&self) -> &DOEPINT1 {
        &self.doepint1
    }
    #[doc = "0xb30 - "]
    #[inline(always)]
    pub const fn doeptsiz1(&self) -> &DOEPTSIZ1 {
        &self.doeptsiz1
    }
    #[doc = "0xb34 - "]
    #[inline(always)]
    pub const fn doepdma1(&self) -> &DOEPDMA1 {
        &self.doepdma1
    }
    #[doc = "0xb3c - "]
    #[inline(always)]
    pub const fn doepdmab1(&self) -> &DOEPDMAB1 {
        &self.doepdmab1
    }
    #[doc = "0xb40 - "]
    #[inline(always)]
    pub const fn doepctl2(&self) -> &DOEPCTL2 {
        &self.doepctl2
    }
    #[doc = "0xb48 - "]
    #[inline(always)]
    pub const fn doepint2(&self) -> &DOEPINT2 {
        &self.doepint2
    }
    #[doc = "0xb50 - "]
    #[inline(always)]
    pub const fn doeptsiz2(&self) -> &DOEPTSIZ2 {
        &self.doeptsiz2
    }
    #[doc = "0xb54 - "]
    #[inline(always)]
    pub const fn doepdma2(&self) -> &DOEPDMA2 {
        &self.doepdma2
    }
    #[doc = "0xb5c - "]
    #[inline(always)]
    pub const fn doepdmab2(&self) -> &DOEPDMAB2 {
        &self.doepdmab2
    }
    #[doc = "0xb60 - "]
    #[inline(always)]
    pub const fn doepctl3(&self) -> &DOEPCTL3 {
        &self.doepctl3
    }
    #[doc = "0xb68 - "]
    #[inline(always)]
    pub const fn doepint3(&self) -> &DOEPINT3 {
        &self.doepint3
    }
    #[doc = "0xb70 - "]
    #[inline(always)]
    pub const fn doeptsiz3(&self) -> &DOEPTSIZ3 {
        &self.doeptsiz3
    }
    #[doc = "0xb74 - "]
    #[inline(always)]
    pub const fn doepdma3(&self) -> &DOEPDMA3 {
        &self.doepdma3
    }
    #[doc = "0xb7c - "]
    #[inline(always)]
    pub const fn doepdmab3(&self) -> &DOEPDMAB3 {
        &self.doepdmab3
    }
    #[doc = "0xb80 - "]
    #[inline(always)]
    pub const fn doepctl4(&self) -> &DOEPCTL4 {
        &self.doepctl4
    }
    #[doc = "0xb88 - "]
    #[inline(always)]
    pub const fn doepint4(&self) -> &DOEPINT4 {
        &self.doepint4
    }
    #[doc = "0xb90 - "]
    #[inline(always)]
    pub const fn doeptsiz4(&self) -> &DOEPTSIZ4 {
        &self.doeptsiz4
    }
    #[doc = "0xb94 - "]
    #[inline(always)]
    pub const fn doepdma4(&self) -> &DOEPDMA4 {
        &self.doepdma4
    }
    #[doc = "0xb9c - "]
    #[inline(always)]
    pub const fn doepdmab4(&self) -> &DOEPDMAB4 {
        &self.doepdmab4
    }
    #[doc = "0xba0 - "]
    #[inline(always)]
    pub const fn doepctl5(&self) -> &DOEPCTL5 {
        &self.doepctl5
    }
    #[doc = "0xba8 - "]
    #[inline(always)]
    pub const fn doepint5(&self) -> &DOEPINT5 {
        &self.doepint5
    }
    #[doc = "0xbb0 - "]
    #[inline(always)]
    pub const fn doeptsiz5(&self) -> &DOEPTSIZ5 {
        &self.doeptsiz5
    }
    #[doc = "0xbb4 - "]
    #[inline(always)]
    pub const fn doepdma5(&self) -> &DOEPDMA5 {
        &self.doepdma5
    }
    #[doc = "0xbbc - "]
    #[inline(always)]
    pub const fn doepdmab5(&self) -> &DOEPDMAB5 {
        &self.doepdmab5
    }
    #[doc = "0xbc0 - "]
    #[inline(always)]
    pub const fn doepctl6(&self) -> &DOEPCTL6 {
        &self.doepctl6
    }
    #[doc = "0xbc8 - "]
    #[inline(always)]
    pub const fn doepint6(&self) -> &DOEPINT6 {
        &self.doepint6
    }
    #[doc = "0xbd0 - "]
    #[inline(always)]
    pub const fn doeptsiz6(&self) -> &DOEPTSIZ6 {
        &self.doeptsiz6
    }
    #[doc = "0xbd4 - "]
    #[inline(always)]
    pub const fn doepdma6(&self) -> &DOEPDMA6 {
        &self.doepdma6
    }
    #[doc = "0xbdc - "]
    #[inline(always)]
    pub const fn doepdmab6(&self) -> &DOEPDMAB6 {
        &self.doepdmab6
    }
    #[doc = "0xe00 - "]
    #[inline(always)]
    pub const fn pcgcctl(&self) -> &PCGCCTL {
        &self.pcgcctl
    }
}
#[doc = "GOTGCTL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gotgctl`] module"]
pub type GOTGCTL = crate::Reg<gotgctl::GOTGCTL_SPEC>;
#[doc = ""]
pub mod gotgctl;
#[doc = "GOTGINT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gotgint`] module"]
pub type GOTGINT = crate::Reg<gotgint::GOTGINT_SPEC>;
#[doc = ""]
pub mod gotgint;
#[doc = "GAHBCFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gahbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gahbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gahbcfg`] module"]
pub type GAHBCFG = crate::Reg<gahbcfg::GAHBCFG_SPEC>;
#[doc = ""]
pub mod gahbcfg;
#[doc = "GUSBCFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gusbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gusbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gusbcfg`] module"]
pub type GUSBCFG = crate::Reg<gusbcfg::GUSBCFG_SPEC>;
#[doc = ""]
pub mod gusbcfg;
#[doc = "GRSTCTL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grstctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grstctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grstctl`] module"]
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTL_SPEC>;
#[doc = ""]
pub mod grstctl;
#[doc = "GINTSTS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintsts`] module"]
pub type GINTSTS = crate::Reg<gintsts::GINTSTS_SPEC>;
#[doc = ""]
pub mod gintsts;
#[doc = "GINTMSK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintmsk`] module"]
pub type GINTMSK = crate::Reg<gintmsk::GINTMSK_SPEC>;
#[doc = ""]
pub mod gintmsk;
#[doc = "GRXSTSR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsr`] module"]
pub type GRXSTSR = crate::Reg<grxstsr::GRXSTSR_SPEC>;
#[doc = ""]
pub mod grxstsr;
#[doc = "GRXSTSP (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsp`] module"]
pub type GRXSTSP = crate::Reg<grxstsp::GRXSTSP_SPEC>;
#[doc = ""]
pub mod grxstsp;
#[doc = "GRXFSIZ (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxfsiz`] module"]
pub type GRXFSIZ = crate::Reg<grxfsiz::GRXFSIZ_SPEC>;
#[doc = ""]
pub mod grxfsiz;
#[doc = "GNPTXFSIZ (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gnptxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gnptxfsiz`] module"]
pub type GNPTXFSIZ = crate::Reg<gnptxfsiz::GNPTXFSIZ_SPEC>;
#[doc = ""]
pub mod gnptxfsiz;
#[doc = "GNPTXSTS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gnptxsts`] module"]
pub type GNPTXSTS = crate::Reg<gnptxsts::GNPTXSTS_SPEC>;
#[doc = ""]
pub mod gnptxsts;
#[doc = "GSNPSID (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gsnpsid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsnpsid`] module"]
pub type GSNPSID = crate::Reg<gsnpsid::GSNPSID_SPEC>;
#[doc = ""]
pub mod gsnpsid;
#[doc = "GHWCFG1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghwcfg1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghwcfg1`] module"]
pub type GHWCFG1 = crate::Reg<ghwcfg1::GHWCFG1_SPEC>;
#[doc = ""]
pub mod ghwcfg1;
#[doc = "GHWCFG2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghwcfg2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghwcfg2`] module"]
pub type GHWCFG2 = crate::Reg<ghwcfg2::GHWCFG2_SPEC>;
#[doc = ""]
pub mod ghwcfg2;
#[doc = "GHWCFG3 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghwcfg3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghwcfg3`] module"]
pub type GHWCFG3 = crate::Reg<ghwcfg3::GHWCFG3_SPEC>;
#[doc = ""]
pub mod ghwcfg3;
#[doc = "GHWCFG4 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghwcfg4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghwcfg4`] module"]
pub type GHWCFG4 = crate::Reg<ghwcfg4::GHWCFG4_SPEC>;
#[doc = ""]
pub mod ghwcfg4;
#[doc = "GDFIFOCFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdfifocfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdfifocfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdfifocfg`] module"]
pub type GDFIFOCFG = crate::Reg<gdfifocfg::GDFIFOCFG_SPEC>;
#[doc = ""]
pub mod gdfifocfg;
#[doc = "HPTXFSIZ (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hptxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptxfsiz`] module"]
pub type HPTXFSIZ = crate::Reg<hptxfsiz::HPTXFSIZ_SPEC>;
#[doc = ""]
pub mod hptxfsiz;
#[doc = "DIEPTXF1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf1`] module"]
pub type DIEPTXF1 = crate::Reg<dieptxf1::DIEPTXF1_SPEC>;
#[doc = ""]
pub mod dieptxf1;
#[doc = "DIEPTXF2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf2`] module"]
pub type DIEPTXF2 = crate::Reg<dieptxf2::DIEPTXF2_SPEC>;
#[doc = ""]
pub mod dieptxf2;
#[doc = "DIEPTXF3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf3`] module"]
pub type DIEPTXF3 = crate::Reg<dieptxf3::DIEPTXF3_SPEC>;
#[doc = ""]
pub mod dieptxf3;
#[doc = "DIEPTXF4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf4`] module"]
pub type DIEPTXF4 = crate::Reg<dieptxf4::DIEPTXF4_SPEC>;
#[doc = ""]
pub mod dieptxf4;
#[doc = "HCFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcfg`] module"]
pub type HCFG = crate::Reg<hcfg::HCFG_SPEC>;
#[doc = ""]
pub mod hcfg;
#[doc = "HFIR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfir`] module"]
pub type HFIR = crate::Reg<hfir::HFIR_SPEC>;
#[doc = ""]
pub mod hfir;
#[doc = "HFNUM (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfnum::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfnum`] module"]
pub type HFNUM = crate::Reg<hfnum::HFNUM_SPEC>;
#[doc = ""]
pub mod hfnum;
#[doc = "HPTXSTS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptxsts`] module"]
pub type HPTXSTS = crate::Reg<hptxsts::HPTXSTS_SPEC>;
#[doc = ""]
pub mod hptxsts;
#[doc = "HAINT (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haint`] module"]
pub type HAINT = crate::Reg<haint::HAINT_SPEC>;
#[doc = ""]
pub mod haint;
#[doc = "HAINTMSK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`haintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haintmsk`] module"]
pub type HAINTMSK = crate::Reg<haintmsk::HAINTMSK_SPEC>;
#[doc = ""]
pub mod haintmsk;
#[doc = "HFLBADDR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hflbaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hflbaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hflbaddr`] module"]
pub type HFLBADDR = crate::Reg<hflbaddr::HFLBADDR_SPEC>;
#[doc = ""]
pub mod hflbaddr;
#[doc = "HPRT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hprt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hprt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hprt`] module"]
pub type HPRT = crate::Reg<hprt::HPRT_SPEC>;
#[doc = ""]
pub mod hprt;
#[doc = "HCCHAR0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar0`] module"]
pub type HCCHAR0 = crate::Reg<hcchar0::HCCHAR0_SPEC>;
#[doc = ""]
pub mod hcchar0;
#[doc = "HCINT0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint0`] module"]
pub type HCINT0 = crate::Reg<hcint0::HCINT0_SPEC>;
#[doc = ""]
pub mod hcint0;
#[doc = "HCINTMSK0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk0`] module"]
pub type HCINTMSK0 = crate::Reg<hcintmsk0::HCINTMSK0_SPEC>;
#[doc = ""]
pub mod hcintmsk0;
#[doc = "HCTSIZ0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz0`] module"]
pub type HCTSIZ0 = crate::Reg<hctsiz0::HCTSIZ0_SPEC>;
#[doc = ""]
pub mod hctsiz0;
#[doc = "HCDMA0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma0`] module"]
pub type HCDMA0 = crate::Reg<hcdma0::HCDMA0_SPEC>;
#[doc = ""]
pub mod hcdma0;
#[doc = "HCDMAB0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdmab0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdmab0`] module"]
pub type HCDMAB0 = crate::Reg<hcdmab0::HCDMAB0_SPEC>;
#[doc = ""]
pub mod hcdmab0;
#[doc = "HCCHAR1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar1`] module"]
pub type HCCHAR1 = crate::Reg<hcchar1::HCCHAR1_SPEC>;
#[doc = ""]
pub mod hcchar1;
#[doc = "HCINT1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint1`] module"]
pub type HCINT1 = crate::Reg<hcint1::HCINT1_SPEC>;
#[doc = ""]
pub mod hcint1;
#[doc = "HCINTMSK1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk1`] module"]
pub type HCINTMSK1 = crate::Reg<hcintmsk1::HCINTMSK1_SPEC>;
#[doc = ""]
pub mod hcintmsk1;
#[doc = "HCTSIZ1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz1`] module"]
pub type HCTSIZ1 = crate::Reg<hctsiz1::HCTSIZ1_SPEC>;
#[doc = ""]
pub mod hctsiz1;
#[doc = "HCDMA1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma1`] module"]
pub type HCDMA1 = crate::Reg<hcdma1::HCDMA1_SPEC>;
#[doc = ""]
pub mod hcdma1;
#[doc = "HCDMAB1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdmab1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdmab1`] module"]
pub type HCDMAB1 = crate::Reg<hcdmab1::HCDMAB1_SPEC>;
#[doc = ""]
pub mod hcdmab1;
#[doc = "HCCHAR2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar2`] module"]
pub type HCCHAR2 = crate::Reg<hcchar2::HCCHAR2_SPEC>;
#[doc = ""]
pub mod hcchar2;
#[doc = "HCINT2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint2`] module"]
pub type HCINT2 = crate::Reg<hcint2::HCINT2_SPEC>;
#[doc = ""]
pub mod hcint2;
#[doc = "HCINTMSK2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk2`] module"]
pub type HCINTMSK2 = crate::Reg<hcintmsk2::HCINTMSK2_SPEC>;
#[doc = ""]
pub mod hcintmsk2;
#[doc = "HCTSIZ2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz2`] module"]
pub type HCTSIZ2 = crate::Reg<hctsiz2::HCTSIZ2_SPEC>;
#[doc = ""]
pub mod hctsiz2;
#[doc = "HCDMA2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma2`] module"]
pub type HCDMA2 = crate::Reg<hcdma2::HCDMA2_SPEC>;
#[doc = ""]
pub mod hcdma2;
#[doc = "HCDMAB2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdmab2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdmab2`] module"]
pub type HCDMAB2 = crate::Reg<hcdmab2::HCDMAB2_SPEC>;
#[doc = ""]
pub mod hcdmab2;
#[doc = "HCCHAR3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar3`] module"]
pub type HCCHAR3 = crate::Reg<hcchar3::HCCHAR3_SPEC>;
#[doc = ""]
pub mod hcchar3;
#[doc = "HCINT3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint3`] module"]
pub type HCINT3 = crate::Reg<hcint3::HCINT3_SPEC>;
#[doc = ""]
pub mod hcint3;
#[doc = "HCINTMSK3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk3`] module"]
pub type HCINTMSK3 = crate::Reg<hcintmsk3::HCINTMSK3_SPEC>;
#[doc = ""]
pub mod hcintmsk3;
#[doc = "HCTSIZ3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz3`] module"]
pub type HCTSIZ3 = crate::Reg<hctsiz3::HCTSIZ3_SPEC>;
#[doc = ""]
pub mod hctsiz3;
#[doc = "HCDMA3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma3`] module"]
pub type HCDMA3 = crate::Reg<hcdma3::HCDMA3_SPEC>;
#[doc = ""]
pub mod hcdma3;
#[doc = "HCDMAB3 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdmab3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdmab3`] module"]
pub type HCDMAB3 = crate::Reg<hcdmab3::HCDMAB3_SPEC>;
#[doc = ""]
pub mod hcdmab3;
#[doc = "HCCHAR4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar4`] module"]
pub type HCCHAR4 = crate::Reg<hcchar4::HCCHAR4_SPEC>;
#[doc = ""]
pub mod hcchar4;
#[doc = "HCINT4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint4`] module"]
pub type HCINT4 = crate::Reg<hcint4::HCINT4_SPEC>;
#[doc = ""]
pub mod hcint4;
#[doc = "HCINTMSK4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk4`] module"]
pub type HCINTMSK4 = crate::Reg<hcintmsk4::HCINTMSK4_SPEC>;
#[doc = ""]
pub mod hcintmsk4;
#[doc = "HCTSIZ4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz4`] module"]
pub type HCTSIZ4 = crate::Reg<hctsiz4::HCTSIZ4_SPEC>;
#[doc = ""]
pub mod hctsiz4;
#[doc = "HCDMA4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma4`] module"]
pub type HCDMA4 = crate::Reg<hcdma4::HCDMA4_SPEC>;
#[doc = ""]
pub mod hcdma4;
#[doc = "HCDMAB4 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdmab4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdmab4`] module"]
pub type HCDMAB4 = crate::Reg<hcdmab4::HCDMAB4_SPEC>;
#[doc = ""]
pub mod hcdmab4;
#[doc = "HCCHAR5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar5`] module"]
pub type HCCHAR5 = crate::Reg<hcchar5::HCCHAR5_SPEC>;
#[doc = ""]
pub mod hcchar5;
#[doc = "HCINT5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint5`] module"]
pub type HCINT5 = crate::Reg<hcint5::HCINT5_SPEC>;
#[doc = ""]
pub mod hcint5;
#[doc = "HCINTMSK5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk5`] module"]
pub type HCINTMSK5 = crate::Reg<hcintmsk5::HCINTMSK5_SPEC>;
#[doc = ""]
pub mod hcintmsk5;
#[doc = "HCTSIZ5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz5`] module"]
pub type HCTSIZ5 = crate::Reg<hctsiz5::HCTSIZ5_SPEC>;
#[doc = ""]
pub mod hctsiz5;
#[doc = "HCDMA5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma5`] module"]
pub type HCDMA5 = crate::Reg<hcdma5::HCDMA5_SPEC>;
#[doc = ""]
pub mod hcdma5;
#[doc = "HCDMAB5 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdmab5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdmab5`] module"]
pub type HCDMAB5 = crate::Reg<hcdmab5::HCDMAB5_SPEC>;
#[doc = ""]
pub mod hcdmab5;
#[doc = "HCCHAR6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar6`] module"]
pub type HCCHAR6 = crate::Reg<hcchar6::HCCHAR6_SPEC>;
#[doc = ""]
pub mod hcchar6;
#[doc = "HCINT6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint6`] module"]
pub type HCINT6 = crate::Reg<hcint6::HCINT6_SPEC>;
#[doc = ""]
pub mod hcint6;
#[doc = "HCINTMSK6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk6`] module"]
pub type HCINTMSK6 = crate::Reg<hcintmsk6::HCINTMSK6_SPEC>;
#[doc = ""]
pub mod hcintmsk6;
#[doc = "HCTSIZ6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz6`] module"]
pub type HCTSIZ6 = crate::Reg<hctsiz6::HCTSIZ6_SPEC>;
#[doc = ""]
pub mod hctsiz6;
#[doc = "HCDMA6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma6`] module"]
pub type HCDMA6 = crate::Reg<hcdma6::HCDMA6_SPEC>;
#[doc = ""]
pub mod hcdma6;
#[doc = "HCDMAB6 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdmab6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdmab6`] module"]
pub type HCDMAB6 = crate::Reg<hcdmab6::HCDMAB6_SPEC>;
#[doc = ""]
pub mod hcdmab6;
#[doc = "HCCHAR7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar7`] module"]
pub type HCCHAR7 = crate::Reg<hcchar7::HCCHAR7_SPEC>;
#[doc = ""]
pub mod hcchar7;
#[doc = "HCINT7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint7`] module"]
pub type HCINT7 = crate::Reg<hcint7::HCINT7_SPEC>;
#[doc = ""]
pub mod hcint7;
#[doc = "HCINTMSK7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk7`] module"]
pub type HCINTMSK7 = crate::Reg<hcintmsk7::HCINTMSK7_SPEC>;
#[doc = ""]
pub mod hcintmsk7;
#[doc = "HCTSIZ7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz7`] module"]
pub type HCTSIZ7 = crate::Reg<hctsiz7::HCTSIZ7_SPEC>;
#[doc = ""]
pub mod hctsiz7;
#[doc = "HCDMA7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma7`] module"]
pub type HCDMA7 = crate::Reg<hcdma7::HCDMA7_SPEC>;
#[doc = ""]
pub mod hcdma7;
#[doc = "HCDMAB7 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdmab7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdmab7`] module"]
pub type HCDMAB7 = crate::Reg<hcdmab7::HCDMAB7_SPEC>;
#[doc = ""]
pub mod hcdmab7;
#[doc = "DCFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfg`] module"]
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
#[doc = ""]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctl`] module"]
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
#[doc = ""]
pub mod dctl;
#[doc = "DSTS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsts`] module"]
pub type DSTS = crate::Reg<dsts::DSTS_SPEC>;
#[doc = ""]
pub mod dsts;
#[doc = "DIEPMSK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepmsk`] module"]
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSK_SPEC>;
#[doc = ""]
pub mod diepmsk;
#[doc = "DOEPMSK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepmsk`] module"]
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSK_SPEC>;
#[doc = ""]
pub mod doepmsk;
#[doc = "DAINT (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daint`] module"]
pub type DAINT = crate::Reg<daint::DAINT_SPEC>;
#[doc = ""]
pub mod daint;
#[doc = "DAINTMSK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daintmsk`] module"]
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSK_SPEC>;
#[doc = ""]
pub mod daintmsk;
#[doc = "DVBUSDIS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbusdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbusdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbusdis`] module"]
pub type DVBUSDIS = crate::Reg<dvbusdis::DVBUSDIS_SPEC>;
#[doc = ""]
pub mod dvbusdis;
#[doc = "DVBUSPULSE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbuspulse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbuspulse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbuspulse`] module"]
pub type DVBUSPULSE = crate::Reg<dvbuspulse::DVBUSPULSE_SPEC>;
#[doc = ""]
pub mod dvbuspulse;
#[doc = "DTHRCTL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dthrctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dthrctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dthrctl`] module"]
pub type DTHRCTL = crate::Reg<dthrctl::DTHRCTL_SPEC>;
#[doc = ""]
pub mod dthrctl;
#[doc = "DIEPEMPMSK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepempmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepempmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepempmsk`] module"]
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>;
#[doc = ""]
pub mod diepempmsk;
#[doc = "DIEPCTL0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl0`] module"]
pub type DIEPCTL0 = crate::Reg<diepctl0::DIEPCTL0_SPEC>;
#[doc = ""]
pub mod diepctl0;
#[doc = "DIEPINT0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint0`] module"]
pub type DIEPINT0 = crate::Reg<diepint0::DIEPINT0_SPEC>;
#[doc = ""]
pub mod diepint0;
#[doc = "DIEPTSIZ0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz0`] module"]
pub type DIEPTSIZ0 = crate::Reg<dieptsiz0::DIEPTSIZ0_SPEC>;
#[doc = ""]
pub mod dieptsiz0;
#[doc = "DIEPDMA0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma0`] module"]
pub type DIEPDMA0 = crate::Reg<diepdma0::DIEPDMA0_SPEC>;
#[doc = ""]
pub mod diepdma0;
#[doc = "DTXFSTS0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts0`] module"]
pub type DTXFSTS0 = crate::Reg<dtxfsts0::DTXFSTS0_SPEC>;
#[doc = ""]
pub mod dtxfsts0;
#[doc = "DIEPDMAB0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdmab0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdmab0`] module"]
pub type DIEPDMAB0 = crate::Reg<diepdmab0::DIEPDMAB0_SPEC>;
#[doc = ""]
pub mod diepdmab0;
#[doc = "DIEPCTL1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl1`] module"]
pub type DIEPCTL1 = crate::Reg<diepctl1::DIEPCTL1_SPEC>;
#[doc = ""]
pub mod diepctl1;
#[doc = "DIEPINT1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint1`] module"]
pub type DIEPINT1 = crate::Reg<diepint1::DIEPINT1_SPEC>;
#[doc = ""]
pub mod diepint1;
#[doc = "DIEPTSIZ1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz1`] module"]
pub type DIEPTSIZ1 = crate::Reg<dieptsiz1::DIEPTSIZ1_SPEC>;
#[doc = ""]
pub mod dieptsiz1;
#[doc = "DIEPDMA1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma1`] module"]
pub type DIEPDMA1 = crate::Reg<diepdma1::DIEPDMA1_SPEC>;
#[doc = ""]
pub mod diepdma1;
#[doc = "DTXFSTS1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts1`] module"]
pub type DTXFSTS1 = crate::Reg<dtxfsts1::DTXFSTS1_SPEC>;
#[doc = ""]
pub mod dtxfsts1;
#[doc = "DIEPDMAB1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdmab1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdmab1`] module"]
pub type DIEPDMAB1 = crate::Reg<diepdmab1::DIEPDMAB1_SPEC>;
#[doc = ""]
pub mod diepdmab1;
#[doc = "DIEPCTL2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl2`] module"]
pub type DIEPCTL2 = crate::Reg<diepctl2::DIEPCTL2_SPEC>;
#[doc = ""]
pub mod diepctl2;
#[doc = "DIEPINT2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint2`] module"]
pub type DIEPINT2 = crate::Reg<diepint2::DIEPINT2_SPEC>;
#[doc = ""]
pub mod diepint2;
#[doc = "DIEPTSIZ2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz2`] module"]
pub type DIEPTSIZ2 = crate::Reg<dieptsiz2::DIEPTSIZ2_SPEC>;
#[doc = ""]
pub mod dieptsiz2;
#[doc = "DIEPDMA2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma2`] module"]
pub type DIEPDMA2 = crate::Reg<diepdma2::DIEPDMA2_SPEC>;
#[doc = ""]
pub mod diepdma2;
#[doc = "DTXFSTS2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts2`] module"]
pub type DTXFSTS2 = crate::Reg<dtxfsts2::DTXFSTS2_SPEC>;
#[doc = ""]
pub mod dtxfsts2;
#[doc = "DIEPDMAB2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdmab2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdmab2`] module"]
pub type DIEPDMAB2 = crate::Reg<diepdmab2::DIEPDMAB2_SPEC>;
#[doc = ""]
pub mod diepdmab2;
#[doc = "DIEPCTL3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl3`] module"]
pub type DIEPCTL3 = crate::Reg<diepctl3::DIEPCTL3_SPEC>;
#[doc = ""]
pub mod diepctl3;
#[doc = "DIEPINT3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint3`] module"]
pub type DIEPINT3 = crate::Reg<diepint3::DIEPINT3_SPEC>;
#[doc = ""]
pub mod diepint3;
#[doc = "DIEPTSIZ3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz3`] module"]
pub type DIEPTSIZ3 = crate::Reg<dieptsiz3::DIEPTSIZ3_SPEC>;
#[doc = ""]
pub mod dieptsiz3;
#[doc = "DIEPDMA3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma3`] module"]
pub type DIEPDMA3 = crate::Reg<diepdma3::DIEPDMA3_SPEC>;
#[doc = ""]
pub mod diepdma3;
#[doc = "DTXFSTS3 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts3`] module"]
pub type DTXFSTS3 = crate::Reg<dtxfsts3::DTXFSTS3_SPEC>;
#[doc = ""]
pub mod dtxfsts3;
#[doc = "DIEPDMAB3 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdmab3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdmab3`] module"]
pub type DIEPDMAB3 = crate::Reg<diepdmab3::DIEPDMAB3_SPEC>;
#[doc = ""]
pub mod diepdmab3;
#[doc = "DIEPCTL4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl4`] module"]
pub type DIEPCTL4 = crate::Reg<diepctl4::DIEPCTL4_SPEC>;
#[doc = ""]
pub mod diepctl4;
#[doc = "DIEPINT4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint4`] module"]
pub type DIEPINT4 = crate::Reg<diepint4::DIEPINT4_SPEC>;
#[doc = ""]
pub mod diepint4;
#[doc = "DIEPTSIZ4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz4`] module"]
pub type DIEPTSIZ4 = crate::Reg<dieptsiz4::DIEPTSIZ4_SPEC>;
#[doc = ""]
pub mod dieptsiz4;
#[doc = "DIEPDMA4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma4`] module"]
pub type DIEPDMA4 = crate::Reg<diepdma4::DIEPDMA4_SPEC>;
#[doc = ""]
pub mod diepdma4;
#[doc = "DTXFSTS4 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts4`] module"]
pub type DTXFSTS4 = crate::Reg<dtxfsts4::DTXFSTS4_SPEC>;
#[doc = ""]
pub mod dtxfsts4;
#[doc = "DIEPDMAB4 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdmab4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdmab4`] module"]
pub type DIEPDMAB4 = crate::Reg<diepdmab4::DIEPDMAB4_SPEC>;
#[doc = ""]
pub mod diepdmab4;
#[doc = "DIEPCTL5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl5`] module"]
pub type DIEPCTL5 = crate::Reg<diepctl5::DIEPCTL5_SPEC>;
#[doc = ""]
pub mod diepctl5;
#[doc = "DIEPINT5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint5`] module"]
pub type DIEPINT5 = crate::Reg<diepint5::DIEPINT5_SPEC>;
#[doc = ""]
pub mod diepint5;
#[doc = "DIEPTSIZ5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz5`] module"]
pub type DIEPTSIZ5 = crate::Reg<dieptsiz5::DIEPTSIZ5_SPEC>;
#[doc = ""]
pub mod dieptsiz5;
#[doc = "DIEPDMA5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma5`] module"]
pub type DIEPDMA5 = crate::Reg<diepdma5::DIEPDMA5_SPEC>;
#[doc = ""]
pub mod diepdma5;
#[doc = "DTXFSTS5 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts5`] module"]
pub type DTXFSTS5 = crate::Reg<dtxfsts5::DTXFSTS5_SPEC>;
#[doc = ""]
pub mod dtxfsts5;
#[doc = "DIEPDMAB5 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdmab5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdmab5`] module"]
pub type DIEPDMAB5 = crate::Reg<diepdmab5::DIEPDMAB5_SPEC>;
#[doc = ""]
pub mod diepdmab5;
#[doc = "DIEPCTL6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl6`] module"]
pub type DIEPCTL6 = crate::Reg<diepctl6::DIEPCTL6_SPEC>;
#[doc = ""]
pub mod diepctl6;
#[doc = "DIEPINT6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint6`] module"]
pub type DIEPINT6 = crate::Reg<diepint6::DIEPINT6_SPEC>;
#[doc = ""]
pub mod diepint6;
#[doc = "DIEPTSIZ6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz6`] module"]
pub type DIEPTSIZ6 = crate::Reg<dieptsiz6::DIEPTSIZ6_SPEC>;
#[doc = ""]
pub mod dieptsiz6;
#[doc = "DIEPDMA6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma6`] module"]
pub type DIEPDMA6 = crate::Reg<diepdma6::DIEPDMA6_SPEC>;
#[doc = ""]
pub mod diepdma6;
#[doc = "DTXFSTS6 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts6`] module"]
pub type DTXFSTS6 = crate::Reg<dtxfsts6::DTXFSTS6_SPEC>;
#[doc = ""]
pub mod dtxfsts6;
#[doc = "DIEPDMAB6 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdmab6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdmab6`] module"]
pub type DIEPDMAB6 = crate::Reg<diepdmab6::DIEPDMAB6_SPEC>;
#[doc = ""]
pub mod diepdmab6;
#[doc = "DOEPCTL0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl0`] module"]
pub type DOEPCTL0 = crate::Reg<doepctl0::DOEPCTL0_SPEC>;
#[doc = ""]
pub mod doepctl0;
#[doc = "DOEPINT0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint0`] module"]
pub type DOEPINT0 = crate::Reg<doepint0::DOEPINT0_SPEC>;
#[doc = ""]
pub mod doepint0;
#[doc = "DOEPTSIZ0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz0`] module"]
pub type DOEPTSIZ0 = crate::Reg<doeptsiz0::DOEPTSIZ0_SPEC>;
#[doc = ""]
pub mod doeptsiz0;
#[doc = "DOEPDMA0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma0`] module"]
pub type DOEPDMA0 = crate::Reg<doepdma0::DOEPDMA0_SPEC>;
#[doc = ""]
pub mod doepdma0;
#[doc = "DOEPDMAB0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdmab0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdmab0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdmab0`] module"]
pub type DOEPDMAB0 = crate::Reg<doepdmab0::DOEPDMAB0_SPEC>;
#[doc = ""]
pub mod doepdmab0;
#[doc = "DOEPCTL1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl1`] module"]
pub type DOEPCTL1 = crate::Reg<doepctl1::DOEPCTL1_SPEC>;
#[doc = ""]
pub mod doepctl1;
#[doc = "DOEPINT1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint1`] module"]
pub type DOEPINT1 = crate::Reg<doepint1::DOEPINT1_SPEC>;
#[doc = ""]
pub mod doepint1;
#[doc = "DOEPTSIZ1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz1`] module"]
pub type DOEPTSIZ1 = crate::Reg<doeptsiz1::DOEPTSIZ1_SPEC>;
#[doc = ""]
pub mod doeptsiz1;
#[doc = "DOEPDMA1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma1`] module"]
pub type DOEPDMA1 = crate::Reg<doepdma1::DOEPDMA1_SPEC>;
#[doc = ""]
pub mod doepdma1;
#[doc = "DOEPDMAB1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdmab1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdmab1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdmab1`] module"]
pub type DOEPDMAB1 = crate::Reg<doepdmab1::DOEPDMAB1_SPEC>;
#[doc = ""]
pub mod doepdmab1;
#[doc = "DOEPCTL2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl2`] module"]
pub type DOEPCTL2 = crate::Reg<doepctl2::DOEPCTL2_SPEC>;
#[doc = ""]
pub mod doepctl2;
#[doc = "DOEPINT2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint2`] module"]
pub type DOEPINT2 = crate::Reg<doepint2::DOEPINT2_SPEC>;
#[doc = ""]
pub mod doepint2;
#[doc = "DOEPTSIZ2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz2`] module"]
pub type DOEPTSIZ2 = crate::Reg<doeptsiz2::DOEPTSIZ2_SPEC>;
#[doc = ""]
pub mod doeptsiz2;
#[doc = "DOEPDMA2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma2`] module"]
pub type DOEPDMA2 = crate::Reg<doepdma2::DOEPDMA2_SPEC>;
#[doc = ""]
pub mod doepdma2;
#[doc = "DOEPDMAB2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdmab2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdmab2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdmab2`] module"]
pub type DOEPDMAB2 = crate::Reg<doepdmab2::DOEPDMAB2_SPEC>;
#[doc = ""]
pub mod doepdmab2;
#[doc = "DOEPCTL3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl3`] module"]
pub type DOEPCTL3 = crate::Reg<doepctl3::DOEPCTL3_SPEC>;
#[doc = ""]
pub mod doepctl3;
#[doc = "DOEPINT3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint3`] module"]
pub type DOEPINT3 = crate::Reg<doepint3::DOEPINT3_SPEC>;
#[doc = ""]
pub mod doepint3;
#[doc = "DOEPTSIZ3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz3`] module"]
pub type DOEPTSIZ3 = crate::Reg<doeptsiz3::DOEPTSIZ3_SPEC>;
#[doc = ""]
pub mod doeptsiz3;
#[doc = "DOEPDMA3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma3`] module"]
pub type DOEPDMA3 = crate::Reg<doepdma3::DOEPDMA3_SPEC>;
#[doc = ""]
pub mod doepdma3;
#[doc = "DOEPDMAB3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdmab3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdmab3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdmab3`] module"]
pub type DOEPDMAB3 = crate::Reg<doepdmab3::DOEPDMAB3_SPEC>;
#[doc = ""]
pub mod doepdmab3;
#[doc = "DOEPCTL4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl4`] module"]
pub type DOEPCTL4 = crate::Reg<doepctl4::DOEPCTL4_SPEC>;
#[doc = ""]
pub mod doepctl4;
#[doc = "DOEPINT4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint4`] module"]
pub type DOEPINT4 = crate::Reg<doepint4::DOEPINT4_SPEC>;
#[doc = ""]
pub mod doepint4;
#[doc = "DOEPTSIZ4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz4`] module"]
pub type DOEPTSIZ4 = crate::Reg<doeptsiz4::DOEPTSIZ4_SPEC>;
#[doc = ""]
pub mod doeptsiz4;
#[doc = "DOEPDMA4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma4`] module"]
pub type DOEPDMA4 = crate::Reg<doepdma4::DOEPDMA4_SPEC>;
#[doc = ""]
pub mod doepdma4;
#[doc = "DOEPDMAB4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdmab4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdmab4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdmab4`] module"]
pub type DOEPDMAB4 = crate::Reg<doepdmab4::DOEPDMAB4_SPEC>;
#[doc = ""]
pub mod doepdmab4;
#[doc = "DOEPCTL5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl5`] module"]
pub type DOEPCTL5 = crate::Reg<doepctl5::DOEPCTL5_SPEC>;
#[doc = ""]
pub mod doepctl5;
#[doc = "DOEPINT5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint5`] module"]
pub type DOEPINT5 = crate::Reg<doepint5::DOEPINT5_SPEC>;
#[doc = ""]
pub mod doepint5;
#[doc = "DOEPTSIZ5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz5`] module"]
pub type DOEPTSIZ5 = crate::Reg<doeptsiz5::DOEPTSIZ5_SPEC>;
#[doc = ""]
pub mod doeptsiz5;
#[doc = "DOEPDMA5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma5`] module"]
pub type DOEPDMA5 = crate::Reg<doepdma5::DOEPDMA5_SPEC>;
#[doc = ""]
pub mod doepdma5;
#[doc = "DOEPDMAB5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdmab5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdmab5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdmab5`] module"]
pub type DOEPDMAB5 = crate::Reg<doepdmab5::DOEPDMAB5_SPEC>;
#[doc = ""]
pub mod doepdmab5;
#[doc = "DOEPCTL6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl6`] module"]
pub type DOEPCTL6 = crate::Reg<doepctl6::DOEPCTL6_SPEC>;
#[doc = ""]
pub mod doepctl6;
#[doc = "DOEPINT6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint6`] module"]
pub type DOEPINT6 = crate::Reg<doepint6::DOEPINT6_SPEC>;
#[doc = ""]
pub mod doepint6;
#[doc = "DOEPTSIZ6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz6`] module"]
pub type DOEPTSIZ6 = crate::Reg<doeptsiz6::DOEPTSIZ6_SPEC>;
#[doc = ""]
pub mod doeptsiz6;
#[doc = "DOEPDMA6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma6`] module"]
pub type DOEPDMA6 = crate::Reg<doepdma6::DOEPDMA6_SPEC>;
#[doc = ""]
pub mod doepdma6;
#[doc = "DOEPDMAB6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdmab6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdmab6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdmab6`] module"]
pub type DOEPDMAB6 = crate::Reg<doepdmab6::DOEPDMAB6_SPEC>;
#[doc = ""]
pub mod doepdmab6;
#[doc = "PCGCCTL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcgcctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcgcctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcgcctl`] module"]
pub type PCGCCTL = crate::Reg<pcgcctl::PCGCCTL_SPEC>;
#[doc = ""]
pub mod pcgcctl;
