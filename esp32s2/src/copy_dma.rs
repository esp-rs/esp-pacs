#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    out_link: OUT_LINK,
    in_link: IN_LINK,
    out_eof_des_addr: OUT_EOF_DES_ADDR,
    in_eof_des_addr: IN_EOF_DES_ADDR,
    out_eof_bfr_des_addr: OUT_EOF_BFR_DES_ADDR,
    inlink_dscr: INLINK_DSCR,
    inlink_dscr_bf0: INLINK_DSCR_BF0,
    _reserved11: [u8; 0x04],
    outlink_dscr: OUTLINK_DSCR,
    outlink_dscr_bf0: OUTLINK_DSCR_BF0,
    _reserved13: [u8; 0x04],
    conf: CONF,
    in_st: IN_ST,
    out_st: OUT_ST,
    _reserved16: [u8; 0xb4],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - Raw interrupt status"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x04 - Masked interrupt status"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x08 - Interrupt enable bits"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x0c - Interrupt clear bits"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x10 - Link descriptor address and control"]
    #[inline(always)]
    pub const fn out_link(&self) -> &OUT_LINK {
        &self.out_link
    }
    #[doc = "0x14 - Link descriptor address and control"]
    #[inline(always)]
    pub const fn in_link(&self) -> &IN_LINK {
        &self.in_link
    }
    #[doc = "0x18 - Transmit descriptor address when EOF occurs"]
    #[inline(always)]
    pub const fn out_eof_des_addr(&self) -> &OUT_EOF_DES_ADDR {
        &self.out_eof_des_addr
    }
    #[doc = "0x1c - Receive descriptor address when EOF occurs"]
    #[inline(always)]
    pub const fn in_eof_des_addr(&self) -> &IN_EOF_DES_ADDR {
        &self.in_eof_des_addr
    }
    #[doc = "0x20 - Transmit descriptor address before the last transmit descriptor"]
    #[inline(always)]
    pub const fn out_eof_bfr_des_addr(&self) -> &OUT_EOF_BFR_DES_ADDR {
        &self.out_eof_bfr_des_addr
    }
    #[doc = "0x24 - Address of current receive descriptor"]
    #[inline(always)]
    pub const fn inlink_dscr(&self) -> &INLINK_DSCR {
        &self.inlink_dscr
    }
    #[doc = "0x28 - Address of last receive descriptor"]
    #[inline(always)]
    pub const fn inlink_dscr_bf0(&self) -> &INLINK_DSCR_BF0 {
        &self.inlink_dscr_bf0
    }
    #[doc = "0x30 - Address of current transmit descriptor"]
    #[inline(always)]
    pub const fn outlink_dscr(&self) -> &OUTLINK_DSCR {
        &self.outlink_dscr
    }
    #[doc = "0x34 - Address of last transmit descriptor"]
    #[inline(always)]
    pub const fn outlink_dscr_bf0(&self) -> &OUTLINK_DSCR_BF0 {
        &self.outlink_dscr_bf0
    }
    #[doc = "0x3c - Copy DMA configuration register"]
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    #[doc = "0x40 - Status register of receiving data"]
    #[inline(always)]
    pub const fn in_st(&self) -> &IN_ST {
        &self.in_st
    }
    #[doc = "0x44 - Status register of transmitting data"]
    #[inline(always)]
    pub const fn out_st(&self) -> &OUT_ST {
        &self.out_st
    }
    #[doc = "0xfc - Copy DMA version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
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
#[doc = "OUT_LINK (rw) register accessor: Link descriptor address and control\n\nYou can [`read`](crate::Reg::read) this register and get [`out_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link`] module"]
pub type OUT_LINK = crate::Reg<out_link::OUT_LINK_SPEC>;
#[doc = "Link descriptor address and control"]
pub mod out_link;
#[doc = "IN_LINK (rw) register accessor: Link descriptor address and control\n\nYou can [`read`](crate::Reg::read) this register and get [`in_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link`] module"]
pub type IN_LINK = crate::Reg<in_link::IN_LINK_SPEC>;
#[doc = "Link descriptor address and control"]
pub mod in_link;
#[doc = "CONF (r) register accessor: Copy DMA configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`] module"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "Copy DMA configuration register"]
pub mod conf;
#[doc = "OUT_EOF_DES_ADDR (r) register accessor: Transmit descriptor address when EOF occurs\n\nYou can [`read`](crate::Reg::read) this register and get [`out_eof_des_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_des_addr`] module"]
pub type OUT_EOF_DES_ADDR = crate::Reg<out_eof_des_addr::OUT_EOF_DES_ADDR_SPEC>;
#[doc = "Transmit descriptor address when EOF occurs"]
pub mod out_eof_des_addr;
#[doc = "IN_EOF_DES_ADDR (r) register accessor: Receive descriptor address when EOF occurs\n\nYou can [`read`](crate::Reg::read) this register and get [`in_eof_des_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_eof_des_addr`] module"]
pub type IN_EOF_DES_ADDR = crate::Reg<in_eof_des_addr::IN_EOF_DES_ADDR_SPEC>;
#[doc = "Receive descriptor address when EOF occurs"]
pub mod in_eof_des_addr;
#[doc = "OUT_EOF_BFR_DES_ADDR (r) register accessor: Transmit descriptor address before the last transmit descriptor\n\nYou can [`read`](crate::Reg::read) this register and get [`out_eof_bfr_des_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_bfr_des_addr`] module"]
pub type OUT_EOF_BFR_DES_ADDR = crate::Reg<out_eof_bfr_des_addr::OUT_EOF_BFR_DES_ADDR_SPEC>;
#[doc = "Transmit descriptor address before the last transmit descriptor"]
pub mod out_eof_bfr_des_addr;
#[doc = "INLINK_DSCR (r) register accessor: Address of current receive descriptor\n\nYou can [`read`](crate::Reg::read) this register and get [`inlink_dscr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inlink_dscr`] module"]
pub type INLINK_DSCR = crate::Reg<inlink_dscr::INLINK_DSCR_SPEC>;
#[doc = "Address of current receive descriptor"]
pub mod inlink_dscr;
#[doc = "INLINK_DSCR_BF0 (r) register accessor: Address of last receive descriptor\n\nYou can [`read`](crate::Reg::read) this register and get [`inlink_dscr_bf0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inlink_dscr_bf0`] module"]
pub type INLINK_DSCR_BF0 = crate::Reg<inlink_dscr_bf0::INLINK_DSCR_BF0_SPEC>;
#[doc = "Address of last receive descriptor"]
pub mod inlink_dscr_bf0;
#[doc = "OUTLINK_DSCR (r) register accessor: Address of current transmit descriptor\n\nYou can [`read`](crate::Reg::read) this register and get [`outlink_dscr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outlink_dscr`] module"]
pub type OUTLINK_DSCR = crate::Reg<outlink_dscr::OUTLINK_DSCR_SPEC>;
#[doc = "Address of current transmit descriptor"]
pub mod outlink_dscr;
#[doc = "OUTLINK_DSCR_BF0 (r) register accessor: Address of last transmit descriptor\n\nYou can [`read`](crate::Reg::read) this register and get [`outlink_dscr_bf0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outlink_dscr_bf0`] module"]
pub type OUTLINK_DSCR_BF0 = crate::Reg<outlink_dscr_bf0::OUTLINK_DSCR_BF0_SPEC>;
#[doc = "Address of last transmit descriptor"]
pub mod outlink_dscr_bf0;
#[doc = "IN_ST (r) register accessor: Status register of receiving data\n\nYou can [`read`](crate::Reg::read) this register and get [`in_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_st`] module"]
pub type IN_ST = crate::Reg<in_st::IN_ST_SPEC>;
#[doc = "Status register of receiving data"]
pub mod in_st;
#[doc = "OUT_ST (r) register accessor: Status register of transmitting data\n\nYou can [`read`](crate::Reg::read) this register and get [`out_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_st`] module"]
pub type OUT_ST = crate::Reg<out_st::OUT_ST_SPEC>;
#[doc = "Status register of transmitting data"]
pub mod out_st;
#[doc = "DATE (rw) register accessor: Copy DMA version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Copy DMA version control register"]
pub mod date;
