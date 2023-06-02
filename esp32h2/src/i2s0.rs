#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    #[doc = "0x0c - I2S interrupt raw register, valid in level."]
    pub int_raw: INT_RAW,
    #[doc = "0x10 - I2S interrupt status register."]
    pub int_st: INT_ST,
    #[doc = "0x14 - I2S interrupt enable register."]
    pub int_ena: INT_ENA,
    #[doc = "0x18 - I2S interrupt clear register."]
    pub int_clr: INT_CLR,
    _reserved4: [u8; 0x04],
    #[doc = "0x20 - I2S RX configure register"]
    pub rx_conf: RX_CONF,
    #[doc = "0x24 - I2S TX configure register"]
    pub tx_conf: TX_CONF,
    #[doc = "0x28 - I2S RX configure register 1"]
    pub rx_conf1: RX_CONF1,
    #[doc = "0x2c - I2S TX configure register 1"]
    pub tx_conf1: TX_CONF1,
    #[doc = "0x30 - I2S RX clock configure register"]
    pub rx_clkm_conf: RX_CLKM_CONF,
    #[doc = "0x34 - I2S TX clock configure register"]
    pub tx_clkm_conf: TX_CLKM_CONF,
    #[doc = "0x38 - I2S RX module clock divider configure register"]
    pub rx_clkm_div_conf: RX_CLKM_DIV_CONF,
    #[doc = "0x3c - I2S TX module clock divider configure register"]
    pub tx_clkm_div_conf: TX_CLKM_DIV_CONF,
    #[doc = "0x40 - I2S TX PCM2PDM configuration register"]
    pub tx_pcm2pdm_conf: TX_PCM2PDM_CONF,
    #[doc = "0x44 - I2S TX PCM2PDM configuration register"]
    pub tx_pcm2pdm_conf1: TX_PCM2PDM_CONF1,
    _reserved14: [u8; 0x08],
    #[doc = "0x50 - I2S TX TDM mode control register"]
    pub rx_tdm_ctrl: RX_TDM_CTRL,
    #[doc = "0x54 - I2S TX TDM mode control register"]
    pub tx_tdm_ctrl: TX_TDM_CTRL,
    #[doc = "0x58 - I2S RX timing control register"]
    pub rx_timing: RX_TIMING,
    #[doc = "0x5c - I2S TX timing control register"]
    pub tx_timing: TX_TIMING,
    #[doc = "0x60 - I2S HUNG configure register."]
    pub lc_hung_conf: LC_HUNG_CONF,
    #[doc = "0x64 - I2S RX data number control register."]
    pub rxeof_num: RXEOF_NUM,
    #[doc = "0x68 - I2S signal data register"]
    pub conf_sigle_data: CONF_SIGLE_DATA,
    #[doc = "0x6c - I2S TX status register"]
    pub state: STATE,
    #[doc = "0x70 - I2S ETM configure register"]
    pub etm_conf: ETM_CONF,
    _reserved23: [u8; 0x0c],
    #[doc = "0x80 - Version control register"]
    pub date: DATE,
}
#[doc = "INT_RAW (r) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "I2S interrupt raw register, valid in level."]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "I2S interrupt status register."]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "I2S interrupt enable register."]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "I2S interrupt clear register."]
pub mod int_clr;
#[doc = "RX_CONF (rw) register accessor: an alias for `Reg<RX_CONF_SPEC>`"]
pub type RX_CONF = crate::Reg<rx_conf::RX_CONF_SPEC>;
#[doc = "I2S RX configure register"]
pub mod rx_conf;
#[doc = "TX_CONF (rw) register accessor: an alias for `Reg<TX_CONF_SPEC>`"]
pub type TX_CONF = crate::Reg<tx_conf::TX_CONF_SPEC>;
#[doc = "I2S TX configure register"]
pub mod tx_conf;
#[doc = "RX_CONF1 (rw) register accessor: an alias for `Reg<RX_CONF1_SPEC>`"]
pub type RX_CONF1 = crate::Reg<rx_conf1::RX_CONF1_SPEC>;
#[doc = "I2S RX configure register 1"]
pub mod rx_conf1;
#[doc = "TX_CONF1 (rw) register accessor: an alias for `Reg<TX_CONF1_SPEC>`"]
pub type TX_CONF1 = crate::Reg<tx_conf1::TX_CONF1_SPEC>;
#[doc = "I2S TX configure register 1"]
pub mod tx_conf1;
#[doc = "RX_CLKM_CONF (rw) register accessor: an alias for `Reg<RX_CLKM_CONF_SPEC>`"]
pub type RX_CLKM_CONF = crate::Reg<rx_clkm_conf::RX_CLKM_CONF_SPEC>;
#[doc = "I2S RX clock configure register"]
pub mod rx_clkm_conf;
#[doc = "TX_CLKM_CONF (rw) register accessor: an alias for `Reg<TX_CLKM_CONF_SPEC>`"]
pub type TX_CLKM_CONF = crate::Reg<tx_clkm_conf::TX_CLKM_CONF_SPEC>;
#[doc = "I2S TX clock configure register"]
pub mod tx_clkm_conf;
#[doc = "RX_CLKM_DIV_CONF (rw) register accessor: an alias for `Reg<RX_CLKM_DIV_CONF_SPEC>`"]
pub type RX_CLKM_DIV_CONF = crate::Reg<rx_clkm_div_conf::RX_CLKM_DIV_CONF_SPEC>;
#[doc = "I2S RX module clock divider configure register"]
pub mod rx_clkm_div_conf;
#[doc = "TX_CLKM_DIV_CONF (rw) register accessor: an alias for `Reg<TX_CLKM_DIV_CONF_SPEC>`"]
pub type TX_CLKM_DIV_CONF = crate::Reg<tx_clkm_div_conf::TX_CLKM_DIV_CONF_SPEC>;
#[doc = "I2S TX module clock divider configure register"]
pub mod tx_clkm_div_conf;
#[doc = "TX_PCM2PDM_CONF (rw) register accessor: an alias for `Reg<TX_PCM2PDM_CONF_SPEC>`"]
pub type TX_PCM2PDM_CONF = crate::Reg<tx_pcm2pdm_conf::TX_PCM2PDM_CONF_SPEC>;
#[doc = "I2S TX PCM2PDM configuration register"]
pub mod tx_pcm2pdm_conf;
#[doc = "TX_PCM2PDM_CONF1 (rw) register accessor: an alias for `Reg<TX_PCM2PDM_CONF1_SPEC>`"]
pub type TX_PCM2PDM_CONF1 = crate::Reg<tx_pcm2pdm_conf1::TX_PCM2PDM_CONF1_SPEC>;
#[doc = "I2S TX PCM2PDM configuration register"]
pub mod tx_pcm2pdm_conf1;
#[doc = "RX_TDM_CTRL (rw) register accessor: an alias for `Reg<RX_TDM_CTRL_SPEC>`"]
pub type RX_TDM_CTRL = crate::Reg<rx_tdm_ctrl::RX_TDM_CTRL_SPEC>;
#[doc = "I2S TX TDM mode control register"]
pub mod rx_tdm_ctrl;
#[doc = "TX_TDM_CTRL (rw) register accessor: an alias for `Reg<TX_TDM_CTRL_SPEC>`"]
pub type TX_TDM_CTRL = crate::Reg<tx_tdm_ctrl::TX_TDM_CTRL_SPEC>;
#[doc = "I2S TX TDM mode control register"]
pub mod tx_tdm_ctrl;
#[doc = "RX_TIMING (rw) register accessor: an alias for `Reg<RX_TIMING_SPEC>`"]
pub type RX_TIMING = crate::Reg<rx_timing::RX_TIMING_SPEC>;
#[doc = "I2S RX timing control register"]
pub mod rx_timing;
#[doc = "TX_TIMING (rw) register accessor: an alias for `Reg<TX_TIMING_SPEC>`"]
pub type TX_TIMING = crate::Reg<tx_timing::TX_TIMING_SPEC>;
#[doc = "I2S TX timing control register"]
pub mod tx_timing;
#[doc = "LC_HUNG_CONF (rw) register accessor: an alias for `Reg<LC_HUNG_CONF_SPEC>`"]
pub type LC_HUNG_CONF = crate::Reg<lc_hung_conf::LC_HUNG_CONF_SPEC>;
#[doc = "I2S HUNG configure register."]
pub mod lc_hung_conf;
#[doc = "RXEOF_NUM (rw) register accessor: an alias for `Reg<RXEOF_NUM_SPEC>`"]
pub type RXEOF_NUM = crate::Reg<rxeof_num::RXEOF_NUM_SPEC>;
#[doc = "I2S RX data number control register."]
pub mod rxeof_num;
#[doc = "CONF_SIGLE_DATA (rw) register accessor: an alias for `Reg<CONF_SIGLE_DATA_SPEC>`"]
pub type CONF_SIGLE_DATA = crate::Reg<conf_sigle_data::CONF_SIGLE_DATA_SPEC>;
#[doc = "I2S signal data register"]
pub mod conf_sigle_data;
#[doc = "STATE (r) register accessor: an alias for `Reg<STATE_SPEC>`"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "I2S TX status register"]
pub mod state;
#[doc = "ETM_CONF (rw) register accessor: an alias for `Reg<ETM_CONF_SPEC>`"]
pub type ETM_CONF = crate::Reg<etm_conf::ETM_CONF_SPEC>;
#[doc = "I2S ETM configure register"]
pub mod etm_conf;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
