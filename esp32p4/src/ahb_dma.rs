#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    in_int_ch: [IN_INT_CH; 3],
    out_int_ch: [OUT_INT_CH; 3],
    ahb_test: AHB_TEST,
    misc_conf: MISC_CONF,
    date: DATE,
    _reserved5: [u8; 0x04],
    ch: [CH; 3],
    _reserved6: [u8; 0x0c],
    out_crc_ch: [OUT_CRC_CH; 3],
    in_crc_ch: [IN_CRC_CH; 3],
    in_link_addr_ch: [IN_LINK_ADDR_CH; 3],
    out_link_addr_ch: [OUT_LINK_ADDR_CH; 3],
    intr_mem_start_addr: INTR_MEM_START_ADDR,
    intr_mem_end_addr: INTR_MEM_END_ADDR,
    arb_timeout_tx: ARB_TIMEOUT_TX,
    arb_timeout_rx: ARB_TIMEOUT_RX,
    weight_en_tx: WEIGHT_EN_TX,
    weight_en_rx: WEIGHT_EN_RX,
}
impl RegisterBlock {
    #[doc = "0x00..0x30 - Cluster IN_INT_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?"]
    #[inline(always)]
    pub const fn in_int_ch(&self, n: usize) -> &IN_INT_CH {
        &self.in_int_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x30 - Cluster IN_INT_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?"]
    #[inline(always)]
    pub fn in_int_ch_iter(&self) -> impl Iterator<Item = &IN_INT_CH> {
        self.in_int_ch.iter()
    }
    #[doc = "0x30..0x60 - Cluster OUT_INT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?"]
    #[inline(always)]
    pub const fn out_int_ch(&self, n: usize) -> &OUT_INT_CH {
        &self.out_int_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x60 - Cluster OUT_INT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?"]
    #[inline(always)]
    pub fn out_int_ch_iter(&self) -> impl Iterator<Item = &OUT_INT_CH> {
        self.out_int_ch.iter()
    }
    #[doc = "0x60 - reserved"]
    #[inline(always)]
    pub const fn ahb_test(&self) -> &AHB_TEST {
        &self.ahb_test
    }
    #[doc = "0x64 - MISC register"]
    #[inline(always)]
    pub const fn misc_conf(&self) -> &MISC_CONF {
        &self.misc_conf
    }
    #[doc = "0x68 - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    #[doc = "0x70..0x2b0 - Cluster CH%s, containing IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x2b0 - Cluster CH%s, containing IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
    #[doc = "0x2bc..0x334 - Cluster OUT_CRC_CH%s, containing OUT_CRC_INIT_DATA_CH?, TX_CRC_WIDTH_CH?, OUT_CRC_CLEAR_CH?, OUT_CRC_FINAL_RESULT_CH?, TX_CRC_EN_WR_DATA_CH?, TX_CRC_EN_ADDR_CH?, TX_CRC_DATA_EN_WR_DATA_CH?, TX_CRC_DATA_EN_ADDR_CH?, TX_CH_ARB_WEIGH_CH?, TX_ARB_WEIGH_OPT_DIR_CH?"]
    #[inline(always)]
    pub const fn out_crc_ch(&self, n: usize) -> &OUT_CRC_CH {
        &self.out_crc_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2bc..0x334 - Cluster OUT_CRC_CH%s, containing OUT_CRC_INIT_DATA_CH?, TX_CRC_WIDTH_CH?, OUT_CRC_CLEAR_CH?, OUT_CRC_FINAL_RESULT_CH?, TX_CRC_EN_WR_DATA_CH?, TX_CRC_EN_ADDR_CH?, TX_CRC_DATA_EN_WR_DATA_CH?, TX_CRC_DATA_EN_ADDR_CH?, TX_CH_ARB_WEIGH_CH?, TX_ARB_WEIGH_OPT_DIR_CH?"]
    #[inline(always)]
    pub fn out_crc_ch_iter(&self) -> impl Iterator<Item = &OUT_CRC_CH> {
        self.out_crc_ch.iter()
    }
    #[doc = "0x334..0x3ac - Cluster IN_CRC_CH%s, containing IN_CRC_INIT_DATA_CH?, RX_CRC_WIDTH_CH?, IN_CRC_CLEAR_CH?, IN_CRC_FINAL_RESULT_CH?, RX_CRC_EN_WR_DATA_CH?, RX_CRC_EN_ADDR_CH?, RX_CRC_DATA_EN_WR_DATA_CH?, RX_CRC_DATA_EN_ADDR_CH?, RX_CH_ARB_WEIGH_CH?, RX_ARB_WEIGH_OPT_DIR_CH?"]
    #[inline(always)]
    pub const fn in_crc_ch(&self, n: usize) -> &IN_CRC_CH {
        &self.in_crc_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x334..0x3ac - Cluster IN_CRC_CH%s, containing IN_CRC_INIT_DATA_CH?, RX_CRC_WIDTH_CH?, IN_CRC_CLEAR_CH?, IN_CRC_FINAL_RESULT_CH?, RX_CRC_EN_WR_DATA_CH?, RX_CRC_EN_ADDR_CH?, RX_CRC_DATA_EN_WR_DATA_CH?, RX_CRC_DATA_EN_ADDR_CH?, RX_CH_ARB_WEIGH_CH?, RX_ARB_WEIGH_OPT_DIR_CH?"]
    #[inline(always)]
    pub fn in_crc_ch_iter(&self) -> impl Iterator<Item = &IN_CRC_CH> {
        self.in_crc_ch.iter()
    }
    #[doc = "0x3ac..0x3b8 - Link descriptor configure of Rx channel 0"]
    #[inline(always)]
    pub const fn in_link_addr_ch(&self, n: usize) -> &IN_LINK_ADDR_CH {
        &self.in_link_addr_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3ac..0x3b8 - Link descriptor configure of Rx channel 0"]
    #[inline(always)]
    pub fn in_link_addr_ch_iter(&self) -> impl Iterator<Item = &IN_LINK_ADDR_CH> {
        self.in_link_addr_ch.iter()
    }
    #[doc = "0x3b8..0x3c4 - Link descriptor configure of Tx channel 0"]
    #[inline(always)]
    pub const fn out_link_addr_ch(&self, n: usize) -> &OUT_LINK_ADDR_CH {
        &self.out_link_addr_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3b8..0x3c4 - Link descriptor configure of Tx channel 0"]
    #[inline(always)]
    pub fn out_link_addr_ch_iter(&self) -> impl Iterator<Item = &OUT_LINK_ADDR_CH> {
        self.out_link_addr_ch.iter()
    }
    #[doc = "0x3c4 - The start address of accessible address space."]
    #[inline(always)]
    pub const fn intr_mem_start_addr(&self) -> &INTR_MEM_START_ADDR {
        &self.intr_mem_start_addr
    }
    #[doc = "0x3c8 - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
    #[inline(always)]
    pub const fn intr_mem_end_addr(&self) -> &INTR_MEM_END_ADDR {
        &self.intr_mem_end_addr
    }
    #[doc = "0x3cc - This retister is used to config arbiter time slice for tx dir"]
    #[inline(always)]
    pub const fn arb_timeout_tx(&self) -> &ARB_TIMEOUT_TX {
        &self.arb_timeout_tx
    }
    #[doc = "0x3d0 - This retister is used to config arbiter time slice for rx dir"]
    #[inline(always)]
    pub const fn arb_timeout_rx(&self) -> &ARB_TIMEOUT_RX {
        &self.arb_timeout_rx
    }
    #[doc = "0x3d4 - This register is used to config arbiter weigh function to on or off for tx dir"]
    #[inline(always)]
    pub const fn weight_en_tx(&self) -> &WEIGHT_EN_TX {
        &self.weight_en_tx
    }
    #[doc = "0x3d8 - This register is used to config arbiter weigh function to on or off for rx dir"]
    #[inline(always)]
    pub const fn weight_en_rx(&self) -> &WEIGHT_EN_RX {
        &self.weight_en_rx
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
#[doc = "MISC_CONF (rw) register accessor: MISC register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_conf`] module"]
pub type MISC_CONF = crate::Reg<misc_conf::MISC_CONF_SPEC>;
#[doc = "MISC register"]
pub mod misc_conf;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
#[doc = "Cluster CH%s, containing IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "Cluster CH%s, containing IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?"]
pub mod ch;
#[doc = "Cluster OUT_CRC_CH%s, containing OUT_CRC_INIT_DATA_CH?, TX_CRC_WIDTH_CH?, OUT_CRC_CLEAR_CH?, OUT_CRC_FINAL_RESULT_CH?, TX_CRC_EN_WR_DATA_CH?, TX_CRC_EN_ADDR_CH?, TX_CRC_DATA_EN_WR_DATA_CH?, TX_CRC_DATA_EN_ADDR_CH?, TX_CH_ARB_WEIGH_CH?, TX_ARB_WEIGH_OPT_DIR_CH?"]
pub use self::out_crc_ch::OUT_CRC_CH;
#[doc = r"Cluster"]
#[doc = "Cluster OUT_CRC_CH%s, containing OUT_CRC_INIT_DATA_CH?, TX_CRC_WIDTH_CH?, OUT_CRC_CLEAR_CH?, OUT_CRC_FINAL_RESULT_CH?, TX_CRC_EN_WR_DATA_CH?, TX_CRC_EN_ADDR_CH?, TX_CRC_DATA_EN_WR_DATA_CH?, TX_CRC_DATA_EN_ADDR_CH?, TX_CH_ARB_WEIGH_CH?, TX_ARB_WEIGH_OPT_DIR_CH?"]
pub mod out_crc_ch;
#[doc = "Cluster IN_CRC_CH%s, containing IN_CRC_INIT_DATA_CH?, RX_CRC_WIDTH_CH?, IN_CRC_CLEAR_CH?, IN_CRC_FINAL_RESULT_CH?, RX_CRC_EN_WR_DATA_CH?, RX_CRC_EN_ADDR_CH?, RX_CRC_DATA_EN_WR_DATA_CH?, RX_CRC_DATA_EN_ADDR_CH?, RX_CH_ARB_WEIGH_CH?, RX_ARB_WEIGH_OPT_DIR_CH?"]
pub use self::in_crc_ch::IN_CRC_CH;
#[doc = r"Cluster"]
#[doc = "Cluster IN_CRC_CH%s, containing IN_CRC_INIT_DATA_CH?, RX_CRC_WIDTH_CH?, IN_CRC_CLEAR_CH?, IN_CRC_FINAL_RESULT_CH?, RX_CRC_EN_WR_DATA_CH?, RX_CRC_EN_ADDR_CH?, RX_CRC_DATA_EN_WR_DATA_CH?, RX_CRC_DATA_EN_ADDR_CH?, RX_CH_ARB_WEIGH_CH?, RX_ARB_WEIGH_OPT_DIR_CH?"]
pub mod in_crc_ch;
#[doc = "IN_LINK_ADDR_CH (rw) register accessor: Link descriptor configure of Rx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_link_addr_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_link_addr_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link_addr_ch`] module"]
pub type IN_LINK_ADDR_CH = crate::Reg<in_link_addr_ch::IN_LINK_ADDR_CH_SPEC>;
#[doc = "Link descriptor configure of Rx channel 0"]
pub mod in_link_addr_ch;
#[doc = "OUT_LINK_ADDR_CH (rw) register accessor: Link descriptor configure of Tx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_link_addr_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_link_addr_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link_addr_ch`] module"]
pub type OUT_LINK_ADDR_CH = crate::Reg<out_link_addr_ch::OUT_LINK_ADDR_CH_SPEC>;
#[doc = "Link descriptor configure of Tx channel 0"]
pub mod out_link_addr_ch;
#[doc = "INTR_MEM_START_ADDR (rw) register accessor: The start address of accessible address space.\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mem_start_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mem_start_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mem_start_addr`] module"]
pub type INTR_MEM_START_ADDR = crate::Reg<intr_mem_start_addr::INTR_MEM_START_ADDR_SPEC>;
#[doc = "The start address of accessible address space."]
pub mod intr_mem_start_addr;
#[doc = "INTR_MEM_END_ADDR (rw) register accessor: The end address of accessible address space. The access address beyond this range would lead to descriptor error.\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mem_end_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mem_end_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mem_end_addr`] module"]
pub type INTR_MEM_END_ADDR = crate::Reg<intr_mem_end_addr::INTR_MEM_END_ADDR_SPEC>;
#[doc = "The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
pub mod intr_mem_end_addr;
#[doc = "ARB_TIMEOUT_TX (rw) register accessor: This retister is used to config arbiter time slice for tx dir\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_timeout_tx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_timeout_tx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_timeout_tx`] module"]
pub type ARB_TIMEOUT_TX = crate::Reg<arb_timeout_tx::ARB_TIMEOUT_TX_SPEC>;
#[doc = "This retister is used to config arbiter time slice for tx dir"]
pub mod arb_timeout_tx;
#[doc = "ARB_TIMEOUT_RX (rw) register accessor: This retister is used to config arbiter time slice for rx dir\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_timeout_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_timeout_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_timeout_rx`] module"]
pub type ARB_TIMEOUT_RX = crate::Reg<arb_timeout_rx::ARB_TIMEOUT_RX_SPEC>;
#[doc = "This retister is used to config arbiter time slice for rx dir"]
pub mod arb_timeout_rx;
#[doc = "WEIGHT_EN_TX (rw) register accessor: This register is used to config arbiter weigh function to on or off for tx dir\n\nYou can [`read`](crate::Reg::read) this register and get [`weight_en_tx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`weight_en_tx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@weight_en_tx`] module"]
pub type WEIGHT_EN_TX = crate::Reg<weight_en_tx::WEIGHT_EN_TX_SPEC>;
#[doc = "This register is used to config arbiter weigh function to on or off for tx dir"]
pub mod weight_en_tx;
#[doc = "WEIGHT_EN_RX (rw) register accessor: This register is used to config arbiter weigh function to on or off for rx dir\n\nYou can [`read`](crate::Reg::read) this register and get [`weight_en_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`weight_en_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@weight_en_rx`] module"]
pub type WEIGHT_EN_RX = crate::Reg<weight_en_rx::WEIGHT_EN_RX_SPEC>;
#[doc = "This register is used to config arbiter weigh function to on or off for rx dir"]
pub mod weight_en_rx;
