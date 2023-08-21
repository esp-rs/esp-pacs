#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00..0x20 - The read and write data register for CHANNEL%s by apb fifo access."]
    pub chdata: [CHDATA; 8],
    #[doc = "0x20..0x30 - Channel %s configure register 0"]
    pub ch_tx_conf0: [CH_TX_CONF0; 4],
    #[doc = "0x30 - Channel %s configure register 0"]
    pub ch4_rx_conf0: CH_RX_CONF0,
    #[doc = "0x34 - Channel %s configure register 1"]
    pub ch4_rx_conf1: CH_RX_CONF1,
    #[doc = "0x38 - Channel %s configure register 0"]
    pub ch5_rx_conf0: CH_RX_CONF0,
    #[doc = "0x3c - Channel %s configure register 1"]
    pub ch5_rx_conf1: CH_RX_CONF1,
    #[doc = "0x40 - Channel %s configure register 0"]
    pub ch6_rx_conf0: CH_RX_CONF0,
    #[doc = "0x44 - Channel %s configure register 1"]
    pub ch6_rx_conf1: CH_RX_CONF1,
    #[doc = "0x48 - Channel %s configure register 0"]
    pub ch7_rx_conf0: CH_RX_CONF0,
    #[doc = "0x4c - Channel %s configure register 1"]
    pub ch7_rx_conf1: CH_RX_CONF1,
    #[doc = "0x50..0x60 - Channel %s status register"]
    pub ch_tx_status: [CH_TX_STATUS; 4],
    #[doc = "0x60..0x70 - Channel %s status register"]
    pub ch_rx_status: [CH_RX_STATUS; 4],
    #[doc = "0x70 - Raw interrupt status"]
    pub int_raw: INT_RAW,
    #[doc = "0x74 - Masked interrupt status"]
    pub int_st: INT_ST,
    #[doc = "0x78 - Interrupt enable bits"]
    pub int_ena: INT_ENA,
    #[doc = "0x7c - Interrupt clear bits"]
    pub int_clr: INT_CLR,
    #[doc = "0x80..0x90 - Channel %s duty cycle configuration register"]
    pub chcarrier_duty: [CHCARRIER_DUTY; 4],
    #[doc = "0x90..0xa0 - Channel %s carrier remove register"]
    pub ch_rx_carrier_rm: [CH_RX_CARRIER_RM; 4],
    #[doc = "0xa0..0xb0 - Channel %s Tx event configuration register"]
    pub ch_tx_lim: [CH_TX_LIM; 4],
    #[doc = "0xb0..0xc0 - Channel %s Rx event configuration register"]
    pub ch_rx_lim: [CH_RX_LIM; 4],
    #[doc = "0xc0 - RMT apb configuration register"]
    pub sys_conf: SYS_CONF,
    #[doc = "0xc4 - RMT TX synchronous register"]
    pub tx_sim: TX_SIM,
    #[doc = "0xc8 - RMT clock divider reset register"]
    pub ref_cnt_rst: REF_CNT_RST,
    #[doc = "0xcc - RMT version register"]
    pub date: DATE,
}
#[doc = "CHDATA (rw) register accessor: The read and write data register for CHANNEL%s by apb fifo access.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chdata`] module"]
pub type CHDATA = crate::Reg<chdata::CHDATA_SPEC>;
#[doc = "The read and write data register for CHANNEL%s by apb fifo access."]
pub mod chdata;
#[doc = "CH_TX_CONF0 (rw) register accessor: Channel %s configure register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_tx_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_tx_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch_tx_conf0`] module"]
pub type CH_TX_CONF0 = crate::Reg<ch_tx_conf0::CH_TX_CONF0_SPEC>;
#[doc = "Channel %s configure register 0"]
pub mod ch_tx_conf0;
#[doc = "CH_RX_CONF0 (rw) register accessor: Channel %s configure register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_rx_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_rx_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch_rx_conf0`] module"]
pub type CH_RX_CONF0 = crate::Reg<ch_rx_conf0::CH_RX_CONF0_SPEC>;
#[doc = "Channel %s configure register 0"]
pub mod ch_rx_conf0;
#[doc = "CH_RX_CONF1 (rw) register accessor: Channel %s configure register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_rx_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_rx_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch_rx_conf1`] module"]
pub type CH_RX_CONF1 = crate::Reg<ch_rx_conf1::CH_RX_CONF1_SPEC>;
#[doc = "Channel %s configure register 1"]
pub mod ch_rx_conf1;
#[doc = "CH_TX_STATUS (r) register accessor: Channel %s status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_tx_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch_tx_status`] module"]
pub type CH_TX_STATUS = crate::Reg<ch_tx_status::CH_TX_STATUS_SPEC>;
#[doc = "Channel %s status register"]
pub mod ch_tx_status;
#[doc = "CH_RX_STATUS (r) register accessor: Channel %s status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_rx_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch_rx_status`] module"]
pub type CH_RX_STATUS = crate::Reg<ch_rx_status::CH_RX_STATUS_SPEC>;
#[doc = "Channel %s status register"]
pub mod ch_rx_status;
#[doc = "INT_RAW (rw) register accessor: Raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Raw interrupt status"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Masked interrupt status"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: Interrupt enable bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt enable bits"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: Interrupt clear bits\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt clear bits"]
pub mod int_clr;
#[doc = "CHCARRIER_DUTY (rw) register accessor: Channel %s duty cycle configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chcarrier_duty::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chcarrier_duty::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chcarrier_duty`] module"]
pub type CHCARRIER_DUTY = crate::Reg<chcarrier_duty::CHCARRIER_DUTY_SPEC>;
#[doc = "Channel %s duty cycle configuration register"]
pub mod chcarrier_duty;
#[doc = "CH_RX_CARRIER_RM (rw) register accessor: Channel %s carrier remove register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_rx_carrier_rm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_rx_carrier_rm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch_rx_carrier_rm`] module"]
pub type CH_RX_CARRIER_RM = crate::Reg<ch_rx_carrier_rm::CH_RX_CARRIER_RM_SPEC>;
#[doc = "Channel %s carrier remove register"]
pub mod ch_rx_carrier_rm;
#[doc = "CH_TX_LIM (rw) register accessor: Channel %s Tx event configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_tx_lim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_tx_lim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch_tx_lim`] module"]
pub type CH_TX_LIM = crate::Reg<ch_tx_lim::CH_TX_LIM_SPEC>;
#[doc = "Channel %s Tx event configuration register"]
pub mod ch_tx_lim;
#[doc = "CH_RX_LIM (rw) register accessor: Channel %s Rx event configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_rx_lim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_rx_lim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch_rx_lim`] module"]
pub type CH_RX_LIM = crate::Reg<ch_rx_lim::CH_RX_LIM_SPEC>;
#[doc = "Channel %s Rx event configuration register"]
pub mod ch_rx_lim;
#[doc = "SYS_CONF (rw) register accessor: RMT apb configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sys_conf`] module"]
pub type SYS_CONF = crate::Reg<sys_conf::SYS_CONF_SPEC>;
#[doc = "RMT apb configuration register"]
pub mod sys_conf;
#[doc = "TX_SIM (rw) register accessor: RMT TX synchronous register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_sim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_sim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tx_sim`] module"]
pub type TX_SIM = crate::Reg<tx_sim::TX_SIM_SPEC>;
#[doc = "RMT TX synchronous register"]
pub mod tx_sim;
#[doc = "REF_CNT_RST (w) register accessor: RMT clock divider reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ref_cnt_rst::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ref_cnt_rst`] module"]
pub type REF_CNT_RST = crate::Reg<ref_cnt_rst::REF_CNT_RST_SPEC>;
#[doc = "RMT clock divider reset register"]
pub mod ref_cnt_rst;
#[doc = "DATE (rw) register accessor: RMT version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "RMT version register"]
pub mod date;
