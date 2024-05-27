#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
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
    ///0x00 -
    #[inline(always)]
    pub const fn gotgctl(&self) -> &GOTGCTL {
        &self.gotgctl
    }
    ///0x04 -
    #[inline(always)]
    pub const fn gotgint(&self) -> &GOTGINT {
        &self.gotgint
    }
    ///0x08 -
    #[inline(always)]
    pub const fn gahbcfg(&self) -> &GAHBCFG {
        &self.gahbcfg
    }
    ///0x0c -
    #[inline(always)]
    pub const fn gusbcfg(&self) -> &GUSBCFG {
        &self.gusbcfg
    }
    ///0x10 -
    #[inline(always)]
    pub const fn grstctl(&self) -> &GRSTCTL {
        &self.grstctl
    }
    ///0x14 -
    #[inline(always)]
    pub const fn gintsts(&self) -> &GINTSTS {
        &self.gintsts
    }
    ///0x18 -
    #[inline(always)]
    pub const fn gintmsk(&self) -> &GINTMSK {
        &self.gintmsk
    }
    ///0x1c -
    #[inline(always)]
    pub const fn grxstsr(&self) -> &GRXSTSR {
        &self.grxstsr
    }
    ///0x20 -
    #[inline(always)]
    pub const fn grxstsp(&self) -> &GRXSTSP {
        &self.grxstsp
    }
    ///0x24 -
    #[inline(always)]
    pub const fn grxfsiz(&self) -> &GRXFSIZ {
        &self.grxfsiz
    }
    ///0x28 -
    #[inline(always)]
    pub const fn gnptxfsiz(&self) -> &GNPTXFSIZ {
        &self.gnptxfsiz
    }
    ///0x2c -
    #[inline(always)]
    pub const fn gnptxsts(&self) -> &GNPTXSTS {
        &self.gnptxsts
    }
    ///0x40 -
    #[inline(always)]
    pub const fn gsnpsid(&self) -> &GSNPSID {
        &self.gsnpsid
    }
    ///0x44 -
    #[inline(always)]
    pub const fn ghwcfg1(&self) -> &GHWCFG1 {
        &self.ghwcfg1
    }
    ///0x48 -
    #[inline(always)]
    pub const fn ghwcfg2(&self) -> &GHWCFG2 {
        &self.ghwcfg2
    }
    ///0x4c -
    #[inline(always)]
    pub const fn ghwcfg3(&self) -> &GHWCFG3 {
        &self.ghwcfg3
    }
    ///0x50 -
    #[inline(always)]
    pub const fn ghwcfg4(&self) -> &GHWCFG4 {
        &self.ghwcfg4
    }
    ///0x5c -
    #[inline(always)]
    pub const fn gdfifocfg(&self) -> &GDFIFOCFG {
        &self.gdfifocfg
    }
    ///0x100 -
    #[inline(always)]
    pub const fn hptxfsiz(&self) -> &HPTXFSIZ {
        &self.hptxfsiz
    }
    ///0x104..0x114 -
    #[inline(always)]
    pub const fn dieptxf(&self, n: usize) -> &DIEPTXF {
        &self.dieptxf[n]
    }
    ///Iterator for array of:
    ///0x104..0x114 -
    #[inline(always)]
    pub fn dieptxf_iter(&self) -> impl Iterator<Item = &DIEPTXF> {
        self.dieptxf.iter()
    }
    ///0x104 - DIEPTXF1
    #[inline(always)]
    pub const fn dieptxf1(&self) -> &DIEPTXF {
        self.dieptxf(0)
    }
    ///0x108 - DIEPTXF2
    #[inline(always)]
    pub const fn dieptxf2(&self) -> &DIEPTXF {
        self.dieptxf(1)
    }
    ///0x10c - DIEPTXF3
    #[inline(always)]
    pub const fn dieptxf3(&self) -> &DIEPTXF {
        self.dieptxf(2)
    }
    ///0x110 - DIEPTXF4
    #[inline(always)]
    pub const fn dieptxf4(&self) -> &DIEPTXF {
        self.dieptxf(3)
    }
    ///0x400 -
    #[inline(always)]
    pub const fn hcfg(&self) -> &HCFG {
        &self.hcfg
    }
    ///0x404 -
    #[inline(always)]
    pub const fn hfir(&self) -> &HFIR {
        &self.hfir
    }
    ///0x408 -
    #[inline(always)]
    pub const fn hfnum(&self) -> &HFNUM {
        &self.hfnum
    }
    ///0x410 -
    #[inline(always)]
    pub const fn hptxsts(&self) -> &HPTXSTS {
        &self.hptxsts
    }
    ///0x414 -
    #[inline(always)]
    pub const fn haint(&self) -> &HAINT {
        &self.haint
    }
    ///0x418 -
    #[inline(always)]
    pub const fn haintmsk(&self) -> &HAINTMSK {
        &self.haintmsk
    }
    ///0x41c -
    #[inline(always)]
    pub const fn hflbaddr(&self) -> &HFLBADDR {
        &self.hflbaddr
    }
    ///0x440 -
    #[inline(always)]
    pub const fn hprt(&self) -> &HPRT {
        &self.hprt
    }
    ///0x500..0x600 - Cluster HC%s, containing HCCHAR?, HCINT?, HCINTMSK?, HCTSIZ?, HCDMA?, HCDMAB?
    #[inline(always)]
    pub const fn hc(&self, n: usize) -> &HC {
        &self.hc[n]
    }
    ///Iterator for array of:
    ///0x500..0x600 - Cluster HC%s, containing HCCHAR?, HCINT?, HCINTMSK?, HCTSIZ?, HCDMA?, HCDMAB?
    #[inline(always)]
    pub fn hc_iter(&self) -> impl Iterator<Item = &HC> {
        self.hc.iter()
    }
    ///0x800 -
    #[inline(always)]
    pub const fn dcfg(&self) -> &DCFG {
        &self.dcfg
    }
    ///0x804 -
    #[inline(always)]
    pub const fn dctl(&self) -> &DCTL {
        &self.dctl
    }
    ///0x808 -
    #[inline(always)]
    pub const fn dsts(&self) -> &DSTS {
        &self.dsts
    }
    ///0x810 -
    #[inline(always)]
    pub const fn diepmsk(&self) -> &DIEPMSK {
        &self.diepmsk
    }
    ///0x814 -
    #[inline(always)]
    pub const fn doepmsk(&self) -> &DOEPMSK {
        &self.doepmsk
    }
    ///0x818 -
    #[inline(always)]
    pub const fn daint(&self) -> &DAINT {
        &self.daint
    }
    ///0x81c -
    #[inline(always)]
    pub const fn daintmsk(&self) -> &DAINTMSK {
        &self.daintmsk
    }
    ///0x828 -
    #[inline(always)]
    pub const fn dvbusdis(&self) -> &DVBUSDIS {
        &self.dvbusdis
    }
    ///0x82c -
    #[inline(always)]
    pub const fn dvbuspulse(&self) -> &DVBUSPULSE {
        &self.dvbuspulse
    }
    ///0x830 -
    #[inline(always)]
    pub const fn dthrctl(&self) -> &DTHRCTL {
        &self.dthrctl
    }
    ///0x834 -
    #[inline(always)]
    pub const fn diepempmsk(&self) -> &DIEPEMPMSK {
        &self.diepempmsk
    }
    ///0x900..0x920 - Device IN endpoint 0
    #[inline(always)]
    pub const fn in_ep0(&self) -> &IN_EP0 {
        &self.in_ep0
    }
    ///0x920..0x9e0 - Device IN endpoints 1-6
    #[inline(always)]
    pub const fn in_ep(&self, n: usize) -> &IN_EP {
        &self.in_ep[n]
    }
    ///Iterator for array of:
    ///0x920..0x9e0 - Device IN endpoints 1-6
    #[inline(always)]
    pub fn in_ep_iter(&self) -> impl Iterator<Item = &IN_EP> {
        self.in_ep.iter()
    }
    ///0x920..0x940 - Device IN endpoints 1-6
    #[inline(always)]
    pub const fn in_ep1(&self) -> &IN_EP {
        self.in_ep(0)
    }
    ///0x940..0x960 - Device IN endpoints 1-6
    #[inline(always)]
    pub const fn in_ep2(&self) -> &IN_EP {
        self.in_ep(1)
    }
    ///0x960..0x980 - Device IN endpoints 1-6
    #[inline(always)]
    pub const fn in_ep3(&self) -> &IN_EP {
        self.in_ep(2)
    }
    ///0x980..0x9a0 - Device IN endpoints 1-6
    #[inline(always)]
    pub const fn in_ep4(&self) -> &IN_EP {
        self.in_ep(3)
    }
    ///0x9a0..0x9c0 - Device IN endpoints 1-6
    #[inline(always)]
    pub const fn in_ep5(&self) -> &IN_EP {
        self.in_ep(4)
    }
    ///0x9c0..0x9e0 - Device IN endpoints 1-6
    #[inline(always)]
    pub const fn in_ep6(&self) -> &IN_EP {
        self.in_ep(5)
    }
    ///0xb00..0xb20 - Device OUT endpoint 0
    #[inline(always)]
    pub const fn out_ep0(&self) -> &OUT_EP0 {
        &self.out_ep0
    }
    ///0xb20..0xbe0 - Device OUT endpoints 1-6
    #[inline(always)]
    pub const fn out_ep(&self, n: usize) -> &OUT_EP {
        &self.out_ep[n]
    }
    ///Iterator for array of:
    ///0xb20..0xbe0 - Device OUT endpoints 1-6
    #[inline(always)]
    pub fn out_ep_iter(&self) -> impl Iterator<Item = &OUT_EP> {
        self.out_ep.iter()
    }
    ///0xb20..0xb40 - Device OUT endpoints 1-6
    #[inline(always)]
    pub const fn out_ep1(&self) -> &OUT_EP {
        self.out_ep(0)
    }
    ///0xb40..0xb60 - Device OUT endpoints 1-6
    #[inline(always)]
    pub const fn out_ep2(&self) -> &OUT_EP {
        self.out_ep(1)
    }
    ///0xb60..0xb80 - Device OUT endpoints 1-6
    #[inline(always)]
    pub const fn out_ep3(&self) -> &OUT_EP {
        self.out_ep(2)
    }
    ///0xb80..0xba0 - Device OUT endpoints 1-6
    #[inline(always)]
    pub const fn out_ep4(&self) -> &OUT_EP {
        self.out_ep(3)
    }
    ///0xba0..0xbc0 - Device OUT endpoints 1-6
    #[inline(always)]
    pub const fn out_ep5(&self) -> &OUT_EP {
        self.out_ep(4)
    }
    ///0xbc0..0xbe0 - Device OUT endpoints 1-6
    #[inline(always)]
    pub const fn out_ep6(&self) -> &OUT_EP {
        self.out_ep(5)
    }
    ///0xe00 -
    #[inline(always)]
    pub const fn pcgcctl(&self) -> &PCGCCTL {
        &self.pcgcctl
    }
    ///0x1000..0x1040 - Read and write data to the USB FIFOs through this register.
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
    ///Iterator for array of:
    ///0x1000..0x1040 - Read and write data to the USB FIFOs through this register.
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
/**GOTGCTL (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`gotgctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gotgctl`] module*/
pub type GOTGCTL = crate::Reg<gotgctl::GOTGCTL_SPEC>;
///
pub mod gotgctl;
/**GOTGINT (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`gotgint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gotgint`] module*/
pub type GOTGINT = crate::Reg<gotgint::GOTGINT_SPEC>;
///
pub mod gotgint;
/**GAHBCFG (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`gahbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gahbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gahbcfg`] module*/
pub type GAHBCFG = crate::Reg<gahbcfg::GAHBCFG_SPEC>;
///
pub mod gahbcfg;
/**GUSBCFG (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`gusbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gusbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gusbcfg`] module*/
pub type GUSBCFG = crate::Reg<gusbcfg::GUSBCFG_SPEC>;
///
pub mod gusbcfg;
/**GRSTCTL (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`grstctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grstctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@grstctl`] module*/
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTL_SPEC>;
///
pub mod grstctl;
/**GINTSTS (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`gintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gintsts`] module*/
pub type GINTSTS = crate::Reg<gintsts::GINTSTS_SPEC>;
///
pub mod gintsts;
/**GINTMSK (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`gintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gintmsk`] module*/
pub type GINTMSK = crate::Reg<gintmsk::GINTMSK_SPEC>;
///
pub mod gintmsk;
/**GRXSTSR (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`grxstsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@grxstsr`] module*/
pub type GRXSTSR = crate::Reg<grxstsr::GRXSTSR_SPEC>;
///
pub mod grxstsr;
/**GRXSTSP (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`grxstsp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@grxstsp`] module*/
pub type GRXSTSP = crate::Reg<grxstsp::GRXSTSP_SPEC>;
///
pub mod grxstsp;
/**GRXFSIZ (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`grxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@grxfsiz`] module*/
pub type GRXFSIZ = crate::Reg<grxfsiz::GRXFSIZ_SPEC>;
///
pub mod grxfsiz;
/**GNPTXFSIZ (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`gnptxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gnptxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gnptxfsiz`] module*/
pub type GNPTXFSIZ = crate::Reg<gnptxfsiz::GNPTXFSIZ_SPEC>;
///
pub mod gnptxfsiz;
/**GNPTXSTS (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`gnptxsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gnptxsts`] module*/
pub type GNPTXSTS = crate::Reg<gnptxsts::GNPTXSTS_SPEC>;
///
pub mod gnptxsts;
/**GSNPSID (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`gsnpsid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gsnpsid`] module*/
pub type GSNPSID = crate::Reg<gsnpsid::GSNPSID_SPEC>;
///
pub mod gsnpsid;
/**GHWCFG1 (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`ghwcfg1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ghwcfg1`] module*/
pub type GHWCFG1 = crate::Reg<ghwcfg1::GHWCFG1_SPEC>;
///
pub mod ghwcfg1;
/**GHWCFG2 (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`ghwcfg2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ghwcfg2`] module*/
pub type GHWCFG2 = crate::Reg<ghwcfg2::GHWCFG2_SPEC>;
///
pub mod ghwcfg2;
/**GHWCFG3 (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`ghwcfg3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ghwcfg3`] module*/
pub type GHWCFG3 = crate::Reg<ghwcfg3::GHWCFG3_SPEC>;
///
pub mod ghwcfg3;
/**GHWCFG4 (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`ghwcfg4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ghwcfg4`] module*/
pub type GHWCFG4 = crate::Reg<ghwcfg4::GHWCFG4_SPEC>;
///
pub mod ghwcfg4;
/**GDFIFOCFG (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`gdfifocfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdfifocfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gdfifocfg`] module*/
pub type GDFIFOCFG = crate::Reg<gdfifocfg::GDFIFOCFG_SPEC>;
///
pub mod gdfifocfg;
/**HPTXFSIZ (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`hptxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hptxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hptxfsiz`] module*/
pub type HPTXFSIZ = crate::Reg<hptxfsiz::HPTXFSIZ_SPEC>;
///
pub mod hptxfsiz;
/**DIEPTXF (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`dieptxf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dieptxf`] module*/
pub type DIEPTXF = crate::Reg<dieptxf::DIEPTXF_SPEC>;
///
pub mod dieptxf;
/**HCFG (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`hcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hcfg`] module*/
pub type HCFG = crate::Reg<hcfg::HCFG_SPEC>;
///
pub mod hcfg;
/**HFIR (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`hfir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hfir`] module*/
pub type HFIR = crate::Reg<hfir::HFIR_SPEC>;
///
pub mod hfir;
/**HFNUM (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`hfnum::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hfnum`] module*/
pub type HFNUM = crate::Reg<hfnum::HFNUM_SPEC>;
///
pub mod hfnum;
/**HPTXSTS (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`hptxsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hptxsts`] module*/
pub type HPTXSTS = crate::Reg<hptxsts::HPTXSTS_SPEC>;
///
pub mod hptxsts;
/**HAINT (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`haint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@haint`] module*/
pub type HAINT = crate::Reg<haint::HAINT_SPEC>;
///
pub mod haint;
/**HAINTMSK (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`haintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`haintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@haintmsk`] module*/
pub type HAINTMSK = crate::Reg<haintmsk::HAINTMSK_SPEC>;
///
pub mod haintmsk;
/**HFLBADDR (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`hflbaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hflbaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hflbaddr`] module*/
pub type HFLBADDR = crate::Reg<hflbaddr::HFLBADDR_SPEC>;
///
pub mod hflbaddr;
/**HPRT (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`hprt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hprt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hprt`] module*/
pub type HPRT = crate::Reg<hprt::HPRT_SPEC>;
///
pub mod hprt;
///Cluster HC%s, containing HCCHAR?, HCINT?, HCINTMSK?, HCTSIZ?, HCDMA?, HCDMAB?
pub use self::hc::HC;
///Cluster
///Cluster HC%s, containing HCCHAR?, HCINT?, HCINTMSK?, HCTSIZ?, HCDMA?, HCDMAB?
pub mod hc;
/**DCFG (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`dcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dcfg`] module*/
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
///
pub mod dcfg;
/**DCTL (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dctl`] module*/
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
///
pub mod dctl;
/**DSTS (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`dsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dsts`] module*/
pub type DSTS = crate::Reg<dsts::DSTS_SPEC>;
///
pub mod dsts;
/**DIEPMSK (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`diepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@diepmsk`] module*/
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSK_SPEC>;
///
pub mod diepmsk;
/**DOEPMSK (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`doepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@doepmsk`] module*/
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSK_SPEC>;
///
pub mod doepmsk;
/**DAINT (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`daint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@daint`] module*/
pub type DAINT = crate::Reg<daint::DAINT_SPEC>;
///
pub mod daint;
/**DAINTMSK (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`daintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@daintmsk`] module*/
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSK_SPEC>;
///
pub mod daintmsk;
/**DVBUSDIS (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`dvbusdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbusdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dvbusdis`] module*/
pub type DVBUSDIS = crate::Reg<dvbusdis::DVBUSDIS_SPEC>;
///
pub mod dvbusdis;
/**DVBUSPULSE (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`dvbuspulse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbuspulse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dvbuspulse`] module*/
pub type DVBUSPULSE = crate::Reg<dvbuspulse::DVBUSPULSE_SPEC>;
///
pub mod dvbuspulse;
/**DTHRCTL (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`dthrctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dthrctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dthrctl`] module*/
pub type DTHRCTL = crate::Reg<dthrctl::DTHRCTL_SPEC>;
///
pub mod dthrctl;
/**DIEPEMPMSK (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`diepempmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepempmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@diepempmsk`] module*/
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>;
///
pub mod diepempmsk;
///Device IN endpoint 0
pub use self::in_ep0::IN_EP0;
///Cluster
///Device IN endpoint 0
pub mod in_ep0;
///Device IN endpoints 1-6
pub use self::in_ep::IN_EP;
///Cluster
///Device IN endpoints 1-6
pub mod in_ep;
///Device OUT endpoint 0
pub use self::out_ep0::OUT_EP0;
///Cluster
///Device OUT endpoint 0
pub mod out_ep0;
///Device OUT endpoints 1-6
pub use self::out_ep::OUT_EP;
///Cluster
///Device OUT endpoints 1-6
pub mod out_ep;
/**PCGCCTL (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`pcgcctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcgcctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pcgcctl`] module*/
pub type PCGCCTL = crate::Reg<pcgcctl::PCGCCTL_SPEC>;
///
pub mod pcgcctl;
/**FIFO (rw) register accessor: Read and write data to the USB FIFOs through this register.

You can [`read`](crate::generic::Reg::read) this register and get [`fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fifo`] module*/
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
///Read and write data to the USB FIFOs through this register.
pub mod fifo;
