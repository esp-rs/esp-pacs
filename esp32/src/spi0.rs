#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    cmd: CMD,
    addr: ADDR,
    ctrl: CTRL,
    ctrl1: CTRL1,
    rd_status: RD_STATUS,
    ctrl2: CTRL2,
    clock: CLOCK,
    user: USER,
    user1: USER1,
    user2: USER2,
    mosi_dlen: MOSI_DLEN,
    miso_dlen: MISO_DLEN,
    slv_wr_status: SLV_WR_STATUS,
    pin: PIN,
    slave: SLAVE,
    slave1: SLAVE1,
    slave2: SLAVE2,
    slave3: SLAVE3,
    slv_wrbuf_dlen: SLV_WRBUF_DLEN,
    slv_rdbuf_dlen: SLV_RDBUF_DLEN,
    cache_fctrl: CACHE_FCTRL,
    cache_sctrl: CACHE_SCTRL,
    sram_cmd: SRAM_CMD,
    sram_drd_cmd: SRAM_DRD_CMD,
    sram_dwr_cmd: SRAM_DWR_CMD,
    slv_rd_bit: SLV_RD_BIT,
    _reserved26: [u8; 0x18],
    w: [W; 16],
    tx_crc: TX_CRC,
    _reserved28: [u8; 0x2c],
    ext0: EXT0,
    ext1: EXT1,
    ext2: EXT2,
    ext3: EXT3,
    dma_conf: DMA_CONF,
    dma_out_link: DMA_OUT_LINK,
    dma_in_link: DMA_IN_LINK,
    dma_status: DMA_STATUS,
    dma_int_ena: DMA_INT_ENA,
    dma_int_raw: DMA_INT_RAW,
    dma_int_st: DMA_INT_ST,
    dma_int_clr: DMA_INT_CLR,
    in_err_eof_des_addr: IN_ERR_EOF_DES_ADDR,
    in_suc_eof_des_addr: IN_SUC_EOF_DES_ADDR,
    inlink_dscr: INLINK_DSCR,
    inlink_dscr_bf0: INLINK_DSCR_BF0,
    inlink_dscr_bf1: INLINK_DSCR_BF1,
    out_eof_bfr_des_addr: OUT_EOF_BFR_DES_ADDR,
    out_eof_des_addr: OUT_EOF_DES_ADDR,
    outlink_dscr: OUTLINK_DSCR,
    outlink_dscr_bf0: OUTLINK_DSCR_BF0,
    outlink_dscr_bf1: OUTLINK_DSCR_BF1,
    dma_rstatus: DMA_RSTATUS,
    dma_tstatus: DMA_TSTATUS,
    _reserved52: [u8; 0x02ac],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn addr(&self) -> &ADDR {
        &self.addr
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &CTRL1 {
        &self.ctrl1
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn rd_status(&self) -> &RD_STATUS {
        &self.rd_status
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &CTRL2 {
        &self.ctrl2
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn clock(&self) -> &CLOCK {
        &self.clock
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn user(&self) -> &USER {
        &self.user
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn user1(&self) -> &USER1 {
        &self.user1
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn user2(&self) -> &USER2 {
        &self.user2
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn mosi_dlen(&self) -> &MOSI_DLEN {
        &self.mosi_dlen
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn miso_dlen(&self) -> &MISO_DLEN {
        &self.miso_dlen
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn slv_wr_status(&self) -> &SLV_WR_STATUS {
        &self.slv_wr_status
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn pin(&self) -> &PIN {
        &self.pin
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn slave(&self) -> &SLAVE {
        &self.slave
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn slave1(&self) -> &SLAVE1 {
        &self.slave1
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn slave2(&self) -> &SLAVE2 {
        &self.slave2
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn slave3(&self) -> &SLAVE3 {
        &self.slave3
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn slv_wrbuf_dlen(&self) -> &SLV_WRBUF_DLEN {
        &self.slv_wrbuf_dlen
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn slv_rdbuf_dlen(&self) -> &SLV_RDBUF_DLEN {
        &self.slv_rdbuf_dlen
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn cache_fctrl(&self) -> &CACHE_FCTRL {
        &self.cache_fctrl
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn cache_sctrl(&self) -> &CACHE_SCTRL {
        &self.cache_sctrl
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn sram_cmd(&self) -> &SRAM_CMD {
        &self.sram_cmd
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn sram_drd_cmd(&self) -> &SRAM_DRD_CMD {
        &self.sram_drd_cmd
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn sram_dwr_cmd(&self) -> &SRAM_DWR_CMD {
        &self.sram_dwr_cmd
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn slv_rd_bit(&self) -> &SLV_RD_BIT {
        &self.slv_rd_bit
    }
    #[doc = "0x80..0xc0 - "]
    #[inline(always)]
    pub const fn w(&self, n: usize) -> &W {
        &self.w[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0xc0 - "]
    #[inline(always)]
    pub fn w_iter(&self) -> impl Iterator<Item = &W> {
        self.w.iter()
    }
    #[doc = "0xc0 - "]
    #[inline(always)]
    pub const fn tx_crc(&self) -> &TX_CRC {
        &self.tx_crc
    }
    #[doc = "0xf0 - "]
    #[inline(always)]
    pub const fn ext0(&self) -> &EXT0 {
        &self.ext0
    }
    #[doc = "0xf4 - "]
    #[inline(always)]
    pub const fn ext1(&self) -> &EXT1 {
        &self.ext1
    }
    #[doc = "0xf8 - "]
    #[inline(always)]
    pub const fn ext2(&self) -> &EXT2 {
        &self.ext2
    }
    #[doc = "0xfc - "]
    #[inline(always)]
    pub const fn ext3(&self) -> &EXT3 {
        &self.ext3
    }
    #[doc = "0x100 - "]
    #[inline(always)]
    pub const fn dma_conf(&self) -> &DMA_CONF {
        &self.dma_conf
    }
    #[doc = "0x104 - "]
    #[inline(always)]
    pub const fn dma_out_link(&self) -> &DMA_OUT_LINK {
        &self.dma_out_link
    }
    #[doc = "0x108 - "]
    #[inline(always)]
    pub const fn dma_in_link(&self) -> &DMA_IN_LINK {
        &self.dma_in_link
    }
    #[doc = "0x10c - "]
    #[inline(always)]
    pub const fn dma_status(&self) -> &DMA_STATUS {
        &self.dma_status
    }
    #[doc = "0x110 - "]
    #[inline(always)]
    pub const fn dma_int_ena(&self) -> &DMA_INT_ENA {
        &self.dma_int_ena
    }
    #[doc = "0x114 - "]
    #[inline(always)]
    pub const fn dma_int_raw(&self) -> &DMA_INT_RAW {
        &self.dma_int_raw
    }
    #[doc = "0x118 - "]
    #[inline(always)]
    pub const fn dma_int_st(&self) -> &DMA_INT_ST {
        &self.dma_int_st
    }
    #[doc = "0x11c - "]
    #[inline(always)]
    pub const fn dma_int_clr(&self) -> &DMA_INT_CLR {
        &self.dma_int_clr
    }
    #[doc = "0x120 - "]
    #[inline(always)]
    pub const fn in_err_eof_des_addr(&self) -> &IN_ERR_EOF_DES_ADDR {
        &self.in_err_eof_des_addr
    }
    #[doc = "0x124 - "]
    #[inline(always)]
    pub const fn in_suc_eof_des_addr(&self) -> &IN_SUC_EOF_DES_ADDR {
        &self.in_suc_eof_des_addr
    }
    #[doc = "0x128 - "]
    #[inline(always)]
    pub const fn inlink_dscr(&self) -> &INLINK_DSCR {
        &self.inlink_dscr
    }
    #[doc = "0x12c - "]
    #[inline(always)]
    pub const fn inlink_dscr_bf0(&self) -> &INLINK_DSCR_BF0 {
        &self.inlink_dscr_bf0
    }
    #[doc = "0x130 - "]
    #[inline(always)]
    pub const fn inlink_dscr_bf1(&self) -> &INLINK_DSCR_BF1 {
        &self.inlink_dscr_bf1
    }
    #[doc = "0x134 - "]
    #[inline(always)]
    pub const fn out_eof_bfr_des_addr(&self) -> &OUT_EOF_BFR_DES_ADDR {
        &self.out_eof_bfr_des_addr
    }
    #[doc = "0x138 - "]
    #[inline(always)]
    pub const fn out_eof_des_addr(&self) -> &OUT_EOF_DES_ADDR {
        &self.out_eof_des_addr
    }
    #[doc = "0x13c - "]
    #[inline(always)]
    pub const fn outlink_dscr(&self) -> &OUTLINK_DSCR {
        &self.outlink_dscr
    }
    #[doc = "0x140 - "]
    #[inline(always)]
    pub const fn outlink_dscr_bf0(&self) -> &OUTLINK_DSCR_BF0 {
        &self.outlink_dscr_bf0
    }
    #[doc = "0x144 - "]
    #[inline(always)]
    pub const fn outlink_dscr_bf1(&self) -> &OUTLINK_DSCR_BF1 {
        &self.outlink_dscr_bf1
    }
    #[doc = "0x148 - "]
    #[inline(always)]
    pub const fn dma_rstatus(&self) -> &DMA_RSTATUS {
        &self.dma_rstatus
    }
    #[doc = "0x14c - "]
    #[inline(always)]
    pub const fn dma_tstatus(&self) -> &DMA_TSTATUS {
        &self.dma_tstatus
    }
    #[doc = "0x3fc - "]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CMD (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = ""]
pub mod cmd;
#[doc = "ADDR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`] module"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = ""]
pub mod addr;
#[doc = "CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = ""]
pub mod ctrl;
#[doc = "CTRL1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`] module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = ""]
pub mod ctrl1;
#[doc = "RD_STATUS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_status`] module"]
pub type RD_STATUS = crate::Reg<rd_status::RD_STATUS_SPEC>;
#[doc = ""]
pub mod rd_status;
#[doc = "CTRL2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`] module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = ""]
pub mod ctrl2;
#[doc = "CLOCK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock`] module"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = ""]
pub mod clock;
#[doc = "USER (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`user::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user`] module"]
pub type USER = crate::Reg<user::USER_SPEC>;
#[doc = ""]
pub mod user;
#[doc = "USER1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`user1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user1`] module"]
pub type USER1 = crate::Reg<user1::USER1_SPEC>;
#[doc = ""]
pub mod user1;
#[doc = "USER2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`user2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user2`] module"]
pub type USER2 = crate::Reg<user2::USER2_SPEC>;
#[doc = ""]
pub mod user2;
#[doc = "MOSI_DLEN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mosi_dlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mosi_dlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mosi_dlen`] module"]
pub type MOSI_DLEN = crate::Reg<mosi_dlen::MOSI_DLEN_SPEC>;
#[doc = ""]
pub mod mosi_dlen;
#[doc = "MISO_DLEN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`miso_dlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`miso_dlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@miso_dlen`] module"]
pub type MISO_DLEN = crate::Reg<miso_dlen::MISO_DLEN_SPEC>;
#[doc = ""]
pub mod miso_dlen;
#[doc = "SLV_WR_STATUS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slv_wr_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slv_wr_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slv_wr_status`] module"]
pub type SLV_WR_STATUS = crate::Reg<slv_wr_status::SLV_WR_STATUS_SPEC>;
#[doc = ""]
pub mod slv_wr_status;
#[doc = "PIN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin`] module"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = ""]
pub mod pin;
#[doc = "SLAVE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slave::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slave`] module"]
pub type SLAVE = crate::Reg<slave::SLAVE_SPEC>;
#[doc = ""]
pub mod slave;
#[doc = "SLAVE1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slave1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slave1`] module"]
pub type SLAVE1 = crate::Reg<slave1::SLAVE1_SPEC>;
#[doc = ""]
pub mod slave1;
#[doc = "SLAVE2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slave2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slave2`] module"]
pub type SLAVE2 = crate::Reg<slave2::SLAVE2_SPEC>;
#[doc = ""]
pub mod slave2;
#[doc = "SLAVE3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slave3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slave3`] module"]
pub type SLAVE3 = crate::Reg<slave3::SLAVE3_SPEC>;
#[doc = ""]
pub mod slave3;
#[doc = "SLV_WRBUF_DLEN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slv_wrbuf_dlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slv_wrbuf_dlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slv_wrbuf_dlen`] module"]
pub type SLV_WRBUF_DLEN = crate::Reg<slv_wrbuf_dlen::SLV_WRBUF_DLEN_SPEC>;
#[doc = ""]
pub mod slv_wrbuf_dlen;
#[doc = "SLV_RDBUF_DLEN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slv_rdbuf_dlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slv_rdbuf_dlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slv_rdbuf_dlen`] module"]
pub type SLV_RDBUF_DLEN = crate::Reg<slv_rdbuf_dlen::SLV_RDBUF_DLEN_SPEC>;
#[doc = ""]
pub mod slv_rdbuf_dlen;
#[doc = "CACHE_FCTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_fctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_fctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_fctrl`] module"]
pub type CACHE_FCTRL = crate::Reg<cache_fctrl::CACHE_FCTRL_SPEC>;
#[doc = ""]
pub mod cache_fctrl;
#[doc = "CACHE_SCTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_sctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_sctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_sctrl`] module"]
pub type CACHE_SCTRL = crate::Reg<cache_sctrl::CACHE_SCTRL_SPEC>;
#[doc = ""]
pub mod cache_sctrl;
#[doc = "SRAM_CMD (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_cmd`] module"]
pub type SRAM_CMD = crate::Reg<sram_cmd::SRAM_CMD_SPEC>;
#[doc = ""]
pub mod sram_cmd;
#[doc = "SRAM_DRD_CMD (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_drd_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_drd_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_drd_cmd`] module"]
pub type SRAM_DRD_CMD = crate::Reg<sram_drd_cmd::SRAM_DRD_CMD_SPEC>;
#[doc = ""]
pub mod sram_drd_cmd;
#[doc = "SRAM_DWR_CMD (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_dwr_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_dwr_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_dwr_cmd`] module"]
pub type SRAM_DWR_CMD = crate::Reg<sram_dwr_cmd::SRAM_DWR_CMD_SPEC>;
#[doc = ""]
pub mod sram_dwr_cmd;
#[doc = "SLV_RD_BIT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slv_rd_bit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slv_rd_bit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slv_rd_bit`] module"]
pub type SLV_RD_BIT = crate::Reg<slv_rd_bit::SLV_RD_BIT_SPEC>;
#[doc = ""]
pub mod slv_rd_bit;
#[doc = "W (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w`] module"]
pub type W = crate::Reg<w::W_SPEC>;
#[doc = ""]
pub mod w;
#[doc = "TX_CRC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_crc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_crc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_crc`] module"]
pub type TX_CRC = crate::Reg<tx_crc::TX_CRC_SPEC>;
#[doc = ""]
pub mod tx_crc;
#[doc = "EXT0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext0`] module"]
pub type EXT0 = crate::Reg<ext0::EXT0_SPEC>;
#[doc = ""]
pub mod ext0;
#[doc = "EXT1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext1`] module"]
pub type EXT1 = crate::Reg<ext1::EXT1_SPEC>;
#[doc = ""]
pub mod ext1;
#[doc = "EXT2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext2`] module"]
pub type EXT2 = crate::Reg<ext2::EXT2_SPEC>;
#[doc = ""]
pub mod ext2;
#[doc = "EXT3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext3`] module"]
pub type EXT3 = crate::Reg<ext3::EXT3_SPEC>;
#[doc = ""]
pub mod ext3;
#[doc = "DMA_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_conf`] module"]
pub type DMA_CONF = crate::Reg<dma_conf::DMA_CONF_SPEC>;
#[doc = ""]
pub mod dma_conf;
#[doc = "DMA_OUT_LINK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_out_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_out_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_out_link`] module"]
pub type DMA_OUT_LINK = crate::Reg<dma_out_link::DMA_OUT_LINK_SPEC>;
#[doc = ""]
pub mod dma_out_link;
#[doc = "DMA_IN_LINK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_in_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_in_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_in_link`] module"]
pub type DMA_IN_LINK = crate::Reg<dma_in_link::DMA_IN_LINK_SPEC>;
#[doc = ""]
pub mod dma_in_link;
#[doc = "DMA_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_status`] module"]
pub type DMA_STATUS = crate::Reg<dma_status::DMA_STATUS_SPEC>;
#[doc = ""]
pub mod dma_status;
#[doc = "DMA_INT_ENA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_ena`] module"]
pub type DMA_INT_ENA = crate::Reg<dma_int_ena::DMA_INT_ENA_SPEC>;
#[doc = ""]
pub mod dma_int_ena;
#[doc = "DMA_INT_RAW (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_raw`] module"]
pub type DMA_INT_RAW = crate::Reg<dma_int_raw::DMA_INT_RAW_SPEC>;
#[doc = ""]
pub mod dma_int_raw;
#[doc = "DMA_INT_ST (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_st`] module"]
pub type DMA_INT_ST = crate::Reg<dma_int_st::DMA_INT_ST_SPEC>;
#[doc = ""]
pub mod dma_int_st;
#[doc = "DMA_INT_CLR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_int_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_int_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_clr`] module"]
pub type DMA_INT_CLR = crate::Reg<dma_int_clr::DMA_INT_CLR_SPEC>;
#[doc = ""]
pub mod dma_int_clr;
#[doc = "IN_ERR_EOF_DES_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_err_eof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_err_eof_des_addr`] module"]
pub type IN_ERR_EOF_DES_ADDR = crate::Reg<in_err_eof_des_addr::IN_ERR_EOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod in_err_eof_des_addr;
#[doc = "IN_SUC_EOF_DES_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_suc_eof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_suc_eof_des_addr`] module"]
pub type IN_SUC_EOF_DES_ADDR = crate::Reg<in_suc_eof_des_addr::IN_SUC_EOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod in_suc_eof_des_addr;
#[doc = "INLINK_DSCR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inlink_dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inlink_dscr`] module"]
pub type INLINK_DSCR = crate::Reg<inlink_dscr::INLINK_DSCR_SPEC>;
#[doc = ""]
pub mod inlink_dscr;
#[doc = "INLINK_DSCR_BF0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inlink_dscr_bf0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inlink_dscr_bf0`] module"]
pub type INLINK_DSCR_BF0 = crate::Reg<inlink_dscr_bf0::INLINK_DSCR_BF0_SPEC>;
#[doc = ""]
pub mod inlink_dscr_bf0;
#[doc = "INLINK_DSCR_BF1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inlink_dscr_bf1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inlink_dscr_bf1`] module"]
pub type INLINK_DSCR_BF1 = crate::Reg<inlink_dscr_bf1::INLINK_DSCR_BF1_SPEC>;
#[doc = ""]
pub mod inlink_dscr_bf1;
#[doc = "OUT_EOF_BFR_DES_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_eof_bfr_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_bfr_des_addr`] module"]
pub type OUT_EOF_BFR_DES_ADDR = crate::Reg<out_eof_bfr_des_addr::OUT_EOF_BFR_DES_ADDR_SPEC>;
#[doc = ""]
pub mod out_eof_bfr_des_addr;
#[doc = "OUT_EOF_DES_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_eof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_des_addr`] module"]
pub type OUT_EOF_DES_ADDR = crate::Reg<out_eof_des_addr::OUT_EOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod out_eof_des_addr;
#[doc = "OUTLINK_DSCR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outlink_dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outlink_dscr`] module"]
pub type OUTLINK_DSCR = crate::Reg<outlink_dscr::OUTLINK_DSCR_SPEC>;
#[doc = ""]
pub mod outlink_dscr;
#[doc = "OUTLINK_DSCR_BF0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outlink_dscr_bf0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outlink_dscr_bf0`] module"]
pub type OUTLINK_DSCR_BF0 = crate::Reg<outlink_dscr_bf0::OUTLINK_DSCR_BF0_SPEC>;
#[doc = ""]
pub mod outlink_dscr_bf0;
#[doc = "OUTLINK_DSCR_BF1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outlink_dscr_bf1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outlink_dscr_bf1`] module"]
pub type OUTLINK_DSCR_BF1 = crate::Reg<outlink_dscr_bf1::OUTLINK_DSCR_BF1_SPEC>;
#[doc = ""]
pub mod outlink_dscr_bf1;
#[doc = "DMA_RSTATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_rstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_rstatus`] module"]
pub type DMA_RSTATUS = crate::Reg<dma_rstatus::DMA_RSTATUS_SPEC>;
#[doc = ""]
pub mod dma_rstatus;
#[doc = "DMA_TSTATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_tstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_tstatus`] module"]
pub type DMA_TSTATUS = crate::Reg<dma_tstatus::DMA_TSTATUS_SPEC>;
#[doc = ""]
pub mod dma_tstatus;
#[doc = "DATE (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
