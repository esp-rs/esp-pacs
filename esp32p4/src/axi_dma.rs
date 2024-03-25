#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    in_ch: [IN_CH; 3],
    out_ch: [OUT_CH; 3],
    arb_timeout: ARB_TIMEOUT,
    weight_en: WEIGHT_EN,
    in_mem_conf: IN_MEM_CONF,
    intr_mem_start_addr: INTR_MEM_START_ADDR,
    intr_mem_end_addr: INTR_MEM_END_ADDR,
    extr_mem_start_addr: EXTR_MEM_START_ADDR,
    extr_mem_end_addr: EXTR_MEM_END_ADDR,
    in_reset_avail_ch: [IN_RESET_AVAIL_CH; 3],
    out_reset_avail_ch: [OUT_RESET_AVAIL_CH; 3],
    _reserved11: [u8; 0x04],
    misc_conf: MISC_CONF,
    rdn_result: RDN_RESULT,
    rdn_eco_high: RDN_ECO_HIGH,
    rdn_eco_low: RDN_ECO_LOW,
    wresp_cnt: WRESP_CNT,
    rresp_cnt: RRESP_CNT,
    infifo_status1_ch: [INFIFO_STATUS1_CH; 3],
    outfifo_status1_ch: [OUTFIFO_STATUS1_CH; 3],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x138 - Cluster IN_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?, IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK1_CH?, IN_LINK2_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, IN_CRC_INIT_DATA_CH?, RX_CRC_WIDTH_CH?, IN_CRC_CLEAR_CH?, IN_CRC_FINAL_RESULT_CH?, RX_CRC_EN_WR_DATA_CH?, RX_CRC_EN_ADDR_CH?, RX_CRC_DATA_EN_WR_DATA_CH?, RX_CRC_DATA_EN_ADDR_CH?"]
    #[inline(always)]
    pub const fn in_ch(&self, n: usize) -> &IN_CH {
        &self.in_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x138 - Cluster IN_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?, IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK1_CH?, IN_LINK2_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, IN_CRC_INIT_DATA_CH?, RX_CRC_WIDTH_CH?, IN_CRC_CLEAR_CH?, IN_CRC_FINAL_RESULT_CH?, RX_CRC_EN_WR_DATA_CH?, RX_CRC_EN_ADDR_CH?, RX_CRC_DATA_EN_WR_DATA_CH?, RX_CRC_DATA_EN_ADDR_CH?"]
    #[inline(always)]
    pub fn in_ch_iter(&self) -> impl Iterator<Item = &IN_CH> {
        self.in_ch.iter()
    }
    #[doc = "0x138..0x270 - Cluster OUT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK1_CH?, OUT_LINK2_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?, OUT_CRC_INIT_DATA_CH?, TX_CRC_WIDTH_CH?, OUT_CRC_CLEAR_CH?, OUT_CRC_FINAL_RESULT_CH?, TX_CRC_EN_WR_DATA_CH?, TX_CRC_EN_ADDR_CH?, TX_CRC_DATA_EN_WR_DATA_CH?, TX_CRC_DATA_EN_ADDR_CH?"]
    #[inline(always)]
    pub const fn out_ch(&self, n: usize) -> &OUT_CH {
        &self.out_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x138..0x270 - Cluster OUT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK1_CH?, OUT_LINK2_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?, OUT_CRC_INIT_DATA_CH?, TX_CRC_WIDTH_CH?, OUT_CRC_CLEAR_CH?, OUT_CRC_FINAL_RESULT_CH?, TX_CRC_EN_WR_DATA_CH?, TX_CRC_EN_ADDR_CH?, TX_CRC_DATA_EN_WR_DATA_CH?, TX_CRC_DATA_EN_ADDR_CH?"]
    #[inline(always)]
    pub fn out_ch_iter(&self) -> impl Iterator<Item = &OUT_CH> {
        self.out_ch.iter()
    }
    #[doc = "0x270 - This retister is used to config arbiter time slice"]
    #[inline(always)]
    pub const fn arb_timeout(&self) -> &ARB_TIMEOUT {
        &self.arb_timeout
    }
    #[doc = "0x274 - This register is used to config arbiter weight function to on or off"]
    #[inline(always)]
    pub const fn weight_en(&self) -> &WEIGHT_EN {
        &self.weight_en
    }
    #[doc = "0x278 - Mem power configure register of Rx channel"]
    #[inline(always)]
    pub const fn in_mem_conf(&self) -> &IN_MEM_CONF {
        &self.in_mem_conf
    }
    #[doc = "0x27c - The start address of accessible address space."]
    #[inline(always)]
    pub const fn intr_mem_start_addr(&self) -> &INTR_MEM_START_ADDR {
        &self.intr_mem_start_addr
    }
    #[doc = "0x280 - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
    #[inline(always)]
    pub const fn intr_mem_end_addr(&self) -> &INTR_MEM_END_ADDR {
        &self.intr_mem_end_addr
    }
    #[doc = "0x284 - The start address of accessible address space."]
    #[inline(always)]
    pub const fn extr_mem_start_addr(&self) -> &EXTR_MEM_START_ADDR {
        &self.extr_mem_start_addr
    }
    #[doc = "0x288 - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
    #[inline(always)]
    pub const fn extr_mem_end_addr(&self) -> &EXTR_MEM_END_ADDR {
        &self.extr_mem_end_addr
    }
    #[doc = "0x28c..0x298 - The rx channel 0 reset valid_flag register."]
    #[inline(always)]
    pub const fn in_reset_avail_ch(&self, n: usize) -> &IN_RESET_AVAIL_CH {
        &self.in_reset_avail_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x28c..0x298 - The rx channel 0 reset valid_flag register."]
    #[inline(always)]
    pub fn in_reset_avail_ch_iter(&self) -> impl Iterator<Item = &IN_RESET_AVAIL_CH> {
        self.in_reset_avail_ch.iter()
    }
    #[doc = "0x298..0x2a4 - The tx channel 0 reset valid_flag register."]
    #[inline(always)]
    pub const fn out_reset_avail_ch(&self, n: usize) -> &OUT_RESET_AVAIL_CH {
        &self.out_reset_avail_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x298..0x2a4 - The tx channel 0 reset valid_flag register."]
    #[inline(always)]
    pub fn out_reset_avail_ch_iter(&self) -> impl Iterator<Item = &OUT_RESET_AVAIL_CH> {
        self.out_reset_avail_ch.iter()
    }
    #[doc = "0x2a8 - MISC register"]
    #[inline(always)]
    pub const fn misc_conf(&self) -> &MISC_CONF {
        &self.misc_conf
    }
    #[doc = "0x2ac - reserved"]
    #[inline(always)]
    pub const fn rdn_result(&self) -> &RDN_RESULT {
        &self.rdn_result
    }
    #[doc = "0x2b0 - reserved"]
    #[inline(always)]
    pub const fn rdn_eco_high(&self) -> &RDN_ECO_HIGH {
        &self.rdn_eco_high
    }
    #[doc = "0x2b4 - reserved"]
    #[inline(always)]
    pub const fn rdn_eco_low(&self) -> &RDN_ECO_LOW {
        &self.rdn_eco_low
    }
    #[doc = "0x2b8 - AXI wr responce cnt register."]
    #[inline(always)]
    pub const fn wresp_cnt(&self) -> &WRESP_CNT {
        &self.wresp_cnt
    }
    #[doc = "0x2bc - AXI wr responce cnt register."]
    #[inline(always)]
    pub const fn rresp_cnt(&self) -> &RRESP_CNT {
        &self.rresp_cnt
    }
    #[doc = "0x2c0..0x2cc - Receive FIFO status of Rx channel 0"]
    #[inline(always)]
    pub const fn infifo_status1_ch(&self, n: usize) -> &INFIFO_STATUS1_CH {
        &self.infifo_status1_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2c0..0x2cc - Receive FIFO status of Rx channel 0"]
    #[inline(always)]
    pub fn infifo_status1_ch_iter(&self) -> impl Iterator<Item = &INFIFO_STATUS1_CH> {
        self.infifo_status1_ch.iter()
    }
    #[doc = "0x2cc..0x2d8 - Receive FIFO status of Tx channel 0"]
    #[inline(always)]
    pub const fn outfifo_status1_ch(&self, n: usize) -> &OUTFIFO_STATUS1_CH {
        &self.outfifo_status1_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2cc..0x2d8 - Receive FIFO status of Tx channel 0"]
    #[inline(always)]
    pub fn outfifo_status1_ch_iter(&self) -> impl Iterator<Item = &OUTFIFO_STATUS1_CH> {
        self.outfifo_status1_ch.iter()
    }
    #[doc = "0x2d8 - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "Cluster IN_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?, IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK1_CH?, IN_LINK2_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, IN_CRC_INIT_DATA_CH?, RX_CRC_WIDTH_CH?, IN_CRC_CLEAR_CH?, IN_CRC_FINAL_RESULT_CH?, RX_CRC_EN_WR_DATA_CH?, RX_CRC_EN_ADDR_CH?, RX_CRC_DATA_EN_WR_DATA_CH?, RX_CRC_DATA_EN_ADDR_CH?"]
pub use self::in_ch::IN_CH;
#[doc = r"Cluster"]
#[doc = "Cluster IN_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?, IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK1_CH?, IN_LINK2_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, IN_CRC_INIT_DATA_CH?, RX_CRC_WIDTH_CH?, IN_CRC_CLEAR_CH?, IN_CRC_FINAL_RESULT_CH?, RX_CRC_EN_WR_DATA_CH?, RX_CRC_EN_ADDR_CH?, RX_CRC_DATA_EN_WR_DATA_CH?, RX_CRC_DATA_EN_ADDR_CH?"]
pub mod in_ch;
#[doc = "Cluster OUT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK1_CH?, OUT_LINK2_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?, OUT_CRC_INIT_DATA_CH?, TX_CRC_WIDTH_CH?, OUT_CRC_CLEAR_CH?, OUT_CRC_FINAL_RESULT_CH?, TX_CRC_EN_WR_DATA_CH?, TX_CRC_EN_ADDR_CH?, TX_CRC_DATA_EN_WR_DATA_CH?, TX_CRC_DATA_EN_ADDR_CH?"]
pub use self::out_ch::OUT_CH;
#[doc = r"Cluster"]
#[doc = "Cluster OUT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK1_CH?, OUT_LINK2_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?, OUT_CRC_INIT_DATA_CH?, TX_CRC_WIDTH_CH?, OUT_CRC_CLEAR_CH?, OUT_CRC_FINAL_RESULT_CH?, TX_CRC_EN_WR_DATA_CH?, TX_CRC_EN_ADDR_CH?, TX_CRC_DATA_EN_WR_DATA_CH?, TX_CRC_DATA_EN_ADDR_CH?"]
pub mod out_ch;
#[doc = "ARB_TIMEOUT (rw) register accessor: This retister is used to config arbiter time slice\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_timeout`] module"]
pub type ARB_TIMEOUT = crate::Reg<arb_timeout::ARB_TIMEOUT_SPEC>;
#[doc = "This retister is used to config arbiter time slice"]
pub mod arb_timeout;
#[doc = "WEIGHT_EN (rw) register accessor: This register is used to config arbiter weight function to on or off\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`weight_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`weight_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@weight_en`] module"]
pub type WEIGHT_EN = crate::Reg<weight_en::WEIGHT_EN_SPEC>;
#[doc = "This register is used to config arbiter weight function to on or off"]
pub mod weight_en;
#[doc = "IN_MEM_CONF (rw) register accessor: Mem power configure register of Rx channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_mem_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_mem_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_mem_conf`] module"]
pub type IN_MEM_CONF = crate::Reg<in_mem_conf::IN_MEM_CONF_SPEC>;
#[doc = "Mem power configure register of Rx channel"]
pub mod in_mem_conf;
#[doc = "INTR_MEM_START_ADDR (rw) register accessor: The start address of accessible address space.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_mem_start_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_mem_start_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mem_start_addr`] module"]
pub type INTR_MEM_START_ADDR = crate::Reg<intr_mem_start_addr::INTR_MEM_START_ADDR_SPEC>;
#[doc = "The start address of accessible address space."]
pub mod intr_mem_start_addr;
#[doc = "INTR_MEM_END_ADDR (rw) register accessor: The end address of accessible address space. The access address beyond this range would lead to descriptor error.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_mem_end_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_mem_end_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mem_end_addr`] module"]
pub type INTR_MEM_END_ADDR = crate::Reg<intr_mem_end_addr::INTR_MEM_END_ADDR_SPEC>;
#[doc = "The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
pub mod intr_mem_end_addr;
#[doc = "EXTR_MEM_START_ADDR (rw) register accessor: The start address of accessible address space.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extr_mem_start_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extr_mem_start_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extr_mem_start_addr`] module"]
pub type EXTR_MEM_START_ADDR = crate::Reg<extr_mem_start_addr::EXTR_MEM_START_ADDR_SPEC>;
#[doc = "The start address of accessible address space."]
pub mod extr_mem_start_addr;
#[doc = "EXTR_MEM_END_ADDR (rw) register accessor: The end address of accessible address space. The access address beyond this range would lead to descriptor error.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extr_mem_end_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extr_mem_end_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extr_mem_end_addr`] module"]
pub type EXTR_MEM_END_ADDR = crate::Reg<extr_mem_end_addr::EXTR_MEM_END_ADDR_SPEC>;
#[doc = "The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
pub mod extr_mem_end_addr;
#[doc = "IN_RESET_AVAIL_CH (r) register accessor: The rx channel 0 reset valid_flag register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_reset_avail_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_reset_avail_ch`] module"]
pub type IN_RESET_AVAIL_CH = crate::Reg<in_reset_avail_ch::IN_RESET_AVAIL_CH_SPEC>;
#[doc = "The rx channel 0 reset valid_flag register."]
pub mod in_reset_avail_ch;
#[doc = "OUT_RESET_AVAIL_CH (r) register accessor: The tx channel 0 reset valid_flag register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_reset_avail_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_reset_avail_ch`] module"]
pub type OUT_RESET_AVAIL_CH = crate::Reg<out_reset_avail_ch::OUT_RESET_AVAIL_CH_SPEC>;
#[doc = "The tx channel 0 reset valid_flag register."]
pub mod out_reset_avail_ch;
#[doc = "MISC_CONF (rw) register accessor: MISC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_conf`] module"]
pub type MISC_CONF = crate::Reg<misc_conf::MISC_CONF_SPEC>;
#[doc = "MISC register"]
pub mod misc_conf;
#[doc = "RDN_RESULT (rw) register accessor: reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdn_result::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdn_result::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdn_result`] module"]
pub type RDN_RESULT = crate::Reg<rdn_result::RDN_RESULT_SPEC>;
#[doc = "reserved"]
pub mod rdn_result;
#[doc = "RDN_ECO_HIGH (rw) register accessor: reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdn_eco_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdn_eco_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdn_eco_high`] module"]
pub type RDN_ECO_HIGH = crate::Reg<rdn_eco_high::RDN_ECO_HIGH_SPEC>;
#[doc = "reserved"]
pub mod rdn_eco_high;
#[doc = "RDN_ECO_LOW (rw) register accessor: reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdn_eco_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdn_eco_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdn_eco_low`] module"]
pub type RDN_ECO_LOW = crate::Reg<rdn_eco_low::RDN_ECO_LOW_SPEC>;
#[doc = "reserved"]
pub mod rdn_eco_low;
#[doc = "WRESP_CNT (r) register accessor: AXI wr responce cnt register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wresp_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wresp_cnt`] module"]
pub type WRESP_CNT = crate::Reg<wresp_cnt::WRESP_CNT_SPEC>;
#[doc = "AXI wr responce cnt register."]
pub mod wresp_cnt;
#[doc = "RRESP_CNT (r) register accessor: AXI wr responce cnt register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rresp_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rresp_cnt`] module"]
pub type RRESP_CNT = crate::Reg<rresp_cnt::RRESP_CNT_SPEC>;
#[doc = "AXI wr responce cnt register."]
pub mod rresp_cnt;
#[doc = "INFIFO_STATUS1_CH (r) register accessor: Receive FIFO status of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`infifo_status1_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@infifo_status1_ch`] module"]
pub type INFIFO_STATUS1_CH = crate::Reg<infifo_status1_ch::INFIFO_STATUS1_CH_SPEC>;
#[doc = "Receive FIFO status of Rx channel 0"]
pub mod infifo_status1_ch;
#[doc = "OUTFIFO_STATUS1_CH (r) register accessor: Receive FIFO status of Tx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outfifo_status1_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outfifo_status1_ch`] module"]
pub type OUTFIFO_STATUS1_CH = crate::Reg<outfifo_status1_ch::OUTFIFO_STATUS1_CH_SPEC>;
#[doc = "Receive FIFO status of Tx channel 0"]
pub mod outfifo_status1_ch;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
