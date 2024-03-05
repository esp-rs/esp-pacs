#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    conf: CONF,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    timing: TIMING,
    fifo_conf: FIFO_CONF,
    rxeof_num: RXEOF_NUM,
    conf_sigle_data: CONF_SIGLE_DATA,
    conf_chan: CONF_CHAN,
    out_link: OUT_LINK,
    in_link: IN_LINK,
    out_eof_des_addr: OUT_EOF_DES_ADDR,
    in_eof_des_addr: IN_EOF_DES_ADDR,
    out_eof_bfr_des_addr: OUT_EOF_BFR_DES_ADDR,
    ahb_test: AHB_TEST,
    inlink_dscr: INLINK_DSCR,
    inlink_dscr_bf0: INLINK_DSCR_BF0,
    inlink_dscr_bf1: INLINK_DSCR_BF1,
    outlink_dscr: OUTLINK_DSCR,
    outlink_dscr_bf0: OUTLINK_DSCR_BF0,
    outlink_dscr_bf1: OUTLINK_DSCR_BF1,
    lc_conf: LC_CONF,
    outfifo_push: OUTFIFO_PUSH,
    infifo_pop: INFIFO_POP,
    lc_state0: LC_STATE0,
    lc_state1: LC_STATE1,
    lc_hung_conf: LC_HUNG_CONF,
    _reserved28: [u8; 0x08],
    cvsd_conf0: CVSD_CONF0,
    cvsd_conf1: CVSD_CONF1,
    cvsd_conf2: CVSD_CONF2,
    plc_conf0: PLC_CONF0,
    plc_conf1: PLC_CONF1,
    plc_conf2: PLC_CONF2,
    esco_conf0: ESCO_CONF0,
    sco_conf0: SCO_CONF0,
    conf1: CONF1,
    pd_conf: PD_CONF,
    conf2: CONF2,
    clkm_conf: CLKM_CONF,
    sample_rate_conf: SAMPLE_RATE_CONF,
    pdm_conf: PDM_CONF,
    pdm_freq_conf: PDM_FREQ_CONF,
    state: STATE,
    _reserved44: [u8; 0x3c],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn timing(&self) -> &TIMING {
        &self.timing
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn fifo_conf(&self) -> &FIFO_CONF {
        &self.fifo_conf
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn rxeof_num(&self) -> &RXEOF_NUM {
        &self.rxeof_num
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn conf_sigle_data(&self) -> &CONF_SIGLE_DATA {
        &self.conf_sigle_data
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn conf_chan(&self) -> &CONF_CHAN {
        &self.conf_chan
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn out_link(&self) -> &OUT_LINK {
        &self.out_link
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn in_link(&self) -> &IN_LINK {
        &self.in_link
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn out_eof_des_addr(&self) -> &OUT_EOF_DES_ADDR {
        &self.out_eof_des_addr
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn in_eof_des_addr(&self) -> &IN_EOF_DES_ADDR {
        &self.in_eof_des_addr
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn out_eof_bfr_des_addr(&self) -> &OUT_EOF_BFR_DES_ADDR {
        &self.out_eof_bfr_des_addr
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn ahb_test(&self) -> &AHB_TEST {
        &self.ahb_test
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn inlink_dscr(&self) -> &INLINK_DSCR {
        &self.inlink_dscr
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn inlink_dscr_bf0(&self) -> &INLINK_DSCR_BF0 {
        &self.inlink_dscr_bf0
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn inlink_dscr_bf1(&self) -> &INLINK_DSCR_BF1 {
        &self.inlink_dscr_bf1
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn outlink_dscr(&self) -> &OUTLINK_DSCR {
        &self.outlink_dscr
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn outlink_dscr_bf0(&self) -> &OUTLINK_DSCR_BF0 {
        &self.outlink_dscr_bf0
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn outlink_dscr_bf1(&self) -> &OUTLINK_DSCR_BF1 {
        &self.outlink_dscr_bf1
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn lc_conf(&self) -> &LC_CONF {
        &self.lc_conf
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn outfifo_push(&self) -> &OUTFIFO_PUSH {
        &self.outfifo_push
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn infifo_pop(&self) -> &INFIFO_POP {
        &self.infifo_pop
    }
    #[doc = "0x6c - "]
    #[inline(always)]
    pub const fn lc_state0(&self) -> &LC_STATE0 {
        &self.lc_state0
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn lc_state1(&self) -> &LC_STATE1 {
        &self.lc_state1
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn lc_hung_conf(&self) -> &LC_HUNG_CONF {
        &self.lc_hung_conf
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn cvsd_conf0(&self) -> &CVSD_CONF0 {
        &self.cvsd_conf0
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn cvsd_conf1(&self) -> &CVSD_CONF1 {
        &self.cvsd_conf1
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn cvsd_conf2(&self) -> &CVSD_CONF2 {
        &self.cvsd_conf2
    }
    #[doc = "0x8c - "]
    #[inline(always)]
    pub const fn plc_conf0(&self) -> &PLC_CONF0 {
        &self.plc_conf0
    }
    #[doc = "0x90 - "]
    #[inline(always)]
    pub const fn plc_conf1(&self) -> &PLC_CONF1 {
        &self.plc_conf1
    }
    #[doc = "0x94 - "]
    #[inline(always)]
    pub const fn plc_conf2(&self) -> &PLC_CONF2 {
        &self.plc_conf2
    }
    #[doc = "0x98 - "]
    #[inline(always)]
    pub const fn esco_conf0(&self) -> &ESCO_CONF0 {
        &self.esco_conf0
    }
    #[doc = "0x9c - "]
    #[inline(always)]
    pub const fn sco_conf0(&self) -> &SCO_CONF0 {
        &self.sco_conf0
    }
    #[doc = "0xa0 - "]
    #[inline(always)]
    pub const fn conf1(&self) -> &CONF1 {
        &self.conf1
    }
    #[doc = "0xa4 - "]
    #[inline(always)]
    pub const fn pd_conf(&self) -> &PD_CONF {
        &self.pd_conf
    }
    #[doc = "0xa8 - "]
    #[inline(always)]
    pub const fn conf2(&self) -> &CONF2 {
        &self.conf2
    }
    #[doc = "0xac - "]
    #[inline(always)]
    pub const fn clkm_conf(&self) -> &CLKM_CONF {
        &self.clkm_conf
    }
    #[doc = "0xb0 - "]
    #[inline(always)]
    pub const fn sample_rate_conf(&self) -> &SAMPLE_RATE_CONF {
        &self.sample_rate_conf
    }
    #[doc = "0xb4 - "]
    #[inline(always)]
    pub const fn pdm_conf(&self) -> &PDM_CONF {
        &self.pdm_conf
    }
    #[doc = "0xb8 - "]
    #[inline(always)]
    pub const fn pdm_freq_conf(&self) -> &PDM_FREQ_CONF {
        &self.pdm_freq_conf
    }
    #[doc = "0xbc - "]
    #[inline(always)]
    pub const fn state(&self) -> &STATE {
        &self.state
    }
    #[doc = "0xfc - "]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`] module"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = ""]
pub mod conf;
#[doc = "INT_RAW (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = ""]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = ""]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = ""]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = ""]
pub mod int_clr;
#[doc = "TIMING (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timing::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timing::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timing`] module"]
pub type TIMING = crate::Reg<timing::TIMING_SPEC>;
#[doc = ""]
pub mod timing;
#[doc = "FIFO_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_conf`] module"]
pub type FIFO_CONF = crate::Reg<fifo_conf::FIFO_CONF_SPEC>;
#[doc = ""]
pub mod fifo_conf;
#[doc = "RXEOF_NUM (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxeof_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxeof_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxeof_num`] module"]
pub type RXEOF_NUM = crate::Reg<rxeof_num::RXEOF_NUM_SPEC>;
#[doc = ""]
pub mod rxeof_num;
#[doc = "CONF_SIGLE_DATA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf_sigle_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_sigle_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf_sigle_data`] module"]
pub type CONF_SIGLE_DATA = crate::Reg<conf_sigle_data::CONF_SIGLE_DATA_SPEC>;
#[doc = ""]
pub mod conf_sigle_data;
#[doc = "CONF_CHAN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf_chan::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_chan::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf_chan`] module"]
pub type CONF_CHAN = crate::Reg<conf_chan::CONF_CHAN_SPEC>;
#[doc = ""]
pub mod conf_chan;
#[doc = "OUT_LINK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link`] module"]
pub type OUT_LINK = crate::Reg<out_link::OUT_LINK_SPEC>;
#[doc = ""]
pub mod out_link;
#[doc = "IN_LINK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link`] module"]
pub type IN_LINK = crate::Reg<in_link::IN_LINK_SPEC>;
#[doc = ""]
pub mod in_link;
#[doc = "OUT_EOF_DES_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_eof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_des_addr`] module"]
pub type OUT_EOF_DES_ADDR = crate::Reg<out_eof_des_addr::OUT_EOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod out_eof_des_addr;
#[doc = "IN_EOF_DES_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_eof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_eof_des_addr`] module"]
pub type IN_EOF_DES_ADDR = crate::Reg<in_eof_des_addr::IN_EOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod in_eof_des_addr;
#[doc = "OUT_EOF_BFR_DES_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_eof_bfr_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_bfr_des_addr`] module"]
pub type OUT_EOF_BFR_DES_ADDR = crate::Reg<out_eof_bfr_des_addr::OUT_EOF_BFR_DES_ADDR_SPEC>;
#[doc = ""]
pub mod out_eof_bfr_des_addr;
#[doc = "AHB_TEST (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_test`] module"]
pub type AHB_TEST = crate::Reg<ahb_test::AHB_TEST_SPEC>;
#[doc = ""]
pub mod ahb_test;
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
#[doc = "LC_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lc_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lc_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lc_conf`] module"]
pub type LC_CONF = crate::Reg<lc_conf::LC_CONF_SPEC>;
#[doc = ""]
pub mod lc_conf;
#[doc = "OUTFIFO_PUSH (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outfifo_push::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outfifo_push::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outfifo_push`] module"]
pub type OUTFIFO_PUSH = crate::Reg<outfifo_push::OUTFIFO_PUSH_SPEC>;
#[doc = ""]
pub mod outfifo_push;
#[doc = "INFIFO_POP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`infifo_pop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`infifo_pop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@infifo_pop`] module"]
pub type INFIFO_POP = crate::Reg<infifo_pop::INFIFO_POP_SPEC>;
#[doc = ""]
pub mod infifo_pop;
#[doc = "LC_STATE0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lc_state0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lc_state0`] module"]
pub type LC_STATE0 = crate::Reg<lc_state0::LC_STATE0_SPEC>;
#[doc = ""]
pub mod lc_state0;
#[doc = "LC_STATE1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lc_state1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lc_state1`] module"]
pub type LC_STATE1 = crate::Reg<lc_state1::LC_STATE1_SPEC>;
#[doc = ""]
pub mod lc_state1;
#[doc = "LC_HUNG_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lc_hung_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lc_hung_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lc_hung_conf`] module"]
pub type LC_HUNG_CONF = crate::Reg<lc_hung_conf::LC_HUNG_CONF_SPEC>;
#[doc = ""]
pub mod lc_hung_conf;
#[doc = "CVSD_CONF0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cvsd_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cvsd_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cvsd_conf0`] module"]
pub type CVSD_CONF0 = crate::Reg<cvsd_conf0::CVSD_CONF0_SPEC>;
#[doc = ""]
pub mod cvsd_conf0;
#[doc = "CVSD_CONF1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cvsd_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cvsd_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cvsd_conf1`] module"]
pub type CVSD_CONF1 = crate::Reg<cvsd_conf1::CVSD_CONF1_SPEC>;
#[doc = ""]
pub mod cvsd_conf1;
#[doc = "CVSD_CONF2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cvsd_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cvsd_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cvsd_conf2`] module"]
pub type CVSD_CONF2 = crate::Reg<cvsd_conf2::CVSD_CONF2_SPEC>;
#[doc = ""]
pub mod cvsd_conf2;
#[doc = "PLC_CONF0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plc_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plc_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plc_conf0`] module"]
pub type PLC_CONF0 = crate::Reg<plc_conf0::PLC_CONF0_SPEC>;
#[doc = ""]
pub mod plc_conf0;
#[doc = "PLC_CONF1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plc_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plc_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plc_conf1`] module"]
pub type PLC_CONF1 = crate::Reg<plc_conf1::PLC_CONF1_SPEC>;
#[doc = ""]
pub mod plc_conf1;
#[doc = "PLC_CONF2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plc_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plc_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plc_conf2`] module"]
pub type PLC_CONF2 = crate::Reg<plc_conf2::PLC_CONF2_SPEC>;
#[doc = ""]
pub mod plc_conf2;
#[doc = "ESCO_CONF0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esco_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`esco_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esco_conf0`] module"]
pub type ESCO_CONF0 = crate::Reg<esco_conf0::ESCO_CONF0_SPEC>;
#[doc = ""]
pub mod esco_conf0;
#[doc = "SCO_CONF0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sco_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sco_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sco_conf0`] module"]
pub type SCO_CONF0 = crate::Reg<sco_conf0::SCO_CONF0_SPEC>;
#[doc = ""]
pub mod sco_conf0;
#[doc = "CONF1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf1`] module"]
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
#[doc = ""]
pub mod conf1;
#[doc = "PD_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_conf`] module"]
pub type PD_CONF = crate::Reg<pd_conf::PD_CONF_SPEC>;
#[doc = ""]
pub mod pd_conf;
#[doc = "CONF2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf2`] module"]
pub type CONF2 = crate::Reg<conf2::CONF2_SPEC>;
#[doc = ""]
pub mod conf2;
#[doc = "CLKM_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkm_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkm_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkm_conf`] module"]
pub type CLKM_CONF = crate::Reg<clkm_conf::CLKM_CONF_SPEC>;
#[doc = ""]
pub mod clkm_conf;
#[doc = "SAMPLE_RATE_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sample_rate_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sample_rate_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sample_rate_conf`] module"]
pub type SAMPLE_RATE_CONF = crate::Reg<sample_rate_conf::SAMPLE_RATE_CONF_SPEC>;
#[doc = ""]
pub mod sample_rate_conf;
#[doc = "PDM_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdm_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdm_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdm_conf`] module"]
pub type PDM_CONF = crate::Reg<pdm_conf::PDM_CONF_SPEC>;
#[doc = ""]
pub mod pdm_conf;
#[doc = "PDM_FREQ_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdm_freq_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdm_freq_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdm_freq_conf`] module"]
pub type PDM_FREQ_CONF = crate::Reg<pdm_freq_conf::PDM_FREQ_CONF_SPEC>;
#[doc = ""]
pub mod pdm_freq_conf;
#[doc = "STATE (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`] module"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = ""]
pub mod state;
#[doc = "DATE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
