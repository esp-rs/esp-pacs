#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    in_int_raw_ch0: IN_INT_RAW_CH0,
    in_int_st_ch0: IN_INT_ST_CH0,
    in_int_ena_ch0: IN_INT_ENA_CH0,
    in_int_clr_ch0: IN_INT_CLR_CH0,
    in_int_raw_ch1: IN_INT_RAW_CH1,
    in_int_st_ch1: IN_INT_ST_CH1,
    in_int_ena_ch1: IN_INT_ENA_CH1,
    in_int_clr_ch1: IN_INT_CLR_CH1,
    _reserved8: [u8; 0x10],
    out_int_raw_ch0: OUT_INT_RAW_CH0,
    out_int_st_ch0: OUT_INT_ST_CH0,
    out_int_ena_ch0: OUT_INT_ENA_CH0,
    out_int_clr_ch0: OUT_INT_CLR_CH0,
    out_int_raw_ch1: OUT_INT_RAW_CH1,
    out_int_st_ch1: OUT_INT_ST_CH1,
    out_int_ena_ch1: OUT_INT_ENA_CH1,
    out_int_clr_ch1: OUT_INT_CLR_CH1,
    _reserved16: [u8; 0x10],
    ahb_test: AHB_TEST,
    misc_conf: MISC_CONF,
    date: DATE,
    _reserved19: [u8; 0x04],
    in_conf0_ch0: IN_CONF0_CH0,
    in_conf1_ch0: IN_CONF1_CH0,
    infifo_status_ch0: INFIFO_STATUS_CH0,
    in_pop_ch0: IN_POP_CH0,
    in_link_ch0: IN_LINK_CH0,
    in_state_ch0: IN_STATE_CH0,
    in_suc_eof_des_addr_ch0: IN_SUC_EOF_DES_ADDR_CH0,
    in_err_eof_des_addr_ch0: IN_ERR_EOF_DES_ADDR_CH0,
    in_dscr_ch0: IN_DSCR_CH0,
    in_dscr_bf0_ch0: IN_DSCR_BF0_CH0,
    in_dscr_bf1_ch0: IN_DSCR_BF1_CH0,
    in_peri_ch0: IN_PERI_CH0,
    in_peri_sel_ch0: IN_PERI_SEL_CH0,
    _reserved32: [u8; 0x0c],
    in_done_des_addr_ch0: IN_DONE_DES_ADDR_CH0,
    _reserved33: [u8; 0x1c],
    out_conf0_ch0: OUT_CONF0_CH0,
    out_conf1_ch0: OUT_CONF1_CH0,
    outfifo_status_ch0: OUTFIFO_STATUS_CH0,
    out_push_ch0: OUT_PUSH_CH0,
    out_link_ch0: OUT_LINK_CH0,
    out_state_ch0: OUT_STATE_CH0,
    out_eof_des_addr_ch0: OUT_EOF_DES_ADDR_CH0,
    out_eof_bfr_des_addr_ch0: OUT_EOF_BFR_DES_ADDR_CH0,
    out_dscr_ch0: OUT_DSCR_CH0,
    out_dscr_bf0_ch0: OUT_DSCR_BF0_CH0,
    out_dscr_bf1_ch0: OUT_DSCR_BF1_CH0,
    out_peri_ch0: OUT_PERI_CH0,
    out_peri_sel_ch0: OUT_PERI_SEL_CH0,
    _reserved46: [u8; 0x0c],
    out_done_des_addr_ch0: OUT_DONE_DES_ADDR_CH0,
    _reserved47: [u8; 0x1c],
    in_conf0_ch1: IN_CONF0_CH1,
    in_conf1_ch1: IN_CONF1_CH1,
    infifo_status_ch1: INFIFO_STATUS_CH1,
    in_pop_ch1: IN_POP_CH1,
    in_link_ch1: IN_LINK_CH1,
    in_state_ch1: IN_STATE_CH1,
    in_suc_eof_des_addr_ch1: IN_SUC_EOF_DES_ADDR_CH1,
    in_err_eof_des_addr_ch1: IN_ERR_EOF_DES_ADDR_CH1,
    in_dscr_ch1: IN_DSCR_CH1,
    in_dscr_bf0_ch1: IN_DSCR_BF0_CH1,
    in_dscr_bf1_ch1: IN_DSCR_BF1_CH1,
    in_peri_ch1: IN_PERI_CH1,
    in_peri_sel_ch1: IN_PERI_SEL_CH1,
    _reserved60: [u8; 0x0c],
    in_done_des_addr_ch1: IN_DONE_DES_ADDR_CH1,
    _reserved61: [u8; 0x1c],
    out_conf0_ch1: OUT_CONF0_CH1,
    out_conf1_ch1: OUT_CONF1_CH1,
    outfifo_status_ch1: OUTFIFO_STATUS_CH1,
    out_push_ch1: OUT_PUSH_CH1,
    out_link_ch1: OUT_LINK_CH1,
    out_state_ch1: OUT_STATE_CH1,
    out_eof_des_addr_ch1: OUT_EOF_DES_ADDR_CH1,
    out_eof_bfr_des_addr_ch1: OUT_EOF_BFR_DES_ADDR_CH1,
    out_dscr_ch1: OUT_DSCR_CH1,
    out_dscr_bf0_ch1: OUT_DSCR_BF0_CH1,
    out_dscr_bf1_ch1: OUT_DSCR_BF1_CH1,
    out_peri_ch1: OUT_PERI_CH1,
    out_peri_sel_ch1: OUT_PERI_SEL_CH1,
    _reserved74: [u8; 0x0c],
    out_done_des_addr_ch1: OUT_DONE_DES_ADDR_CH1,
    _reserved75: [u8; 0x0108],
    tx_ch_arb_weight_ch0: TX_CH_ARB_WEIGHT_CH0,
    tx_arb_weight_opt_dir_ch0: TX_ARB_WEIGHT_OPT_DIR_CH0,
    _reserved77: [u8; 0x20],
    tx_ch_arb_weight_ch1: TX_CH_ARB_WEIGHT_CH1,
    tx_arb_weight_opt_dir_ch1: TX_ARB_WEIGHT_OPT_DIR_CH1,
    _reserved79: [u8; 0x48],
    rx_ch_arb_weight_ch0: RX_CH_ARB_WEIGHT_CH0,
    rx_arb_weight_opt_dir_ch0: RX_ARB_WEIGHT_OPT_DIR_CH0,
    _reserved81: [u8; 0x20],
    rx_ch_arb_weight_ch1: RX_CH_ARB_WEIGHT_CH1,
    rx_arb_weight_opt_dir_ch1: RX_ARB_WEIGHT_OPT_DIR_CH1,
    _reserved83: [u8; 0x28],
    in_link_addr_ch0: IN_LINK_ADDR_CH0,
    in_link_addr_ch1: IN_LINK_ADDR_CH1,
    _reserved85: [u8; 0x04],
    out_link_addr_ch0: OUT_LINK_ADDR_CH0,
    out_link_addr_ch1: OUT_LINK_ADDR_CH1,
    _reserved87: [u8; 0x04],
    intr_mem_start_addr: INTR_MEM_START_ADDR,
    intr_mem_end_addr: INTR_MEM_END_ADDR,
    arb_timeout_tx: ARB_TIMEOUT_TX,
    arb_timeout_rx: ARB_TIMEOUT_RX,
    weight_en_tx: WEIGHT_EN_TX,
    weight_en_rx: WEIGHT_EN_RX,
    arb_timeout: ARB_TIMEOUT,
    _reserved94: [u8; 0x20],
    weight_en: WEIGHT_EN,
    module_clk_en: MODULE_CLK_EN,
    ahbinf_resp_err_status0: AHBINF_RESP_ERR_STATUS0,
    ahbinf_resp_err_status1: AHBINF_RESP_ERR_STATUS1,
}
impl RegisterBlock {
    #[doc = "0x00 - Raw interrupt status of RX channel 0"]
    #[inline(always)]
    pub const fn in_int_raw_ch0(&self) -> &IN_INT_RAW_CH0 {
        &self.in_int_raw_ch0
    }
    #[doc = "0x04 - Masked interrupt status of RX channel 0"]
    #[inline(always)]
    pub const fn in_int_st_ch0(&self) -> &IN_INT_ST_CH0 {
        &self.in_int_st_ch0
    }
    #[doc = "0x08 - Interrupt enable bits of RX channel 0"]
    #[inline(always)]
    pub const fn in_int_ena_ch0(&self) -> &IN_INT_ENA_CH0 {
        &self.in_int_ena_ch0
    }
    #[doc = "0x0c - Interrupt clear bits of RX channel 0"]
    #[inline(always)]
    pub const fn in_int_clr_ch0(&self) -> &IN_INT_CLR_CH0 {
        &self.in_int_clr_ch0
    }
    #[doc = "0x10 - Raw interrupt status of RX channel 1"]
    #[inline(always)]
    pub const fn in_int_raw_ch1(&self) -> &IN_INT_RAW_CH1 {
        &self.in_int_raw_ch1
    }
    #[doc = "0x14 - Masked interrupt status of RX channel 1"]
    #[inline(always)]
    pub const fn in_int_st_ch1(&self) -> &IN_INT_ST_CH1 {
        &self.in_int_st_ch1
    }
    #[doc = "0x18 - Interrupt enable bits of RX channel 1"]
    #[inline(always)]
    pub const fn in_int_ena_ch1(&self) -> &IN_INT_ENA_CH1 {
        &self.in_int_ena_ch1
    }
    #[doc = "0x1c - Interrupt clear bits of RX channel 1"]
    #[inline(always)]
    pub const fn in_int_clr_ch1(&self) -> &IN_INT_CLR_CH1 {
        &self.in_int_clr_ch1
    }
    #[doc = "0x30 - //Raw interrupt status of TX channel 0"]
    #[inline(always)]
    pub const fn out_int_raw_ch0(&self) -> &OUT_INT_RAW_CH0 {
        &self.out_int_raw_ch0
    }
    #[doc = "0x34 - Masked interrupt status of TX channel 0"]
    #[inline(always)]
    pub const fn out_int_st_ch0(&self) -> &OUT_INT_ST_CH0 {
        &self.out_int_st_ch0
    }
    #[doc = "0x38 - Interrupt enable bits of TX channel 0"]
    #[inline(always)]
    pub const fn out_int_ena_ch0(&self) -> &OUT_INT_ENA_CH0 {
        &self.out_int_ena_ch0
    }
    #[doc = "0x3c - Interrupt clear bits of TX channel 0"]
    #[inline(always)]
    pub const fn out_int_clr_ch0(&self) -> &OUT_INT_CLR_CH0 {
        &self.out_int_clr_ch0
    }
    #[doc = "0x40 - //Raw interrupt status of TX channel 1"]
    #[inline(always)]
    pub const fn out_int_raw_ch1(&self) -> &OUT_INT_RAW_CH1 {
        &self.out_int_raw_ch1
    }
    #[doc = "0x44 - Masked interrupt status of TX channel 1"]
    #[inline(always)]
    pub const fn out_int_st_ch1(&self) -> &OUT_INT_ST_CH1 {
        &self.out_int_st_ch1
    }
    #[doc = "0x48 - Interrupt enable bits of TX channel 1"]
    #[inline(always)]
    pub const fn out_int_ena_ch1(&self) -> &OUT_INT_ENA_CH1 {
        &self.out_int_ena_ch1
    }
    #[doc = "0x4c - Interrupt clear bits of TX channel 1"]
    #[inline(always)]
    pub const fn out_int_clr_ch1(&self) -> &OUT_INT_CLR_CH1 {
        &self.out_int_clr_ch1
    }
    #[doc = "0x60 - only for test"]
    #[inline(always)]
    pub const fn ahb_test(&self) -> &AHB_TEST {
        &self.ahb_test
    }
    #[doc = "0x64 - reserved"]
    #[inline(always)]
    pub const fn misc_conf(&self) -> &MISC_CONF {
        &self.misc_conf
    }
    #[doc = "0x68 - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    #[doc = "0x70 - Configuration register 0 of RX channel 0"]
    #[inline(always)]
    pub const fn in_conf0_ch0(&self) -> &IN_CONF0_CH0 {
        &self.in_conf0_ch0
    }
    #[doc = "0x74 - Configuration register 1 of RX channel 0"]
    #[inline(always)]
    pub const fn in_conf1_ch0(&self) -> &IN_CONF1_CH0 {
        &self.in_conf1_ch0
    }
    #[doc = "0x78 - Receive FIFO status of RX channel 0"]
    #[inline(always)]
    pub const fn infifo_status_ch0(&self) -> &INFIFO_STATUS_CH0 {
        &self.infifo_status_ch0
    }
    #[doc = "0x7c - Receive FIFO status of RX channel 0"]
    #[inline(always)]
    pub const fn in_pop_ch0(&self) -> &IN_POP_CH0 {
        &self.in_pop_ch0
    }
    #[doc = "0x80 - Receive FIFO status of RX channel 0"]
    #[inline(always)]
    pub const fn in_link_ch0(&self) -> &IN_LINK_CH0 {
        &self.in_link_ch0
    }
    #[doc = "0x84 - Receive status of RX channel 0"]
    #[inline(always)]
    pub const fn in_state_ch0(&self) -> &IN_STATE_CH0 {
        &self.in_state_ch0
    }
    #[doc = "0x88 - Receive descriptor address when EOF occurs on RX channel 0"]
    #[inline(always)]
    pub const fn in_suc_eof_des_addr_ch0(&self) -> &IN_SUC_EOF_DES_ADDR_CH0 {
        &self.in_suc_eof_des_addr_ch0
    }
    #[doc = "0x8c - Receive descriptor address when errors occur of RX channel 0"]
    #[inline(always)]
    pub const fn in_err_eof_des_addr_ch0(&self) -> &IN_ERR_EOF_DES_ADDR_CH0 {
        &self.in_err_eof_des_addr_ch0
    }
    #[doc = "0x90 - Current receive descriptor address of RX channel 0"]
    #[inline(always)]
    pub const fn in_dscr_ch0(&self) -> &IN_DSCR_CH0 {
        &self.in_dscr_ch0
    }
    #[doc = "0x94 - The last receive descriptor address of RX channel 0"]
    #[inline(always)]
    pub const fn in_dscr_bf0_ch0(&self) -> &IN_DSCR_BF0_CH0 {
        &self.in_dscr_bf0_ch0
    }
    #[doc = "0x98 - The second-to-last receive descriptor address of RX channel 0"]
    #[inline(always)]
    pub const fn in_dscr_bf1_ch0(&self) -> &IN_DSCR_BF1_CH0 {
        &self.in_dscr_bf1_ch0
    }
    #[doc = "0x9c - Priority register of RX channel 0"]
    #[inline(always)]
    pub const fn in_peri_ch0(&self) -> &IN_PERI_CH0 {
        &self.in_peri_ch0
    }
    #[doc = "0xa0 - Peripheral selection register of RX channel 0"]
    #[inline(always)]
    pub const fn in_peri_sel_ch0(&self) -> &IN_PERI_SEL_CH0 {
        &self.in_peri_sel_ch0
    }
    #[doc = "0xb0 - RX_done Inlink descriptor address of RX channel 0"]
    #[inline(always)]
    pub const fn in_done_des_addr_ch0(&self) -> &IN_DONE_DES_ADDR_CH0 {
        &self.in_done_des_addr_ch0
    }
    #[doc = "0xd0 - Configuration register 0 of TX channel 0"]
    #[inline(always)]
    pub const fn out_conf0_ch0(&self) -> &OUT_CONF0_CH0 {
        &self.out_conf0_ch0
    }
    #[doc = "0xd4 - Configuration register 1 of TX channel 0"]
    #[inline(always)]
    pub const fn out_conf1_ch0(&self) -> &OUT_CONF1_CH0 {
        &self.out_conf1_ch0
    }
    #[doc = "0xd8 - Receive FIFO status of RX channel 0"]
    #[inline(always)]
    pub const fn outfifo_status_ch0(&self) -> &OUTFIFO_STATUS_CH0 {
        &self.outfifo_status_ch0
    }
    #[doc = "0xdc - Push control register of TX channel 0"]
    #[inline(always)]
    pub const fn out_push_ch0(&self) -> &OUT_PUSH_CH0 {
        &self.out_push_ch0
    }
    #[doc = "0xe0 - Push control register of TX channel 0"]
    #[inline(always)]
    pub const fn out_link_ch0(&self) -> &OUT_LINK_CH0 {
        &self.out_link_ch0
    }
    #[doc = "0xe4 - Transmit status of TX channel 0"]
    #[inline(always)]
    pub const fn out_state_ch0(&self) -> &OUT_STATE_CH0 {
        &self.out_state_ch0
    }
    #[doc = "0xe8 - Transmit descriptor address when EOF occurs on TX channel 0"]
    #[inline(always)]
    pub const fn out_eof_des_addr_ch0(&self) -> &OUT_EOF_DES_ADDR_CH0 {
        &self.out_eof_des_addr_ch0
    }
    #[doc = "0xec - The last transmit descriptor address when EOF occurs on TX channel 0"]
    #[inline(always)]
    pub const fn out_eof_bfr_des_addr_ch0(&self) -> &OUT_EOF_BFR_DES_ADDR_CH0 {
        &self.out_eof_bfr_des_addr_ch0
    }
    #[doc = "0xf0 - Current transmit descriptor address of TX channel 0"]
    #[inline(always)]
    pub const fn out_dscr_ch0(&self) -> &OUT_DSCR_CH0 {
        &self.out_dscr_ch0
    }
    #[doc = "0xf4 - The last transmit descriptor address of TX channel 0"]
    #[inline(always)]
    pub const fn out_dscr_bf0_ch0(&self) -> &OUT_DSCR_BF0_CH0 {
        &self.out_dscr_bf0_ch0
    }
    #[doc = "0xf8 - The second-to-last transmit descriptor address of TX channel 0"]
    #[inline(always)]
    pub const fn out_dscr_bf1_ch0(&self) -> &OUT_DSCR_BF1_CH0 {
        &self.out_dscr_bf1_ch0
    }
    #[doc = "0xfc - Priority register of TX channel 0"]
    #[inline(always)]
    pub const fn out_peri_ch0(&self) -> &OUT_PERI_CH0 {
        &self.out_peri_ch0
    }
    #[doc = "0x100 - Peripheral selection register of TX channel 0"]
    #[inline(always)]
    pub const fn out_peri_sel_ch0(&self) -> &OUT_PERI_SEL_CH0 {
        &self.out_peri_sel_ch0
    }
    #[doc = "0x110 - TX done outlink descriptor address of TX channel 0"]
    #[inline(always)]
    pub const fn out_done_des_addr_ch0(&self) -> &OUT_DONE_DES_ADDR_CH0 {
        &self.out_done_des_addr_ch0
    }
    #[doc = "0x130 - Configuration register 0 of RX channel 1"]
    #[inline(always)]
    pub const fn in_conf0_ch1(&self) -> &IN_CONF0_CH1 {
        &self.in_conf0_ch1
    }
    #[doc = "0x134 - Configuration register 1 of RX channel 1"]
    #[inline(always)]
    pub const fn in_conf1_ch1(&self) -> &IN_CONF1_CH1 {
        &self.in_conf1_ch1
    }
    #[doc = "0x138 - Receive FIFO status of RX channel 1"]
    #[inline(always)]
    pub const fn infifo_status_ch1(&self) -> &INFIFO_STATUS_CH1 {
        &self.infifo_status_ch1
    }
    #[doc = "0x13c - Receive FIFO status of RX channel 1"]
    #[inline(always)]
    pub const fn in_pop_ch1(&self) -> &IN_POP_CH1 {
        &self.in_pop_ch1
    }
    #[doc = "0x140 - Receive FIFO status of RX channel 1"]
    #[inline(always)]
    pub const fn in_link_ch1(&self) -> &IN_LINK_CH1 {
        &self.in_link_ch1
    }
    #[doc = "0x144 - Receive status of RX channel 1"]
    #[inline(always)]
    pub const fn in_state_ch1(&self) -> &IN_STATE_CH1 {
        &self.in_state_ch1
    }
    #[doc = "0x148 - Receive descriptor address when EOF occurs on RX channel 1"]
    #[inline(always)]
    pub const fn in_suc_eof_des_addr_ch1(&self) -> &IN_SUC_EOF_DES_ADDR_CH1 {
        &self.in_suc_eof_des_addr_ch1
    }
    #[doc = "0x14c - Receive descriptor address when errors occur of RX channel 1"]
    #[inline(always)]
    pub const fn in_err_eof_des_addr_ch1(&self) -> &IN_ERR_EOF_DES_ADDR_CH1 {
        &self.in_err_eof_des_addr_ch1
    }
    #[doc = "0x150 - Current receive descriptor address of RX channel 1"]
    #[inline(always)]
    pub const fn in_dscr_ch1(&self) -> &IN_DSCR_CH1 {
        &self.in_dscr_ch1
    }
    #[doc = "0x154 - The last receive descriptor address of RX channel 1"]
    #[inline(always)]
    pub const fn in_dscr_bf0_ch1(&self) -> &IN_DSCR_BF0_CH1 {
        &self.in_dscr_bf0_ch1
    }
    #[doc = "0x158 - The second-to-last receive descriptor address of RX channel 1"]
    #[inline(always)]
    pub const fn in_dscr_bf1_ch1(&self) -> &IN_DSCR_BF1_CH1 {
        &self.in_dscr_bf1_ch1
    }
    #[doc = "0x15c - Priority register of RX channel 1"]
    #[inline(always)]
    pub const fn in_peri_ch1(&self) -> &IN_PERI_CH1 {
        &self.in_peri_ch1
    }
    #[doc = "0x160 - Peripheral selection register of RX channel 1"]
    #[inline(always)]
    pub const fn in_peri_sel_ch1(&self) -> &IN_PERI_SEL_CH1 {
        &self.in_peri_sel_ch1
    }
    #[doc = "0x170 - RX_done Inlink descriptor address of RX channel 1"]
    #[inline(always)]
    pub const fn in_done_des_addr_ch1(&self) -> &IN_DONE_DES_ADDR_CH1 {
        &self.in_done_des_addr_ch1
    }
    #[doc = "0x190 - Configuration register 0 of TX channel 1"]
    #[inline(always)]
    pub const fn out_conf0_ch1(&self) -> &OUT_CONF0_CH1 {
        &self.out_conf0_ch1
    }
    #[doc = "0x194 - Configuration register 1 of TX channel 1"]
    #[inline(always)]
    pub const fn out_conf1_ch1(&self) -> &OUT_CONF1_CH1 {
        &self.out_conf1_ch1
    }
    #[doc = "0x198 - Receive FIFO status of RX channel 1"]
    #[inline(always)]
    pub const fn outfifo_status_ch1(&self) -> &OUTFIFO_STATUS_CH1 {
        &self.outfifo_status_ch1
    }
    #[doc = "0x19c - Push control register of TX channel 1"]
    #[inline(always)]
    pub const fn out_push_ch1(&self) -> &OUT_PUSH_CH1 {
        &self.out_push_ch1
    }
    #[doc = "0x1a0 - Push control register of TX channel 1"]
    #[inline(always)]
    pub const fn out_link_ch1(&self) -> &OUT_LINK_CH1 {
        &self.out_link_ch1
    }
    #[doc = "0x1a4 - Transmit status of TX channel 1"]
    #[inline(always)]
    pub const fn out_state_ch1(&self) -> &OUT_STATE_CH1 {
        &self.out_state_ch1
    }
    #[doc = "0x1a8 - Transmit descriptor address when EOF occurs on TX channel 1"]
    #[inline(always)]
    pub const fn out_eof_des_addr_ch1(&self) -> &OUT_EOF_DES_ADDR_CH1 {
        &self.out_eof_des_addr_ch1
    }
    #[doc = "0x1ac - The last transmit descriptor address when EOF occurs on TX channel 1"]
    #[inline(always)]
    pub const fn out_eof_bfr_des_addr_ch1(&self) -> &OUT_EOF_BFR_DES_ADDR_CH1 {
        &self.out_eof_bfr_des_addr_ch1
    }
    #[doc = "0x1b0 - Current transmit descriptor address of TX channel 1"]
    #[inline(always)]
    pub const fn out_dscr_ch1(&self) -> &OUT_DSCR_CH1 {
        &self.out_dscr_ch1
    }
    #[doc = "0x1b4 - The last transmit descriptor address of TX channel 1"]
    #[inline(always)]
    pub const fn out_dscr_bf0_ch1(&self) -> &OUT_DSCR_BF0_CH1 {
        &self.out_dscr_bf0_ch1
    }
    #[doc = "0x1b8 - The second-to-last transmit descriptor address of TX channel 1"]
    #[inline(always)]
    pub const fn out_dscr_bf1_ch1(&self) -> &OUT_DSCR_BF1_CH1 {
        &self.out_dscr_bf1_ch1
    }
    #[doc = "0x1bc - Priority register of TX channel 1"]
    #[inline(always)]
    pub const fn out_peri_ch1(&self) -> &OUT_PERI_CH1 {
        &self.out_peri_ch1
    }
    #[doc = "0x1c0 - Peripheral selection register of TX channel 1"]
    #[inline(always)]
    pub const fn out_peri_sel_ch1(&self) -> &OUT_PERI_SEL_CH1 {
        &self.out_peri_sel_ch1
    }
    #[doc = "0x1d0 - TX done outlink descriptor address of TX channel 1"]
    #[inline(always)]
    pub const fn out_done_des_addr_ch1(&self) -> &OUT_DONE_DES_ADDR_CH1 {
        &self.out_done_des_addr_ch1
    }
    #[doc = "0x2dc - TX channel 0 arbitration weight configuration register"]
    #[inline(always)]
    pub const fn tx_ch_arb_weight_ch0(&self) -> &TX_CH_ARB_WEIGHT_CH0 {
        &self.tx_ch_arb_weight_ch0
    }
    #[doc = "0x2e0 - TX channel 0 weight arbitration optimization enable register"]
    #[inline(always)]
    pub const fn tx_arb_weight_opt_dir_ch0(&self) -> &TX_ARB_WEIGHT_OPT_DIR_CH0 {
        &self.tx_arb_weight_opt_dir_ch0
    }
    #[doc = "0x304 - TX channel 1 arbitration weight configuration register"]
    #[inline(always)]
    pub const fn tx_ch_arb_weight_ch1(&self) -> &TX_CH_ARB_WEIGHT_CH1 {
        &self.tx_ch_arb_weight_ch1
    }
    #[doc = "0x308 - TX channel 1 weight arbitration optimization enable register"]
    #[inline(always)]
    pub const fn tx_arb_weight_opt_dir_ch1(&self) -> &TX_ARB_WEIGHT_OPT_DIR_CH1 {
        &self.tx_arb_weight_opt_dir_ch1
    }
    #[doc = "0x354 - RX channel 0 arbitration weight configuration register"]
    #[inline(always)]
    pub const fn rx_ch_arb_weight_ch0(&self) -> &RX_CH_ARB_WEIGHT_CH0 {
        &self.rx_ch_arb_weight_ch0
    }
    #[doc = "0x358 - RX channel 0 weight arbitration optimization enable register"]
    #[inline(always)]
    pub const fn rx_arb_weight_opt_dir_ch0(&self) -> &RX_ARB_WEIGHT_OPT_DIR_CH0 {
        &self.rx_arb_weight_opt_dir_ch0
    }
    #[doc = "0x37c - RX channel 1 arbitration weight configuration register"]
    #[inline(always)]
    pub const fn rx_ch_arb_weight_ch1(&self) -> &RX_CH_ARB_WEIGHT_CH1 {
        &self.rx_ch_arb_weight_ch1
    }
    #[doc = "0x380 - RX channel 1 weight arbitration optimization enable register"]
    #[inline(always)]
    pub const fn rx_arb_weight_opt_dir_ch1(&self) -> &RX_ARB_WEIGHT_OPT_DIR_CH1 {
        &self.rx_arb_weight_opt_dir_ch1
    }
    #[doc = "0x3ac - Link list descriptor address configuration of RX channel 0"]
    #[inline(always)]
    pub const fn in_link_addr_ch0(&self) -> &IN_LINK_ADDR_CH0 {
        &self.in_link_addr_ch0
    }
    #[doc = "0x3b0 - Link list descriptor address configuration of RX channel 1"]
    #[inline(always)]
    pub const fn in_link_addr_ch1(&self) -> &IN_LINK_ADDR_CH1 {
        &self.in_link_addr_ch1
    }
    #[doc = "0x3b8 - Link list descriptor address configuration of TX channel 0"]
    #[inline(always)]
    pub const fn out_link_addr_ch0(&self) -> &OUT_LINK_ADDR_CH0 {
        &self.out_link_addr_ch0
    }
    #[doc = "0x3bc - Link list descriptor address configuration of TX channel 1"]
    #[inline(always)]
    pub const fn out_link_addr_ch1(&self) -> &OUT_LINK_ADDR_CH1 {
        &self.out_link_addr_ch1
    }
    #[doc = "0x3c4 - Accessible address space start address configuration register"]
    #[inline(always)]
    pub const fn intr_mem_start_addr(&self) -> &INTR_MEM_START_ADDR {
        &self.intr_mem_start_addr
    }
    #[doc = "0x3c8 - Accessible address space end address configuration register"]
    #[inline(always)]
    pub const fn intr_mem_end_addr(&self) -> &INTR_MEM_END_ADDR {
        &self.intr_mem_end_addr
    }
    #[doc = "0x3cc - TX arbitration timeout configuration register"]
    #[inline(always)]
    pub const fn arb_timeout_tx(&self) -> &ARB_TIMEOUT_TX {
        &self.arb_timeout_tx
    }
    #[doc = "0x3d0 - RX arbitration timeout configuration register"]
    #[inline(always)]
    pub const fn arb_timeout_rx(&self) -> &ARB_TIMEOUT_RX {
        &self.arb_timeout_rx
    }
    #[doc = "0x3d4 - TX weight arbitration enable register"]
    #[inline(always)]
    pub const fn weight_en_tx(&self) -> &WEIGHT_EN_TX {
        &self.weight_en_tx
    }
    #[doc = "0x3d8 - RX weight arbitration enable register"]
    #[inline(always)]
    pub const fn weight_en_rx(&self) -> &WEIGHT_EN_RX {
        &self.weight_en_rx
    }
    #[doc = "0x3dc - TX arbitration timeout configuration register"]
    #[inline(always)]
    pub const fn arb_timeout(&self) -> &ARB_TIMEOUT {
        &self.arb_timeout
    }
    #[doc = "0x400 - TX weight arbitration enable register"]
    #[inline(always)]
    pub const fn weight_en(&self) -> &WEIGHT_EN {
        &self.weight_en
    }
    #[doc = "0x404 - Module clock force on register"]
    #[inline(always)]
    pub const fn module_clk_en(&self) -> &MODULE_CLK_EN {
        &self.module_clk_en
    }
    #[doc = "0x408 - AHB response error status 0 register"]
    #[inline(always)]
    pub const fn ahbinf_resp_err_status0(&self) -> &AHBINF_RESP_ERR_STATUS0 {
        &self.ahbinf_resp_err_status0
    }
    #[doc = "0x40c - AHB response error status 1 register"]
    #[inline(always)]
    pub const fn ahbinf_resp_err_status1(&self) -> &AHBINF_RESP_ERR_STATUS1 {
        &self.ahbinf_resp_err_status1
    }
}
#[doc = "IN_INT_RAW_CH0 (rw) register accessor: Raw interrupt status of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_int_raw_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_int_raw_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_raw_ch0`] module"]
pub type IN_INT_RAW_CH0 = crate::Reg<in_int_raw_ch0::IN_INT_RAW_CH0_SPEC>;
#[doc = "Raw interrupt status of RX channel 0"]
pub mod in_int_raw_ch0;
#[doc = "IN_INT_ST_CH0 (r) register accessor: Masked interrupt status of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_int_st_ch0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_st_ch0`] module"]
pub type IN_INT_ST_CH0 = crate::Reg<in_int_st_ch0::IN_INT_ST_CH0_SPEC>;
#[doc = "Masked interrupt status of RX channel 0"]
pub mod in_int_st_ch0;
#[doc = "IN_INT_ENA_CH0 (rw) register accessor: Interrupt enable bits of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_int_ena_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_int_ena_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_ena_ch0`] module"]
pub type IN_INT_ENA_CH0 = crate::Reg<in_int_ena_ch0::IN_INT_ENA_CH0_SPEC>;
#[doc = "Interrupt enable bits of RX channel 0"]
pub mod in_int_ena_ch0;
#[doc = "IN_INT_CLR_CH0 (w) register accessor: Interrupt clear bits of RX channel 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_int_clr_ch0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_clr_ch0`] module"]
pub type IN_INT_CLR_CH0 = crate::Reg<in_int_clr_ch0::IN_INT_CLR_CH0_SPEC>;
#[doc = "Interrupt clear bits of RX channel 0"]
pub mod in_int_clr_ch0;
#[doc = "IN_INT_RAW_CH1 (rw) register accessor: Raw interrupt status of RX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`in_int_raw_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_int_raw_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_raw_ch1`] module"]
pub type IN_INT_RAW_CH1 = crate::Reg<in_int_raw_ch1::IN_INT_RAW_CH1_SPEC>;
#[doc = "Raw interrupt status of RX channel 1"]
pub mod in_int_raw_ch1;
#[doc = "IN_INT_ST_CH1 (r) register accessor: Masked interrupt status of RX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`in_int_st_ch1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_st_ch1`] module"]
pub type IN_INT_ST_CH1 = crate::Reg<in_int_st_ch1::IN_INT_ST_CH1_SPEC>;
#[doc = "Masked interrupt status of RX channel 1"]
pub mod in_int_st_ch1;
#[doc = "IN_INT_ENA_CH1 (rw) register accessor: Interrupt enable bits of RX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`in_int_ena_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_int_ena_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_ena_ch1`] module"]
pub type IN_INT_ENA_CH1 = crate::Reg<in_int_ena_ch1::IN_INT_ENA_CH1_SPEC>;
#[doc = "Interrupt enable bits of RX channel 1"]
pub mod in_int_ena_ch1;
#[doc = "IN_INT_CLR_CH1 (w) register accessor: Interrupt clear bits of RX channel 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_int_clr_ch1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_clr_ch1`] module"]
pub type IN_INT_CLR_CH1 = crate::Reg<in_int_clr_ch1::IN_INT_CLR_CH1_SPEC>;
#[doc = "Interrupt clear bits of RX channel 1"]
pub mod in_int_clr_ch1;
#[doc = "OUT_INT_RAW_CH0 (rw) register accessor: //Raw interrupt status of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_int_raw_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_int_raw_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_raw_ch0`] module"]
pub type OUT_INT_RAW_CH0 = crate::Reg<out_int_raw_ch0::OUT_INT_RAW_CH0_SPEC>;
#[doc = "//Raw interrupt status of TX channel 0"]
pub mod out_int_raw_ch0;
#[doc = "OUT_INT_ST_CH0 (r) register accessor: Masked interrupt status of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_int_st_ch0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_st_ch0`] module"]
pub type OUT_INT_ST_CH0 = crate::Reg<out_int_st_ch0::OUT_INT_ST_CH0_SPEC>;
#[doc = "Masked interrupt status of TX channel 0"]
pub mod out_int_st_ch0;
#[doc = "OUT_INT_ENA_CH0 (rw) register accessor: Interrupt enable bits of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_int_ena_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_int_ena_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_ena_ch0`] module"]
pub type OUT_INT_ENA_CH0 = crate::Reg<out_int_ena_ch0::OUT_INT_ENA_CH0_SPEC>;
#[doc = "Interrupt enable bits of TX channel 0"]
pub mod out_int_ena_ch0;
#[doc = "OUT_INT_CLR_CH0 (w) register accessor: Interrupt clear bits of TX channel 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_int_clr_ch0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_clr_ch0`] module"]
pub type OUT_INT_CLR_CH0 = crate::Reg<out_int_clr_ch0::OUT_INT_CLR_CH0_SPEC>;
#[doc = "Interrupt clear bits of TX channel 0"]
pub mod out_int_clr_ch0;
#[doc = "OUT_INT_RAW_CH1 (rw) register accessor: //Raw interrupt status of TX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`out_int_raw_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_int_raw_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_raw_ch1`] module"]
pub type OUT_INT_RAW_CH1 = crate::Reg<out_int_raw_ch1::OUT_INT_RAW_CH1_SPEC>;
#[doc = "//Raw interrupt status of TX channel 1"]
pub mod out_int_raw_ch1;
#[doc = "OUT_INT_ST_CH1 (r) register accessor: Masked interrupt status of TX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`out_int_st_ch1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_st_ch1`] module"]
pub type OUT_INT_ST_CH1 = crate::Reg<out_int_st_ch1::OUT_INT_ST_CH1_SPEC>;
#[doc = "Masked interrupt status of TX channel 1"]
pub mod out_int_st_ch1;
#[doc = "OUT_INT_ENA_CH1 (rw) register accessor: Interrupt enable bits of TX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`out_int_ena_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_int_ena_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_ena_ch1`] module"]
pub type OUT_INT_ENA_CH1 = crate::Reg<out_int_ena_ch1::OUT_INT_ENA_CH1_SPEC>;
#[doc = "Interrupt enable bits of TX channel 1"]
pub mod out_int_ena_ch1;
#[doc = "OUT_INT_CLR_CH1 (w) register accessor: Interrupt clear bits of TX channel 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_int_clr_ch1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_clr_ch1`] module"]
pub type OUT_INT_CLR_CH1 = crate::Reg<out_int_clr_ch1::OUT_INT_CLR_CH1_SPEC>;
#[doc = "Interrupt clear bits of TX channel 1"]
pub mod out_int_clr_ch1;
#[doc = "AHB_TEST (rw) register accessor: only for test\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_test`] module"]
pub type AHB_TEST = crate::Reg<ahb_test::AHB_TEST_SPEC>;
#[doc = "only for test"]
pub mod ahb_test;
#[doc = "MISC_CONF (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`misc_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_conf`] module"]
pub type MISC_CONF = crate::Reg<misc_conf::MISC_CONF_SPEC>;
#[doc = "reserved"]
pub mod misc_conf;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
#[doc = "IN_CONF0_CH0 (rw) register accessor: Configuration register 0 of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_conf0_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_conf0_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf0_ch0`] module"]
pub type IN_CONF0_CH0 = crate::Reg<in_conf0_ch0::IN_CONF0_CH0_SPEC>;
#[doc = "Configuration register 0 of RX channel 0"]
pub mod in_conf0_ch0;
#[doc = "IN_CONF1_CH0 (rw) register accessor: Configuration register 1 of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_conf1_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_conf1_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf1_ch0`] module"]
pub type IN_CONF1_CH0 = crate::Reg<in_conf1_ch0::IN_CONF1_CH0_SPEC>;
#[doc = "Configuration register 1 of RX channel 0"]
pub mod in_conf1_ch0;
#[doc = "INFIFO_STATUS_CH0 (r) register accessor: Receive FIFO status of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`infifo_status_ch0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@infifo_status_ch0`] module"]
pub type INFIFO_STATUS_CH0 = crate::Reg<infifo_status_ch0::INFIFO_STATUS_CH0_SPEC>;
#[doc = "Receive FIFO status of RX channel 0"]
pub mod infifo_status_ch0;
#[doc = "IN_POP_CH0 (rw) register accessor: Receive FIFO status of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_pop_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_pop_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pop_ch0`] module"]
pub type IN_POP_CH0 = crate::Reg<in_pop_ch0::IN_POP_CH0_SPEC>;
#[doc = "Receive FIFO status of RX channel 0"]
pub mod in_pop_ch0;
#[doc = "IN_LINK_CH0 (rw) register accessor: Receive FIFO status of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_link_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_link_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link_ch0`] module"]
pub type IN_LINK_CH0 = crate::Reg<in_link_ch0::IN_LINK_CH0_SPEC>;
#[doc = "Receive FIFO status of RX channel 0"]
pub mod in_link_ch0;
#[doc = "IN_STATE_CH0 (r) register accessor: Receive status of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_state_ch0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_state_ch0`] module"]
pub type IN_STATE_CH0 = crate::Reg<in_state_ch0::IN_STATE_CH0_SPEC>;
#[doc = "Receive status of RX channel 0"]
pub mod in_state_ch0;
#[doc = "IN_SUC_EOF_DES_ADDR_CH0 (r) register accessor: Receive descriptor address when EOF occurs on RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_suc_eof_des_addr_ch0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_suc_eof_des_addr_ch0`] module"]
pub type IN_SUC_EOF_DES_ADDR_CH0 =
    crate::Reg<in_suc_eof_des_addr_ch0::IN_SUC_EOF_DES_ADDR_CH0_SPEC>;
#[doc = "Receive descriptor address when EOF occurs on RX channel 0"]
pub mod in_suc_eof_des_addr_ch0;
#[doc = "IN_ERR_EOF_DES_ADDR_CH0 (r) register accessor: Receive descriptor address when errors occur of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_err_eof_des_addr_ch0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_err_eof_des_addr_ch0`] module"]
pub type IN_ERR_EOF_DES_ADDR_CH0 =
    crate::Reg<in_err_eof_des_addr_ch0::IN_ERR_EOF_DES_ADDR_CH0_SPEC>;
#[doc = "Receive descriptor address when errors occur of RX channel 0"]
pub mod in_err_eof_des_addr_ch0;
#[doc = "IN_DSCR_CH0 (r) register accessor: Current receive descriptor address of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_dscr_ch0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_ch0`] module"]
pub type IN_DSCR_CH0 = crate::Reg<in_dscr_ch0::IN_DSCR_CH0_SPEC>;
#[doc = "Current receive descriptor address of RX channel 0"]
pub mod in_dscr_ch0;
#[doc = "IN_DSCR_BF0_CH0 (r) register accessor: The last receive descriptor address of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_dscr_bf0_ch0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf0_ch0`] module"]
pub type IN_DSCR_BF0_CH0 = crate::Reg<in_dscr_bf0_ch0::IN_DSCR_BF0_CH0_SPEC>;
#[doc = "The last receive descriptor address of RX channel 0"]
pub mod in_dscr_bf0_ch0;
#[doc = "IN_DSCR_BF1_CH0 (r) register accessor: The second-to-last receive descriptor address of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_dscr_bf1_ch0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf1_ch0`] module"]
pub type IN_DSCR_BF1_CH0 = crate::Reg<in_dscr_bf1_ch0::IN_DSCR_BF1_CH0_SPEC>;
#[doc = "The second-to-last receive descriptor address of RX channel 0"]
pub mod in_dscr_bf1_ch0;
#[doc = "IN_PERI_CH0 (rw) register accessor: Priority register of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_peri_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_peri_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_peri_ch0`] module"]
pub type IN_PERI_CH0 = crate::Reg<in_peri_ch0::IN_PERI_CH0_SPEC>;
#[doc = "Priority register of RX channel 0"]
pub mod in_peri_ch0;
#[doc = "IN_PERI_SEL_CH0 (rw) register accessor: Peripheral selection register of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_peri_sel_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_peri_sel_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_peri_sel_ch0`] module"]
pub type IN_PERI_SEL_CH0 = crate::Reg<in_peri_sel_ch0::IN_PERI_SEL_CH0_SPEC>;
#[doc = "Peripheral selection register of RX channel 0"]
pub mod in_peri_sel_ch0;
#[doc = "IN_DONE_DES_ADDR_CH0 (r) register accessor: RX_done Inlink descriptor address of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_done_des_addr_ch0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_done_des_addr_ch0`] module"]
pub type IN_DONE_DES_ADDR_CH0 = crate::Reg<in_done_des_addr_ch0::IN_DONE_DES_ADDR_CH0_SPEC>;
#[doc = "RX_done Inlink descriptor address of RX channel 0"]
pub mod in_done_des_addr_ch0;
#[doc = "OUT_CONF0_CH0 (rw) register accessor: Configuration register 0 of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_conf0_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_conf0_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf0_ch0`] module"]
pub type OUT_CONF0_CH0 = crate::Reg<out_conf0_ch0::OUT_CONF0_CH0_SPEC>;
#[doc = "Configuration register 0 of TX channel 0"]
pub mod out_conf0_ch0;
#[doc = "OUT_CONF1_CH0 (rw) register accessor: Configuration register 1 of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_conf1_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_conf1_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf1_ch0`] module"]
pub type OUT_CONF1_CH0 = crate::Reg<out_conf1_ch0::OUT_CONF1_CH0_SPEC>;
#[doc = "Configuration register 1 of TX channel 0"]
pub mod out_conf1_ch0;
#[doc = "OUTFIFO_STATUS_CH0 (r) register accessor: Receive FIFO status of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`outfifo_status_ch0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outfifo_status_ch0`] module"]
pub type OUTFIFO_STATUS_CH0 = crate::Reg<outfifo_status_ch0::OUTFIFO_STATUS_CH0_SPEC>;
#[doc = "Receive FIFO status of RX channel 0"]
pub mod outfifo_status_ch0;
#[doc = "OUT_PUSH_CH0 (rw) register accessor: Push control register of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_push_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_push_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_push_ch0`] module"]
pub type OUT_PUSH_CH0 = crate::Reg<out_push_ch0::OUT_PUSH_CH0_SPEC>;
#[doc = "Push control register of TX channel 0"]
pub mod out_push_ch0;
#[doc = "OUT_LINK_CH0 (rw) register accessor: Push control register of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_link_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_link_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link_ch0`] module"]
pub type OUT_LINK_CH0 = crate::Reg<out_link_ch0::OUT_LINK_CH0_SPEC>;
#[doc = "Push control register of TX channel 0"]
pub mod out_link_ch0;
#[doc = "OUT_STATE_CH0 (r) register accessor: Transmit status of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_state_ch0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_state_ch0`] module"]
pub type OUT_STATE_CH0 = crate::Reg<out_state_ch0::OUT_STATE_CH0_SPEC>;
#[doc = "Transmit status of TX channel 0"]
pub mod out_state_ch0;
#[doc = "OUT_EOF_DES_ADDR_CH0 (r) register accessor: Transmit descriptor address when EOF occurs on TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_eof_des_addr_ch0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_des_addr_ch0`] module"]
pub type OUT_EOF_DES_ADDR_CH0 = crate::Reg<out_eof_des_addr_ch0::OUT_EOF_DES_ADDR_CH0_SPEC>;
#[doc = "Transmit descriptor address when EOF occurs on TX channel 0"]
pub mod out_eof_des_addr_ch0;
#[doc = "OUT_EOF_BFR_DES_ADDR_CH0 (r) register accessor: The last transmit descriptor address when EOF occurs on TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_eof_bfr_des_addr_ch0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_bfr_des_addr_ch0`] module"]
pub type OUT_EOF_BFR_DES_ADDR_CH0 =
    crate::Reg<out_eof_bfr_des_addr_ch0::OUT_EOF_BFR_DES_ADDR_CH0_SPEC>;
#[doc = "The last transmit descriptor address when EOF occurs on TX channel 0"]
pub mod out_eof_bfr_des_addr_ch0;
#[doc = "OUT_DSCR_CH0 (r) register accessor: Current transmit descriptor address of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_dscr_ch0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_ch0`] module"]
pub type OUT_DSCR_CH0 = crate::Reg<out_dscr_ch0::OUT_DSCR_CH0_SPEC>;
#[doc = "Current transmit descriptor address of TX channel 0"]
pub mod out_dscr_ch0;
#[doc = "OUT_DSCR_BF0_CH0 (r) register accessor: The last transmit descriptor address of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_dscr_bf0_ch0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf0_ch0`] module"]
pub type OUT_DSCR_BF0_CH0 = crate::Reg<out_dscr_bf0_ch0::OUT_DSCR_BF0_CH0_SPEC>;
#[doc = "The last transmit descriptor address of TX channel 0"]
pub mod out_dscr_bf0_ch0;
#[doc = "OUT_DSCR_BF1_CH0 (r) register accessor: The second-to-last transmit descriptor address of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_dscr_bf1_ch0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf1_ch0`] module"]
pub type OUT_DSCR_BF1_CH0 = crate::Reg<out_dscr_bf1_ch0::OUT_DSCR_BF1_CH0_SPEC>;
#[doc = "The second-to-last transmit descriptor address of TX channel 0"]
pub mod out_dscr_bf1_ch0;
#[doc = "OUT_PERI_CH0 (rw) register accessor: Priority register of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_peri_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_peri_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_peri_ch0`] module"]
pub type OUT_PERI_CH0 = crate::Reg<out_peri_ch0::OUT_PERI_CH0_SPEC>;
#[doc = "Priority register of TX channel 0"]
pub mod out_peri_ch0;
#[doc = "OUT_PERI_SEL_CH0 (rw) register accessor: Peripheral selection register of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_peri_sel_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_peri_sel_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_peri_sel_ch0`] module"]
pub type OUT_PERI_SEL_CH0 = crate::Reg<out_peri_sel_ch0::OUT_PERI_SEL_CH0_SPEC>;
#[doc = "Peripheral selection register of TX channel 0"]
pub mod out_peri_sel_ch0;
#[doc = "OUT_DONE_DES_ADDR_CH0 (r) register accessor: TX done outlink descriptor address of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_done_des_addr_ch0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_done_des_addr_ch0`] module"]
pub type OUT_DONE_DES_ADDR_CH0 = crate::Reg<out_done_des_addr_ch0::OUT_DONE_DES_ADDR_CH0_SPEC>;
#[doc = "TX done outlink descriptor address of TX channel 0"]
pub mod out_done_des_addr_ch0;
#[doc = "IN_CONF0_CH1 (rw) register accessor: Configuration register 0 of RX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`in_conf0_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_conf0_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf0_ch1`] module"]
pub type IN_CONF0_CH1 = crate::Reg<in_conf0_ch1::IN_CONF0_CH1_SPEC>;
#[doc = "Configuration register 0 of RX channel 1"]
pub mod in_conf0_ch1;
#[doc = "IN_CONF1_CH1 (rw) register accessor: Configuration register 1 of RX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`in_conf1_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_conf1_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf1_ch1`] module"]
pub type IN_CONF1_CH1 = crate::Reg<in_conf1_ch1::IN_CONF1_CH1_SPEC>;
#[doc = "Configuration register 1 of RX channel 1"]
pub mod in_conf1_ch1;
#[doc = "INFIFO_STATUS_CH1 (r) register accessor: Receive FIFO status of RX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`infifo_status_ch1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@infifo_status_ch1`] module"]
pub type INFIFO_STATUS_CH1 = crate::Reg<infifo_status_ch1::INFIFO_STATUS_CH1_SPEC>;
#[doc = "Receive FIFO status of RX channel 1"]
pub mod infifo_status_ch1;
#[doc = "IN_POP_CH1 (rw) register accessor: Receive FIFO status of RX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`in_pop_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_pop_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pop_ch1`] module"]
pub type IN_POP_CH1 = crate::Reg<in_pop_ch1::IN_POP_CH1_SPEC>;
#[doc = "Receive FIFO status of RX channel 1"]
pub mod in_pop_ch1;
#[doc = "IN_LINK_CH1 (rw) register accessor: Receive FIFO status of RX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`in_link_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_link_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link_ch1`] module"]
pub type IN_LINK_CH1 = crate::Reg<in_link_ch1::IN_LINK_CH1_SPEC>;
#[doc = "Receive FIFO status of RX channel 1"]
pub mod in_link_ch1;
#[doc = "IN_STATE_CH1 (r) register accessor: Receive status of RX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`in_state_ch1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_state_ch1`] module"]
pub type IN_STATE_CH1 = crate::Reg<in_state_ch1::IN_STATE_CH1_SPEC>;
#[doc = "Receive status of RX channel 1"]
pub mod in_state_ch1;
#[doc = "IN_SUC_EOF_DES_ADDR_CH1 (r) register accessor: Receive descriptor address when EOF occurs on RX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`in_suc_eof_des_addr_ch1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_suc_eof_des_addr_ch1`] module"]
pub type IN_SUC_EOF_DES_ADDR_CH1 =
    crate::Reg<in_suc_eof_des_addr_ch1::IN_SUC_EOF_DES_ADDR_CH1_SPEC>;
#[doc = "Receive descriptor address when EOF occurs on RX channel 1"]
pub mod in_suc_eof_des_addr_ch1;
#[doc = "IN_ERR_EOF_DES_ADDR_CH1 (r) register accessor: Receive descriptor address when errors occur of RX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`in_err_eof_des_addr_ch1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_err_eof_des_addr_ch1`] module"]
pub type IN_ERR_EOF_DES_ADDR_CH1 =
    crate::Reg<in_err_eof_des_addr_ch1::IN_ERR_EOF_DES_ADDR_CH1_SPEC>;
#[doc = "Receive descriptor address when errors occur of RX channel 1"]
pub mod in_err_eof_des_addr_ch1;
#[doc = "IN_DSCR_CH1 (r) register accessor: Current receive descriptor address of RX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`in_dscr_ch1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_ch1`] module"]
pub type IN_DSCR_CH1 = crate::Reg<in_dscr_ch1::IN_DSCR_CH1_SPEC>;
#[doc = "Current receive descriptor address of RX channel 1"]
pub mod in_dscr_ch1;
#[doc = "IN_DSCR_BF0_CH1 (r) register accessor: The last receive descriptor address of RX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`in_dscr_bf0_ch1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf0_ch1`] module"]
pub type IN_DSCR_BF0_CH1 = crate::Reg<in_dscr_bf0_ch1::IN_DSCR_BF0_CH1_SPEC>;
#[doc = "The last receive descriptor address of RX channel 1"]
pub mod in_dscr_bf0_ch1;
#[doc = "IN_DSCR_BF1_CH1 (r) register accessor: The second-to-last receive descriptor address of RX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`in_dscr_bf1_ch1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf1_ch1`] module"]
pub type IN_DSCR_BF1_CH1 = crate::Reg<in_dscr_bf1_ch1::IN_DSCR_BF1_CH1_SPEC>;
#[doc = "The second-to-last receive descriptor address of RX channel 1"]
pub mod in_dscr_bf1_ch1;
#[doc = "IN_PERI_CH1 (rw) register accessor: Priority register of RX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`in_peri_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_peri_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_peri_ch1`] module"]
pub type IN_PERI_CH1 = crate::Reg<in_peri_ch1::IN_PERI_CH1_SPEC>;
#[doc = "Priority register of RX channel 1"]
pub mod in_peri_ch1;
#[doc = "IN_PERI_SEL_CH1 (rw) register accessor: Peripheral selection register of RX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`in_peri_sel_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_peri_sel_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_peri_sel_ch1`] module"]
pub type IN_PERI_SEL_CH1 = crate::Reg<in_peri_sel_ch1::IN_PERI_SEL_CH1_SPEC>;
#[doc = "Peripheral selection register of RX channel 1"]
pub mod in_peri_sel_ch1;
#[doc = "IN_DONE_DES_ADDR_CH1 (r) register accessor: RX_done Inlink descriptor address of RX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`in_done_des_addr_ch1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_done_des_addr_ch1`] module"]
pub type IN_DONE_DES_ADDR_CH1 = crate::Reg<in_done_des_addr_ch1::IN_DONE_DES_ADDR_CH1_SPEC>;
#[doc = "RX_done Inlink descriptor address of RX channel 1"]
pub mod in_done_des_addr_ch1;
#[doc = "OUT_CONF0_CH1 (rw) register accessor: Configuration register 0 of TX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`out_conf0_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_conf0_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf0_ch1`] module"]
pub type OUT_CONF0_CH1 = crate::Reg<out_conf0_ch1::OUT_CONF0_CH1_SPEC>;
#[doc = "Configuration register 0 of TX channel 1"]
pub mod out_conf0_ch1;
#[doc = "OUT_CONF1_CH1 (rw) register accessor: Configuration register 1 of TX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`out_conf1_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_conf1_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf1_ch1`] module"]
pub type OUT_CONF1_CH1 = crate::Reg<out_conf1_ch1::OUT_CONF1_CH1_SPEC>;
#[doc = "Configuration register 1 of TX channel 1"]
pub mod out_conf1_ch1;
#[doc = "OUTFIFO_STATUS_CH1 (r) register accessor: Receive FIFO status of RX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`outfifo_status_ch1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outfifo_status_ch1`] module"]
pub type OUTFIFO_STATUS_CH1 = crate::Reg<outfifo_status_ch1::OUTFIFO_STATUS_CH1_SPEC>;
#[doc = "Receive FIFO status of RX channel 1"]
pub mod outfifo_status_ch1;
#[doc = "OUT_PUSH_CH1 (rw) register accessor: Push control register of TX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`out_push_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_push_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_push_ch1`] module"]
pub type OUT_PUSH_CH1 = crate::Reg<out_push_ch1::OUT_PUSH_CH1_SPEC>;
#[doc = "Push control register of TX channel 1"]
pub mod out_push_ch1;
#[doc = "OUT_LINK_CH1 (rw) register accessor: Push control register of TX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`out_link_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_link_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link_ch1`] module"]
pub type OUT_LINK_CH1 = crate::Reg<out_link_ch1::OUT_LINK_CH1_SPEC>;
#[doc = "Push control register of TX channel 1"]
pub mod out_link_ch1;
#[doc = "OUT_STATE_CH1 (r) register accessor: Transmit status of TX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`out_state_ch1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_state_ch1`] module"]
pub type OUT_STATE_CH1 = crate::Reg<out_state_ch1::OUT_STATE_CH1_SPEC>;
#[doc = "Transmit status of TX channel 1"]
pub mod out_state_ch1;
#[doc = "OUT_EOF_DES_ADDR_CH1 (r) register accessor: Transmit descriptor address when EOF occurs on TX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`out_eof_des_addr_ch1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_des_addr_ch1`] module"]
pub type OUT_EOF_DES_ADDR_CH1 = crate::Reg<out_eof_des_addr_ch1::OUT_EOF_DES_ADDR_CH1_SPEC>;
#[doc = "Transmit descriptor address when EOF occurs on TX channel 1"]
pub mod out_eof_des_addr_ch1;
#[doc = "OUT_EOF_BFR_DES_ADDR_CH1 (r) register accessor: The last transmit descriptor address when EOF occurs on TX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`out_eof_bfr_des_addr_ch1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_bfr_des_addr_ch1`] module"]
pub type OUT_EOF_BFR_DES_ADDR_CH1 =
    crate::Reg<out_eof_bfr_des_addr_ch1::OUT_EOF_BFR_DES_ADDR_CH1_SPEC>;
#[doc = "The last transmit descriptor address when EOF occurs on TX channel 1"]
pub mod out_eof_bfr_des_addr_ch1;
#[doc = "OUT_DSCR_CH1 (r) register accessor: Current transmit descriptor address of TX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`out_dscr_ch1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_ch1`] module"]
pub type OUT_DSCR_CH1 = crate::Reg<out_dscr_ch1::OUT_DSCR_CH1_SPEC>;
#[doc = "Current transmit descriptor address of TX channel 1"]
pub mod out_dscr_ch1;
#[doc = "OUT_DSCR_BF0_CH1 (r) register accessor: The last transmit descriptor address of TX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`out_dscr_bf0_ch1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf0_ch1`] module"]
pub type OUT_DSCR_BF0_CH1 = crate::Reg<out_dscr_bf0_ch1::OUT_DSCR_BF0_CH1_SPEC>;
#[doc = "The last transmit descriptor address of TX channel 1"]
pub mod out_dscr_bf0_ch1;
#[doc = "OUT_DSCR_BF1_CH1 (r) register accessor: The second-to-last transmit descriptor address of TX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`out_dscr_bf1_ch1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf1_ch1`] module"]
pub type OUT_DSCR_BF1_CH1 = crate::Reg<out_dscr_bf1_ch1::OUT_DSCR_BF1_CH1_SPEC>;
#[doc = "The second-to-last transmit descriptor address of TX channel 1"]
pub mod out_dscr_bf1_ch1;
#[doc = "OUT_PERI_CH1 (rw) register accessor: Priority register of TX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`out_peri_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_peri_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_peri_ch1`] module"]
pub type OUT_PERI_CH1 = crate::Reg<out_peri_ch1::OUT_PERI_CH1_SPEC>;
#[doc = "Priority register of TX channel 1"]
pub mod out_peri_ch1;
#[doc = "OUT_PERI_SEL_CH1 (rw) register accessor: Peripheral selection register of TX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`out_peri_sel_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_peri_sel_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_peri_sel_ch1`] module"]
pub type OUT_PERI_SEL_CH1 = crate::Reg<out_peri_sel_ch1::OUT_PERI_SEL_CH1_SPEC>;
#[doc = "Peripheral selection register of TX channel 1"]
pub mod out_peri_sel_ch1;
#[doc = "OUT_DONE_DES_ADDR_CH1 (r) register accessor: TX done outlink descriptor address of TX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`out_done_des_addr_ch1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_done_des_addr_ch1`] module"]
pub type OUT_DONE_DES_ADDR_CH1 = crate::Reg<out_done_des_addr_ch1::OUT_DONE_DES_ADDR_CH1_SPEC>;
#[doc = "TX done outlink descriptor address of TX channel 1"]
pub mod out_done_des_addr_ch1;
#[doc = "TX_CH_ARB_WEIGHT_CH0 (rw) register accessor: TX channel 0 arbitration weight configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_ch_arb_weight_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ch_arb_weight_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_ch_arb_weight_ch0`] module"]
pub type TX_CH_ARB_WEIGHT_CH0 = crate::Reg<tx_ch_arb_weight_ch0::TX_CH_ARB_WEIGHT_CH0_SPEC>;
#[doc = "TX channel 0 arbitration weight configuration register"]
pub mod tx_ch_arb_weight_ch0;
#[doc = "TX_ARB_WEIGHT_OPT_DIR_CH0 (rw) register accessor: TX channel 0 weight arbitration optimization enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_arb_weight_opt_dir_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_arb_weight_opt_dir_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_arb_weight_opt_dir_ch0`] module"]
pub type TX_ARB_WEIGHT_OPT_DIR_CH0 =
    crate::Reg<tx_arb_weight_opt_dir_ch0::TX_ARB_WEIGHT_OPT_DIR_CH0_SPEC>;
#[doc = "TX channel 0 weight arbitration optimization enable register"]
pub mod tx_arb_weight_opt_dir_ch0;
#[doc = "TX_CH_ARB_WEIGHT_CH1 (rw) register accessor: TX channel 1 arbitration weight configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_ch_arb_weight_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ch_arb_weight_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_ch_arb_weight_ch1`] module"]
pub type TX_CH_ARB_WEIGHT_CH1 = crate::Reg<tx_ch_arb_weight_ch1::TX_CH_ARB_WEIGHT_CH1_SPEC>;
#[doc = "TX channel 1 arbitration weight configuration register"]
pub mod tx_ch_arb_weight_ch1;
#[doc = "TX_ARB_WEIGHT_OPT_DIR_CH1 (rw) register accessor: TX channel 1 weight arbitration optimization enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_arb_weight_opt_dir_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_arb_weight_opt_dir_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_arb_weight_opt_dir_ch1`] module"]
pub type TX_ARB_WEIGHT_OPT_DIR_CH1 =
    crate::Reg<tx_arb_weight_opt_dir_ch1::TX_ARB_WEIGHT_OPT_DIR_CH1_SPEC>;
#[doc = "TX channel 1 weight arbitration optimization enable register"]
pub mod tx_arb_weight_opt_dir_ch1;
#[doc = "RX_CH_ARB_WEIGHT_CH0 (rw) register accessor: RX channel 0 arbitration weight configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ch_arb_weight_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ch_arb_weight_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ch_arb_weight_ch0`] module"]
pub type RX_CH_ARB_WEIGHT_CH0 = crate::Reg<rx_ch_arb_weight_ch0::RX_CH_ARB_WEIGHT_CH0_SPEC>;
#[doc = "RX channel 0 arbitration weight configuration register"]
pub mod rx_ch_arb_weight_ch0;
#[doc = "RX_ARB_WEIGHT_OPT_DIR_CH0 (rw) register accessor: RX channel 0 weight arbitration optimization enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_arb_weight_opt_dir_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_arb_weight_opt_dir_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_arb_weight_opt_dir_ch0`] module"]
pub type RX_ARB_WEIGHT_OPT_DIR_CH0 =
    crate::Reg<rx_arb_weight_opt_dir_ch0::RX_ARB_WEIGHT_OPT_DIR_CH0_SPEC>;
#[doc = "RX channel 0 weight arbitration optimization enable register"]
pub mod rx_arb_weight_opt_dir_ch0;
#[doc = "RX_CH_ARB_WEIGHT_CH1 (rw) register accessor: RX channel 1 arbitration weight configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ch_arb_weight_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ch_arb_weight_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ch_arb_weight_ch1`] module"]
pub type RX_CH_ARB_WEIGHT_CH1 = crate::Reg<rx_ch_arb_weight_ch1::RX_CH_ARB_WEIGHT_CH1_SPEC>;
#[doc = "RX channel 1 arbitration weight configuration register"]
pub mod rx_ch_arb_weight_ch1;
#[doc = "RX_ARB_WEIGHT_OPT_DIR_CH1 (rw) register accessor: RX channel 1 weight arbitration optimization enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_arb_weight_opt_dir_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_arb_weight_opt_dir_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_arb_weight_opt_dir_ch1`] module"]
pub type RX_ARB_WEIGHT_OPT_DIR_CH1 =
    crate::Reg<rx_arb_weight_opt_dir_ch1::RX_ARB_WEIGHT_OPT_DIR_CH1_SPEC>;
#[doc = "RX channel 1 weight arbitration optimization enable register"]
pub mod rx_arb_weight_opt_dir_ch1;
#[doc = "IN_LINK_ADDR_CH0 (rw) register accessor: Link list descriptor address configuration of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_link_addr_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_link_addr_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link_addr_ch0`] module"]
pub type IN_LINK_ADDR_CH0 = crate::Reg<in_link_addr_ch0::IN_LINK_ADDR_CH0_SPEC>;
#[doc = "Link list descriptor address configuration of RX channel 0"]
pub mod in_link_addr_ch0;
#[doc = "IN_LINK_ADDR_CH1 (rw) register accessor: Link list descriptor address configuration of RX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`in_link_addr_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_link_addr_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link_addr_ch1`] module"]
pub type IN_LINK_ADDR_CH1 = crate::Reg<in_link_addr_ch1::IN_LINK_ADDR_CH1_SPEC>;
#[doc = "Link list descriptor address configuration of RX channel 1"]
pub mod in_link_addr_ch1;
#[doc = "OUT_LINK_ADDR_CH0 (rw) register accessor: Link list descriptor address configuration of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_link_addr_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_link_addr_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link_addr_ch0`] module"]
pub type OUT_LINK_ADDR_CH0 = crate::Reg<out_link_addr_ch0::OUT_LINK_ADDR_CH0_SPEC>;
#[doc = "Link list descriptor address configuration of TX channel 0"]
pub mod out_link_addr_ch0;
#[doc = "OUT_LINK_ADDR_CH1 (rw) register accessor: Link list descriptor address configuration of TX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`out_link_addr_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_link_addr_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link_addr_ch1`] module"]
pub type OUT_LINK_ADDR_CH1 = crate::Reg<out_link_addr_ch1::OUT_LINK_ADDR_CH1_SPEC>;
#[doc = "Link list descriptor address configuration of TX channel 1"]
pub mod out_link_addr_ch1;
#[doc = "INTR_MEM_START_ADDR (rw) register accessor: Accessible address space start address configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mem_start_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mem_start_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mem_start_addr`] module"]
pub type INTR_MEM_START_ADDR = crate::Reg<intr_mem_start_addr::INTR_MEM_START_ADDR_SPEC>;
#[doc = "Accessible address space start address configuration register"]
pub mod intr_mem_start_addr;
#[doc = "INTR_MEM_END_ADDR (rw) register accessor: Accessible address space end address configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mem_end_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mem_end_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mem_end_addr`] module"]
pub type INTR_MEM_END_ADDR = crate::Reg<intr_mem_end_addr::INTR_MEM_END_ADDR_SPEC>;
#[doc = "Accessible address space end address configuration register"]
pub mod intr_mem_end_addr;
#[doc = "ARB_TIMEOUT_TX (rw) register accessor: TX arbitration timeout configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_timeout_tx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_timeout_tx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_timeout_tx`] module"]
pub type ARB_TIMEOUT_TX = crate::Reg<arb_timeout_tx::ARB_TIMEOUT_TX_SPEC>;
#[doc = "TX arbitration timeout configuration register"]
pub mod arb_timeout_tx;
#[doc = "ARB_TIMEOUT_RX (rw) register accessor: RX arbitration timeout configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_timeout_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_timeout_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_timeout_rx`] module"]
pub type ARB_TIMEOUT_RX = crate::Reg<arb_timeout_rx::ARB_TIMEOUT_RX_SPEC>;
#[doc = "RX arbitration timeout configuration register"]
pub mod arb_timeout_rx;
#[doc = "WEIGHT_EN_TX (rw) register accessor: TX weight arbitration enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`weight_en_tx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`weight_en_tx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@weight_en_tx`] module"]
pub type WEIGHT_EN_TX = crate::Reg<weight_en_tx::WEIGHT_EN_TX_SPEC>;
#[doc = "TX weight arbitration enable register"]
pub mod weight_en_tx;
#[doc = "WEIGHT_EN_RX (rw) register accessor: RX weight arbitration enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`weight_en_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`weight_en_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@weight_en_rx`] module"]
pub type WEIGHT_EN_RX = crate::Reg<weight_en_rx::WEIGHT_EN_RX_SPEC>;
#[doc = "RX weight arbitration enable register"]
pub mod weight_en_rx;
#[doc = "ARB_TIMEOUT (rw) register accessor: TX arbitration timeout configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_timeout`] module"]
pub type ARB_TIMEOUT = crate::Reg<arb_timeout::ARB_TIMEOUT_SPEC>;
#[doc = "TX arbitration timeout configuration register"]
pub mod arb_timeout;
#[doc = "WEIGHT_EN (rw) register accessor: TX weight arbitration enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`weight_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`weight_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@weight_en`] module"]
pub type WEIGHT_EN = crate::Reg<weight_en::WEIGHT_EN_SPEC>;
#[doc = "TX weight arbitration enable register"]
pub mod weight_en;
#[doc = "MODULE_CLK_EN (rw) register accessor: Module clock force on register\n\nYou can [`read`](crate::Reg::read) this register and get [`module_clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`module_clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@module_clk_en`] module"]
pub type MODULE_CLK_EN = crate::Reg<module_clk_en::MODULE_CLK_EN_SPEC>;
#[doc = "Module clock force on register"]
pub mod module_clk_en;
#[doc = "AHBINF_RESP_ERR_STATUS0 (r) register accessor: AHB response error status 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbinf_resp_err_status0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbinf_resp_err_status0`] module"]
pub type AHBINF_RESP_ERR_STATUS0 =
    crate::Reg<ahbinf_resp_err_status0::AHBINF_RESP_ERR_STATUS0_SPEC>;
#[doc = "AHB response error status 0 register"]
pub mod ahbinf_resp_err_status0;
#[doc = "AHBINF_RESP_ERR_STATUS1 (r) register accessor: AHB response error status 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbinf_resp_err_status1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbinf_resp_err_status1`] module"]
pub type AHBINF_RESP_ERR_STATUS1 =
    crate::Reg<ahbinf_resp_err_status1::AHBINF_RESP_ERR_STATUS1_SPEC>;
#[doc = "AHB response error status 1 register"]
pub mod ahbinf_resp_err_status1;
