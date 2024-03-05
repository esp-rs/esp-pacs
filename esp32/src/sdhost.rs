#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: CTRL,
    _reserved1: [u8; 0x04],
    clkdiv: CLKDIV,
    clksrc: CLKSRC,
    clkena: CLKENA,
    tmout: TMOUT,
    ctype: CTYPE,
    blksiz: BLKSIZ,
    bytcnt: BYTCNT,
    intmask: INTMASK,
    cmdarg: CMDARG,
    cmd: CMD,
    resp0: RESP0,
    resp1: RESP1,
    resp2: RESP2,
    resp3: RESP3,
    mintsts: MINTSTS,
    rintsts: RINTSTS,
    status: STATUS,
    fifoth: FIFOTH,
    cdetect: CDETECT,
    wrtprt: WRTPRT,
    _reserved21: [u8; 0x04],
    tcbcnt: TCBCNT,
    tbbcnt: TBBCNT,
    debnce: DEBNCE,
    usrid: USRID,
    verid: VERID,
    hcon: HCON,
    uhs: UHS,
    rst_n: RST_N,
    _reserved29: [u8; 0x04],
    bmod: BMOD,
    pldmnd: PLDMND,
    dbaddr: DBADDR,
    idsts: IDSTS,
    idinten: IDINTEN,
    dscaddr: DSCADDR,
    bufaddr: BUFADDR,
    _reserved36: [u8; 0x64],
    cardthrctl: CARDTHRCTL,
    _reserved37: [u8; 0x08],
    emmcddr: EMMCDDR,
    enshift: ENSHIFT,
    _reserved39: [u8; 0xec],
    buffifo: BUFFIFO,
    _reserved40: [u8; 0x05fc],
    clk_edge_sel: CLK_EDGE_SEL,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x08 - Clock divider configuration register"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &CLKDIV {
        &self.clkdiv
    }
    #[doc = "0x0c - Clock source selection register"]
    #[inline(always)]
    pub const fn clksrc(&self) -> &CLKSRC {
        &self.clksrc
    }
    #[doc = "0x10 - Clock enable register"]
    #[inline(always)]
    pub const fn clkena(&self) -> &CLKENA {
        &self.clkena
    }
    #[doc = "0x14 - Data and response timeout configuration register"]
    #[inline(always)]
    pub const fn tmout(&self) -> &TMOUT {
        &self.tmout
    }
    #[doc = "0x18 - Card bus width configuration register"]
    #[inline(always)]
    pub const fn ctype(&self) -> &CTYPE {
        &self.ctype
    }
    #[doc = "0x1c - Card data block size configuration register"]
    #[inline(always)]
    pub const fn blksiz(&self) -> &BLKSIZ {
        &self.blksiz
    }
    #[doc = "0x20 - Data transfer length configuration register"]
    #[inline(always)]
    pub const fn bytcnt(&self) -> &BYTCNT {
        &self.bytcnt
    }
    #[doc = "0x24 - SDIO interrupt mask register"]
    #[inline(always)]
    pub const fn intmask(&self) -> &INTMASK {
        &self.intmask
    }
    #[doc = "0x28 - Command argument data register"]
    #[inline(always)]
    pub const fn cmdarg(&self) -> &CMDARG {
        &self.cmdarg
    }
    #[doc = "0x2c - Command and boot configuration register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x30 - Response data register"]
    #[inline(always)]
    pub const fn resp0(&self) -> &RESP0 {
        &self.resp0
    }
    #[doc = "0x34 - Long response data register"]
    #[inline(always)]
    pub const fn resp1(&self) -> &RESP1 {
        &self.resp1
    }
    #[doc = "0x38 - Long response data register"]
    #[inline(always)]
    pub const fn resp2(&self) -> &RESP2 {
        &self.resp2
    }
    #[doc = "0x3c - Long response data register"]
    #[inline(always)]
    pub const fn resp3(&self) -> &RESP3 {
        &self.resp3
    }
    #[doc = "0x40 - Masked interrupt status register"]
    #[inline(always)]
    pub const fn mintsts(&self) -> &MINTSTS {
        &self.mintsts
    }
    #[doc = "0x44 - Raw interrupt status register"]
    #[inline(always)]
    pub const fn rintsts(&self) -> &RINTSTS {
        &self.rintsts
    }
    #[doc = "0x48 - SD/MMC status register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x4c - FIFO configuration register"]
    #[inline(always)]
    pub const fn fifoth(&self) -> &FIFOTH {
        &self.fifoth
    }
    #[doc = "0x50 - Card detect register"]
    #[inline(always)]
    pub const fn cdetect(&self) -> &CDETECT {
        &self.cdetect
    }
    #[doc = "0x54 - Card write protection (WP) status register"]
    #[inline(always)]
    pub const fn wrtprt(&self) -> &WRTPRT {
        &self.wrtprt
    }
    #[doc = "0x5c - Transferred byte count register"]
    #[inline(always)]
    pub const fn tcbcnt(&self) -> &TCBCNT {
        &self.tcbcnt
    }
    #[doc = "0x60 - Transferred byte count register"]
    #[inline(always)]
    pub const fn tbbcnt(&self) -> &TBBCNT {
        &self.tbbcnt
    }
    #[doc = "0x64 - Debounce filter time configuration register"]
    #[inline(always)]
    pub const fn debnce(&self) -> &DEBNCE {
        &self.debnce
    }
    #[doc = "0x68 - User ID (scratchpad) register"]
    #[inline(always)]
    pub const fn usrid(&self) -> &USRID {
        &self.usrid
    }
    #[doc = "0x6c - Version ID (scratchpad) register"]
    #[inline(always)]
    pub const fn verid(&self) -> &VERID {
        &self.verid
    }
    #[doc = "0x70 - Hardware feature register"]
    #[inline(always)]
    pub const fn hcon(&self) -> &HCON {
        &self.hcon
    }
    #[doc = "0x74 - UHS-1 register"]
    #[inline(always)]
    pub const fn uhs(&self) -> &UHS {
        &self.uhs
    }
    #[doc = "0x78 - Card reset register"]
    #[inline(always)]
    pub const fn rst_n(&self) -> &RST_N {
        &self.rst_n
    }
    #[doc = "0x80 - Burst mode transfer configuration register"]
    #[inline(always)]
    pub const fn bmod(&self) -> &BMOD {
        &self.bmod
    }
    #[doc = "0x84 - Poll demand configuration register"]
    #[inline(always)]
    pub const fn pldmnd(&self) -> &PLDMND {
        &self.pldmnd
    }
    #[doc = "0x88 - Descriptor base address register"]
    #[inline(always)]
    pub const fn dbaddr(&self) -> &DBADDR {
        &self.dbaddr
    }
    #[doc = "0x8c - IDMAC status register"]
    #[inline(always)]
    pub const fn idsts(&self) -> &IDSTS {
        &self.idsts
    }
    #[doc = "0x90 - IDMAC interrupt enable register"]
    #[inline(always)]
    pub const fn idinten(&self) -> &IDINTEN {
        &self.idinten
    }
    #[doc = "0x94 - Host descriptor address pointer"]
    #[inline(always)]
    pub const fn dscaddr(&self) -> &DSCADDR {
        &self.dscaddr
    }
    #[doc = "0x98 - Host buffer address pointer register"]
    #[inline(always)]
    pub const fn bufaddr(&self) -> &BUFADDR {
        &self.bufaddr
    }
    #[doc = "0x100 - Card Threshold Control register"]
    #[inline(always)]
    pub const fn cardthrctl(&self) -> &CARDTHRCTL {
        &self.cardthrctl
    }
    #[doc = "0x10c - eMMC DDR register"]
    #[inline(always)]
    pub const fn emmcddr(&self) -> &EMMCDDR {
        &self.emmcddr
    }
    #[doc = "0x110 - Enable Phase Shift register"]
    #[inline(always)]
    pub const fn enshift(&self) -> &ENSHIFT {
        &self.enshift
    }
    #[doc = "0x200 - CPU write and read transmit data by FIFO"]
    #[inline(always)]
    pub const fn buffifo(&self) -> &BUFFIFO {
        &self.buffifo
    }
    #[doc = "0x800 - SDIO control register."]
    #[inline(always)]
    pub const fn clk_edge_sel(&self) -> &CLK_EDGE_SEL {
        &self.clk_edge_sel
    }
}
#[doc = "CTRL (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "CLKDIV (rw) register accessor: Clock divider configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`] module"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "Clock divider configuration register"]
pub mod clkdiv;
#[doc = "CLKSRC (rw) register accessor: Clock source selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksrc`] module"]
pub type CLKSRC = crate::Reg<clksrc::CLKSRC_SPEC>;
#[doc = "Clock source selection register"]
pub mod clksrc;
#[doc = "CLKENA (rw) register accessor: Clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkena`] module"]
pub type CLKENA = crate::Reg<clkena::CLKENA_SPEC>;
#[doc = "Clock enable register"]
pub mod clkena;
#[doc = "TMOUT (rw) register accessor: Data and response timeout configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmout`] module"]
pub type TMOUT = crate::Reg<tmout::TMOUT_SPEC>;
#[doc = "Data and response timeout configuration register"]
pub mod tmout;
#[doc = "CTYPE (rw) register accessor: Card bus width configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctype::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctype::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctype`] module"]
pub type CTYPE = crate::Reg<ctype::CTYPE_SPEC>;
#[doc = "Card bus width configuration register"]
pub mod ctype;
#[doc = "BLKSIZ (rw) register accessor: Card data block size configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blksiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blksiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blksiz`] module"]
pub type BLKSIZ = crate::Reg<blksiz::BLKSIZ_SPEC>;
#[doc = "Card data block size configuration register"]
pub mod blksiz;
#[doc = "BYTCNT (rw) register accessor: Data transfer length configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bytcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bytcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bytcnt`] module"]
pub type BYTCNT = crate::Reg<bytcnt::BYTCNT_SPEC>;
#[doc = "Data transfer length configuration register"]
pub mod bytcnt;
#[doc = "INTMASK (rw) register accessor: SDIO interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmask`] module"]
pub type INTMASK = crate::Reg<intmask::INTMASK_SPEC>;
#[doc = "SDIO interrupt mask register"]
pub mod intmask;
#[doc = "CMDARG (rw) register accessor: Command argument data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdarg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdarg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdarg`] module"]
pub type CMDARG = crate::Reg<cmdarg::CMDARG_SPEC>;
#[doc = "Command argument data register"]
pub mod cmdarg;
#[doc = "CMD (rw) register accessor: Command and boot configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command and boot configuration register"]
pub mod cmd;
#[doc = "RESP0 (r) register accessor: Response data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp0`] module"]
pub type RESP0 = crate::Reg<resp0::RESP0_SPEC>;
#[doc = "Response data register"]
pub mod resp0;
#[doc = "RESP1 (r) register accessor: Long response data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp1`] module"]
pub type RESP1 = crate::Reg<resp1::RESP1_SPEC>;
#[doc = "Long response data register"]
pub mod resp1;
#[doc = "RESP2 (r) register accessor: Long response data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp2`] module"]
pub type RESP2 = crate::Reg<resp2::RESP2_SPEC>;
#[doc = "Long response data register"]
pub mod resp2;
#[doc = "RESP3 (r) register accessor: Long response data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp3`] module"]
pub type RESP3 = crate::Reg<resp3::RESP3_SPEC>;
#[doc = "Long response data register"]
pub mod resp3;
#[doc = "MINTSTS (r) register accessor: Masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mintsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mintsts`] module"]
pub type MINTSTS = crate::Reg<mintsts::MINTSTS_SPEC>;
#[doc = "Masked interrupt status register"]
pub mod mintsts;
#[doc = "RINTSTS (rw) register accessor: Raw interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rintsts`] module"]
pub type RINTSTS = crate::Reg<rintsts::RINTSTS_SPEC>;
#[doc = "Raw interrupt status register"]
pub mod rintsts;
#[doc = "STATUS (r) register accessor: SD/MMC status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "SD/MMC status register"]
pub mod status;
#[doc = "FIFOTH (rw) register accessor: FIFO configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifoth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifoth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifoth`] module"]
pub type FIFOTH = crate::Reg<fifoth::FIFOTH_SPEC>;
#[doc = "FIFO configuration register"]
pub mod fifoth;
#[doc = "CDETECT (r) register accessor: Card detect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdetect::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdetect`] module"]
pub type CDETECT = crate::Reg<cdetect::CDETECT_SPEC>;
#[doc = "Card detect register"]
pub mod cdetect;
#[doc = "WRTPRT (r) register accessor: Card write protection (WP) status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrtprt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrtprt`] module"]
pub type WRTPRT = crate::Reg<wrtprt::WRTPRT_SPEC>;
#[doc = "Card write protection (WP) status register"]
pub mod wrtprt;
#[doc = "TCBCNT (r) register accessor: Transferred byte count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcbcnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcbcnt`] module"]
pub type TCBCNT = crate::Reg<tcbcnt::TCBCNT_SPEC>;
#[doc = "Transferred byte count register"]
pub mod tcbcnt;
#[doc = "TBBCNT (r) register accessor: Transferred byte count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbbcnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbbcnt`] module"]
pub type TBBCNT = crate::Reg<tbbcnt::TBBCNT_SPEC>;
#[doc = "Transferred byte count register"]
pub mod tbbcnt;
#[doc = "DEBNCE (rw) register accessor: Debounce filter time configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debnce::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debnce::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debnce`] module"]
pub type DEBNCE = crate::Reg<debnce::DEBNCE_SPEC>;
#[doc = "Debounce filter time configuration register"]
pub mod debnce;
#[doc = "USRID (rw) register accessor: User ID (scratchpad) register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usrid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usrid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usrid`] module"]
pub type USRID = crate::Reg<usrid::USRID_SPEC>;
#[doc = "User ID (scratchpad) register"]
pub mod usrid;
#[doc = "VERID (r) register accessor: Version ID (scratchpad) register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`verid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@verid`] module"]
pub type VERID = crate::Reg<verid::VERID_SPEC>;
#[doc = "Version ID (scratchpad) register"]
pub mod verid;
#[doc = "HCON (r) register accessor: Hardware feature register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcon`] module"]
pub type HCON = crate::Reg<hcon::HCON_SPEC>;
#[doc = "Hardware feature register"]
pub mod hcon;
#[doc = "UHS (rw) register accessor: UHS-1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uhs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uhs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs`] module"]
pub type UHS = crate::Reg<uhs::UHS_SPEC>;
#[doc = "UHS-1 register"]
pub mod uhs;
#[doc = "RST_N (rw) register accessor: Card reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst_n::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst_n::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_n`] module"]
pub type RST_N = crate::Reg<rst_n::RST_N_SPEC>;
#[doc = "Card reset register"]
pub mod rst_n;
#[doc = "BMOD (rw) register accessor: Burst mode transfer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmod`] module"]
pub type BMOD = crate::Reg<bmod::BMOD_SPEC>;
#[doc = "Burst mode transfer configuration register"]
pub mod bmod;
#[doc = "PLDMND (w) register accessor: Poll demand configuration register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pldmnd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pldmnd`] module"]
pub type PLDMND = crate::Reg<pldmnd::PLDMND_SPEC>;
#[doc = "Poll demand configuration register"]
pub mod pldmnd;
#[doc = "DBADDR (rw) register accessor: Descriptor base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbaddr`] module"]
pub type DBADDR = crate::Reg<dbaddr::DBADDR_SPEC>;
#[doc = "Descriptor base address register"]
pub mod dbaddr;
#[doc = "IDSTS (rw) register accessor: IDMAC status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idsts`] module"]
pub type IDSTS = crate::Reg<idsts::IDSTS_SPEC>;
#[doc = "IDMAC status register"]
pub mod idsts;
#[doc = "IDINTEN (rw) register accessor: IDMAC interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idinten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idinten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idinten`] module"]
pub type IDINTEN = crate::Reg<idinten::IDINTEN_SPEC>;
#[doc = "IDMAC interrupt enable register"]
pub mod idinten;
#[doc = "DSCADDR (r) register accessor: Host descriptor address pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dscaddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dscaddr`] module"]
pub type DSCADDR = crate::Reg<dscaddr::DSCADDR_SPEC>;
#[doc = "Host descriptor address pointer"]
pub mod dscaddr;
#[doc = "BUFADDR (r) register accessor: Host buffer address pointer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bufaddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bufaddr`] module"]
pub type BUFADDR = crate::Reg<bufaddr::BUFADDR_SPEC>;
#[doc = "Host buffer address pointer register"]
pub mod bufaddr;
#[doc = "CARDTHRCTL (rw) register accessor: Card Threshold Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cardthrctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cardthrctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cardthrctl`] module"]
pub type CARDTHRCTL = crate::Reg<cardthrctl::CARDTHRCTL_SPEC>;
#[doc = "Card Threshold Control register"]
pub mod cardthrctl;
#[doc = "EMMCDDR (rw) register accessor: eMMC DDR register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmcddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmcddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmcddr`] module"]
pub type EMMCDDR = crate::Reg<emmcddr::EMMCDDR_SPEC>;
#[doc = "eMMC DDR register"]
pub mod emmcddr;
#[doc = "ENSHIFT (rw) register accessor: Enable Phase Shift register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enshift::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enshift::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enshift`] module"]
pub type ENSHIFT = crate::Reg<enshift::ENSHIFT_SPEC>;
#[doc = "Enable Phase Shift register"]
pub mod enshift;
#[doc = "BUFFIFO (rw) register accessor: CPU write and read transmit data by FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buffifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buffifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buffifo`] module"]
pub type BUFFIFO = crate::Reg<buffifo::BUFFIFO_SPEC>;
#[doc = "CPU write and read transmit data by FIFO"]
pub mod buffifo;
#[doc = "CLK_EDGE_SEL (rw) register accessor: SDIO control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_edge_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_edge_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_edge_sel`] module"]
pub type CLK_EDGE_SEL = crate::Reg<clk_edge_sel::CLK_EDGE_SEL_SPEC>;
#[doc = "SDIO control register."]
pub mod clk_edge_sel;
