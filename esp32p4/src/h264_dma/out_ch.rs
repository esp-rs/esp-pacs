#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster OUT_CH%s, containing OUT_CONF0_CH?, OUT_INT_RAW_CH?, OUT_INT_ENA_CH?, OUT_INT_ST_CH?, OUT_INT_CLR_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CONF_CH?, OUT_LINK_ADDR_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_ARB_CH?, OUT_RO_STATUS_CH?, OUT_RO_PD_CONF_CH?, OUT_MODE_ENABLE_CH?, OUT_MODE_YUV_CH?, OUT_ETM_CONF_CH?, OUT_BUF_LEN_CH?, OUT_FIFO_BCNT_CH?, OUT_PUSH_BYTECNT_CH?, OUT_XADDR_CH?, OUT_BLOCK_BUF_LEN_CH?"]
pub struct OUT_CH {
    conf0: CONF0,
    int_raw: INT_RAW,
    int_ena: INT_ENA,
    int_st: INT_ST,
    int_clr: INT_CLR,
    fifo_status: FIFO_STATUS,
    push: PUSH,
    link_conf: LINK_CONF,
    link_addr: LINK_ADDR,
    state: STATE,
    eof_des_addr: EOF_DES_ADDR,
    dscr: DSCR,
    dscr_bf0: DSCR_BF0,
    dscr_bf1: DSCR_BF1,
    _reserved14: [u8; 0x04],
    arb: ARB,
    ro_status: RO_STATUS,
    ro_pd_conf: RO_PD_CONF,
    _reserved17: [u8; 0x08],
    mode_enable: MODE_ENABLE,
    mode_yuv: MODE_YUV,
    _reserved19: [u8; 0x10],
    etm_conf: ETM_CONF,
    _reserved20: [u8; 0x04],
    buf_len: BUF_LEN,
    fifo_bcnt: FIFO_BCNT,
    push_bytecnt: PUSH_BYTECNT,
    xaddr: XADDR,
    block_buf_len: BLOCK_BUF_LEN,
    _reserved_end: [u8; 0x7c],
}
impl OUT_CH {
    #[doc = "0x00 - TX CHx config0 register"]
    #[inline(always)]
    pub const fn conf0(&self) -> &CONF0 {
        &self.conf0
    }
    #[doc = "0x04 - TX CHx interrupt raw register"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x08 - TX CHx interrupt ena register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x0c - TX CHx interrupt st register"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x10 - TX CHx interrupt clr register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x14 - TX CHx outfifo status register"]
    #[inline(always)]
    pub const fn fifo_status(&self) -> &FIFO_STATUS {
        &self.fifo_status
    }
    #[doc = "0x18 - TX CHx outfifo push register"]
    #[inline(always)]
    pub const fn push(&self) -> &PUSH {
        &self.push
    }
    #[doc = "0x1c - TX CHx out_link dscr ctrl register"]
    #[inline(always)]
    pub const fn link_conf(&self) -> &LINK_CONF {
        &self.link_conf
    }
    #[doc = "0x20 - TX CHx out_link dscr addr register"]
    #[inline(always)]
    pub const fn link_addr(&self) -> &LINK_ADDR {
        &self.link_addr
    }
    #[doc = "0x24 - TX CHx state register"]
    #[inline(always)]
    pub const fn state(&self) -> &STATE {
        &self.state
    }
    #[doc = "0x28 - TX CHx eof des addr register"]
    #[inline(always)]
    pub const fn eof_des_addr(&self) -> &EOF_DES_ADDR {
        &self.eof_des_addr
    }
    #[doc = "0x2c - TX CHx next dscr addr register"]
    #[inline(always)]
    pub const fn dscr(&self) -> &DSCR {
        &self.dscr
    }
    #[doc = "0x30 - TX CHx last dscr addr register"]
    #[inline(always)]
    pub const fn dscr_bf0(&self) -> &DSCR_BF0 {
        &self.dscr_bf0
    }
    #[doc = "0x34 - TX CHx second-to-last dscr addr register"]
    #[inline(always)]
    pub const fn dscr_bf1(&self) -> &DSCR_BF1 {
        &self.dscr_bf1
    }
    #[doc = "0x3c - TX CHx arb register"]
    #[inline(always)]
    pub const fn arb(&self) -> &ARB {
        &self.arb
    }
    #[doc = "0x40 - TX CHx reorder status register. Available on CH0"]
    #[inline(always)]
    pub const fn ro_status(&self) -> &RO_STATUS {
        &self.ro_status
    }
    #[doc = "0x44 - TX CHx reorder power config register. Available on CH0"]
    #[inline(always)]
    pub const fn ro_pd_conf(&self) -> &RO_PD_CONF {
        &self.ro_pd_conf
    }
    #[doc = "0x50 - TX CHx mode enable register. Available on CH0"]
    #[inline(always)]
    pub const fn mode_enable(&self) -> &MODE_ENABLE {
        &self.mode_enable
    }
    #[doc = "0x54 - TX CHx test mode yuv value register. Available on CH0"]
    #[inline(always)]
    pub const fn mode_yuv(&self) -> &MODE_YUV {
        &self.mode_yuv
    }
    #[doc = "0x68 - TX CHx ETM config register"]
    #[inline(always)]
    pub const fn etm_conf(&self) -> &ETM_CONF {
        &self.etm_conf
    }
    #[doc = "0x70 - TX CHx buf len register"]
    #[inline(always)]
    pub const fn buf_len(&self) -> &BUF_LEN {
        &self.buf_len
    }
    #[doc = "0x74 - TX CHx fifo byte cnt register"]
    #[inline(always)]
    pub const fn fifo_bcnt(&self) -> &FIFO_BCNT {
        &self.fifo_bcnt
    }
    #[doc = "0x78 - TX CHx push byte cnt register"]
    #[inline(always)]
    pub const fn push_bytecnt(&self) -> &PUSH_BYTECNT {
        &self.push_bytecnt
    }
    #[doc = "0x7c - TX CHx xaddr register"]
    #[inline(always)]
    pub const fn xaddr(&self) -> &XADDR {
        &self.xaddr
    }
    #[doc = "0x80 - TX CHx block buf len register"]
    #[inline(always)]
    pub const fn block_buf_len(&self) -> &BLOCK_BUF_LEN {
        &self.block_buf_len
    }
}
#[doc = "CONF0 (rw) register accessor: TX CHx config0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf0`] module"]
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
#[doc = "TX CHx config0 register"]
pub mod conf0;
#[doc = "INT_RAW (rw) register accessor: TX CHx interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "TX CHx interrupt raw register"]
pub mod int_raw;
#[doc = "INT_ENA (rw) register accessor: TX CHx interrupt ena register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "TX CHx interrupt ena register"]
pub mod int_ena;
#[doc = "INT_ST (r) register accessor: TX CHx interrupt st register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "TX CHx interrupt st register"]
pub mod int_st;
#[doc = "INT_CLR (w) register accessor: TX CHx interrupt clr register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "TX CHx interrupt clr register"]
pub mod int_clr;
#[doc = "FIFO_STATUS (r) register accessor: TX CHx outfifo status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_status`] module"]
pub type FIFO_STATUS = crate::Reg<fifo_status::FIFO_STATUS_SPEC>;
#[doc = "TX CHx outfifo status register"]
pub mod fifo_status;
#[doc = "PUSH (rw) register accessor: TX CHx outfifo push register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`push::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`push::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@push`] module"]
pub type PUSH = crate::Reg<push::PUSH_SPEC>;
#[doc = "TX CHx outfifo push register"]
pub mod push;
#[doc = "LINK_CONF (rw) register accessor: TX CHx out_link dscr ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`link_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`link_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link_conf`] module"]
pub type LINK_CONF = crate::Reg<link_conf::LINK_CONF_SPEC>;
#[doc = "TX CHx out_link dscr ctrl register"]
pub mod link_conf;
#[doc = "LINK_ADDR (rw) register accessor: TX CHx out_link dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`link_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`link_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link_addr`] module"]
pub type LINK_ADDR = crate::Reg<link_addr::LINK_ADDR_SPEC>;
#[doc = "TX CHx out_link dscr addr register"]
pub mod link_addr;
#[doc = "STATE (r) register accessor: TX CHx state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`] module"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "TX CHx state register"]
pub mod state;
#[doc = "EOF_DES_ADDR (r) register accessor: TX CHx eof des addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eof_des_addr`] module"]
pub type EOF_DES_ADDR = crate::Reg<eof_des_addr::EOF_DES_ADDR_SPEC>;
#[doc = "TX CHx eof des addr register"]
pub mod eof_des_addr;
#[doc = "DSCR (r) register accessor: TX CHx next dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dscr`] module"]
pub type DSCR = crate::Reg<dscr::DSCR_SPEC>;
#[doc = "TX CHx next dscr addr register"]
pub mod dscr;
#[doc = "DSCR_BF0 (r) register accessor: TX CHx last dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dscr_bf0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dscr_bf0`] module"]
pub type DSCR_BF0 = crate::Reg<dscr_bf0::DSCR_BF0_SPEC>;
#[doc = "TX CHx last dscr addr register"]
pub mod dscr_bf0;
#[doc = "DSCR_BF1 (r) register accessor: TX CHx second-to-last dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dscr_bf1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dscr_bf1`] module"]
pub type DSCR_BF1 = crate::Reg<dscr_bf1::DSCR_BF1_SPEC>;
#[doc = "TX CHx second-to-last dscr addr register"]
pub mod dscr_bf1;
#[doc = "ARB (rw) register accessor: TX CHx arb register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb`] module"]
pub type ARB = crate::Reg<arb::ARB_SPEC>;
#[doc = "TX CHx arb register"]
pub mod arb;
#[doc = "RO_STATUS (r) register accessor: TX CHx reorder status register. Available on CH0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ro_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ro_status`] module"]
pub type RO_STATUS = crate::Reg<ro_status::RO_STATUS_SPEC>;
#[doc = "TX CHx reorder status register. Available on CH0"]
pub mod ro_status;
#[doc = "RO_PD_CONF (rw) register accessor: TX CHx reorder power config register. Available on CH0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ro_pd_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ro_pd_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ro_pd_conf`] module"]
pub type RO_PD_CONF = crate::Reg<ro_pd_conf::RO_PD_CONF_SPEC>;
#[doc = "TX CHx reorder power config register. Available on CH0"]
pub mod ro_pd_conf;
#[doc = "MODE_ENABLE (rw) register accessor: TX CHx mode enable register. Available on CH0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode_enable`] module"]
pub type MODE_ENABLE = crate::Reg<mode_enable::MODE_ENABLE_SPEC>;
#[doc = "TX CHx mode enable register. Available on CH0"]
pub mod mode_enable;
#[doc = "MODE_YUV (rw) register accessor: TX CHx test mode yuv value register. Available on CH0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode_yuv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode_yuv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode_yuv`] module"]
pub type MODE_YUV = crate::Reg<mode_yuv::MODE_YUV_SPEC>;
#[doc = "TX CHx test mode yuv value register. Available on CH0"]
pub mod mode_yuv;
#[doc = "ETM_CONF (rw) register accessor: TX CHx ETM config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etm_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_conf`] module"]
pub type ETM_CONF = crate::Reg<etm_conf::ETM_CONF_SPEC>;
#[doc = "TX CHx ETM config register"]
pub mod etm_conf;
#[doc = "BUF_LEN (r) register accessor: TX CHx buf len register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_len::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_len`] module"]
pub type BUF_LEN = crate::Reg<buf_len::BUF_LEN_SPEC>;
#[doc = "TX CHx buf len register"]
pub mod buf_len;
#[doc = "FIFO_BCNT (r) register accessor: TX CHx fifo byte cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_bcnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_bcnt`] module"]
pub type FIFO_BCNT = crate::Reg<fifo_bcnt::FIFO_BCNT_SPEC>;
#[doc = "TX CHx fifo byte cnt register"]
pub mod fifo_bcnt;
#[doc = "PUSH_BYTECNT (r) register accessor: TX CHx push byte cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`push_bytecnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@push_bytecnt`] module"]
pub type PUSH_BYTECNT = crate::Reg<push_bytecnt::PUSH_BYTECNT_SPEC>;
#[doc = "TX CHx push byte cnt register"]
pub mod push_bytecnt;
#[doc = "XADDR (r) register accessor: TX CHx xaddr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xaddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xaddr`] module"]
pub type XADDR = crate::Reg<xaddr::XADDR_SPEC>;
#[doc = "TX CHx xaddr register"]
pub mod xaddr;
#[doc = "BLOCK_BUF_LEN (r) register accessor: TX CHx block buf len register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`block_buf_len::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@block_buf_len`] module"]
pub type BLOCK_BUF_LEN = crate::Reg<block_buf_len::BLOCK_BUF_LEN_SPEC>;
#[doc = "TX CHx block buf len register"]
pub mod block_buf_len;
