#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster IN_CH%s, containing IN_CONF0_CH\\[0-4\\], IN_INT_RAW_CH\\[0-4\\], IN_INT_ENA_CH\\[0-4\\], IN_INT_ST_CH\\[0-4\\], IN_INT_CLR_CH\\[0-4\\], INFIFO_STATUS_CH\\[0-4\\], IN_POP_CH\\[0-4\\], IN_LINK_CONF_CH\\[0-4\\], IN_LINK_ADDR_CH\\[0-4\\], IN_STATE_CH\\[0-4\\], IN_SUC_EOF_DES_ADDR_CH\\[0-4\\], IN_ERR_EOF_DES_ADDR_CH\\[0-4\\], IN_DSCR_CH\\[0-4\\], IN_DSCR_BF0_CH\\[0-4\\], IN_DSCR_BF1_CH\\[0-4\\], IN_ARB_CH\\[0-4\\], IN_RO_PD_CONF_CH\\[0-4\\], IN_ETM_CONF_CH\\[0-4\\], IN_FIFO_CNT_CH\\[0-4\\], IN_POP_DATA_CNT_CH\\[0-4\\], IN_XADDR_CH\\[0-4\\], IN_BUF_HB_RCV_CH\\[0-4\\]"]
pub struct IN_CH {
    conf0: CONF0,
    int_raw: INT_RAW,
    int_ena: INT_ENA,
    int_st: INT_ST,
    int_clr: INT_CLR,
    fifo_status: FIFO_STATUS,
    pop: POP,
    link_conf: LINK_CONF,
    link_addr: LINK_ADDR,
    state: STATE,
    suc_eof_des_addr: SUC_EOF_DES_ADDR,
    err_eof_des_addr: ERR_EOF_DES_ADDR,
    dscr: DSCR,
    dscr_bf0: DSCR_BF0,
    dscr_bf1: DSCR_BF1,
    _reserved15: [u8; 0x04],
    arb: ARB,
    _reserved16: [u8; 0x04],
    ro_pd_conf: RO_PD_CONF,
    _reserved17: [u8; 0x20],
    etm_conf: ETM_CONF,
    _reserved18: [u8; 0x10],
    fifo_cnt: FIFO_CNT,
    pop_data_cnt: POP_DATA_CNT,
    xaddr: XADDR,
    buf_hb_rcv: BUF_HB_RCV,
    _reserved_end: [u8; 0x70],
}
impl IN_CH {
    #[doc = "0x00 - RX CHx config0 register"]
    #[inline(always)]
    pub const fn conf0(&self) -> &CONF0 {
        &self.conf0
    }
    #[doc = "0x04 - RX CHx interrupt raw register"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x08 - RX CHx interrupt ena register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x0c - RX CHx interrupt st register"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x10 - RX CHx interrupt clr register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x14 - RX CHx INFIFO status register"]
    #[inline(always)]
    pub const fn fifo_status(&self) -> &FIFO_STATUS {
        &self.fifo_status
    }
    #[doc = "0x18 - RX CHx INFIFO pop register"]
    #[inline(always)]
    pub const fn pop(&self) -> &POP {
        &self.pop
    }
    #[doc = "0x1c - RX CHx in_link dscr ctrl register"]
    #[inline(always)]
    pub const fn link_conf(&self) -> &LINK_CONF {
        &self.link_conf
    }
    #[doc = "0x20 - RX CHx in_link dscr addr register"]
    #[inline(always)]
    pub const fn link_addr(&self) -> &LINK_ADDR {
        &self.link_addr
    }
    #[doc = "0x24 - RX CHx state register"]
    #[inline(always)]
    pub const fn state(&self) -> &STATE {
        &self.state
    }
    #[doc = "0x28 - RX CHx eof des addr register"]
    #[inline(always)]
    pub const fn suc_eof_des_addr(&self) -> &SUC_EOF_DES_ADDR {
        &self.suc_eof_des_addr
    }
    #[doc = "0x2c - RX CHx err eof des addr register"]
    #[inline(always)]
    pub const fn err_eof_des_addr(&self) -> &ERR_EOF_DES_ADDR {
        &self.err_eof_des_addr
    }
    #[doc = "0x30 - RX CHx next dscr addr register"]
    #[inline(always)]
    pub const fn dscr(&self) -> &DSCR {
        &self.dscr
    }
    #[doc = "0x34 - RX CHx last dscr addr register"]
    #[inline(always)]
    pub const fn dscr_bf0(&self) -> &DSCR_BF0 {
        &self.dscr_bf0
    }
    #[doc = "0x38 - RX CHx second-to-last dscr addr register"]
    #[inline(always)]
    pub const fn dscr_bf1(&self) -> &DSCR_BF1 {
        &self.dscr_bf1
    }
    #[doc = "0x40 - RX CHx arb register"]
    #[inline(always)]
    pub const fn arb(&self) -> &ARB {
        &self.arb
    }
    #[doc = "0x48 - RX CHx reorder power config register. Available on CH0"]
    #[inline(always)]
    pub const fn ro_pd_conf(&self) -> &RO_PD_CONF {
        &self.ro_pd_conf
    }
    #[doc = "0x6c - RX CHx ETM config register"]
    #[inline(always)]
    pub const fn etm_conf(&self) -> &ETM_CONF {
        &self.etm_conf
    }
    #[doc = "0x80 - RX CHx fifo cnt register"]
    #[inline(always)]
    pub const fn fifo_cnt(&self) -> &FIFO_CNT {
        &self.fifo_cnt
    }
    #[doc = "0x84 - RX CHx pop data cnt register"]
    #[inline(always)]
    pub const fn pop_data_cnt(&self) -> &POP_DATA_CNT {
        &self.pop_data_cnt
    }
    #[doc = "0x88 - RX CHx xaddr register"]
    #[inline(always)]
    pub const fn xaddr(&self) -> &XADDR {
        &self.xaddr
    }
    #[doc = "0x8c - RX CH0 buf len hb rcv register"]
    #[inline(always)]
    pub const fn buf_hb_rcv(&self) -> &BUF_HB_RCV {
        &self.buf_hb_rcv
    }
}
#[doc = "CONF0 (rw) register accessor: RX CHx config0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf0`] module"]
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
#[doc = "RX CHx config0 register"]
pub mod conf0;
#[doc = "INT_RAW (rw) register accessor: RX CHx interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "RX CHx interrupt raw register"]
pub mod int_raw;
#[doc = "INT_ENA (rw) register accessor: RX CHx interrupt ena register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "RX CHx interrupt ena register"]
pub mod int_ena;
#[doc = "INT_ST (r) register accessor: RX CHx interrupt st register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "RX CHx interrupt st register"]
pub mod int_st;
#[doc = "INT_CLR (w) register accessor: RX CHx interrupt clr register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "RX CHx interrupt clr register"]
pub mod int_clr;
#[doc = "FIFO_STATUS (r) register accessor: RX CHx INFIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_status`] module"]
pub type FIFO_STATUS = crate::Reg<fifo_status::FIFO_STATUS_SPEC>;
#[doc = "RX CHx INFIFO status register"]
pub mod fifo_status;
#[doc = "POP (rw) register accessor: RX CHx INFIFO pop register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pop`] module"]
pub type POP = crate::Reg<pop::POP_SPEC>;
#[doc = "RX CHx INFIFO pop register"]
pub mod pop;
#[doc = "LINK_CONF (rw) register accessor: RX CHx in_link dscr ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`link_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`link_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link_conf`] module"]
pub type LINK_CONF = crate::Reg<link_conf::LINK_CONF_SPEC>;
#[doc = "RX CHx in_link dscr ctrl register"]
pub mod link_conf;
#[doc = "LINK_ADDR (rw) register accessor: RX CHx in_link dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`link_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`link_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link_addr`] module"]
pub type LINK_ADDR = crate::Reg<link_addr::LINK_ADDR_SPEC>;
#[doc = "RX CHx in_link dscr addr register"]
pub mod link_addr;
#[doc = "STATE (r) register accessor: RX CHx state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`] module"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "RX CHx state register"]
pub mod state;
#[doc = "SUC_EOF_DES_ADDR (r) register accessor: RX CHx eof des addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`suc_eof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@suc_eof_des_addr`] module"]
pub type SUC_EOF_DES_ADDR = crate::Reg<suc_eof_des_addr::SUC_EOF_DES_ADDR_SPEC>;
#[doc = "RX CHx eof des addr register"]
pub mod suc_eof_des_addr;
#[doc = "ERR_EOF_DES_ADDR (r) register accessor: RX CHx err eof des addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_eof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_eof_des_addr`] module"]
pub type ERR_EOF_DES_ADDR = crate::Reg<err_eof_des_addr::ERR_EOF_DES_ADDR_SPEC>;
#[doc = "RX CHx err eof des addr register"]
pub mod err_eof_des_addr;
#[doc = "DSCR (r) register accessor: RX CHx next dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dscr`] module"]
pub type DSCR = crate::Reg<dscr::DSCR_SPEC>;
#[doc = "RX CHx next dscr addr register"]
pub mod dscr;
#[doc = "DSCR_BF0 (r) register accessor: RX CHx last dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dscr_bf0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dscr_bf0`] module"]
pub type DSCR_BF0 = crate::Reg<dscr_bf0::DSCR_BF0_SPEC>;
#[doc = "RX CHx last dscr addr register"]
pub mod dscr_bf0;
#[doc = "DSCR_BF1 (r) register accessor: RX CHx second-to-last dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dscr_bf1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dscr_bf1`] module"]
pub type DSCR_BF1 = crate::Reg<dscr_bf1::DSCR_BF1_SPEC>;
#[doc = "RX CHx second-to-last dscr addr register"]
pub mod dscr_bf1;
#[doc = "ARB (rw) register accessor: RX CHx arb register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb`] module"]
pub type ARB = crate::Reg<arb::ARB_SPEC>;
#[doc = "RX CHx arb register"]
pub mod arb;
#[doc = "RO_PD_CONF (rw) register accessor: RX CHx reorder power config register. Available on CH0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ro_pd_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ro_pd_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ro_pd_conf`] module"]
pub type RO_PD_CONF = crate::Reg<ro_pd_conf::RO_PD_CONF_SPEC>;
#[doc = "RX CHx reorder power config register. Available on CH0"]
pub mod ro_pd_conf;
#[doc = "ETM_CONF (rw) register accessor: RX CHx ETM config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etm_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_conf`] module"]
pub type ETM_CONF = crate::Reg<etm_conf::ETM_CONF_SPEC>;
#[doc = "RX CHx ETM config register"]
pub mod etm_conf;
#[doc = "FIFO_CNT (r) register accessor: RX CHx fifo cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_cnt`] module"]
pub type FIFO_CNT = crate::Reg<fifo_cnt::FIFO_CNT_SPEC>;
#[doc = "RX CHx fifo cnt register"]
pub mod fifo_cnt;
#[doc = "POP_DATA_CNT (r) register accessor: RX CHx pop data cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pop_data_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pop_data_cnt`] module"]
pub type POP_DATA_CNT = crate::Reg<pop_data_cnt::POP_DATA_CNT_SPEC>;
#[doc = "RX CHx pop data cnt register"]
pub mod pop_data_cnt;
#[doc = "XADDR (r) register accessor: RX CHx xaddr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xaddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xaddr`] module"]
pub type XADDR = crate::Reg<xaddr::XADDR_SPEC>;
#[doc = "RX CHx xaddr register"]
pub mod xaddr;
#[doc = "BUF_HB_RCV (r) register accessor: RX CH0 buf len hb rcv register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_hb_rcv::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_hb_rcv`] module"]
pub type BUF_HB_RCV = crate::Reg<buf_hb_rcv::BUF_HB_RCV_SPEC>;
#[doc = "RX CH0 buf len hb rcv register"]
pub mod buf_hb_rcv;
