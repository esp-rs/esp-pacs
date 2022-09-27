#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    #[doc = "0x0c - I2S interrupt raw register, valid in level."]
    pub i2s_int_raw: I2S_INT_RAW,
    #[doc = "0x10 - I2S interrupt status register."]
    pub i2s_int_st: I2S_INT_ST,
    #[doc = "0x14 - I2S interrupt enable register."]
    pub i2s_int_ena: I2S_INT_ENA,
    #[doc = "0x18 - I2S interrupt clear register."]
    pub i2s_int_clr: I2S_INT_CLR,
    _reserved4: [u8; 0x04],
    #[doc = "0x20 - I2S RX configure register"]
    pub i2s_rx_conf: I2S_RX_CONF,
    #[doc = "0x24 - I2S TX configure register"]
    pub i2s_tx_conf: I2S_TX_CONF,
    #[doc = "0x28 - I2S RX configure register 1"]
    pub i2s_rx_conf1: I2S_RX_CONF1,
    #[doc = "0x2c - I2S TX configure register 1"]
    pub i2s_tx_conf1: I2S_TX_CONF1,
    #[doc = "0x30 - I2S RX clock configure register"]
    pub i2s_rx_clkm_conf: I2S_RX_CLKM_CONF,
    #[doc = "0x34 - I2S TX clock configure register"]
    pub i2s_tx_clkm_conf: I2S_TX_CLKM_CONF,
    #[doc = "0x38 - I2S RX module clock divider configure register"]
    pub i2s_rx_clkm_div_conf: I2S_RX_CLKM_DIV_CONF,
    #[doc = "0x3c - I2S TX module clock divider configure register"]
    pub i2s_tx_clkm_div_conf: I2S_TX_CLKM_DIV_CONF,
    _reserved12: [u8; 0x10],
    #[doc = "0x50 - I2S TX TDM mode control register"]
    pub i2s_rx_tdm_ctrl: I2S_RX_TDM_CTRL,
    #[doc = "0x54 - I2S TX TDM mode control register"]
    pub i2s_tx_tdm_ctrl: I2S_TX_TDM_CTRL,
    #[doc = "0x58 - I2S RX timing control register"]
    pub i2s_rx_timing: I2S_RX_TIMING,
    #[doc = "0x5c - I2S TX timing control register"]
    pub i2s_tx_timing: I2S_TX_TIMING,
    #[doc = "0x60 - I2S HUNG configure register."]
    pub i2s_lc_hung_conf: I2S_LC_HUNG_CONF,
    #[doc = "0x64 - I2S RX data number control register."]
    pub i2s_rxeof_num: I2S_RXEOF_NUM,
    #[doc = "0x68 - I2S signal data register"]
    pub i2s_conf_sigle_data: I2S_CONF_SIGLE_DATA,
    #[doc = "0x6c - I2S TX status register"]
    pub i2s_state: I2S_STATE,
    _reserved20: [u8; 0x10],
    #[doc = "0x80 - Version control register"]
    pub i2s_date: I2S_DATE,
}
#[doc = "I2S_INT_RAW (r) register accessor: an alias for `Reg<I2S_INT_RAW_SPEC>`"]
pub type I2S_INT_RAW = crate::Reg<i2s_int_raw::I2S_INT_RAW_SPEC>;
#[doc = "I2S interrupt raw register, valid in level."]
pub mod i2s_int_raw;
#[doc = "I2S_INT_ST (r) register accessor: an alias for `Reg<I2S_INT_ST_SPEC>`"]
pub type I2S_INT_ST = crate::Reg<i2s_int_st::I2S_INT_ST_SPEC>;
#[doc = "I2S interrupt status register."]
pub mod i2s_int_st;
#[doc = "I2S_INT_ENA (rw) register accessor: an alias for `Reg<I2S_INT_ENA_SPEC>`"]
pub type I2S_INT_ENA = crate::Reg<i2s_int_ena::I2S_INT_ENA_SPEC>;
#[doc = "I2S interrupt enable register."]
pub mod i2s_int_ena;
#[doc = "I2S_INT_CLR (w) register accessor: an alias for `Reg<I2S_INT_CLR_SPEC>`"]
pub type I2S_INT_CLR = crate::Reg<i2s_int_clr::I2S_INT_CLR_SPEC>;
#[doc = "I2S interrupt clear register."]
pub mod i2s_int_clr;
#[doc = "I2S_RX_CONF (rw) register accessor: an alias for `Reg<I2S_RX_CONF_SPEC>`"]
pub type I2S_RX_CONF = crate::Reg<i2s_rx_conf::I2S_RX_CONF_SPEC>;
#[doc = "I2S RX configure register"]
pub mod i2s_rx_conf;
#[doc = "I2S_TX_CONF (rw) register accessor: an alias for `Reg<I2S_TX_CONF_SPEC>`"]
pub type I2S_TX_CONF = crate::Reg<i2s_tx_conf::I2S_TX_CONF_SPEC>;
#[doc = "I2S TX configure register"]
pub mod i2s_tx_conf;
#[doc = "I2S_RX_CONF1 (rw) register accessor: an alias for `Reg<I2S_RX_CONF1_SPEC>`"]
pub type I2S_RX_CONF1 = crate::Reg<i2s_rx_conf1::I2S_RX_CONF1_SPEC>;
#[doc = "I2S RX configure register 1"]
pub mod i2s_rx_conf1;
#[doc = "I2S_TX_CONF1 (rw) register accessor: an alias for `Reg<I2S_TX_CONF1_SPEC>`"]
pub type I2S_TX_CONF1 = crate::Reg<i2s_tx_conf1::I2S_TX_CONF1_SPEC>;
#[doc = "I2S TX configure register 1"]
pub mod i2s_tx_conf1;
#[doc = "I2S_RX_CLKM_CONF (rw) register accessor: an alias for `Reg<I2S_RX_CLKM_CONF_SPEC>`"]
pub type I2S_RX_CLKM_CONF = crate::Reg<i2s_rx_clkm_conf::I2S_RX_CLKM_CONF_SPEC>;
#[doc = "I2S RX clock configure register"]
pub mod i2s_rx_clkm_conf;
#[doc = "I2S_TX_CLKM_CONF (rw) register accessor: an alias for `Reg<I2S_TX_CLKM_CONF_SPEC>`"]
pub type I2S_TX_CLKM_CONF = crate::Reg<i2s_tx_clkm_conf::I2S_TX_CLKM_CONF_SPEC>;
#[doc = "I2S TX clock configure register"]
pub mod i2s_tx_clkm_conf;
#[doc = "I2S_RX_CLKM_DIV_CONF (rw) register accessor: an alias for `Reg<I2S_RX_CLKM_DIV_CONF_SPEC>`"]
pub type I2S_RX_CLKM_DIV_CONF = crate::Reg<i2s_rx_clkm_div_conf::I2S_RX_CLKM_DIV_CONF_SPEC>;
#[doc = "I2S RX module clock divider configure register"]
pub mod i2s_rx_clkm_div_conf;
#[doc = "I2S_TX_CLKM_DIV_CONF (rw) register accessor: an alias for `Reg<I2S_TX_CLKM_DIV_CONF_SPEC>`"]
pub type I2S_TX_CLKM_DIV_CONF = crate::Reg<i2s_tx_clkm_div_conf::I2S_TX_CLKM_DIV_CONF_SPEC>;
#[doc = "I2S TX module clock divider configure register"]
pub mod i2s_tx_clkm_div_conf;
#[doc = "I2S_RX_TDM_CTRL (rw) register accessor: an alias for `Reg<I2S_RX_TDM_CTRL_SPEC>`"]
pub type I2S_RX_TDM_CTRL = crate::Reg<i2s_rx_tdm_ctrl::I2S_RX_TDM_CTRL_SPEC>;
#[doc = "I2S TX TDM mode control register"]
pub mod i2s_rx_tdm_ctrl;
#[doc = "I2S_TX_TDM_CTRL (rw) register accessor: an alias for `Reg<I2S_TX_TDM_CTRL_SPEC>`"]
pub type I2S_TX_TDM_CTRL = crate::Reg<i2s_tx_tdm_ctrl::I2S_TX_TDM_CTRL_SPEC>;
#[doc = "I2S TX TDM mode control register"]
pub mod i2s_tx_tdm_ctrl;
#[doc = "I2S_RX_TIMING (rw) register accessor: an alias for `Reg<I2S_RX_TIMING_SPEC>`"]
pub type I2S_RX_TIMING = crate::Reg<i2s_rx_timing::I2S_RX_TIMING_SPEC>;
#[doc = "I2S RX timing control register"]
pub mod i2s_rx_timing;
#[doc = "I2S_TX_TIMING (rw) register accessor: an alias for `Reg<I2S_TX_TIMING_SPEC>`"]
pub type I2S_TX_TIMING = crate::Reg<i2s_tx_timing::I2S_TX_TIMING_SPEC>;
#[doc = "I2S TX timing control register"]
pub mod i2s_tx_timing;
#[doc = "I2S_LC_HUNG_CONF (rw) register accessor: an alias for `Reg<I2S_LC_HUNG_CONF_SPEC>`"]
pub type I2S_LC_HUNG_CONF = crate::Reg<i2s_lc_hung_conf::I2S_LC_HUNG_CONF_SPEC>;
#[doc = "I2S HUNG configure register."]
pub mod i2s_lc_hung_conf;
#[doc = "I2S_RXEOF_NUM (rw) register accessor: an alias for `Reg<I2S_RXEOF_NUM_SPEC>`"]
pub type I2S_RXEOF_NUM = crate::Reg<i2s_rxeof_num::I2S_RXEOF_NUM_SPEC>;
#[doc = "I2S RX data number control register."]
pub mod i2s_rxeof_num;
#[doc = "I2S_CONF_SIGLE_DATA (rw) register accessor: an alias for `Reg<I2S_CONF_SIGLE_DATA_SPEC>`"]
pub type I2S_CONF_SIGLE_DATA = crate::Reg<i2s_conf_sigle_data::I2S_CONF_SIGLE_DATA_SPEC>;
#[doc = "I2S signal data register"]
pub mod i2s_conf_sigle_data;
#[doc = "I2S_STATE (r) register accessor: an alias for `Reg<I2S_STATE_SPEC>`"]
pub type I2S_STATE = crate::Reg<i2s_state::I2S_STATE_SPEC>;
#[doc = "I2S TX status register"]
pub mod i2s_state;
#[doc = "I2S_DATE (rw) register accessor: an alias for `Reg<I2S_DATE_SPEC>`"]
pub type I2S_DATE = crate::Reg<i2s_date::I2S_DATE_SPEC>;
#[doc = "Version control register"]
pub mod i2s_date;
