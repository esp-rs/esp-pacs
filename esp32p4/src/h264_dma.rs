#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    out_conf0_ch0: OUT_CONF0_CH0,
    out_int_raw_ch0: OUT_INT_RAW_CH0,
    out_int_ena_ch0: OUT_INT_ENA_CH0,
    out_int_st_ch0: OUT_INT_ST_CH0,
    out_int_clr_ch0: OUT_INT_CLR_CH0,
    outfifo_status_ch0: OUTFIFO_STATUS_CH0,
    out_push_ch0: OUT_PUSH_CH0,
    out_link_conf_ch0: OUT_LINK_CONF_CH0,
    out_link_addr_ch0: OUT_LINK_ADDR_CH0,
    out_state_ch0: OUT_STATE_CH0,
    out_eof_des_addr_ch0: OUT_EOF_DES_ADDR_CH0,
    out_dscr_ch0: OUT_DSCR_CH0,
    out_dscr_bf0_ch0: OUT_DSCR_BF0_CH0,
    out_dscr_bf1_ch0: OUT_DSCR_BF1_CH0,
    _reserved14: [u8; 0x04],
    out_arb_ch0: OUT_ARB_CH0,
    out_ro_status_ch0: OUT_RO_STATUS_CH0,
    out_ro_pd_conf_ch0: OUT_RO_PD_CONF_CH0,
    _reserved17: [u8; 0x08],
    out_mode_enable_ch0: OUT_MODE_ENABLE_CH0,
    out_mode_yuv_ch0: OUT_MODE_YUV_CH0,
    _reserved19: [u8; 0x10],
    out_etm_conf_ch0: OUT_ETM_CONF_CH0,
    _reserved20: [u8; 0x04],
    out_buf_len_ch0: OUT_BUF_LEN_CH0,
    out_fifo_bcnt_ch0: OUT_FIFO_BCNT_CH0,
    out_push_bytecnt_ch0: OUT_PUSH_BYTECNT_CH0,
    out_xaddr_ch0: OUT_XADDR_CH0,
    _reserved24: [u8; 0x80],
    out_conf0_ch1: OUT_CONF0_CH1,
    out_int_raw_ch1: OUT_INT_RAW_CH1,
    out_int_ena_ch1: OUT_INT_ENA_CH1,
    out_int_st_ch1: OUT_INT_ST_CH1,
    out_int_clr_ch1: OUT_INT_CLR_CH1,
    outfifo_status_ch1: OUTFIFO_STATUS_CH1,
    out_push_ch1: OUT_PUSH_CH1,
    out_link_conf_ch1: OUT_LINK_CONF_CH1,
    out_link_addr_ch1: OUT_LINK_ADDR_CH1,
    out_state_ch1: OUT_STATE_CH1,
    out_eof_des_addr_ch1: OUT_EOF_DES_ADDR_CH1,
    out_dscr_ch1: OUT_DSCR_CH1,
    out_dscr_bf0_ch1: OUT_DSCR_BF0_CH1,
    out_dscr_bf1_ch1: OUT_DSCR_BF1_CH1,
    _reserved38: [u8; 0x04],
    out_arb_ch1: OUT_ARB_CH1,
    _reserved39: [u8; 0x28],
    out_etm_conf_ch1: OUT_ETM_CONF_CH1,
    _reserved40: [u8; 0x04],
    out_buf_len_ch1: OUT_BUF_LEN_CH1,
    out_fifo_bcnt_ch1: OUT_FIFO_BCNT_CH1,
    out_push_bytecnt_ch1: OUT_PUSH_BYTECNT_CH1,
    out_xaddr_ch1: OUT_XADDR_CH1,
    _reserved44: [u8; 0x80],
    out_conf0_ch2: OUT_CONF0_CH2,
    out_int_raw_ch2: OUT_INT_RAW_CH2,
    out_int_ena_ch2: OUT_INT_ENA_CH2,
    out_int_st_ch2: OUT_INT_ST_CH2,
    out_int_clr_ch2: OUT_INT_CLR_CH2,
    outfifo_status_ch2: OUTFIFO_STATUS_CH2,
    out_push_ch2: OUT_PUSH_CH2,
    out_link_conf_ch2: OUT_LINK_CONF_CH2,
    out_link_addr_ch2: OUT_LINK_ADDR_CH2,
    out_state_ch2: OUT_STATE_CH2,
    out_eof_des_addr_ch2: OUT_EOF_DES_ADDR_CH2,
    out_dscr_ch2: OUT_DSCR_CH2,
    out_dscr_bf0_ch2: OUT_DSCR_BF0_CH2,
    out_dscr_bf1_ch2: OUT_DSCR_BF1_CH2,
    _reserved58: [u8; 0x04],
    out_arb_ch2: OUT_ARB_CH2,
    _reserved59: [u8; 0x28],
    out_etm_conf_ch2: OUT_ETM_CONF_CH2,
    _reserved60: [u8; 0x04],
    out_buf_len_ch2: OUT_BUF_LEN_CH2,
    out_fifo_bcnt_ch2: OUT_FIFO_BCNT_CH2,
    out_push_bytecnt_ch2: OUT_PUSH_BYTECNT_CH2,
    out_xaddr_ch2: OUT_XADDR_CH2,
    _reserved64: [u8; 0x80],
    out_conf0_ch3: OUT_CONF0_CH3,
    out_int_raw_ch3: OUT_INT_RAW_CH3,
    out_int_ena_ch3: OUT_INT_ENA_CH3,
    out_int_st_ch3: OUT_INT_ST_CH3,
    out_int_clr_ch3: OUT_INT_CLR_CH3,
    outfifo_status_ch3: OUTFIFO_STATUS_CH3,
    out_push_ch3: OUT_PUSH_CH3,
    out_link_conf_ch3: OUT_LINK_CONF_CH3,
    out_link_addr_ch3: OUT_LINK_ADDR_CH3,
    out_state_ch3: OUT_STATE_CH3,
    out_eof_des_addr_ch3: OUT_EOF_DES_ADDR_CH3,
    out_dscr_ch3: OUT_DSCR_CH3,
    out_dscr_bf0_ch3: OUT_DSCR_BF0_CH3,
    out_dscr_bf1_ch3: OUT_DSCR_BF1_CH3,
    _reserved78: [u8; 0x04],
    out_arb_ch3: OUT_ARB_CH3,
    _reserved79: [u8; 0x28],
    out_etm_conf_ch3: OUT_ETM_CONF_CH3,
    _reserved80: [u8; 0x04],
    out_buf_len_ch3: OUT_BUF_LEN_CH3,
    out_fifo_bcnt_ch3: OUT_FIFO_BCNT_CH3,
    out_push_bytecnt_ch3: OUT_PUSH_BYTECNT_CH3,
    out_xaddr_ch3: OUT_XADDR_CH3,
    out_block_buf_len_ch3: OUT_BLOCK_BUF_LEN_CH3,
    _reserved85: [u8; 0x7c],
    out_conf0_ch4: OUT_CONF0_CH4,
    out_int_raw_ch4: OUT_INT_RAW_CH4,
    out_int_ena_ch4: OUT_INT_ENA_CH4,
    out_int_st_ch4: OUT_INT_ST_CH4,
    out_int_clr_ch4: OUT_INT_CLR_CH4,
    outfifo_status_ch4: OUTFIFO_STATUS_CH4,
    out_push_ch4: OUT_PUSH_CH4,
    out_link_conf_ch4: OUT_LINK_CONF_CH4,
    out_link_addr_ch4: OUT_LINK_ADDR_CH4,
    out_state_ch4: OUT_STATE_CH4,
    out_eof_des_addr_ch4: OUT_EOF_DES_ADDR_CH4,
    out_dscr_ch4: OUT_DSCR_CH4,
    out_dscr_bf0_ch4: OUT_DSCR_BF0_CH4,
    out_dscr_bf1_ch4: OUT_DSCR_BF1_CH4,
    _reserved99: [u8; 0x04],
    out_arb_ch4: OUT_ARB_CH4,
    _reserved100: [u8; 0x28],
    out_etm_conf_ch4: OUT_ETM_CONF_CH4,
    _reserved101: [u8; 0x04],
    out_buf_len_ch4: OUT_BUF_LEN_CH4,
    out_fifo_bcnt_ch4: OUT_FIFO_BCNT_CH4,
    out_push_bytecnt_ch4: OUT_PUSH_BYTECNT_CH4,
    out_xaddr_ch4: OUT_XADDR_CH4,
    out_block_buf_len_ch4: OUT_BLOCK_BUF_LEN_CH4,
    _reserved106: [u8; 0x7c],
    in_conf0_ch0: IN_CONF0_CH0,
    in_int_raw_ch0: IN_INT_RAW_CH0,
    in_int_ena_ch0: IN_INT_ENA_CH0,
    in_int_st_ch0: IN_INT_ST_CH0,
    in_int_clr_ch0: IN_INT_CLR_CH0,
    infifo_status_ch0: INFIFO_STATUS_CH0,
    in_pop_ch0: IN_POP_CH0,
    in_link_conf_ch0: IN_LINK_CONF_CH0,
    in_link_addr_ch0: IN_LINK_ADDR_CH0,
    in_state_ch0: IN_STATE_CH0,
    in_suc_eof_des_addr_ch0: IN_SUC_EOF_DES_ADDR_CH0,
    in_err_eof_des_addr_ch0: IN_ERR_EOF_DES_ADDR_CH0,
    in_dscr_ch0: IN_DSCR_CH0,
    in_dscr_bf0_ch0: IN_DSCR_BF0_CH0,
    in_dscr_bf1_ch0: IN_DSCR_BF1_CH0,
    _reserved121: [u8; 0x04],
    in_arb_ch0: IN_ARB_CH0,
    _reserved122: [u8; 0x04],
    in_ro_pd_conf_ch0: IN_RO_PD_CONF_CH0,
    _reserved123: [u8; 0x20],
    in_etm_conf_ch0: IN_ETM_CONF_CH0,
    _reserved124: [u8; 0x10],
    in_fifo_cnt_ch0: IN_FIFO_CNT_CH0,
    in_pop_data_cnt_ch0: IN_POP_DATA_CNT_CH0,
    in_xaddr_ch0: IN_XADDR_CH0,
    in_buf_hb_rcv_ch0: IN_BUF_HB_RCV_CH0,
    _reserved128: [u8; 0x70],
    in_conf0_ch1: IN_CONF0_CH1,
    in_int_raw_ch1: IN_INT_RAW_CH1,
    in_int_ena_ch1: IN_INT_ENA_CH1,
    in_int_st_ch1: IN_INT_ST_CH1,
    in_int_clr_ch1: IN_INT_CLR_CH1,
    infifo_status_ch1: INFIFO_STATUS_CH1,
    in_pop_ch1: IN_POP_CH1,
    in_link_conf_ch1: IN_LINK_CONF_CH1,
    in_link_addr_ch1: IN_LINK_ADDR_CH1,
    in_state_ch1: IN_STATE_CH1,
    in_suc_eof_des_addr_ch1: IN_SUC_EOF_DES_ADDR_CH1,
    in_err_eof_des_addr_ch1: IN_ERR_EOF_DES_ADDR_CH1,
    in_dscr_ch1: IN_DSCR_CH1,
    in_dscr_bf0_ch1: IN_DSCR_BF0_CH1,
    in_dscr_bf1_ch1: IN_DSCR_BF1_CH1,
    _reserved143: [u8; 0x04],
    in_arb_ch1: IN_ARB_CH1,
    _reserved144: [u8; 0x04],
    in_etm_conf_ch1: IN_ETM_CONF_CH1,
    _reserved145: [u8; 0x34],
    in_fifo_cnt_ch1: IN_FIFO_CNT_CH1,
    in_pop_data_cnt_ch1: IN_POP_DATA_CNT_CH1,
    in_xaddr_ch1: IN_XADDR_CH1,
    in_buf_hb_rcv_ch1: IN_BUF_HB_RCV_CH1,
    _reserved149: [u8; 0x70],
    in_conf0_ch2: IN_CONF0_CH2,
    in_int_raw_ch2: IN_INT_RAW_CH2,
    in_int_ena_ch2: IN_INT_ENA_CH2,
    in_int_st_ch2: IN_INT_ST_CH2,
    in_int_clr_ch2: IN_INT_CLR_CH2,
    infifo_status_ch2: INFIFO_STATUS_CH2,
    in_pop_ch2: IN_POP_CH2,
    in_link_conf_ch2: IN_LINK_CONF_CH2,
    in_link_addr_ch2: IN_LINK_ADDR_CH2,
    in_state_ch2: IN_STATE_CH2,
    in_suc_eof_des_addr_ch2: IN_SUC_EOF_DES_ADDR_CH2,
    in_err_eof_des_addr_ch2: IN_ERR_EOF_DES_ADDR_CH2,
    in_dscr_ch2: IN_DSCR_CH2,
    in_dscr_bf0_ch2: IN_DSCR_BF0_CH2,
    in_dscr_bf1_ch2: IN_DSCR_BF1_CH2,
    _reserved164: [u8; 0x04],
    in_arb_ch2: IN_ARB_CH2,
    _reserved165: [u8; 0x04],
    in_etm_conf_ch2: IN_ETM_CONF_CH2,
    _reserved166: [u8; 0x34],
    in_fifo_cnt_ch2: IN_FIFO_CNT_CH2,
    in_pop_data_cnt_ch2: IN_POP_DATA_CNT_CH2,
    in_xaddr_ch2: IN_XADDR_CH2,
    in_buf_hb_rcv_ch2: IN_BUF_HB_RCV_CH2,
    _reserved170: [u8; 0x70],
    in_conf0_ch3: IN_CONF0_CH3,
    in_int_raw_ch3: IN_INT_RAW_CH3,
    in_int_ena_ch3: IN_INT_ENA_CH3,
    in_int_st_ch3: IN_INT_ST_CH3,
    in_int_clr_ch3: IN_INT_CLR_CH3,
    infifo_status_ch3: INFIFO_STATUS_CH3,
    in_pop_ch3: IN_POP_CH3,
    in_link_conf_ch3: IN_LINK_CONF_CH3,
    in_link_addr_ch3: IN_LINK_ADDR_CH3,
    in_state_ch3: IN_STATE_CH3,
    in_suc_eof_des_addr_ch3: IN_SUC_EOF_DES_ADDR_CH3,
    in_err_eof_des_addr_ch3: IN_ERR_EOF_DES_ADDR_CH3,
    in_dscr_ch3: IN_DSCR_CH3,
    in_dscr_bf0_ch3: IN_DSCR_BF0_CH3,
    in_dscr_bf1_ch3: IN_DSCR_BF1_CH3,
    _reserved185: [u8; 0x04],
    in_arb_ch3: IN_ARB_CH3,
    _reserved186: [u8; 0x04],
    in_etm_conf_ch3: IN_ETM_CONF_CH3,
    _reserved187: [u8; 0x34],
    in_fifo_cnt_ch3: IN_FIFO_CNT_CH3,
    in_pop_data_cnt_ch3: IN_POP_DATA_CNT_CH3,
    in_xaddr_ch3: IN_XADDR_CH3,
    in_buf_hb_rcv_ch3: IN_BUF_HB_RCV_CH3,
    _reserved191: [u8; 0x70],
    in_conf0_ch4: IN_CONF0_CH4,
    in_int_raw_ch4: IN_INT_RAW_CH4,
    in_int_ena_ch4: IN_INT_ENA_CH4,
    in_int_st_ch4: IN_INT_ST_CH4,
    in_int_clr_ch4: IN_INT_CLR_CH4,
    infifo_status_ch4: INFIFO_STATUS_CH4,
    in_pop_ch4: IN_POP_CH4,
    in_link_conf_ch4: IN_LINK_CONF_CH4,
    in_link_addr_ch4: IN_LINK_ADDR_CH4,
    in_state_ch4: IN_STATE_CH4,
    in_suc_eof_des_addr_ch4: IN_SUC_EOF_DES_ADDR_CH4,
    in_err_eof_des_addr_ch4: IN_ERR_EOF_DES_ADDR_CH4,
    in_dscr_ch4: IN_DSCR_CH4,
    in_dscr_bf0_ch4: IN_DSCR_BF0_CH4,
    in_dscr_bf1_ch4: IN_DSCR_BF1_CH4,
    _reserved206: [u8; 0x04],
    in_arb_ch4: IN_ARB_CH4,
    _reserved207: [u8; 0x04],
    in_etm_conf_ch4: IN_ETM_CONF_CH4,
    _reserved208: [u8; 0x34],
    in_fifo_cnt_ch4: IN_FIFO_CNT_CH4,
    in_pop_data_cnt_ch4: IN_POP_DATA_CNT_CH4,
    in_xaddr_ch4: IN_XADDR_CH4,
    in_buf_hb_rcv_ch4: IN_BUF_HB_RCV_CH4,
    _reserved212: [u8; 0x70],
    in_conf0_ch5: IN_CONF0_CH5,
    in_conf1_ch5: IN_CONF1_CH5,
    in_conf2_ch5: IN_CONF2_CH5,
    in_conf3_ch5: IN_CONF3_CH5,
    in_int_raw_ch5: IN_INT_RAW_CH5,
    in_int_ena_ch5: IN_INT_ENA_CH5,
    in_int_st_ch5: IN_INT_ST_CH5,
    in_int_clr_ch5: IN_INT_CLR_CH5,
    infifo_status_ch5: INFIFO_STATUS_CH5,
    in_pop_ch5: IN_POP_CH5,
    in_state_ch5: IN_STATE_CH5,
    _reserved223: [u8; 0x14],
    in_arb_ch5: IN_ARB_CH5,
    _reserved224: [u8; 0x3c],
    in_fifo_cnt_ch5: IN_FIFO_CNT_CH5,
    in_pop_data_cnt_ch5: IN_POP_DATA_CNT_CH5,
    in_xaddr_ch5: IN_XADDR_CH5,
    in_buf_hb_rcv_ch5: IN_BUF_HB_RCV_CH5,
    _reserved228: [u8; 0x70],
    inter_axi_err: INTER_AXI_ERR,
    exter_axi_err: EXTER_AXI_ERR,
    rst_conf: RST_CONF,
    inter_mem_start_addr0: INTER_MEM_START_ADDR0,
    inter_mem_end_addr0: INTER_MEM_END_ADDR0,
    inter_mem_start_addr1: INTER_MEM_START_ADDR1,
    inter_mem_end_addr1: INTER_MEM_END_ADDR1,
    _reserved235: [u8; 0x04],
    exter_mem_start_addr0: EXTER_MEM_START_ADDR0,
    exter_mem_end_addr0: EXTER_MEM_END_ADDR0,
    exter_mem_start_addr1: EXTER_MEM_START_ADDR1,
    exter_mem_end_addr1: EXTER_MEM_END_ADDR1,
    out_arb_config: OUT_ARB_CONFIG,
    in_arb_config: IN_ARB_CONFIG,
    _reserved241: [u8; 0x04],
    date: DATE,
    _reserved242: [u8; 0x10],
    counter_rst: COUNTER_RST,
    rx_ch0_counter: RX_CH0_COUNTER,
    rx_ch1_counter: RX_CH1_COUNTER,
    rx_ch2_counter: RX_CH2_COUNTER,
    rx_ch5_counter: RX_CH5_COUNTER,
}
impl RegisterBlock {
    #[doc = "0x00 - TX CH0 config0 register"]
    #[inline(always)]
    pub const fn out_conf0_ch0(&self) -> &OUT_CONF0_CH0 {
        &self.out_conf0_ch0
    }
    #[doc = "0x04 - TX CH0 interrupt raw register"]
    #[inline(always)]
    pub const fn out_int_raw_ch0(&self) -> &OUT_INT_RAW_CH0 {
        &self.out_int_raw_ch0
    }
    #[doc = "0x08 - TX CH0 interrupt ena register"]
    #[inline(always)]
    pub const fn out_int_ena_ch0(&self) -> &OUT_INT_ENA_CH0 {
        &self.out_int_ena_ch0
    }
    #[doc = "0x0c - TX CH0 interrupt st register"]
    #[inline(always)]
    pub const fn out_int_st_ch0(&self) -> &OUT_INT_ST_CH0 {
        &self.out_int_st_ch0
    }
    #[doc = "0x10 - TX CH0 interrupt clr register"]
    #[inline(always)]
    pub const fn out_int_clr_ch0(&self) -> &OUT_INT_CLR_CH0 {
        &self.out_int_clr_ch0
    }
    #[doc = "0x14 - TX CH0 outfifo status register"]
    #[inline(always)]
    pub const fn outfifo_status_ch0(&self) -> &OUTFIFO_STATUS_CH0 {
        &self.outfifo_status_ch0
    }
    #[doc = "0x18 - TX CH0 outfifo push register"]
    #[inline(always)]
    pub const fn out_push_ch0(&self) -> &OUT_PUSH_CH0 {
        &self.out_push_ch0
    }
    #[doc = "0x1c - TX CH0 out_link dscr ctrl register"]
    #[inline(always)]
    pub const fn out_link_conf_ch0(&self) -> &OUT_LINK_CONF_CH0 {
        &self.out_link_conf_ch0
    }
    #[doc = "0x20 - TX CH0 out_link dscr addr register"]
    #[inline(always)]
    pub const fn out_link_addr_ch0(&self) -> &OUT_LINK_ADDR_CH0 {
        &self.out_link_addr_ch0
    }
    #[doc = "0x24 - TX CH0 state register"]
    #[inline(always)]
    pub const fn out_state_ch0(&self) -> &OUT_STATE_CH0 {
        &self.out_state_ch0
    }
    #[doc = "0x28 - TX CH0 eof des addr register"]
    #[inline(always)]
    pub const fn out_eof_des_addr_ch0(&self) -> &OUT_EOF_DES_ADDR_CH0 {
        &self.out_eof_des_addr_ch0
    }
    #[doc = "0x2c - TX CH0 next dscr addr register"]
    #[inline(always)]
    pub const fn out_dscr_ch0(&self) -> &OUT_DSCR_CH0 {
        &self.out_dscr_ch0
    }
    #[doc = "0x30 - TX CH0 last dscr addr register"]
    #[inline(always)]
    pub const fn out_dscr_bf0_ch0(&self) -> &OUT_DSCR_BF0_CH0 {
        &self.out_dscr_bf0_ch0
    }
    #[doc = "0x34 - TX CH0 second-to-last dscr addr register"]
    #[inline(always)]
    pub const fn out_dscr_bf1_ch0(&self) -> &OUT_DSCR_BF1_CH0 {
        &self.out_dscr_bf1_ch0
    }
    #[doc = "0x3c - TX CH0 arb register"]
    #[inline(always)]
    pub const fn out_arb_ch0(&self) -> &OUT_ARB_CH0 {
        &self.out_arb_ch0
    }
    #[doc = "0x40 - TX CH0 reorder status register"]
    #[inline(always)]
    pub const fn out_ro_status_ch0(&self) -> &OUT_RO_STATUS_CH0 {
        &self.out_ro_status_ch0
    }
    #[doc = "0x44 - TX CH0 reorder power config register"]
    #[inline(always)]
    pub const fn out_ro_pd_conf_ch0(&self) -> &OUT_RO_PD_CONF_CH0 {
        &self.out_ro_pd_conf_ch0
    }
    #[doc = "0x50 - tx CH0 mode enable register"]
    #[inline(always)]
    pub const fn out_mode_enable_ch0(&self) -> &OUT_MODE_ENABLE_CH0 {
        &self.out_mode_enable_ch0
    }
    #[doc = "0x54 - tx CH0 test mode yuv value register"]
    #[inline(always)]
    pub const fn out_mode_yuv_ch0(&self) -> &OUT_MODE_YUV_CH0 {
        &self.out_mode_yuv_ch0
    }
    #[doc = "0x68 - TX CH0 ETM config register"]
    #[inline(always)]
    pub const fn out_etm_conf_ch0(&self) -> &OUT_ETM_CONF_CH0 {
        &self.out_etm_conf_ch0
    }
    #[doc = "0x70 - tx CH0 buf len register"]
    #[inline(always)]
    pub const fn out_buf_len_ch0(&self) -> &OUT_BUF_LEN_CH0 {
        &self.out_buf_len_ch0
    }
    #[doc = "0x74 - tx CH0 fifo byte cnt register"]
    #[inline(always)]
    pub const fn out_fifo_bcnt_ch0(&self) -> &OUT_FIFO_BCNT_CH0 {
        &self.out_fifo_bcnt_ch0
    }
    #[doc = "0x78 - tx CH0 push byte cnt register"]
    #[inline(always)]
    pub const fn out_push_bytecnt_ch0(&self) -> &OUT_PUSH_BYTECNT_CH0 {
        &self.out_push_bytecnt_ch0
    }
    #[doc = "0x7c - tx CH0 xaddr register"]
    #[inline(always)]
    pub const fn out_xaddr_ch0(&self) -> &OUT_XADDR_CH0 {
        &self.out_xaddr_ch0
    }
    #[doc = "0x100 - TX CH1 config0 register"]
    #[inline(always)]
    pub const fn out_conf0_ch1(&self) -> &OUT_CONF0_CH1 {
        &self.out_conf0_ch1
    }
    #[doc = "0x104 - TX CH1 interrupt raw register"]
    #[inline(always)]
    pub const fn out_int_raw_ch1(&self) -> &OUT_INT_RAW_CH1 {
        &self.out_int_raw_ch1
    }
    #[doc = "0x108 - TX CH1 interrupt ena register"]
    #[inline(always)]
    pub const fn out_int_ena_ch1(&self) -> &OUT_INT_ENA_CH1 {
        &self.out_int_ena_ch1
    }
    #[doc = "0x10c - TX CH1 interrupt st register"]
    #[inline(always)]
    pub const fn out_int_st_ch1(&self) -> &OUT_INT_ST_CH1 {
        &self.out_int_st_ch1
    }
    #[doc = "0x110 - TX CH1 interrupt clr register"]
    #[inline(always)]
    pub const fn out_int_clr_ch1(&self) -> &OUT_INT_CLR_CH1 {
        &self.out_int_clr_ch1
    }
    #[doc = "0x114 - TX CH1 outfifo status register"]
    #[inline(always)]
    pub const fn outfifo_status_ch1(&self) -> &OUTFIFO_STATUS_CH1 {
        &self.outfifo_status_ch1
    }
    #[doc = "0x118 - TX CH1 outfifo push register"]
    #[inline(always)]
    pub const fn out_push_ch1(&self) -> &OUT_PUSH_CH1 {
        &self.out_push_ch1
    }
    #[doc = "0x11c - TX CH1 out_link dscr ctrl register"]
    #[inline(always)]
    pub const fn out_link_conf_ch1(&self) -> &OUT_LINK_CONF_CH1 {
        &self.out_link_conf_ch1
    }
    #[doc = "0x120 - TX CH1 out_link dscr addr register"]
    #[inline(always)]
    pub const fn out_link_addr_ch1(&self) -> &OUT_LINK_ADDR_CH1 {
        &self.out_link_addr_ch1
    }
    #[doc = "0x124 - TX CH1 state register"]
    #[inline(always)]
    pub const fn out_state_ch1(&self) -> &OUT_STATE_CH1 {
        &self.out_state_ch1
    }
    #[doc = "0x128 - TX CH1 eof des addr register"]
    #[inline(always)]
    pub const fn out_eof_des_addr_ch1(&self) -> &OUT_EOF_DES_ADDR_CH1 {
        &self.out_eof_des_addr_ch1
    }
    #[doc = "0x12c - TX CH1 next dscr addr register"]
    #[inline(always)]
    pub const fn out_dscr_ch1(&self) -> &OUT_DSCR_CH1 {
        &self.out_dscr_ch1
    }
    #[doc = "0x130 - TX CH1 last dscr addr register"]
    #[inline(always)]
    pub const fn out_dscr_bf0_ch1(&self) -> &OUT_DSCR_BF0_CH1 {
        &self.out_dscr_bf0_ch1
    }
    #[doc = "0x134 - TX CH1 second-to-last dscr addr register"]
    #[inline(always)]
    pub const fn out_dscr_bf1_ch1(&self) -> &OUT_DSCR_BF1_CH1 {
        &self.out_dscr_bf1_ch1
    }
    #[doc = "0x13c - TX CH1 arb register"]
    #[inline(always)]
    pub const fn out_arb_ch1(&self) -> &OUT_ARB_CH1 {
        &self.out_arb_ch1
    }
    #[doc = "0x168 - TX CH1 ETM config register"]
    #[inline(always)]
    pub const fn out_etm_conf_ch1(&self) -> &OUT_ETM_CONF_CH1 {
        &self.out_etm_conf_ch1
    }
    #[doc = "0x170 - tx CH1 buf len register"]
    #[inline(always)]
    pub const fn out_buf_len_ch1(&self) -> &OUT_BUF_LEN_CH1 {
        &self.out_buf_len_ch1
    }
    #[doc = "0x174 - tx CH1 fifo byte cnt register"]
    #[inline(always)]
    pub const fn out_fifo_bcnt_ch1(&self) -> &OUT_FIFO_BCNT_CH1 {
        &self.out_fifo_bcnt_ch1
    }
    #[doc = "0x178 - tx CH1 push byte cnt register"]
    #[inline(always)]
    pub const fn out_push_bytecnt_ch1(&self) -> &OUT_PUSH_BYTECNT_CH1 {
        &self.out_push_bytecnt_ch1
    }
    #[doc = "0x17c - tx CH1 xaddr register"]
    #[inline(always)]
    pub const fn out_xaddr_ch1(&self) -> &OUT_XADDR_CH1 {
        &self.out_xaddr_ch1
    }
    #[doc = "0x200 - TX CH2 config0 register"]
    #[inline(always)]
    pub const fn out_conf0_ch2(&self) -> &OUT_CONF0_CH2 {
        &self.out_conf0_ch2
    }
    #[doc = "0x204 - TX CH2 interrupt raw register"]
    #[inline(always)]
    pub const fn out_int_raw_ch2(&self) -> &OUT_INT_RAW_CH2 {
        &self.out_int_raw_ch2
    }
    #[doc = "0x208 - TX CH2 interrupt ena register"]
    #[inline(always)]
    pub const fn out_int_ena_ch2(&self) -> &OUT_INT_ENA_CH2 {
        &self.out_int_ena_ch2
    }
    #[doc = "0x20c - TX CH2 interrupt st register"]
    #[inline(always)]
    pub const fn out_int_st_ch2(&self) -> &OUT_INT_ST_CH2 {
        &self.out_int_st_ch2
    }
    #[doc = "0x210 - TX CH2 interrupt clr register"]
    #[inline(always)]
    pub const fn out_int_clr_ch2(&self) -> &OUT_INT_CLR_CH2 {
        &self.out_int_clr_ch2
    }
    #[doc = "0x214 - TX CH2 outfifo status register"]
    #[inline(always)]
    pub const fn outfifo_status_ch2(&self) -> &OUTFIFO_STATUS_CH2 {
        &self.outfifo_status_ch2
    }
    #[doc = "0x218 - TX CH2 outfifo push register"]
    #[inline(always)]
    pub const fn out_push_ch2(&self) -> &OUT_PUSH_CH2 {
        &self.out_push_ch2
    }
    #[doc = "0x21c - TX CH2 out_link dscr ctrl register"]
    #[inline(always)]
    pub const fn out_link_conf_ch2(&self) -> &OUT_LINK_CONF_CH2 {
        &self.out_link_conf_ch2
    }
    #[doc = "0x220 - TX CH2 out_link dscr addr register"]
    #[inline(always)]
    pub const fn out_link_addr_ch2(&self) -> &OUT_LINK_ADDR_CH2 {
        &self.out_link_addr_ch2
    }
    #[doc = "0x224 - TX CH2 state register"]
    #[inline(always)]
    pub const fn out_state_ch2(&self) -> &OUT_STATE_CH2 {
        &self.out_state_ch2
    }
    #[doc = "0x228 - TX CH2 eof des addr register"]
    #[inline(always)]
    pub const fn out_eof_des_addr_ch2(&self) -> &OUT_EOF_DES_ADDR_CH2 {
        &self.out_eof_des_addr_ch2
    }
    #[doc = "0x22c - TX CH2 next dscr addr register"]
    #[inline(always)]
    pub const fn out_dscr_ch2(&self) -> &OUT_DSCR_CH2 {
        &self.out_dscr_ch2
    }
    #[doc = "0x230 - TX CH2 last dscr addr register"]
    #[inline(always)]
    pub const fn out_dscr_bf0_ch2(&self) -> &OUT_DSCR_BF0_CH2 {
        &self.out_dscr_bf0_ch2
    }
    #[doc = "0x234 - TX CH2 second-to-last dscr addr register"]
    #[inline(always)]
    pub const fn out_dscr_bf1_ch2(&self) -> &OUT_DSCR_BF1_CH2 {
        &self.out_dscr_bf1_ch2
    }
    #[doc = "0x23c - TX CH2 arb register"]
    #[inline(always)]
    pub const fn out_arb_ch2(&self) -> &OUT_ARB_CH2 {
        &self.out_arb_ch2
    }
    #[doc = "0x268 - TX CH2 ETM config register"]
    #[inline(always)]
    pub const fn out_etm_conf_ch2(&self) -> &OUT_ETM_CONF_CH2 {
        &self.out_etm_conf_ch2
    }
    #[doc = "0x270 - tx CH2 buf len register"]
    #[inline(always)]
    pub const fn out_buf_len_ch2(&self) -> &OUT_BUF_LEN_CH2 {
        &self.out_buf_len_ch2
    }
    #[doc = "0x274 - tx CH2 fifo byte cnt register"]
    #[inline(always)]
    pub const fn out_fifo_bcnt_ch2(&self) -> &OUT_FIFO_BCNT_CH2 {
        &self.out_fifo_bcnt_ch2
    }
    #[doc = "0x278 - tx CH2 push byte cnt register"]
    #[inline(always)]
    pub const fn out_push_bytecnt_ch2(&self) -> &OUT_PUSH_BYTECNT_CH2 {
        &self.out_push_bytecnt_ch2
    }
    #[doc = "0x27c - tx CH2 xaddr register"]
    #[inline(always)]
    pub const fn out_xaddr_ch2(&self) -> &OUT_XADDR_CH2 {
        &self.out_xaddr_ch2
    }
    #[doc = "0x300 - TX CH3 config0 register"]
    #[inline(always)]
    pub const fn out_conf0_ch3(&self) -> &OUT_CONF0_CH3 {
        &self.out_conf0_ch3
    }
    #[doc = "0x304 - TX CH3 interrupt raw register"]
    #[inline(always)]
    pub const fn out_int_raw_ch3(&self) -> &OUT_INT_RAW_CH3 {
        &self.out_int_raw_ch3
    }
    #[doc = "0x308 - TX CH3 interrupt ena register"]
    #[inline(always)]
    pub const fn out_int_ena_ch3(&self) -> &OUT_INT_ENA_CH3 {
        &self.out_int_ena_ch3
    }
    #[doc = "0x30c - TX CH3 interrupt st register"]
    #[inline(always)]
    pub const fn out_int_st_ch3(&self) -> &OUT_INT_ST_CH3 {
        &self.out_int_st_ch3
    }
    #[doc = "0x310 - TX CH3 interrupt clr register"]
    #[inline(always)]
    pub const fn out_int_clr_ch3(&self) -> &OUT_INT_CLR_CH3 {
        &self.out_int_clr_ch3
    }
    #[doc = "0x314 - TX CH3 outfifo status register"]
    #[inline(always)]
    pub const fn outfifo_status_ch3(&self) -> &OUTFIFO_STATUS_CH3 {
        &self.outfifo_status_ch3
    }
    #[doc = "0x318 - TX CH3 outfifo push register"]
    #[inline(always)]
    pub const fn out_push_ch3(&self) -> &OUT_PUSH_CH3 {
        &self.out_push_ch3
    }
    #[doc = "0x31c - TX CH3 out_link dscr ctrl register"]
    #[inline(always)]
    pub const fn out_link_conf_ch3(&self) -> &OUT_LINK_CONF_CH3 {
        &self.out_link_conf_ch3
    }
    #[doc = "0x320 - TX CH3 out_link dscr addr register"]
    #[inline(always)]
    pub const fn out_link_addr_ch3(&self) -> &OUT_LINK_ADDR_CH3 {
        &self.out_link_addr_ch3
    }
    #[doc = "0x324 - TX CH3 state register"]
    #[inline(always)]
    pub const fn out_state_ch3(&self) -> &OUT_STATE_CH3 {
        &self.out_state_ch3
    }
    #[doc = "0x328 - TX CH3 eof des addr register"]
    #[inline(always)]
    pub const fn out_eof_des_addr_ch3(&self) -> &OUT_EOF_DES_ADDR_CH3 {
        &self.out_eof_des_addr_ch3
    }
    #[doc = "0x32c - TX CH3 next dscr addr register"]
    #[inline(always)]
    pub const fn out_dscr_ch3(&self) -> &OUT_DSCR_CH3 {
        &self.out_dscr_ch3
    }
    #[doc = "0x330 - TX CH3 last dscr addr register"]
    #[inline(always)]
    pub const fn out_dscr_bf0_ch3(&self) -> &OUT_DSCR_BF0_CH3 {
        &self.out_dscr_bf0_ch3
    }
    #[doc = "0x334 - TX CH3 second-to-last dscr addr register"]
    #[inline(always)]
    pub const fn out_dscr_bf1_ch3(&self) -> &OUT_DSCR_BF1_CH3 {
        &self.out_dscr_bf1_ch3
    }
    #[doc = "0x33c - TX CH3 arb register"]
    #[inline(always)]
    pub const fn out_arb_ch3(&self) -> &OUT_ARB_CH3 {
        &self.out_arb_ch3
    }
    #[doc = "0x368 - TX CH3 ETM config register"]
    #[inline(always)]
    pub const fn out_etm_conf_ch3(&self) -> &OUT_ETM_CONF_CH3 {
        &self.out_etm_conf_ch3
    }
    #[doc = "0x370 - tx CH3 buf len register"]
    #[inline(always)]
    pub const fn out_buf_len_ch3(&self) -> &OUT_BUF_LEN_CH3 {
        &self.out_buf_len_ch3
    }
    #[doc = "0x374 - tx CH3 fifo byte cnt register"]
    #[inline(always)]
    pub const fn out_fifo_bcnt_ch3(&self) -> &OUT_FIFO_BCNT_CH3 {
        &self.out_fifo_bcnt_ch3
    }
    #[doc = "0x378 - tx CH3 push byte cnt register"]
    #[inline(always)]
    pub const fn out_push_bytecnt_ch3(&self) -> &OUT_PUSH_BYTECNT_CH3 {
        &self.out_push_bytecnt_ch3
    }
    #[doc = "0x37c - tx CH3 xaddr register"]
    #[inline(always)]
    pub const fn out_xaddr_ch3(&self) -> &OUT_XADDR_CH3 {
        &self.out_xaddr_ch3
    }
    #[doc = "0x380 - tx CH3 block buf len register"]
    #[inline(always)]
    pub const fn out_block_buf_len_ch3(&self) -> &OUT_BLOCK_BUF_LEN_CH3 {
        &self.out_block_buf_len_ch3
    }
    #[doc = "0x400 - TX CH4 config0 register"]
    #[inline(always)]
    pub const fn out_conf0_ch4(&self) -> &OUT_CONF0_CH4 {
        &self.out_conf0_ch4
    }
    #[doc = "0x404 - TX CH4 interrupt raw register"]
    #[inline(always)]
    pub const fn out_int_raw_ch4(&self) -> &OUT_INT_RAW_CH4 {
        &self.out_int_raw_ch4
    }
    #[doc = "0x408 - TX CH4 interrupt ena register"]
    #[inline(always)]
    pub const fn out_int_ena_ch4(&self) -> &OUT_INT_ENA_CH4 {
        &self.out_int_ena_ch4
    }
    #[doc = "0x40c - TX CH4 interrupt st register"]
    #[inline(always)]
    pub const fn out_int_st_ch4(&self) -> &OUT_INT_ST_CH4 {
        &self.out_int_st_ch4
    }
    #[doc = "0x410 - TX CH4 interrupt clr register"]
    #[inline(always)]
    pub const fn out_int_clr_ch4(&self) -> &OUT_INT_CLR_CH4 {
        &self.out_int_clr_ch4
    }
    #[doc = "0x414 - TX CH4 outfifo status register"]
    #[inline(always)]
    pub const fn outfifo_status_ch4(&self) -> &OUTFIFO_STATUS_CH4 {
        &self.outfifo_status_ch4
    }
    #[doc = "0x418 - TX CH4 outfifo push register"]
    #[inline(always)]
    pub const fn out_push_ch4(&self) -> &OUT_PUSH_CH4 {
        &self.out_push_ch4
    }
    #[doc = "0x41c - TX CH4 out_link dscr ctrl register"]
    #[inline(always)]
    pub const fn out_link_conf_ch4(&self) -> &OUT_LINK_CONF_CH4 {
        &self.out_link_conf_ch4
    }
    #[doc = "0x420 - TX CH4 out_link dscr addr register"]
    #[inline(always)]
    pub const fn out_link_addr_ch4(&self) -> &OUT_LINK_ADDR_CH4 {
        &self.out_link_addr_ch4
    }
    #[doc = "0x424 - TX CH4 state register"]
    #[inline(always)]
    pub const fn out_state_ch4(&self) -> &OUT_STATE_CH4 {
        &self.out_state_ch4
    }
    #[doc = "0x428 - TX CH4 eof des addr register"]
    #[inline(always)]
    pub const fn out_eof_des_addr_ch4(&self) -> &OUT_EOF_DES_ADDR_CH4 {
        &self.out_eof_des_addr_ch4
    }
    #[doc = "0x42c - TX CH4 next dscr addr register"]
    #[inline(always)]
    pub const fn out_dscr_ch4(&self) -> &OUT_DSCR_CH4 {
        &self.out_dscr_ch4
    }
    #[doc = "0x430 - TX CH4 last dscr addr register"]
    #[inline(always)]
    pub const fn out_dscr_bf0_ch4(&self) -> &OUT_DSCR_BF0_CH4 {
        &self.out_dscr_bf0_ch4
    }
    #[doc = "0x434 - TX CH4 second-to-last dscr addr register"]
    #[inline(always)]
    pub const fn out_dscr_bf1_ch4(&self) -> &OUT_DSCR_BF1_CH4 {
        &self.out_dscr_bf1_ch4
    }
    #[doc = "0x43c - TX CH4 arb register"]
    #[inline(always)]
    pub const fn out_arb_ch4(&self) -> &OUT_ARB_CH4 {
        &self.out_arb_ch4
    }
    #[doc = "0x468 - TX CH4 ETM config register"]
    #[inline(always)]
    pub const fn out_etm_conf_ch4(&self) -> &OUT_ETM_CONF_CH4 {
        &self.out_etm_conf_ch4
    }
    #[doc = "0x470 - tx CH4 buf len register"]
    #[inline(always)]
    pub const fn out_buf_len_ch4(&self) -> &OUT_BUF_LEN_CH4 {
        &self.out_buf_len_ch4
    }
    #[doc = "0x474 - tx CH4 fifo byte cnt register"]
    #[inline(always)]
    pub const fn out_fifo_bcnt_ch4(&self) -> &OUT_FIFO_BCNT_CH4 {
        &self.out_fifo_bcnt_ch4
    }
    #[doc = "0x478 - tx CH4 push byte cnt register"]
    #[inline(always)]
    pub const fn out_push_bytecnt_ch4(&self) -> &OUT_PUSH_BYTECNT_CH4 {
        &self.out_push_bytecnt_ch4
    }
    #[doc = "0x47c - tx CH4 xaddr register"]
    #[inline(always)]
    pub const fn out_xaddr_ch4(&self) -> &OUT_XADDR_CH4 {
        &self.out_xaddr_ch4
    }
    #[doc = "0x480 - tx CH4 block buf len register"]
    #[inline(always)]
    pub const fn out_block_buf_len_ch4(&self) -> &OUT_BLOCK_BUF_LEN_CH4 {
        &self.out_block_buf_len_ch4
    }
    #[doc = "0x500 - RX CH0 config0 register"]
    #[inline(always)]
    pub const fn in_conf0_ch0(&self) -> &IN_CONF0_CH0 {
        &self.in_conf0_ch0
    }
    #[doc = "0x504 - RX CH0 interrupt raw register"]
    #[inline(always)]
    pub const fn in_int_raw_ch0(&self) -> &IN_INT_RAW_CH0 {
        &self.in_int_raw_ch0
    }
    #[doc = "0x508 - RX CH0 interrupt ena register"]
    #[inline(always)]
    pub const fn in_int_ena_ch0(&self) -> &IN_INT_ENA_CH0 {
        &self.in_int_ena_ch0
    }
    #[doc = "0x50c - RX CH0 interrupt st register"]
    #[inline(always)]
    pub const fn in_int_st_ch0(&self) -> &IN_INT_ST_CH0 {
        &self.in_int_st_ch0
    }
    #[doc = "0x510 - RX CH0 interrupt clr register"]
    #[inline(always)]
    pub const fn in_int_clr_ch0(&self) -> &IN_INT_CLR_CH0 {
        &self.in_int_clr_ch0
    }
    #[doc = "0x514 - RX CH0 INFIFO status register"]
    #[inline(always)]
    pub const fn infifo_status_ch0(&self) -> &INFIFO_STATUS_CH0 {
        &self.infifo_status_ch0
    }
    #[doc = "0x518 - RX CH0 INFIFO pop register"]
    #[inline(always)]
    pub const fn in_pop_ch0(&self) -> &IN_POP_CH0 {
        &self.in_pop_ch0
    }
    #[doc = "0x51c - RX CH0 in_link dscr ctrl register"]
    #[inline(always)]
    pub const fn in_link_conf_ch0(&self) -> &IN_LINK_CONF_CH0 {
        &self.in_link_conf_ch0
    }
    #[doc = "0x520 - RX CH0 in_link dscr addr register"]
    #[inline(always)]
    pub const fn in_link_addr_ch0(&self) -> &IN_LINK_ADDR_CH0 {
        &self.in_link_addr_ch0
    }
    #[doc = "0x524 - RX CH0 state register"]
    #[inline(always)]
    pub const fn in_state_ch0(&self) -> &IN_STATE_CH0 {
        &self.in_state_ch0
    }
    #[doc = "0x528 - RX CH0 eof des addr register"]
    #[inline(always)]
    pub const fn in_suc_eof_des_addr_ch0(&self) -> &IN_SUC_EOF_DES_ADDR_CH0 {
        &self.in_suc_eof_des_addr_ch0
    }
    #[doc = "0x52c - RX CH0 err eof des addr register"]
    #[inline(always)]
    pub const fn in_err_eof_des_addr_ch0(&self) -> &IN_ERR_EOF_DES_ADDR_CH0 {
        &self.in_err_eof_des_addr_ch0
    }
    #[doc = "0x530 - RX CH0 next dscr addr register"]
    #[inline(always)]
    pub const fn in_dscr_ch0(&self) -> &IN_DSCR_CH0 {
        &self.in_dscr_ch0
    }
    #[doc = "0x534 - RX CH0 last dscr addr register"]
    #[inline(always)]
    pub const fn in_dscr_bf0_ch0(&self) -> &IN_DSCR_BF0_CH0 {
        &self.in_dscr_bf0_ch0
    }
    #[doc = "0x538 - RX CH0 second-to-last dscr addr register"]
    #[inline(always)]
    pub const fn in_dscr_bf1_ch0(&self) -> &IN_DSCR_BF1_CH0 {
        &self.in_dscr_bf1_ch0
    }
    #[doc = "0x540 - RX CH0 arb register"]
    #[inline(always)]
    pub const fn in_arb_ch0(&self) -> &IN_ARB_CH0 {
        &self.in_arb_ch0
    }
    #[doc = "0x548 - RX CH0 reorder power config register"]
    #[inline(always)]
    pub const fn in_ro_pd_conf_ch0(&self) -> &IN_RO_PD_CONF_CH0 {
        &self.in_ro_pd_conf_ch0
    }
    #[doc = "0x56c - RX CH0 ETM config register"]
    #[inline(always)]
    pub const fn in_etm_conf_ch0(&self) -> &IN_ETM_CONF_CH0 {
        &self.in_etm_conf_ch0
    }
    #[doc = "0x580 - rx CH0 fifo cnt register"]
    #[inline(always)]
    pub const fn in_fifo_cnt_ch0(&self) -> &IN_FIFO_CNT_CH0 {
        &self.in_fifo_cnt_ch0
    }
    #[doc = "0x584 - rx CH0 pop data cnt register"]
    #[inline(always)]
    pub const fn in_pop_data_cnt_ch0(&self) -> &IN_POP_DATA_CNT_CH0 {
        &self.in_pop_data_cnt_ch0
    }
    #[doc = "0x588 - rx CH0 xaddr register"]
    #[inline(always)]
    pub const fn in_xaddr_ch0(&self) -> &IN_XADDR_CH0 {
        &self.in_xaddr_ch0
    }
    #[doc = "0x58c - rx CH0 buf len hb rcv register"]
    #[inline(always)]
    pub const fn in_buf_hb_rcv_ch0(&self) -> &IN_BUF_HB_RCV_CH0 {
        &self.in_buf_hb_rcv_ch0
    }
    #[doc = "0x600 - RX CH1 config0 register"]
    #[inline(always)]
    pub const fn in_conf0_ch1(&self) -> &IN_CONF0_CH1 {
        &self.in_conf0_ch1
    }
    #[doc = "0x604 - RX CH1 interrupt raw register"]
    #[inline(always)]
    pub const fn in_int_raw_ch1(&self) -> &IN_INT_RAW_CH1 {
        &self.in_int_raw_ch1
    }
    #[doc = "0x608 - RX CH1 interrupt ena register"]
    #[inline(always)]
    pub const fn in_int_ena_ch1(&self) -> &IN_INT_ENA_CH1 {
        &self.in_int_ena_ch1
    }
    #[doc = "0x60c - RX CH1 interrupt st register"]
    #[inline(always)]
    pub const fn in_int_st_ch1(&self) -> &IN_INT_ST_CH1 {
        &self.in_int_st_ch1
    }
    #[doc = "0x610 - RX CH1 interrupt clr register"]
    #[inline(always)]
    pub const fn in_int_clr_ch1(&self) -> &IN_INT_CLR_CH1 {
        &self.in_int_clr_ch1
    }
    #[doc = "0x614 - RX CH1 INFIFO status register"]
    #[inline(always)]
    pub const fn infifo_status_ch1(&self) -> &INFIFO_STATUS_CH1 {
        &self.infifo_status_ch1
    }
    #[doc = "0x618 - RX CH1 INFIFO pop register"]
    #[inline(always)]
    pub const fn in_pop_ch1(&self) -> &IN_POP_CH1 {
        &self.in_pop_ch1
    }
    #[doc = "0x61c - RX CH1 in_link dscr ctrl register"]
    #[inline(always)]
    pub const fn in_link_conf_ch1(&self) -> &IN_LINK_CONF_CH1 {
        &self.in_link_conf_ch1
    }
    #[doc = "0x620 - RX CH1 in_link dscr addr register"]
    #[inline(always)]
    pub const fn in_link_addr_ch1(&self) -> &IN_LINK_ADDR_CH1 {
        &self.in_link_addr_ch1
    }
    #[doc = "0x624 - RX CH1 state register"]
    #[inline(always)]
    pub const fn in_state_ch1(&self) -> &IN_STATE_CH1 {
        &self.in_state_ch1
    }
    #[doc = "0x628 - RX CH1 eof des addr register"]
    #[inline(always)]
    pub const fn in_suc_eof_des_addr_ch1(&self) -> &IN_SUC_EOF_DES_ADDR_CH1 {
        &self.in_suc_eof_des_addr_ch1
    }
    #[doc = "0x62c - RX CH1 err eof des addr register"]
    #[inline(always)]
    pub const fn in_err_eof_des_addr_ch1(&self) -> &IN_ERR_EOF_DES_ADDR_CH1 {
        &self.in_err_eof_des_addr_ch1
    }
    #[doc = "0x630 - RX CH1 next dscr addr register"]
    #[inline(always)]
    pub const fn in_dscr_ch1(&self) -> &IN_DSCR_CH1 {
        &self.in_dscr_ch1
    }
    #[doc = "0x634 - RX CH1 last dscr addr register"]
    #[inline(always)]
    pub const fn in_dscr_bf0_ch1(&self) -> &IN_DSCR_BF0_CH1 {
        &self.in_dscr_bf0_ch1
    }
    #[doc = "0x638 - RX CH1 second-to-last dscr addr register"]
    #[inline(always)]
    pub const fn in_dscr_bf1_ch1(&self) -> &IN_DSCR_BF1_CH1 {
        &self.in_dscr_bf1_ch1
    }
    #[doc = "0x640 - RX CH1 arb register"]
    #[inline(always)]
    pub const fn in_arb_ch1(&self) -> &IN_ARB_CH1 {
        &self.in_arb_ch1
    }
    #[doc = "0x648 - RX CH1 ETM config register"]
    #[inline(always)]
    pub const fn in_etm_conf_ch1(&self) -> &IN_ETM_CONF_CH1 {
        &self.in_etm_conf_ch1
    }
    #[doc = "0x680 - rx CH1 fifo cnt register"]
    #[inline(always)]
    pub const fn in_fifo_cnt_ch1(&self) -> &IN_FIFO_CNT_CH1 {
        &self.in_fifo_cnt_ch1
    }
    #[doc = "0x684 - rx CH1 pop data cnt register"]
    #[inline(always)]
    pub const fn in_pop_data_cnt_ch1(&self) -> &IN_POP_DATA_CNT_CH1 {
        &self.in_pop_data_cnt_ch1
    }
    #[doc = "0x688 - rx CH1 xaddr register"]
    #[inline(always)]
    pub const fn in_xaddr_ch1(&self) -> &IN_XADDR_CH1 {
        &self.in_xaddr_ch1
    }
    #[doc = "0x68c - rx CH1 buf len hb rcv register"]
    #[inline(always)]
    pub const fn in_buf_hb_rcv_ch1(&self) -> &IN_BUF_HB_RCV_CH1 {
        &self.in_buf_hb_rcv_ch1
    }
    #[doc = "0x700 - RX CH2 config0 register"]
    #[inline(always)]
    pub const fn in_conf0_ch2(&self) -> &IN_CONF0_CH2 {
        &self.in_conf0_ch2
    }
    #[doc = "0x704 - RX CH2 interrupt raw register"]
    #[inline(always)]
    pub const fn in_int_raw_ch2(&self) -> &IN_INT_RAW_CH2 {
        &self.in_int_raw_ch2
    }
    #[doc = "0x708 - RX CH2 interrupt ena register"]
    #[inline(always)]
    pub const fn in_int_ena_ch2(&self) -> &IN_INT_ENA_CH2 {
        &self.in_int_ena_ch2
    }
    #[doc = "0x70c - RX CH2 interrupt st register"]
    #[inline(always)]
    pub const fn in_int_st_ch2(&self) -> &IN_INT_ST_CH2 {
        &self.in_int_st_ch2
    }
    #[doc = "0x710 - RX CH2 interrupt clr register"]
    #[inline(always)]
    pub const fn in_int_clr_ch2(&self) -> &IN_INT_CLR_CH2 {
        &self.in_int_clr_ch2
    }
    #[doc = "0x714 - RX CH2 INFIFO status register"]
    #[inline(always)]
    pub const fn infifo_status_ch2(&self) -> &INFIFO_STATUS_CH2 {
        &self.infifo_status_ch2
    }
    #[doc = "0x718 - RX CH2 INFIFO pop register"]
    #[inline(always)]
    pub const fn in_pop_ch2(&self) -> &IN_POP_CH2 {
        &self.in_pop_ch2
    }
    #[doc = "0x71c - RX CH2 in_link dscr ctrl register"]
    #[inline(always)]
    pub const fn in_link_conf_ch2(&self) -> &IN_LINK_CONF_CH2 {
        &self.in_link_conf_ch2
    }
    #[doc = "0x720 - RX CH2 in_link dscr addr register"]
    #[inline(always)]
    pub const fn in_link_addr_ch2(&self) -> &IN_LINK_ADDR_CH2 {
        &self.in_link_addr_ch2
    }
    #[doc = "0x724 - RX CH2 state register"]
    #[inline(always)]
    pub const fn in_state_ch2(&self) -> &IN_STATE_CH2 {
        &self.in_state_ch2
    }
    #[doc = "0x728 - RX CH2 eof des addr register"]
    #[inline(always)]
    pub const fn in_suc_eof_des_addr_ch2(&self) -> &IN_SUC_EOF_DES_ADDR_CH2 {
        &self.in_suc_eof_des_addr_ch2
    }
    #[doc = "0x72c - RX CH2 err eof des addr register"]
    #[inline(always)]
    pub const fn in_err_eof_des_addr_ch2(&self) -> &IN_ERR_EOF_DES_ADDR_CH2 {
        &self.in_err_eof_des_addr_ch2
    }
    #[doc = "0x730 - RX CH2 next dscr addr register"]
    #[inline(always)]
    pub const fn in_dscr_ch2(&self) -> &IN_DSCR_CH2 {
        &self.in_dscr_ch2
    }
    #[doc = "0x734 - RX CH2 last dscr addr register"]
    #[inline(always)]
    pub const fn in_dscr_bf0_ch2(&self) -> &IN_DSCR_BF0_CH2 {
        &self.in_dscr_bf0_ch2
    }
    #[doc = "0x738 - RX CH2 second-to-last dscr addr register"]
    #[inline(always)]
    pub const fn in_dscr_bf1_ch2(&self) -> &IN_DSCR_BF1_CH2 {
        &self.in_dscr_bf1_ch2
    }
    #[doc = "0x740 - RX CH2 arb register"]
    #[inline(always)]
    pub const fn in_arb_ch2(&self) -> &IN_ARB_CH2 {
        &self.in_arb_ch2
    }
    #[doc = "0x748 - RX CH2 ETM config register"]
    #[inline(always)]
    pub const fn in_etm_conf_ch2(&self) -> &IN_ETM_CONF_CH2 {
        &self.in_etm_conf_ch2
    }
    #[doc = "0x780 - rx CH2 fifo cnt register"]
    #[inline(always)]
    pub const fn in_fifo_cnt_ch2(&self) -> &IN_FIFO_CNT_CH2 {
        &self.in_fifo_cnt_ch2
    }
    #[doc = "0x784 - rx CH2 pop data cnt register"]
    #[inline(always)]
    pub const fn in_pop_data_cnt_ch2(&self) -> &IN_POP_DATA_CNT_CH2 {
        &self.in_pop_data_cnt_ch2
    }
    #[doc = "0x788 - rx CH2 xaddr register"]
    #[inline(always)]
    pub const fn in_xaddr_ch2(&self) -> &IN_XADDR_CH2 {
        &self.in_xaddr_ch2
    }
    #[doc = "0x78c - rx CH2 buf len hb rcv register"]
    #[inline(always)]
    pub const fn in_buf_hb_rcv_ch2(&self) -> &IN_BUF_HB_RCV_CH2 {
        &self.in_buf_hb_rcv_ch2
    }
    #[doc = "0x800 - RX CH3 config0 register"]
    #[inline(always)]
    pub const fn in_conf0_ch3(&self) -> &IN_CONF0_CH3 {
        &self.in_conf0_ch3
    }
    #[doc = "0x804 - RX CH3 interrupt raw register"]
    #[inline(always)]
    pub const fn in_int_raw_ch3(&self) -> &IN_INT_RAW_CH3 {
        &self.in_int_raw_ch3
    }
    #[doc = "0x808 - RX CH3 interrupt ena register"]
    #[inline(always)]
    pub const fn in_int_ena_ch3(&self) -> &IN_INT_ENA_CH3 {
        &self.in_int_ena_ch3
    }
    #[doc = "0x80c - RX CH3 interrupt st register"]
    #[inline(always)]
    pub const fn in_int_st_ch3(&self) -> &IN_INT_ST_CH3 {
        &self.in_int_st_ch3
    }
    #[doc = "0x810 - RX CH3 interrupt clr register"]
    #[inline(always)]
    pub const fn in_int_clr_ch3(&self) -> &IN_INT_CLR_CH3 {
        &self.in_int_clr_ch3
    }
    #[doc = "0x814 - RX CH3 INFIFO status register"]
    #[inline(always)]
    pub const fn infifo_status_ch3(&self) -> &INFIFO_STATUS_CH3 {
        &self.infifo_status_ch3
    }
    #[doc = "0x818 - RX CH3 INFIFO pop register"]
    #[inline(always)]
    pub const fn in_pop_ch3(&self) -> &IN_POP_CH3 {
        &self.in_pop_ch3
    }
    #[doc = "0x81c - RX CH3 in_link dscr ctrl register"]
    #[inline(always)]
    pub const fn in_link_conf_ch3(&self) -> &IN_LINK_CONF_CH3 {
        &self.in_link_conf_ch3
    }
    #[doc = "0x820 - RX CH3 in_link dscr addr register"]
    #[inline(always)]
    pub const fn in_link_addr_ch3(&self) -> &IN_LINK_ADDR_CH3 {
        &self.in_link_addr_ch3
    }
    #[doc = "0x824 - RX CH3 state register"]
    #[inline(always)]
    pub const fn in_state_ch3(&self) -> &IN_STATE_CH3 {
        &self.in_state_ch3
    }
    #[doc = "0x828 - RX CH3 eof des addr register"]
    #[inline(always)]
    pub const fn in_suc_eof_des_addr_ch3(&self) -> &IN_SUC_EOF_DES_ADDR_CH3 {
        &self.in_suc_eof_des_addr_ch3
    }
    #[doc = "0x82c - RX CH3 err eof des addr register"]
    #[inline(always)]
    pub const fn in_err_eof_des_addr_ch3(&self) -> &IN_ERR_EOF_DES_ADDR_CH3 {
        &self.in_err_eof_des_addr_ch3
    }
    #[doc = "0x830 - RX CH3 next dscr addr register"]
    #[inline(always)]
    pub const fn in_dscr_ch3(&self) -> &IN_DSCR_CH3 {
        &self.in_dscr_ch3
    }
    #[doc = "0x834 - RX CH3 last dscr addr register"]
    #[inline(always)]
    pub const fn in_dscr_bf0_ch3(&self) -> &IN_DSCR_BF0_CH3 {
        &self.in_dscr_bf0_ch3
    }
    #[doc = "0x838 - RX CH3 second-to-last dscr addr register"]
    #[inline(always)]
    pub const fn in_dscr_bf1_ch3(&self) -> &IN_DSCR_BF1_CH3 {
        &self.in_dscr_bf1_ch3
    }
    #[doc = "0x840 - RX CH3 arb register"]
    #[inline(always)]
    pub const fn in_arb_ch3(&self) -> &IN_ARB_CH3 {
        &self.in_arb_ch3
    }
    #[doc = "0x848 - RX CH3 ETM config register"]
    #[inline(always)]
    pub const fn in_etm_conf_ch3(&self) -> &IN_ETM_CONF_CH3 {
        &self.in_etm_conf_ch3
    }
    #[doc = "0x880 - rx CH3 fifo cnt register"]
    #[inline(always)]
    pub const fn in_fifo_cnt_ch3(&self) -> &IN_FIFO_CNT_CH3 {
        &self.in_fifo_cnt_ch3
    }
    #[doc = "0x884 - rx CH3 pop data cnt register"]
    #[inline(always)]
    pub const fn in_pop_data_cnt_ch3(&self) -> &IN_POP_DATA_CNT_CH3 {
        &self.in_pop_data_cnt_ch3
    }
    #[doc = "0x888 - rx CH3 xaddr register"]
    #[inline(always)]
    pub const fn in_xaddr_ch3(&self) -> &IN_XADDR_CH3 {
        &self.in_xaddr_ch3
    }
    #[doc = "0x88c - rx CH3 buf len hb rcv register"]
    #[inline(always)]
    pub const fn in_buf_hb_rcv_ch3(&self) -> &IN_BUF_HB_RCV_CH3 {
        &self.in_buf_hb_rcv_ch3
    }
    #[doc = "0x900 - RX CH4 config0 register"]
    #[inline(always)]
    pub const fn in_conf0_ch4(&self) -> &IN_CONF0_CH4 {
        &self.in_conf0_ch4
    }
    #[doc = "0x904 - RX CH4 interrupt raw register"]
    #[inline(always)]
    pub const fn in_int_raw_ch4(&self) -> &IN_INT_RAW_CH4 {
        &self.in_int_raw_ch4
    }
    #[doc = "0x908 - RX CH4 interrupt ena register"]
    #[inline(always)]
    pub const fn in_int_ena_ch4(&self) -> &IN_INT_ENA_CH4 {
        &self.in_int_ena_ch4
    }
    #[doc = "0x90c - RX CH4 interrupt st register"]
    #[inline(always)]
    pub const fn in_int_st_ch4(&self) -> &IN_INT_ST_CH4 {
        &self.in_int_st_ch4
    }
    #[doc = "0x910 - RX CH4 interrupt clr register"]
    #[inline(always)]
    pub const fn in_int_clr_ch4(&self) -> &IN_INT_CLR_CH4 {
        &self.in_int_clr_ch4
    }
    #[doc = "0x914 - RX CH4 INFIFO status register"]
    #[inline(always)]
    pub const fn infifo_status_ch4(&self) -> &INFIFO_STATUS_CH4 {
        &self.infifo_status_ch4
    }
    #[doc = "0x918 - RX CH4 INFIFO pop register"]
    #[inline(always)]
    pub const fn in_pop_ch4(&self) -> &IN_POP_CH4 {
        &self.in_pop_ch4
    }
    #[doc = "0x91c - RX CH4 in_link dscr ctrl register"]
    #[inline(always)]
    pub const fn in_link_conf_ch4(&self) -> &IN_LINK_CONF_CH4 {
        &self.in_link_conf_ch4
    }
    #[doc = "0x920 - RX CH4 in_link dscr addr register"]
    #[inline(always)]
    pub const fn in_link_addr_ch4(&self) -> &IN_LINK_ADDR_CH4 {
        &self.in_link_addr_ch4
    }
    #[doc = "0x924 - RX CH4 state register"]
    #[inline(always)]
    pub const fn in_state_ch4(&self) -> &IN_STATE_CH4 {
        &self.in_state_ch4
    }
    #[doc = "0x928 - RX CH4 eof des addr register"]
    #[inline(always)]
    pub const fn in_suc_eof_des_addr_ch4(&self) -> &IN_SUC_EOF_DES_ADDR_CH4 {
        &self.in_suc_eof_des_addr_ch4
    }
    #[doc = "0x92c - RX CH4 err eof des addr register"]
    #[inline(always)]
    pub const fn in_err_eof_des_addr_ch4(&self) -> &IN_ERR_EOF_DES_ADDR_CH4 {
        &self.in_err_eof_des_addr_ch4
    }
    #[doc = "0x930 - RX CH4 next dscr addr register"]
    #[inline(always)]
    pub const fn in_dscr_ch4(&self) -> &IN_DSCR_CH4 {
        &self.in_dscr_ch4
    }
    #[doc = "0x934 - RX CH4 last dscr addr register"]
    #[inline(always)]
    pub const fn in_dscr_bf0_ch4(&self) -> &IN_DSCR_BF0_CH4 {
        &self.in_dscr_bf0_ch4
    }
    #[doc = "0x938 - RX CH4 second-to-last dscr addr register"]
    #[inline(always)]
    pub const fn in_dscr_bf1_ch4(&self) -> &IN_DSCR_BF1_CH4 {
        &self.in_dscr_bf1_ch4
    }
    #[doc = "0x940 - RX CH4 arb register"]
    #[inline(always)]
    pub const fn in_arb_ch4(&self) -> &IN_ARB_CH4 {
        &self.in_arb_ch4
    }
    #[doc = "0x948 - RX CH4 ETM config register"]
    #[inline(always)]
    pub const fn in_etm_conf_ch4(&self) -> &IN_ETM_CONF_CH4 {
        &self.in_etm_conf_ch4
    }
    #[doc = "0x980 - rx CH4 fifo cnt register"]
    #[inline(always)]
    pub const fn in_fifo_cnt_ch4(&self) -> &IN_FIFO_CNT_CH4 {
        &self.in_fifo_cnt_ch4
    }
    #[doc = "0x984 - rx CH4 pop data cnt register"]
    #[inline(always)]
    pub const fn in_pop_data_cnt_ch4(&self) -> &IN_POP_DATA_CNT_CH4 {
        &self.in_pop_data_cnt_ch4
    }
    #[doc = "0x988 - rx CH4 xaddr register"]
    #[inline(always)]
    pub const fn in_xaddr_ch4(&self) -> &IN_XADDR_CH4 {
        &self.in_xaddr_ch4
    }
    #[doc = "0x98c - rx CH4 buf len hb rcv register"]
    #[inline(always)]
    pub const fn in_buf_hb_rcv_ch4(&self) -> &IN_BUF_HB_RCV_CH4 {
        &self.in_buf_hb_rcv_ch4
    }
    #[doc = "0xa00 - RX CH5 config0 register"]
    #[inline(always)]
    pub const fn in_conf0_ch5(&self) -> &IN_CONF0_CH5 {
        &self.in_conf0_ch5
    }
    #[doc = "0xa04 - RX CH5 config1 register"]
    #[inline(always)]
    pub const fn in_conf1_ch5(&self) -> &IN_CONF1_CH5 {
        &self.in_conf1_ch5
    }
    #[doc = "0xa08 - RX CH5 config2 register"]
    #[inline(always)]
    pub const fn in_conf2_ch5(&self) -> &IN_CONF2_CH5 {
        &self.in_conf2_ch5
    }
    #[doc = "0xa0c - RX CH5 config3 register"]
    #[inline(always)]
    pub const fn in_conf3_ch5(&self) -> &IN_CONF3_CH5 {
        &self.in_conf3_ch5
    }
    #[doc = "0xa10 - RX CH5 interrupt raw register"]
    #[inline(always)]
    pub const fn in_int_raw_ch5(&self) -> &IN_INT_RAW_CH5 {
        &self.in_int_raw_ch5
    }
    #[doc = "0xa14 - RX CH5 interrupt ena register"]
    #[inline(always)]
    pub const fn in_int_ena_ch5(&self) -> &IN_INT_ENA_CH5 {
        &self.in_int_ena_ch5
    }
    #[doc = "0xa18 - RX CH5 interrupt st register"]
    #[inline(always)]
    pub const fn in_int_st_ch5(&self) -> &IN_INT_ST_CH5 {
        &self.in_int_st_ch5
    }
    #[doc = "0xa1c - RX CH5 interrupt clr register"]
    #[inline(always)]
    pub const fn in_int_clr_ch5(&self) -> &IN_INT_CLR_CH5 {
        &self.in_int_clr_ch5
    }
    #[doc = "0xa20 - RX CH5 INFIFO status register"]
    #[inline(always)]
    pub const fn infifo_status_ch5(&self) -> &INFIFO_STATUS_CH5 {
        &self.infifo_status_ch5
    }
    #[doc = "0xa24 - RX CH5 INFIFO pop register"]
    #[inline(always)]
    pub const fn in_pop_ch5(&self) -> &IN_POP_CH5 {
        &self.in_pop_ch5
    }
    #[doc = "0xa28 - RX CH5 state register"]
    #[inline(always)]
    pub const fn in_state_ch5(&self) -> &IN_STATE_CH5 {
        &self.in_state_ch5
    }
    #[doc = "0xa40 - RX CH5 arb register"]
    #[inline(always)]
    pub const fn in_arb_ch5(&self) -> &IN_ARB_CH5 {
        &self.in_arb_ch5
    }
    #[doc = "0xa80 - rx CH5 fifo cnt register"]
    #[inline(always)]
    pub const fn in_fifo_cnt_ch5(&self) -> &IN_FIFO_CNT_CH5 {
        &self.in_fifo_cnt_ch5
    }
    #[doc = "0xa84 - rx CH5 pop data cnt register"]
    #[inline(always)]
    pub const fn in_pop_data_cnt_ch5(&self) -> &IN_POP_DATA_CNT_CH5 {
        &self.in_pop_data_cnt_ch5
    }
    #[doc = "0xa88 - rx CH5 xaddr register"]
    #[inline(always)]
    pub const fn in_xaddr_ch5(&self) -> &IN_XADDR_CH5 {
        &self.in_xaddr_ch5
    }
    #[doc = "0xa8c - rx CH5 buf len hb rcv register"]
    #[inline(always)]
    pub const fn in_buf_hb_rcv_ch5(&self) -> &IN_BUF_HB_RCV_CH5 {
        &self.in_buf_hb_rcv_ch5
    }
    #[doc = "0xb00 - inter memory axi err register"]
    #[inline(always)]
    pub const fn inter_axi_err(&self) -> &INTER_AXI_ERR {
        &self.inter_axi_err
    }
    #[doc = "0xb04 - exter memory axi err register"]
    #[inline(always)]
    pub const fn exter_axi_err(&self) -> &EXTER_AXI_ERR {
        &self.exter_axi_err
    }
    #[doc = "0xb08 - axi reset config register"]
    #[inline(always)]
    pub const fn rst_conf(&self) -> &RST_CONF {
        &self.rst_conf
    }
    #[doc = "0xb0c - Start address of inter memory range0 register"]
    #[inline(always)]
    pub const fn inter_mem_start_addr0(&self) -> &INTER_MEM_START_ADDR0 {
        &self.inter_mem_start_addr0
    }
    #[doc = "0xb10 - end address of inter memory range0 register"]
    #[inline(always)]
    pub const fn inter_mem_end_addr0(&self) -> &INTER_MEM_END_ADDR0 {
        &self.inter_mem_end_addr0
    }
    #[doc = "0xb14 - Start address of inter memory range1 register"]
    #[inline(always)]
    pub const fn inter_mem_start_addr1(&self) -> &INTER_MEM_START_ADDR1 {
        &self.inter_mem_start_addr1
    }
    #[doc = "0xb18 - end address of inter memory range1 register"]
    #[inline(always)]
    pub const fn inter_mem_end_addr1(&self) -> &INTER_MEM_END_ADDR1 {
        &self.inter_mem_end_addr1
    }
    #[doc = "0xb20 - Start address of exter memory range0 register"]
    #[inline(always)]
    pub const fn exter_mem_start_addr0(&self) -> &EXTER_MEM_START_ADDR0 {
        &self.exter_mem_start_addr0
    }
    #[doc = "0xb24 - end address of exter memory range0 register"]
    #[inline(always)]
    pub const fn exter_mem_end_addr0(&self) -> &EXTER_MEM_END_ADDR0 {
        &self.exter_mem_end_addr0
    }
    #[doc = "0xb28 - Start address of exter memory range1 register"]
    #[inline(always)]
    pub const fn exter_mem_start_addr1(&self) -> &EXTER_MEM_START_ADDR1 {
        &self.exter_mem_start_addr1
    }
    #[doc = "0xb2c - end address of exter memory range1 register"]
    #[inline(always)]
    pub const fn exter_mem_end_addr1(&self) -> &EXTER_MEM_END_ADDR1 {
        &self.exter_mem_end_addr1
    }
    #[doc = "0xb30 - reserved"]
    #[inline(always)]
    pub const fn out_arb_config(&self) -> &OUT_ARB_CONFIG {
        &self.out_arb_config
    }
    #[doc = "0xb34 - reserved"]
    #[inline(always)]
    pub const fn in_arb_config(&self) -> &IN_ARB_CONFIG {
        &self.in_arb_config
    }
    #[doc = "0xb3c - reserved"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    #[doc = "0xb50 - counter reset register"]
    #[inline(always)]
    pub const fn counter_rst(&self) -> &COUNTER_RST {
        &self.counter_rst
    }
    #[doc = "0xb54 - rx ch0 counter register"]
    #[inline(always)]
    pub const fn rx_ch0_counter(&self) -> &RX_CH0_COUNTER {
        &self.rx_ch0_counter
    }
    #[doc = "0xb58 - rx ch1 counter register"]
    #[inline(always)]
    pub const fn rx_ch1_counter(&self) -> &RX_CH1_COUNTER {
        &self.rx_ch1_counter
    }
    #[doc = "0xb5c - rx ch2 counter register"]
    #[inline(always)]
    pub const fn rx_ch2_counter(&self) -> &RX_CH2_COUNTER {
        &self.rx_ch2_counter
    }
    #[doc = "0xb60 - rx ch5 counter register"]
    #[inline(always)]
    pub const fn rx_ch5_counter(&self) -> &RX_CH5_COUNTER {
        &self.rx_ch5_counter
    }
}
#[doc = "OUT_CONF0_CH0 (rw) register accessor: TX CH0 config0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf0_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf0_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf0_ch0`] module"]
pub type OUT_CONF0_CH0 = crate::Reg<out_conf0_ch0::OUT_CONF0_CH0_SPEC>;
#[doc = "TX CH0 config0 register"]
pub mod out_conf0_ch0;
#[doc = "OUT_INT_RAW_CH0 (rw) register accessor: TX CH0 interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_raw_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_raw_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_raw_ch0`] module"]
pub type OUT_INT_RAW_CH0 = crate::Reg<out_int_raw_ch0::OUT_INT_RAW_CH0_SPEC>;
#[doc = "TX CH0 interrupt raw register"]
pub mod out_int_raw_ch0;
#[doc = "OUT_INT_ENA_CH0 (rw) register accessor: TX CH0 interrupt ena register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_ena_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_ena_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_ena_ch0`] module"]
pub type OUT_INT_ENA_CH0 = crate::Reg<out_int_ena_ch0::OUT_INT_ENA_CH0_SPEC>;
#[doc = "TX CH0 interrupt ena register"]
pub mod out_int_ena_ch0;
#[doc = "OUT_INT_ST_CH0 (r) register accessor: TX CH0 interrupt st register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_st_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_st_ch0`] module"]
pub type OUT_INT_ST_CH0 = crate::Reg<out_int_st_ch0::OUT_INT_ST_CH0_SPEC>;
#[doc = "TX CH0 interrupt st register"]
pub mod out_int_st_ch0;
#[doc = "OUT_INT_CLR_CH0 (w) register accessor: TX CH0 interrupt clr register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_clr_ch0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_clr_ch0`] module"]
pub type OUT_INT_CLR_CH0 = crate::Reg<out_int_clr_ch0::OUT_INT_CLR_CH0_SPEC>;
#[doc = "TX CH0 interrupt clr register"]
pub mod out_int_clr_ch0;
#[doc = "OUTFIFO_STATUS_CH0 (r) register accessor: TX CH0 outfifo status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outfifo_status_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outfifo_status_ch0`] module"]
pub type OUTFIFO_STATUS_CH0 = crate::Reg<outfifo_status_ch0::OUTFIFO_STATUS_CH0_SPEC>;
#[doc = "TX CH0 outfifo status register"]
pub mod outfifo_status_ch0;
#[doc = "OUT_PUSH_CH0 (rw) register accessor: TX CH0 outfifo push register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_push_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_push_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_push_ch0`] module"]
pub type OUT_PUSH_CH0 = crate::Reg<out_push_ch0::OUT_PUSH_CH0_SPEC>;
#[doc = "TX CH0 outfifo push register"]
pub mod out_push_ch0;
#[doc = "OUT_LINK_CONF_CH0 (rw) register accessor: TX CH0 out_link dscr ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_link_conf_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link_conf_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link_conf_ch0`] module"]
pub type OUT_LINK_CONF_CH0 = crate::Reg<out_link_conf_ch0::OUT_LINK_CONF_CH0_SPEC>;
#[doc = "TX CH0 out_link dscr ctrl register"]
pub mod out_link_conf_ch0;
#[doc = "OUT_LINK_ADDR_CH0 (rw) register accessor: TX CH0 out_link dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_link_addr_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link_addr_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link_addr_ch0`] module"]
pub type OUT_LINK_ADDR_CH0 = crate::Reg<out_link_addr_ch0::OUT_LINK_ADDR_CH0_SPEC>;
#[doc = "TX CH0 out_link dscr addr register"]
pub mod out_link_addr_ch0;
#[doc = "OUT_STATE_CH0 (r) register accessor: TX CH0 state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_state_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_state_ch0`] module"]
pub type OUT_STATE_CH0 = crate::Reg<out_state_ch0::OUT_STATE_CH0_SPEC>;
#[doc = "TX CH0 state register"]
pub mod out_state_ch0;
#[doc = "OUT_EOF_DES_ADDR_CH0 (r) register accessor: TX CH0 eof des addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_eof_des_addr_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_des_addr_ch0`] module"]
pub type OUT_EOF_DES_ADDR_CH0 = crate::Reg<out_eof_des_addr_ch0::OUT_EOF_DES_ADDR_CH0_SPEC>;
#[doc = "TX CH0 eof des addr register"]
pub mod out_eof_des_addr_ch0;
#[doc = "OUT_DSCR_CH0 (r) register accessor: TX CH0 next dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_ch0`] module"]
pub type OUT_DSCR_CH0 = crate::Reg<out_dscr_ch0::OUT_DSCR_CH0_SPEC>;
#[doc = "TX CH0 next dscr addr register"]
pub mod out_dscr_ch0;
#[doc = "OUT_DSCR_BF0_CH0 (r) register accessor: TX CH0 last dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_bf0_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf0_ch0`] module"]
pub type OUT_DSCR_BF0_CH0 = crate::Reg<out_dscr_bf0_ch0::OUT_DSCR_BF0_CH0_SPEC>;
#[doc = "TX CH0 last dscr addr register"]
pub mod out_dscr_bf0_ch0;
#[doc = "OUT_DSCR_BF1_CH0 (r) register accessor: TX CH0 second-to-last dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_bf1_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf1_ch0`] module"]
pub type OUT_DSCR_BF1_CH0 = crate::Reg<out_dscr_bf1_ch0::OUT_DSCR_BF1_CH0_SPEC>;
#[doc = "TX CH0 second-to-last dscr addr register"]
pub mod out_dscr_bf1_ch0;
#[doc = "OUT_ARB_CH0 (rw) register accessor: TX CH0 arb register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_arb_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_arb_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_arb_ch0`] module"]
pub type OUT_ARB_CH0 = crate::Reg<out_arb_ch0::OUT_ARB_CH0_SPEC>;
#[doc = "TX CH0 arb register"]
pub mod out_arb_ch0;
#[doc = "OUT_RO_STATUS_CH0 (r) register accessor: TX CH0 reorder status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_ro_status_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_ro_status_ch0`] module"]
pub type OUT_RO_STATUS_CH0 = crate::Reg<out_ro_status_ch0::OUT_RO_STATUS_CH0_SPEC>;
#[doc = "TX CH0 reorder status register"]
pub mod out_ro_status_ch0;
#[doc = "OUT_RO_PD_CONF_CH0 (rw) register accessor: TX CH0 reorder power config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_ro_pd_conf_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_ro_pd_conf_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_ro_pd_conf_ch0`] module"]
pub type OUT_RO_PD_CONF_CH0 = crate::Reg<out_ro_pd_conf_ch0::OUT_RO_PD_CONF_CH0_SPEC>;
#[doc = "TX CH0 reorder power config register"]
pub mod out_ro_pd_conf_ch0;
#[doc = "OUT_MODE_ENABLE_CH0 (rw) register accessor: tx CH0 mode enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_mode_enable_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_mode_enable_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_mode_enable_ch0`] module"]
pub type OUT_MODE_ENABLE_CH0 = crate::Reg<out_mode_enable_ch0::OUT_MODE_ENABLE_CH0_SPEC>;
#[doc = "tx CH0 mode enable register"]
pub mod out_mode_enable_ch0;
#[doc = "OUT_MODE_YUV_CH0 (rw) register accessor: tx CH0 test mode yuv value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_mode_yuv_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_mode_yuv_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_mode_yuv_ch0`] module"]
pub type OUT_MODE_YUV_CH0 = crate::Reg<out_mode_yuv_ch0::OUT_MODE_YUV_CH0_SPEC>;
#[doc = "tx CH0 test mode yuv value register"]
pub mod out_mode_yuv_ch0;
#[doc = "OUT_ETM_CONF_CH0 (rw) register accessor: TX CH0 ETM config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_etm_conf_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_etm_conf_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_etm_conf_ch0`] module"]
pub type OUT_ETM_CONF_CH0 = crate::Reg<out_etm_conf_ch0::OUT_ETM_CONF_CH0_SPEC>;
#[doc = "TX CH0 ETM config register"]
pub mod out_etm_conf_ch0;
#[doc = "OUT_BUF_LEN_CH0 (r) register accessor: tx CH0 buf len register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_buf_len_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_buf_len_ch0`] module"]
pub type OUT_BUF_LEN_CH0 = crate::Reg<out_buf_len_ch0::OUT_BUF_LEN_CH0_SPEC>;
#[doc = "tx CH0 buf len register"]
pub mod out_buf_len_ch0;
#[doc = "OUT_FIFO_BCNT_CH0 (r) register accessor: tx CH0 fifo byte cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_fifo_bcnt_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_fifo_bcnt_ch0`] module"]
pub type OUT_FIFO_BCNT_CH0 = crate::Reg<out_fifo_bcnt_ch0::OUT_FIFO_BCNT_CH0_SPEC>;
#[doc = "tx CH0 fifo byte cnt register"]
pub mod out_fifo_bcnt_ch0;
#[doc = "OUT_PUSH_BYTECNT_CH0 (r) register accessor: tx CH0 push byte cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_push_bytecnt_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_push_bytecnt_ch0`] module"]
pub type OUT_PUSH_BYTECNT_CH0 = crate::Reg<out_push_bytecnt_ch0::OUT_PUSH_BYTECNT_CH0_SPEC>;
#[doc = "tx CH0 push byte cnt register"]
pub mod out_push_bytecnt_ch0;
#[doc = "OUT_XADDR_CH0 (r) register accessor: tx CH0 xaddr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_xaddr_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_xaddr_ch0`] module"]
pub type OUT_XADDR_CH0 = crate::Reg<out_xaddr_ch0::OUT_XADDR_CH0_SPEC>;
#[doc = "tx CH0 xaddr register"]
pub mod out_xaddr_ch0;
#[doc = "OUT_CONF0_CH1 (rw) register accessor: TX CH1 config0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf0_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf0_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf0_ch1`] module"]
pub type OUT_CONF0_CH1 = crate::Reg<out_conf0_ch1::OUT_CONF0_CH1_SPEC>;
#[doc = "TX CH1 config0 register"]
pub mod out_conf0_ch1;
#[doc = "OUT_INT_RAW_CH1 (rw) register accessor: TX CH1 interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_raw_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_raw_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_raw_ch1`] module"]
pub type OUT_INT_RAW_CH1 = crate::Reg<out_int_raw_ch1::OUT_INT_RAW_CH1_SPEC>;
#[doc = "TX CH1 interrupt raw register"]
pub mod out_int_raw_ch1;
#[doc = "OUT_INT_ENA_CH1 (rw) register accessor: TX CH1 interrupt ena register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_ena_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_ena_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_ena_ch1`] module"]
pub type OUT_INT_ENA_CH1 = crate::Reg<out_int_ena_ch1::OUT_INT_ENA_CH1_SPEC>;
#[doc = "TX CH1 interrupt ena register"]
pub mod out_int_ena_ch1;
#[doc = "OUT_INT_ST_CH1 (r) register accessor: TX CH1 interrupt st register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_st_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_st_ch1`] module"]
pub type OUT_INT_ST_CH1 = crate::Reg<out_int_st_ch1::OUT_INT_ST_CH1_SPEC>;
#[doc = "TX CH1 interrupt st register"]
pub mod out_int_st_ch1;
#[doc = "OUT_INT_CLR_CH1 (w) register accessor: TX CH1 interrupt clr register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_clr_ch1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_clr_ch1`] module"]
pub type OUT_INT_CLR_CH1 = crate::Reg<out_int_clr_ch1::OUT_INT_CLR_CH1_SPEC>;
#[doc = "TX CH1 interrupt clr register"]
pub mod out_int_clr_ch1;
#[doc = "OUTFIFO_STATUS_CH1 (r) register accessor: TX CH1 outfifo status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outfifo_status_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outfifo_status_ch1`] module"]
pub type OUTFIFO_STATUS_CH1 = crate::Reg<outfifo_status_ch1::OUTFIFO_STATUS_CH1_SPEC>;
#[doc = "TX CH1 outfifo status register"]
pub mod outfifo_status_ch1;
#[doc = "OUT_PUSH_CH1 (rw) register accessor: TX CH1 outfifo push register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_push_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_push_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_push_ch1`] module"]
pub type OUT_PUSH_CH1 = crate::Reg<out_push_ch1::OUT_PUSH_CH1_SPEC>;
#[doc = "TX CH1 outfifo push register"]
pub mod out_push_ch1;
#[doc = "OUT_LINK_CONF_CH1 (rw) register accessor: TX CH1 out_link dscr ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_link_conf_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link_conf_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link_conf_ch1`] module"]
pub type OUT_LINK_CONF_CH1 = crate::Reg<out_link_conf_ch1::OUT_LINK_CONF_CH1_SPEC>;
#[doc = "TX CH1 out_link dscr ctrl register"]
pub mod out_link_conf_ch1;
#[doc = "OUT_LINK_ADDR_CH1 (rw) register accessor: TX CH1 out_link dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_link_addr_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link_addr_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link_addr_ch1`] module"]
pub type OUT_LINK_ADDR_CH1 = crate::Reg<out_link_addr_ch1::OUT_LINK_ADDR_CH1_SPEC>;
#[doc = "TX CH1 out_link dscr addr register"]
pub mod out_link_addr_ch1;
#[doc = "OUT_STATE_CH1 (r) register accessor: TX CH1 state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_state_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_state_ch1`] module"]
pub type OUT_STATE_CH1 = crate::Reg<out_state_ch1::OUT_STATE_CH1_SPEC>;
#[doc = "TX CH1 state register"]
pub mod out_state_ch1;
#[doc = "OUT_EOF_DES_ADDR_CH1 (r) register accessor: TX CH1 eof des addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_eof_des_addr_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_des_addr_ch1`] module"]
pub type OUT_EOF_DES_ADDR_CH1 = crate::Reg<out_eof_des_addr_ch1::OUT_EOF_DES_ADDR_CH1_SPEC>;
#[doc = "TX CH1 eof des addr register"]
pub mod out_eof_des_addr_ch1;
#[doc = "OUT_DSCR_CH1 (r) register accessor: TX CH1 next dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_ch1`] module"]
pub type OUT_DSCR_CH1 = crate::Reg<out_dscr_ch1::OUT_DSCR_CH1_SPEC>;
#[doc = "TX CH1 next dscr addr register"]
pub mod out_dscr_ch1;
#[doc = "OUT_DSCR_BF0_CH1 (r) register accessor: TX CH1 last dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_bf0_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf0_ch1`] module"]
pub type OUT_DSCR_BF0_CH1 = crate::Reg<out_dscr_bf0_ch1::OUT_DSCR_BF0_CH1_SPEC>;
#[doc = "TX CH1 last dscr addr register"]
pub mod out_dscr_bf0_ch1;
#[doc = "OUT_DSCR_BF1_CH1 (r) register accessor: TX CH1 second-to-last dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_bf1_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf1_ch1`] module"]
pub type OUT_DSCR_BF1_CH1 = crate::Reg<out_dscr_bf1_ch1::OUT_DSCR_BF1_CH1_SPEC>;
#[doc = "TX CH1 second-to-last dscr addr register"]
pub mod out_dscr_bf1_ch1;
#[doc = "OUT_ARB_CH1 (rw) register accessor: TX CH1 arb register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_arb_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_arb_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_arb_ch1`] module"]
pub type OUT_ARB_CH1 = crate::Reg<out_arb_ch1::OUT_ARB_CH1_SPEC>;
#[doc = "TX CH1 arb register"]
pub mod out_arb_ch1;
#[doc = "OUT_ETM_CONF_CH1 (rw) register accessor: TX CH1 ETM config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_etm_conf_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_etm_conf_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_etm_conf_ch1`] module"]
pub type OUT_ETM_CONF_CH1 = crate::Reg<out_etm_conf_ch1::OUT_ETM_CONF_CH1_SPEC>;
#[doc = "TX CH1 ETM config register"]
pub mod out_etm_conf_ch1;
#[doc = "OUT_BUF_LEN_CH1 (r) register accessor: tx CH1 buf len register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_buf_len_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_buf_len_ch1`] module"]
pub type OUT_BUF_LEN_CH1 = crate::Reg<out_buf_len_ch1::OUT_BUF_LEN_CH1_SPEC>;
#[doc = "tx CH1 buf len register"]
pub mod out_buf_len_ch1;
#[doc = "OUT_FIFO_BCNT_CH1 (r) register accessor: tx CH1 fifo byte cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_fifo_bcnt_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_fifo_bcnt_ch1`] module"]
pub type OUT_FIFO_BCNT_CH1 = crate::Reg<out_fifo_bcnt_ch1::OUT_FIFO_BCNT_CH1_SPEC>;
#[doc = "tx CH1 fifo byte cnt register"]
pub mod out_fifo_bcnt_ch1;
#[doc = "OUT_PUSH_BYTECNT_CH1 (r) register accessor: tx CH1 push byte cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_push_bytecnt_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_push_bytecnt_ch1`] module"]
pub type OUT_PUSH_BYTECNT_CH1 = crate::Reg<out_push_bytecnt_ch1::OUT_PUSH_BYTECNT_CH1_SPEC>;
#[doc = "tx CH1 push byte cnt register"]
pub mod out_push_bytecnt_ch1;
#[doc = "OUT_XADDR_CH1 (r) register accessor: tx CH1 xaddr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_xaddr_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_xaddr_ch1`] module"]
pub type OUT_XADDR_CH1 = crate::Reg<out_xaddr_ch1::OUT_XADDR_CH1_SPEC>;
#[doc = "tx CH1 xaddr register"]
pub mod out_xaddr_ch1;
#[doc = "OUT_CONF0_CH2 (rw) register accessor: TX CH2 config0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf0_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf0_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf0_ch2`] module"]
pub type OUT_CONF0_CH2 = crate::Reg<out_conf0_ch2::OUT_CONF0_CH2_SPEC>;
#[doc = "TX CH2 config0 register"]
pub mod out_conf0_ch2;
#[doc = "OUT_INT_RAW_CH2 (rw) register accessor: TX CH2 interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_raw_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_raw_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_raw_ch2`] module"]
pub type OUT_INT_RAW_CH2 = crate::Reg<out_int_raw_ch2::OUT_INT_RAW_CH2_SPEC>;
#[doc = "TX CH2 interrupt raw register"]
pub mod out_int_raw_ch2;
#[doc = "OUT_INT_ENA_CH2 (rw) register accessor: TX CH2 interrupt ena register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_ena_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_ena_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_ena_ch2`] module"]
pub type OUT_INT_ENA_CH2 = crate::Reg<out_int_ena_ch2::OUT_INT_ENA_CH2_SPEC>;
#[doc = "TX CH2 interrupt ena register"]
pub mod out_int_ena_ch2;
#[doc = "OUT_INT_ST_CH2 (r) register accessor: TX CH2 interrupt st register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_st_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_st_ch2`] module"]
pub type OUT_INT_ST_CH2 = crate::Reg<out_int_st_ch2::OUT_INT_ST_CH2_SPEC>;
#[doc = "TX CH2 interrupt st register"]
pub mod out_int_st_ch2;
#[doc = "OUT_INT_CLR_CH2 (w) register accessor: TX CH2 interrupt clr register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_clr_ch2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_clr_ch2`] module"]
pub type OUT_INT_CLR_CH2 = crate::Reg<out_int_clr_ch2::OUT_INT_CLR_CH2_SPEC>;
#[doc = "TX CH2 interrupt clr register"]
pub mod out_int_clr_ch2;
#[doc = "OUTFIFO_STATUS_CH2 (r) register accessor: TX CH2 outfifo status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outfifo_status_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outfifo_status_ch2`] module"]
pub type OUTFIFO_STATUS_CH2 = crate::Reg<outfifo_status_ch2::OUTFIFO_STATUS_CH2_SPEC>;
#[doc = "TX CH2 outfifo status register"]
pub mod outfifo_status_ch2;
#[doc = "OUT_PUSH_CH2 (rw) register accessor: TX CH2 outfifo push register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_push_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_push_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_push_ch2`] module"]
pub type OUT_PUSH_CH2 = crate::Reg<out_push_ch2::OUT_PUSH_CH2_SPEC>;
#[doc = "TX CH2 outfifo push register"]
pub mod out_push_ch2;
#[doc = "OUT_LINK_CONF_CH2 (rw) register accessor: TX CH2 out_link dscr ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_link_conf_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link_conf_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link_conf_ch2`] module"]
pub type OUT_LINK_CONF_CH2 = crate::Reg<out_link_conf_ch2::OUT_LINK_CONF_CH2_SPEC>;
#[doc = "TX CH2 out_link dscr ctrl register"]
pub mod out_link_conf_ch2;
#[doc = "OUT_LINK_ADDR_CH2 (rw) register accessor: TX CH2 out_link dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_link_addr_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link_addr_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link_addr_ch2`] module"]
pub type OUT_LINK_ADDR_CH2 = crate::Reg<out_link_addr_ch2::OUT_LINK_ADDR_CH2_SPEC>;
#[doc = "TX CH2 out_link dscr addr register"]
pub mod out_link_addr_ch2;
#[doc = "OUT_STATE_CH2 (r) register accessor: TX CH2 state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_state_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_state_ch2`] module"]
pub type OUT_STATE_CH2 = crate::Reg<out_state_ch2::OUT_STATE_CH2_SPEC>;
#[doc = "TX CH2 state register"]
pub mod out_state_ch2;
#[doc = "OUT_EOF_DES_ADDR_CH2 (r) register accessor: TX CH2 eof des addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_eof_des_addr_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_des_addr_ch2`] module"]
pub type OUT_EOF_DES_ADDR_CH2 = crate::Reg<out_eof_des_addr_ch2::OUT_EOF_DES_ADDR_CH2_SPEC>;
#[doc = "TX CH2 eof des addr register"]
pub mod out_eof_des_addr_ch2;
#[doc = "OUT_DSCR_CH2 (r) register accessor: TX CH2 next dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_ch2`] module"]
pub type OUT_DSCR_CH2 = crate::Reg<out_dscr_ch2::OUT_DSCR_CH2_SPEC>;
#[doc = "TX CH2 next dscr addr register"]
pub mod out_dscr_ch2;
#[doc = "OUT_DSCR_BF0_CH2 (r) register accessor: TX CH2 last dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_bf0_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf0_ch2`] module"]
pub type OUT_DSCR_BF0_CH2 = crate::Reg<out_dscr_bf0_ch2::OUT_DSCR_BF0_CH2_SPEC>;
#[doc = "TX CH2 last dscr addr register"]
pub mod out_dscr_bf0_ch2;
#[doc = "OUT_DSCR_BF1_CH2 (r) register accessor: TX CH2 second-to-last dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_bf1_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf1_ch2`] module"]
pub type OUT_DSCR_BF1_CH2 = crate::Reg<out_dscr_bf1_ch2::OUT_DSCR_BF1_CH2_SPEC>;
#[doc = "TX CH2 second-to-last dscr addr register"]
pub mod out_dscr_bf1_ch2;
#[doc = "OUT_ARB_CH2 (rw) register accessor: TX CH2 arb register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_arb_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_arb_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_arb_ch2`] module"]
pub type OUT_ARB_CH2 = crate::Reg<out_arb_ch2::OUT_ARB_CH2_SPEC>;
#[doc = "TX CH2 arb register"]
pub mod out_arb_ch2;
#[doc = "OUT_ETM_CONF_CH2 (rw) register accessor: TX CH2 ETM config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_etm_conf_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_etm_conf_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_etm_conf_ch2`] module"]
pub type OUT_ETM_CONF_CH2 = crate::Reg<out_etm_conf_ch2::OUT_ETM_CONF_CH2_SPEC>;
#[doc = "TX CH2 ETM config register"]
pub mod out_etm_conf_ch2;
#[doc = "OUT_BUF_LEN_CH2 (r) register accessor: tx CH2 buf len register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_buf_len_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_buf_len_ch2`] module"]
pub type OUT_BUF_LEN_CH2 = crate::Reg<out_buf_len_ch2::OUT_BUF_LEN_CH2_SPEC>;
#[doc = "tx CH2 buf len register"]
pub mod out_buf_len_ch2;
#[doc = "OUT_FIFO_BCNT_CH2 (r) register accessor: tx CH2 fifo byte cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_fifo_bcnt_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_fifo_bcnt_ch2`] module"]
pub type OUT_FIFO_BCNT_CH2 = crate::Reg<out_fifo_bcnt_ch2::OUT_FIFO_BCNT_CH2_SPEC>;
#[doc = "tx CH2 fifo byte cnt register"]
pub mod out_fifo_bcnt_ch2;
#[doc = "OUT_PUSH_BYTECNT_CH2 (r) register accessor: tx CH2 push byte cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_push_bytecnt_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_push_bytecnt_ch2`] module"]
pub type OUT_PUSH_BYTECNT_CH2 = crate::Reg<out_push_bytecnt_ch2::OUT_PUSH_BYTECNT_CH2_SPEC>;
#[doc = "tx CH2 push byte cnt register"]
pub mod out_push_bytecnt_ch2;
#[doc = "OUT_XADDR_CH2 (r) register accessor: tx CH2 xaddr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_xaddr_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_xaddr_ch2`] module"]
pub type OUT_XADDR_CH2 = crate::Reg<out_xaddr_ch2::OUT_XADDR_CH2_SPEC>;
#[doc = "tx CH2 xaddr register"]
pub mod out_xaddr_ch2;
#[doc = "OUT_CONF0_CH3 (rw) register accessor: TX CH3 config0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf0_ch3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf0_ch3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf0_ch3`] module"]
pub type OUT_CONF0_CH3 = crate::Reg<out_conf0_ch3::OUT_CONF0_CH3_SPEC>;
#[doc = "TX CH3 config0 register"]
pub mod out_conf0_ch3;
#[doc = "OUT_INT_RAW_CH3 (rw) register accessor: TX CH3 interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_raw_ch3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_raw_ch3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_raw_ch3`] module"]
pub type OUT_INT_RAW_CH3 = crate::Reg<out_int_raw_ch3::OUT_INT_RAW_CH3_SPEC>;
#[doc = "TX CH3 interrupt raw register"]
pub mod out_int_raw_ch3;
#[doc = "OUT_INT_ENA_CH3 (rw) register accessor: TX CH3 interrupt ena register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_ena_ch3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_ena_ch3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_ena_ch3`] module"]
pub type OUT_INT_ENA_CH3 = crate::Reg<out_int_ena_ch3::OUT_INT_ENA_CH3_SPEC>;
#[doc = "TX CH3 interrupt ena register"]
pub mod out_int_ena_ch3;
#[doc = "OUT_INT_ST_CH3 (r) register accessor: TX CH3 interrupt st register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_st_ch3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_st_ch3`] module"]
pub type OUT_INT_ST_CH3 = crate::Reg<out_int_st_ch3::OUT_INT_ST_CH3_SPEC>;
#[doc = "TX CH3 interrupt st register"]
pub mod out_int_st_ch3;
#[doc = "OUT_INT_CLR_CH3 (w) register accessor: TX CH3 interrupt clr register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_clr_ch3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_clr_ch3`] module"]
pub type OUT_INT_CLR_CH3 = crate::Reg<out_int_clr_ch3::OUT_INT_CLR_CH3_SPEC>;
#[doc = "TX CH3 interrupt clr register"]
pub mod out_int_clr_ch3;
#[doc = "OUTFIFO_STATUS_CH3 (r) register accessor: TX CH3 outfifo status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outfifo_status_ch3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outfifo_status_ch3`] module"]
pub type OUTFIFO_STATUS_CH3 = crate::Reg<outfifo_status_ch3::OUTFIFO_STATUS_CH3_SPEC>;
#[doc = "TX CH3 outfifo status register"]
pub mod outfifo_status_ch3;
#[doc = "OUT_PUSH_CH3 (rw) register accessor: TX CH3 outfifo push register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_push_ch3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_push_ch3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_push_ch3`] module"]
pub type OUT_PUSH_CH3 = crate::Reg<out_push_ch3::OUT_PUSH_CH3_SPEC>;
#[doc = "TX CH3 outfifo push register"]
pub mod out_push_ch3;
#[doc = "OUT_LINK_CONF_CH3 (rw) register accessor: TX CH3 out_link dscr ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_link_conf_ch3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link_conf_ch3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link_conf_ch3`] module"]
pub type OUT_LINK_CONF_CH3 = crate::Reg<out_link_conf_ch3::OUT_LINK_CONF_CH3_SPEC>;
#[doc = "TX CH3 out_link dscr ctrl register"]
pub mod out_link_conf_ch3;
#[doc = "OUT_LINK_ADDR_CH3 (rw) register accessor: TX CH3 out_link dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_link_addr_ch3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link_addr_ch3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link_addr_ch3`] module"]
pub type OUT_LINK_ADDR_CH3 = crate::Reg<out_link_addr_ch3::OUT_LINK_ADDR_CH3_SPEC>;
#[doc = "TX CH3 out_link dscr addr register"]
pub mod out_link_addr_ch3;
#[doc = "OUT_STATE_CH3 (r) register accessor: TX CH3 state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_state_ch3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_state_ch3`] module"]
pub type OUT_STATE_CH3 = crate::Reg<out_state_ch3::OUT_STATE_CH3_SPEC>;
#[doc = "TX CH3 state register"]
pub mod out_state_ch3;
#[doc = "OUT_EOF_DES_ADDR_CH3 (r) register accessor: TX CH3 eof des addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_eof_des_addr_ch3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_des_addr_ch3`] module"]
pub type OUT_EOF_DES_ADDR_CH3 = crate::Reg<out_eof_des_addr_ch3::OUT_EOF_DES_ADDR_CH3_SPEC>;
#[doc = "TX CH3 eof des addr register"]
pub mod out_eof_des_addr_ch3;
#[doc = "OUT_DSCR_CH3 (r) register accessor: TX CH3 next dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_ch3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_ch3`] module"]
pub type OUT_DSCR_CH3 = crate::Reg<out_dscr_ch3::OUT_DSCR_CH3_SPEC>;
#[doc = "TX CH3 next dscr addr register"]
pub mod out_dscr_ch3;
#[doc = "OUT_DSCR_BF0_CH3 (r) register accessor: TX CH3 last dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_bf0_ch3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf0_ch3`] module"]
pub type OUT_DSCR_BF0_CH3 = crate::Reg<out_dscr_bf0_ch3::OUT_DSCR_BF0_CH3_SPEC>;
#[doc = "TX CH3 last dscr addr register"]
pub mod out_dscr_bf0_ch3;
#[doc = "OUT_DSCR_BF1_CH3 (r) register accessor: TX CH3 second-to-last dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_bf1_ch3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf1_ch3`] module"]
pub type OUT_DSCR_BF1_CH3 = crate::Reg<out_dscr_bf1_ch3::OUT_DSCR_BF1_CH3_SPEC>;
#[doc = "TX CH3 second-to-last dscr addr register"]
pub mod out_dscr_bf1_ch3;
#[doc = "OUT_ARB_CH3 (rw) register accessor: TX CH3 arb register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_arb_ch3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_arb_ch3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_arb_ch3`] module"]
pub type OUT_ARB_CH3 = crate::Reg<out_arb_ch3::OUT_ARB_CH3_SPEC>;
#[doc = "TX CH3 arb register"]
pub mod out_arb_ch3;
#[doc = "OUT_ETM_CONF_CH3 (rw) register accessor: TX CH3 ETM config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_etm_conf_ch3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_etm_conf_ch3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_etm_conf_ch3`] module"]
pub type OUT_ETM_CONF_CH3 = crate::Reg<out_etm_conf_ch3::OUT_ETM_CONF_CH3_SPEC>;
#[doc = "TX CH3 ETM config register"]
pub mod out_etm_conf_ch3;
#[doc = "OUT_BUF_LEN_CH3 (r) register accessor: tx CH3 buf len register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_buf_len_ch3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_buf_len_ch3`] module"]
pub type OUT_BUF_LEN_CH3 = crate::Reg<out_buf_len_ch3::OUT_BUF_LEN_CH3_SPEC>;
#[doc = "tx CH3 buf len register"]
pub mod out_buf_len_ch3;
#[doc = "OUT_FIFO_BCNT_CH3 (r) register accessor: tx CH3 fifo byte cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_fifo_bcnt_ch3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_fifo_bcnt_ch3`] module"]
pub type OUT_FIFO_BCNT_CH3 = crate::Reg<out_fifo_bcnt_ch3::OUT_FIFO_BCNT_CH3_SPEC>;
#[doc = "tx CH3 fifo byte cnt register"]
pub mod out_fifo_bcnt_ch3;
#[doc = "OUT_PUSH_BYTECNT_CH3 (r) register accessor: tx CH3 push byte cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_push_bytecnt_ch3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_push_bytecnt_ch3`] module"]
pub type OUT_PUSH_BYTECNT_CH3 = crate::Reg<out_push_bytecnt_ch3::OUT_PUSH_BYTECNT_CH3_SPEC>;
#[doc = "tx CH3 push byte cnt register"]
pub mod out_push_bytecnt_ch3;
#[doc = "OUT_XADDR_CH3 (r) register accessor: tx CH3 xaddr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_xaddr_ch3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_xaddr_ch3`] module"]
pub type OUT_XADDR_CH3 = crate::Reg<out_xaddr_ch3::OUT_XADDR_CH3_SPEC>;
#[doc = "tx CH3 xaddr register"]
pub mod out_xaddr_ch3;
#[doc = "OUT_BLOCK_BUF_LEN_CH3 (r) register accessor: tx CH3 block buf len register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_block_buf_len_ch3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_block_buf_len_ch3`] module"]
pub type OUT_BLOCK_BUF_LEN_CH3 = crate::Reg<out_block_buf_len_ch3::OUT_BLOCK_BUF_LEN_CH3_SPEC>;
#[doc = "tx CH3 block buf len register"]
pub mod out_block_buf_len_ch3;
#[doc = "OUT_CONF0_CH4 (rw) register accessor: TX CH4 config0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf0_ch4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf0_ch4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf0_ch4`] module"]
pub type OUT_CONF0_CH4 = crate::Reg<out_conf0_ch4::OUT_CONF0_CH4_SPEC>;
#[doc = "TX CH4 config0 register"]
pub mod out_conf0_ch4;
#[doc = "OUT_INT_RAW_CH4 (rw) register accessor: TX CH4 interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_raw_ch4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_raw_ch4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_raw_ch4`] module"]
pub type OUT_INT_RAW_CH4 = crate::Reg<out_int_raw_ch4::OUT_INT_RAW_CH4_SPEC>;
#[doc = "TX CH4 interrupt raw register"]
pub mod out_int_raw_ch4;
#[doc = "OUT_INT_ENA_CH4 (rw) register accessor: TX CH4 interrupt ena register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_ena_ch4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_ena_ch4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_ena_ch4`] module"]
pub type OUT_INT_ENA_CH4 = crate::Reg<out_int_ena_ch4::OUT_INT_ENA_CH4_SPEC>;
#[doc = "TX CH4 interrupt ena register"]
pub mod out_int_ena_ch4;
#[doc = "OUT_INT_ST_CH4 (r) register accessor: TX CH4 interrupt st register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_st_ch4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_st_ch4`] module"]
pub type OUT_INT_ST_CH4 = crate::Reg<out_int_st_ch4::OUT_INT_ST_CH4_SPEC>;
#[doc = "TX CH4 interrupt st register"]
pub mod out_int_st_ch4;
#[doc = "OUT_INT_CLR_CH4 (w) register accessor: TX CH4 interrupt clr register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_clr_ch4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_clr_ch4`] module"]
pub type OUT_INT_CLR_CH4 = crate::Reg<out_int_clr_ch4::OUT_INT_CLR_CH4_SPEC>;
#[doc = "TX CH4 interrupt clr register"]
pub mod out_int_clr_ch4;
#[doc = "OUTFIFO_STATUS_CH4 (r) register accessor: TX CH4 outfifo status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outfifo_status_ch4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outfifo_status_ch4`] module"]
pub type OUTFIFO_STATUS_CH4 = crate::Reg<outfifo_status_ch4::OUTFIFO_STATUS_CH4_SPEC>;
#[doc = "TX CH4 outfifo status register"]
pub mod outfifo_status_ch4;
#[doc = "OUT_PUSH_CH4 (rw) register accessor: TX CH4 outfifo push register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_push_ch4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_push_ch4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_push_ch4`] module"]
pub type OUT_PUSH_CH4 = crate::Reg<out_push_ch4::OUT_PUSH_CH4_SPEC>;
#[doc = "TX CH4 outfifo push register"]
pub mod out_push_ch4;
#[doc = "OUT_LINK_CONF_CH4 (rw) register accessor: TX CH4 out_link dscr ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_link_conf_ch4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link_conf_ch4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link_conf_ch4`] module"]
pub type OUT_LINK_CONF_CH4 = crate::Reg<out_link_conf_ch4::OUT_LINK_CONF_CH4_SPEC>;
#[doc = "TX CH4 out_link dscr ctrl register"]
pub mod out_link_conf_ch4;
#[doc = "OUT_LINK_ADDR_CH4 (rw) register accessor: TX CH4 out_link dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_link_addr_ch4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link_addr_ch4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link_addr_ch4`] module"]
pub type OUT_LINK_ADDR_CH4 = crate::Reg<out_link_addr_ch4::OUT_LINK_ADDR_CH4_SPEC>;
#[doc = "TX CH4 out_link dscr addr register"]
pub mod out_link_addr_ch4;
#[doc = "OUT_STATE_CH4 (r) register accessor: TX CH4 state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_state_ch4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_state_ch4`] module"]
pub type OUT_STATE_CH4 = crate::Reg<out_state_ch4::OUT_STATE_CH4_SPEC>;
#[doc = "TX CH4 state register"]
pub mod out_state_ch4;
#[doc = "OUT_EOF_DES_ADDR_CH4 (r) register accessor: TX CH4 eof des addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_eof_des_addr_ch4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_des_addr_ch4`] module"]
pub type OUT_EOF_DES_ADDR_CH4 = crate::Reg<out_eof_des_addr_ch4::OUT_EOF_DES_ADDR_CH4_SPEC>;
#[doc = "TX CH4 eof des addr register"]
pub mod out_eof_des_addr_ch4;
#[doc = "OUT_DSCR_CH4 (r) register accessor: TX CH4 next dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_ch4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_ch4`] module"]
pub type OUT_DSCR_CH4 = crate::Reg<out_dscr_ch4::OUT_DSCR_CH4_SPEC>;
#[doc = "TX CH4 next dscr addr register"]
pub mod out_dscr_ch4;
#[doc = "OUT_DSCR_BF0_CH4 (r) register accessor: TX CH4 last dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_bf0_ch4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf0_ch4`] module"]
pub type OUT_DSCR_BF0_CH4 = crate::Reg<out_dscr_bf0_ch4::OUT_DSCR_BF0_CH4_SPEC>;
#[doc = "TX CH4 last dscr addr register"]
pub mod out_dscr_bf0_ch4;
#[doc = "OUT_DSCR_BF1_CH4 (r) register accessor: TX CH4 second-to-last dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_bf1_ch4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf1_ch4`] module"]
pub type OUT_DSCR_BF1_CH4 = crate::Reg<out_dscr_bf1_ch4::OUT_DSCR_BF1_CH4_SPEC>;
#[doc = "TX CH4 second-to-last dscr addr register"]
pub mod out_dscr_bf1_ch4;
#[doc = "OUT_ARB_CH4 (rw) register accessor: TX CH4 arb register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_arb_ch4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_arb_ch4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_arb_ch4`] module"]
pub type OUT_ARB_CH4 = crate::Reg<out_arb_ch4::OUT_ARB_CH4_SPEC>;
#[doc = "TX CH4 arb register"]
pub mod out_arb_ch4;
#[doc = "OUT_ETM_CONF_CH4 (rw) register accessor: TX CH4 ETM config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_etm_conf_ch4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_etm_conf_ch4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_etm_conf_ch4`] module"]
pub type OUT_ETM_CONF_CH4 = crate::Reg<out_etm_conf_ch4::OUT_ETM_CONF_CH4_SPEC>;
#[doc = "TX CH4 ETM config register"]
pub mod out_etm_conf_ch4;
#[doc = "OUT_BUF_LEN_CH4 (r) register accessor: tx CH4 buf len register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_buf_len_ch4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_buf_len_ch4`] module"]
pub type OUT_BUF_LEN_CH4 = crate::Reg<out_buf_len_ch4::OUT_BUF_LEN_CH4_SPEC>;
#[doc = "tx CH4 buf len register"]
pub mod out_buf_len_ch4;
#[doc = "OUT_FIFO_BCNT_CH4 (r) register accessor: tx CH4 fifo byte cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_fifo_bcnt_ch4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_fifo_bcnt_ch4`] module"]
pub type OUT_FIFO_BCNT_CH4 = crate::Reg<out_fifo_bcnt_ch4::OUT_FIFO_BCNT_CH4_SPEC>;
#[doc = "tx CH4 fifo byte cnt register"]
pub mod out_fifo_bcnt_ch4;
#[doc = "OUT_PUSH_BYTECNT_CH4 (r) register accessor: tx CH4 push byte cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_push_bytecnt_ch4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_push_bytecnt_ch4`] module"]
pub type OUT_PUSH_BYTECNT_CH4 = crate::Reg<out_push_bytecnt_ch4::OUT_PUSH_BYTECNT_CH4_SPEC>;
#[doc = "tx CH4 push byte cnt register"]
pub mod out_push_bytecnt_ch4;
#[doc = "OUT_XADDR_CH4 (r) register accessor: tx CH4 xaddr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_xaddr_ch4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_xaddr_ch4`] module"]
pub type OUT_XADDR_CH4 = crate::Reg<out_xaddr_ch4::OUT_XADDR_CH4_SPEC>;
#[doc = "tx CH4 xaddr register"]
pub mod out_xaddr_ch4;
#[doc = "OUT_BLOCK_BUF_LEN_CH4 (r) register accessor: tx CH4 block buf len register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_block_buf_len_ch4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_block_buf_len_ch4`] module"]
pub type OUT_BLOCK_BUF_LEN_CH4 = crate::Reg<out_block_buf_len_ch4::OUT_BLOCK_BUF_LEN_CH4_SPEC>;
#[doc = "tx CH4 block buf len register"]
pub mod out_block_buf_len_ch4;
#[doc = "IN_CONF0_CH0 (rw) register accessor: RX CH0 config0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf0_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf0_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf0_ch0`] module"]
pub type IN_CONF0_CH0 = crate::Reg<in_conf0_ch0::IN_CONF0_CH0_SPEC>;
#[doc = "RX CH0 config0 register"]
pub mod in_conf0_ch0;
#[doc = "IN_INT_RAW_CH0 (rw) register accessor: RX CH0 interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_raw_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_raw_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_raw_ch0`] module"]
pub type IN_INT_RAW_CH0 = crate::Reg<in_int_raw_ch0::IN_INT_RAW_CH0_SPEC>;
#[doc = "RX CH0 interrupt raw register"]
pub mod in_int_raw_ch0;
#[doc = "IN_INT_ENA_CH0 (rw) register accessor: RX CH0 interrupt ena register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_ena_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_ena_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_ena_ch0`] module"]
pub type IN_INT_ENA_CH0 = crate::Reg<in_int_ena_ch0::IN_INT_ENA_CH0_SPEC>;
#[doc = "RX CH0 interrupt ena register"]
pub mod in_int_ena_ch0;
#[doc = "IN_INT_ST_CH0 (r) register accessor: RX CH0 interrupt st register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_st_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_st_ch0`] module"]
pub type IN_INT_ST_CH0 = crate::Reg<in_int_st_ch0::IN_INT_ST_CH0_SPEC>;
#[doc = "RX CH0 interrupt st register"]
pub mod in_int_st_ch0;
#[doc = "IN_INT_CLR_CH0 (w) register accessor: RX CH0 interrupt clr register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_clr_ch0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_clr_ch0`] module"]
pub type IN_INT_CLR_CH0 = crate::Reg<in_int_clr_ch0::IN_INT_CLR_CH0_SPEC>;
#[doc = "RX CH0 interrupt clr register"]
pub mod in_int_clr_ch0;
#[doc = "INFIFO_STATUS_CH0 (r) register accessor: RX CH0 INFIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`infifo_status_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@infifo_status_ch0`] module"]
pub type INFIFO_STATUS_CH0 = crate::Reg<infifo_status_ch0::INFIFO_STATUS_CH0_SPEC>;
#[doc = "RX CH0 INFIFO status register"]
pub mod infifo_status_ch0;
#[doc = "IN_POP_CH0 (rw) register accessor: RX CH0 INFIFO pop register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pop_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_pop_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pop_ch0`] module"]
pub type IN_POP_CH0 = crate::Reg<in_pop_ch0::IN_POP_CH0_SPEC>;
#[doc = "RX CH0 INFIFO pop register"]
pub mod in_pop_ch0;
#[doc = "IN_LINK_CONF_CH0 (rw) register accessor: RX CH0 in_link dscr ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_link_conf_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_link_conf_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link_conf_ch0`] module"]
pub type IN_LINK_CONF_CH0 = crate::Reg<in_link_conf_ch0::IN_LINK_CONF_CH0_SPEC>;
#[doc = "RX CH0 in_link dscr ctrl register"]
pub mod in_link_conf_ch0;
#[doc = "IN_LINK_ADDR_CH0 (rw) register accessor: RX CH0 in_link dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_link_addr_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_link_addr_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link_addr_ch0`] module"]
pub type IN_LINK_ADDR_CH0 = crate::Reg<in_link_addr_ch0::IN_LINK_ADDR_CH0_SPEC>;
#[doc = "RX CH0 in_link dscr addr register"]
pub mod in_link_addr_ch0;
#[doc = "IN_STATE_CH0 (r) register accessor: RX CH0 state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_state_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_state_ch0`] module"]
pub type IN_STATE_CH0 = crate::Reg<in_state_ch0::IN_STATE_CH0_SPEC>;
#[doc = "RX CH0 state register"]
pub mod in_state_ch0;
#[doc = "IN_SUC_EOF_DES_ADDR_CH0 (r) register accessor: RX CH0 eof des addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_suc_eof_des_addr_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_suc_eof_des_addr_ch0`] module"]
pub type IN_SUC_EOF_DES_ADDR_CH0 =
    crate::Reg<in_suc_eof_des_addr_ch0::IN_SUC_EOF_DES_ADDR_CH0_SPEC>;
#[doc = "RX CH0 eof des addr register"]
pub mod in_suc_eof_des_addr_ch0;
#[doc = "IN_ERR_EOF_DES_ADDR_CH0 (r) register accessor: RX CH0 err eof des addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_err_eof_des_addr_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_err_eof_des_addr_ch0`] module"]
pub type IN_ERR_EOF_DES_ADDR_CH0 =
    crate::Reg<in_err_eof_des_addr_ch0::IN_ERR_EOF_DES_ADDR_CH0_SPEC>;
#[doc = "RX CH0 err eof des addr register"]
pub mod in_err_eof_des_addr_ch0;
#[doc = "IN_DSCR_CH0 (r) register accessor: RX CH0 next dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_ch0`] module"]
pub type IN_DSCR_CH0 = crate::Reg<in_dscr_ch0::IN_DSCR_CH0_SPEC>;
#[doc = "RX CH0 next dscr addr register"]
pub mod in_dscr_ch0;
#[doc = "IN_DSCR_BF0_CH0 (r) register accessor: RX CH0 last dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_bf0_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf0_ch0`] module"]
pub type IN_DSCR_BF0_CH0 = crate::Reg<in_dscr_bf0_ch0::IN_DSCR_BF0_CH0_SPEC>;
#[doc = "RX CH0 last dscr addr register"]
pub mod in_dscr_bf0_ch0;
#[doc = "IN_DSCR_BF1_CH0 (r) register accessor: RX CH0 second-to-last dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_bf1_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf1_ch0`] module"]
pub type IN_DSCR_BF1_CH0 = crate::Reg<in_dscr_bf1_ch0::IN_DSCR_BF1_CH0_SPEC>;
#[doc = "RX CH0 second-to-last dscr addr register"]
pub mod in_dscr_bf1_ch0;
#[doc = "IN_ARB_CH0 (rw) register accessor: RX CH0 arb register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_arb_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_arb_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_arb_ch0`] module"]
pub type IN_ARB_CH0 = crate::Reg<in_arb_ch0::IN_ARB_CH0_SPEC>;
#[doc = "RX CH0 arb register"]
pub mod in_arb_ch0;
#[doc = "IN_RO_PD_CONF_CH0 (rw) register accessor: RX CH0 reorder power config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_ro_pd_conf_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_ro_pd_conf_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_ro_pd_conf_ch0`] module"]
pub type IN_RO_PD_CONF_CH0 = crate::Reg<in_ro_pd_conf_ch0::IN_RO_PD_CONF_CH0_SPEC>;
#[doc = "RX CH0 reorder power config register"]
pub mod in_ro_pd_conf_ch0;
#[doc = "IN_ETM_CONF_CH0 (rw) register accessor: RX CH0 ETM config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_etm_conf_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_etm_conf_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_etm_conf_ch0`] module"]
pub type IN_ETM_CONF_CH0 = crate::Reg<in_etm_conf_ch0::IN_ETM_CONF_CH0_SPEC>;
#[doc = "RX CH0 ETM config register"]
pub mod in_etm_conf_ch0;
#[doc = "IN_FIFO_CNT_CH0 (r) register accessor: rx CH0 fifo cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_fifo_cnt_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_fifo_cnt_ch0`] module"]
pub type IN_FIFO_CNT_CH0 = crate::Reg<in_fifo_cnt_ch0::IN_FIFO_CNT_CH0_SPEC>;
#[doc = "rx CH0 fifo cnt register"]
pub mod in_fifo_cnt_ch0;
#[doc = "IN_POP_DATA_CNT_CH0 (r) register accessor: rx CH0 pop data cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pop_data_cnt_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pop_data_cnt_ch0`] module"]
pub type IN_POP_DATA_CNT_CH0 = crate::Reg<in_pop_data_cnt_ch0::IN_POP_DATA_CNT_CH0_SPEC>;
#[doc = "rx CH0 pop data cnt register"]
pub mod in_pop_data_cnt_ch0;
#[doc = "IN_XADDR_CH0 (r) register accessor: rx CH0 xaddr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_xaddr_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_xaddr_ch0`] module"]
pub type IN_XADDR_CH0 = crate::Reg<in_xaddr_ch0::IN_XADDR_CH0_SPEC>;
#[doc = "rx CH0 xaddr register"]
pub mod in_xaddr_ch0;
#[doc = "IN_BUF_HB_RCV_CH0 (r) register accessor: rx CH0 buf len hb rcv register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_buf_hb_rcv_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_buf_hb_rcv_ch0`] module"]
pub type IN_BUF_HB_RCV_CH0 = crate::Reg<in_buf_hb_rcv_ch0::IN_BUF_HB_RCV_CH0_SPEC>;
#[doc = "rx CH0 buf len hb rcv register"]
pub mod in_buf_hb_rcv_ch0;
#[doc = "IN_CONF0_CH1 (rw) register accessor: RX CH1 config0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf0_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf0_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf0_ch1`] module"]
pub type IN_CONF0_CH1 = crate::Reg<in_conf0_ch1::IN_CONF0_CH1_SPEC>;
#[doc = "RX CH1 config0 register"]
pub mod in_conf0_ch1;
#[doc = "IN_INT_RAW_CH1 (rw) register accessor: RX CH1 interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_raw_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_raw_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_raw_ch1`] module"]
pub type IN_INT_RAW_CH1 = crate::Reg<in_int_raw_ch1::IN_INT_RAW_CH1_SPEC>;
#[doc = "RX CH1 interrupt raw register"]
pub mod in_int_raw_ch1;
#[doc = "IN_INT_ENA_CH1 (rw) register accessor: RX CH1 interrupt ena register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_ena_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_ena_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_ena_ch1`] module"]
pub type IN_INT_ENA_CH1 = crate::Reg<in_int_ena_ch1::IN_INT_ENA_CH1_SPEC>;
#[doc = "RX CH1 interrupt ena register"]
pub mod in_int_ena_ch1;
#[doc = "IN_INT_ST_CH1 (r) register accessor: RX CH1 interrupt st register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_st_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_st_ch1`] module"]
pub type IN_INT_ST_CH1 = crate::Reg<in_int_st_ch1::IN_INT_ST_CH1_SPEC>;
#[doc = "RX CH1 interrupt st register"]
pub mod in_int_st_ch1;
#[doc = "IN_INT_CLR_CH1 (w) register accessor: RX CH1 interrupt clr register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_clr_ch1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_clr_ch1`] module"]
pub type IN_INT_CLR_CH1 = crate::Reg<in_int_clr_ch1::IN_INT_CLR_CH1_SPEC>;
#[doc = "RX CH1 interrupt clr register"]
pub mod in_int_clr_ch1;
#[doc = "INFIFO_STATUS_CH1 (r) register accessor: RX CH1 INFIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`infifo_status_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@infifo_status_ch1`] module"]
pub type INFIFO_STATUS_CH1 = crate::Reg<infifo_status_ch1::INFIFO_STATUS_CH1_SPEC>;
#[doc = "RX CH1 INFIFO status register"]
pub mod infifo_status_ch1;
#[doc = "IN_POP_CH1 (rw) register accessor: RX CH1 INFIFO pop register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pop_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_pop_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pop_ch1`] module"]
pub type IN_POP_CH1 = crate::Reg<in_pop_ch1::IN_POP_CH1_SPEC>;
#[doc = "RX CH1 INFIFO pop register"]
pub mod in_pop_ch1;
#[doc = "IN_LINK_CONF_CH1 (rw) register accessor: RX CH1 in_link dscr ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_link_conf_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_link_conf_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link_conf_ch1`] module"]
pub type IN_LINK_CONF_CH1 = crate::Reg<in_link_conf_ch1::IN_LINK_CONF_CH1_SPEC>;
#[doc = "RX CH1 in_link dscr ctrl register"]
pub mod in_link_conf_ch1;
#[doc = "IN_LINK_ADDR_CH1 (rw) register accessor: RX CH1 in_link dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_link_addr_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_link_addr_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link_addr_ch1`] module"]
pub type IN_LINK_ADDR_CH1 = crate::Reg<in_link_addr_ch1::IN_LINK_ADDR_CH1_SPEC>;
#[doc = "RX CH1 in_link dscr addr register"]
pub mod in_link_addr_ch1;
#[doc = "IN_STATE_CH1 (r) register accessor: RX CH1 state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_state_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_state_ch1`] module"]
pub type IN_STATE_CH1 = crate::Reg<in_state_ch1::IN_STATE_CH1_SPEC>;
#[doc = "RX CH1 state register"]
pub mod in_state_ch1;
#[doc = "IN_SUC_EOF_DES_ADDR_CH1 (r) register accessor: RX CH1 eof des addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_suc_eof_des_addr_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_suc_eof_des_addr_ch1`] module"]
pub type IN_SUC_EOF_DES_ADDR_CH1 =
    crate::Reg<in_suc_eof_des_addr_ch1::IN_SUC_EOF_DES_ADDR_CH1_SPEC>;
#[doc = "RX CH1 eof des addr register"]
pub mod in_suc_eof_des_addr_ch1;
#[doc = "IN_ERR_EOF_DES_ADDR_CH1 (r) register accessor: RX CH1 err eof des addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_err_eof_des_addr_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_err_eof_des_addr_ch1`] module"]
pub type IN_ERR_EOF_DES_ADDR_CH1 =
    crate::Reg<in_err_eof_des_addr_ch1::IN_ERR_EOF_DES_ADDR_CH1_SPEC>;
#[doc = "RX CH1 err eof des addr register"]
pub mod in_err_eof_des_addr_ch1;
#[doc = "IN_DSCR_CH1 (r) register accessor: RX CH1 next dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_ch1`] module"]
pub type IN_DSCR_CH1 = crate::Reg<in_dscr_ch1::IN_DSCR_CH1_SPEC>;
#[doc = "RX CH1 next dscr addr register"]
pub mod in_dscr_ch1;
#[doc = "IN_DSCR_BF0_CH1 (r) register accessor: RX CH1 last dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_bf0_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf0_ch1`] module"]
pub type IN_DSCR_BF0_CH1 = crate::Reg<in_dscr_bf0_ch1::IN_DSCR_BF0_CH1_SPEC>;
#[doc = "RX CH1 last dscr addr register"]
pub mod in_dscr_bf0_ch1;
#[doc = "IN_DSCR_BF1_CH1 (r) register accessor: RX CH1 second-to-last dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_bf1_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf1_ch1`] module"]
pub type IN_DSCR_BF1_CH1 = crate::Reg<in_dscr_bf1_ch1::IN_DSCR_BF1_CH1_SPEC>;
#[doc = "RX CH1 second-to-last dscr addr register"]
pub mod in_dscr_bf1_ch1;
#[doc = "IN_ARB_CH1 (rw) register accessor: RX CH1 arb register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_arb_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_arb_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_arb_ch1`] module"]
pub type IN_ARB_CH1 = crate::Reg<in_arb_ch1::IN_ARB_CH1_SPEC>;
#[doc = "RX CH1 arb register"]
pub mod in_arb_ch1;
#[doc = "IN_ETM_CONF_CH1 (rw) register accessor: RX CH1 ETM config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_etm_conf_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_etm_conf_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_etm_conf_ch1`] module"]
pub type IN_ETM_CONF_CH1 = crate::Reg<in_etm_conf_ch1::IN_ETM_CONF_CH1_SPEC>;
#[doc = "RX CH1 ETM config register"]
pub mod in_etm_conf_ch1;
#[doc = "IN_FIFO_CNT_CH1 (r) register accessor: rx CH1 fifo cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_fifo_cnt_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_fifo_cnt_ch1`] module"]
pub type IN_FIFO_CNT_CH1 = crate::Reg<in_fifo_cnt_ch1::IN_FIFO_CNT_CH1_SPEC>;
#[doc = "rx CH1 fifo cnt register"]
pub mod in_fifo_cnt_ch1;
#[doc = "IN_POP_DATA_CNT_CH1 (r) register accessor: rx CH1 pop data cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pop_data_cnt_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pop_data_cnt_ch1`] module"]
pub type IN_POP_DATA_CNT_CH1 = crate::Reg<in_pop_data_cnt_ch1::IN_POP_DATA_CNT_CH1_SPEC>;
#[doc = "rx CH1 pop data cnt register"]
pub mod in_pop_data_cnt_ch1;
#[doc = "IN_XADDR_CH1 (r) register accessor: rx CH1 xaddr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_xaddr_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_xaddr_ch1`] module"]
pub type IN_XADDR_CH1 = crate::Reg<in_xaddr_ch1::IN_XADDR_CH1_SPEC>;
#[doc = "rx CH1 xaddr register"]
pub mod in_xaddr_ch1;
#[doc = "IN_BUF_HB_RCV_CH1 (r) register accessor: rx CH1 buf len hb rcv register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_buf_hb_rcv_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_buf_hb_rcv_ch1`] module"]
pub type IN_BUF_HB_RCV_CH1 = crate::Reg<in_buf_hb_rcv_ch1::IN_BUF_HB_RCV_CH1_SPEC>;
#[doc = "rx CH1 buf len hb rcv register"]
pub mod in_buf_hb_rcv_ch1;
#[doc = "IN_CONF0_CH2 (rw) register accessor: RX CH2 config0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf0_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf0_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf0_ch2`] module"]
pub type IN_CONF0_CH2 = crate::Reg<in_conf0_ch2::IN_CONF0_CH2_SPEC>;
#[doc = "RX CH2 config0 register"]
pub mod in_conf0_ch2;
#[doc = "IN_INT_RAW_CH2 (rw) register accessor: RX CH2 interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_raw_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_raw_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_raw_ch2`] module"]
pub type IN_INT_RAW_CH2 = crate::Reg<in_int_raw_ch2::IN_INT_RAW_CH2_SPEC>;
#[doc = "RX CH2 interrupt raw register"]
pub mod in_int_raw_ch2;
#[doc = "IN_INT_ENA_CH2 (rw) register accessor: RX CH2 interrupt ena register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_ena_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_ena_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_ena_ch2`] module"]
pub type IN_INT_ENA_CH2 = crate::Reg<in_int_ena_ch2::IN_INT_ENA_CH2_SPEC>;
#[doc = "RX CH2 interrupt ena register"]
pub mod in_int_ena_ch2;
#[doc = "IN_INT_ST_CH2 (r) register accessor: RX CH2 interrupt st register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_st_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_st_ch2`] module"]
pub type IN_INT_ST_CH2 = crate::Reg<in_int_st_ch2::IN_INT_ST_CH2_SPEC>;
#[doc = "RX CH2 interrupt st register"]
pub mod in_int_st_ch2;
#[doc = "IN_INT_CLR_CH2 (w) register accessor: RX CH2 interrupt clr register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_clr_ch2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_clr_ch2`] module"]
pub type IN_INT_CLR_CH2 = crate::Reg<in_int_clr_ch2::IN_INT_CLR_CH2_SPEC>;
#[doc = "RX CH2 interrupt clr register"]
pub mod in_int_clr_ch2;
#[doc = "INFIFO_STATUS_CH2 (r) register accessor: RX CH2 INFIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`infifo_status_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@infifo_status_ch2`] module"]
pub type INFIFO_STATUS_CH2 = crate::Reg<infifo_status_ch2::INFIFO_STATUS_CH2_SPEC>;
#[doc = "RX CH2 INFIFO status register"]
pub mod infifo_status_ch2;
#[doc = "IN_POP_CH2 (rw) register accessor: RX CH2 INFIFO pop register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pop_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_pop_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pop_ch2`] module"]
pub type IN_POP_CH2 = crate::Reg<in_pop_ch2::IN_POP_CH2_SPEC>;
#[doc = "RX CH2 INFIFO pop register"]
pub mod in_pop_ch2;
#[doc = "IN_LINK_CONF_CH2 (rw) register accessor: RX CH2 in_link dscr ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_link_conf_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_link_conf_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link_conf_ch2`] module"]
pub type IN_LINK_CONF_CH2 = crate::Reg<in_link_conf_ch2::IN_LINK_CONF_CH2_SPEC>;
#[doc = "RX CH2 in_link dscr ctrl register"]
pub mod in_link_conf_ch2;
#[doc = "IN_LINK_ADDR_CH2 (rw) register accessor: RX CH2 in_link dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_link_addr_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_link_addr_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link_addr_ch2`] module"]
pub type IN_LINK_ADDR_CH2 = crate::Reg<in_link_addr_ch2::IN_LINK_ADDR_CH2_SPEC>;
#[doc = "RX CH2 in_link dscr addr register"]
pub mod in_link_addr_ch2;
#[doc = "IN_STATE_CH2 (r) register accessor: RX CH2 state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_state_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_state_ch2`] module"]
pub type IN_STATE_CH2 = crate::Reg<in_state_ch2::IN_STATE_CH2_SPEC>;
#[doc = "RX CH2 state register"]
pub mod in_state_ch2;
#[doc = "IN_SUC_EOF_DES_ADDR_CH2 (r) register accessor: RX CH2 eof des addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_suc_eof_des_addr_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_suc_eof_des_addr_ch2`] module"]
pub type IN_SUC_EOF_DES_ADDR_CH2 =
    crate::Reg<in_suc_eof_des_addr_ch2::IN_SUC_EOF_DES_ADDR_CH2_SPEC>;
#[doc = "RX CH2 eof des addr register"]
pub mod in_suc_eof_des_addr_ch2;
#[doc = "IN_ERR_EOF_DES_ADDR_CH2 (r) register accessor: RX CH2 err eof des addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_err_eof_des_addr_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_err_eof_des_addr_ch2`] module"]
pub type IN_ERR_EOF_DES_ADDR_CH2 =
    crate::Reg<in_err_eof_des_addr_ch2::IN_ERR_EOF_DES_ADDR_CH2_SPEC>;
#[doc = "RX CH2 err eof des addr register"]
pub mod in_err_eof_des_addr_ch2;
#[doc = "IN_DSCR_CH2 (r) register accessor: RX CH2 next dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_ch2`] module"]
pub type IN_DSCR_CH2 = crate::Reg<in_dscr_ch2::IN_DSCR_CH2_SPEC>;
#[doc = "RX CH2 next dscr addr register"]
pub mod in_dscr_ch2;
#[doc = "IN_DSCR_BF0_CH2 (r) register accessor: RX CH2 last dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_bf0_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf0_ch2`] module"]
pub type IN_DSCR_BF0_CH2 = crate::Reg<in_dscr_bf0_ch2::IN_DSCR_BF0_CH2_SPEC>;
#[doc = "RX CH2 last dscr addr register"]
pub mod in_dscr_bf0_ch2;
#[doc = "IN_DSCR_BF1_CH2 (r) register accessor: RX CH2 second-to-last dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_bf1_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf1_ch2`] module"]
pub type IN_DSCR_BF1_CH2 = crate::Reg<in_dscr_bf1_ch2::IN_DSCR_BF1_CH2_SPEC>;
#[doc = "RX CH2 second-to-last dscr addr register"]
pub mod in_dscr_bf1_ch2;
#[doc = "IN_ARB_CH2 (rw) register accessor: RX CH2 arb register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_arb_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_arb_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_arb_ch2`] module"]
pub type IN_ARB_CH2 = crate::Reg<in_arb_ch2::IN_ARB_CH2_SPEC>;
#[doc = "RX CH2 arb register"]
pub mod in_arb_ch2;
#[doc = "IN_ETM_CONF_CH2 (rw) register accessor: RX CH2 ETM config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_etm_conf_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_etm_conf_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_etm_conf_ch2`] module"]
pub type IN_ETM_CONF_CH2 = crate::Reg<in_etm_conf_ch2::IN_ETM_CONF_CH2_SPEC>;
#[doc = "RX CH2 ETM config register"]
pub mod in_etm_conf_ch2;
#[doc = "IN_FIFO_CNT_CH2 (r) register accessor: rx CH2 fifo cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_fifo_cnt_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_fifo_cnt_ch2`] module"]
pub type IN_FIFO_CNT_CH2 = crate::Reg<in_fifo_cnt_ch2::IN_FIFO_CNT_CH2_SPEC>;
#[doc = "rx CH2 fifo cnt register"]
pub mod in_fifo_cnt_ch2;
#[doc = "IN_POP_DATA_CNT_CH2 (r) register accessor: rx CH2 pop data cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pop_data_cnt_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pop_data_cnt_ch2`] module"]
pub type IN_POP_DATA_CNT_CH2 = crate::Reg<in_pop_data_cnt_ch2::IN_POP_DATA_CNT_CH2_SPEC>;
#[doc = "rx CH2 pop data cnt register"]
pub mod in_pop_data_cnt_ch2;
#[doc = "IN_XADDR_CH2 (r) register accessor: rx CH2 xaddr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_xaddr_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_xaddr_ch2`] module"]
pub type IN_XADDR_CH2 = crate::Reg<in_xaddr_ch2::IN_XADDR_CH2_SPEC>;
#[doc = "rx CH2 xaddr register"]
pub mod in_xaddr_ch2;
#[doc = "IN_BUF_HB_RCV_CH2 (r) register accessor: rx CH2 buf len hb rcv register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_buf_hb_rcv_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_buf_hb_rcv_ch2`] module"]
pub type IN_BUF_HB_RCV_CH2 = crate::Reg<in_buf_hb_rcv_ch2::IN_BUF_HB_RCV_CH2_SPEC>;
#[doc = "rx CH2 buf len hb rcv register"]
pub mod in_buf_hb_rcv_ch2;
#[doc = "IN_CONF0_CH3 (rw) register accessor: RX CH3 config0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf0_ch3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf0_ch3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf0_ch3`] module"]
pub type IN_CONF0_CH3 = crate::Reg<in_conf0_ch3::IN_CONF0_CH3_SPEC>;
#[doc = "RX CH3 config0 register"]
pub mod in_conf0_ch3;
#[doc = "IN_INT_RAW_CH3 (rw) register accessor: RX CH3 interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_raw_ch3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_raw_ch3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_raw_ch3`] module"]
pub type IN_INT_RAW_CH3 = crate::Reg<in_int_raw_ch3::IN_INT_RAW_CH3_SPEC>;
#[doc = "RX CH3 interrupt raw register"]
pub mod in_int_raw_ch3;
#[doc = "IN_INT_ENA_CH3 (rw) register accessor: RX CH3 interrupt ena register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_ena_ch3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_ena_ch3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_ena_ch3`] module"]
pub type IN_INT_ENA_CH3 = crate::Reg<in_int_ena_ch3::IN_INT_ENA_CH3_SPEC>;
#[doc = "RX CH3 interrupt ena register"]
pub mod in_int_ena_ch3;
#[doc = "IN_INT_ST_CH3 (r) register accessor: RX CH3 interrupt st register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_st_ch3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_st_ch3`] module"]
pub type IN_INT_ST_CH3 = crate::Reg<in_int_st_ch3::IN_INT_ST_CH3_SPEC>;
#[doc = "RX CH3 interrupt st register"]
pub mod in_int_st_ch3;
#[doc = "IN_INT_CLR_CH3 (w) register accessor: RX CH3 interrupt clr register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_clr_ch3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_clr_ch3`] module"]
pub type IN_INT_CLR_CH3 = crate::Reg<in_int_clr_ch3::IN_INT_CLR_CH3_SPEC>;
#[doc = "RX CH3 interrupt clr register"]
pub mod in_int_clr_ch3;
#[doc = "INFIFO_STATUS_CH3 (r) register accessor: RX CH3 INFIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`infifo_status_ch3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@infifo_status_ch3`] module"]
pub type INFIFO_STATUS_CH3 = crate::Reg<infifo_status_ch3::INFIFO_STATUS_CH3_SPEC>;
#[doc = "RX CH3 INFIFO status register"]
pub mod infifo_status_ch3;
#[doc = "IN_POP_CH3 (rw) register accessor: RX CH3 INFIFO pop register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pop_ch3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_pop_ch3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pop_ch3`] module"]
pub type IN_POP_CH3 = crate::Reg<in_pop_ch3::IN_POP_CH3_SPEC>;
#[doc = "RX CH3 INFIFO pop register"]
pub mod in_pop_ch3;
#[doc = "IN_LINK_CONF_CH3 (rw) register accessor: RX CH3 in_link dscr ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_link_conf_ch3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_link_conf_ch3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link_conf_ch3`] module"]
pub type IN_LINK_CONF_CH3 = crate::Reg<in_link_conf_ch3::IN_LINK_CONF_CH3_SPEC>;
#[doc = "RX CH3 in_link dscr ctrl register"]
pub mod in_link_conf_ch3;
#[doc = "IN_LINK_ADDR_CH3 (rw) register accessor: RX CH3 in_link dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_link_addr_ch3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_link_addr_ch3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link_addr_ch3`] module"]
pub type IN_LINK_ADDR_CH3 = crate::Reg<in_link_addr_ch3::IN_LINK_ADDR_CH3_SPEC>;
#[doc = "RX CH3 in_link dscr addr register"]
pub mod in_link_addr_ch3;
#[doc = "IN_STATE_CH3 (r) register accessor: RX CH3 state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_state_ch3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_state_ch3`] module"]
pub type IN_STATE_CH3 = crate::Reg<in_state_ch3::IN_STATE_CH3_SPEC>;
#[doc = "RX CH3 state register"]
pub mod in_state_ch3;
#[doc = "IN_SUC_EOF_DES_ADDR_CH3 (r) register accessor: RX CH3 eof des addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_suc_eof_des_addr_ch3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_suc_eof_des_addr_ch3`] module"]
pub type IN_SUC_EOF_DES_ADDR_CH3 =
    crate::Reg<in_suc_eof_des_addr_ch3::IN_SUC_EOF_DES_ADDR_CH3_SPEC>;
#[doc = "RX CH3 eof des addr register"]
pub mod in_suc_eof_des_addr_ch3;
#[doc = "IN_ERR_EOF_DES_ADDR_CH3 (r) register accessor: RX CH3 err eof des addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_err_eof_des_addr_ch3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_err_eof_des_addr_ch3`] module"]
pub type IN_ERR_EOF_DES_ADDR_CH3 =
    crate::Reg<in_err_eof_des_addr_ch3::IN_ERR_EOF_DES_ADDR_CH3_SPEC>;
#[doc = "RX CH3 err eof des addr register"]
pub mod in_err_eof_des_addr_ch3;
#[doc = "IN_DSCR_CH3 (r) register accessor: RX CH3 next dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_ch3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_ch3`] module"]
pub type IN_DSCR_CH3 = crate::Reg<in_dscr_ch3::IN_DSCR_CH3_SPEC>;
#[doc = "RX CH3 next dscr addr register"]
pub mod in_dscr_ch3;
#[doc = "IN_DSCR_BF0_CH3 (r) register accessor: RX CH3 last dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_bf0_ch3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf0_ch3`] module"]
pub type IN_DSCR_BF0_CH3 = crate::Reg<in_dscr_bf0_ch3::IN_DSCR_BF0_CH3_SPEC>;
#[doc = "RX CH3 last dscr addr register"]
pub mod in_dscr_bf0_ch3;
#[doc = "IN_DSCR_BF1_CH3 (r) register accessor: RX CH3 second-to-last dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_bf1_ch3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf1_ch3`] module"]
pub type IN_DSCR_BF1_CH3 = crate::Reg<in_dscr_bf1_ch3::IN_DSCR_BF1_CH3_SPEC>;
#[doc = "RX CH3 second-to-last dscr addr register"]
pub mod in_dscr_bf1_ch3;
#[doc = "IN_ARB_CH3 (rw) register accessor: RX CH3 arb register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_arb_ch3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_arb_ch3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_arb_ch3`] module"]
pub type IN_ARB_CH3 = crate::Reg<in_arb_ch3::IN_ARB_CH3_SPEC>;
#[doc = "RX CH3 arb register"]
pub mod in_arb_ch3;
#[doc = "IN_ETM_CONF_CH3 (rw) register accessor: RX CH3 ETM config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_etm_conf_ch3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_etm_conf_ch3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_etm_conf_ch3`] module"]
pub type IN_ETM_CONF_CH3 = crate::Reg<in_etm_conf_ch3::IN_ETM_CONF_CH3_SPEC>;
#[doc = "RX CH3 ETM config register"]
pub mod in_etm_conf_ch3;
#[doc = "IN_FIFO_CNT_CH3 (r) register accessor: rx CH3 fifo cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_fifo_cnt_ch3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_fifo_cnt_ch3`] module"]
pub type IN_FIFO_CNT_CH3 = crate::Reg<in_fifo_cnt_ch3::IN_FIFO_CNT_CH3_SPEC>;
#[doc = "rx CH3 fifo cnt register"]
pub mod in_fifo_cnt_ch3;
#[doc = "IN_POP_DATA_CNT_CH3 (r) register accessor: rx CH3 pop data cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pop_data_cnt_ch3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pop_data_cnt_ch3`] module"]
pub type IN_POP_DATA_CNT_CH3 = crate::Reg<in_pop_data_cnt_ch3::IN_POP_DATA_CNT_CH3_SPEC>;
#[doc = "rx CH3 pop data cnt register"]
pub mod in_pop_data_cnt_ch3;
#[doc = "IN_XADDR_CH3 (r) register accessor: rx CH3 xaddr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_xaddr_ch3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_xaddr_ch3`] module"]
pub type IN_XADDR_CH3 = crate::Reg<in_xaddr_ch3::IN_XADDR_CH3_SPEC>;
#[doc = "rx CH3 xaddr register"]
pub mod in_xaddr_ch3;
#[doc = "IN_BUF_HB_RCV_CH3 (r) register accessor: rx CH3 buf len hb rcv register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_buf_hb_rcv_ch3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_buf_hb_rcv_ch3`] module"]
pub type IN_BUF_HB_RCV_CH3 = crate::Reg<in_buf_hb_rcv_ch3::IN_BUF_HB_RCV_CH3_SPEC>;
#[doc = "rx CH3 buf len hb rcv register"]
pub mod in_buf_hb_rcv_ch3;
#[doc = "IN_CONF0_CH4 (rw) register accessor: RX CH4 config0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf0_ch4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf0_ch4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf0_ch4`] module"]
pub type IN_CONF0_CH4 = crate::Reg<in_conf0_ch4::IN_CONF0_CH4_SPEC>;
#[doc = "RX CH4 config0 register"]
pub mod in_conf0_ch4;
#[doc = "IN_INT_RAW_CH4 (rw) register accessor: RX CH4 interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_raw_ch4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_raw_ch4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_raw_ch4`] module"]
pub type IN_INT_RAW_CH4 = crate::Reg<in_int_raw_ch4::IN_INT_RAW_CH4_SPEC>;
#[doc = "RX CH4 interrupt raw register"]
pub mod in_int_raw_ch4;
#[doc = "IN_INT_ENA_CH4 (rw) register accessor: RX CH4 interrupt ena register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_ena_ch4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_ena_ch4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_ena_ch4`] module"]
pub type IN_INT_ENA_CH4 = crate::Reg<in_int_ena_ch4::IN_INT_ENA_CH4_SPEC>;
#[doc = "RX CH4 interrupt ena register"]
pub mod in_int_ena_ch4;
#[doc = "IN_INT_ST_CH4 (r) register accessor: RX CH4 interrupt st register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_st_ch4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_st_ch4`] module"]
pub type IN_INT_ST_CH4 = crate::Reg<in_int_st_ch4::IN_INT_ST_CH4_SPEC>;
#[doc = "RX CH4 interrupt st register"]
pub mod in_int_st_ch4;
#[doc = "IN_INT_CLR_CH4 (w) register accessor: RX CH4 interrupt clr register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_clr_ch4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_clr_ch4`] module"]
pub type IN_INT_CLR_CH4 = crate::Reg<in_int_clr_ch4::IN_INT_CLR_CH4_SPEC>;
#[doc = "RX CH4 interrupt clr register"]
pub mod in_int_clr_ch4;
#[doc = "INFIFO_STATUS_CH4 (r) register accessor: RX CH4 INFIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`infifo_status_ch4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@infifo_status_ch4`] module"]
pub type INFIFO_STATUS_CH4 = crate::Reg<infifo_status_ch4::INFIFO_STATUS_CH4_SPEC>;
#[doc = "RX CH4 INFIFO status register"]
pub mod infifo_status_ch4;
#[doc = "IN_POP_CH4 (rw) register accessor: RX CH4 INFIFO pop register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pop_ch4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_pop_ch4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pop_ch4`] module"]
pub type IN_POP_CH4 = crate::Reg<in_pop_ch4::IN_POP_CH4_SPEC>;
#[doc = "RX CH4 INFIFO pop register"]
pub mod in_pop_ch4;
#[doc = "IN_LINK_CONF_CH4 (rw) register accessor: RX CH4 in_link dscr ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_link_conf_ch4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_link_conf_ch4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link_conf_ch4`] module"]
pub type IN_LINK_CONF_CH4 = crate::Reg<in_link_conf_ch4::IN_LINK_CONF_CH4_SPEC>;
#[doc = "RX CH4 in_link dscr ctrl register"]
pub mod in_link_conf_ch4;
#[doc = "IN_LINK_ADDR_CH4 (rw) register accessor: RX CH4 in_link dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_link_addr_ch4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_link_addr_ch4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link_addr_ch4`] module"]
pub type IN_LINK_ADDR_CH4 = crate::Reg<in_link_addr_ch4::IN_LINK_ADDR_CH4_SPEC>;
#[doc = "RX CH4 in_link dscr addr register"]
pub mod in_link_addr_ch4;
#[doc = "IN_STATE_CH4 (r) register accessor: RX CH4 state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_state_ch4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_state_ch4`] module"]
pub type IN_STATE_CH4 = crate::Reg<in_state_ch4::IN_STATE_CH4_SPEC>;
#[doc = "RX CH4 state register"]
pub mod in_state_ch4;
#[doc = "IN_SUC_EOF_DES_ADDR_CH4 (r) register accessor: RX CH4 eof des addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_suc_eof_des_addr_ch4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_suc_eof_des_addr_ch4`] module"]
pub type IN_SUC_EOF_DES_ADDR_CH4 =
    crate::Reg<in_suc_eof_des_addr_ch4::IN_SUC_EOF_DES_ADDR_CH4_SPEC>;
#[doc = "RX CH4 eof des addr register"]
pub mod in_suc_eof_des_addr_ch4;
#[doc = "IN_ERR_EOF_DES_ADDR_CH4 (r) register accessor: RX CH4 err eof des addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_err_eof_des_addr_ch4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_err_eof_des_addr_ch4`] module"]
pub type IN_ERR_EOF_DES_ADDR_CH4 =
    crate::Reg<in_err_eof_des_addr_ch4::IN_ERR_EOF_DES_ADDR_CH4_SPEC>;
#[doc = "RX CH4 err eof des addr register"]
pub mod in_err_eof_des_addr_ch4;
#[doc = "IN_DSCR_CH4 (r) register accessor: RX CH4 next dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_ch4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_ch4`] module"]
pub type IN_DSCR_CH4 = crate::Reg<in_dscr_ch4::IN_DSCR_CH4_SPEC>;
#[doc = "RX CH4 next dscr addr register"]
pub mod in_dscr_ch4;
#[doc = "IN_DSCR_BF0_CH4 (r) register accessor: RX CH4 last dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_bf0_ch4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf0_ch4`] module"]
pub type IN_DSCR_BF0_CH4 = crate::Reg<in_dscr_bf0_ch4::IN_DSCR_BF0_CH4_SPEC>;
#[doc = "RX CH4 last dscr addr register"]
pub mod in_dscr_bf0_ch4;
#[doc = "IN_DSCR_BF1_CH4 (r) register accessor: RX CH4 second-to-last dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_bf1_ch4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf1_ch4`] module"]
pub type IN_DSCR_BF1_CH4 = crate::Reg<in_dscr_bf1_ch4::IN_DSCR_BF1_CH4_SPEC>;
#[doc = "RX CH4 second-to-last dscr addr register"]
pub mod in_dscr_bf1_ch4;
#[doc = "IN_ARB_CH4 (rw) register accessor: RX CH4 arb register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_arb_ch4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_arb_ch4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_arb_ch4`] module"]
pub type IN_ARB_CH4 = crate::Reg<in_arb_ch4::IN_ARB_CH4_SPEC>;
#[doc = "RX CH4 arb register"]
pub mod in_arb_ch4;
#[doc = "IN_ETM_CONF_CH4 (rw) register accessor: RX CH4 ETM config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_etm_conf_ch4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_etm_conf_ch4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_etm_conf_ch4`] module"]
pub type IN_ETM_CONF_CH4 = crate::Reg<in_etm_conf_ch4::IN_ETM_CONF_CH4_SPEC>;
#[doc = "RX CH4 ETM config register"]
pub mod in_etm_conf_ch4;
#[doc = "IN_FIFO_CNT_CH4 (r) register accessor: rx CH4 fifo cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_fifo_cnt_ch4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_fifo_cnt_ch4`] module"]
pub type IN_FIFO_CNT_CH4 = crate::Reg<in_fifo_cnt_ch4::IN_FIFO_CNT_CH4_SPEC>;
#[doc = "rx CH4 fifo cnt register"]
pub mod in_fifo_cnt_ch4;
#[doc = "IN_POP_DATA_CNT_CH4 (r) register accessor: rx CH4 pop data cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pop_data_cnt_ch4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pop_data_cnt_ch4`] module"]
pub type IN_POP_DATA_CNT_CH4 = crate::Reg<in_pop_data_cnt_ch4::IN_POP_DATA_CNT_CH4_SPEC>;
#[doc = "rx CH4 pop data cnt register"]
pub mod in_pop_data_cnt_ch4;
#[doc = "IN_XADDR_CH4 (r) register accessor: rx CH4 xaddr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_xaddr_ch4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_xaddr_ch4`] module"]
pub type IN_XADDR_CH4 = crate::Reg<in_xaddr_ch4::IN_XADDR_CH4_SPEC>;
#[doc = "rx CH4 xaddr register"]
pub mod in_xaddr_ch4;
#[doc = "IN_BUF_HB_RCV_CH4 (r) register accessor: rx CH4 buf len hb rcv register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_buf_hb_rcv_ch4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_buf_hb_rcv_ch4`] module"]
pub type IN_BUF_HB_RCV_CH4 = crate::Reg<in_buf_hb_rcv_ch4::IN_BUF_HB_RCV_CH4_SPEC>;
#[doc = "rx CH4 buf len hb rcv register"]
pub mod in_buf_hb_rcv_ch4;
#[doc = "IN_CONF0_CH5 (rw) register accessor: RX CH5 config0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf0_ch5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf0_ch5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf0_ch5`] module"]
pub type IN_CONF0_CH5 = crate::Reg<in_conf0_ch5::IN_CONF0_CH5_SPEC>;
#[doc = "RX CH5 config0 register"]
pub mod in_conf0_ch5;
#[doc = "IN_CONF1_CH5 (rw) register accessor: RX CH5 config1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf1_ch5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf1_ch5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf1_ch5`] module"]
pub type IN_CONF1_CH5 = crate::Reg<in_conf1_ch5::IN_CONF1_CH5_SPEC>;
#[doc = "RX CH5 config1 register"]
pub mod in_conf1_ch5;
#[doc = "IN_CONF2_CH5 (rw) register accessor: RX CH5 config2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf2_ch5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf2_ch5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf2_ch5`] module"]
pub type IN_CONF2_CH5 = crate::Reg<in_conf2_ch5::IN_CONF2_CH5_SPEC>;
#[doc = "RX CH5 config2 register"]
pub mod in_conf2_ch5;
#[doc = "IN_CONF3_CH5 (rw) register accessor: RX CH5 config3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf3_ch5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf3_ch5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf3_ch5`] module"]
pub type IN_CONF3_CH5 = crate::Reg<in_conf3_ch5::IN_CONF3_CH5_SPEC>;
#[doc = "RX CH5 config3 register"]
pub mod in_conf3_ch5;
#[doc = "IN_INT_RAW_CH5 (rw) register accessor: RX CH5 interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_raw_ch5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_raw_ch5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_raw_ch5`] module"]
pub type IN_INT_RAW_CH5 = crate::Reg<in_int_raw_ch5::IN_INT_RAW_CH5_SPEC>;
#[doc = "RX CH5 interrupt raw register"]
pub mod in_int_raw_ch5;
#[doc = "IN_INT_ENA_CH5 (rw) register accessor: RX CH5 interrupt ena register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_ena_ch5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_ena_ch5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_ena_ch5`] module"]
pub type IN_INT_ENA_CH5 = crate::Reg<in_int_ena_ch5::IN_INT_ENA_CH5_SPEC>;
#[doc = "RX CH5 interrupt ena register"]
pub mod in_int_ena_ch5;
#[doc = "IN_INT_ST_CH5 (r) register accessor: RX CH5 interrupt st register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_st_ch5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_st_ch5`] module"]
pub type IN_INT_ST_CH5 = crate::Reg<in_int_st_ch5::IN_INT_ST_CH5_SPEC>;
#[doc = "RX CH5 interrupt st register"]
pub mod in_int_st_ch5;
#[doc = "IN_INT_CLR_CH5 (w) register accessor: RX CH5 interrupt clr register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_clr_ch5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_clr_ch5`] module"]
pub type IN_INT_CLR_CH5 = crate::Reg<in_int_clr_ch5::IN_INT_CLR_CH5_SPEC>;
#[doc = "RX CH5 interrupt clr register"]
pub mod in_int_clr_ch5;
#[doc = "INFIFO_STATUS_CH5 (r) register accessor: RX CH5 INFIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`infifo_status_ch5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@infifo_status_ch5`] module"]
pub type INFIFO_STATUS_CH5 = crate::Reg<infifo_status_ch5::INFIFO_STATUS_CH5_SPEC>;
#[doc = "RX CH5 INFIFO status register"]
pub mod infifo_status_ch5;
#[doc = "IN_POP_CH5 (rw) register accessor: RX CH5 INFIFO pop register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pop_ch5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_pop_ch5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pop_ch5`] module"]
pub type IN_POP_CH5 = crate::Reg<in_pop_ch5::IN_POP_CH5_SPEC>;
#[doc = "RX CH5 INFIFO pop register"]
pub mod in_pop_ch5;
#[doc = "IN_STATE_CH5 (r) register accessor: RX CH5 state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_state_ch5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_state_ch5`] module"]
pub type IN_STATE_CH5 = crate::Reg<in_state_ch5::IN_STATE_CH5_SPEC>;
#[doc = "RX CH5 state register"]
pub mod in_state_ch5;
#[doc = "IN_ARB_CH5 (rw) register accessor: RX CH5 arb register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_arb_ch5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_arb_ch5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_arb_ch5`] module"]
pub type IN_ARB_CH5 = crate::Reg<in_arb_ch5::IN_ARB_CH5_SPEC>;
#[doc = "RX CH5 arb register"]
pub mod in_arb_ch5;
#[doc = "IN_FIFO_CNT_CH5 (r) register accessor: rx CH5 fifo cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_fifo_cnt_ch5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_fifo_cnt_ch5`] module"]
pub type IN_FIFO_CNT_CH5 = crate::Reg<in_fifo_cnt_ch5::IN_FIFO_CNT_CH5_SPEC>;
#[doc = "rx CH5 fifo cnt register"]
pub mod in_fifo_cnt_ch5;
#[doc = "IN_POP_DATA_CNT_CH5 (r) register accessor: rx CH5 pop data cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pop_data_cnt_ch5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pop_data_cnt_ch5`] module"]
pub type IN_POP_DATA_CNT_CH5 = crate::Reg<in_pop_data_cnt_ch5::IN_POP_DATA_CNT_CH5_SPEC>;
#[doc = "rx CH5 pop data cnt register"]
pub mod in_pop_data_cnt_ch5;
#[doc = "IN_XADDR_CH5 (r) register accessor: rx CH5 xaddr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_xaddr_ch5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_xaddr_ch5`] module"]
pub type IN_XADDR_CH5 = crate::Reg<in_xaddr_ch5::IN_XADDR_CH5_SPEC>;
#[doc = "rx CH5 xaddr register"]
pub mod in_xaddr_ch5;
#[doc = "IN_BUF_HB_RCV_CH5 (r) register accessor: rx CH5 buf len hb rcv register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_buf_hb_rcv_ch5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_buf_hb_rcv_ch5`] module"]
pub type IN_BUF_HB_RCV_CH5 = crate::Reg<in_buf_hb_rcv_ch5::IN_BUF_HB_RCV_CH5_SPEC>;
#[doc = "rx CH5 buf len hb rcv register"]
pub mod in_buf_hb_rcv_ch5;
#[doc = "INTER_AXI_ERR (r) register accessor: inter memory axi err register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inter_axi_err::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inter_axi_err`] module"]
pub type INTER_AXI_ERR = crate::Reg<inter_axi_err::INTER_AXI_ERR_SPEC>;
#[doc = "inter memory axi err register"]
pub mod inter_axi_err;
#[doc = "EXTER_AXI_ERR (r) register accessor: exter memory axi err register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exter_axi_err::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exter_axi_err`] module"]
pub type EXTER_AXI_ERR = crate::Reg<exter_axi_err::EXTER_AXI_ERR_SPEC>;
#[doc = "exter memory axi err register"]
pub mod exter_axi_err;
#[doc = "RST_CONF (rw) register accessor: axi reset config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_conf`] module"]
pub type RST_CONF = crate::Reg<rst_conf::RST_CONF_SPEC>;
#[doc = "axi reset config register"]
pub mod rst_conf;
#[doc = "INTER_MEM_START_ADDR0 (rw) register accessor: Start address of inter memory range0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inter_mem_start_addr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inter_mem_start_addr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inter_mem_start_addr0`] module"]
pub type INTER_MEM_START_ADDR0 = crate::Reg<inter_mem_start_addr0::INTER_MEM_START_ADDR0_SPEC>;
#[doc = "Start address of inter memory range0 register"]
pub mod inter_mem_start_addr0;
#[doc = "INTER_MEM_END_ADDR0 (rw) register accessor: end address of inter memory range0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inter_mem_end_addr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inter_mem_end_addr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inter_mem_end_addr0`] module"]
pub type INTER_MEM_END_ADDR0 = crate::Reg<inter_mem_end_addr0::INTER_MEM_END_ADDR0_SPEC>;
#[doc = "end address of inter memory range0 register"]
pub mod inter_mem_end_addr0;
#[doc = "INTER_MEM_START_ADDR1 (rw) register accessor: Start address of inter memory range1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inter_mem_start_addr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inter_mem_start_addr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inter_mem_start_addr1`] module"]
pub type INTER_MEM_START_ADDR1 = crate::Reg<inter_mem_start_addr1::INTER_MEM_START_ADDR1_SPEC>;
#[doc = "Start address of inter memory range1 register"]
pub mod inter_mem_start_addr1;
#[doc = "INTER_MEM_END_ADDR1 (rw) register accessor: end address of inter memory range1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inter_mem_end_addr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inter_mem_end_addr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inter_mem_end_addr1`] module"]
pub type INTER_MEM_END_ADDR1 = crate::Reg<inter_mem_end_addr1::INTER_MEM_END_ADDR1_SPEC>;
#[doc = "end address of inter memory range1 register"]
pub mod inter_mem_end_addr1;
#[doc = "EXTER_MEM_START_ADDR0 (rw) register accessor: Start address of exter memory range0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exter_mem_start_addr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exter_mem_start_addr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exter_mem_start_addr0`] module"]
pub type EXTER_MEM_START_ADDR0 = crate::Reg<exter_mem_start_addr0::EXTER_MEM_START_ADDR0_SPEC>;
#[doc = "Start address of exter memory range0 register"]
pub mod exter_mem_start_addr0;
#[doc = "EXTER_MEM_END_ADDR0 (rw) register accessor: end address of exter memory range0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exter_mem_end_addr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exter_mem_end_addr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exter_mem_end_addr0`] module"]
pub type EXTER_MEM_END_ADDR0 = crate::Reg<exter_mem_end_addr0::EXTER_MEM_END_ADDR0_SPEC>;
#[doc = "end address of exter memory range0 register"]
pub mod exter_mem_end_addr0;
#[doc = "EXTER_MEM_START_ADDR1 (rw) register accessor: Start address of exter memory range1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exter_mem_start_addr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exter_mem_start_addr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exter_mem_start_addr1`] module"]
pub type EXTER_MEM_START_ADDR1 = crate::Reg<exter_mem_start_addr1::EXTER_MEM_START_ADDR1_SPEC>;
#[doc = "Start address of exter memory range1 register"]
pub mod exter_mem_start_addr1;
#[doc = "EXTER_MEM_END_ADDR1 (rw) register accessor: end address of exter memory range1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exter_mem_end_addr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exter_mem_end_addr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exter_mem_end_addr1`] module"]
pub type EXTER_MEM_END_ADDR1 = crate::Reg<exter_mem_end_addr1::EXTER_MEM_END_ADDR1_SPEC>;
#[doc = "end address of exter memory range1 register"]
pub mod exter_mem_end_addr1;
#[doc = "OUT_ARB_CONFIG (rw) register accessor: reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_arb_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_arb_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_arb_config`] module"]
pub type OUT_ARB_CONFIG = crate::Reg<out_arb_config::OUT_ARB_CONFIG_SPEC>;
#[doc = "reserved"]
pub mod out_arb_config;
#[doc = "IN_ARB_CONFIG (rw) register accessor: reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_arb_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_arb_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_arb_config`] module"]
pub type IN_ARB_CONFIG = crate::Reg<in_arb_config::IN_ARB_CONFIG_SPEC>;
#[doc = "reserved"]
pub mod in_arb_config;
#[doc = "DATE (rw) register accessor: reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "reserved"]
pub mod date;
#[doc = "COUNTER_RST (rw) register accessor: counter reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`counter_rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`counter_rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@counter_rst`] module"]
pub type COUNTER_RST = crate::Reg<counter_rst::COUNTER_RST_SPEC>;
#[doc = "counter reset register"]
pub mod counter_rst;
#[doc = "RX_CH0_COUNTER (r) register accessor: rx ch0 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_ch0_counter::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ch0_counter`] module"]
pub type RX_CH0_COUNTER = crate::Reg<rx_ch0_counter::RX_CH0_COUNTER_SPEC>;
#[doc = "rx ch0 counter register"]
pub mod rx_ch0_counter;
#[doc = "RX_CH1_COUNTER (r) register accessor: rx ch1 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_ch1_counter::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ch1_counter`] module"]
pub type RX_CH1_COUNTER = crate::Reg<rx_ch1_counter::RX_CH1_COUNTER_SPEC>;
#[doc = "rx ch1 counter register"]
pub mod rx_ch1_counter;
#[doc = "RX_CH2_COUNTER (r) register accessor: rx ch2 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_ch2_counter::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ch2_counter`] module"]
pub type RX_CH2_COUNTER = crate::Reg<rx_ch2_counter::RX_CH2_COUNTER_SPEC>;
#[doc = "rx ch2 counter register"]
pub mod rx_ch2_counter;
#[doc = "RX_CH5_COUNTER (r) register accessor: rx ch5 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_ch5_counter::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ch5_counter`] module"]
pub type RX_CH5_COUNTER = crate::Reg<rx_ch5_counter::RX_CH5_COUNTER_SPEC>;
#[doc = "rx ch5 counter register"]
pub mod rx_ch5_counter;
