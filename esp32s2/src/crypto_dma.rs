#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    conf: CONF,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    out_status: OUT_STATUS,
    _reserved6: [u8; 0x04],
    in_status: IN_STATUS,
    _reserved7: [u8; 0x04],
    out_link: OUT_LINK,
    in_link: IN_LINK,
    conf1: CONF1,
    state0: STATE0,
    state1: STATE1,
    out_eof_des_addr: OUT_EOF_DES_ADDR,
    in_suc_eof_des_addr: IN_SUC_EOF_DES_ADDR,
    in_err_eof_des_addr: IN_ERR_EOF_DES_ADDR,
    out_eof_bfr_des_addr: OUT_EOF_BFR_DES_ADDR,
    ahb_test: AHB_TEST,
    dma_in_dscr: DMA_IN_DSCR,
    dma_in_dscr_bf0: DMA_IN_DSCR_BF0,
    _reserved19: [u8; 0x04],
    dma_out_dscr: DMA_OUT_DSCR,
    dma_out_dscr_bf0: DMA_OUT_DSCR_BF0,
    _reserved21: [u8; 0x04],
    aes_sha_select: AES_SHA_SELECT,
    pd_conf: PD_CONF,
    _reserved23: [u8; 0x90],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA configuration register"]
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    #[doc = "0x04 - Raw interrupt status"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x08 - Masked interrupt status"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x0c - Interrupt enable bits"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x10 - Interrupt clear bits"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x14 - TX FIFO status register"]
    #[inline(always)]
    pub const fn out_status(&self) -> &OUT_STATUS {
        &self.out_status
    }
    #[doc = "0x1c - RX FIFO status register"]
    #[inline(always)]
    pub const fn in_status(&self) -> &IN_STATUS {
        &self.in_status
    }
    #[doc = "0x24 - Link descriptor address and control"]
    #[inline(always)]
    pub const fn out_link(&self) -> &OUT_LINK {
        &self.out_link
    }
    #[doc = "0x28 - Link descriptor address and control"]
    #[inline(always)]
    pub const fn in_link(&self) -> &IN_LINK {
        &self.in_link
    }
    #[doc = "0x2c - DMA configuration register"]
    #[inline(always)]
    pub const fn conf1(&self) -> &CONF1 {
        &self.conf1
    }
    #[doc = "0x30 - Status register of receiving data"]
    #[inline(always)]
    pub const fn state0(&self) -> &STATE0 {
        &self.state0
    }
    #[doc = "0x34 - Status register of transmitting data"]
    #[inline(always)]
    pub const fn state1(&self) -> &STATE1 {
        &self.state1
    }
    #[doc = "0x38 - Transmit descriptor address when EOF occurs"]
    #[inline(always)]
    pub const fn out_eof_des_addr(&self) -> &OUT_EOF_DES_ADDR {
        &self.out_eof_des_addr
    }
    #[doc = "0x3c - Receive descriptor address when EOF occurs"]
    #[inline(always)]
    pub const fn in_suc_eof_des_addr(&self) -> &IN_SUC_EOF_DES_ADDR {
        &self.in_suc_eof_des_addr
    }
    #[doc = "0x40 - Receive descriptor address when errors occur"]
    #[inline(always)]
    pub const fn in_err_eof_des_addr(&self) -> &IN_ERR_EOF_DES_ADDR {
        &self.in_err_eof_des_addr
    }
    #[doc = "0x44 - Transmit descriptor address before the last transmit descriptor"]
    #[inline(always)]
    pub const fn out_eof_bfr_des_addr(&self) -> &OUT_EOF_BFR_DES_ADDR {
        &self.out_eof_bfr_des_addr
    }
    #[doc = "0x48 - AHB test register"]
    #[inline(always)]
    pub const fn ahb_test(&self) -> &AHB_TEST {
        &self.ahb_test
    }
    #[doc = "0x4c - Address of current receive descriptor"]
    #[inline(always)]
    pub const fn dma_in_dscr(&self) -> &DMA_IN_DSCR {
        &self.dma_in_dscr
    }
    #[doc = "0x50 - Address of last receive descriptor"]
    #[inline(always)]
    pub const fn dma_in_dscr_bf0(&self) -> &DMA_IN_DSCR_BF0 {
        &self.dma_in_dscr_bf0
    }
    #[doc = "0x58 - Address of current transmit descriptor"]
    #[inline(always)]
    pub const fn dma_out_dscr(&self) -> &DMA_OUT_DSCR {
        &self.dma_out_dscr
    }
    #[doc = "0x5c - Address of last transmit descriptor"]
    #[inline(always)]
    pub const fn dma_out_dscr_bf0(&self) -> &DMA_OUT_DSCR_BF0 {
        &self.dma_out_dscr_bf0
    }
    #[doc = "0x64 - AES/SHA select register"]
    #[inline(always)]
    pub const fn aes_sha_select(&self) -> &AES_SHA_SELECT {
        &self.aes_sha_select
    }
    #[doc = "0x68 - Power control register"]
    #[inline(always)]
    pub const fn pd_conf(&self) -> &PD_CONF {
        &self.pd_conf
    }
    #[doc = "0xfc - Crypto DMA version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CONF (rw) register accessor: DMA configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`] module"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "DMA configuration register"]
pub mod conf;
#[doc = "OUT_LINK (rw) register accessor: Link descriptor address and control\n\nYou can [`read`](crate::Reg::read) this register and get [`out_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link`] module"]
pub type OUT_LINK = crate::Reg<out_link::OUT_LINK_SPEC>;
#[doc = "Link descriptor address and control"]
pub mod out_link;
#[doc = "IN_LINK (rw) register accessor: Link descriptor address and control\n\nYou can [`read`](crate::Reg::read) this register and get [`in_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link`] module"]
pub type IN_LINK = crate::Reg<in_link::IN_LINK_SPEC>;
#[doc = "Link descriptor address and control"]
pub mod in_link;
#[doc = "CONF1 (rw) register accessor: DMA configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf1`] module"]
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
#[doc = "DMA configuration register"]
pub mod conf1;
#[doc = "AHB_TEST (rw) register accessor: AHB test register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_test`] module"]
pub type AHB_TEST = crate::Reg<ahb_test::AHB_TEST_SPEC>;
#[doc = "AHB test register"]
pub mod ahb_test;
#[doc = "AES_SHA_SELECT (rw) register accessor: AES/SHA select register\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_sha_select::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_sha_select::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_sha_select`] module"]
pub type AES_SHA_SELECT = crate::Reg<aes_sha_select::AES_SHA_SELECT_SPEC>;
#[doc = "AES/SHA select register"]
pub mod aes_sha_select;
#[doc = "PD_CONF (rw) register accessor: Power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_conf`] module"]
pub type PD_CONF = crate::Reg<pd_conf::PD_CONF_SPEC>;
#[doc = "Power control register"]
pub mod pd_conf;
#[doc = "DATE (rw) register accessor: Crypto DMA version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Crypto DMA version control register"]
pub mod date;
#[doc = "INT_RAW (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Raw interrupt status"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Masked interrupt status"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: Interrupt enable bits\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt enable bits"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: Interrupt clear bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt clear bits"]
pub mod int_clr;
#[doc = "OUT_STATUS (r) register accessor: TX FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`out_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_status`] module"]
pub type OUT_STATUS = crate::Reg<out_status::OUT_STATUS_SPEC>;
#[doc = "TX FIFO status register"]
pub mod out_status;
#[doc = "IN_STATUS (r) register accessor: RX FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`in_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_status`] module"]
pub type IN_STATUS = crate::Reg<in_status::IN_STATUS_SPEC>;
#[doc = "RX FIFO status register"]
pub mod in_status;
#[doc = "STATE0 (r) register accessor: Status register of receiving data\n\nYou can [`read`](crate::Reg::read) this register and get [`state0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state0`] module"]
pub type STATE0 = crate::Reg<state0::STATE0_SPEC>;
#[doc = "Status register of receiving data"]
pub mod state0;
#[doc = "STATE1 (r) register accessor: Status register of transmitting data\n\nYou can [`read`](crate::Reg::read) this register and get [`state1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state1`] module"]
pub type STATE1 = crate::Reg<state1::STATE1_SPEC>;
#[doc = "Status register of transmitting data"]
pub mod state1;
#[doc = "OUT_EOF_DES_ADDR (r) register accessor: Transmit descriptor address when EOF occurs\n\nYou can [`read`](crate::Reg::read) this register and get [`out_eof_des_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_des_addr`] module"]
pub type OUT_EOF_DES_ADDR = crate::Reg<out_eof_des_addr::OUT_EOF_DES_ADDR_SPEC>;
#[doc = "Transmit descriptor address when EOF occurs"]
pub mod out_eof_des_addr;
#[doc = "IN_SUC_EOF_DES_ADDR (r) register accessor: Receive descriptor address when EOF occurs\n\nYou can [`read`](crate::Reg::read) this register and get [`in_suc_eof_des_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_suc_eof_des_addr`] module"]
pub type IN_SUC_EOF_DES_ADDR = crate::Reg<in_suc_eof_des_addr::IN_SUC_EOF_DES_ADDR_SPEC>;
#[doc = "Receive descriptor address when EOF occurs"]
pub mod in_suc_eof_des_addr;
#[doc = "IN_ERR_EOF_DES_ADDR (r) register accessor: Receive descriptor address when errors occur\n\nYou can [`read`](crate::Reg::read) this register and get [`in_err_eof_des_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_err_eof_des_addr`] module"]
pub type IN_ERR_EOF_DES_ADDR = crate::Reg<in_err_eof_des_addr::IN_ERR_EOF_DES_ADDR_SPEC>;
#[doc = "Receive descriptor address when errors occur"]
pub mod in_err_eof_des_addr;
#[doc = "OUT_EOF_BFR_DES_ADDR (r) register accessor: Transmit descriptor address before the last transmit descriptor\n\nYou can [`read`](crate::Reg::read) this register and get [`out_eof_bfr_des_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_bfr_des_addr`] module"]
pub type OUT_EOF_BFR_DES_ADDR = crate::Reg<out_eof_bfr_des_addr::OUT_EOF_BFR_DES_ADDR_SPEC>;
#[doc = "Transmit descriptor address before the last transmit descriptor"]
pub mod out_eof_bfr_des_addr;
#[doc = "DMA_IN_DSCR (r) register accessor: Address of current receive descriptor\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_in_dscr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_in_dscr`] module"]
pub type DMA_IN_DSCR = crate::Reg<dma_in_dscr::DMA_IN_DSCR_SPEC>;
#[doc = "Address of current receive descriptor"]
pub mod dma_in_dscr;
#[doc = "DMA_IN_DSCR_BF0 (r) register accessor: Address of last receive descriptor\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_in_dscr_bf0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_in_dscr_bf0`] module"]
pub type DMA_IN_DSCR_BF0 = crate::Reg<dma_in_dscr_bf0::DMA_IN_DSCR_BF0_SPEC>;
#[doc = "Address of last receive descriptor"]
pub mod dma_in_dscr_bf0;
#[doc = "DMA_OUT_DSCR (r) register accessor: Address of current transmit descriptor\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_out_dscr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_out_dscr`] module"]
pub type DMA_OUT_DSCR = crate::Reg<dma_out_dscr::DMA_OUT_DSCR_SPEC>;
#[doc = "Address of current transmit descriptor"]
pub mod dma_out_dscr;
#[doc = "DMA_OUT_DSCR_BF0 (r) register accessor: Address of last transmit descriptor\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_out_dscr_bf0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_out_dscr_bf0`] module"]
pub type DMA_OUT_DSCR_BF0 = crate::Reg<dma_out_dscr_bf0::DMA_OUT_DSCR_BF0_SPEC>;
#[doc = "Address of last transmit descriptor"]
pub mod dma_out_dscr_bf0;
