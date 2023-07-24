#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub ctrl: CTRL,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Clock divider configuration register"]
    pub clkdiv: CLKDIV,
    #[doc = "0x0c - Clock source selection register"]
    pub clksrc: CLKSRC,
    #[doc = "0x10 - Clock enable register"]
    pub clkena: CLKENA,
    #[doc = "0x14 - Data and response timeout configuration register"]
    pub tmout: TMOUT,
    #[doc = "0x18 - Card bus width configuration register"]
    pub ctype: CTYPE,
    #[doc = "0x1c - Card data block size configuration register"]
    pub blksiz: BLKSIZ,
    #[doc = "0x20 - Data transfer length configuration register"]
    pub bytcnt: BYTCNT,
    #[doc = "0x24 - SDIO interrupt mask register"]
    pub intmask: INTMASK,
    #[doc = "0x28 - Command argument data register"]
    pub cmdarg: CMDARG,
    #[doc = "0x2c - Command and boot configuration register"]
    pub cmd: CMD,
    #[doc = "0x30 - Response data register"]
    pub resp0: RESP0,
    #[doc = "0x34 - Long response data register"]
    pub resp1: RESP1,
    #[doc = "0x38 - Long response data register"]
    pub resp2: RESP2,
    #[doc = "0x3c - Long response data register"]
    pub resp3: RESP3,
    #[doc = "0x40 - Masked interrupt status register"]
    pub mintsts: MINTSTS,
    #[doc = "0x44 - Raw interrupt status register"]
    pub rintsts: RINTSTS,
    #[doc = "0x48 - SD/MMC status register"]
    pub status: STATUS,
    #[doc = "0x4c - FIFO configuration register"]
    pub fifoth: FIFOTH,
    #[doc = "0x50 - Card detect register"]
    pub cdetect: CDETECT,
    #[doc = "0x54 - Card write protection (WP) status register"]
    pub wrtprt: WRTPRT,
    _reserved21: [u8; 0x04],
    #[doc = "0x5c - Transferred byte count register"]
    pub tcbcnt: TCBCNT,
    #[doc = "0x60 - Transferred byte count register"]
    pub tbbcnt: TBBCNT,
    #[doc = "0x64 - Debounce filter time configuration register"]
    pub debnce: DEBNCE,
    #[doc = "0x68 - User ID (scratchpad) register"]
    pub usrid: USRID,
    #[doc = "0x6c - Version ID (scratchpad) register"]
    pub verid: VERID,
    #[doc = "0x70 - Hardware feature register"]
    pub hcon: HCON,
    #[doc = "0x74 - UHS-1 register"]
    pub uhs: UHS,
    #[doc = "0x78 - Card reset register"]
    pub rst_n: RST_N,
    _reserved29: [u8; 0x04],
    #[doc = "0x80 - Burst mode transfer configuration register"]
    pub bmod: BMOD,
    #[doc = "0x84 - Poll demand configuration register"]
    pub pldmnd: PLDMND,
    #[doc = "0x88 - Descriptor base address register"]
    pub dbaddr: DBADDR,
    #[doc = "0x8c - IDMAC status register"]
    pub idsts: IDSTS,
    #[doc = "0x90 - IDMAC interrupt enable register"]
    pub idinten: IDINTEN,
    #[doc = "0x94 - Host descriptor address pointer"]
    pub dscaddr: DSCADDR,
    #[doc = "0x98 - Host buffer address pointer register"]
    pub bufaddr: BUFADDR,
    _reserved36: [u8; 0x64],
    #[doc = "0x100 - Card Threshold Control register"]
    pub cardthrctl: CARDTHRCTL,
    _reserved37: [u8; 0x08],
    #[doc = "0x10c - eMMC DDR register"]
    pub emmcddr: EMMCDDR,
    #[doc = "0x110 - Enable Phase Shift register"]
    pub enshift: ENSHIFT,
    _reserved39: [u8; 0xec],
    #[doc = "0x200 - CPU write and read transmit data by FIFO"]
    pub buffifo: BUFFIFO,
    _reserved40: [u8; 0x05fc],
    #[doc = "0x800 - SDIO control register."]
    pub clk_edge_sel: CLK_EDGE_SEL,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "CLKDIV (rw) register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "Clock divider configuration register"]
pub mod clkdiv;
#[doc = "CLKSRC (rw) register accessor: an alias for `Reg<CLKSRC_SPEC>`"]
pub type CLKSRC = crate::Reg<clksrc::CLKSRC_SPEC>;
#[doc = "Clock source selection register"]
pub mod clksrc;
#[doc = "CLKENA (rw) register accessor: an alias for `Reg<CLKENA_SPEC>`"]
pub type CLKENA = crate::Reg<clkena::CLKENA_SPEC>;
#[doc = "Clock enable register"]
pub mod clkena;
#[doc = "TMOUT (rw) register accessor: an alias for `Reg<TMOUT_SPEC>`"]
pub type TMOUT = crate::Reg<tmout::TMOUT_SPEC>;
#[doc = "Data and response timeout configuration register"]
pub mod tmout;
#[doc = "CTYPE (rw) register accessor: an alias for `Reg<CTYPE_SPEC>`"]
pub type CTYPE = crate::Reg<ctype::CTYPE_SPEC>;
#[doc = "Card bus width configuration register"]
pub mod ctype;
#[doc = "BLKSIZ (rw) register accessor: an alias for `Reg<BLKSIZ_SPEC>`"]
pub type BLKSIZ = crate::Reg<blksiz::BLKSIZ_SPEC>;
#[doc = "Card data block size configuration register"]
pub mod blksiz;
#[doc = "BYTCNT (rw) register accessor: an alias for `Reg<BYTCNT_SPEC>`"]
pub type BYTCNT = crate::Reg<bytcnt::BYTCNT_SPEC>;
#[doc = "Data transfer length configuration register"]
pub mod bytcnt;
#[doc = "INTMASK (rw) register accessor: an alias for `Reg<INTMASK_SPEC>`"]
pub type INTMASK = crate::Reg<intmask::INTMASK_SPEC>;
#[doc = "SDIO interrupt mask register"]
pub mod intmask;
#[doc = "CMDARG (rw) register accessor: an alias for `Reg<CMDARG_SPEC>`"]
pub type CMDARG = crate::Reg<cmdarg::CMDARG_SPEC>;
#[doc = "Command argument data register"]
pub mod cmdarg;
#[doc = "CMD (rw) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command and boot configuration register"]
pub mod cmd;
#[doc = "RESP0 (r) register accessor: an alias for `Reg<RESP0_SPEC>`"]
pub type RESP0 = crate::Reg<resp0::RESP0_SPEC>;
#[doc = "Response data register"]
pub mod resp0;
#[doc = "RESP1 (r) register accessor: an alias for `Reg<RESP1_SPEC>`"]
pub type RESP1 = crate::Reg<resp1::RESP1_SPEC>;
#[doc = "Long response data register"]
pub mod resp1;
#[doc = "RESP2 (r) register accessor: an alias for `Reg<RESP2_SPEC>`"]
pub type RESP2 = crate::Reg<resp2::RESP2_SPEC>;
#[doc = "Long response data register"]
pub mod resp2;
#[doc = "RESP3 (r) register accessor: an alias for `Reg<RESP3_SPEC>`"]
pub type RESP3 = crate::Reg<resp3::RESP3_SPEC>;
#[doc = "Long response data register"]
pub mod resp3;
#[doc = "MINTSTS (r) register accessor: an alias for `Reg<MINTSTS_SPEC>`"]
pub type MINTSTS = crate::Reg<mintsts::MINTSTS_SPEC>;
#[doc = "Masked interrupt status register"]
pub mod mintsts;
#[doc = "RINTSTS (rw) register accessor: an alias for `Reg<RINTSTS_SPEC>`"]
pub type RINTSTS = crate::Reg<rintsts::RINTSTS_SPEC>;
#[doc = "Raw interrupt status register"]
pub mod rintsts;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "SD/MMC status register"]
pub mod status;
#[doc = "FIFOTH (rw) register accessor: an alias for `Reg<FIFOTH_SPEC>`"]
pub type FIFOTH = crate::Reg<fifoth::FIFOTH_SPEC>;
#[doc = "FIFO configuration register"]
pub mod fifoth;
#[doc = "CDETECT (r) register accessor: an alias for `Reg<CDETECT_SPEC>`"]
pub type CDETECT = crate::Reg<cdetect::CDETECT_SPEC>;
#[doc = "Card detect register"]
pub mod cdetect;
#[doc = "WRTPRT (r) register accessor: an alias for `Reg<WRTPRT_SPEC>`"]
pub type WRTPRT = crate::Reg<wrtprt::WRTPRT_SPEC>;
#[doc = "Card write protection (WP) status register"]
pub mod wrtprt;
#[doc = "TCBCNT (r) register accessor: an alias for `Reg<TCBCNT_SPEC>`"]
pub type TCBCNT = crate::Reg<tcbcnt::TCBCNT_SPEC>;
#[doc = "Transferred byte count register"]
pub mod tcbcnt;
#[doc = "TBBCNT (r) register accessor: an alias for `Reg<TBBCNT_SPEC>`"]
pub type TBBCNT = crate::Reg<tbbcnt::TBBCNT_SPEC>;
#[doc = "Transferred byte count register"]
pub mod tbbcnt;
#[doc = "DEBNCE (rw) register accessor: an alias for `Reg<DEBNCE_SPEC>`"]
pub type DEBNCE = crate::Reg<debnce::DEBNCE_SPEC>;
#[doc = "Debounce filter time configuration register"]
pub mod debnce;
#[doc = "USRID (rw) register accessor: an alias for `Reg<USRID_SPEC>`"]
pub type USRID = crate::Reg<usrid::USRID_SPEC>;
#[doc = "User ID (scratchpad) register"]
pub mod usrid;
#[doc = "VERID (r) register accessor: an alias for `Reg<VERID_SPEC>`"]
pub type VERID = crate::Reg<verid::VERID_SPEC>;
#[doc = "Version ID (scratchpad) register"]
pub mod verid;
#[doc = "HCON (r) register accessor: an alias for `Reg<HCON_SPEC>`"]
pub type HCON = crate::Reg<hcon::HCON_SPEC>;
#[doc = "Hardware feature register"]
pub mod hcon;
#[doc = "UHS (rw) register accessor: an alias for `Reg<UHS_SPEC>`"]
pub type UHS = crate::Reg<uhs::UHS_SPEC>;
#[doc = "UHS-1 register"]
pub mod uhs;
#[doc = "RST_N (rw) register accessor: an alias for `Reg<RST_N_SPEC>`"]
pub type RST_N = crate::Reg<rst_n::RST_N_SPEC>;
#[doc = "Card reset register"]
pub mod rst_n;
#[doc = "BMOD (rw) register accessor: an alias for `Reg<BMOD_SPEC>`"]
pub type BMOD = crate::Reg<bmod::BMOD_SPEC>;
#[doc = "Burst mode transfer configuration register"]
pub mod bmod;
#[doc = "PLDMND (w) register accessor: an alias for `Reg<PLDMND_SPEC>`"]
pub type PLDMND = crate::Reg<pldmnd::PLDMND_SPEC>;
#[doc = "Poll demand configuration register"]
pub mod pldmnd;
#[doc = "DBADDR (rw) register accessor: an alias for `Reg<DBADDR_SPEC>`"]
pub type DBADDR = crate::Reg<dbaddr::DBADDR_SPEC>;
#[doc = "Descriptor base address register"]
pub mod dbaddr;
#[doc = "IDSTS (rw) register accessor: an alias for `Reg<IDSTS_SPEC>`"]
pub type IDSTS = crate::Reg<idsts::IDSTS_SPEC>;
#[doc = "IDMAC status register"]
pub mod idsts;
#[doc = "IDINTEN (rw) register accessor: an alias for `Reg<IDINTEN_SPEC>`"]
pub type IDINTEN = crate::Reg<idinten::IDINTEN_SPEC>;
#[doc = "IDMAC interrupt enable register"]
pub mod idinten;
#[doc = "DSCADDR (r) register accessor: an alias for `Reg<DSCADDR_SPEC>`"]
pub type DSCADDR = crate::Reg<dscaddr::DSCADDR_SPEC>;
#[doc = "Host descriptor address pointer"]
pub mod dscaddr;
#[doc = "BUFADDR (r) register accessor: an alias for `Reg<BUFADDR_SPEC>`"]
pub type BUFADDR = crate::Reg<bufaddr::BUFADDR_SPEC>;
#[doc = "Host buffer address pointer register"]
pub mod bufaddr;
#[doc = "CARDTHRCTL (rw) register accessor: an alias for `Reg<CARDTHRCTL_SPEC>`"]
pub type CARDTHRCTL = crate::Reg<cardthrctl::CARDTHRCTL_SPEC>;
#[doc = "Card Threshold Control register"]
pub mod cardthrctl;
#[doc = "EMMCDDR (rw) register accessor: an alias for `Reg<EMMCDDR_SPEC>`"]
pub type EMMCDDR = crate::Reg<emmcddr::EMMCDDR_SPEC>;
#[doc = "eMMC DDR register"]
pub mod emmcddr;
#[doc = "ENSHIFT (rw) register accessor: an alias for `Reg<ENSHIFT_SPEC>`"]
pub type ENSHIFT = crate::Reg<enshift::ENSHIFT_SPEC>;
#[doc = "Enable Phase Shift register"]
pub mod enshift;
#[doc = "BUFFIFO (rw) register accessor: an alias for `Reg<BUFFIFO_SPEC>`"]
pub type BUFFIFO = crate::Reg<buffifo::BUFFIFO_SPEC>;
#[doc = "CPU write and read transmit data by FIFO"]
pub mod buffifo;
#[doc = "CLK_EDGE_SEL (rw) register accessor: an alias for `Reg<CLK_EDGE_SEL_SPEC>`"]
pub type CLK_EDGE_SEL = crate::Reg<clk_edge_sel::CLK_EDGE_SEL_SPEC>;
#[doc = "SDIO control register."]
pub mod clk_edge_sel;
