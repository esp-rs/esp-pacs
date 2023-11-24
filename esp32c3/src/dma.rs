#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    int_raw_ch0: INT_RAW_CH0,
    int_st_ch0: INT_ST_CH0,
    int_ena_ch0: INT_ENA_CH0,
    int_clr_ch0: INT_CLR_CH0,
    int_raw_ch1: INT_RAW_CH1,
    int_st_ch1: INT_ST_CH1,
    int_ena_ch1: INT_ENA_CH1,
    int_clr_ch1: INT_CLR_CH1,
    int_raw_ch2: INT_RAW_CH2,
    int_st_ch2: INT_ST_CH2,
    int_ena_ch2: INT_ENA_CH2,
    int_clr_ch2: INT_CLR_CH2,
    _reserved12: [u8; 0x10],
    ahb_test: AHB_TEST,
    misc_conf: MISC_CONF,
    date: DATE,
    _reserved15: [u8; 0x24],
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
    in_pri_ch0: IN_PRI_CH0,
    in_peri_sel_ch0: IN_PERI_SEL_CH0,
    _reserved28: [u8; 0x2c],
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
    out_pri_ch0: OUT_PRI_CH0,
    out_peri_sel_ch0: OUT_PERI_SEL_CH0,
    _reserved41: [u8; 0x2c],
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
    in_pri_ch1: IN_PRI_CH1,
    in_peri_sel_ch1: IN_PERI_SEL_CH1,
    _reserved54: [u8; 0x2c],
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
    out_pri_ch1: OUT_PRI_CH1,
    out_peri_sel_ch1: OUT_PERI_SEL_CH1,
    _reserved67: [u8; 0x2c],
    in_conf0_ch2: IN_CONF0_CH2,
    in_conf1_ch2: IN_CONF1_CH2,
    infifo_status_ch2: INFIFO_STATUS_CH2,
    in_pop_ch2: IN_POP_CH2,
    in_link_ch2: IN_LINK_CH2,
    in_state_ch2: IN_STATE_CH2,
    in_suc_eof_des_addr_ch2: IN_SUC_EOF_DES_ADDR_CH2,
    in_err_eof_des_addr_ch2: IN_ERR_EOF_DES_ADDR_CH2,
    in_dscr_ch2: IN_DSCR_CH2,
    in_dscr_bf0_ch2: IN_DSCR_BF0_CH2,
    in_dscr_bf1_ch2: IN_DSCR_BF1_CH2,
    in_pri_ch2: IN_PRI_CH2,
    in_peri_sel_ch2: IN_PERI_SEL_CH2,
    _reserved80: [u8; 0x2c],
    out_conf0_ch2: OUT_CONF0_CH2,
    out_conf1_ch2: OUT_CONF1_CH2,
    outfifo_status_ch2: OUTFIFO_STATUS_CH2,
    out_push_ch2: OUT_PUSH_CH2,
    out_link_ch2: OUT_LINK_CH2,
    out_state_ch2: OUT_STATE_CH2,
    out_eof_des_addr_ch2: OUT_EOF_DES_ADDR_CH2,
    out_eof_bfr_des_addr_ch2: OUT_EOF_BFR_DES_ADDR_CH2,
    out_dscr_ch2: OUT_DSCR_CH2,
    out_dscr_bf0_ch2: OUT_DSCR_BF0_CH2,
    out_dscr_bf1_ch2: OUT_DSCR_BF1_CH2,
    out_pri_ch2: OUT_PRI_CH2,
    out_peri_sel_ch2: OUT_PERI_SEL_CH2,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA_INT_RAW_CH0_REG."]
    #[inline(always)]
    pub const fn int_raw_ch0(&self) -> &INT_RAW_CH0 {
        &self.int_raw_ch0
    }
    #[doc = "0x04 - DMA_INT_ST_CH0_REG."]
    #[inline(always)]
    pub const fn int_st_ch0(&self) -> &INT_ST_CH0 {
        &self.int_st_ch0
    }
    #[doc = "0x08 - DMA_INT_ENA_CH0_REG."]
    #[inline(always)]
    pub const fn int_ena_ch0(&self) -> &INT_ENA_CH0 {
        &self.int_ena_ch0
    }
    #[doc = "0x0c - DMA_INT_CLR_CH0_REG."]
    #[inline(always)]
    pub const fn int_clr_ch0(&self) -> &INT_CLR_CH0 {
        &self.int_clr_ch0
    }
    #[doc = "0x10 - DMA_INT_RAW_CH1_REG."]
    #[inline(always)]
    pub const fn int_raw_ch1(&self) -> &INT_RAW_CH1 {
        &self.int_raw_ch1
    }
    #[doc = "0x14 - DMA_INT_ST_CH1_REG."]
    #[inline(always)]
    pub const fn int_st_ch1(&self) -> &INT_ST_CH1 {
        &self.int_st_ch1
    }
    #[doc = "0x18 - DMA_INT_ENA_CH1_REG."]
    #[inline(always)]
    pub const fn int_ena_ch1(&self) -> &INT_ENA_CH1 {
        &self.int_ena_ch1
    }
    #[doc = "0x1c - DMA_INT_CLR_CH1_REG."]
    #[inline(always)]
    pub const fn int_clr_ch1(&self) -> &INT_CLR_CH1 {
        &self.int_clr_ch1
    }
    #[doc = "0x20 - DMA_INT_RAW_CH2_REG."]
    #[inline(always)]
    pub const fn int_raw_ch2(&self) -> &INT_RAW_CH2 {
        &self.int_raw_ch2
    }
    #[doc = "0x24 - DMA_INT_ST_CH2_REG."]
    #[inline(always)]
    pub const fn int_st_ch2(&self) -> &INT_ST_CH2 {
        &self.int_st_ch2
    }
    #[doc = "0x28 - DMA_INT_ENA_CH2_REG."]
    #[inline(always)]
    pub const fn int_ena_ch2(&self) -> &INT_ENA_CH2 {
        &self.int_ena_ch2
    }
    #[doc = "0x2c - DMA_INT_CLR_CH2_REG."]
    #[inline(always)]
    pub const fn int_clr_ch2(&self) -> &INT_CLR_CH2 {
        &self.int_clr_ch2
    }
    #[doc = "0x40 - DMA_AHB_TEST_REG."]
    #[inline(always)]
    pub const fn ahb_test(&self) -> &AHB_TEST {
        &self.ahb_test
    }
    #[doc = "0x44 - DMA_MISC_CONF_REG."]
    #[inline(always)]
    pub const fn misc_conf(&self) -> &MISC_CONF {
        &self.misc_conf
    }
    #[doc = "0x48 - DMA_DATE_REG."]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    #[doc = "0x70 - DMA_IN_CONF0_CH0_REG."]
    #[inline(always)]
    pub const fn in_conf0_ch0(&self) -> &IN_CONF0_CH0 {
        &self.in_conf0_ch0
    }
    #[doc = "0x74 - DMA_IN_CONF1_CH0_REG."]
    #[inline(always)]
    pub const fn in_conf1_ch0(&self) -> &IN_CONF1_CH0 {
        &self.in_conf1_ch0
    }
    #[doc = "0x78 - DMA_INFIFO_STATUS_CH0_REG."]
    #[inline(always)]
    pub const fn infifo_status_ch0(&self) -> &INFIFO_STATUS_CH0 {
        &self.infifo_status_ch0
    }
    #[doc = "0x7c - DMA_IN_POP_CH0_REG."]
    #[inline(always)]
    pub const fn in_pop_ch0(&self) -> &IN_POP_CH0 {
        &self.in_pop_ch0
    }
    #[doc = "0x80 - DMA_IN_LINK_CH0_REG."]
    #[inline(always)]
    pub const fn in_link_ch0(&self) -> &IN_LINK_CH0 {
        &self.in_link_ch0
    }
    #[doc = "0x84 - DMA_IN_STATE_CH0_REG."]
    #[inline(always)]
    pub const fn in_state_ch0(&self) -> &IN_STATE_CH0 {
        &self.in_state_ch0
    }
    #[doc = "0x88 - DMA_IN_SUC_EOF_DES_ADDR_CH0_REG."]
    #[inline(always)]
    pub const fn in_suc_eof_des_addr_ch0(&self) -> &IN_SUC_EOF_DES_ADDR_CH0 {
        &self.in_suc_eof_des_addr_ch0
    }
    #[doc = "0x8c - DMA_IN_ERR_EOF_DES_ADDR_CH0_REG."]
    #[inline(always)]
    pub const fn in_err_eof_des_addr_ch0(&self) -> &IN_ERR_EOF_DES_ADDR_CH0 {
        &self.in_err_eof_des_addr_ch0
    }
    #[doc = "0x90 - DMA_IN_DSCR_CH0_REG."]
    #[inline(always)]
    pub const fn in_dscr_ch0(&self) -> &IN_DSCR_CH0 {
        &self.in_dscr_ch0
    }
    #[doc = "0x94 - DMA_IN_DSCR_BF0_CH0_REG."]
    #[inline(always)]
    pub const fn in_dscr_bf0_ch0(&self) -> &IN_DSCR_BF0_CH0 {
        &self.in_dscr_bf0_ch0
    }
    #[doc = "0x98 - DMA_IN_DSCR_BF1_CH0_REG."]
    #[inline(always)]
    pub const fn in_dscr_bf1_ch0(&self) -> &IN_DSCR_BF1_CH0 {
        &self.in_dscr_bf1_ch0
    }
    #[doc = "0x9c - DMA_IN_PRI_CH0_REG."]
    #[inline(always)]
    pub const fn in_pri_ch0(&self) -> &IN_PRI_CH0 {
        &self.in_pri_ch0
    }
    #[doc = "0xa0 - DMA_IN_PERI_SEL_CH0_REG."]
    #[inline(always)]
    pub const fn in_peri_sel_ch0(&self) -> &IN_PERI_SEL_CH0 {
        &self.in_peri_sel_ch0
    }
    #[doc = "0xd0 - DMA_OUT_CONF0_CH0_REG."]
    #[inline(always)]
    pub const fn out_conf0_ch0(&self) -> &OUT_CONF0_CH0 {
        &self.out_conf0_ch0
    }
    #[doc = "0xd4 - DMA_OUT_CONF1_CH0_REG."]
    #[inline(always)]
    pub const fn out_conf1_ch0(&self) -> &OUT_CONF1_CH0 {
        &self.out_conf1_ch0
    }
    #[doc = "0xd8 - DMA_OUTFIFO_STATUS_CH0_REG."]
    #[inline(always)]
    pub const fn outfifo_status_ch0(&self) -> &OUTFIFO_STATUS_CH0 {
        &self.outfifo_status_ch0
    }
    #[doc = "0xdc - DMA_OUT_PUSH_CH0_REG."]
    #[inline(always)]
    pub const fn out_push_ch0(&self) -> &OUT_PUSH_CH0 {
        &self.out_push_ch0
    }
    #[doc = "0xe0 - DMA_OUT_LINK_CH0_REG."]
    #[inline(always)]
    pub const fn out_link_ch0(&self) -> &OUT_LINK_CH0 {
        &self.out_link_ch0
    }
    #[doc = "0xe4 - DMA_OUT_STATE_CH0_REG."]
    #[inline(always)]
    pub const fn out_state_ch0(&self) -> &OUT_STATE_CH0 {
        &self.out_state_ch0
    }
    #[doc = "0xe8 - DMA_OUT_EOF_DES_ADDR_CH0_REG."]
    #[inline(always)]
    pub const fn out_eof_des_addr_ch0(&self) -> &OUT_EOF_DES_ADDR_CH0 {
        &self.out_eof_des_addr_ch0
    }
    #[doc = "0xec - DMA_OUT_EOF_BFR_DES_ADDR_CH0_REG."]
    #[inline(always)]
    pub const fn out_eof_bfr_des_addr_ch0(&self) -> &OUT_EOF_BFR_DES_ADDR_CH0 {
        &self.out_eof_bfr_des_addr_ch0
    }
    #[doc = "0xf0 - DMA_OUT_DSCR_CH0_REG."]
    #[inline(always)]
    pub const fn out_dscr_ch0(&self) -> &OUT_DSCR_CH0 {
        &self.out_dscr_ch0
    }
    #[doc = "0xf4 - DMA_OUT_DSCR_BF0_CH0_REG."]
    #[inline(always)]
    pub const fn out_dscr_bf0_ch0(&self) -> &OUT_DSCR_BF0_CH0 {
        &self.out_dscr_bf0_ch0
    }
    #[doc = "0xf8 - DMA_OUT_DSCR_BF1_CH0_REG."]
    #[inline(always)]
    pub const fn out_dscr_bf1_ch0(&self) -> &OUT_DSCR_BF1_CH0 {
        &self.out_dscr_bf1_ch0
    }
    #[doc = "0xfc - DMA_OUT_PRI_CH0_REG."]
    #[inline(always)]
    pub const fn out_pri_ch0(&self) -> &OUT_PRI_CH0 {
        &self.out_pri_ch0
    }
    #[doc = "0x100 - DMA_OUT_PERI_SEL_CH0_REG."]
    #[inline(always)]
    pub const fn out_peri_sel_ch0(&self) -> &OUT_PERI_SEL_CH0 {
        &self.out_peri_sel_ch0
    }
    #[doc = "0x130 - DMA_IN_CONF0_CH1_REG."]
    #[inline(always)]
    pub const fn in_conf0_ch1(&self) -> &IN_CONF0_CH1 {
        &self.in_conf0_ch1
    }
    #[doc = "0x134 - DMA_IN_CONF1_CH1_REG."]
    #[inline(always)]
    pub const fn in_conf1_ch1(&self) -> &IN_CONF1_CH1 {
        &self.in_conf1_ch1
    }
    #[doc = "0x138 - DMA_INFIFO_STATUS_CH1_REG."]
    #[inline(always)]
    pub const fn infifo_status_ch1(&self) -> &INFIFO_STATUS_CH1 {
        &self.infifo_status_ch1
    }
    #[doc = "0x13c - DMA_IN_POP_CH1_REG."]
    #[inline(always)]
    pub const fn in_pop_ch1(&self) -> &IN_POP_CH1 {
        &self.in_pop_ch1
    }
    #[doc = "0x140 - DMA_IN_LINK_CH1_REG."]
    #[inline(always)]
    pub const fn in_link_ch1(&self) -> &IN_LINK_CH1 {
        &self.in_link_ch1
    }
    #[doc = "0x144 - DMA_IN_STATE_CH1_REG."]
    #[inline(always)]
    pub const fn in_state_ch1(&self) -> &IN_STATE_CH1 {
        &self.in_state_ch1
    }
    #[doc = "0x148 - DMA_IN_SUC_EOF_DES_ADDR_CH1_REG."]
    #[inline(always)]
    pub const fn in_suc_eof_des_addr_ch1(&self) -> &IN_SUC_EOF_DES_ADDR_CH1 {
        &self.in_suc_eof_des_addr_ch1
    }
    #[doc = "0x14c - DMA_IN_ERR_EOF_DES_ADDR_CH1_REG."]
    #[inline(always)]
    pub const fn in_err_eof_des_addr_ch1(&self) -> &IN_ERR_EOF_DES_ADDR_CH1 {
        &self.in_err_eof_des_addr_ch1
    }
    #[doc = "0x150 - DMA_IN_DSCR_CH1_REG."]
    #[inline(always)]
    pub const fn in_dscr_ch1(&self) -> &IN_DSCR_CH1 {
        &self.in_dscr_ch1
    }
    #[doc = "0x154 - DMA_IN_DSCR_BF0_CH1_REG."]
    #[inline(always)]
    pub const fn in_dscr_bf0_ch1(&self) -> &IN_DSCR_BF0_CH1 {
        &self.in_dscr_bf0_ch1
    }
    #[doc = "0x158 - DMA_IN_DSCR_BF1_CH1_REG."]
    #[inline(always)]
    pub const fn in_dscr_bf1_ch1(&self) -> &IN_DSCR_BF1_CH1 {
        &self.in_dscr_bf1_ch1
    }
    #[doc = "0x15c - DMA_IN_PRI_CH1_REG."]
    #[inline(always)]
    pub const fn in_pri_ch1(&self) -> &IN_PRI_CH1 {
        &self.in_pri_ch1
    }
    #[doc = "0x160 - DMA_IN_PERI_SEL_CH1_REG."]
    #[inline(always)]
    pub const fn in_peri_sel_ch1(&self) -> &IN_PERI_SEL_CH1 {
        &self.in_peri_sel_ch1
    }
    #[doc = "0x190 - DMA_OUT_CONF0_CH1_REG."]
    #[inline(always)]
    pub const fn out_conf0_ch1(&self) -> &OUT_CONF0_CH1 {
        &self.out_conf0_ch1
    }
    #[doc = "0x194 - DMA_OUT_CONF1_CH1_REG."]
    #[inline(always)]
    pub const fn out_conf1_ch1(&self) -> &OUT_CONF1_CH1 {
        &self.out_conf1_ch1
    }
    #[doc = "0x198 - DMA_OUTFIFO_STATUS_CH1_REG."]
    #[inline(always)]
    pub const fn outfifo_status_ch1(&self) -> &OUTFIFO_STATUS_CH1 {
        &self.outfifo_status_ch1
    }
    #[doc = "0x19c - DMA_OUT_PUSH_CH1_REG."]
    #[inline(always)]
    pub const fn out_push_ch1(&self) -> &OUT_PUSH_CH1 {
        &self.out_push_ch1
    }
    #[doc = "0x1a0 - DMA_OUT_LINK_CH1_REG."]
    #[inline(always)]
    pub const fn out_link_ch1(&self) -> &OUT_LINK_CH1 {
        &self.out_link_ch1
    }
    #[doc = "0x1a4 - DMA_OUT_STATE_CH1_REG."]
    #[inline(always)]
    pub const fn out_state_ch1(&self) -> &OUT_STATE_CH1 {
        &self.out_state_ch1
    }
    #[doc = "0x1a8 - DMA_OUT_EOF_DES_ADDR_CH1_REG."]
    #[inline(always)]
    pub const fn out_eof_des_addr_ch1(&self) -> &OUT_EOF_DES_ADDR_CH1 {
        &self.out_eof_des_addr_ch1
    }
    #[doc = "0x1ac - DMA_OUT_EOF_BFR_DES_ADDR_CH1_REG."]
    #[inline(always)]
    pub const fn out_eof_bfr_des_addr_ch1(&self) -> &OUT_EOF_BFR_DES_ADDR_CH1 {
        &self.out_eof_bfr_des_addr_ch1
    }
    #[doc = "0x1b0 - DMA_OUT_DSCR_CH1_REG."]
    #[inline(always)]
    pub const fn out_dscr_ch1(&self) -> &OUT_DSCR_CH1 {
        &self.out_dscr_ch1
    }
    #[doc = "0x1b4 - DMA_OUT_DSCR_BF0_CH1_REG."]
    #[inline(always)]
    pub const fn out_dscr_bf0_ch1(&self) -> &OUT_DSCR_BF0_CH1 {
        &self.out_dscr_bf0_ch1
    }
    #[doc = "0x1b8 - DMA_OUT_DSCR_BF1_CH1_REG."]
    #[inline(always)]
    pub const fn out_dscr_bf1_ch1(&self) -> &OUT_DSCR_BF1_CH1 {
        &self.out_dscr_bf1_ch1
    }
    #[doc = "0x1bc - DMA_OUT_PRI_CH1_REG."]
    #[inline(always)]
    pub const fn out_pri_ch1(&self) -> &OUT_PRI_CH1 {
        &self.out_pri_ch1
    }
    #[doc = "0x1c0 - DMA_OUT_PERI_SEL_CH1_REG."]
    #[inline(always)]
    pub const fn out_peri_sel_ch1(&self) -> &OUT_PERI_SEL_CH1 {
        &self.out_peri_sel_ch1
    }
    #[doc = "0x1f0 - DMA_IN_CONF0_CH2_REG."]
    #[inline(always)]
    pub const fn in_conf0_ch2(&self) -> &IN_CONF0_CH2 {
        &self.in_conf0_ch2
    }
    #[doc = "0x1f4 - DMA_IN_CONF1_CH2_REG."]
    #[inline(always)]
    pub const fn in_conf1_ch2(&self) -> &IN_CONF1_CH2 {
        &self.in_conf1_ch2
    }
    #[doc = "0x1f8 - DMA_INFIFO_STATUS_CH2_REG."]
    #[inline(always)]
    pub const fn infifo_status_ch2(&self) -> &INFIFO_STATUS_CH2 {
        &self.infifo_status_ch2
    }
    #[doc = "0x1fc - DMA_IN_POP_CH2_REG."]
    #[inline(always)]
    pub const fn in_pop_ch2(&self) -> &IN_POP_CH2 {
        &self.in_pop_ch2
    }
    #[doc = "0x200 - DMA_IN_LINK_CH2_REG."]
    #[inline(always)]
    pub const fn in_link_ch2(&self) -> &IN_LINK_CH2 {
        &self.in_link_ch2
    }
    #[doc = "0x204 - DMA_IN_STATE_CH2_REG."]
    #[inline(always)]
    pub const fn in_state_ch2(&self) -> &IN_STATE_CH2 {
        &self.in_state_ch2
    }
    #[doc = "0x208 - DMA_IN_SUC_EOF_DES_ADDR_CH2_REG."]
    #[inline(always)]
    pub const fn in_suc_eof_des_addr_ch2(&self) -> &IN_SUC_EOF_DES_ADDR_CH2 {
        &self.in_suc_eof_des_addr_ch2
    }
    #[doc = "0x20c - DMA_IN_ERR_EOF_DES_ADDR_CH2_REG."]
    #[inline(always)]
    pub const fn in_err_eof_des_addr_ch2(&self) -> &IN_ERR_EOF_DES_ADDR_CH2 {
        &self.in_err_eof_des_addr_ch2
    }
    #[doc = "0x210 - DMA_IN_DSCR_CH2_REG."]
    #[inline(always)]
    pub const fn in_dscr_ch2(&self) -> &IN_DSCR_CH2 {
        &self.in_dscr_ch2
    }
    #[doc = "0x214 - DMA_IN_DSCR_BF0_CH2_REG."]
    #[inline(always)]
    pub const fn in_dscr_bf0_ch2(&self) -> &IN_DSCR_BF0_CH2 {
        &self.in_dscr_bf0_ch2
    }
    #[doc = "0x218 - DMA_IN_DSCR_BF1_CH2_REG."]
    #[inline(always)]
    pub const fn in_dscr_bf1_ch2(&self) -> &IN_DSCR_BF1_CH2 {
        &self.in_dscr_bf1_ch2
    }
    #[doc = "0x21c - DMA_IN_PRI_CH2_REG."]
    #[inline(always)]
    pub const fn in_pri_ch2(&self) -> &IN_PRI_CH2 {
        &self.in_pri_ch2
    }
    #[doc = "0x220 - DMA_IN_PERI_SEL_CH2_REG."]
    #[inline(always)]
    pub const fn in_peri_sel_ch2(&self) -> &IN_PERI_SEL_CH2 {
        &self.in_peri_sel_ch2
    }
    #[doc = "0x250 - DMA_OUT_CONF0_CH2_REG."]
    #[inline(always)]
    pub const fn out_conf0_ch2(&self) -> &OUT_CONF0_CH2 {
        &self.out_conf0_ch2
    }
    #[doc = "0x254 - DMA_OUT_CONF1_CH2_REG."]
    #[inline(always)]
    pub const fn out_conf1_ch2(&self) -> &OUT_CONF1_CH2 {
        &self.out_conf1_ch2
    }
    #[doc = "0x258 - DMA_OUTFIFO_STATUS_CH2_REG."]
    #[inline(always)]
    pub const fn outfifo_status_ch2(&self) -> &OUTFIFO_STATUS_CH2 {
        &self.outfifo_status_ch2
    }
    #[doc = "0x25c - DMA_OUT_PUSH_CH2_REG."]
    #[inline(always)]
    pub const fn out_push_ch2(&self) -> &OUT_PUSH_CH2 {
        &self.out_push_ch2
    }
    #[doc = "0x260 - DMA_OUT_LINK_CH2_REG."]
    #[inline(always)]
    pub const fn out_link_ch2(&self) -> &OUT_LINK_CH2 {
        &self.out_link_ch2
    }
    #[doc = "0x264 - DMA_OUT_STATE_CH2_REG."]
    #[inline(always)]
    pub const fn out_state_ch2(&self) -> &OUT_STATE_CH2 {
        &self.out_state_ch2
    }
    #[doc = "0x268 - DMA_OUT_EOF_DES_ADDR_CH2_REG."]
    #[inline(always)]
    pub const fn out_eof_des_addr_ch2(&self) -> &OUT_EOF_DES_ADDR_CH2 {
        &self.out_eof_des_addr_ch2
    }
    #[doc = "0x26c - DMA_OUT_EOF_BFR_DES_ADDR_CH2_REG."]
    #[inline(always)]
    pub const fn out_eof_bfr_des_addr_ch2(&self) -> &OUT_EOF_BFR_DES_ADDR_CH2 {
        &self.out_eof_bfr_des_addr_ch2
    }
    #[doc = "0x270 - DMA_OUT_DSCR_CH2_REG."]
    #[inline(always)]
    pub const fn out_dscr_ch2(&self) -> &OUT_DSCR_CH2 {
        &self.out_dscr_ch2
    }
    #[doc = "0x274 - DMA_OUT_DSCR_BF0_CH2_REG."]
    #[inline(always)]
    pub const fn out_dscr_bf0_ch2(&self) -> &OUT_DSCR_BF0_CH2 {
        &self.out_dscr_bf0_ch2
    }
    #[doc = "0x278 - DMA_OUT_DSCR_BF1_CH2_REG."]
    #[inline(always)]
    pub const fn out_dscr_bf1_ch2(&self) -> &OUT_DSCR_BF1_CH2 {
        &self.out_dscr_bf1_ch2
    }
    #[doc = "0x27c - DMA_OUT_PRI_CH2_REG."]
    #[inline(always)]
    pub const fn out_pri_ch2(&self) -> &OUT_PRI_CH2 {
        &self.out_pri_ch2
    }
    #[doc = "0x280 - DMA_OUT_PERI_SEL_CH2_REG."]
    #[inline(always)]
    pub const fn out_peri_sel_ch2(&self) -> &OUT_PERI_SEL_CH2 {
        &self.out_peri_sel_ch2
    }
}
#[doc = "INT_RAW_CH0 (rw) register accessor: DMA_INT_RAW_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw_ch0`] module"]
pub type INT_RAW_CH0 = crate::Reg<int_raw_ch0::INT_RAW_CH0_SPEC>;
#[doc = "DMA_INT_RAW_CH0_REG."]
pub mod int_raw_ch0;
#[doc = "INT_ST_CH0 (r) register accessor: DMA_INT_ST_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_ch0`] module"]
pub type INT_ST_CH0 = crate::Reg<int_st_ch0::INT_ST_CH0_SPEC>;
#[doc = "DMA_INT_ST_CH0_REG."]
pub mod int_st_ch0;
#[doc = "INT_ENA_CH0 (rw) register accessor: DMA_INT_ENA_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena_ch0`] module"]
pub type INT_ENA_CH0 = crate::Reg<int_ena_ch0::INT_ENA_CH0_SPEC>;
#[doc = "DMA_INT_ENA_CH0_REG."]
pub mod int_ena_ch0;
#[doc = "INT_CLR_CH0 (w) register accessor: DMA_INT_CLR_CH0_REG.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr_ch0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr_ch0`] module"]
pub type INT_CLR_CH0 = crate::Reg<int_clr_ch0::INT_CLR_CH0_SPEC>;
#[doc = "DMA_INT_CLR_CH0_REG."]
pub mod int_clr_ch0;
#[doc = "INT_RAW_CH1 (rw) register accessor: DMA_INT_RAW_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw_ch1`] module"]
pub type INT_RAW_CH1 = crate::Reg<int_raw_ch1::INT_RAW_CH1_SPEC>;
#[doc = "DMA_INT_RAW_CH1_REG."]
pub mod int_raw_ch1;
#[doc = "INT_ST_CH1 (r) register accessor: DMA_INT_ST_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_ch1`] module"]
pub type INT_ST_CH1 = crate::Reg<int_st_ch1::INT_ST_CH1_SPEC>;
#[doc = "DMA_INT_ST_CH1_REG."]
pub mod int_st_ch1;
#[doc = "INT_ENA_CH1 (rw) register accessor: DMA_INT_ENA_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena_ch1`] module"]
pub type INT_ENA_CH1 = crate::Reg<int_ena_ch1::INT_ENA_CH1_SPEC>;
#[doc = "DMA_INT_ENA_CH1_REG."]
pub mod int_ena_ch1;
#[doc = "INT_CLR_CH1 (w) register accessor: DMA_INT_CLR_CH1_REG.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr_ch1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr_ch1`] module"]
pub type INT_CLR_CH1 = crate::Reg<int_clr_ch1::INT_CLR_CH1_SPEC>;
#[doc = "DMA_INT_CLR_CH1_REG."]
pub mod int_clr_ch1;
#[doc = "INT_RAW_CH2 (rw) register accessor: DMA_INT_RAW_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw_ch2`] module"]
pub type INT_RAW_CH2 = crate::Reg<int_raw_ch2::INT_RAW_CH2_SPEC>;
#[doc = "DMA_INT_RAW_CH2_REG."]
pub mod int_raw_ch2;
#[doc = "INT_ST_CH2 (r) register accessor: DMA_INT_ST_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_ch2`] module"]
pub type INT_ST_CH2 = crate::Reg<int_st_ch2::INT_ST_CH2_SPEC>;
#[doc = "DMA_INT_ST_CH2_REG."]
pub mod int_st_ch2;
#[doc = "INT_ENA_CH2 (rw) register accessor: DMA_INT_ENA_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena_ch2`] module"]
pub type INT_ENA_CH2 = crate::Reg<int_ena_ch2::INT_ENA_CH2_SPEC>;
#[doc = "DMA_INT_ENA_CH2_REG."]
pub mod int_ena_ch2;
#[doc = "INT_CLR_CH2 (w) register accessor: DMA_INT_CLR_CH2_REG.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr_ch2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr_ch2`] module"]
pub type INT_CLR_CH2 = crate::Reg<int_clr_ch2::INT_CLR_CH2_SPEC>;
#[doc = "DMA_INT_CLR_CH2_REG."]
pub mod int_clr_ch2;
#[doc = "AHB_TEST (rw) register accessor: DMA_AHB_TEST_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_test`] module"]
pub type AHB_TEST = crate::Reg<ahb_test::AHB_TEST_SPEC>;
#[doc = "DMA_AHB_TEST_REG."]
pub mod ahb_test;
#[doc = "MISC_CONF (rw) register accessor: DMA_MISC_CONF_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_conf`] module"]
pub type MISC_CONF = crate::Reg<misc_conf::MISC_CONF_SPEC>;
#[doc = "DMA_MISC_CONF_REG."]
pub mod misc_conf;
#[doc = "DATE (rw) register accessor: DMA_DATE_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "DMA_DATE_REG."]
pub mod date;
#[doc = "IN_CONF0_CH0 (rw) register accessor: DMA_IN_CONF0_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf0_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf0_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf0_ch0`] module"]
pub type IN_CONF0_CH0 = crate::Reg<in_conf0_ch0::IN_CONF0_CH0_SPEC>;
#[doc = "DMA_IN_CONF0_CH0_REG."]
pub mod in_conf0_ch0;
#[doc = "IN_CONF1_CH0 (rw) register accessor: DMA_IN_CONF1_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf1_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf1_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf1_ch0`] module"]
pub type IN_CONF1_CH0 = crate::Reg<in_conf1_ch0::IN_CONF1_CH0_SPEC>;
#[doc = "DMA_IN_CONF1_CH0_REG."]
pub mod in_conf1_ch0;
#[doc = "INFIFO_STATUS_CH0 (r) register accessor: DMA_INFIFO_STATUS_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`infifo_status_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@infifo_status_ch0`] module"]
pub type INFIFO_STATUS_CH0 = crate::Reg<infifo_status_ch0::INFIFO_STATUS_CH0_SPEC>;
#[doc = "DMA_INFIFO_STATUS_CH0_REG."]
pub mod infifo_status_ch0;
#[doc = "IN_POP_CH0 (rw) register accessor: DMA_IN_POP_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pop_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_pop_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pop_ch0`] module"]
pub type IN_POP_CH0 = crate::Reg<in_pop_ch0::IN_POP_CH0_SPEC>;
#[doc = "DMA_IN_POP_CH0_REG."]
pub mod in_pop_ch0;
#[doc = "IN_LINK_CH0 (rw) register accessor: DMA_IN_LINK_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_link_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_link_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link_ch0`] module"]
pub type IN_LINK_CH0 = crate::Reg<in_link_ch0::IN_LINK_CH0_SPEC>;
#[doc = "DMA_IN_LINK_CH0_REG."]
pub mod in_link_ch0;
#[doc = "IN_STATE_CH0 (r) register accessor: DMA_IN_STATE_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_state_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_state_ch0`] module"]
pub type IN_STATE_CH0 = crate::Reg<in_state_ch0::IN_STATE_CH0_SPEC>;
#[doc = "DMA_IN_STATE_CH0_REG."]
pub mod in_state_ch0;
#[doc = "IN_SUC_EOF_DES_ADDR_CH0 (r) register accessor: DMA_IN_SUC_EOF_DES_ADDR_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_suc_eof_des_addr_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_suc_eof_des_addr_ch0`] module"]
pub type IN_SUC_EOF_DES_ADDR_CH0 =
    crate::Reg<in_suc_eof_des_addr_ch0::IN_SUC_EOF_DES_ADDR_CH0_SPEC>;
#[doc = "DMA_IN_SUC_EOF_DES_ADDR_CH0_REG."]
pub mod in_suc_eof_des_addr_ch0;
#[doc = "IN_ERR_EOF_DES_ADDR_CH0 (r) register accessor: DMA_IN_ERR_EOF_DES_ADDR_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_err_eof_des_addr_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_err_eof_des_addr_ch0`] module"]
pub type IN_ERR_EOF_DES_ADDR_CH0 =
    crate::Reg<in_err_eof_des_addr_ch0::IN_ERR_EOF_DES_ADDR_CH0_SPEC>;
#[doc = "DMA_IN_ERR_EOF_DES_ADDR_CH0_REG."]
pub mod in_err_eof_des_addr_ch0;
#[doc = "IN_DSCR_CH0 (r) register accessor: DMA_IN_DSCR_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_ch0`] module"]
pub type IN_DSCR_CH0 = crate::Reg<in_dscr_ch0::IN_DSCR_CH0_SPEC>;
#[doc = "DMA_IN_DSCR_CH0_REG."]
pub mod in_dscr_ch0;
#[doc = "IN_DSCR_BF0_CH0 (r) register accessor: DMA_IN_DSCR_BF0_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_bf0_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf0_ch0`] module"]
pub type IN_DSCR_BF0_CH0 = crate::Reg<in_dscr_bf0_ch0::IN_DSCR_BF0_CH0_SPEC>;
#[doc = "DMA_IN_DSCR_BF0_CH0_REG."]
pub mod in_dscr_bf0_ch0;
#[doc = "IN_DSCR_BF1_CH0 (r) register accessor: DMA_IN_DSCR_BF1_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_bf1_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf1_ch0`] module"]
pub type IN_DSCR_BF1_CH0 = crate::Reg<in_dscr_bf1_ch0::IN_DSCR_BF1_CH0_SPEC>;
#[doc = "DMA_IN_DSCR_BF1_CH0_REG."]
pub mod in_dscr_bf1_ch0;
#[doc = "IN_PRI_CH0 (rw) register accessor: DMA_IN_PRI_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pri_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_pri_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pri_ch0`] module"]
pub type IN_PRI_CH0 = crate::Reg<in_pri_ch0::IN_PRI_CH0_SPEC>;
#[doc = "DMA_IN_PRI_CH0_REG."]
pub mod in_pri_ch0;
#[doc = "IN_PERI_SEL_CH0 (rw) register accessor: DMA_IN_PERI_SEL_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_peri_sel_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_peri_sel_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_peri_sel_ch0`] module"]
pub type IN_PERI_SEL_CH0 = crate::Reg<in_peri_sel_ch0::IN_PERI_SEL_CH0_SPEC>;
#[doc = "DMA_IN_PERI_SEL_CH0_REG."]
pub mod in_peri_sel_ch0;
#[doc = "OUT_CONF0_CH0 (rw) register accessor: DMA_OUT_CONF0_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf0_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf0_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf0_ch0`] module"]
pub type OUT_CONF0_CH0 = crate::Reg<out_conf0_ch0::OUT_CONF0_CH0_SPEC>;
#[doc = "DMA_OUT_CONF0_CH0_REG."]
pub mod out_conf0_ch0;
#[doc = "OUT_CONF1_CH0 (rw) register accessor: DMA_OUT_CONF1_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf1_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf1_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf1_ch0`] module"]
pub type OUT_CONF1_CH0 = crate::Reg<out_conf1_ch0::OUT_CONF1_CH0_SPEC>;
#[doc = "DMA_OUT_CONF1_CH0_REG."]
pub mod out_conf1_ch0;
#[doc = "OUTFIFO_STATUS_CH0 (r) register accessor: DMA_OUTFIFO_STATUS_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outfifo_status_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outfifo_status_ch0`] module"]
pub type OUTFIFO_STATUS_CH0 = crate::Reg<outfifo_status_ch0::OUTFIFO_STATUS_CH0_SPEC>;
#[doc = "DMA_OUTFIFO_STATUS_CH0_REG."]
pub mod outfifo_status_ch0;
#[doc = "OUT_PUSH_CH0 (rw) register accessor: DMA_OUT_PUSH_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_push_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_push_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_push_ch0`] module"]
pub type OUT_PUSH_CH0 = crate::Reg<out_push_ch0::OUT_PUSH_CH0_SPEC>;
#[doc = "DMA_OUT_PUSH_CH0_REG."]
pub mod out_push_ch0;
#[doc = "OUT_LINK_CH0 (rw) register accessor: DMA_OUT_LINK_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_link_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link_ch0`] module"]
pub type OUT_LINK_CH0 = crate::Reg<out_link_ch0::OUT_LINK_CH0_SPEC>;
#[doc = "DMA_OUT_LINK_CH0_REG."]
pub mod out_link_ch0;
#[doc = "OUT_STATE_CH0 (r) register accessor: DMA_OUT_STATE_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_state_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_state_ch0`] module"]
pub type OUT_STATE_CH0 = crate::Reg<out_state_ch0::OUT_STATE_CH0_SPEC>;
#[doc = "DMA_OUT_STATE_CH0_REG."]
pub mod out_state_ch0;
#[doc = "OUT_EOF_DES_ADDR_CH0 (r) register accessor: DMA_OUT_EOF_DES_ADDR_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_eof_des_addr_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_des_addr_ch0`] module"]
pub type OUT_EOF_DES_ADDR_CH0 = crate::Reg<out_eof_des_addr_ch0::OUT_EOF_DES_ADDR_CH0_SPEC>;
#[doc = "DMA_OUT_EOF_DES_ADDR_CH0_REG."]
pub mod out_eof_des_addr_ch0;
#[doc = "OUT_EOF_BFR_DES_ADDR_CH0 (r) register accessor: DMA_OUT_EOF_BFR_DES_ADDR_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_eof_bfr_des_addr_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_bfr_des_addr_ch0`] module"]
pub type OUT_EOF_BFR_DES_ADDR_CH0 =
    crate::Reg<out_eof_bfr_des_addr_ch0::OUT_EOF_BFR_DES_ADDR_CH0_SPEC>;
#[doc = "DMA_OUT_EOF_BFR_DES_ADDR_CH0_REG."]
pub mod out_eof_bfr_des_addr_ch0;
#[doc = "OUT_DSCR_CH0 (r) register accessor: DMA_OUT_DSCR_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_ch0`] module"]
pub type OUT_DSCR_CH0 = crate::Reg<out_dscr_ch0::OUT_DSCR_CH0_SPEC>;
#[doc = "DMA_OUT_DSCR_CH0_REG."]
pub mod out_dscr_ch0;
#[doc = "OUT_DSCR_BF0_CH0 (r) register accessor: DMA_OUT_DSCR_BF0_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_bf0_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf0_ch0`] module"]
pub type OUT_DSCR_BF0_CH0 = crate::Reg<out_dscr_bf0_ch0::OUT_DSCR_BF0_CH0_SPEC>;
#[doc = "DMA_OUT_DSCR_BF0_CH0_REG."]
pub mod out_dscr_bf0_ch0;
#[doc = "OUT_DSCR_BF1_CH0 (r) register accessor: DMA_OUT_DSCR_BF1_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_bf1_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf1_ch0`] module"]
pub type OUT_DSCR_BF1_CH0 = crate::Reg<out_dscr_bf1_ch0::OUT_DSCR_BF1_CH0_SPEC>;
#[doc = "DMA_OUT_DSCR_BF1_CH0_REG."]
pub mod out_dscr_bf1_ch0;
#[doc = "OUT_PRI_CH0 (rw) register accessor: DMA_OUT_PRI_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_pri_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_pri_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_pri_ch0`] module"]
pub type OUT_PRI_CH0 = crate::Reg<out_pri_ch0::OUT_PRI_CH0_SPEC>;
#[doc = "DMA_OUT_PRI_CH0_REG."]
pub mod out_pri_ch0;
#[doc = "OUT_PERI_SEL_CH0 (rw) register accessor: DMA_OUT_PERI_SEL_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_peri_sel_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_peri_sel_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_peri_sel_ch0`] module"]
pub type OUT_PERI_SEL_CH0 = crate::Reg<out_peri_sel_ch0::OUT_PERI_SEL_CH0_SPEC>;
#[doc = "DMA_OUT_PERI_SEL_CH0_REG."]
pub mod out_peri_sel_ch0;
#[doc = "IN_CONF0_CH1 (rw) register accessor: DMA_IN_CONF0_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf0_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf0_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf0_ch1`] module"]
pub type IN_CONF0_CH1 = crate::Reg<in_conf0_ch1::IN_CONF0_CH1_SPEC>;
#[doc = "DMA_IN_CONF0_CH1_REG."]
pub mod in_conf0_ch1;
#[doc = "IN_CONF1_CH1 (rw) register accessor: DMA_IN_CONF1_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf1_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf1_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf1_ch1`] module"]
pub type IN_CONF1_CH1 = crate::Reg<in_conf1_ch1::IN_CONF1_CH1_SPEC>;
#[doc = "DMA_IN_CONF1_CH1_REG."]
pub mod in_conf1_ch1;
#[doc = "INFIFO_STATUS_CH1 (r) register accessor: DMA_INFIFO_STATUS_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`infifo_status_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@infifo_status_ch1`] module"]
pub type INFIFO_STATUS_CH1 = crate::Reg<infifo_status_ch1::INFIFO_STATUS_CH1_SPEC>;
#[doc = "DMA_INFIFO_STATUS_CH1_REG."]
pub mod infifo_status_ch1;
#[doc = "IN_POP_CH1 (rw) register accessor: DMA_IN_POP_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pop_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_pop_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pop_ch1`] module"]
pub type IN_POP_CH1 = crate::Reg<in_pop_ch1::IN_POP_CH1_SPEC>;
#[doc = "DMA_IN_POP_CH1_REG."]
pub mod in_pop_ch1;
#[doc = "IN_LINK_CH1 (rw) register accessor: DMA_IN_LINK_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_link_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_link_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link_ch1`] module"]
pub type IN_LINK_CH1 = crate::Reg<in_link_ch1::IN_LINK_CH1_SPEC>;
#[doc = "DMA_IN_LINK_CH1_REG."]
pub mod in_link_ch1;
#[doc = "IN_STATE_CH1 (r) register accessor: DMA_IN_STATE_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_state_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_state_ch1`] module"]
pub type IN_STATE_CH1 = crate::Reg<in_state_ch1::IN_STATE_CH1_SPEC>;
#[doc = "DMA_IN_STATE_CH1_REG."]
pub mod in_state_ch1;
#[doc = "IN_SUC_EOF_DES_ADDR_CH1 (r) register accessor: DMA_IN_SUC_EOF_DES_ADDR_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_suc_eof_des_addr_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_suc_eof_des_addr_ch1`] module"]
pub type IN_SUC_EOF_DES_ADDR_CH1 =
    crate::Reg<in_suc_eof_des_addr_ch1::IN_SUC_EOF_DES_ADDR_CH1_SPEC>;
#[doc = "DMA_IN_SUC_EOF_DES_ADDR_CH1_REG."]
pub mod in_suc_eof_des_addr_ch1;
#[doc = "IN_ERR_EOF_DES_ADDR_CH1 (r) register accessor: DMA_IN_ERR_EOF_DES_ADDR_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_err_eof_des_addr_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_err_eof_des_addr_ch1`] module"]
pub type IN_ERR_EOF_DES_ADDR_CH1 =
    crate::Reg<in_err_eof_des_addr_ch1::IN_ERR_EOF_DES_ADDR_CH1_SPEC>;
#[doc = "DMA_IN_ERR_EOF_DES_ADDR_CH1_REG."]
pub mod in_err_eof_des_addr_ch1;
#[doc = "IN_DSCR_CH1 (r) register accessor: DMA_IN_DSCR_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_ch1`] module"]
pub type IN_DSCR_CH1 = crate::Reg<in_dscr_ch1::IN_DSCR_CH1_SPEC>;
#[doc = "DMA_IN_DSCR_CH1_REG."]
pub mod in_dscr_ch1;
#[doc = "IN_DSCR_BF0_CH1 (r) register accessor: DMA_IN_DSCR_BF0_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_bf0_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf0_ch1`] module"]
pub type IN_DSCR_BF0_CH1 = crate::Reg<in_dscr_bf0_ch1::IN_DSCR_BF0_CH1_SPEC>;
#[doc = "DMA_IN_DSCR_BF0_CH1_REG."]
pub mod in_dscr_bf0_ch1;
#[doc = "IN_DSCR_BF1_CH1 (r) register accessor: DMA_IN_DSCR_BF1_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_bf1_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf1_ch1`] module"]
pub type IN_DSCR_BF1_CH1 = crate::Reg<in_dscr_bf1_ch1::IN_DSCR_BF1_CH1_SPEC>;
#[doc = "DMA_IN_DSCR_BF1_CH1_REG."]
pub mod in_dscr_bf1_ch1;
#[doc = "IN_PRI_CH1 (rw) register accessor: DMA_IN_PRI_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pri_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_pri_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pri_ch1`] module"]
pub type IN_PRI_CH1 = crate::Reg<in_pri_ch1::IN_PRI_CH1_SPEC>;
#[doc = "DMA_IN_PRI_CH1_REG."]
pub mod in_pri_ch1;
#[doc = "IN_PERI_SEL_CH1 (rw) register accessor: DMA_IN_PERI_SEL_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_peri_sel_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_peri_sel_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_peri_sel_ch1`] module"]
pub type IN_PERI_SEL_CH1 = crate::Reg<in_peri_sel_ch1::IN_PERI_SEL_CH1_SPEC>;
#[doc = "DMA_IN_PERI_SEL_CH1_REG."]
pub mod in_peri_sel_ch1;
#[doc = "OUT_CONF0_CH1 (rw) register accessor: DMA_OUT_CONF0_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf0_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf0_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf0_ch1`] module"]
pub type OUT_CONF0_CH1 = crate::Reg<out_conf0_ch1::OUT_CONF0_CH1_SPEC>;
#[doc = "DMA_OUT_CONF0_CH1_REG."]
pub mod out_conf0_ch1;
#[doc = "OUT_CONF1_CH1 (rw) register accessor: DMA_OUT_CONF1_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf1_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf1_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf1_ch1`] module"]
pub type OUT_CONF1_CH1 = crate::Reg<out_conf1_ch1::OUT_CONF1_CH1_SPEC>;
#[doc = "DMA_OUT_CONF1_CH1_REG."]
pub mod out_conf1_ch1;
#[doc = "OUTFIFO_STATUS_CH1 (r) register accessor: DMA_OUTFIFO_STATUS_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outfifo_status_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outfifo_status_ch1`] module"]
pub type OUTFIFO_STATUS_CH1 = crate::Reg<outfifo_status_ch1::OUTFIFO_STATUS_CH1_SPEC>;
#[doc = "DMA_OUTFIFO_STATUS_CH1_REG."]
pub mod outfifo_status_ch1;
#[doc = "OUT_PUSH_CH1 (rw) register accessor: DMA_OUT_PUSH_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_push_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_push_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_push_ch1`] module"]
pub type OUT_PUSH_CH1 = crate::Reg<out_push_ch1::OUT_PUSH_CH1_SPEC>;
#[doc = "DMA_OUT_PUSH_CH1_REG."]
pub mod out_push_ch1;
#[doc = "OUT_LINK_CH1 (rw) register accessor: DMA_OUT_LINK_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_link_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link_ch1`] module"]
pub type OUT_LINK_CH1 = crate::Reg<out_link_ch1::OUT_LINK_CH1_SPEC>;
#[doc = "DMA_OUT_LINK_CH1_REG."]
pub mod out_link_ch1;
#[doc = "OUT_STATE_CH1 (r) register accessor: DMA_OUT_STATE_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_state_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_state_ch1`] module"]
pub type OUT_STATE_CH1 = crate::Reg<out_state_ch1::OUT_STATE_CH1_SPEC>;
#[doc = "DMA_OUT_STATE_CH1_REG."]
pub mod out_state_ch1;
#[doc = "OUT_EOF_DES_ADDR_CH1 (r) register accessor: DMA_OUT_EOF_DES_ADDR_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_eof_des_addr_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_des_addr_ch1`] module"]
pub type OUT_EOF_DES_ADDR_CH1 = crate::Reg<out_eof_des_addr_ch1::OUT_EOF_DES_ADDR_CH1_SPEC>;
#[doc = "DMA_OUT_EOF_DES_ADDR_CH1_REG."]
pub mod out_eof_des_addr_ch1;
#[doc = "OUT_EOF_BFR_DES_ADDR_CH1 (r) register accessor: DMA_OUT_EOF_BFR_DES_ADDR_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_eof_bfr_des_addr_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_bfr_des_addr_ch1`] module"]
pub type OUT_EOF_BFR_DES_ADDR_CH1 =
    crate::Reg<out_eof_bfr_des_addr_ch1::OUT_EOF_BFR_DES_ADDR_CH1_SPEC>;
#[doc = "DMA_OUT_EOF_BFR_DES_ADDR_CH1_REG."]
pub mod out_eof_bfr_des_addr_ch1;
#[doc = "OUT_DSCR_CH1 (r) register accessor: DMA_OUT_DSCR_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_ch1`] module"]
pub type OUT_DSCR_CH1 = crate::Reg<out_dscr_ch1::OUT_DSCR_CH1_SPEC>;
#[doc = "DMA_OUT_DSCR_CH1_REG."]
pub mod out_dscr_ch1;
#[doc = "OUT_DSCR_BF0_CH1 (r) register accessor: DMA_OUT_DSCR_BF0_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_bf0_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf0_ch1`] module"]
pub type OUT_DSCR_BF0_CH1 = crate::Reg<out_dscr_bf0_ch1::OUT_DSCR_BF0_CH1_SPEC>;
#[doc = "DMA_OUT_DSCR_BF0_CH1_REG."]
pub mod out_dscr_bf0_ch1;
#[doc = "OUT_DSCR_BF1_CH1 (r) register accessor: DMA_OUT_DSCR_BF1_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_bf1_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf1_ch1`] module"]
pub type OUT_DSCR_BF1_CH1 = crate::Reg<out_dscr_bf1_ch1::OUT_DSCR_BF1_CH1_SPEC>;
#[doc = "DMA_OUT_DSCR_BF1_CH1_REG."]
pub mod out_dscr_bf1_ch1;
#[doc = "OUT_PRI_CH1 (rw) register accessor: DMA_OUT_PRI_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_pri_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_pri_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_pri_ch1`] module"]
pub type OUT_PRI_CH1 = crate::Reg<out_pri_ch1::OUT_PRI_CH1_SPEC>;
#[doc = "DMA_OUT_PRI_CH1_REG."]
pub mod out_pri_ch1;
#[doc = "OUT_PERI_SEL_CH1 (rw) register accessor: DMA_OUT_PERI_SEL_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_peri_sel_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_peri_sel_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_peri_sel_ch1`] module"]
pub type OUT_PERI_SEL_CH1 = crate::Reg<out_peri_sel_ch1::OUT_PERI_SEL_CH1_SPEC>;
#[doc = "DMA_OUT_PERI_SEL_CH1_REG."]
pub mod out_peri_sel_ch1;
#[doc = "IN_CONF0_CH2 (rw) register accessor: DMA_IN_CONF0_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf0_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf0_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf0_ch2`] module"]
pub type IN_CONF0_CH2 = crate::Reg<in_conf0_ch2::IN_CONF0_CH2_SPEC>;
#[doc = "DMA_IN_CONF0_CH2_REG."]
pub mod in_conf0_ch2;
#[doc = "IN_CONF1_CH2 (rw) register accessor: DMA_IN_CONF1_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf1_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf1_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf1_ch2`] module"]
pub type IN_CONF1_CH2 = crate::Reg<in_conf1_ch2::IN_CONF1_CH2_SPEC>;
#[doc = "DMA_IN_CONF1_CH2_REG."]
pub mod in_conf1_ch2;
#[doc = "INFIFO_STATUS_CH2 (r) register accessor: DMA_INFIFO_STATUS_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`infifo_status_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@infifo_status_ch2`] module"]
pub type INFIFO_STATUS_CH2 = crate::Reg<infifo_status_ch2::INFIFO_STATUS_CH2_SPEC>;
#[doc = "DMA_INFIFO_STATUS_CH2_REG."]
pub mod infifo_status_ch2;
#[doc = "IN_POP_CH2 (rw) register accessor: DMA_IN_POP_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pop_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_pop_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pop_ch2`] module"]
pub type IN_POP_CH2 = crate::Reg<in_pop_ch2::IN_POP_CH2_SPEC>;
#[doc = "DMA_IN_POP_CH2_REG."]
pub mod in_pop_ch2;
#[doc = "IN_LINK_CH2 (rw) register accessor: DMA_IN_LINK_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_link_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_link_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link_ch2`] module"]
pub type IN_LINK_CH2 = crate::Reg<in_link_ch2::IN_LINK_CH2_SPEC>;
#[doc = "DMA_IN_LINK_CH2_REG."]
pub mod in_link_ch2;
#[doc = "IN_STATE_CH2 (r) register accessor: DMA_IN_STATE_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_state_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_state_ch2`] module"]
pub type IN_STATE_CH2 = crate::Reg<in_state_ch2::IN_STATE_CH2_SPEC>;
#[doc = "DMA_IN_STATE_CH2_REG."]
pub mod in_state_ch2;
#[doc = "IN_SUC_EOF_DES_ADDR_CH2 (r) register accessor: DMA_IN_SUC_EOF_DES_ADDR_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_suc_eof_des_addr_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_suc_eof_des_addr_ch2`] module"]
pub type IN_SUC_EOF_DES_ADDR_CH2 =
    crate::Reg<in_suc_eof_des_addr_ch2::IN_SUC_EOF_DES_ADDR_CH2_SPEC>;
#[doc = "DMA_IN_SUC_EOF_DES_ADDR_CH2_REG."]
pub mod in_suc_eof_des_addr_ch2;
#[doc = "IN_ERR_EOF_DES_ADDR_CH2 (r) register accessor: DMA_IN_ERR_EOF_DES_ADDR_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_err_eof_des_addr_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_err_eof_des_addr_ch2`] module"]
pub type IN_ERR_EOF_DES_ADDR_CH2 =
    crate::Reg<in_err_eof_des_addr_ch2::IN_ERR_EOF_DES_ADDR_CH2_SPEC>;
#[doc = "DMA_IN_ERR_EOF_DES_ADDR_CH2_REG."]
pub mod in_err_eof_des_addr_ch2;
#[doc = "IN_DSCR_CH2 (r) register accessor: DMA_IN_DSCR_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_ch2`] module"]
pub type IN_DSCR_CH2 = crate::Reg<in_dscr_ch2::IN_DSCR_CH2_SPEC>;
#[doc = "DMA_IN_DSCR_CH2_REG."]
pub mod in_dscr_ch2;
#[doc = "IN_DSCR_BF0_CH2 (r) register accessor: DMA_IN_DSCR_BF0_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_bf0_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf0_ch2`] module"]
pub type IN_DSCR_BF0_CH2 = crate::Reg<in_dscr_bf0_ch2::IN_DSCR_BF0_CH2_SPEC>;
#[doc = "DMA_IN_DSCR_BF0_CH2_REG."]
pub mod in_dscr_bf0_ch2;
#[doc = "IN_DSCR_BF1_CH2 (r) register accessor: DMA_IN_DSCR_BF1_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_bf1_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf1_ch2`] module"]
pub type IN_DSCR_BF1_CH2 = crate::Reg<in_dscr_bf1_ch2::IN_DSCR_BF1_CH2_SPEC>;
#[doc = "DMA_IN_DSCR_BF1_CH2_REG."]
pub mod in_dscr_bf1_ch2;
#[doc = "IN_PRI_CH2 (rw) register accessor: DMA_IN_PRI_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pri_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_pri_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pri_ch2`] module"]
pub type IN_PRI_CH2 = crate::Reg<in_pri_ch2::IN_PRI_CH2_SPEC>;
#[doc = "DMA_IN_PRI_CH2_REG."]
pub mod in_pri_ch2;
#[doc = "IN_PERI_SEL_CH2 (rw) register accessor: DMA_IN_PERI_SEL_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_peri_sel_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_peri_sel_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_peri_sel_ch2`] module"]
pub type IN_PERI_SEL_CH2 = crate::Reg<in_peri_sel_ch2::IN_PERI_SEL_CH2_SPEC>;
#[doc = "DMA_IN_PERI_SEL_CH2_REG."]
pub mod in_peri_sel_ch2;
#[doc = "OUT_CONF0_CH2 (rw) register accessor: DMA_OUT_CONF0_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf0_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf0_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf0_ch2`] module"]
pub type OUT_CONF0_CH2 = crate::Reg<out_conf0_ch2::OUT_CONF0_CH2_SPEC>;
#[doc = "DMA_OUT_CONF0_CH2_REG."]
pub mod out_conf0_ch2;
#[doc = "OUT_CONF1_CH2 (rw) register accessor: DMA_OUT_CONF1_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf1_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf1_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf1_ch2`] module"]
pub type OUT_CONF1_CH2 = crate::Reg<out_conf1_ch2::OUT_CONF1_CH2_SPEC>;
#[doc = "DMA_OUT_CONF1_CH2_REG."]
pub mod out_conf1_ch2;
#[doc = "OUTFIFO_STATUS_CH2 (r) register accessor: DMA_OUTFIFO_STATUS_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outfifo_status_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outfifo_status_ch2`] module"]
pub type OUTFIFO_STATUS_CH2 = crate::Reg<outfifo_status_ch2::OUTFIFO_STATUS_CH2_SPEC>;
#[doc = "DMA_OUTFIFO_STATUS_CH2_REG."]
pub mod outfifo_status_ch2;
#[doc = "OUT_PUSH_CH2 (rw) register accessor: DMA_OUT_PUSH_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_push_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_push_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_push_ch2`] module"]
pub type OUT_PUSH_CH2 = crate::Reg<out_push_ch2::OUT_PUSH_CH2_SPEC>;
#[doc = "DMA_OUT_PUSH_CH2_REG."]
pub mod out_push_ch2;
#[doc = "OUT_LINK_CH2 (rw) register accessor: DMA_OUT_LINK_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_link_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link_ch2`] module"]
pub type OUT_LINK_CH2 = crate::Reg<out_link_ch2::OUT_LINK_CH2_SPEC>;
#[doc = "DMA_OUT_LINK_CH2_REG."]
pub mod out_link_ch2;
#[doc = "OUT_STATE_CH2 (r) register accessor: DMA_OUT_STATE_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_state_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_state_ch2`] module"]
pub type OUT_STATE_CH2 = crate::Reg<out_state_ch2::OUT_STATE_CH2_SPEC>;
#[doc = "DMA_OUT_STATE_CH2_REG."]
pub mod out_state_ch2;
#[doc = "OUT_EOF_DES_ADDR_CH2 (r) register accessor: DMA_OUT_EOF_DES_ADDR_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_eof_des_addr_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_des_addr_ch2`] module"]
pub type OUT_EOF_DES_ADDR_CH2 = crate::Reg<out_eof_des_addr_ch2::OUT_EOF_DES_ADDR_CH2_SPEC>;
#[doc = "DMA_OUT_EOF_DES_ADDR_CH2_REG."]
pub mod out_eof_des_addr_ch2;
#[doc = "OUT_EOF_BFR_DES_ADDR_CH2 (r) register accessor: DMA_OUT_EOF_BFR_DES_ADDR_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_eof_bfr_des_addr_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_bfr_des_addr_ch2`] module"]
pub type OUT_EOF_BFR_DES_ADDR_CH2 =
    crate::Reg<out_eof_bfr_des_addr_ch2::OUT_EOF_BFR_DES_ADDR_CH2_SPEC>;
#[doc = "DMA_OUT_EOF_BFR_DES_ADDR_CH2_REG."]
pub mod out_eof_bfr_des_addr_ch2;
#[doc = "OUT_DSCR_CH2 (r) register accessor: DMA_OUT_DSCR_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_ch2`] module"]
pub type OUT_DSCR_CH2 = crate::Reg<out_dscr_ch2::OUT_DSCR_CH2_SPEC>;
#[doc = "DMA_OUT_DSCR_CH2_REG."]
pub mod out_dscr_ch2;
#[doc = "OUT_DSCR_BF0_CH2 (r) register accessor: DMA_OUT_DSCR_BF0_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_bf0_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf0_ch2`] module"]
pub type OUT_DSCR_BF0_CH2 = crate::Reg<out_dscr_bf0_ch2::OUT_DSCR_BF0_CH2_SPEC>;
#[doc = "DMA_OUT_DSCR_BF0_CH2_REG."]
pub mod out_dscr_bf0_ch2;
#[doc = "OUT_DSCR_BF1_CH2 (r) register accessor: DMA_OUT_DSCR_BF1_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_bf1_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf1_ch2`] module"]
pub type OUT_DSCR_BF1_CH2 = crate::Reg<out_dscr_bf1_ch2::OUT_DSCR_BF1_CH2_SPEC>;
#[doc = "DMA_OUT_DSCR_BF1_CH2_REG."]
pub mod out_dscr_bf1_ch2;
#[doc = "OUT_PRI_CH2 (rw) register accessor: DMA_OUT_PRI_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_pri_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_pri_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_pri_ch2`] module"]
pub type OUT_PRI_CH2 = crate::Reg<out_pri_ch2::OUT_PRI_CH2_SPEC>;
#[doc = "DMA_OUT_PRI_CH2_REG."]
pub mod out_pri_ch2;
#[doc = "OUT_PERI_SEL_CH2 (rw) register accessor: DMA_OUT_PERI_SEL_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_peri_sel_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_peri_sel_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_peri_sel_ch2`] module"]
pub type OUT_PERI_SEL_CH2 = crate::Reg<out_peri_sel_ch2::OUT_PERI_SEL_CH2_SPEC>;
#[doc = "DMA_OUT_PERI_SEL_CH2_REG."]
pub mod out_peri_sel_ch2;
