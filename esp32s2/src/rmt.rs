#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00..0x10 - The read and write data register for CHANNEL%s by apb fifo access."]
    pub chdata: [CHDATA; 4],
    #[doc = "0x10 - Channel %s configure register 0"]
    pub ch0conf0: CHCONF0,
    #[doc = "0x14 - Channel %s configure register 1"]
    pub ch0conf1: CHCONF1,
    #[doc = "0x18 - Channel %s configure register 0"]
    pub ch1conf0: CHCONF0,
    #[doc = "0x1c - Channel %s configure register 1"]
    pub ch1conf1: CHCONF1,
    #[doc = "0x20 - Channel %s configure register 0"]
    pub ch2conf0: CHCONF0,
    #[doc = "0x24 - Channel %s configure register 1"]
    pub ch2conf1: CHCONF1,
    #[doc = "0x28 - Channel %s configure register 0"]
    pub ch3conf0: CHCONF0,
    #[doc = "0x2c - Channel %s configure register 1"]
    pub ch3conf1: CHCONF1,
    #[doc = "0x30..0x40 - Channel %s status register"]
    pub chstatus: [CHSTATUS; 4],
    #[doc = "0x40..0x50 - Channel %s address register"]
    pub chaddr: [CHADDR; 4],
    #[doc = "0x50 - Raw interrupt status"]
    pub int_raw: INT_RAW,
    #[doc = "0x54 - Masked interrupt status"]
    pub int_st: INT_ST,
    #[doc = "0x58 - Interrupt enable bits"]
    pub int_ena: INT_ENA,
    #[doc = "0x5c - Interrupt clear bits"]
    pub int_clr: INT_CLR,
    #[doc = "0x60..0x70 - Channel %s duty cycle configuration register"]
    pub chcarrier_duty: [CHCARRIER_DUTY; 4],
    #[doc = "0x70..0x80 - Channel %s Tx event configuration register"]
    pub ch_tx_lim: [CH_TX_LIM; 4],
    #[doc = "0x80 - RMT apb configuration register"]
    pub apb_conf: APB_CONF,
    #[doc = "0x84 - RMT TX synchronous register"]
    pub tx_sim: TX_SIM,
    #[doc = "0x88 - RMT clock divider reset register"]
    pub ref_cnt_rst: REF_CNT_RST,
    #[doc = "0x8c..0x9c - Channel %s carrier remove register"]
    pub ch_rx_carrier_rm: [CH_RX_CARRIER_RM; 4],
    _reserved21: [u8; 0x60],
    #[doc = "0xfc - RMT version register"]
    pub date: DATE,
}
#[doc = "CHDATA (rw) register accessor: an alias for `Reg<CHDATA_SPEC>`"]
pub type CHDATA = crate::Reg<chdata::CHDATA_SPEC>;
#[doc = "The read and write data register for CHANNEL%s by apb fifo access."]
pub mod chdata;
#[doc = "CHCONF0 (rw) register accessor: an alias for `Reg<CHCONF0_SPEC>`"]
pub type CHCONF0 = crate::Reg<chconf0::CHCONF0_SPEC>;
#[doc = "Channel %s configure register 0"]
pub mod chconf0;
#[doc = "CHCONF1 (rw) register accessor: an alias for `Reg<CHCONF1_SPEC>`"]
pub type CHCONF1 = crate::Reg<chconf1::CHCONF1_SPEC>;
#[doc = "Channel %s configure register 1"]
pub mod chconf1;
#[doc = "CHSTATUS (r) register accessor: an alias for `Reg<CHSTATUS_SPEC>`"]
pub type CHSTATUS = crate::Reg<chstatus::CHSTATUS_SPEC>;
#[doc = "Channel %s status register"]
pub mod chstatus;
#[doc = "CHADDR (r) register accessor: an alias for `Reg<CHADDR_SPEC>`"]
pub type CHADDR = crate::Reg<chaddr::CHADDR_SPEC>;
#[doc = "Channel %s address register"]
pub mod chaddr;
#[doc = "INT_RAW (r) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Raw interrupt status"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Masked interrupt status"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt enable bits"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt clear bits"]
pub mod int_clr;
#[doc = "CHCARRIER_DUTY (rw) register accessor: an alias for `Reg<CHCARRIER_DUTY_SPEC>`"]
pub type CHCARRIER_DUTY = crate::Reg<chcarrier_duty::CHCARRIER_DUTY_SPEC>;
#[doc = "Channel %s duty cycle configuration register"]
pub mod chcarrier_duty;
#[doc = "CH_TX_LIM (rw) register accessor: an alias for `Reg<CH_TX_LIM_SPEC>`"]
pub type CH_TX_LIM = crate::Reg<ch_tx_lim::CH_TX_LIM_SPEC>;
#[doc = "Channel %s Tx event configuration register"]
pub mod ch_tx_lim;
#[doc = "APB_CONF (rw) register accessor: an alias for `Reg<APB_CONF_SPEC>`"]
pub type APB_CONF = crate::Reg<apb_conf::APB_CONF_SPEC>;
#[doc = "RMT apb configuration register"]
pub mod apb_conf;
#[doc = "TX_SIM (rw) register accessor: an alias for `Reg<TX_SIM_SPEC>`"]
pub type TX_SIM = crate::Reg<tx_sim::TX_SIM_SPEC>;
#[doc = "RMT TX synchronous register"]
pub mod tx_sim;
#[doc = "REF_CNT_RST (rw) register accessor: an alias for `Reg<REF_CNT_RST_SPEC>`"]
pub type REF_CNT_RST = crate::Reg<ref_cnt_rst::REF_CNT_RST_SPEC>;
#[doc = "RMT clock divider reset register"]
pub mod ref_cnt_rst;
#[doc = "CH_RX_CARRIER_RM (rw) register accessor: an alias for `Reg<CH_RX_CARRIER_RM_SPEC>`"]
pub type CH_RX_CARRIER_RM = crate::Reg<ch_rx_carrier_rm::CH_RX_CARRIER_RM_SPEC>;
#[doc = "Channel %s carrier remove register"]
pub mod ch_rx_carrier_rm;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "RMT version register"]
pub mod date;
