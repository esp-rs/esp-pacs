#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Raw status interrupt of channel 0"]
    pub in_int_raw_ch0: IN_INT_RAW_CH,
    #[doc = "0x04 - Masked interrupt of channel 0"]
    pub in_int_st_ch0: IN_INT_ST_CH,
    #[doc = "0x08 - Interrupt enable bits of channel 0"]
    pub in_int_ena_ch0: IN_INT_ENA_CH,
    #[doc = "0x0c - Interrupt clear bits of channel 0"]
    pub in_int_clr_ch0: IN_INT_CLR_CH,
    #[doc = "0x10 - Raw status interrupt of channel 0"]
    pub in_int_raw_ch1: IN_INT_RAW_CH,
    #[doc = "0x14 - Masked interrupt of channel 0"]
    pub in_int_st_ch1: IN_INT_ST_CH,
    #[doc = "0x18 - Interrupt enable bits of channel 0"]
    pub in_int_ena_ch1: IN_INT_ENA_CH,
    #[doc = "0x1c - Interrupt clear bits of channel 0"]
    pub in_int_clr_ch1: IN_INT_CLR_CH,
    #[doc = "0x20 - Raw status interrupt of channel 0"]
    pub in_int_raw_ch2: IN_INT_RAW_CH,
    #[doc = "0x24 - Masked interrupt of channel 0"]
    pub in_int_st_ch2: IN_INT_ST_CH,
    #[doc = "0x28 - Interrupt enable bits of channel 0"]
    pub in_int_ena_ch2: IN_INT_ENA_CH,
    #[doc = "0x2c - Interrupt clear bits of channel 0"]
    pub in_int_clr_ch2: IN_INT_CLR_CH,
    #[doc = "0x30 - Raw status interrupt of channel 0"]
    pub out_int_raw_ch0: OUT_INT_RAW_CH,
    #[doc = "0x34 - Masked interrupt of channel 0"]
    pub out_int_st_ch0: OUT_INT_ST_CH,
    #[doc = "0x38 - Interrupt enable bits of channel 0"]
    pub out_int_ena_ch0: OUT_INT_ENA_CH,
    #[doc = "0x3c - Interrupt clear bits of channel 0"]
    pub out_int_clr_ch0: OUT_INT_CLR_CH,
    #[doc = "0x40 - Raw status interrupt of channel 0"]
    pub out_int_raw_ch1: OUT_INT_RAW_CH,
    #[doc = "0x44 - Masked interrupt of channel 0"]
    pub out_int_st_ch1: OUT_INT_ST_CH,
    #[doc = "0x48 - Interrupt enable bits of channel 0"]
    pub out_int_ena_ch1: OUT_INT_ENA_CH,
    #[doc = "0x4c - Interrupt clear bits of channel 0"]
    pub out_int_clr_ch1: OUT_INT_CLR_CH,
    #[doc = "0x50 - Raw status interrupt of channel 0"]
    pub out_int_raw_ch2: OUT_INT_RAW_CH,
    #[doc = "0x54 - Masked interrupt of channel 0"]
    pub out_int_st_ch2: OUT_INT_ST_CH,
    #[doc = "0x58 - Interrupt enable bits of channel 0"]
    pub out_int_ena_ch2: OUT_INT_ENA_CH,
    #[doc = "0x5c - Interrupt clear bits of channel 0"]
    pub out_int_clr_ch2: OUT_INT_CLR_CH,
    #[doc = "0x60 - reserved"]
    pub ahb_test: AHB_TEST,
    #[doc = "0x64 - MISC register"]
    pub misc_conf: MISC_CONF,
    #[doc = "0x68 - Version control register"]
    pub date: DATE,
    _reserved27: [u8; 0x04],
    #[doc = "0x70 - Configure 0 register of Rx channel 0"]
    pub in_conf0_ch0: IN_CONF0_CH,
    #[doc = "0x74 - Configure 1 register of Rx channel 0"]
    pub in_conf1_ch0: IN_CONF1_CH,
    #[doc = "0x78 - Receive FIFO status of Rx channel 0"]
    pub infifo_status_ch0: INFIFO_STATUS_CH,
    #[doc = "0x7c - Pop control register of Rx channel 0"]
    pub in_pop_ch0: IN_POP_CH,
    #[doc = "0x80 - Link descriptor configure and control register of Rx channel 0"]
    pub in_link_ch0: IN_LINK_CH,
    #[doc = "0x84 - Receive status of Rx channel 0"]
    pub in_state_ch0: IN_STATE_CH,
    #[doc = "0x88 - Inlink descriptor address when EOF occurs of Rx channel 0"]
    pub in_suc_eof_des_addr_ch0: IN_SUC_EOF_DES_ADDR_CH,
    #[doc = "0x8c - Inlink descriptor address when errors occur of Rx channel 0"]
    pub in_err_eof_des_addr_ch0: IN_ERR_EOF_DES_ADDR_CH,
    #[doc = "0x90 - Current inlink descriptor address of Rx channel 0"]
    pub in_dscr_ch0: IN_DSCR_CH,
    #[doc = "0x94 - The last inlink descriptor address of Rx channel 0"]
    pub in_dscr_bf0_ch0: IN_DSCR_BF0_CH,
    #[doc = "0x98 - The second-to-last inlink descriptor address of Rx channel 0"]
    pub in_dscr_bf1_ch0: IN_DSCR_BF1_CH,
    #[doc = "0x9c - Priority register of Rx channel 0"]
    pub in_pri_ch0: IN_PRI_CH,
    #[doc = "0xa0 - Peripheral selection of Rx channel 0"]
    pub in_peri_sel_ch0: IN_PERI_SEL_CH,
    _reserved40: [u8; 0x30],
    #[doc = "0xd4 - Configure 1 register of Tx channel 0"]
    pub out_conf1_ch0: OUT_CONF1_CH,
    #[doc = "0xd8 - Transmit FIFO status of Tx channel 0"]
    pub outfifo_status_ch0: OUTFIFO_STATUS_CH,
    #[doc = "0xdc - Push control register of Rx channel 0"]
    pub out_push_ch0: OUT_PUSH_CH,
    #[doc = "0xe0 - Link descriptor configure and control register of Tx channel 0"]
    pub out_link_ch0: OUT_LINK_CH,
    #[doc = "0xe4 - Transmit status of Tx channel 0"]
    pub out_state_ch0: OUT_STATE_CH,
    #[doc = "0xe8 - Outlink descriptor address when EOF occurs of Tx channel 0"]
    pub out_eof_des_addr_ch0: OUT_EOF_DES_ADDR_CH,
    #[doc = "0xec - The last outlink descriptor address when EOF occurs of Tx channel 0"]
    pub out_eof_bfr_des_addr_ch0: OUT_EOF_BFR_DES_ADDR_CH,
    #[doc = "0xf0 - Current inlink descriptor address of Tx channel 0"]
    pub out_dscr_ch0: OUT_DSCR_CH,
    #[doc = "0xf4 - The last inlink descriptor address of Tx channel 0"]
    pub out_dscr_bf0_ch0: OUT_DSCR_BF0_CH,
    #[doc = "0xf8 - The second-to-last inlink descriptor address of Tx channel 0"]
    pub out_dscr_bf1_ch0: OUT_DSCR_BF1_CH,
    #[doc = "0xfc - Priority register of Tx channel 0."]
    pub out_pri_ch0: OUT_PRI_CH,
    #[doc = "0x100 - Peripheral selection of Tx channel 0"]
    pub out_peri_sel_ch0: OUT_PERI_SEL_CH,
    _reserved52: [u8; 0x2c],
    #[doc = "0x130 - Configure 0 register of Rx channel 0"]
    pub in_conf0_ch1: IN_CONF0_CH,
    #[doc = "0x134 - Configure 1 register of Rx channel 0"]
    pub in_conf1_ch1: IN_CONF1_CH,
    #[doc = "0x138 - Receive FIFO status of Rx channel 0"]
    pub infifo_status_ch1: INFIFO_STATUS_CH,
    #[doc = "0x13c - Pop control register of Rx channel 0"]
    pub in_pop_ch1: IN_POP_CH,
    #[doc = "0x140 - Link descriptor configure and control register of Rx channel 0"]
    pub in_link_ch1: IN_LINK_CH,
    #[doc = "0x144 - Receive status of Rx channel 0"]
    pub in_state_ch1: IN_STATE_CH,
    #[doc = "0x148 - Inlink descriptor address when EOF occurs of Rx channel 0"]
    pub in_suc_eof_des_addr_ch1: IN_SUC_EOF_DES_ADDR_CH,
    #[doc = "0x14c - Inlink descriptor address when errors occur of Rx channel 0"]
    pub in_err_eof_des_addr_ch1: IN_ERR_EOF_DES_ADDR_CH,
    #[doc = "0x150 - Current inlink descriptor address of Rx channel 0"]
    pub in_dscr_ch1: IN_DSCR_CH,
    #[doc = "0x154 - The last inlink descriptor address of Rx channel 0"]
    pub in_dscr_bf0_ch1: IN_DSCR_BF0_CH,
    #[doc = "0x158 - The second-to-last inlink descriptor address of Rx channel 0"]
    pub in_dscr_bf1_ch1: IN_DSCR_BF1_CH,
    #[doc = "0x15c - Priority register of Rx channel 0"]
    pub in_pri_ch1: IN_PRI_CH,
    #[doc = "0x160 - Peripheral selection of Rx channel 0"]
    pub in_peri_sel_ch1: IN_PERI_SEL_CH,
    _reserved65: [u8; 0x2c],
    #[doc = "0x190 - Configure 0 register of Tx channel 1"]
    pub out_conf0_ch0: OUT_CONF0_CH,
    #[doc = "0x194 - Configure 1 register of Tx channel 0"]
    pub out_conf1_ch1: OUT_CONF1_CH,
    #[doc = "0x198 - Transmit FIFO status of Tx channel 0"]
    pub outfifo_status_ch1: OUTFIFO_STATUS_CH,
    #[doc = "0x19c - Push control register of Rx channel 0"]
    pub out_push_ch1: OUT_PUSH_CH,
    #[doc = "0x1a0 - Link descriptor configure and control register of Tx channel 0"]
    pub out_link_ch1: OUT_LINK_CH,
    #[doc = "0x1a4 - Transmit status of Tx channel 0"]
    pub out_state_ch1: OUT_STATE_CH,
    #[doc = "0x1a8 - Outlink descriptor address when EOF occurs of Tx channel 0"]
    pub out_eof_des_addr_ch1: OUT_EOF_DES_ADDR_CH,
    #[doc = "0x1ac - The last outlink descriptor address when EOF occurs of Tx channel 0"]
    pub out_eof_bfr_des_addr_ch1: OUT_EOF_BFR_DES_ADDR_CH,
    #[doc = "0x1b0 - Current inlink descriptor address of Tx channel 0"]
    pub out_dscr_ch1: OUT_DSCR_CH,
    #[doc = "0x1b4 - The last inlink descriptor address of Tx channel 0"]
    pub out_dscr_bf0_ch1: OUT_DSCR_BF0_CH,
    #[doc = "0x1b8 - The second-to-last inlink descriptor address of Tx channel 0"]
    pub out_dscr_bf1_ch1: OUT_DSCR_BF1_CH,
    #[doc = "0x1bc - Priority register of Tx channel 0."]
    pub out_pri_ch1: OUT_PRI_CH,
    #[doc = "0x1c0 - Peripheral selection of Tx channel 0"]
    pub out_peri_sel_ch1: OUT_PERI_SEL_CH,
    _reserved78: [u8; 0x2c],
    #[doc = "0x1f0 - Configure 0 register of Rx channel 0"]
    pub in_conf0_ch2: IN_CONF0_CH,
    #[doc = "0x1f4 - Configure 1 register of Rx channel 0"]
    pub in_conf1_ch2: IN_CONF1_CH,
    #[doc = "0x1f8 - Receive FIFO status of Rx channel 0"]
    pub infifo_status_ch2: INFIFO_STATUS_CH,
    #[doc = "0x1fc - Pop control register of Rx channel 0"]
    pub in_pop_ch2: IN_POP_CH,
    #[doc = "0x200 - Link descriptor configure and control register of Rx channel 0"]
    pub in_link_ch2: IN_LINK_CH,
    #[doc = "0x204 - Receive status of Rx channel 0"]
    pub in_state_ch2: IN_STATE_CH,
    #[doc = "0x208 - Inlink descriptor address when EOF occurs of Rx channel 0"]
    pub in_suc_eof_des_addr_ch2: IN_SUC_EOF_DES_ADDR_CH,
    #[doc = "0x20c - Inlink descriptor address when errors occur of Rx channel 0"]
    pub in_err_eof_des_addr_ch2: IN_ERR_EOF_DES_ADDR_CH,
    #[doc = "0x210 - Current inlink descriptor address of Rx channel 0"]
    pub in_dscr_ch2: IN_DSCR_CH,
    #[doc = "0x214 - The last inlink descriptor address of Rx channel 0"]
    pub in_dscr_bf0_ch2: IN_DSCR_BF0_CH,
    #[doc = "0x218 - The second-to-last inlink descriptor address of Rx channel 0"]
    pub in_dscr_bf1_ch2: IN_DSCR_BF1_CH,
    #[doc = "0x21c - Priority register of Rx channel 0"]
    pub in_pri_ch2: IN_PRI_CH,
    #[doc = "0x220 - Peripheral selection of Rx channel 0"]
    pub in_peri_sel_ch2: IN_PERI_SEL_CH,
    _reserved91: [u8; 0x2c],
    #[doc = "0x250 - Configure 0 register of Tx channel 1"]
    pub out_conf0_ch1: OUT_CONF0_CH,
    #[doc = "0x254 - Configure 1 register of Tx channel 0"]
    pub out_conf1_ch2: OUT_CONF1_CH,
    #[doc = "0x258 - Transmit FIFO status of Tx channel 0"]
    pub outfifo_status_ch2: OUTFIFO_STATUS_CH,
    #[doc = "0x25c - Push control register of Rx channel 0"]
    pub out_push_ch2: OUT_PUSH_CH,
    #[doc = "0x260 - Link descriptor configure and control register of Tx channel 0"]
    pub out_link_ch2: OUT_LINK_CH,
    #[doc = "0x264 - Transmit status of Tx channel 0"]
    pub out_state_ch2: OUT_STATE_CH,
    #[doc = "0x268 - Outlink descriptor address when EOF occurs of Tx channel 0"]
    pub out_eof_des_addr_ch2: OUT_EOF_DES_ADDR_CH,
    #[doc = "0x26c - The last outlink descriptor address when EOF occurs of Tx channel 0"]
    pub out_eof_bfr_des_addr_ch2: OUT_EOF_BFR_DES_ADDR_CH,
    #[doc = "0x270 - Current inlink descriptor address of Tx channel 0"]
    pub out_dscr_ch2: OUT_DSCR_CH,
    #[doc = "0x274 - The last inlink descriptor address of Tx channel 0"]
    pub out_dscr_bf0_ch2: OUT_DSCR_BF0_CH,
    #[doc = "0x278 - The second-to-last inlink descriptor address of Tx channel 0"]
    pub out_dscr_bf1_ch2: OUT_DSCR_BF1_CH,
    #[doc = "0x27c - Priority register of Tx channel 0."]
    pub out_pri_ch2: OUT_PRI_CH,
    #[doc = "0x280 - Peripheral selection of Tx channel 0"]
    pub out_peri_sel_ch2: OUT_PERI_SEL_CH,
    _reserved104: [u8; 0x8c],
    #[doc = "0x310 - Configure 0 register of Tx channel 1"]
    pub out_conf0_ch2: OUT_CONF0_CH,
}
#[doc = "IN_INT_RAW_CH (rw) register accessor: an alias for `Reg<IN_INT_RAW_CH_SPEC>`"]
pub type IN_INT_RAW_CH = crate::Reg<in_int_raw_ch::IN_INT_RAW_CH_SPEC>;
#[doc = "Raw status interrupt of channel 0"]
pub mod in_int_raw_ch;
#[doc = "IN_INT_ST_CH (r) register accessor: an alias for `Reg<IN_INT_ST_CH_SPEC>`"]
pub type IN_INT_ST_CH = crate::Reg<in_int_st_ch::IN_INT_ST_CH_SPEC>;
#[doc = "Masked interrupt of channel 0"]
pub mod in_int_st_ch;
#[doc = "IN_INT_ENA_CH (rw) register accessor: an alias for `Reg<IN_INT_ENA_CH_SPEC>`"]
pub type IN_INT_ENA_CH = crate::Reg<in_int_ena_ch::IN_INT_ENA_CH_SPEC>;
#[doc = "Interrupt enable bits of channel 0"]
pub mod in_int_ena_ch;
#[doc = "IN_INT_CLR_CH (w) register accessor: an alias for `Reg<IN_INT_CLR_CH_SPEC>`"]
pub type IN_INT_CLR_CH = crate::Reg<in_int_clr_ch::IN_INT_CLR_CH_SPEC>;
#[doc = "Interrupt clear bits of channel 0"]
pub mod in_int_clr_ch;
#[doc = "OUT_INT_RAW_CH (rw) register accessor: an alias for `Reg<OUT_INT_RAW_CH_SPEC>`"]
pub type OUT_INT_RAW_CH = crate::Reg<out_int_raw_ch::OUT_INT_RAW_CH_SPEC>;
#[doc = "Raw status interrupt of channel 0"]
pub mod out_int_raw_ch;
#[doc = "OUT_INT_ST_CH (r) register accessor: an alias for `Reg<OUT_INT_ST_CH_SPEC>`"]
pub type OUT_INT_ST_CH = crate::Reg<out_int_st_ch::OUT_INT_ST_CH_SPEC>;
#[doc = "Masked interrupt of channel 0"]
pub mod out_int_st_ch;
#[doc = "OUT_INT_ENA_CH (rw) register accessor: an alias for `Reg<OUT_INT_ENA_CH_SPEC>`"]
pub type OUT_INT_ENA_CH = crate::Reg<out_int_ena_ch::OUT_INT_ENA_CH_SPEC>;
#[doc = "Interrupt enable bits of channel 0"]
pub mod out_int_ena_ch;
#[doc = "OUT_INT_CLR_CH (w) register accessor: an alias for `Reg<OUT_INT_CLR_CH_SPEC>`"]
pub type OUT_INT_CLR_CH = crate::Reg<out_int_clr_ch::OUT_INT_CLR_CH_SPEC>;
#[doc = "Interrupt clear bits of channel 0"]
pub mod out_int_clr_ch;
#[doc = "AHB_TEST (rw) register accessor: an alias for `Reg<AHB_TEST_SPEC>`"]
pub type AHB_TEST = crate::Reg<ahb_test::AHB_TEST_SPEC>;
#[doc = "reserved"]
pub mod ahb_test;
#[doc = "MISC_CONF (rw) register accessor: an alias for `Reg<MISC_CONF_SPEC>`"]
pub type MISC_CONF = crate::Reg<misc_conf::MISC_CONF_SPEC>;
#[doc = "MISC register"]
pub mod misc_conf;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
#[doc = "IN_CONF0_CH (rw) register accessor: an alias for `Reg<IN_CONF0_CH_SPEC>`"]
pub type IN_CONF0_CH = crate::Reg<in_conf0_ch::IN_CONF0_CH_SPEC>;
#[doc = "Configure 0 register of Rx channel 0"]
pub mod in_conf0_ch;
#[doc = "IN_CONF1_CH (rw) register accessor: an alias for `Reg<IN_CONF1_CH_SPEC>`"]
pub type IN_CONF1_CH = crate::Reg<in_conf1_ch::IN_CONF1_CH_SPEC>;
#[doc = "Configure 1 register of Rx channel 0"]
pub mod in_conf1_ch;
#[doc = "INFIFO_STATUS_CH (r) register accessor: an alias for `Reg<INFIFO_STATUS_CH_SPEC>`"]
pub type INFIFO_STATUS_CH = crate::Reg<infifo_status_ch::INFIFO_STATUS_CH_SPEC>;
#[doc = "Receive FIFO status of Rx channel 0"]
pub mod infifo_status_ch;
#[doc = "IN_POP_CH (rw) register accessor: an alias for `Reg<IN_POP_CH_SPEC>`"]
pub type IN_POP_CH = crate::Reg<in_pop_ch::IN_POP_CH_SPEC>;
#[doc = "Pop control register of Rx channel 0"]
pub mod in_pop_ch;
#[doc = "IN_LINK_CH (rw) register accessor: an alias for `Reg<IN_LINK_CH_SPEC>`"]
pub type IN_LINK_CH = crate::Reg<in_link_ch::IN_LINK_CH_SPEC>;
#[doc = "Link descriptor configure and control register of Rx channel 0"]
pub mod in_link_ch;
#[doc = "IN_STATE_CH (r) register accessor: an alias for `Reg<IN_STATE_CH_SPEC>`"]
pub type IN_STATE_CH = crate::Reg<in_state_ch::IN_STATE_CH_SPEC>;
#[doc = "Receive status of Rx channel 0"]
pub mod in_state_ch;
#[doc = "IN_SUC_EOF_DES_ADDR_CH (r) register accessor: an alias for `Reg<IN_SUC_EOF_DES_ADDR_CH_SPEC>`"]
pub type IN_SUC_EOF_DES_ADDR_CH = crate::Reg<in_suc_eof_des_addr_ch::IN_SUC_EOF_DES_ADDR_CH_SPEC>;
#[doc = "Inlink descriptor address when EOF occurs of Rx channel 0"]
pub mod in_suc_eof_des_addr_ch;
#[doc = "IN_ERR_EOF_DES_ADDR_CH (r) register accessor: an alias for `Reg<IN_ERR_EOF_DES_ADDR_CH_SPEC>`"]
pub type IN_ERR_EOF_DES_ADDR_CH = crate::Reg<in_err_eof_des_addr_ch::IN_ERR_EOF_DES_ADDR_CH_SPEC>;
#[doc = "Inlink descriptor address when errors occur of Rx channel 0"]
pub mod in_err_eof_des_addr_ch;
#[doc = "IN_DSCR_CH (r) register accessor: an alias for `Reg<IN_DSCR_CH_SPEC>`"]
pub type IN_DSCR_CH = crate::Reg<in_dscr_ch::IN_DSCR_CH_SPEC>;
#[doc = "Current inlink descriptor address of Rx channel 0"]
pub mod in_dscr_ch;
#[doc = "IN_DSCR_BF0_CH (r) register accessor: an alias for `Reg<IN_DSCR_BF0_CH_SPEC>`"]
pub type IN_DSCR_BF0_CH = crate::Reg<in_dscr_bf0_ch::IN_DSCR_BF0_CH_SPEC>;
#[doc = "The last inlink descriptor address of Rx channel 0"]
pub mod in_dscr_bf0_ch;
#[doc = "IN_DSCR_BF1_CH (r) register accessor: an alias for `Reg<IN_DSCR_BF1_CH_SPEC>`"]
pub type IN_DSCR_BF1_CH = crate::Reg<in_dscr_bf1_ch::IN_DSCR_BF1_CH_SPEC>;
#[doc = "The second-to-last inlink descriptor address of Rx channel 0"]
pub mod in_dscr_bf1_ch;
#[doc = "IN_PRI_CH (rw) register accessor: an alias for `Reg<IN_PRI_CH_SPEC>`"]
pub type IN_PRI_CH = crate::Reg<in_pri_ch::IN_PRI_CH_SPEC>;
#[doc = "Priority register of Rx channel 0"]
pub mod in_pri_ch;
#[doc = "IN_PERI_SEL_CH (rw) register accessor: an alias for `Reg<IN_PERI_SEL_CH_SPEC>`"]
pub type IN_PERI_SEL_CH = crate::Reg<in_peri_sel_ch::IN_PERI_SEL_CH_SPEC>;
#[doc = "Peripheral selection of Rx channel 0"]
pub mod in_peri_sel_ch;
#[doc = "OUT_CONF1_CH (rw) register accessor: an alias for `Reg<OUT_CONF1_CH_SPEC>`"]
pub type OUT_CONF1_CH = crate::Reg<out_conf1_ch::OUT_CONF1_CH_SPEC>;
#[doc = "Configure 1 register of Tx channel 0"]
pub mod out_conf1_ch;
#[doc = "OUTFIFO_STATUS_CH (r) register accessor: an alias for `Reg<OUTFIFO_STATUS_CH_SPEC>`"]
pub type OUTFIFO_STATUS_CH = crate::Reg<outfifo_status_ch::OUTFIFO_STATUS_CH_SPEC>;
#[doc = "Transmit FIFO status of Tx channel 0"]
pub mod outfifo_status_ch;
#[doc = "OUT_PUSH_CH (rw) register accessor: an alias for `Reg<OUT_PUSH_CH_SPEC>`"]
pub type OUT_PUSH_CH = crate::Reg<out_push_ch::OUT_PUSH_CH_SPEC>;
#[doc = "Push control register of Rx channel 0"]
pub mod out_push_ch;
#[doc = "OUT_LINK_CH (rw) register accessor: an alias for `Reg<OUT_LINK_CH_SPEC>`"]
pub type OUT_LINK_CH = crate::Reg<out_link_ch::OUT_LINK_CH_SPEC>;
#[doc = "Link descriptor configure and control register of Tx channel 0"]
pub mod out_link_ch;
#[doc = "OUT_STATE_CH (r) register accessor: an alias for `Reg<OUT_STATE_CH_SPEC>`"]
pub type OUT_STATE_CH = crate::Reg<out_state_ch::OUT_STATE_CH_SPEC>;
#[doc = "Transmit status of Tx channel 0"]
pub mod out_state_ch;
#[doc = "OUT_EOF_DES_ADDR_CH (r) register accessor: an alias for `Reg<OUT_EOF_DES_ADDR_CH_SPEC>`"]
pub type OUT_EOF_DES_ADDR_CH = crate::Reg<out_eof_des_addr_ch::OUT_EOF_DES_ADDR_CH_SPEC>;
#[doc = "Outlink descriptor address when EOF occurs of Tx channel 0"]
pub mod out_eof_des_addr_ch;
#[doc = "OUT_EOF_BFR_DES_ADDR_CH (r) register accessor: an alias for `Reg<OUT_EOF_BFR_DES_ADDR_CH_SPEC>`"]
pub type OUT_EOF_BFR_DES_ADDR_CH =
    crate::Reg<out_eof_bfr_des_addr_ch::OUT_EOF_BFR_DES_ADDR_CH_SPEC>;
#[doc = "The last outlink descriptor address when EOF occurs of Tx channel 0"]
pub mod out_eof_bfr_des_addr_ch;
#[doc = "OUT_DSCR_CH (r) register accessor: an alias for `Reg<OUT_DSCR_CH_SPEC>`"]
pub type OUT_DSCR_CH = crate::Reg<out_dscr_ch::OUT_DSCR_CH_SPEC>;
#[doc = "Current inlink descriptor address of Tx channel 0"]
pub mod out_dscr_ch;
#[doc = "OUT_DSCR_BF0_CH (r) register accessor: an alias for `Reg<OUT_DSCR_BF0_CH_SPEC>`"]
pub type OUT_DSCR_BF0_CH = crate::Reg<out_dscr_bf0_ch::OUT_DSCR_BF0_CH_SPEC>;
#[doc = "The last inlink descriptor address of Tx channel 0"]
pub mod out_dscr_bf0_ch;
#[doc = "OUT_DSCR_BF1_CH (r) register accessor: an alias for `Reg<OUT_DSCR_BF1_CH_SPEC>`"]
pub type OUT_DSCR_BF1_CH = crate::Reg<out_dscr_bf1_ch::OUT_DSCR_BF1_CH_SPEC>;
#[doc = "The second-to-last inlink descriptor address of Tx channel 0"]
pub mod out_dscr_bf1_ch;
#[doc = "OUT_PRI_CH (rw) register accessor: an alias for `Reg<OUT_PRI_CH_SPEC>`"]
pub type OUT_PRI_CH = crate::Reg<out_pri_ch::OUT_PRI_CH_SPEC>;
#[doc = "Priority register of Tx channel 0."]
pub mod out_pri_ch;
#[doc = "OUT_PERI_SEL_CH (rw) register accessor: an alias for `Reg<OUT_PERI_SEL_CH_SPEC>`"]
pub type OUT_PERI_SEL_CH = crate::Reg<out_peri_sel_ch::OUT_PERI_SEL_CH_SPEC>;
#[doc = "Peripheral selection of Tx channel 0"]
pub mod out_peri_sel_ch;
#[doc = "OUT_CONF0_CH (rw) register accessor: an alias for `Reg<OUT_CONF0_CH_SPEC>`"]
pub type OUT_CONF0_CH = crate::Reg<out_conf0_ch::OUT_CONF0_CH_SPEC>;
#[doc = "Configure 0 register of Tx channel 1"]
pub mod out_conf0_ch;
