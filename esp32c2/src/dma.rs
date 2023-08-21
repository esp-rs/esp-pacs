#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA_INT_RAW_CH0_REG."]
    pub int_raw_ch0: INT_RAW_CH0,
    #[doc = "0x04 - DMA_INT_ST_CH0_REG."]
    pub int_st_ch0: INT_ST_CH0,
    #[doc = "0x08 - DMA_INT_ENA_CH0_REG."]
    pub int_ena_ch0: INT_ENA_CH0,
    #[doc = "0x0c - DMA_INT_CLR_CH0_REG."]
    pub int_clr_ch0: INT_CLR_CH0,
    _reserved4: [u8; 0x30],
    #[doc = "0x40 - DMA_AHB_TEST_REG."]
    pub ahb_test: AHB_TEST,
    #[doc = "0x44 - DMA_MISC_CONF_REG."]
    pub misc_conf: MISC_CONF,
    #[doc = "0x48 - DMA_DATE_REG."]
    pub date: DATE,
    _reserved7: [u8; 0x24],
    #[doc = "0x70 - DMA_IN_CONF0_CH0_REG."]
    pub in_conf0_ch0: IN_CONF0_CH0,
    #[doc = "0x74 - DMA_IN_CONF1_CH0_REG."]
    pub in_conf1_ch0: IN_CONF1_CH0,
    #[doc = "0x78 - DMA_INFIFO_STATUS_CH0_REG."]
    pub infifo_status_ch0: INFIFO_STATUS_CH0,
    #[doc = "0x7c - DMA_IN_POP_CH0_REG."]
    pub in_pop_ch0: IN_POP_CH0,
    #[doc = "0x80 - DMA_IN_LINK_CH0_REG."]
    pub in_link_ch0: IN_LINK_CH0,
    #[doc = "0x84 - DMA_IN_STATE_CH0_REG."]
    pub in_state_ch0: IN_STATE_CH0,
    #[doc = "0x88 - DMA_IN_SUC_EOF_DES_ADDR_CH0_REG."]
    pub in_suc_eof_des_addr_ch0: IN_SUC_EOF_DES_ADDR_CH0,
    #[doc = "0x8c - DMA_IN_ERR_EOF_DES_ADDR_CH0_REG."]
    pub in_err_eof_des_addr_ch0: IN_ERR_EOF_DES_ADDR_CH0,
    #[doc = "0x90 - DMA_IN_DSCR_CH0_REG."]
    pub in_dscr_ch0: IN_DSCR_CH0,
    #[doc = "0x94 - DMA_IN_DSCR_BF0_CH0_REG."]
    pub in_dscr_bf0_ch0: IN_DSCR_BF0_CH0,
    #[doc = "0x98 - DMA_IN_DSCR_BF1_CH0_REG."]
    pub in_dscr_bf1_ch0: IN_DSCR_BF1_CH0,
    #[doc = "0x9c - DMA_IN_PRI_CH0_REG."]
    pub in_pri_ch0: IN_PRI_CH0,
    #[doc = "0xa0 - DMA_IN_PERI_SEL_CH0_REG."]
    pub in_peri_sel_ch0: IN_PERI_SEL_CH0,
    _reserved20: [u8; 0x2c],
    #[doc = "0xd0 - DMA_OUT_CONF0_CH0_REG."]
    pub out_conf0_ch0: OUT_CONF0_CH0,
    #[doc = "0xd4 - DMA_OUT_CONF1_CH0_REG."]
    pub out_conf1_ch0: OUT_CONF1_CH0,
    #[doc = "0xd8 - DMA_OUTFIFO_STATUS_CH0_REG."]
    pub outfifo_status_ch0: OUTFIFO_STATUS_CH0,
    #[doc = "0xdc - DMA_OUT_PUSH_CH0_REG."]
    pub out_push_ch0: OUT_PUSH_CH0,
    #[doc = "0xe0 - DMA_OUT_LINK_CH0_REG."]
    pub out_link_ch0: OUT_LINK_CH0,
    #[doc = "0xe4 - DMA_OUT_STATE_CH0_REG."]
    pub out_state_ch0: OUT_STATE_CH0,
    #[doc = "0xe8 - DMA_OUT_EOF_DES_ADDR_CH0_REG."]
    pub out_eof_des_addr_ch0: OUT_EOF_DES_ADDR_CH0,
    #[doc = "0xec - DMA_OUT_EOF_BFR_DES_ADDR_CH0_REG."]
    pub out_eof_bfr_des_addr_ch0: OUT_EOF_BFR_DES_ADDR_CH0,
    #[doc = "0xf0 - DMA_OUT_DSCR_CH0_REG."]
    pub out_dscr_ch0: OUT_DSCR_CH0,
    #[doc = "0xf4 - DMA_OUT_DSCR_BF0_CH0_REG."]
    pub out_dscr_bf0_ch0: OUT_DSCR_BF0_CH0,
    #[doc = "0xf8 - DMA_OUT_DSCR_BF1_CH0_REG."]
    pub out_dscr_bf1_ch0: OUT_DSCR_BF1_CH0,
    #[doc = "0xfc - DMA_OUT_PRI_CH0_REG."]
    pub out_pri_ch0: OUT_PRI_CH0,
    #[doc = "0x100 - DMA_OUT_PERI_SEL_CH0_REG."]
    pub out_peri_sel_ch0: OUT_PERI_SEL_CH0,
}
#[doc = "INT_RAW_CH0 (rw) register accessor: DMA_INT_RAW_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_raw_ch0`] module"]
pub type INT_RAW_CH0 = crate::Reg<int_raw_ch0::INT_RAW_CH0_SPEC>;
#[doc = "DMA_INT_RAW_CH0_REG."]
pub mod int_raw_ch0;
#[doc = "INT_ST_CH0 (r) register accessor: DMA_INT_ST_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_st_ch0`] module"]
pub type INT_ST_CH0 = crate::Reg<int_st_ch0::INT_ST_CH0_SPEC>;
#[doc = "DMA_INT_ST_CH0_REG."]
pub mod int_st_ch0;
#[doc = "INT_ENA_CH0 (rw) register accessor: DMA_INT_ENA_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_ena_ch0`] module"]
pub type INT_ENA_CH0 = crate::Reg<int_ena_ch0::INT_ENA_CH0_SPEC>;
#[doc = "DMA_INT_ENA_CH0_REG."]
pub mod int_ena_ch0;
#[doc = "INT_CLR_CH0 (w) register accessor: DMA_INT_CLR_CH0_REG.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr_ch0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_clr_ch0`] module"]
pub type INT_CLR_CH0 = crate::Reg<int_clr_ch0::INT_CLR_CH0_SPEC>;
#[doc = "DMA_INT_CLR_CH0_REG."]
pub mod int_clr_ch0;
#[doc = "AHB_TEST (rw) register accessor: DMA_AHB_TEST_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ahb_test`] module"]
pub type AHB_TEST = crate::Reg<ahb_test::AHB_TEST_SPEC>;
#[doc = "DMA_AHB_TEST_REG."]
pub mod ahb_test;
#[doc = "MISC_CONF (rw) register accessor: DMA_MISC_CONF_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`misc_conf`] module"]
pub type MISC_CONF = crate::Reg<misc_conf::MISC_CONF_SPEC>;
#[doc = "DMA_MISC_CONF_REG."]
pub mod misc_conf;
#[doc = "DATE (rw) register accessor: DMA_DATE_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "DMA_DATE_REG."]
pub mod date;
#[doc = "IN_CONF0_CH0 (rw) register accessor: DMA_IN_CONF0_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf0_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf0_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in_conf0_ch0`] module"]
pub type IN_CONF0_CH0 = crate::Reg<in_conf0_ch0::IN_CONF0_CH0_SPEC>;
#[doc = "DMA_IN_CONF0_CH0_REG."]
pub mod in_conf0_ch0;
#[doc = "IN_CONF1_CH0 (rw) register accessor: DMA_IN_CONF1_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf1_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf1_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in_conf1_ch0`] module"]
pub type IN_CONF1_CH0 = crate::Reg<in_conf1_ch0::IN_CONF1_CH0_SPEC>;
#[doc = "DMA_IN_CONF1_CH0_REG."]
pub mod in_conf1_ch0;
#[doc = "INFIFO_STATUS_CH0 (r) register accessor: DMA_INFIFO_STATUS_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`infifo_status_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`infifo_status_ch0`] module"]
pub type INFIFO_STATUS_CH0 = crate::Reg<infifo_status_ch0::INFIFO_STATUS_CH0_SPEC>;
#[doc = "DMA_INFIFO_STATUS_CH0_REG."]
pub mod infifo_status_ch0;
#[doc = "IN_POP_CH0 (rw) register accessor: DMA_IN_POP_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pop_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_pop_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in_pop_ch0`] module"]
pub type IN_POP_CH0 = crate::Reg<in_pop_ch0::IN_POP_CH0_SPEC>;
#[doc = "DMA_IN_POP_CH0_REG."]
pub mod in_pop_ch0;
#[doc = "IN_LINK_CH0 (rw) register accessor: DMA_IN_LINK_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_link_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_link_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in_link_ch0`] module"]
pub type IN_LINK_CH0 = crate::Reg<in_link_ch0::IN_LINK_CH0_SPEC>;
#[doc = "DMA_IN_LINK_CH0_REG."]
pub mod in_link_ch0;
#[doc = "IN_STATE_CH0 (r) register accessor: DMA_IN_STATE_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_state_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in_state_ch0`] module"]
pub type IN_STATE_CH0 = crate::Reg<in_state_ch0::IN_STATE_CH0_SPEC>;
#[doc = "DMA_IN_STATE_CH0_REG."]
pub mod in_state_ch0;
#[doc = "IN_SUC_EOF_DES_ADDR_CH0 (r) register accessor: DMA_IN_SUC_EOF_DES_ADDR_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_suc_eof_des_addr_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in_suc_eof_des_addr_ch0`] module"]
pub type IN_SUC_EOF_DES_ADDR_CH0 =
    crate::Reg<in_suc_eof_des_addr_ch0::IN_SUC_EOF_DES_ADDR_CH0_SPEC>;
#[doc = "DMA_IN_SUC_EOF_DES_ADDR_CH0_REG."]
pub mod in_suc_eof_des_addr_ch0;
#[doc = "IN_ERR_EOF_DES_ADDR_CH0 (r) register accessor: DMA_IN_ERR_EOF_DES_ADDR_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_err_eof_des_addr_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in_err_eof_des_addr_ch0`] module"]
pub type IN_ERR_EOF_DES_ADDR_CH0 =
    crate::Reg<in_err_eof_des_addr_ch0::IN_ERR_EOF_DES_ADDR_CH0_SPEC>;
#[doc = "DMA_IN_ERR_EOF_DES_ADDR_CH0_REG."]
pub mod in_err_eof_des_addr_ch0;
#[doc = "IN_DSCR_CH0 (r) register accessor: DMA_IN_DSCR_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in_dscr_ch0`] module"]
pub type IN_DSCR_CH0 = crate::Reg<in_dscr_ch0::IN_DSCR_CH0_SPEC>;
#[doc = "DMA_IN_DSCR_CH0_REG."]
pub mod in_dscr_ch0;
#[doc = "IN_DSCR_BF0_CH0 (r) register accessor: DMA_IN_DSCR_BF0_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_bf0_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in_dscr_bf0_ch0`] module"]
pub type IN_DSCR_BF0_CH0 = crate::Reg<in_dscr_bf0_ch0::IN_DSCR_BF0_CH0_SPEC>;
#[doc = "DMA_IN_DSCR_BF0_CH0_REG."]
pub mod in_dscr_bf0_ch0;
#[doc = "IN_DSCR_BF1_CH0 (r) register accessor: DMA_IN_DSCR_BF1_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_bf1_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in_dscr_bf1_ch0`] module"]
pub type IN_DSCR_BF1_CH0 = crate::Reg<in_dscr_bf1_ch0::IN_DSCR_BF1_CH0_SPEC>;
#[doc = "DMA_IN_DSCR_BF1_CH0_REG."]
pub mod in_dscr_bf1_ch0;
#[doc = "IN_PRI_CH0 (rw) register accessor: DMA_IN_PRI_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pri_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_pri_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in_pri_ch0`] module"]
pub type IN_PRI_CH0 = crate::Reg<in_pri_ch0::IN_PRI_CH0_SPEC>;
#[doc = "DMA_IN_PRI_CH0_REG."]
pub mod in_pri_ch0;
#[doc = "IN_PERI_SEL_CH0 (rw) register accessor: DMA_IN_PERI_SEL_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_peri_sel_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_peri_sel_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in_peri_sel_ch0`] module"]
pub type IN_PERI_SEL_CH0 = crate::Reg<in_peri_sel_ch0::IN_PERI_SEL_CH0_SPEC>;
#[doc = "DMA_IN_PERI_SEL_CH0_REG."]
pub mod in_peri_sel_ch0;
#[doc = "OUT_CONF0_CH0 (rw) register accessor: DMA_OUT_CONF0_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf0_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf0_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_conf0_ch0`] module"]
pub type OUT_CONF0_CH0 = crate::Reg<out_conf0_ch0::OUT_CONF0_CH0_SPEC>;
#[doc = "DMA_OUT_CONF0_CH0_REG."]
pub mod out_conf0_ch0;
#[doc = "OUT_CONF1_CH0 (rw) register accessor: DMA_OUT_CONF1_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf1_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf1_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_conf1_ch0`] module"]
pub type OUT_CONF1_CH0 = crate::Reg<out_conf1_ch0::OUT_CONF1_CH0_SPEC>;
#[doc = "DMA_OUT_CONF1_CH0_REG."]
pub mod out_conf1_ch0;
#[doc = "OUTFIFO_STATUS_CH0 (r) register accessor: DMA_OUTFIFO_STATUS_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outfifo_status_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`outfifo_status_ch0`] module"]
pub type OUTFIFO_STATUS_CH0 = crate::Reg<outfifo_status_ch0::OUTFIFO_STATUS_CH0_SPEC>;
#[doc = "DMA_OUTFIFO_STATUS_CH0_REG."]
pub mod outfifo_status_ch0;
#[doc = "OUT_PUSH_CH0 (rw) register accessor: DMA_OUT_PUSH_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_push_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_push_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_push_ch0`] module"]
pub type OUT_PUSH_CH0 = crate::Reg<out_push_ch0::OUT_PUSH_CH0_SPEC>;
#[doc = "DMA_OUT_PUSH_CH0_REG."]
pub mod out_push_ch0;
#[doc = "OUT_LINK_CH0 (rw) register accessor: DMA_OUT_LINK_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_link_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_link_ch0`] module"]
pub type OUT_LINK_CH0 = crate::Reg<out_link_ch0::OUT_LINK_CH0_SPEC>;
#[doc = "DMA_OUT_LINK_CH0_REG."]
pub mod out_link_ch0;
#[doc = "OUT_STATE_CH0 (r) register accessor: DMA_OUT_STATE_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_state_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_state_ch0`] module"]
pub type OUT_STATE_CH0 = crate::Reg<out_state_ch0::OUT_STATE_CH0_SPEC>;
#[doc = "DMA_OUT_STATE_CH0_REG."]
pub mod out_state_ch0;
#[doc = "OUT_EOF_DES_ADDR_CH0 (r) register accessor: DMA_OUT_EOF_DES_ADDR_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_eof_des_addr_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_eof_des_addr_ch0`] module"]
pub type OUT_EOF_DES_ADDR_CH0 = crate::Reg<out_eof_des_addr_ch0::OUT_EOF_DES_ADDR_CH0_SPEC>;
#[doc = "DMA_OUT_EOF_DES_ADDR_CH0_REG."]
pub mod out_eof_des_addr_ch0;
#[doc = "OUT_EOF_BFR_DES_ADDR_CH0 (r) register accessor: DMA_OUT_EOF_BFR_DES_ADDR_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_eof_bfr_des_addr_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_eof_bfr_des_addr_ch0`] module"]
pub type OUT_EOF_BFR_DES_ADDR_CH0 =
    crate::Reg<out_eof_bfr_des_addr_ch0::OUT_EOF_BFR_DES_ADDR_CH0_SPEC>;
#[doc = "DMA_OUT_EOF_BFR_DES_ADDR_CH0_REG."]
pub mod out_eof_bfr_des_addr_ch0;
#[doc = "OUT_DSCR_CH0 (r) register accessor: DMA_OUT_DSCR_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_dscr_ch0`] module"]
pub type OUT_DSCR_CH0 = crate::Reg<out_dscr_ch0::OUT_DSCR_CH0_SPEC>;
#[doc = "DMA_OUT_DSCR_CH0_REG."]
pub mod out_dscr_ch0;
#[doc = "OUT_DSCR_BF0_CH0 (r) register accessor: DMA_OUT_DSCR_BF0_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_bf0_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_dscr_bf0_ch0`] module"]
pub type OUT_DSCR_BF0_CH0 = crate::Reg<out_dscr_bf0_ch0::OUT_DSCR_BF0_CH0_SPEC>;
#[doc = "DMA_OUT_DSCR_BF0_CH0_REG."]
pub mod out_dscr_bf0_ch0;
#[doc = "OUT_DSCR_BF1_CH0 (r) register accessor: DMA_OUT_DSCR_BF1_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_bf1_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_dscr_bf1_ch0`] module"]
pub type OUT_DSCR_BF1_CH0 = crate::Reg<out_dscr_bf1_ch0::OUT_DSCR_BF1_CH0_SPEC>;
#[doc = "DMA_OUT_DSCR_BF1_CH0_REG."]
pub mod out_dscr_bf1_ch0;
#[doc = "OUT_PRI_CH0 (rw) register accessor: DMA_OUT_PRI_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_pri_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_pri_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_pri_ch0`] module"]
pub type OUT_PRI_CH0 = crate::Reg<out_pri_ch0::OUT_PRI_CH0_SPEC>;
#[doc = "DMA_OUT_PRI_CH0_REG."]
pub mod out_pri_ch0;
#[doc = "OUT_PERI_SEL_CH0 (rw) register accessor: DMA_OUT_PERI_SEL_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_peri_sel_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_peri_sel_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_peri_sel_ch0`] module"]
pub type OUT_PERI_SEL_CH0 = crate::Reg<out_peri_sel_ch0::OUT_PERI_SEL_CH0_SPEC>;
#[doc = "DMA_OUT_PERI_SEL_CH0_REG."]
pub mod out_peri_sel_ch0;
