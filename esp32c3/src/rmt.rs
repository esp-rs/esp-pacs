#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00..0x10 - RMT_CH%sDATA_REG."]
    pub chdata: [CHDATA; 4],
    #[doc = "0x10..0x18 - RMT_CH%sCONF%s_REG."]
    pub ch_tx_conf0: [CH_TX_CONF0; 2],
    #[doc = "0x18 - RMT_CH2CONF0_REG."]
    pub ch2_rx_conf0: CH_RX_CONF0,
    #[doc = "0x1c - RMT_CH2CONF1_REG."]
    pub ch2_rx_conf1: CH_RX_CONF1,
    #[doc = "0x20 - RMT_CH2CONF0_REG."]
    pub ch3_rx_conf0: CH_RX_CONF0,
    #[doc = "0x24 - RMT_CH2CONF1_REG."]
    pub ch3_rx_conf1: CH_RX_CONF1,
    #[doc = "0x28..0x30 - RMT_CH%sSTATUS_REG."]
    pub ch_tx_status: [CH_TX_STATUS; 2],
    #[doc = "0x30..0x38 - RMT_CH2STATUS_REG."]
    pub ch_rx_status: [CH_RX_STATUS; 2],
    #[doc = "0x38 - RMT_INT_RAW_REG."]
    pub int_raw: INT_RAW,
    #[doc = "0x3c - RMT_INT_ST_REG."]
    pub int_st: INT_ST,
    #[doc = "0x40 - RMT_INT_ENA_REG."]
    pub int_ena: INT_ENA,
    #[doc = "0x44 - RMT_INT_CLR_REG."]
    pub int_clr: INT_CLR,
    #[doc = "0x48..0x50 - RMT_CH%sCARRIER_DUTY_REG."]
    pub chcarrier_duty: [CHCARRIER_DUTY; 2],
    #[doc = "0x50..0x58 - RMT_CH2_RX_CARRIER_RM_REG."]
    pub ch_rx_carrier_rm: [CH_RX_CARRIER_RM; 2],
    #[doc = "0x58..0x60 - RMT_CH%s_TX_LIM_REG."]
    pub ch_tx_lim: [CH_TX_LIM; 2],
    #[doc = "0x60..0x68 - RMT_CH2_RX_LIM_REG."]
    pub ch_rx_lim: [CH_RX_LIM; 2],
    #[doc = "0x68 - RMT_SYS_CONF_REG."]
    pub sys_conf: SYS_CONF,
    #[doc = "0x6c - RMT_TX_SIM_REG."]
    pub tx_sim: TX_SIM,
    #[doc = "0x70 - RMT_REF_CNT_RST_REG."]
    pub ref_cnt_rst: REF_CNT_RST,
    _reserved19: [u8; 0x58],
    #[doc = "0xcc - RMT_DATE_REG."]
    pub date: DATE,
}
impl RegisterBlock {
    #[doc = "0x30 - RMT_CH2STATUS_REG."]
    #[inline(always)]
    pub fn ch2_rx_status(&self) -> &CH_RX_STATUS {
        &self.ch_rx_status[0]
    }
    #[doc = "0x34 - RMT_CH2STATUS_REG."]
    #[inline(always)]
    pub fn ch3_rx_status(&self) -> &CH_RX_STATUS {
        &self.ch_rx_status[1]
    }
    #[doc = "0x50 - RMT_CH2_RX_CARRIER_RM_REG."]
    #[inline(always)]
    pub fn ch2_rx_carrier_rm(&self) -> &CH_RX_CARRIER_RM {
        &self.ch_rx_carrier_rm[0]
    }
    #[doc = "0x54 - RMT_CH2_RX_CARRIER_RM_REG."]
    #[inline(always)]
    pub fn ch3_rx_carrier_rm(&self) -> &CH_RX_CARRIER_RM {
        &self.ch_rx_carrier_rm[1]
    }
    #[doc = "0x60 - RMT_CH2_RX_LIM_REG."]
    #[inline(always)]
    pub fn ch2_rx_lim(&self) -> &CH_RX_LIM {
        &self.ch_rx_lim[0]
    }
    #[doc = "0x64 - RMT_CH2_RX_LIM_REG."]
    #[inline(always)]
    pub fn ch3_rx_lim(&self) -> &CH_RX_LIM {
        &self.ch_rx_lim[1]
    }
}
#[doc = "CHDATA (rw) register accessor: RMT_CH%sDATA_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chdata`] module"]
pub type CHDATA = crate::Reg<chdata::CHDATA_SPEC>;
#[doc = "RMT_CH%sDATA_REG."]
pub mod chdata;
#[doc = "CH_TX_CONF0 (rw) register accessor: RMT_CH%sCONF%s_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_tx_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_tx_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch_tx_conf0`] module"]
pub type CH_TX_CONF0 = crate::Reg<ch_tx_conf0::CH_TX_CONF0_SPEC>;
#[doc = "RMT_CH%sCONF%s_REG."]
pub mod ch_tx_conf0;
#[doc = "CH_RX_CONF0 (rw) register accessor: RMT_CH2CONF0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_rx_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_rx_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch_rx_conf0`] module"]
pub type CH_RX_CONF0 = crate::Reg<ch_rx_conf0::CH_RX_CONF0_SPEC>;
#[doc = "RMT_CH2CONF0_REG."]
pub mod ch_rx_conf0;
#[doc = "CH_RX_CONF1 (rw) register accessor: RMT_CH2CONF1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_rx_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_rx_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch_rx_conf1`] module"]
pub type CH_RX_CONF1 = crate::Reg<ch_rx_conf1::CH_RX_CONF1_SPEC>;
#[doc = "RMT_CH2CONF1_REG."]
pub mod ch_rx_conf1;
#[doc = "CH_TX_STATUS (r) register accessor: RMT_CH%sSTATUS_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_tx_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch_tx_status`] module"]
pub type CH_TX_STATUS = crate::Reg<ch_tx_status::CH_TX_STATUS_SPEC>;
#[doc = "RMT_CH%sSTATUS_REG."]
pub mod ch_tx_status;
#[doc = "CH_RX_STATUS (r) register accessor: RMT_CH2STATUS_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_rx_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch_rx_status`] module"]
pub type CH_RX_STATUS = crate::Reg<ch_rx_status::CH_RX_STATUS_SPEC>;
#[doc = "RMT_CH2STATUS_REG."]
pub mod ch_rx_status;
#[doc = "INT_RAW (rw) register accessor: RMT_INT_RAW_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "RMT_INT_RAW_REG."]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: RMT_INT_ST_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "RMT_INT_ST_REG."]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: RMT_INT_ENA_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "RMT_INT_ENA_REG."]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: RMT_INT_CLR_REG.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "RMT_INT_CLR_REG."]
pub mod int_clr;
#[doc = "CHCARRIER_DUTY (rw) register accessor: RMT_CH%sCARRIER_DUTY_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chcarrier_duty::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chcarrier_duty::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chcarrier_duty`] module"]
pub type CHCARRIER_DUTY = crate::Reg<chcarrier_duty::CHCARRIER_DUTY_SPEC>;
#[doc = "RMT_CH%sCARRIER_DUTY_REG."]
pub mod chcarrier_duty;
#[doc = "CH_RX_CARRIER_RM (rw) register accessor: RMT_CH2_RX_CARRIER_RM_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_rx_carrier_rm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_rx_carrier_rm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch_rx_carrier_rm`] module"]
pub type CH_RX_CARRIER_RM = crate::Reg<ch_rx_carrier_rm::CH_RX_CARRIER_RM_SPEC>;
#[doc = "RMT_CH2_RX_CARRIER_RM_REG."]
pub mod ch_rx_carrier_rm;
#[doc = "CH_TX_LIM (rw) register accessor: RMT_CH%s_TX_LIM_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_tx_lim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_tx_lim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch_tx_lim`] module"]
pub type CH_TX_LIM = crate::Reg<ch_tx_lim::CH_TX_LIM_SPEC>;
#[doc = "RMT_CH%s_TX_LIM_REG."]
pub mod ch_tx_lim;
#[doc = "CH_RX_LIM (rw) register accessor: RMT_CH2_RX_LIM_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_rx_lim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_rx_lim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch_rx_lim`] module"]
pub type CH_RX_LIM = crate::Reg<ch_rx_lim::CH_RX_LIM_SPEC>;
#[doc = "RMT_CH2_RX_LIM_REG."]
pub mod ch_rx_lim;
#[doc = "SYS_CONF (rw) register accessor: RMT_SYS_CONF_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sys_conf`] module"]
pub type SYS_CONF = crate::Reg<sys_conf::SYS_CONF_SPEC>;
#[doc = "RMT_SYS_CONF_REG."]
pub mod sys_conf;
#[doc = "TX_SIM (rw) register accessor: RMT_TX_SIM_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_sim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_sim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tx_sim`] module"]
pub type TX_SIM = crate::Reg<tx_sim::TX_SIM_SPEC>;
#[doc = "RMT_TX_SIM_REG."]
pub mod tx_sim;
#[doc = "REF_CNT_RST (w) register accessor: RMT_REF_CNT_RST_REG.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ref_cnt_rst::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ref_cnt_rst`] module"]
pub type REF_CNT_RST = crate::Reg<ref_cnt_rst::REF_CNT_RST_SPEC>;
#[doc = "RMT_REF_CNT_RST_REG."]
pub mod ref_cnt_rst;
#[doc = "DATE (rw) register accessor: RMT_DATE_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "RMT_DATE_REG."]
pub mod date;
