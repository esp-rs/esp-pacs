#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - I2S configuration register"]
    pub conf: CONF,
    #[doc = "0x0c - Raw interrupt status"]
    pub int_raw: INT_RAW,
    #[doc = "0x10 - Masked interrupt status"]
    pub int_st: INT_ST,
    #[doc = "0x14 - Interrupt enable bits"]
    pub int_ena: INT_ENA,
    #[doc = "0x18 - Interrupt clear bits"]
    pub int_clr: INT_CLR,
    #[doc = "0x1c - I2S timing register"]
    pub timing: TIMING,
    #[doc = "0x20 - I2S FIFO configuration register"]
    pub fifo_conf: FIFO_CONF,
    #[doc = "0x24 - I2S DMA RX EOF data length"]
    pub rxeof_num: RXEOF_NUM,
    #[doc = "0x28 - Constant single channel data"]
    pub conf_sigle_data: CONF_SIGLE_DATA,
    #[doc = "0x2c - I2S channel configuration register"]
    pub conf_chan: CONF_CHAN,
    #[doc = "0x30 - I2S DMA TX configuration register"]
    pub out_link: OUT_LINK,
    #[doc = "0x34 - I2S DMA RX configuration register"]
    pub in_link: IN_LINK,
    #[doc = "0x38 - Address of outlink descriptor that produces EOF"]
    pub out_eof_des_addr: OUT_EOF_DES_ADDR,
    #[doc = "0x3c - Address of inlink descriptor that produces EOF"]
    pub in_eof_des_addr: IN_EOF_DES_ADDR,
    #[doc = "0x40 - Address of buffer relative to the outlink descriptor that produces EOF"]
    pub out_eof_bfr_des_addr: OUT_EOF_BFR_DES_ADDR,
    _reserved15: [u8; 0x04],
    #[doc = "0x48 - Address of current inlink descriptor"]
    pub inlink_dscr: INLINK_DSCR,
    #[doc = "0x4c - Address of next inlink descriptor"]
    pub inlink_dscr_bf0: INLINK_DSCR_BF0,
    #[doc = "0x50 - Address of next inlink data buffer"]
    pub inlink_dscr_bf1: INLINK_DSCR_BF1,
    #[doc = "0x54 - Address of current outlink descriptor"]
    pub outlink_dscr: OUTLINK_DSCR,
    #[doc = "0x58 - Address of next outlink descriptor"]
    pub outlink_dscr_bf0: OUTLINK_DSCR_BF0,
    #[doc = "0x5c - Address of next outlink data buffer"]
    pub outlink_dscr_bf1: OUTLINK_DSCR_BF1,
    #[doc = "0x60 - I2S DMA configuration register"]
    pub lc_conf: LC_CONF,
    #[doc = "0x64 - APB out FIFO mode register"]
    pub outfifo_push: OUTFIFO_PUSH,
    #[doc = "0x68 - APB in FIFO mode register"]
    pub infifo_pop: INFIFO_POP,
    #[doc = "0x6c - I2S DMA TX status"]
    pub lc_state0: LC_STATE0,
    #[doc = "0x70 - I2S DMA RX status"]
    pub lc_state1: LC_STATE1,
    #[doc = "0x74 - I2S Hung configuration register"]
    pub lc_hung_conf: LC_HUNG_CONF,
    _reserved27: [u8; 0x28],
    #[doc = "0xa0 - I2S configuration register 1"]
    pub conf1: CONF1,
    #[doc = "0xa4 - I2S power-down configuration register"]
    pub pd_conf: PD_CONF,
    #[doc = "0xa8 - I2S configuration register 2"]
    pub conf2: CONF2,
    #[doc = "0xac - I2S module clock configuration register"]
    pub clkm_conf: CLKM_CONF,
    #[doc = "0xb0 - I2S sample rate register"]
    pub sample_rate_conf: SAMPLE_RATE_CONF,
    _reserved32: [u8; 0x08],
    #[doc = "0xbc - I2S TX status register"]
    pub state: STATE,
    _reserved33: [u8; 0x3c],
    #[doc = "0xfc - Version control register"]
    pub date: DATE,
}
#[doc = "CONF (rw) register accessor: an alias for `Reg<CONF_SPEC>`"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "I2S configuration register"]
pub mod conf;
#[doc = "INT_RAW (r) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Raw interrupt status"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Masked interrupt status"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt enable bits"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt clear bits"]
pub mod int_clr;
#[doc = "TIMING (rw) register accessor: an alias for `Reg<TIMING_SPEC>`"]
pub type TIMING = crate::Reg<timing::TIMING_SPEC>;
#[doc = "I2S timing register"]
pub mod timing;
#[doc = "FIFO_CONF (rw) register accessor: an alias for `Reg<FIFO_CONF_SPEC>`"]
pub type FIFO_CONF = crate::Reg<fifo_conf::FIFO_CONF_SPEC>;
#[doc = "I2S FIFO configuration register"]
pub mod fifo_conf;
#[doc = "RXEOF_NUM (rw) register accessor: an alias for `Reg<RXEOF_NUM_SPEC>`"]
pub type RXEOF_NUM = crate::Reg<rxeof_num::RXEOF_NUM_SPEC>;
#[doc = "I2S DMA RX EOF data length"]
pub mod rxeof_num;
#[doc = "CONF_SIGLE_DATA (rw) register accessor: an alias for `Reg<CONF_SIGLE_DATA_SPEC>`"]
pub type CONF_SIGLE_DATA = crate::Reg<conf_sigle_data::CONF_SIGLE_DATA_SPEC>;
#[doc = "Constant single channel data"]
pub mod conf_sigle_data;
#[doc = "CONF_CHAN (rw) register accessor: an alias for `Reg<CONF_CHAN_SPEC>`"]
pub type CONF_CHAN = crate::Reg<conf_chan::CONF_CHAN_SPEC>;
#[doc = "I2S channel configuration register"]
pub mod conf_chan;
#[doc = "OUT_LINK (rw) register accessor: an alias for `Reg<OUT_LINK_SPEC>`"]
pub type OUT_LINK = crate::Reg<out_link::OUT_LINK_SPEC>;
#[doc = "I2S DMA TX configuration register"]
pub mod out_link;
#[doc = "IN_LINK (rw) register accessor: an alias for `Reg<IN_LINK_SPEC>`"]
pub type IN_LINK = crate::Reg<in_link::IN_LINK_SPEC>;
#[doc = "I2S DMA RX configuration register"]
pub mod in_link;
#[doc = "OUT_EOF_DES_ADDR (r) register accessor: an alias for `Reg<OUT_EOF_DES_ADDR_SPEC>`"]
pub type OUT_EOF_DES_ADDR = crate::Reg<out_eof_des_addr::OUT_EOF_DES_ADDR_SPEC>;
#[doc = "Address of outlink descriptor that produces EOF"]
pub mod out_eof_des_addr;
#[doc = "IN_EOF_DES_ADDR (r) register accessor: an alias for `Reg<IN_EOF_DES_ADDR_SPEC>`"]
pub type IN_EOF_DES_ADDR = crate::Reg<in_eof_des_addr::IN_EOF_DES_ADDR_SPEC>;
#[doc = "Address of inlink descriptor that produces EOF"]
pub mod in_eof_des_addr;
#[doc = "OUT_EOF_BFR_DES_ADDR (r) register accessor: an alias for `Reg<OUT_EOF_BFR_DES_ADDR_SPEC>`"]
pub type OUT_EOF_BFR_DES_ADDR = crate::Reg<out_eof_bfr_des_addr::OUT_EOF_BFR_DES_ADDR_SPEC>;
#[doc = "Address of buffer relative to the outlink descriptor that produces EOF"]
pub mod out_eof_bfr_des_addr;
#[doc = "INLINK_DSCR (r) register accessor: an alias for `Reg<INLINK_DSCR_SPEC>`"]
pub type INLINK_DSCR = crate::Reg<inlink_dscr::INLINK_DSCR_SPEC>;
#[doc = "Address of current inlink descriptor"]
pub mod inlink_dscr;
#[doc = "INLINK_DSCR_BF0 (r) register accessor: an alias for `Reg<INLINK_DSCR_BF0_SPEC>`"]
pub type INLINK_DSCR_BF0 = crate::Reg<inlink_dscr_bf0::INLINK_DSCR_BF0_SPEC>;
#[doc = "Address of next inlink descriptor"]
pub mod inlink_dscr_bf0;
#[doc = "INLINK_DSCR_BF1 (r) register accessor: an alias for `Reg<INLINK_DSCR_BF1_SPEC>`"]
pub type INLINK_DSCR_BF1 = crate::Reg<inlink_dscr_bf1::INLINK_DSCR_BF1_SPEC>;
#[doc = "Address of next inlink data buffer"]
pub mod inlink_dscr_bf1;
#[doc = "OUTLINK_DSCR (r) register accessor: an alias for `Reg<OUTLINK_DSCR_SPEC>`"]
pub type OUTLINK_DSCR = crate::Reg<outlink_dscr::OUTLINK_DSCR_SPEC>;
#[doc = "Address of current outlink descriptor"]
pub mod outlink_dscr;
#[doc = "OUTLINK_DSCR_BF0 (r) register accessor: an alias for `Reg<OUTLINK_DSCR_BF0_SPEC>`"]
pub type OUTLINK_DSCR_BF0 = crate::Reg<outlink_dscr_bf0::OUTLINK_DSCR_BF0_SPEC>;
#[doc = "Address of next outlink descriptor"]
pub mod outlink_dscr_bf0;
#[doc = "OUTLINK_DSCR_BF1 (r) register accessor: an alias for `Reg<OUTLINK_DSCR_BF1_SPEC>`"]
pub type OUTLINK_DSCR_BF1 = crate::Reg<outlink_dscr_bf1::OUTLINK_DSCR_BF1_SPEC>;
#[doc = "Address of next outlink data buffer"]
pub mod outlink_dscr_bf1;
#[doc = "LC_CONF (rw) register accessor: an alias for `Reg<LC_CONF_SPEC>`"]
pub type LC_CONF = crate::Reg<lc_conf::LC_CONF_SPEC>;
#[doc = "I2S DMA configuration register"]
pub mod lc_conf;
#[doc = "OUTFIFO_PUSH (rw) register accessor: an alias for `Reg<OUTFIFO_PUSH_SPEC>`"]
pub type OUTFIFO_PUSH = crate::Reg<outfifo_push::OUTFIFO_PUSH_SPEC>;
#[doc = "APB out FIFO mode register"]
pub mod outfifo_push;
#[doc = "INFIFO_POP (rw) register accessor: an alias for `Reg<INFIFO_POP_SPEC>`"]
pub type INFIFO_POP = crate::Reg<infifo_pop::INFIFO_POP_SPEC>;
#[doc = "APB in FIFO mode register"]
pub mod infifo_pop;
#[doc = "LC_STATE0 (r) register accessor: an alias for `Reg<LC_STATE0_SPEC>`"]
pub type LC_STATE0 = crate::Reg<lc_state0::LC_STATE0_SPEC>;
#[doc = "I2S DMA TX status"]
pub mod lc_state0;
#[doc = "LC_STATE1 (r) register accessor: an alias for `Reg<LC_STATE1_SPEC>`"]
pub type LC_STATE1 = crate::Reg<lc_state1::LC_STATE1_SPEC>;
#[doc = "I2S DMA RX status"]
pub mod lc_state1;
#[doc = "LC_HUNG_CONF (rw) register accessor: an alias for `Reg<LC_HUNG_CONF_SPEC>`"]
pub type LC_HUNG_CONF = crate::Reg<lc_hung_conf::LC_HUNG_CONF_SPEC>;
#[doc = "I2S Hung configuration register"]
pub mod lc_hung_conf;
#[doc = "CONF1 (rw) register accessor: an alias for `Reg<CONF1_SPEC>`"]
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
#[doc = "I2S configuration register 1"]
pub mod conf1;
#[doc = "PD_CONF (rw) register accessor: an alias for `Reg<PD_CONF_SPEC>`"]
pub type PD_CONF = crate::Reg<pd_conf::PD_CONF_SPEC>;
#[doc = "I2S power-down configuration register"]
pub mod pd_conf;
#[doc = "CONF2 (rw) register accessor: an alias for `Reg<CONF2_SPEC>`"]
pub type CONF2 = crate::Reg<conf2::CONF2_SPEC>;
#[doc = "I2S configuration register 2"]
pub mod conf2;
#[doc = "CLKM_CONF (rw) register accessor: an alias for `Reg<CLKM_CONF_SPEC>`"]
pub type CLKM_CONF = crate::Reg<clkm_conf::CLKM_CONF_SPEC>;
#[doc = "I2S module clock configuration register"]
pub mod clkm_conf;
#[doc = "SAMPLE_RATE_CONF (rw) register accessor: an alias for `Reg<SAMPLE_RATE_CONF_SPEC>`"]
pub type SAMPLE_RATE_CONF = crate::Reg<sample_rate_conf::SAMPLE_RATE_CONF_SPEC>;
#[doc = "I2S sample rate register"]
pub mod sample_rate_conf;
#[doc = "STATE (r) register accessor: an alias for `Reg<STATE_SPEC>`"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "I2S TX status register"]
pub mod state;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
