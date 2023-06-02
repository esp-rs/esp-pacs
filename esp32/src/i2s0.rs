#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - "]
    pub conf: CONF,
    #[doc = "0x0c - "]
    pub int_raw: INT_RAW,
    #[doc = "0x10 - "]
    pub int_st: INT_ST,
    #[doc = "0x14 - "]
    pub int_ena: INT_ENA,
    #[doc = "0x18 - "]
    pub int_clr: INT_CLR,
    #[doc = "0x1c - "]
    pub timing: TIMING,
    #[doc = "0x20 - "]
    pub fifo_conf: FIFO_CONF,
    #[doc = "0x24 - "]
    pub rxeof_num: RXEOF_NUM,
    #[doc = "0x28 - "]
    pub conf_sigle_data: CONF_SIGLE_DATA,
    #[doc = "0x2c - "]
    pub conf_chan: CONF_CHAN,
    #[doc = "0x30 - "]
    pub out_link: OUT_LINK,
    #[doc = "0x34 - "]
    pub in_link: IN_LINK,
    #[doc = "0x38 - "]
    pub out_eof_des_addr: OUT_EOF_DES_ADDR,
    #[doc = "0x3c - "]
    pub in_eof_des_addr: IN_EOF_DES_ADDR,
    #[doc = "0x40 - "]
    pub out_eof_bfr_des_addr: OUT_EOF_BFR_DES_ADDR,
    #[doc = "0x44 - "]
    pub ahb_test: AHB_TEST,
    #[doc = "0x48 - "]
    pub inlink_dscr: INLINK_DSCR,
    #[doc = "0x4c - "]
    pub inlink_dscr_bf0: INLINK_DSCR_BF0,
    #[doc = "0x50 - "]
    pub inlink_dscr_bf1: INLINK_DSCR_BF1,
    #[doc = "0x54 - "]
    pub outlink_dscr: OUTLINK_DSCR,
    #[doc = "0x58 - "]
    pub outlink_dscr_bf0: OUTLINK_DSCR_BF0,
    #[doc = "0x5c - "]
    pub outlink_dscr_bf1: OUTLINK_DSCR_BF1,
    #[doc = "0x60 - "]
    pub lc_conf: LC_CONF,
    #[doc = "0x64 - "]
    pub outfifo_push: OUTFIFO_PUSH,
    #[doc = "0x68 - "]
    pub infifo_pop: INFIFO_POP,
    #[doc = "0x6c - "]
    pub lc_state0: LC_STATE0,
    #[doc = "0x70 - "]
    pub lc_state1: LC_STATE1,
    #[doc = "0x74 - "]
    pub lc_hung_conf: LC_HUNG_CONF,
    _reserved28: [u8; 0x08],
    #[doc = "0x80 - "]
    pub cvsd_conf0: CVSD_CONF0,
    #[doc = "0x84 - "]
    pub cvsd_conf1: CVSD_CONF1,
    #[doc = "0x88 - "]
    pub cvsd_conf2: CVSD_CONF2,
    #[doc = "0x8c - "]
    pub plc_conf0: PLC_CONF0,
    #[doc = "0x90 - "]
    pub plc_conf1: PLC_CONF1,
    #[doc = "0x94 - "]
    pub plc_conf2: PLC_CONF2,
    #[doc = "0x98 - "]
    pub esco_conf0: ESCO_CONF0,
    #[doc = "0x9c - "]
    pub sco_conf0: SCO_CONF0,
    #[doc = "0xa0 - "]
    pub conf1: CONF1,
    #[doc = "0xa4 - "]
    pub pd_conf: PD_CONF,
    #[doc = "0xa8 - "]
    pub conf2: CONF2,
    #[doc = "0xac - "]
    pub clkm_conf: CLKM_CONF,
    #[doc = "0xb0 - "]
    pub sample_rate_conf: SAMPLE_RATE_CONF,
    #[doc = "0xb4 - "]
    pub pdm_conf: PDM_CONF,
    #[doc = "0xb8 - "]
    pub pdm_freq_conf: PDM_FREQ_CONF,
    #[doc = "0xbc - "]
    pub state: STATE,
    _reserved44: [u8; 0x3c],
    #[doc = "0xfc - "]
    pub date: DATE,
}
#[doc = "CONF (rw) register accessor: an alias for `Reg<CONF_SPEC>`"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = ""]
pub mod conf;
#[doc = "INT_RAW (r) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = ""]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = ""]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = ""]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = ""]
pub mod int_clr;
#[doc = "TIMING (rw) register accessor: an alias for `Reg<TIMING_SPEC>`"]
pub type TIMING = crate::Reg<timing::TIMING_SPEC>;
#[doc = ""]
pub mod timing;
#[doc = "FIFO_CONF (rw) register accessor: an alias for `Reg<FIFO_CONF_SPEC>`"]
pub type FIFO_CONF = crate::Reg<fifo_conf::FIFO_CONF_SPEC>;
#[doc = ""]
pub mod fifo_conf;
#[doc = "RXEOF_NUM (rw) register accessor: an alias for `Reg<RXEOF_NUM_SPEC>`"]
pub type RXEOF_NUM = crate::Reg<rxeof_num::RXEOF_NUM_SPEC>;
#[doc = ""]
pub mod rxeof_num;
#[doc = "CONF_SIGLE_DATA (rw) register accessor: an alias for `Reg<CONF_SIGLE_DATA_SPEC>`"]
pub type CONF_SIGLE_DATA = crate::Reg<conf_sigle_data::CONF_SIGLE_DATA_SPEC>;
#[doc = ""]
pub mod conf_sigle_data;
#[doc = "CONF_CHAN (rw) register accessor: an alias for `Reg<CONF_CHAN_SPEC>`"]
pub type CONF_CHAN = crate::Reg<conf_chan::CONF_CHAN_SPEC>;
#[doc = ""]
pub mod conf_chan;
#[doc = "OUT_LINK (rw) register accessor: an alias for `Reg<OUT_LINK_SPEC>`"]
pub type OUT_LINK = crate::Reg<out_link::OUT_LINK_SPEC>;
#[doc = ""]
pub mod out_link;
#[doc = "IN_LINK (rw) register accessor: an alias for `Reg<IN_LINK_SPEC>`"]
pub type IN_LINK = crate::Reg<in_link::IN_LINK_SPEC>;
#[doc = ""]
pub mod in_link;
#[doc = "OUT_EOF_DES_ADDR (r) register accessor: an alias for `Reg<OUT_EOF_DES_ADDR_SPEC>`"]
pub type OUT_EOF_DES_ADDR = crate::Reg<out_eof_des_addr::OUT_EOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod out_eof_des_addr;
#[doc = "IN_EOF_DES_ADDR (r) register accessor: an alias for `Reg<IN_EOF_DES_ADDR_SPEC>`"]
pub type IN_EOF_DES_ADDR = crate::Reg<in_eof_des_addr::IN_EOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod in_eof_des_addr;
#[doc = "OUT_EOF_BFR_DES_ADDR (r) register accessor: an alias for `Reg<OUT_EOF_BFR_DES_ADDR_SPEC>`"]
pub type OUT_EOF_BFR_DES_ADDR = crate::Reg<out_eof_bfr_des_addr::OUT_EOF_BFR_DES_ADDR_SPEC>;
#[doc = ""]
pub mod out_eof_bfr_des_addr;
#[doc = "AHB_TEST (rw) register accessor: an alias for `Reg<AHB_TEST_SPEC>`"]
pub type AHB_TEST = crate::Reg<ahb_test::AHB_TEST_SPEC>;
#[doc = ""]
pub mod ahb_test;
#[doc = "INLINK_DSCR (r) register accessor: an alias for `Reg<INLINK_DSCR_SPEC>`"]
pub type INLINK_DSCR = crate::Reg<inlink_dscr::INLINK_DSCR_SPEC>;
#[doc = ""]
pub mod inlink_dscr;
#[doc = "INLINK_DSCR_BF0 (r) register accessor: an alias for `Reg<INLINK_DSCR_BF0_SPEC>`"]
pub type INLINK_DSCR_BF0 = crate::Reg<inlink_dscr_bf0::INLINK_DSCR_BF0_SPEC>;
#[doc = ""]
pub mod inlink_dscr_bf0;
#[doc = "INLINK_DSCR_BF1 (r) register accessor: an alias for `Reg<INLINK_DSCR_BF1_SPEC>`"]
pub type INLINK_DSCR_BF1 = crate::Reg<inlink_dscr_bf1::INLINK_DSCR_BF1_SPEC>;
#[doc = ""]
pub mod inlink_dscr_bf1;
#[doc = "OUTLINK_DSCR (r) register accessor: an alias for `Reg<OUTLINK_DSCR_SPEC>`"]
pub type OUTLINK_DSCR = crate::Reg<outlink_dscr::OUTLINK_DSCR_SPEC>;
#[doc = ""]
pub mod outlink_dscr;
#[doc = "OUTLINK_DSCR_BF0 (r) register accessor: an alias for `Reg<OUTLINK_DSCR_BF0_SPEC>`"]
pub type OUTLINK_DSCR_BF0 = crate::Reg<outlink_dscr_bf0::OUTLINK_DSCR_BF0_SPEC>;
#[doc = ""]
pub mod outlink_dscr_bf0;
#[doc = "OUTLINK_DSCR_BF1 (r) register accessor: an alias for `Reg<OUTLINK_DSCR_BF1_SPEC>`"]
pub type OUTLINK_DSCR_BF1 = crate::Reg<outlink_dscr_bf1::OUTLINK_DSCR_BF1_SPEC>;
#[doc = ""]
pub mod outlink_dscr_bf1;
#[doc = "LC_CONF (rw) register accessor: an alias for `Reg<LC_CONF_SPEC>`"]
pub type LC_CONF = crate::Reg<lc_conf::LC_CONF_SPEC>;
#[doc = ""]
pub mod lc_conf;
#[doc = "OUTFIFO_PUSH (rw) register accessor: an alias for `Reg<OUTFIFO_PUSH_SPEC>`"]
pub type OUTFIFO_PUSH = crate::Reg<outfifo_push::OUTFIFO_PUSH_SPEC>;
#[doc = ""]
pub mod outfifo_push;
#[doc = "INFIFO_POP (rw) register accessor: an alias for `Reg<INFIFO_POP_SPEC>`"]
pub type INFIFO_POP = crate::Reg<infifo_pop::INFIFO_POP_SPEC>;
#[doc = ""]
pub mod infifo_pop;
#[doc = "LC_STATE0 (r) register accessor: an alias for `Reg<LC_STATE0_SPEC>`"]
pub type LC_STATE0 = crate::Reg<lc_state0::LC_STATE0_SPEC>;
#[doc = ""]
pub mod lc_state0;
#[doc = "LC_STATE1 (r) register accessor: an alias for `Reg<LC_STATE1_SPEC>`"]
pub type LC_STATE1 = crate::Reg<lc_state1::LC_STATE1_SPEC>;
#[doc = ""]
pub mod lc_state1;
#[doc = "LC_HUNG_CONF (rw) register accessor: an alias for `Reg<LC_HUNG_CONF_SPEC>`"]
pub type LC_HUNG_CONF = crate::Reg<lc_hung_conf::LC_HUNG_CONF_SPEC>;
#[doc = ""]
pub mod lc_hung_conf;
#[doc = "CVSD_CONF0 (rw) register accessor: an alias for `Reg<CVSD_CONF0_SPEC>`"]
pub type CVSD_CONF0 = crate::Reg<cvsd_conf0::CVSD_CONF0_SPEC>;
#[doc = ""]
pub mod cvsd_conf0;
#[doc = "CVSD_CONF1 (rw) register accessor: an alias for `Reg<CVSD_CONF1_SPEC>`"]
pub type CVSD_CONF1 = crate::Reg<cvsd_conf1::CVSD_CONF1_SPEC>;
#[doc = ""]
pub mod cvsd_conf1;
#[doc = "CVSD_CONF2 (rw) register accessor: an alias for `Reg<CVSD_CONF2_SPEC>`"]
pub type CVSD_CONF2 = crate::Reg<cvsd_conf2::CVSD_CONF2_SPEC>;
#[doc = ""]
pub mod cvsd_conf2;
#[doc = "PLC_CONF0 (rw) register accessor: an alias for `Reg<PLC_CONF0_SPEC>`"]
pub type PLC_CONF0 = crate::Reg<plc_conf0::PLC_CONF0_SPEC>;
#[doc = ""]
pub mod plc_conf0;
#[doc = "PLC_CONF1 (rw) register accessor: an alias for `Reg<PLC_CONF1_SPEC>`"]
pub type PLC_CONF1 = crate::Reg<plc_conf1::PLC_CONF1_SPEC>;
#[doc = ""]
pub mod plc_conf1;
#[doc = "PLC_CONF2 (rw) register accessor: an alias for `Reg<PLC_CONF2_SPEC>`"]
pub type PLC_CONF2 = crate::Reg<plc_conf2::PLC_CONF2_SPEC>;
#[doc = ""]
pub mod plc_conf2;
#[doc = "ESCO_CONF0 (rw) register accessor: an alias for `Reg<ESCO_CONF0_SPEC>`"]
pub type ESCO_CONF0 = crate::Reg<esco_conf0::ESCO_CONF0_SPEC>;
#[doc = ""]
pub mod esco_conf0;
#[doc = "SCO_CONF0 (rw) register accessor: an alias for `Reg<SCO_CONF0_SPEC>`"]
pub type SCO_CONF0 = crate::Reg<sco_conf0::SCO_CONF0_SPEC>;
#[doc = ""]
pub mod sco_conf0;
#[doc = "CONF1 (rw) register accessor: an alias for `Reg<CONF1_SPEC>`"]
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
#[doc = ""]
pub mod conf1;
#[doc = "PD_CONF (rw) register accessor: an alias for `Reg<PD_CONF_SPEC>`"]
pub type PD_CONF = crate::Reg<pd_conf::PD_CONF_SPEC>;
#[doc = ""]
pub mod pd_conf;
#[doc = "CONF2 (rw) register accessor: an alias for `Reg<CONF2_SPEC>`"]
pub type CONF2 = crate::Reg<conf2::CONF2_SPEC>;
#[doc = ""]
pub mod conf2;
#[doc = "CLKM_CONF (rw) register accessor: an alias for `Reg<CLKM_CONF_SPEC>`"]
pub type CLKM_CONF = crate::Reg<clkm_conf::CLKM_CONF_SPEC>;
#[doc = ""]
pub mod clkm_conf;
#[doc = "SAMPLE_RATE_CONF (rw) register accessor: an alias for `Reg<SAMPLE_RATE_CONF_SPEC>`"]
pub type SAMPLE_RATE_CONF = crate::Reg<sample_rate_conf::SAMPLE_RATE_CONF_SPEC>;
#[doc = ""]
pub mod sample_rate_conf;
#[doc = "PDM_CONF (rw) register accessor: an alias for `Reg<PDM_CONF_SPEC>`"]
pub type PDM_CONF = crate::Reg<pdm_conf::PDM_CONF_SPEC>;
#[doc = ""]
pub mod pdm_conf;
#[doc = "PDM_FREQ_CONF (rw) register accessor: an alias for `Reg<PDM_FREQ_CONF_SPEC>`"]
pub type PDM_FREQ_CONF = crate::Reg<pdm_freq_conf::PDM_FREQ_CONF_SPEC>;
#[doc = ""]
pub mod pdm_freq_conf;
#[doc = "STATE (r) register accessor: an alias for `Reg<STATE_SPEC>`"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = ""]
pub mod state;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
