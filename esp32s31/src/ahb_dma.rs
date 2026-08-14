#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    in_int_ch: [IN_INT_CH; 5],
    out_int_ch: [OUT_INT_CH; 5],
    ahb_test: AHB_TEST,
    misc_conf: MISC_CONF,
    date: DATE,
    _reserved5: [u8; 0x54],
    ch: [CH; 5],
    intr_mem_start_addr: INTR_MEM_START_ADDR,
    intr_mem_end_addr: INTR_MEM_END_ADDR,
    arb_timeout: ARB_TIMEOUT,
    _reserved9: [u8; 0x04],
    weight_en: WEIGHT_EN,
    _reserved10: [u8; 0x04],
    module_clk_en: MODULE_CLK_EN,
    _reserved11: [u8; 0x04],
    ahbinf_resp_err_status0: AHBINF_RESP_ERR_STATUS0,
    ahbinf_resp_err_status1: AHBINF_RESP_ERR_STATUS1,
}
impl RegisterBlock {
    #[doc = "0x00..0x50 - Cluster IN_INT_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?"]
    #[inline(always)]
    pub const fn in_int_ch(&self, n: usize) -> &IN_INT_CH {
        &self.in_int_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x50 - Cluster IN_INT_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?"]
    #[inline(always)]
    pub fn in_int_ch_iter(&self) -> impl Iterator<Item = &IN_INT_CH> {
        self.in_int_ch.iter()
    }
    #[doc = "0x50..0xa0 - Cluster OUT_INT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?"]
    #[inline(always)]
    pub const fn out_int_ch(&self, n: usize) -> &OUT_INT_CH {
        &self.out_int_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0xa0 - Cluster OUT_INT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?"]
    #[inline(always)]
    pub fn out_int_ch_iter(&self) -> impl Iterator<Item = &OUT_INT_CH> {
        self.out_int_ch.iter()
    }
    #[doc = "0xa0 - reserved"]
    #[inline(always)]
    pub const fn ahb_test(&self) -> &AHB_TEST {
        &self.ahb_test
    }
    #[doc = "0xa4 - Miscellaneous register"]
    #[inline(always)]
    pub const fn misc_conf(&self) -> &MISC_CONF {
        &self.misc_conf
    }
    #[doc = "0xa8 - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    #[doc = "0x100..0x600 - Cluster CH%s, containing IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK_CH?, IN_LINK_ADDR_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DONE_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, RX_CH_ARB_WEIGH_CH?, RX_ARB_WEIGH_OPT_DIR_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CH?, OUT_LINK_ADDR_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DONE_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?, TX_CH_ARB_WEIGH_CH?, TX_ARB_WEIGH_OPT_DIR_CH?"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x600 - Cluster CH%s, containing IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK_CH?, IN_LINK_ADDR_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DONE_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, RX_CH_ARB_WEIGH_CH?, RX_ARB_WEIGH_OPT_DIR_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CH?, OUT_LINK_ADDR_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DONE_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?, TX_CH_ARB_WEIGH_CH?, TX_ARB_WEIGH_OPT_DIR_CH?"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
    #[doc = "0x600 - Accessible address space start address configuration register"]
    #[inline(always)]
    pub const fn intr_mem_start_addr(&self) -> &INTR_MEM_START_ADDR {
        &self.intr_mem_start_addr
    }
    #[doc = "0x604 - Accessible address space end address configuration register"]
    #[inline(always)]
    pub const fn intr_mem_end_addr(&self) -> &INTR_MEM_END_ADDR {
        &self.intr_mem_end_addr
    }
    #[doc = "0x608 - TX arbitration timeout configuration register"]
    #[inline(always)]
    pub const fn arb_timeout(&self) -> &ARB_TIMEOUT {
        &self.arb_timeout
    }
    #[doc = "0x610 - TX weight arbitration enable register"]
    #[inline(always)]
    pub const fn weight_en(&self) -> &WEIGHT_EN {
        &self.weight_en
    }
    #[doc = "0x618 - Module clock force on register"]
    #[inline(always)]
    pub const fn module_clk_en(&self) -> &MODULE_CLK_EN {
        &self.module_clk_en
    }
    #[doc = "0x620 - AHB response error status 0 register"]
    #[inline(always)]
    pub const fn ahbinf_resp_err_status0(&self) -> &AHBINF_RESP_ERR_STATUS0 {
        &self.ahbinf_resp_err_status0
    }
    #[doc = "0x624 - AHB response error status 1 register"]
    #[inline(always)]
    pub const fn ahbinf_resp_err_status1(&self) -> &AHBINF_RESP_ERR_STATUS1 {
        &self.ahbinf_resp_err_status1
    }
}
#[doc = "Cluster IN_INT_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?"]
pub use self::in_int_ch::IN_INT_CH;
#[doc = r"Cluster"]
#[doc = "Cluster IN_INT_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?"]
pub mod in_int_ch;
#[doc = "Cluster OUT_INT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?"]
pub use self::out_int_ch::OUT_INT_CH;
#[doc = r"Cluster"]
#[doc = "Cluster OUT_INT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?"]
pub mod out_int_ch;
#[doc = "AHB_TEST (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_test`] module"]
pub type AHB_TEST = crate::Reg<ahb_test::AHB_TEST_SPEC>;
#[doc = "reserved"]
pub mod ahb_test;
#[doc = "MISC_CONF (rw) register accessor: Miscellaneous register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_conf`] module"]
pub type MISC_CONF = crate::Reg<misc_conf::MISC_CONF_SPEC>;
#[doc = "Miscellaneous register"]
pub mod misc_conf;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
#[doc = "Cluster CH%s, containing IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK_CH?, IN_LINK_ADDR_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DONE_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, RX_CH_ARB_WEIGH_CH?, RX_ARB_WEIGH_OPT_DIR_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CH?, OUT_LINK_ADDR_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DONE_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?, TX_CH_ARB_WEIGH_CH?, TX_ARB_WEIGH_OPT_DIR_CH?"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "Cluster CH%s, containing IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK_CH?, IN_LINK_ADDR_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DONE_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, RX_CH_ARB_WEIGH_CH?, RX_ARB_WEIGH_OPT_DIR_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CH?, OUT_LINK_ADDR_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DONE_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?, TX_CH_ARB_WEIGH_CH?, TX_ARB_WEIGH_OPT_DIR_CH?"]
pub mod ch;
#[doc = "INTR_MEM_START_ADDR (rw) register accessor: Accessible address space start address configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mem_start_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mem_start_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mem_start_addr`] module"]
pub type INTR_MEM_START_ADDR = crate::Reg<intr_mem_start_addr::INTR_MEM_START_ADDR_SPEC>;
#[doc = "Accessible address space start address configuration register"]
pub mod intr_mem_start_addr;
#[doc = "INTR_MEM_END_ADDR (rw) register accessor: Accessible address space end address configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mem_end_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mem_end_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mem_end_addr`] module"]
pub type INTR_MEM_END_ADDR = crate::Reg<intr_mem_end_addr::INTR_MEM_END_ADDR_SPEC>;
#[doc = "Accessible address space end address configuration register"]
pub mod intr_mem_end_addr;
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
