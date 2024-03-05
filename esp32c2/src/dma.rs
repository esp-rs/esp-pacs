#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    int_raw_ch: [INT_RAW_CH; 1],
    int_st_ch0: INT_ST_CH0,
    int_ena_ch: [INT_ENA_CH; 1],
    int_clr_ch: [INT_CLR_CH; 1],
    _reserved4: [u8; 0x30],
    ahb_test: AHB_TEST,
    misc_conf: MISC_CONF,
    date: DATE,
    _reserved7: [u8; 0x24],
    in_conf0_ch: [IN_CONF0_CH; 1],
    in_conf1_ch0: IN_CONF1_CH0,
    infifo_status_ch0: INFIFO_STATUS_CH0,
    in_pop_ch0: IN_POP_CH0,
    in_link_ch: [IN_LINK_CH; 1],
    in_state_ch0: IN_STATE_CH0,
    in_suc_eof_des_addr_ch0: IN_SUC_EOF_DES_ADDR_CH0,
    in_err_eof_des_addr_ch0: IN_ERR_EOF_DES_ADDR_CH0,
    in_dscr_ch0: IN_DSCR_CH0,
    in_dscr_bf0_ch: [IN_DSCR_BF0_CH; 1],
    in_dscr_bf1_ch0: IN_DSCR_BF1_CH0,
    in_pri_ch: [IN_PRI_CH; 1],
    in_peri_sel_ch: [IN_PERI_SEL_CH; 1],
    _reserved20: [u8; 0x2c],
    out_conf0_ch: [OUT_CONF0_CH; 1],
    out_conf1_ch: [OUT_CONF1_CH; 1],
    outfifo_status_ch0: OUTFIFO_STATUS_CH0,
    out_push_ch0: OUT_PUSH_CH0,
    out_link_ch: [OUT_LINK_CH; 1],
    out_state_ch0: OUT_STATE_CH0,
    out_eof_des_addr_ch: [OUT_EOF_DES_ADDR_CH; 1],
    out_eof_bfr_des_addr_ch0: OUT_EOF_BFR_DES_ADDR_CH0,
    out_dscr_ch0: OUT_DSCR_CH0,
    out_dscr_bf0_ch0: OUT_DSCR_BF0_CH0,
    out_dscr_bf1_ch0: OUT_DSCR_BF1_CH0,
    out_pri_ch: [OUT_PRI_CH; 1],
    out_peri_sel_ch: [OUT_PERI_SEL_CH; 1],
}
impl RegisterBlock {
    #[doc = "0x00 - DMA_INT_RAW_CH%s_REG."]
    #[inline(always)]
    pub const fn int_raw_ch(&self, n: usize) -> &INT_RAW_CH {
        &self.int_raw_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00 - DMA_INT_RAW_CH%s_REG."]
    #[inline(always)]
    pub fn int_raw_ch_iter(&self) -> impl Iterator<Item = &INT_RAW_CH> {
        self.int_raw_ch.iter()
    }
    #[doc = "0x04 - DMA_INT_ST_CH0_REG."]
    #[inline(always)]
    pub const fn int_st_ch0(&self) -> &INT_ST_CH0 {
        &self.int_st_ch0
    }
    #[doc = "0x08 - DMA_INT_ENA_CH%s_REG."]
    #[inline(always)]
    pub const fn int_ena_ch(&self, n: usize) -> &INT_ENA_CH {
        &self.int_ena_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08 - DMA_INT_ENA_CH%s_REG."]
    #[inline(always)]
    pub fn int_ena_ch_iter(&self) -> impl Iterator<Item = &INT_ENA_CH> {
        self.int_ena_ch.iter()
    }
    #[doc = "0x0c - DMA_INT_CLR_CH%s_REG."]
    #[inline(always)]
    pub const fn int_clr_ch(&self, n: usize) -> &INT_CLR_CH {
        &self.int_clr_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c - DMA_INT_CLR_CH%s_REG."]
    #[inline(always)]
    pub fn int_clr_ch_iter(&self) -> impl Iterator<Item = &INT_CLR_CH> {
        self.int_clr_ch.iter()
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
    #[doc = "0x70 - DMA_IN_CONF%s_CH%s_REG."]
    #[inline(always)]
    pub const fn in_conf0_ch(&self, n: usize) -> &IN_CONF0_CH {
        &self.in_conf0_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70 - DMA_IN_CONF%s_CH%s_REG."]
    #[inline(always)]
    pub fn in_conf0_ch_iter(&self) -> impl Iterator<Item = &IN_CONF0_CH> {
        self.in_conf0_ch.iter()
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
    #[doc = "0x80 - DMA_IN_LINK_CH%s_REG."]
    #[inline(always)]
    pub const fn in_link_ch(&self, n: usize) -> &IN_LINK_CH {
        &self.in_link_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80 - DMA_IN_LINK_CH%s_REG."]
    #[inline(always)]
    pub fn in_link_ch_iter(&self) -> impl Iterator<Item = &IN_LINK_CH> {
        self.in_link_ch.iter()
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
    #[doc = "0x94 - DMA_IN_DSCR_BF%s_CH%s_REG."]
    #[inline(always)]
    pub const fn in_dscr_bf0_ch(&self, n: usize) -> &IN_DSCR_BF0_CH {
        &self.in_dscr_bf0_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x94 - DMA_IN_DSCR_BF%s_CH%s_REG."]
    #[inline(always)]
    pub fn in_dscr_bf0_ch_iter(&self) -> impl Iterator<Item = &IN_DSCR_BF0_CH> {
        self.in_dscr_bf0_ch.iter()
    }
    #[doc = "0x98 - DMA_IN_DSCR_BF1_CH0_REG."]
    #[inline(always)]
    pub const fn in_dscr_bf1_ch0(&self) -> &IN_DSCR_BF1_CH0 {
        &self.in_dscr_bf1_ch0
    }
    #[doc = "0x9c - DMA_IN_PRI_CH%s_REG."]
    #[inline(always)]
    pub const fn in_pri_ch(&self, n: usize) -> &IN_PRI_CH {
        &self.in_pri_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x9c - DMA_IN_PRI_CH%s_REG."]
    #[inline(always)]
    pub fn in_pri_ch_iter(&self) -> impl Iterator<Item = &IN_PRI_CH> {
        self.in_pri_ch.iter()
    }
    #[doc = "0xa0 - DMA_IN_PERI_SEL_CH%s_REG."]
    #[inline(always)]
    pub const fn in_peri_sel_ch(&self, n: usize) -> &IN_PERI_SEL_CH {
        &self.in_peri_sel_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa0 - DMA_IN_PERI_SEL_CH%s_REG."]
    #[inline(always)]
    pub fn in_peri_sel_ch_iter(&self) -> impl Iterator<Item = &IN_PERI_SEL_CH> {
        self.in_peri_sel_ch.iter()
    }
    #[doc = "0xd0 - DMA_OUT_CONF%s_CH%s_REG."]
    #[inline(always)]
    pub const fn out_conf0_ch(&self, n: usize) -> &OUT_CONF0_CH {
        &self.out_conf0_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xd0 - DMA_OUT_CONF%s_CH%s_REG."]
    #[inline(always)]
    pub fn out_conf0_ch_iter(&self) -> impl Iterator<Item = &OUT_CONF0_CH> {
        self.out_conf0_ch.iter()
    }
    #[doc = "0xd4 - DMA_OUT_CONF1_CH%s_REG."]
    #[inline(always)]
    pub const fn out_conf1_ch(&self, n: usize) -> &OUT_CONF1_CH {
        &self.out_conf1_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xd4 - DMA_OUT_CONF1_CH%s_REG."]
    #[inline(always)]
    pub fn out_conf1_ch_iter(&self) -> impl Iterator<Item = &OUT_CONF1_CH> {
        self.out_conf1_ch.iter()
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
    #[doc = "0xe0 - DMA_OUT_LINK_CH%s_REG."]
    #[inline(always)]
    pub const fn out_link_ch(&self, n: usize) -> &OUT_LINK_CH {
        &self.out_link_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xe0 - DMA_OUT_LINK_CH%s_REG."]
    #[inline(always)]
    pub fn out_link_ch_iter(&self) -> impl Iterator<Item = &OUT_LINK_CH> {
        self.out_link_ch.iter()
    }
    #[doc = "0xe4 - DMA_OUT_STATE_CH0_REG."]
    #[inline(always)]
    pub const fn out_state_ch0(&self) -> &OUT_STATE_CH0 {
        &self.out_state_ch0
    }
    #[doc = "0xe8 - DMA_OUT_EOF_DES_ADDR_CH%s_REG."]
    #[inline(always)]
    pub const fn out_eof_des_addr_ch(&self, n: usize) -> &OUT_EOF_DES_ADDR_CH {
        &self.out_eof_des_addr_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xe8 - DMA_OUT_EOF_DES_ADDR_CH%s_REG."]
    #[inline(always)]
    pub fn out_eof_des_addr_ch_iter(&self) -> impl Iterator<Item = &OUT_EOF_DES_ADDR_CH> {
        self.out_eof_des_addr_ch.iter()
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
    #[doc = "0xfc - DMA_OUT_PRI_CH%s_REG."]
    #[inline(always)]
    pub const fn out_pri_ch(&self, n: usize) -> &OUT_PRI_CH {
        &self.out_pri_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xfc - DMA_OUT_PRI_CH%s_REG."]
    #[inline(always)]
    pub fn out_pri_ch_iter(&self) -> impl Iterator<Item = &OUT_PRI_CH> {
        self.out_pri_ch.iter()
    }
    #[doc = "0x100 - DMA_OUT_PERI_SEL_CH%s_REG."]
    #[inline(always)]
    pub const fn out_peri_sel_ch(&self, n: usize) -> &OUT_PERI_SEL_CH {
        &self.out_peri_sel_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100 - DMA_OUT_PERI_SEL_CH%s_REG."]
    #[inline(always)]
    pub fn out_peri_sel_ch_iter(&self) -> impl Iterator<Item = &OUT_PERI_SEL_CH> {
        self.out_peri_sel_ch.iter()
    }
}
#[doc = "INT_RAW_CH (rw) register accessor: DMA_INT_RAW_CH%s_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw_ch`] module"]
pub type INT_RAW_CH = crate::Reg<int_raw_ch::INT_RAW_CH_SPEC>;
#[doc = "DMA_INT_RAW_CH%s_REG."]
pub mod int_raw_ch;
#[doc = "INT_ST_CH0 (r) register accessor: DMA_INT_ST_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_ch0`] module"]
pub type INT_ST_CH0 = crate::Reg<int_st_ch0::INT_ST_CH0_SPEC>;
#[doc = "DMA_INT_ST_CH0_REG."]
pub mod int_st_ch0;
#[doc = "INT_ENA_CH (rw) register accessor: DMA_INT_ENA_CH%s_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena_ch`] module"]
pub type INT_ENA_CH = crate::Reg<int_ena_ch::INT_ENA_CH_SPEC>;
#[doc = "DMA_INT_ENA_CH%s_REG."]
pub mod int_ena_ch;
#[doc = "INT_CLR_CH (w) register accessor: DMA_INT_CLR_CH%s_REG.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr_ch::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr_ch`] module"]
pub type INT_CLR_CH = crate::Reg<int_clr_ch::INT_CLR_CH_SPEC>;
#[doc = "DMA_INT_CLR_CH%s_REG."]
pub mod int_clr_ch;
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
#[doc = "IN_CONF0_CH (rw) register accessor: DMA_IN_CONF%s_CH%s_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf0_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf0_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf0_ch`] module"]
pub type IN_CONF0_CH = crate::Reg<in_conf0_ch::IN_CONF0_CH_SPEC>;
#[doc = "DMA_IN_CONF%s_CH%s_REG."]
pub mod in_conf0_ch;
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
#[doc = "IN_LINK_CH (rw) register accessor: DMA_IN_LINK_CH%s_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_link_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_link_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link_ch`] module"]
pub type IN_LINK_CH = crate::Reg<in_link_ch::IN_LINK_CH_SPEC>;
#[doc = "DMA_IN_LINK_CH%s_REG."]
pub mod in_link_ch;
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
#[doc = "IN_DSCR_BF0_CH (r) register accessor: DMA_IN_DSCR_BF%s_CH%s_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_bf0_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf0_ch`] module"]
pub type IN_DSCR_BF0_CH = crate::Reg<in_dscr_bf0_ch::IN_DSCR_BF0_CH_SPEC>;
#[doc = "DMA_IN_DSCR_BF%s_CH%s_REG."]
pub mod in_dscr_bf0_ch;
#[doc = "IN_DSCR_BF1_CH0 (r) register accessor: DMA_IN_DSCR_BF1_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_bf1_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf1_ch0`] module"]
pub type IN_DSCR_BF1_CH0 = crate::Reg<in_dscr_bf1_ch0::IN_DSCR_BF1_CH0_SPEC>;
#[doc = "DMA_IN_DSCR_BF1_CH0_REG."]
pub mod in_dscr_bf1_ch0;
#[doc = "IN_PRI_CH (rw) register accessor: DMA_IN_PRI_CH%s_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pri_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_pri_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pri_ch`] module"]
pub type IN_PRI_CH = crate::Reg<in_pri_ch::IN_PRI_CH_SPEC>;
#[doc = "DMA_IN_PRI_CH%s_REG."]
pub mod in_pri_ch;
#[doc = "IN_PERI_SEL_CH (rw) register accessor: DMA_IN_PERI_SEL_CH%s_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_peri_sel_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_peri_sel_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_peri_sel_ch`] module"]
pub type IN_PERI_SEL_CH = crate::Reg<in_peri_sel_ch::IN_PERI_SEL_CH_SPEC>;
#[doc = "DMA_IN_PERI_SEL_CH%s_REG."]
pub mod in_peri_sel_ch;
#[doc = "OUT_CONF0_CH (rw) register accessor: DMA_OUT_CONF%s_CH%s_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf0_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf0_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf0_ch`] module"]
pub type OUT_CONF0_CH = crate::Reg<out_conf0_ch::OUT_CONF0_CH_SPEC>;
#[doc = "DMA_OUT_CONF%s_CH%s_REG."]
pub mod out_conf0_ch;
#[doc = "OUT_CONF1_CH (rw) register accessor: DMA_OUT_CONF1_CH%s_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf1_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf1_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf1_ch`] module"]
pub type OUT_CONF1_CH = crate::Reg<out_conf1_ch::OUT_CONF1_CH_SPEC>;
#[doc = "DMA_OUT_CONF1_CH%s_REG."]
pub mod out_conf1_ch;
#[doc = "OUTFIFO_STATUS_CH0 (r) register accessor: DMA_OUTFIFO_STATUS_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outfifo_status_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outfifo_status_ch0`] module"]
pub type OUTFIFO_STATUS_CH0 = crate::Reg<outfifo_status_ch0::OUTFIFO_STATUS_CH0_SPEC>;
#[doc = "DMA_OUTFIFO_STATUS_CH0_REG."]
pub mod outfifo_status_ch0;
#[doc = "OUT_PUSH_CH0 (rw) register accessor: DMA_OUT_PUSH_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_push_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_push_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_push_ch0`] module"]
pub type OUT_PUSH_CH0 = crate::Reg<out_push_ch0::OUT_PUSH_CH0_SPEC>;
#[doc = "DMA_OUT_PUSH_CH0_REG."]
pub mod out_push_ch0;
#[doc = "OUT_LINK_CH (rw) register accessor: DMA_OUT_LINK_CH%s_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_link_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link_ch`] module"]
pub type OUT_LINK_CH = crate::Reg<out_link_ch::OUT_LINK_CH_SPEC>;
#[doc = "DMA_OUT_LINK_CH%s_REG."]
pub mod out_link_ch;
#[doc = "OUT_STATE_CH0 (r) register accessor: DMA_OUT_STATE_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_state_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_state_ch0`] module"]
pub type OUT_STATE_CH0 = crate::Reg<out_state_ch0::OUT_STATE_CH0_SPEC>;
#[doc = "DMA_OUT_STATE_CH0_REG."]
pub mod out_state_ch0;
#[doc = "OUT_EOF_DES_ADDR_CH (r) register accessor: DMA_OUT_EOF_DES_ADDR_CH%s_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_eof_des_addr_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_des_addr_ch`] module"]
pub type OUT_EOF_DES_ADDR_CH = crate::Reg<out_eof_des_addr_ch::OUT_EOF_DES_ADDR_CH_SPEC>;
#[doc = "DMA_OUT_EOF_DES_ADDR_CH%s_REG."]
pub mod out_eof_des_addr_ch;
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
#[doc = "OUT_PRI_CH (rw) register accessor: DMA_OUT_PRI_CH%s_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_pri_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_pri_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_pri_ch`] module"]
pub type OUT_PRI_CH = crate::Reg<out_pri_ch::OUT_PRI_CH_SPEC>;
#[doc = "DMA_OUT_PRI_CH%s_REG."]
pub mod out_pri_ch;
#[doc = "OUT_PERI_SEL_CH (rw) register accessor: DMA_OUT_PERI_SEL_CH%s_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_peri_sel_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_peri_sel_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_peri_sel_ch`] module"]
pub type OUT_PERI_SEL_CH = crate::Reg<out_peri_sel_ch::OUT_PERI_SEL_CH_SPEC>;
#[doc = "DMA_OUT_PERI_SEL_CH%s_REG."]
pub mod out_peri_sel_ch;
