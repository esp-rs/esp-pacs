#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub cmd: CMD,
    #[doc = "0x04 - "]
    pub addr: ADDR,
    #[doc = "0x08 - "]
    pub ctrl: CTRL,
    #[doc = "0x0c - "]
    pub ctrl1: CTRL1,
    #[doc = "0x10 - "]
    pub rd_status: RD_STATUS,
    #[doc = "0x14 - "]
    pub ctrl2: CTRL2,
    #[doc = "0x18 - "]
    pub clock: CLOCK,
    #[doc = "0x1c - "]
    pub user: USER,
    #[doc = "0x20 - "]
    pub user1: USER1,
    #[doc = "0x24 - "]
    pub user2: USER2,
    #[doc = "0x28 - "]
    pub mosi_dlen: MOSI_DLEN,
    #[doc = "0x2c - "]
    pub miso_dlen: MISO_DLEN,
    #[doc = "0x30 - "]
    pub slv_wr_status: SLV_WR_STATUS,
    #[doc = "0x34 - "]
    pub pin: PIN,
    #[doc = "0x38 - "]
    pub slave: SLAVE,
    #[doc = "0x3c - "]
    pub slave1: SLAVE1,
    #[doc = "0x40 - "]
    pub slave2: SLAVE2,
    #[doc = "0x44 - "]
    pub slave3: SLAVE3,
    #[doc = "0x48 - "]
    pub slv_wrbuf_dlen: SLV_WRBUF_DLEN,
    #[doc = "0x4c - "]
    pub slv_rdbuf_dlen: SLV_RDBUF_DLEN,
    #[doc = "0x50 - "]
    pub cache_fctrl: CACHE_FCTRL,
    #[doc = "0x54 - "]
    pub cache_sctrl: CACHE_SCTRL,
    #[doc = "0x58 - "]
    pub sram_cmd: SRAM_CMD,
    #[doc = "0x5c - "]
    pub sram_drd_cmd: SRAM_DRD_CMD,
    #[doc = "0x60 - "]
    pub sram_dwr_cmd: SRAM_DWR_CMD,
    #[doc = "0x64 - "]
    pub slv_rd_bit: SLV_RD_BIT,
    _reserved26: [u8; 0x18],
    #[doc = "0x80 - "]
    pub w0: W0,
    #[doc = "0x84 - "]
    pub w1: W1,
    #[doc = "0x88 - "]
    pub w2: W2,
    #[doc = "0x8c - "]
    pub w3: W3,
    #[doc = "0x90 - "]
    pub w4: W4,
    #[doc = "0x94 - "]
    pub w5: W5,
    #[doc = "0x98 - "]
    pub w6: W6,
    #[doc = "0x9c - "]
    pub w7: W7,
    #[doc = "0xa0 - "]
    pub w8: W8,
    #[doc = "0xa4 - "]
    pub w9: W9,
    #[doc = "0xa8 - "]
    pub w10: W10,
    #[doc = "0xac - "]
    pub w11: W11,
    #[doc = "0xb0 - "]
    pub w12: W12,
    #[doc = "0xb4 - "]
    pub w13: W13,
    #[doc = "0xb8 - "]
    pub w14: W14,
    #[doc = "0xbc - "]
    pub w15: W15,
    #[doc = "0xc0 - "]
    pub tx_crc: TX_CRC,
    _reserved43: [u8; 0x2c],
    #[doc = "0xf0 - "]
    pub ext0: EXT0,
    #[doc = "0xf4 - "]
    pub ext1: EXT1,
    #[doc = "0xf8 - "]
    pub ext2: EXT2,
    #[doc = "0xfc - "]
    pub ext3: EXT3,
    #[doc = "0x100 - "]
    pub dma_conf: DMA_CONF,
    #[doc = "0x104 - "]
    pub dma_out_link: DMA_OUT_LINK,
    #[doc = "0x108 - "]
    pub dma_in_link: DMA_IN_LINK,
    #[doc = "0x10c - "]
    pub dma_status: DMA_STATUS,
    #[doc = "0x110 - "]
    pub dma_int_ena: DMA_INT_ENA,
    #[doc = "0x114 - "]
    pub dma_int_raw: DMA_INT_RAW,
    #[doc = "0x118 - "]
    pub dma_int_st: DMA_INT_ST,
    #[doc = "0x11c - "]
    pub dma_int_clr: DMA_INT_CLR,
    #[doc = "0x120 - "]
    pub in_err_eof_des_addr: IN_ERR_EOF_DES_ADDR,
    #[doc = "0x124 - "]
    pub in_suc_eof_des_addr: IN_SUC_EOF_DES_ADDR,
    #[doc = "0x128 - "]
    pub inlink_dscr: INLINK_DSCR,
    #[doc = "0x12c - "]
    pub inlink_dscr_bf0: INLINK_DSCR_BF0,
    #[doc = "0x130 - "]
    pub inlink_dscr_bf1: INLINK_DSCR_BF1,
    #[doc = "0x134 - "]
    pub out_eof_bfr_des_addr: OUT_EOF_BFR_DES_ADDR,
    #[doc = "0x138 - "]
    pub out_eof_des_addr: OUT_EOF_DES_ADDR,
    #[doc = "0x13c - "]
    pub outlink_dscr: OUTLINK_DSCR,
    #[doc = "0x140 - "]
    pub outlink_dscr_bf0: OUTLINK_DSCR_BF0,
    #[doc = "0x144 - "]
    pub outlink_dscr_bf1: OUTLINK_DSCR_BF1,
    #[doc = "0x148 - "]
    pub dma_rstatus: DMA_RSTATUS,
    #[doc = "0x14c - "]
    pub dma_tstatus: DMA_TSTATUS,
    _reserved67: [u8; 0x02ac],
    #[doc = "0x3fc - "]
    pub date: DATE,
}
#[doc = "CMD (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd`] module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = ""]
pub mod cmd;
#[doc = "ADDR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`addr`] module"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = ""]
pub mod addr;
#[doc = "CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = ""]
pub mod ctrl;
#[doc = "CTRL1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl1`] module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = ""]
pub mod ctrl1;
#[doc = "RD_STATUS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_status`] module"]
pub type RD_STATUS = crate::Reg<rd_status::RD_STATUS_SPEC>;
#[doc = ""]
pub mod rd_status;
#[doc = "CTRL2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl2`] module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = ""]
pub mod ctrl2;
#[doc = "CLOCK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clock`] module"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = ""]
pub mod clock;
#[doc = "USER (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`user::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`user`] module"]
pub type USER = crate::Reg<user::USER_SPEC>;
#[doc = ""]
pub mod user;
#[doc = "USER1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`user1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`user1`] module"]
pub type USER1 = crate::Reg<user1::USER1_SPEC>;
#[doc = ""]
pub mod user1;
#[doc = "USER2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`user2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`user2`] module"]
pub type USER2 = crate::Reg<user2::USER2_SPEC>;
#[doc = ""]
pub mod user2;
#[doc = "MOSI_DLEN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mosi_dlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mosi_dlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mosi_dlen`] module"]
pub type MOSI_DLEN = crate::Reg<mosi_dlen::MOSI_DLEN_SPEC>;
#[doc = ""]
pub mod mosi_dlen;
#[doc = "MISO_DLEN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`miso_dlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`miso_dlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`miso_dlen`] module"]
pub type MISO_DLEN = crate::Reg<miso_dlen::MISO_DLEN_SPEC>;
#[doc = ""]
pub mod miso_dlen;
#[doc = "SLV_WR_STATUS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slv_wr_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slv_wr_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slv_wr_status`] module"]
pub type SLV_WR_STATUS = crate::Reg<slv_wr_status::SLV_WR_STATUS_SPEC>;
#[doc = ""]
pub mod slv_wr_status;
#[doc = "PIN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pin`] module"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = ""]
pub mod pin;
#[doc = "SLAVE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slave::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slave`] module"]
pub type SLAVE = crate::Reg<slave::SLAVE_SPEC>;
#[doc = ""]
pub mod slave;
#[doc = "SLAVE1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slave1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slave1`] module"]
pub type SLAVE1 = crate::Reg<slave1::SLAVE1_SPEC>;
#[doc = ""]
pub mod slave1;
#[doc = "SLAVE2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slave2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slave2`] module"]
pub type SLAVE2 = crate::Reg<slave2::SLAVE2_SPEC>;
#[doc = ""]
pub mod slave2;
#[doc = "SLAVE3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slave3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slave3`] module"]
pub type SLAVE3 = crate::Reg<slave3::SLAVE3_SPEC>;
#[doc = ""]
pub mod slave3;
#[doc = "SLV_WRBUF_DLEN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slv_wrbuf_dlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slv_wrbuf_dlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slv_wrbuf_dlen`] module"]
pub type SLV_WRBUF_DLEN = crate::Reg<slv_wrbuf_dlen::SLV_WRBUF_DLEN_SPEC>;
#[doc = ""]
pub mod slv_wrbuf_dlen;
#[doc = "SLV_RDBUF_DLEN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slv_rdbuf_dlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slv_rdbuf_dlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slv_rdbuf_dlen`] module"]
pub type SLV_RDBUF_DLEN = crate::Reg<slv_rdbuf_dlen::SLV_RDBUF_DLEN_SPEC>;
#[doc = ""]
pub mod slv_rdbuf_dlen;
#[doc = "CACHE_FCTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_fctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_fctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_fctrl`] module"]
pub type CACHE_FCTRL = crate::Reg<cache_fctrl::CACHE_FCTRL_SPEC>;
#[doc = ""]
pub mod cache_fctrl;
#[doc = "CACHE_SCTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_sctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_sctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_sctrl`] module"]
pub type CACHE_SCTRL = crate::Reg<cache_sctrl::CACHE_SCTRL_SPEC>;
#[doc = ""]
pub mod cache_sctrl;
#[doc = "SRAM_CMD (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sram_cmd`] module"]
pub type SRAM_CMD = crate::Reg<sram_cmd::SRAM_CMD_SPEC>;
#[doc = ""]
pub mod sram_cmd;
#[doc = "SRAM_DRD_CMD (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_drd_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_drd_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sram_drd_cmd`] module"]
pub type SRAM_DRD_CMD = crate::Reg<sram_drd_cmd::SRAM_DRD_CMD_SPEC>;
#[doc = ""]
pub mod sram_drd_cmd;
#[doc = "SRAM_DWR_CMD (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_dwr_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_dwr_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sram_dwr_cmd`] module"]
pub type SRAM_DWR_CMD = crate::Reg<sram_dwr_cmd::SRAM_DWR_CMD_SPEC>;
#[doc = ""]
pub mod sram_dwr_cmd;
#[doc = "SLV_RD_BIT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slv_rd_bit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slv_rd_bit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slv_rd_bit`] module"]
pub type SLV_RD_BIT = crate::Reg<slv_rd_bit::SLV_RD_BIT_SPEC>;
#[doc = ""]
pub mod slv_rd_bit;
#[doc = "W0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`w0`] module"]
pub type W0 = crate::Reg<w0::W0_SPEC>;
#[doc = ""]
pub mod w0;
#[doc = "W1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`w1`] module"]
pub type W1 = crate::Reg<w1::W1_SPEC>;
#[doc = ""]
pub mod w1;
#[doc = "W2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`w2`] module"]
pub type W2 = crate::Reg<w2::W2_SPEC>;
#[doc = ""]
pub mod w2;
#[doc = "W3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`w3`] module"]
pub type W3 = crate::Reg<w3::W3_SPEC>;
#[doc = ""]
pub mod w3;
#[doc = "W4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`w4`] module"]
pub type W4 = crate::Reg<w4::W4_SPEC>;
#[doc = ""]
pub mod w4;
#[doc = "W5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`w5`] module"]
pub type W5 = crate::Reg<w5::W5_SPEC>;
#[doc = ""]
pub mod w5;
#[doc = "W6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`w6`] module"]
pub type W6 = crate::Reg<w6::W6_SPEC>;
#[doc = ""]
pub mod w6;
#[doc = "W7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`w7`] module"]
pub type W7 = crate::Reg<w7::W7_SPEC>;
#[doc = ""]
pub mod w7;
#[doc = "W8 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`w8`] module"]
pub type W8 = crate::Reg<w8::W8_SPEC>;
#[doc = ""]
pub mod w8;
#[doc = "W9 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`w9`] module"]
pub type W9 = crate::Reg<w9::W9_SPEC>;
#[doc = ""]
pub mod w9;
#[doc = "W10 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`w10`] module"]
pub type W10 = crate::Reg<w10::W10_SPEC>;
#[doc = ""]
pub mod w10;
#[doc = "W11 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`w11`] module"]
pub type W11 = crate::Reg<w11::W11_SPEC>;
#[doc = ""]
pub mod w11;
#[doc = "W12 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`w12`] module"]
pub type W12 = crate::Reg<w12::W12_SPEC>;
#[doc = ""]
pub mod w12;
#[doc = "W13 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`w13`] module"]
pub type W13 = crate::Reg<w13::W13_SPEC>;
#[doc = ""]
pub mod w13;
#[doc = "W14 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`w14`] module"]
pub type W14 = crate::Reg<w14::W14_SPEC>;
#[doc = ""]
pub mod w14;
#[doc = "W15 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`w15`] module"]
pub type W15 = crate::Reg<w15::W15_SPEC>;
#[doc = ""]
pub mod w15;
#[doc = "TX_CRC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_crc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_crc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tx_crc`] module"]
pub type TX_CRC = crate::Reg<tx_crc::TX_CRC_SPEC>;
#[doc = ""]
pub mod tx_crc;
#[doc = "EXT0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ext0`] module"]
pub type EXT0 = crate::Reg<ext0::EXT0_SPEC>;
#[doc = ""]
pub mod ext0;
#[doc = "EXT1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ext1`] module"]
pub type EXT1 = crate::Reg<ext1::EXT1_SPEC>;
#[doc = ""]
pub mod ext1;
#[doc = "EXT2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ext2`] module"]
pub type EXT2 = crate::Reg<ext2::EXT2_SPEC>;
#[doc = ""]
pub mod ext2;
#[doc = "EXT3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ext3`] module"]
pub type EXT3 = crate::Reg<ext3::EXT3_SPEC>;
#[doc = ""]
pub mod ext3;
#[doc = "DMA_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_conf`] module"]
pub type DMA_CONF = crate::Reg<dma_conf::DMA_CONF_SPEC>;
#[doc = ""]
pub mod dma_conf;
#[doc = "DMA_OUT_LINK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_out_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_out_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_out_link`] module"]
pub type DMA_OUT_LINK = crate::Reg<dma_out_link::DMA_OUT_LINK_SPEC>;
#[doc = ""]
pub mod dma_out_link;
#[doc = "DMA_IN_LINK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_in_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_in_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_in_link`] module"]
pub type DMA_IN_LINK = crate::Reg<dma_in_link::DMA_IN_LINK_SPEC>;
#[doc = ""]
pub mod dma_in_link;
#[doc = "DMA_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_status`] module"]
pub type DMA_STATUS = crate::Reg<dma_status::DMA_STATUS_SPEC>;
#[doc = ""]
pub mod dma_status;
#[doc = "DMA_INT_ENA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_int_ena`] module"]
pub type DMA_INT_ENA = crate::Reg<dma_int_ena::DMA_INT_ENA_SPEC>;
#[doc = ""]
pub mod dma_int_ena;
#[doc = "DMA_INT_RAW (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_int_raw`] module"]
pub type DMA_INT_RAW = crate::Reg<dma_int_raw::DMA_INT_RAW_SPEC>;
#[doc = ""]
pub mod dma_int_raw;
#[doc = "DMA_INT_ST (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_int_st`] module"]
pub type DMA_INT_ST = crate::Reg<dma_int_st::DMA_INT_ST_SPEC>;
#[doc = ""]
pub mod dma_int_st;
#[doc = "DMA_INT_CLR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_int_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_int_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_int_clr`] module"]
pub type DMA_INT_CLR = crate::Reg<dma_int_clr::DMA_INT_CLR_SPEC>;
#[doc = ""]
pub mod dma_int_clr;
#[doc = "IN_ERR_EOF_DES_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_err_eof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in_err_eof_des_addr`] module"]
pub type IN_ERR_EOF_DES_ADDR = crate::Reg<in_err_eof_des_addr::IN_ERR_EOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod in_err_eof_des_addr;
#[doc = "IN_SUC_EOF_DES_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_suc_eof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in_suc_eof_des_addr`] module"]
pub type IN_SUC_EOF_DES_ADDR = crate::Reg<in_suc_eof_des_addr::IN_SUC_EOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod in_suc_eof_des_addr;
#[doc = "INLINK_DSCR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inlink_dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inlink_dscr`] module"]
pub type INLINK_DSCR = crate::Reg<inlink_dscr::INLINK_DSCR_SPEC>;
#[doc = ""]
pub mod inlink_dscr;
#[doc = "INLINK_DSCR_BF0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inlink_dscr_bf0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inlink_dscr_bf0`] module"]
pub type INLINK_DSCR_BF0 = crate::Reg<inlink_dscr_bf0::INLINK_DSCR_BF0_SPEC>;
#[doc = ""]
pub mod inlink_dscr_bf0;
#[doc = "INLINK_DSCR_BF1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inlink_dscr_bf1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inlink_dscr_bf1`] module"]
pub type INLINK_DSCR_BF1 = crate::Reg<inlink_dscr_bf1::INLINK_DSCR_BF1_SPEC>;
#[doc = ""]
pub mod inlink_dscr_bf1;
#[doc = "OUT_EOF_BFR_DES_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_eof_bfr_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_eof_bfr_des_addr`] module"]
pub type OUT_EOF_BFR_DES_ADDR = crate::Reg<out_eof_bfr_des_addr::OUT_EOF_BFR_DES_ADDR_SPEC>;
#[doc = ""]
pub mod out_eof_bfr_des_addr;
#[doc = "OUT_EOF_DES_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_eof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_eof_des_addr`] module"]
pub type OUT_EOF_DES_ADDR = crate::Reg<out_eof_des_addr::OUT_EOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod out_eof_des_addr;
#[doc = "OUTLINK_DSCR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outlink_dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`outlink_dscr`] module"]
pub type OUTLINK_DSCR = crate::Reg<outlink_dscr::OUTLINK_DSCR_SPEC>;
#[doc = ""]
pub mod outlink_dscr;
#[doc = "OUTLINK_DSCR_BF0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outlink_dscr_bf0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`outlink_dscr_bf0`] module"]
pub type OUTLINK_DSCR_BF0 = crate::Reg<outlink_dscr_bf0::OUTLINK_DSCR_BF0_SPEC>;
#[doc = ""]
pub mod outlink_dscr_bf0;
#[doc = "OUTLINK_DSCR_BF1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outlink_dscr_bf1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`outlink_dscr_bf1`] module"]
pub type OUTLINK_DSCR_BF1 = crate::Reg<outlink_dscr_bf1::OUTLINK_DSCR_BF1_SPEC>;
#[doc = ""]
pub mod outlink_dscr_bf1;
#[doc = "DMA_RSTATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_rstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_rstatus`] module"]
pub type DMA_RSTATUS = crate::Reg<dma_rstatus::DMA_RSTATUS_SPEC>;
#[doc = ""]
pub mod dma_rstatus;
#[doc = "DMA_TSTATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_tstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_tstatus`] module"]
pub type DMA_TSTATUS = crate::Reg<dma_tstatus::DMA_TSTATUS_SPEC>;
#[doc = ""]
pub mod dma_tstatus;
#[doc = "DATE (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
