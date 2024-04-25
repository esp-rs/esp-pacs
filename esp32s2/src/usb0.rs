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
    hc: [HC; 8],
    _reserved29: [u8; 0x0200],
    dcfg: DCFG,
    dctl: DCTL,
    dsts: DSTS,
    _reserved32: [u8; 0x04],
    diepmsk: DIEPMSK,
    doepmsk: DOEPMSK,
    daint: DAINT,
    daintmsk: DAINTMSK,
    _reserved36: [u8; 0x08],
    dvbusdis: DVBUSDIS,
    dvbuspulse: DVBUSPULSE,
    dthrctl: DTHRCTL,
    diepempmsk: DIEPEMPMSK,
    _reserved40: [u8; 0xc8],
    in_ep0: IN_EP0,
    in_ep: [IN_EP; 6],
    _reserved42: [u8; 0x0120],
    out_ep0: OUT_EP0,
    out_ep: [OUT_EP; 6],
    _reserved44: [u8; 0x0220],
    pcgcctl: PCGCCTL,
    _reserved45: [u8; 0x01fc],
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
    #[doc = "0x500..0x600 - Cluster HC%s, containing HCCHAR?, HCINT?, HCINTMSK?, HCTSIZ?, HCDMA?, HCDMAB?"]
    #[inline(always)]
    pub const fn hc(&self, n: usize) -> &HC {
        &self.hc[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x500..0x600 - Cluster HC%s, containing HCCHAR?, HCINT?, HCINTMSK?, HCTSIZ?, HCDMA?, HCDMAB?"]
    #[inline(always)]
    pub fn hc_iter(&self) -> impl Iterator<Item = &HC> {
        self.hc.iter()
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
#[doc = "Cluster HC%s, containing HCCHAR?, HCINT?, HCINTMSK?, HCTSIZ?, HCDMA?, HCDMAB?"]
pub use self::hc::HC;
#[doc = r"Cluster"]
#[doc = "Cluster HC%s, containing HCCHAR?, HCINT?, HCINTMSK?, HCTSIZ?, HCDMA?, HCDMAB?"]
pub mod hc;
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
