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
    _reserved12: [u8; 0x10],
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
    _reserved20: [u8; 0x10],
    #[doc = "0x80 - Version control register"]
    pub date: DATE,
}
#[doc = "INT_RAW (r) register accessor: I2S interrupt raw register, valid in level.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "I2S interrupt raw register, valid in level."]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: I2S interrupt status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "I2S interrupt status register."]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: I2S interrupt enable register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "I2S interrupt enable register."]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: I2S interrupt clear register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "I2S interrupt clear register."]
pub mod int_clr;
#[doc = "RX_CONF (rw) register accessor: I2S RX configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rx_conf`] module"]
pub type RX_CONF = crate::Reg<rx_conf::RX_CONF_SPEC>;
#[doc = "I2S RX configure register"]
pub mod rx_conf;
#[doc = "TX_CONF (rw) register accessor: I2S TX configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tx_conf`] module"]
pub type TX_CONF = crate::Reg<tx_conf::TX_CONF_SPEC>;
#[doc = "I2S TX configure register"]
pub mod tx_conf;
#[doc = "RX_CONF1 (rw) register accessor: I2S RX configure register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rx_conf1`] module"]
pub type RX_CONF1 = crate::Reg<rx_conf1::RX_CONF1_SPEC>;
#[doc = "I2S RX configure register 1"]
pub mod rx_conf1;
#[doc = "TX_CONF1 (rw) register accessor: I2S TX configure register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tx_conf1`] module"]
pub type TX_CONF1 = crate::Reg<tx_conf1::TX_CONF1_SPEC>;
#[doc = "I2S TX configure register 1"]
pub mod tx_conf1;
#[doc = "RX_CLKM_CONF (rw) register accessor: I2S RX clock configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_clkm_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_clkm_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rx_clkm_conf`] module"]
pub type RX_CLKM_CONF = crate::Reg<rx_clkm_conf::RX_CLKM_CONF_SPEC>;
#[doc = "I2S RX clock configure register"]
pub mod rx_clkm_conf;
#[doc = "TX_CLKM_CONF (rw) register accessor: I2S TX clock configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_clkm_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_clkm_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tx_clkm_conf`] module"]
pub type TX_CLKM_CONF = crate::Reg<tx_clkm_conf::TX_CLKM_CONF_SPEC>;
#[doc = "I2S TX clock configure register"]
pub mod tx_clkm_conf;
#[doc = "RX_CLKM_DIV_CONF (rw) register accessor: I2S RX module clock divider configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_clkm_div_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_clkm_div_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rx_clkm_div_conf`] module"]
pub type RX_CLKM_DIV_CONF = crate::Reg<rx_clkm_div_conf::RX_CLKM_DIV_CONF_SPEC>;
#[doc = "I2S RX module clock divider configure register"]
pub mod rx_clkm_div_conf;
#[doc = "TX_CLKM_DIV_CONF (rw) register accessor: I2S TX module clock divider configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_clkm_div_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_clkm_div_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tx_clkm_div_conf`] module"]
pub type TX_CLKM_DIV_CONF = crate::Reg<tx_clkm_div_conf::TX_CLKM_DIV_CONF_SPEC>;
#[doc = "I2S TX module clock divider configure register"]
pub mod tx_clkm_div_conf;
#[doc = "RX_TDM_CTRL (rw) register accessor: I2S TX TDM mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_tdm_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_tdm_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rx_tdm_ctrl`] module"]
pub type RX_TDM_CTRL = crate::Reg<rx_tdm_ctrl::RX_TDM_CTRL_SPEC>;
#[doc = "I2S TX TDM mode control register"]
pub mod rx_tdm_ctrl;
#[doc = "TX_TDM_CTRL (rw) register accessor: I2S TX TDM mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_tdm_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_tdm_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tx_tdm_ctrl`] module"]
pub type TX_TDM_CTRL = crate::Reg<tx_tdm_ctrl::TX_TDM_CTRL_SPEC>;
#[doc = "I2S TX TDM mode control register"]
pub mod tx_tdm_ctrl;
#[doc = "RX_TIMING (rw) register accessor: I2S RX timing control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_timing::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_timing::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rx_timing`] module"]
pub type RX_TIMING = crate::Reg<rx_timing::RX_TIMING_SPEC>;
#[doc = "I2S RX timing control register"]
pub mod rx_timing;
#[doc = "TX_TIMING (rw) register accessor: I2S TX timing control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_timing::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_timing::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tx_timing`] module"]
pub type TX_TIMING = crate::Reg<tx_timing::TX_TIMING_SPEC>;
#[doc = "I2S TX timing control register"]
pub mod tx_timing;
#[doc = "LC_HUNG_CONF (rw) register accessor: I2S HUNG configure register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lc_hung_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lc_hung_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lc_hung_conf`] module"]
pub type LC_HUNG_CONF = crate::Reg<lc_hung_conf::LC_HUNG_CONF_SPEC>;
#[doc = "I2S HUNG configure register."]
pub mod lc_hung_conf;
#[doc = "RXEOF_NUM (rw) register accessor: I2S RX data number control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxeof_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxeof_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rxeof_num`] module"]
pub type RXEOF_NUM = crate::Reg<rxeof_num::RXEOF_NUM_SPEC>;
#[doc = "I2S RX data number control register."]
pub mod rxeof_num;
#[doc = "CONF_SIGLE_DATA (rw) register accessor: I2S signal data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf_sigle_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_sigle_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`conf_sigle_data`] module"]
pub type CONF_SIGLE_DATA = crate::Reg<conf_sigle_data::CONF_SIGLE_DATA_SPEC>;
#[doc = "I2S signal data register"]
pub mod conf_sigle_data;
#[doc = "STATE (r) register accessor: I2S TX status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`state`] module"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "I2S TX status register"]
pub mod state;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
