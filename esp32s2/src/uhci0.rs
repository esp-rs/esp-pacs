#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - UHCI configuration register"]
    pub conf0: CONF0,
    #[doc = "0x04 - Raw interrupt status"]
    pub int_raw: INT_RAW,
    #[doc = "0x08 - Masked interrupt status"]
    pub int_st: INT_ST,
    #[doc = "0x0c - Interrupt enable bits"]
    pub int_ena: INT_ENA,
    #[doc = "0x10 - Interrupt clear bits"]
    pub int_clr: INT_CLR,
    #[doc = "0x14 - DMA data-output status register"]
    pub dma_out_status: DMA_OUT_STATUS,
    #[doc = "0x18 - Push control register of TX FIFO"]
    pub dma_out_push: DMA_OUT_PUSH,
    #[doc = "0x1c - UHCI data-input status register"]
    pub dma_in_status: DMA_IN_STATUS,
    #[doc = "0x20 - Pop control register of RX FIFO"]
    pub dma_in_pop: DMA_IN_POP,
    #[doc = "0x24 - Link descriptor address and control"]
    pub dma_out_link: DMA_OUT_LINK,
    #[doc = "0x28 - Link descriptor address and control"]
    pub dma_in_link: DMA_IN_LINK,
    #[doc = "0x2c - UHCI configuration register"]
    pub conf1: CONF1,
    #[doc = "0x30 - UHCI decoder status register"]
    pub state0: STATE0,
    #[doc = "0x34 - UHCI encoder status register"]
    pub state1: STATE1,
    #[doc = "0x38 - Outlink descriptor address when EOF occurs"]
    pub dma_out_eof_des_addr: DMA_OUT_EOF_DES_ADDR,
    #[doc = "0x3c - Inlink descriptor address when EOF occurs"]
    pub dma_in_suc_eof_des_addr: DMA_IN_SUC_EOF_DES_ADDR,
    #[doc = "0x40 - Inlink descriptor address when errors occur"]
    pub dma_in_err_eof_des_addr: DMA_IN_ERR_EOF_DES_ADDR,
    #[doc = "0x44 - Outlink descriptor address before the last transmit descriptor"]
    pub dma_out_eof_bfr_des_addr: DMA_OUT_EOF_BFR_DES_ADDR,
    #[doc = "0x48 - AHB test register"]
    pub ahb_test: AHB_TEST,
    #[doc = "0x4c - The third word of the next receive descriptor"]
    pub dma_in_dscr: DMA_IN_DSCR,
    #[doc = "0x50 - The third word of current receive descriptor"]
    pub dma_in_dscr_bf0: DMA_IN_DSCR_BF0,
    _reserved21: [u8; 0x04],
    #[doc = "0x58 - The third word of the next transmit descriptor"]
    pub dma_out_dscr: DMA_OUT_DSCR,
    #[doc = "0x5c - The third word of current transmit descriptor"]
    pub dma_out_dscr_bf0: DMA_OUT_DSCR_BF0,
    _reserved23: [u8; 0x04],
    #[doc = "0x64 - Escape character configuration"]
    pub escape_conf: ESCAPE_CONF,
    #[doc = "0x68 - Timeout configuration"]
    pub hung_conf: HUNG_CONF,
    _reserved25: [u8; 0x04],
    #[doc = "0x70 - UHCI packet header register"]
    pub rx_head: RX_HEAD,
    #[doc = "0x74 - UHCI quick_sent configuration register"]
    pub quick_sent: QUICK_SENT,
    #[doc = "0x78 - Q0_WORD0 quick_sent register"]
    pub q0_word0: Q0_WORD0,
    #[doc = "0x7c - Q0_WORD1 quick_sent register"]
    pub q0_word1: Q0_WORD1,
    #[doc = "0x80 - Q1_WORD0 quick_sent register"]
    pub q1_word0: Q1_WORD0,
    #[doc = "0x84 - Q1_WORD1 quick_sent register"]
    pub q1_word1: Q1_WORD1,
    #[doc = "0x88 - Q2_WORD0 quick_sent register"]
    pub q2_word0: Q2_WORD0,
    #[doc = "0x8c - Q2_WORD1 quick_sent register"]
    pub q2_word1: Q2_WORD1,
    #[doc = "0x90 - Q3_WORD0 quick_sent register"]
    pub q3_word0: Q3_WORD0,
    #[doc = "0x94 - Q3_WORD1 quick_sent register"]
    pub q3_word1: Q3_WORD1,
    #[doc = "0x98 - Q4_WORD0 quick_sent register"]
    pub q4_word0: Q4_WORD0,
    #[doc = "0x9c - Q4_WORD1 quick_sent register"]
    pub q4_word1: Q4_WORD1,
    #[doc = "0xa0 - Q5_WORD0 quick_sent register"]
    pub q5_word0: Q5_WORD0,
    #[doc = "0xa4 - Q5_WORD1 quick_sent register"]
    pub q5_word1: Q5_WORD1,
    #[doc = "0xa8 - Q6_WORD0 quick_sent register"]
    pub q6_word0: Q6_WORD0,
    #[doc = "0xac - Q6_WORD1 quick_sent register"]
    pub q6_word1: Q6_WORD1,
    #[doc = "0xb0 - Escape sequence configuration register 0"]
    pub esc_conf0: ESC_CONF0,
    #[doc = "0xb4 - Escape sequence configuration register 1"]
    pub esc_conf1: ESC_CONF1,
    #[doc = "0xb8 - Escape sequence configuration register 2"]
    pub esc_conf2: ESC_CONF2,
    #[doc = "0xbc - Escape sequence configuration register 3"]
    pub esc_conf3: ESC_CONF3,
    #[doc = "0xc0 - Configure register for packet length"]
    pub pkt_thres: PKT_THRES,
    _reserved46: [u8; 0x38],
    #[doc = "0xfc - UHCI version control register"]
    pub date: DATE,
}
#[doc = "CONF0 (rw) register accessor: an alias for `Reg<CONF0_SPEC>`"]
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
#[doc = "UHCI configuration register"]
pub mod conf0;
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
#[doc = "DMA_OUT_STATUS (r) register accessor: an alias for `Reg<DMA_OUT_STATUS_SPEC>`"]
pub type DMA_OUT_STATUS = crate::Reg<dma_out_status::DMA_OUT_STATUS_SPEC>;
#[doc = "DMA data-output status register"]
pub mod dma_out_status;
#[doc = "DMA_OUT_PUSH (rw) register accessor: an alias for `Reg<DMA_OUT_PUSH_SPEC>`"]
pub type DMA_OUT_PUSH = crate::Reg<dma_out_push::DMA_OUT_PUSH_SPEC>;
#[doc = "Push control register of TX FIFO"]
pub mod dma_out_push;
#[doc = "DMA_IN_STATUS (r) register accessor: an alias for `Reg<DMA_IN_STATUS_SPEC>`"]
pub type DMA_IN_STATUS = crate::Reg<dma_in_status::DMA_IN_STATUS_SPEC>;
#[doc = "UHCI data-input status register"]
pub mod dma_in_status;
#[doc = "DMA_IN_POP (rw) register accessor: an alias for `Reg<DMA_IN_POP_SPEC>`"]
pub type DMA_IN_POP = crate::Reg<dma_in_pop::DMA_IN_POP_SPEC>;
#[doc = "Pop control register of RX FIFO"]
pub mod dma_in_pop;
#[doc = "DMA_OUT_LINK (rw) register accessor: an alias for `Reg<DMA_OUT_LINK_SPEC>`"]
pub type DMA_OUT_LINK = crate::Reg<dma_out_link::DMA_OUT_LINK_SPEC>;
#[doc = "Link descriptor address and control"]
pub mod dma_out_link;
#[doc = "DMA_IN_LINK (rw) register accessor: an alias for `Reg<DMA_IN_LINK_SPEC>`"]
pub type DMA_IN_LINK = crate::Reg<dma_in_link::DMA_IN_LINK_SPEC>;
#[doc = "Link descriptor address and control"]
pub mod dma_in_link;
#[doc = "CONF1 (rw) register accessor: an alias for `Reg<CONF1_SPEC>`"]
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
#[doc = "UHCI configuration register"]
pub mod conf1;
#[doc = "STATE0 (r) register accessor: an alias for `Reg<STATE0_SPEC>`"]
pub type STATE0 = crate::Reg<state0::STATE0_SPEC>;
#[doc = "UHCI decoder status register"]
pub mod state0;
#[doc = "STATE1 (r) register accessor: an alias for `Reg<STATE1_SPEC>`"]
pub type STATE1 = crate::Reg<state1::STATE1_SPEC>;
#[doc = "UHCI encoder status register"]
pub mod state1;
#[doc = "DMA_OUT_EOF_DES_ADDR (r) register accessor: an alias for `Reg<DMA_OUT_EOF_DES_ADDR_SPEC>`"]
pub type DMA_OUT_EOF_DES_ADDR = crate::Reg<dma_out_eof_des_addr::DMA_OUT_EOF_DES_ADDR_SPEC>;
#[doc = "Outlink descriptor address when EOF occurs"]
pub mod dma_out_eof_des_addr;
#[doc = "DMA_IN_SUC_EOF_DES_ADDR (r) register accessor: an alias for `Reg<DMA_IN_SUC_EOF_DES_ADDR_SPEC>`"]
pub type DMA_IN_SUC_EOF_DES_ADDR =
    crate::Reg<dma_in_suc_eof_des_addr::DMA_IN_SUC_EOF_DES_ADDR_SPEC>;
#[doc = "Inlink descriptor address when EOF occurs"]
pub mod dma_in_suc_eof_des_addr;
#[doc = "DMA_IN_ERR_EOF_DES_ADDR (r) register accessor: an alias for `Reg<DMA_IN_ERR_EOF_DES_ADDR_SPEC>`"]
pub type DMA_IN_ERR_EOF_DES_ADDR =
    crate::Reg<dma_in_err_eof_des_addr::DMA_IN_ERR_EOF_DES_ADDR_SPEC>;
#[doc = "Inlink descriptor address when errors occur"]
pub mod dma_in_err_eof_des_addr;
#[doc = "DMA_OUT_EOF_BFR_DES_ADDR (r) register accessor: an alias for `Reg<DMA_OUT_EOF_BFR_DES_ADDR_SPEC>`"]
pub type DMA_OUT_EOF_BFR_DES_ADDR =
    crate::Reg<dma_out_eof_bfr_des_addr::DMA_OUT_EOF_BFR_DES_ADDR_SPEC>;
#[doc = "Outlink descriptor address before the last transmit descriptor"]
pub mod dma_out_eof_bfr_des_addr;
#[doc = "AHB_TEST (rw) register accessor: an alias for `Reg<AHB_TEST_SPEC>`"]
pub type AHB_TEST = crate::Reg<ahb_test::AHB_TEST_SPEC>;
#[doc = "AHB test register"]
pub mod ahb_test;
#[doc = "DMA_IN_DSCR (r) register accessor: an alias for `Reg<DMA_IN_DSCR_SPEC>`"]
pub type DMA_IN_DSCR = crate::Reg<dma_in_dscr::DMA_IN_DSCR_SPEC>;
#[doc = "The third word of the next receive descriptor"]
pub mod dma_in_dscr;
#[doc = "DMA_IN_DSCR_BF0 (r) register accessor: an alias for `Reg<DMA_IN_DSCR_BF0_SPEC>`"]
pub type DMA_IN_DSCR_BF0 = crate::Reg<dma_in_dscr_bf0::DMA_IN_DSCR_BF0_SPEC>;
#[doc = "The third word of current receive descriptor"]
pub mod dma_in_dscr_bf0;
#[doc = "DMA_OUT_DSCR (r) register accessor: an alias for `Reg<DMA_OUT_DSCR_SPEC>`"]
pub type DMA_OUT_DSCR = crate::Reg<dma_out_dscr::DMA_OUT_DSCR_SPEC>;
#[doc = "The third word of the next transmit descriptor"]
pub mod dma_out_dscr;
#[doc = "DMA_OUT_DSCR_BF0 (r) register accessor: an alias for `Reg<DMA_OUT_DSCR_BF0_SPEC>`"]
pub type DMA_OUT_DSCR_BF0 = crate::Reg<dma_out_dscr_bf0::DMA_OUT_DSCR_BF0_SPEC>;
#[doc = "The third word of current transmit descriptor"]
pub mod dma_out_dscr_bf0;
#[doc = "ESCAPE_CONF (rw) register accessor: an alias for `Reg<ESCAPE_CONF_SPEC>`"]
pub type ESCAPE_CONF = crate::Reg<escape_conf::ESCAPE_CONF_SPEC>;
#[doc = "Escape character configuration"]
pub mod escape_conf;
#[doc = "HUNG_CONF (rw) register accessor: an alias for `Reg<HUNG_CONF_SPEC>`"]
pub type HUNG_CONF = crate::Reg<hung_conf::HUNG_CONF_SPEC>;
#[doc = "Timeout configuration"]
pub mod hung_conf;
#[doc = "RX_HEAD (r) register accessor: an alias for `Reg<RX_HEAD_SPEC>`"]
pub type RX_HEAD = crate::Reg<rx_head::RX_HEAD_SPEC>;
#[doc = "UHCI packet header register"]
pub mod rx_head;
#[doc = "QUICK_SENT (rw) register accessor: an alias for `Reg<QUICK_SENT_SPEC>`"]
pub type QUICK_SENT = crate::Reg<quick_sent::QUICK_SENT_SPEC>;
#[doc = "UHCI quick_sent configuration register"]
pub mod quick_sent;
#[doc = "Q0_WORD0 (rw) register accessor: an alias for `Reg<Q0_WORD0_SPEC>`"]
pub type Q0_WORD0 = crate::Reg<q0_word0::Q0_WORD0_SPEC>;
#[doc = "Q0_WORD0 quick_sent register"]
pub mod q0_word0;
#[doc = "Q0_WORD1 (rw) register accessor: an alias for `Reg<Q0_WORD1_SPEC>`"]
pub type Q0_WORD1 = crate::Reg<q0_word1::Q0_WORD1_SPEC>;
#[doc = "Q0_WORD1 quick_sent register"]
pub mod q0_word1;
#[doc = "Q1_WORD0 (rw) register accessor: an alias for `Reg<Q1_WORD0_SPEC>`"]
pub type Q1_WORD0 = crate::Reg<q1_word0::Q1_WORD0_SPEC>;
#[doc = "Q1_WORD0 quick_sent register"]
pub mod q1_word0;
#[doc = "Q1_WORD1 (rw) register accessor: an alias for `Reg<Q1_WORD1_SPEC>`"]
pub type Q1_WORD1 = crate::Reg<q1_word1::Q1_WORD1_SPEC>;
#[doc = "Q1_WORD1 quick_sent register"]
pub mod q1_word1;
#[doc = "Q2_WORD0 (rw) register accessor: an alias for `Reg<Q2_WORD0_SPEC>`"]
pub type Q2_WORD0 = crate::Reg<q2_word0::Q2_WORD0_SPEC>;
#[doc = "Q2_WORD0 quick_sent register"]
pub mod q2_word0;
#[doc = "Q2_WORD1 (rw) register accessor: an alias for `Reg<Q2_WORD1_SPEC>`"]
pub type Q2_WORD1 = crate::Reg<q2_word1::Q2_WORD1_SPEC>;
#[doc = "Q2_WORD1 quick_sent register"]
pub mod q2_word1;
#[doc = "Q3_WORD0 (rw) register accessor: an alias for `Reg<Q3_WORD0_SPEC>`"]
pub type Q3_WORD0 = crate::Reg<q3_word0::Q3_WORD0_SPEC>;
#[doc = "Q3_WORD0 quick_sent register"]
pub mod q3_word0;
#[doc = "Q3_WORD1 (rw) register accessor: an alias for `Reg<Q3_WORD1_SPEC>`"]
pub type Q3_WORD1 = crate::Reg<q3_word1::Q3_WORD1_SPEC>;
#[doc = "Q3_WORD1 quick_sent register"]
pub mod q3_word1;
#[doc = "Q4_WORD0 (rw) register accessor: an alias for `Reg<Q4_WORD0_SPEC>`"]
pub type Q4_WORD0 = crate::Reg<q4_word0::Q4_WORD0_SPEC>;
#[doc = "Q4_WORD0 quick_sent register"]
pub mod q4_word0;
#[doc = "Q4_WORD1 (rw) register accessor: an alias for `Reg<Q4_WORD1_SPEC>`"]
pub type Q4_WORD1 = crate::Reg<q4_word1::Q4_WORD1_SPEC>;
#[doc = "Q4_WORD1 quick_sent register"]
pub mod q4_word1;
#[doc = "Q5_WORD0 (rw) register accessor: an alias for `Reg<Q5_WORD0_SPEC>`"]
pub type Q5_WORD0 = crate::Reg<q5_word0::Q5_WORD0_SPEC>;
#[doc = "Q5_WORD0 quick_sent register"]
pub mod q5_word0;
#[doc = "Q5_WORD1 (rw) register accessor: an alias for `Reg<Q5_WORD1_SPEC>`"]
pub type Q5_WORD1 = crate::Reg<q5_word1::Q5_WORD1_SPEC>;
#[doc = "Q5_WORD1 quick_sent register"]
pub mod q5_word1;
#[doc = "Q6_WORD0 (rw) register accessor: an alias for `Reg<Q6_WORD0_SPEC>`"]
pub type Q6_WORD0 = crate::Reg<q6_word0::Q6_WORD0_SPEC>;
#[doc = "Q6_WORD0 quick_sent register"]
pub mod q6_word0;
#[doc = "Q6_WORD1 (rw) register accessor: an alias for `Reg<Q6_WORD1_SPEC>`"]
pub type Q6_WORD1 = crate::Reg<q6_word1::Q6_WORD1_SPEC>;
#[doc = "Q6_WORD1 quick_sent register"]
pub mod q6_word1;
#[doc = "ESC_CONF0 (rw) register accessor: an alias for `Reg<ESC_CONF0_SPEC>`"]
pub type ESC_CONF0 = crate::Reg<esc_conf0::ESC_CONF0_SPEC>;
#[doc = "Escape sequence configuration register 0"]
pub mod esc_conf0;
#[doc = "ESC_CONF1 (rw) register accessor: an alias for `Reg<ESC_CONF1_SPEC>`"]
pub type ESC_CONF1 = crate::Reg<esc_conf1::ESC_CONF1_SPEC>;
#[doc = "Escape sequence configuration register 1"]
pub mod esc_conf1;
#[doc = "ESC_CONF2 (rw) register accessor: an alias for `Reg<ESC_CONF2_SPEC>`"]
pub type ESC_CONF2 = crate::Reg<esc_conf2::ESC_CONF2_SPEC>;
#[doc = "Escape sequence configuration register 2"]
pub mod esc_conf2;
#[doc = "ESC_CONF3 (rw) register accessor: an alias for `Reg<ESC_CONF3_SPEC>`"]
pub type ESC_CONF3 = crate::Reg<esc_conf3::ESC_CONF3_SPEC>;
#[doc = "Escape sequence configuration register 3"]
pub mod esc_conf3;
#[doc = "PKT_THRES (rw) register accessor: an alias for `Reg<PKT_THRES_SPEC>`"]
pub type PKT_THRES = crate::Reg<pkt_thres::PKT_THRES_SPEC>;
#[doc = "Configure register for packet length"]
pub mod pkt_thres;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "UHCI version control register"]
pub mod date;
