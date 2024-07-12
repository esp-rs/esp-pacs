#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    out_ch: [OUT_CH; 5],
    in_ch: [IN_CH; 5],
    in_ch5: IN_CH5,
    _reserved3: [u8; 0x70],
    inter_axi_err: INTER_AXI_ERR,
    exter_axi_err: EXTER_AXI_ERR,
    rst_conf: RST_CONF,
    inter_mem_start_addr0: INTER_MEM_START_ADDR0,
    inter_mem_end_addr0: INTER_MEM_END_ADDR0,
    inter_mem_start_addr1: INTER_MEM_START_ADDR1,
    inter_mem_end_addr1: INTER_MEM_END_ADDR1,
    _reserved10: [u8; 0x04],
    exter_mem_start_addr0: EXTER_MEM_START_ADDR0,
    exter_mem_end_addr0: EXTER_MEM_END_ADDR0,
    exter_mem_start_addr1: EXTER_MEM_START_ADDR1,
    exter_mem_end_addr1: EXTER_MEM_END_ADDR1,
    out_arb_config: OUT_ARB_CONFIG,
    in_arb_config: IN_ARB_CONFIG,
    _reserved16: [u8; 0x04],
    date: DATE,
    _reserved17: [u8; 0x10],
    counter_rst: COUNTER_RST,
    rx_ch0_counter: RX_CH0_COUNTER,
    rx_ch1_counter: RX_CH1_COUNTER,
    rx_ch2_counter: RX_CH2_COUNTER,
    rx_ch5_counter: RX_CH5_COUNTER,
}
impl RegisterBlock {
    #[doc = "0x00..0x500 - Cluster OUT_CH%s, containing OUT_CONF0_CH?, OUT_INT_RAW_CH?, OUT_INT_ENA_CH?, OUT_INT_ST_CH?, OUT_INT_CLR_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CONF_CH?, OUT_LINK_ADDR_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_ARB_CH?, OUT_RO_STATUS_CH?, OUT_RO_PD_CONF_CH?, OUT_MODE_ENABLE_CH?, OUT_MODE_YUV_CH?, OUT_ETM_CONF_CH?, OUT_BUF_LEN_CH?, OUT_FIFO_BCNT_CH?, OUT_PUSH_BYTECNT_CH?, OUT_XADDR_CH?, OUT_BLOCK_BUF_LEN_CH?"]
    #[inline(always)]
    pub const fn out_ch(&self, n: usize) -> &OUT_CH {
        &self.out_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x500 - Cluster OUT_CH%s, containing OUT_CONF0_CH?, OUT_INT_RAW_CH?, OUT_INT_ENA_CH?, OUT_INT_ST_CH?, OUT_INT_CLR_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CONF_CH?, OUT_LINK_ADDR_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_ARB_CH?, OUT_RO_STATUS_CH?, OUT_RO_PD_CONF_CH?, OUT_MODE_ENABLE_CH?, OUT_MODE_YUV_CH?, OUT_ETM_CONF_CH?, OUT_BUF_LEN_CH?, OUT_FIFO_BCNT_CH?, OUT_PUSH_BYTECNT_CH?, OUT_XADDR_CH?, OUT_BLOCK_BUF_LEN_CH?"]
    #[inline(always)]
    pub fn out_ch_iter(&self) -> impl Iterator<Item = &OUT_CH> {
        self.out_ch.iter()
    }
    #[doc = "0x500..0xa00 - Cluster IN_CH%s, containing IN_CONF0_CH\\[0-4\\], IN_INT_RAW_CH\\[0-4\\], IN_INT_ENA_CH\\[0-4\\], IN_INT_ST_CH\\[0-4\\], IN_INT_CLR_CH\\[0-4\\], INFIFO_STATUS_CH\\[0-4\\], IN_POP_CH\\[0-4\\], IN_LINK_CONF_CH\\[0-4\\], IN_LINK_ADDR_CH\\[0-4\\], IN_STATE_CH\\[0-4\\], IN_SUC_EOF_DES_ADDR_CH\\[0-4\\], IN_ERR_EOF_DES_ADDR_CH\\[0-4\\], IN_DSCR_CH\\[0-4\\], IN_DSCR_BF0_CH\\[0-4\\], IN_DSCR_BF1_CH\\[0-4\\], IN_ARB_CH\\[0-4\\], IN_RO_PD_CONF_CH\\[0-4\\], IN_ETM_CONF_CH\\[0-4\\], IN_FIFO_CNT_CH\\[0-4\\], IN_POP_DATA_CNT_CH\\[0-4\\], IN_XADDR_CH\\[0-4\\], IN_BUF_HB_RCV_CH\\[0-4\\]"]
    #[inline(always)]
    pub const fn in_ch(&self, n: usize) -> &IN_CH {
        &self.in_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x500..0xa00 - Cluster IN_CH%s, containing IN_CONF0_CH\\[0-4\\], IN_INT_RAW_CH\\[0-4\\], IN_INT_ENA_CH\\[0-4\\], IN_INT_ST_CH\\[0-4\\], IN_INT_CLR_CH\\[0-4\\], INFIFO_STATUS_CH\\[0-4\\], IN_POP_CH\\[0-4\\], IN_LINK_CONF_CH\\[0-4\\], IN_LINK_ADDR_CH\\[0-4\\], IN_STATE_CH\\[0-4\\], IN_SUC_EOF_DES_ADDR_CH\\[0-4\\], IN_ERR_EOF_DES_ADDR_CH\\[0-4\\], IN_DSCR_CH\\[0-4\\], IN_DSCR_BF0_CH\\[0-4\\], IN_DSCR_BF1_CH\\[0-4\\], IN_ARB_CH\\[0-4\\], IN_RO_PD_CONF_CH\\[0-4\\], IN_ETM_CONF_CH\\[0-4\\], IN_FIFO_CNT_CH\\[0-4\\], IN_POP_DATA_CNT_CH\\[0-4\\], IN_XADDR_CH\\[0-4\\], IN_BUF_HB_RCV_CH\\[0-4\\]"]
    #[inline(always)]
    pub fn in_ch_iter(&self) -> impl Iterator<Item = &IN_CH> {
        self.in_ch.iter()
    }
    #[doc = "0xa00..0xa90 - Cluster IN_CH5, containing IN_CONF0_CH5, IN_CONF1_CH5, IN_CONF2_CH5, IN_CONF3_CH5, IN_INT_RAW_CH5, IN_INT_ENA_CH5, IN_INT_ST_CH5, IN_INT_CLR_CH5, INFIFO_STATUS_CH5, IN_POP_CH5, IN_STATE_CH5, IN_ARB_CH5, IN_FIFO_CNT_CH5, IN_POP_DATA_CNT_CH5, IN_XADDR_CH5, IN_BUF_HB_RCV_CH5"]
    #[inline(always)]
    pub const fn in_ch5(&self) -> &IN_CH5 {
        &self.in_ch5
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
#[doc = "Cluster OUT_CH%s, containing OUT_CONF0_CH?, OUT_INT_RAW_CH?, OUT_INT_ENA_CH?, OUT_INT_ST_CH?, OUT_INT_CLR_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CONF_CH?, OUT_LINK_ADDR_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_ARB_CH?, OUT_RO_STATUS_CH?, OUT_RO_PD_CONF_CH?, OUT_MODE_ENABLE_CH?, OUT_MODE_YUV_CH?, OUT_ETM_CONF_CH?, OUT_BUF_LEN_CH?, OUT_FIFO_BCNT_CH?, OUT_PUSH_BYTECNT_CH?, OUT_XADDR_CH?, OUT_BLOCK_BUF_LEN_CH?"]
pub use self::out_ch::OUT_CH;
#[doc = r"Cluster"]
#[doc = "Cluster OUT_CH%s, containing OUT_CONF0_CH?, OUT_INT_RAW_CH?, OUT_INT_ENA_CH?, OUT_INT_ST_CH?, OUT_INT_CLR_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CONF_CH?, OUT_LINK_ADDR_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_ARB_CH?, OUT_RO_STATUS_CH?, OUT_RO_PD_CONF_CH?, OUT_MODE_ENABLE_CH?, OUT_MODE_YUV_CH?, OUT_ETM_CONF_CH?, OUT_BUF_LEN_CH?, OUT_FIFO_BCNT_CH?, OUT_PUSH_BYTECNT_CH?, OUT_XADDR_CH?, OUT_BLOCK_BUF_LEN_CH?"]
pub mod out_ch;
#[doc = "Cluster IN_CH%s, containing IN_CONF0_CH\\[0-4\\], IN_INT_RAW_CH\\[0-4\\], IN_INT_ENA_CH\\[0-4\\], IN_INT_ST_CH\\[0-4\\], IN_INT_CLR_CH\\[0-4\\], INFIFO_STATUS_CH\\[0-4\\], IN_POP_CH\\[0-4\\], IN_LINK_CONF_CH\\[0-4\\], IN_LINK_ADDR_CH\\[0-4\\], IN_STATE_CH\\[0-4\\], IN_SUC_EOF_DES_ADDR_CH\\[0-4\\], IN_ERR_EOF_DES_ADDR_CH\\[0-4\\], IN_DSCR_CH\\[0-4\\], IN_DSCR_BF0_CH\\[0-4\\], IN_DSCR_BF1_CH\\[0-4\\], IN_ARB_CH\\[0-4\\], IN_RO_PD_CONF_CH\\[0-4\\], IN_ETM_CONF_CH\\[0-4\\], IN_FIFO_CNT_CH\\[0-4\\], IN_POP_DATA_CNT_CH\\[0-4\\], IN_XADDR_CH\\[0-4\\], IN_BUF_HB_RCV_CH\\[0-4\\]"]
pub use self::in_ch::IN_CH;
#[doc = r"Cluster"]
#[doc = "Cluster IN_CH%s, containing IN_CONF0_CH\\[0-4\\], IN_INT_RAW_CH\\[0-4\\], IN_INT_ENA_CH\\[0-4\\], IN_INT_ST_CH\\[0-4\\], IN_INT_CLR_CH\\[0-4\\], INFIFO_STATUS_CH\\[0-4\\], IN_POP_CH\\[0-4\\], IN_LINK_CONF_CH\\[0-4\\], IN_LINK_ADDR_CH\\[0-4\\], IN_STATE_CH\\[0-4\\], IN_SUC_EOF_DES_ADDR_CH\\[0-4\\], IN_ERR_EOF_DES_ADDR_CH\\[0-4\\], IN_DSCR_CH\\[0-4\\], IN_DSCR_BF0_CH\\[0-4\\], IN_DSCR_BF1_CH\\[0-4\\], IN_ARB_CH\\[0-4\\], IN_RO_PD_CONF_CH\\[0-4\\], IN_ETM_CONF_CH\\[0-4\\], IN_FIFO_CNT_CH\\[0-4\\], IN_POP_DATA_CNT_CH\\[0-4\\], IN_XADDR_CH\\[0-4\\], IN_BUF_HB_RCV_CH\\[0-4\\]"]
pub mod in_ch;
#[doc = "Cluster IN_CH5, containing IN_CONF0_CH5, IN_CONF1_CH5, IN_CONF2_CH5, IN_CONF3_CH5, IN_INT_RAW_CH5, IN_INT_ENA_CH5, IN_INT_ST_CH5, IN_INT_CLR_CH5, INFIFO_STATUS_CH5, IN_POP_CH5, IN_STATE_CH5, IN_ARB_CH5, IN_FIFO_CNT_CH5, IN_POP_DATA_CNT_CH5, IN_XADDR_CH5, IN_BUF_HB_RCV_CH5"]
pub use self::in_ch5::IN_CH5;
#[doc = r"Cluster"]
#[doc = "Cluster IN_CH5, containing IN_CONF0_CH5, IN_CONF1_CH5, IN_CONF2_CH5, IN_CONF3_CH5, IN_INT_RAW_CH5, IN_INT_ENA_CH5, IN_INT_ST_CH5, IN_INT_CLR_CH5, INFIFO_STATUS_CH5, IN_POP_CH5, IN_STATE_CH5, IN_ARB_CH5, IN_FIFO_CNT_CH5, IN_POP_DATA_CNT_CH5, IN_XADDR_CH5, IN_BUF_HB_RCV_CH5"]
pub mod in_ch5;
#[doc = "INTER_AXI_ERR (r) register accessor: inter memory axi err register\n\nYou can [`read`](crate::Reg::read) this register and get [`inter_axi_err::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inter_axi_err`] module"]
pub type INTER_AXI_ERR = crate::Reg<inter_axi_err::INTER_AXI_ERR_SPEC>;
#[doc = "inter memory axi err register"]
pub mod inter_axi_err;
#[doc = "EXTER_AXI_ERR (r) register accessor: exter memory axi err register\n\nYou can [`read`](crate::Reg::read) this register and get [`exter_axi_err::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exter_axi_err`] module"]
pub type EXTER_AXI_ERR = crate::Reg<exter_axi_err::EXTER_AXI_ERR_SPEC>;
#[doc = "exter memory axi err register"]
pub mod exter_axi_err;
#[doc = "RST_CONF (rw) register accessor: axi reset config register\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_conf`] module"]
pub type RST_CONF = crate::Reg<rst_conf::RST_CONF_SPEC>;
#[doc = "axi reset config register"]
pub mod rst_conf;
#[doc = "INTER_MEM_START_ADDR0 (rw) register accessor: Start address of inter memory range0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`inter_mem_start_addr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inter_mem_start_addr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inter_mem_start_addr0`] module"]
pub type INTER_MEM_START_ADDR0 = crate::Reg<inter_mem_start_addr0::INTER_MEM_START_ADDR0_SPEC>;
#[doc = "Start address of inter memory range0 register"]
pub mod inter_mem_start_addr0;
#[doc = "INTER_MEM_END_ADDR0 (rw) register accessor: end address of inter memory range0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`inter_mem_end_addr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inter_mem_end_addr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inter_mem_end_addr0`] module"]
pub type INTER_MEM_END_ADDR0 = crate::Reg<inter_mem_end_addr0::INTER_MEM_END_ADDR0_SPEC>;
#[doc = "end address of inter memory range0 register"]
pub mod inter_mem_end_addr0;
#[doc = "INTER_MEM_START_ADDR1 (rw) register accessor: Start address of inter memory range1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`inter_mem_start_addr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inter_mem_start_addr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inter_mem_start_addr1`] module"]
pub type INTER_MEM_START_ADDR1 = crate::Reg<inter_mem_start_addr1::INTER_MEM_START_ADDR1_SPEC>;
#[doc = "Start address of inter memory range1 register"]
pub mod inter_mem_start_addr1;
#[doc = "INTER_MEM_END_ADDR1 (rw) register accessor: end address of inter memory range1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`inter_mem_end_addr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inter_mem_end_addr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inter_mem_end_addr1`] module"]
pub type INTER_MEM_END_ADDR1 = crate::Reg<inter_mem_end_addr1::INTER_MEM_END_ADDR1_SPEC>;
#[doc = "end address of inter memory range1 register"]
pub mod inter_mem_end_addr1;
#[doc = "EXTER_MEM_START_ADDR0 (rw) register accessor: Start address of exter memory range0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`exter_mem_start_addr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exter_mem_start_addr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exter_mem_start_addr0`] module"]
pub type EXTER_MEM_START_ADDR0 = crate::Reg<exter_mem_start_addr0::EXTER_MEM_START_ADDR0_SPEC>;
#[doc = "Start address of exter memory range0 register"]
pub mod exter_mem_start_addr0;
#[doc = "EXTER_MEM_END_ADDR0 (rw) register accessor: end address of exter memory range0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`exter_mem_end_addr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exter_mem_end_addr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exter_mem_end_addr0`] module"]
pub type EXTER_MEM_END_ADDR0 = crate::Reg<exter_mem_end_addr0::EXTER_MEM_END_ADDR0_SPEC>;
#[doc = "end address of exter memory range0 register"]
pub mod exter_mem_end_addr0;
#[doc = "EXTER_MEM_START_ADDR1 (rw) register accessor: Start address of exter memory range1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`exter_mem_start_addr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exter_mem_start_addr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exter_mem_start_addr1`] module"]
pub type EXTER_MEM_START_ADDR1 = crate::Reg<exter_mem_start_addr1::EXTER_MEM_START_ADDR1_SPEC>;
#[doc = "Start address of exter memory range1 register"]
pub mod exter_mem_start_addr1;
#[doc = "EXTER_MEM_END_ADDR1 (rw) register accessor: end address of exter memory range1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`exter_mem_end_addr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exter_mem_end_addr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exter_mem_end_addr1`] module"]
pub type EXTER_MEM_END_ADDR1 = crate::Reg<exter_mem_end_addr1::EXTER_MEM_END_ADDR1_SPEC>;
#[doc = "end address of exter memory range1 register"]
pub mod exter_mem_end_addr1;
#[doc = "OUT_ARB_CONFIG (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`out_arb_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_arb_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_arb_config`] module"]
pub type OUT_ARB_CONFIG = crate::Reg<out_arb_config::OUT_ARB_CONFIG_SPEC>;
#[doc = "reserved"]
pub mod out_arb_config;
#[doc = "IN_ARB_CONFIG (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`in_arb_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_arb_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_arb_config`] module"]
pub type IN_ARB_CONFIG = crate::Reg<in_arb_config::IN_ARB_CONFIG_SPEC>;
#[doc = "reserved"]
pub mod in_arb_config;
#[doc = "DATE (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "reserved"]
pub mod date;
#[doc = "COUNTER_RST (rw) register accessor: counter reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`counter_rst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`counter_rst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@counter_rst`] module"]
pub type COUNTER_RST = crate::Reg<counter_rst::COUNTER_RST_SPEC>;
#[doc = "counter reset register"]
pub mod counter_rst;
#[doc = "RX_CH0_COUNTER (r) register accessor: rx ch0 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ch0_counter::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ch0_counter`] module"]
pub type RX_CH0_COUNTER = crate::Reg<rx_ch0_counter::RX_CH0_COUNTER_SPEC>;
#[doc = "rx ch0 counter register"]
pub mod rx_ch0_counter;
#[doc = "RX_CH1_COUNTER (r) register accessor: rx ch1 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ch1_counter::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ch1_counter`] module"]
pub type RX_CH1_COUNTER = crate::Reg<rx_ch1_counter::RX_CH1_COUNTER_SPEC>;
#[doc = "rx ch1 counter register"]
pub mod rx_ch1_counter;
#[doc = "RX_CH2_COUNTER (r) register accessor: rx ch2 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ch2_counter::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ch2_counter`] module"]
pub type RX_CH2_COUNTER = crate::Reg<rx_ch2_counter::RX_CH2_COUNTER_SPEC>;
#[doc = "rx ch2 counter register"]
pub mod rx_ch2_counter;
#[doc = "RX_CH5_COUNTER (r) register accessor: rx ch5 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ch5_counter::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ch5_counter`] module"]
pub type RX_CH5_COUNTER = crate::Reg<rx_ch5_counter::RX_CH5_COUNTER_SPEC>;
#[doc = "rx ch5 counter register"]
pub mod rx_ch5_counter;
