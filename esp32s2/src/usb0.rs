#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
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
    dieptxf: [DIEPTXF; 4],
    _reserved20: [u8; 0x02ec],
    hcfg: HCFG,
    hfir: HFIR,
    hfnum: HFNUM,
    _reserved23: [u8; 0x04],
    hptxsts: HPTXSTS,
    haint: HAINT,
    haintmsk: HAINTMSK,
    hflbaddr: HFLBADDR,
    _reserved27: [u8; 0x20],
    hprt: HPRT,
    _reserved28: [u8; 0xbc],
    hcchar0: HCCHAR0,
    _reserved29: [u8; 0x04],
    hcint0: HCINT0,
    hcintmsk0: HCINTMSK0,
    hctsiz0: HCTSIZ0,
    hcdma0: HCDMA0,
    _reserved33: [u8; 0x04],
    hcdmab0: HCDMAB0,
    hcchar1: HCCHAR1,
    _reserved35: [u8; 0x04],
    hcint1: HCINT1,
    hcintmsk1: HCINTMSK1,
    hctsiz1: HCTSIZ1,
    hcdma1: HCDMA1,
    _reserved39: [u8; 0x04],
    hcdmab1: HCDMAB1,
    hcchar2: HCCHAR2,
    _reserved41: [u8; 0x04],
    hcint2: HCINT2,
    hcintmsk2: HCINTMSK2,
    hctsiz2: HCTSIZ2,
    hcdma2: HCDMA2,
    _reserved45: [u8; 0x04],
    hcdmab2: HCDMAB2,
    hcchar3: HCCHAR3,
    _reserved47: [u8; 0x04],
    hcint3: HCINT3,
    hcintmsk3: HCINTMSK3,
    hctsiz3: HCTSIZ3,
    hcdma3: HCDMA3,
    _reserved51: [u8; 0x04],
    hcdmab3: HCDMAB3,
    hcchar4: HCCHAR4,
    _reserved53: [u8; 0x04],
    hcint4: HCINT4,
    hcintmsk4: HCINTMSK4,
    hctsiz4: HCTSIZ4,
    hcdma4: HCDMA4,
    _reserved57: [u8; 0x04],
    hcdmab4: HCDMAB4,
    hcchar5: HCCHAR5,
    _reserved59: [u8; 0x04],
    hcint5: HCINT5,
    hcintmsk5: HCINTMSK5,
    hctsiz5: HCTSIZ5,
    hcdma5: HCDMA5,
    _reserved63: [u8; 0x04],
    hcdmab5: HCDMAB5,
    hcchar6: HCCHAR6,
    _reserved65: [u8; 0x04],
    hcint6: HCINT6,
    hcintmsk6: HCINTMSK6,
    hctsiz6: HCTSIZ6,
    hcdma6: HCDMA6,
    _reserved69: [u8; 0x04],
    hcdmab6: HCDMAB6,
    hcchar7: HCCHAR7,
    _reserved71: [u8; 0x04],
    hcint7: HCINT7,
    hcintmsk7: HCINTMSK7,
    hctsiz7: HCTSIZ7,
    hcdma7: HCDMA7,
    _reserved75: [u8; 0x04],
    hcdmab7: HCDMAB7,
    _reserved76: [u8; 0x0200],
    dcfg: DCFG,
    dctl: DCTL,
    dsts: DSTS,
    _reserved79: [u8; 0x04],
    diepmsk: DIEPMSK,
    doepmsk: DOEPMSK,
    daint: DAINT,
    daintmsk: DAINTMSK,
    _reserved83: [u8; 0x08],
    dvbusdis: DVBUSDIS,
    dvbuspulse: DVBUSPULSE,
    dthrctl: DTHRCTL,
    diepempmsk: DIEPEMPMSK,
    _reserved87: [u8; 0xc8],
    in_ep0: IN_EP0,
    in_ep: [IN_EP; 6],
    _reserved89: [u8; 0x0120],
    out_ep0: OUT_EP0,
    out_ep: [OUT_EP; 6],
    _reserved91: [u8; 0x0220],
    pcgcctl: PCGCCTL,
    _reserved92: [u8; 0x01fc],
    fifo: (),
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
    #[doc = "0x104..0x114 - "]
    #[inline(always)]
    pub const fn dieptxf(&self, n: usize) -> &DIEPTXF {
        &self.dieptxf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x104..0x114 - "]
    #[inline(always)]
    pub fn dieptxf_iter(&self) -> impl Iterator<Item = &DIEPTXF> {
        self.dieptxf.iter()
    }
    #[doc = "0x104 - DIEPTXF1"]
    #[inline(always)]
    pub const fn dieptxf1(&self) -> &DIEPTXF {
        self.dieptxf(0)
    }
    #[doc = "0x108 - DIEPTXF2"]
    #[inline(always)]
    pub const fn dieptxf2(&self) -> &DIEPTXF {
        self.dieptxf(1)
    }
    #[doc = "0x10c - DIEPTXF3"]
    #[inline(always)]
    pub const fn dieptxf3(&self) -> &DIEPTXF {
        self.dieptxf(2)
    }
    #[doc = "0x110 - DIEPTXF4"]
    #[inline(always)]
    pub const fn dieptxf4(&self) -> &DIEPTXF {
        self.dieptxf(3)
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
    #[doc = "0x900..0x920 - Device IN endpoint 0"]
    #[inline(always)]
    pub const fn in_ep0(&self) -> &IN_EP0 {
        &self.in_ep0
    }
    #[doc = "0x920..0x9e0 - Device IN endpoints 1-6"]
    #[inline(always)]
    pub const fn in_ep(&self, n: usize) -> &IN_EP {
        &self.in_ep[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x920..0x9e0 - Device IN endpoints 1-6"]
    #[inline(always)]
    pub fn in_ep_iter(&self) -> impl Iterator<Item = &IN_EP> {
        self.in_ep.iter()
    }
    #[doc = "0x920..0x940 - Device IN endpoints 1-6"]
    #[inline(always)]
    pub const fn in_ep1(&self) -> &IN_EP {
        self.in_ep(0)
    }
    #[doc = "0x940..0x960 - Device IN endpoints 1-6"]
    #[inline(always)]
    pub const fn in_ep2(&self) -> &IN_EP {
        self.in_ep(1)
    }
    #[doc = "0x960..0x980 - Device IN endpoints 1-6"]
    #[inline(always)]
    pub const fn in_ep3(&self) -> &IN_EP {
        self.in_ep(2)
    }
    #[doc = "0x980..0x9a0 - Device IN endpoints 1-6"]
    #[inline(always)]
    pub const fn in_ep4(&self) -> &IN_EP {
        self.in_ep(3)
    }
    #[doc = "0x9a0..0x9c0 - Device IN endpoints 1-6"]
    #[inline(always)]
    pub const fn in_ep5(&self) -> &IN_EP {
        self.in_ep(4)
    }
    #[doc = "0x9c0..0x9e0 - Device IN endpoints 1-6"]
    #[inline(always)]
    pub const fn in_ep6(&self) -> &IN_EP {
        self.in_ep(5)
    }
    #[doc = "0xb00..0xb20 - Device OUT endpoint 0"]
    #[inline(always)]
    pub const fn out_ep0(&self) -> &OUT_EP0 {
        &self.out_ep0
    }
    #[doc = "0xb20..0xbe0 - Device OUT endpoints 1-6"]
    #[inline(always)]
    pub const fn out_ep(&self, n: usize) -> &OUT_EP {
        &self.out_ep[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xb20..0xbe0 - Device OUT endpoints 1-6"]
    #[inline(always)]
    pub fn out_ep_iter(&self) -> impl Iterator<Item = &OUT_EP> {
        self.out_ep.iter()
    }
    #[doc = "0xb20..0xb40 - Device OUT endpoints 1-6"]
    #[inline(always)]
    pub const fn out_ep1(&self) -> &OUT_EP {
        self.out_ep(0)
    }
    #[doc = "0xb40..0xb60 - Device OUT endpoints 1-6"]
    #[inline(always)]
    pub const fn out_ep2(&self) -> &OUT_EP {
        self.out_ep(1)
    }
    #[doc = "0xb60..0xb80 - Device OUT endpoints 1-6"]
    #[inline(always)]
    pub const fn out_ep3(&self) -> &OUT_EP {
        self.out_ep(2)
    }
    #[doc = "0xb80..0xba0 - Device OUT endpoints 1-6"]
    #[inline(always)]
    pub const fn out_ep4(&self) -> &OUT_EP {
        self.out_ep(3)
    }
    #[doc = "0xba0..0xbc0 - Device OUT endpoints 1-6"]
    #[inline(always)]
    pub const fn out_ep5(&self) -> &OUT_EP {
        self.out_ep(4)
    }
    #[doc = "0xbc0..0xbe0 - Device OUT endpoints 1-6"]
    #[inline(always)]
    pub const fn out_ep6(&self) -> &OUT_EP {
        self.out_ep(5)
    }
    #[doc = "0xe00 - "]
    #[inline(always)]
    pub const fn pcgcctl(&self) -> &PCGCCTL {
        &self.pcgcctl
    }
    #[doc = "0x1000..0x1040 - Read and write data to the USB FIFOs through this register."]
    #[inline(always)]
    pub const fn fifo(&self, n: usize) -> &FIFO {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4096)
                .add(4096 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1000..0x1040 - Read and write data to the USB FIFOs through this register."]
    #[inline(always)]
    pub fn fifo_iter(&self) -> impl Iterator<Item = &FIFO> {
        (0..16).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4096)
                .add(4096 * n)
                .cast()
        })
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
#[doc = "DIEPTXF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf`] module"]
pub type DIEPTXF = crate::Reg<dieptxf::DIEPTXF_SPEC>;
#[doc = ""]
pub mod dieptxf;
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
#[doc = "Device IN endpoint 0"]
pub use self::in_ep0::IN_EP0;
#[doc = r"Cluster"]
#[doc = "Device IN endpoint 0"]
pub mod in_ep0;
#[doc = "Device IN endpoints 1-6"]
pub use self::in_ep::IN_EP;
#[doc = r"Cluster"]
#[doc = "Device IN endpoints 1-6"]
pub mod in_ep;
#[doc = "Device OUT endpoint 0"]
pub use self::out_ep0::OUT_EP0;
#[doc = r"Cluster"]
#[doc = "Device OUT endpoint 0"]
pub mod out_ep0;
#[doc = "Device OUT endpoints 1-6"]
pub use self::out_ep::OUT_EP;
#[doc = r"Cluster"]
#[doc = "Device OUT endpoints 1-6"]
pub mod out_ep;
#[doc = "PCGCCTL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcgcctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcgcctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcgcctl`] module"]
pub type PCGCCTL = crate::Reg<pcgcctl::PCGCCTL_SPEC>;
#[doc = ""]
pub mod pcgcctl;
#[doc = "FIFO (rw) register accessor: Read and write data to the USB FIFOs through this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo`] module"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "Read and write data to the USB FIFOs through this register."]
pub mod fifo;
