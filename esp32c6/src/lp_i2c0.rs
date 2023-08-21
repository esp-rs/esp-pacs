#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Configures the low level width of the SCL Clock"]
    pub i2c_scl_low_period: I2C_SCL_LOW_PERIOD,
    #[doc = "0x04 - Transmission setting"]
    pub i2c_ctr: I2C_CTR,
    #[doc = "0x08 - Describe I2C work status."]
    pub i2c_sr: I2C_SR,
    #[doc = "0x0c - Setting time out control for receiving data."]
    pub i2c_to: I2C_TO,
    _reserved4: [u8; 0x04],
    #[doc = "0x14 - FIFO status register."]
    pub i2c_fifo_st: I2C_FIFO_ST,
    #[doc = "0x18 - FIFO configuration register."]
    pub fifo_conf: FIFO_CONF,
    #[doc = "0x1c - Rx FIFO read data."]
    pub i2c_data: I2C_DATA,
    #[doc = "0x20 - Raw interrupt status"]
    pub i2c_int_raw: I2C_INT_RAW,
    #[doc = "0x24 - Interrupt clear bits"]
    pub i2c_int_clr: I2C_INT_CLR,
    #[doc = "0x28 - Interrupt enable bits"]
    pub i2c_int_ena: I2C_INT_ENA,
    #[doc = "0x2c - Status of captured I2C communication events"]
    pub i2c_int_status: I2C_INT_STATUS,
    #[doc = "0x30 - Configures the hold time after a negative SCL edge."]
    pub i2c_sda_hold: I2C_SDA_HOLD,
    #[doc = "0x34 - Configures the sample time after a positive SCL edge."]
    pub i2c_sda_sample: I2C_SDA_SAMPLE,
    #[doc = "0x38 - Configures the high level width of SCL"]
    pub i2c_scl_high_period: I2C_SCL_HIGH_PERIOD,
    _reserved14: [u8; 0x04],
    #[doc = "0x40 - Configures the delay between the SDA and SCL negative edge for a start condition"]
    pub i2c_scl_start_hold: I2C_SCL_START_HOLD,
    #[doc = "0x44 - Configures the delay between the positive edge of SCL and the negative edge of SDA"]
    pub i2c_scl_rstart_setup: I2C_SCL_RSTART_SETUP,
    #[doc = "0x48 - Configures the delay after the SCL clock edge for a stop condition"]
    pub i2c_scl_stop_hold: I2C_SCL_STOP_HOLD,
    #[doc = "0x4c - Configures the delay between the SDA and SCL positive edge for a stop condition"]
    pub i2c_scl_stop_setup: I2C_SCL_STOP_SETUP,
    #[doc = "0x50 - SCL and SDA filter configuration register"]
    pub i2c_filter_cfg: I2C_FILTER_CFG,
    #[doc = "0x54 - I2C CLK configuration register"]
    pub i2c_clk_conf: I2C_CLK_CONF,
    #[doc = "0x58 - I2C command register 0"]
    pub i2c_comd0: I2C_COMD0,
    #[doc = "0x5c - I2C command register 1"]
    pub i2c_comd1: I2C_COMD1,
    #[doc = "0x60 - I2C command register 2"]
    pub i2c_comd2: I2C_COMD2,
    #[doc = "0x64 - I2C command register 3"]
    pub i2c_comd3: I2C_COMD3,
    #[doc = "0x68 - I2C command register 4"]
    pub i2c_comd4: I2C_COMD4,
    #[doc = "0x6c - I2C command register 5"]
    pub i2c_comd5: I2C_COMD5,
    #[doc = "0x70 - I2C command register 6"]
    pub i2c_comd6: I2C_COMD6,
    #[doc = "0x74 - I2C command register 7"]
    pub i2c_comd7: I2C_COMD7,
    #[doc = "0x78 - SCL status time out register"]
    pub i2c_scl_st_time_out: I2C_SCL_ST_TIME_OUT,
    #[doc = "0x7c - SCL main status time out register"]
    pub i2c_scl_main_st_time_out: I2C_SCL_MAIN_ST_TIME_OUT,
    #[doc = "0x80 - Power configuration register"]
    pub i2c_scl_sp_conf: I2C_SCL_SP_CONF,
    _reserved31: [u8; 0x74],
    #[doc = "0xf8 - Version register"]
    pub i2c_date: I2C_DATE,
    _reserved32: [u8; 0x04],
    #[doc = "0x100 - I2C TXFIFO base address register"]
    pub i2c_txfifo_start_addr: I2C_TXFIFO_START_ADDR,
    _reserved33: [u8; 0x7c],
    #[doc = "0x180 - I2C RXFIFO base address register"]
    pub i2c_rxfifo_start_addr: I2C_RXFIFO_START_ADDR,
}
#[doc = "I2C_SCL_LOW_PERIOD (rw) register accessor: Configures the low level width of the SCL Clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_scl_low_period::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_scl_low_period::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_scl_low_period`] module"]
pub type I2C_SCL_LOW_PERIOD = crate::Reg<i2c_scl_low_period::I2C_SCL_LOW_PERIOD_SPEC>;
#[doc = "Configures the low level width of the SCL Clock"]
pub mod i2c_scl_low_period;
#[doc = "I2C_CTR (rw) register accessor: Transmission setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_ctr`] module"]
pub type I2C_CTR = crate::Reg<i2c_ctr::I2C_CTR_SPEC>;
#[doc = "Transmission setting"]
pub mod i2c_ctr;
#[doc = "I2C_SR (r) register accessor: Describe I2C work status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_sr`] module"]
pub type I2C_SR = crate::Reg<i2c_sr::I2C_SR_SPEC>;
#[doc = "Describe I2C work status."]
pub mod i2c_sr;
#[doc = "I2C_TO (rw) register accessor: Setting time out control for receiving data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_to::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_to::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_to`] module"]
pub type I2C_TO = crate::Reg<i2c_to::I2C_TO_SPEC>;
#[doc = "Setting time out control for receiving data."]
pub mod i2c_to;
#[doc = "I2C_FIFO_ST (r) register accessor: FIFO status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_fifo_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_fifo_st`] module"]
pub type I2C_FIFO_ST = crate::Reg<i2c_fifo_st::I2C_FIFO_ST_SPEC>;
#[doc = "FIFO status register."]
pub mod i2c_fifo_st;
#[doc = "FIFO_CONF (rw) register accessor: FIFO configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fifo_conf`] module"]
pub type FIFO_CONF = crate::Reg<fifo_conf::FIFO_CONF_SPEC>;
#[doc = "FIFO configuration register."]
pub mod fifo_conf;
#[doc = "I2C_DATA (r) register accessor: Rx FIFO read data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_data`] module"]
pub type I2C_DATA = crate::Reg<i2c_data::I2C_DATA_SPEC>;
#[doc = "Rx FIFO read data."]
pub mod i2c_data;
#[doc = "I2C_INT_RAW (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_int_raw`] module"]
pub type I2C_INT_RAW = crate::Reg<i2c_int_raw::I2C_INT_RAW_SPEC>;
#[doc = "Raw interrupt status"]
pub mod i2c_int_raw;
#[doc = "I2C_INT_CLR (w) register accessor: Interrupt clear bits\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_int_clr`] module"]
pub type I2C_INT_CLR = crate::Reg<i2c_int_clr::I2C_INT_CLR_SPEC>;
#[doc = "Interrupt clear bits"]
pub mod i2c_int_clr;
#[doc = "I2C_INT_ENA (rw) register accessor: Interrupt enable bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_int_ena`] module"]
pub type I2C_INT_ENA = crate::Reg<i2c_int_ena::I2C_INT_ENA_SPEC>;
#[doc = "Interrupt enable bits"]
pub mod i2c_int_ena;
#[doc = "I2C_INT_STATUS (r) register accessor: Status of captured I2C communication events\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_int_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_int_status`] module"]
pub type I2C_INT_STATUS = crate::Reg<i2c_int_status::I2C_INT_STATUS_SPEC>;
#[doc = "Status of captured I2C communication events"]
pub mod i2c_int_status;
#[doc = "I2C_SDA_HOLD (rw) register accessor: Configures the hold time after a negative SCL edge.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_sda_hold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_sda_hold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_sda_hold`] module"]
pub type I2C_SDA_HOLD = crate::Reg<i2c_sda_hold::I2C_SDA_HOLD_SPEC>;
#[doc = "Configures the hold time after a negative SCL edge."]
pub mod i2c_sda_hold;
#[doc = "I2C_SDA_SAMPLE (rw) register accessor: Configures the sample time after a positive SCL edge.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_sda_sample::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_sda_sample::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_sda_sample`] module"]
pub type I2C_SDA_SAMPLE = crate::Reg<i2c_sda_sample::I2C_SDA_SAMPLE_SPEC>;
#[doc = "Configures the sample time after a positive SCL edge."]
pub mod i2c_sda_sample;
#[doc = "I2C_SCL_HIGH_PERIOD (rw) register accessor: Configures the high level width of SCL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_scl_high_period::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_scl_high_period::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_scl_high_period`] module"]
pub type I2C_SCL_HIGH_PERIOD = crate::Reg<i2c_scl_high_period::I2C_SCL_HIGH_PERIOD_SPEC>;
#[doc = "Configures the high level width of SCL"]
pub mod i2c_scl_high_period;
#[doc = "I2C_SCL_START_HOLD (rw) register accessor: Configures the delay between the SDA and SCL negative edge for a start condition\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_scl_start_hold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_scl_start_hold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_scl_start_hold`] module"]
pub type I2C_SCL_START_HOLD = crate::Reg<i2c_scl_start_hold::I2C_SCL_START_HOLD_SPEC>;
#[doc = "Configures the delay between the SDA and SCL negative edge for a start condition"]
pub mod i2c_scl_start_hold;
#[doc = "I2C_SCL_RSTART_SETUP (rw) register accessor: Configures the delay between the positive edge of SCL and the negative edge of SDA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_scl_rstart_setup::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_scl_rstart_setup::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_scl_rstart_setup`] module"]
pub type I2C_SCL_RSTART_SETUP = crate::Reg<i2c_scl_rstart_setup::I2C_SCL_RSTART_SETUP_SPEC>;
#[doc = "Configures the delay between the positive edge of SCL and the negative edge of SDA"]
pub mod i2c_scl_rstart_setup;
#[doc = "I2C_SCL_STOP_HOLD (rw) register accessor: Configures the delay after the SCL clock edge for a stop condition\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_scl_stop_hold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_scl_stop_hold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_scl_stop_hold`] module"]
pub type I2C_SCL_STOP_HOLD = crate::Reg<i2c_scl_stop_hold::I2C_SCL_STOP_HOLD_SPEC>;
#[doc = "Configures the delay after the SCL clock edge for a stop condition"]
pub mod i2c_scl_stop_hold;
#[doc = "I2C_SCL_STOP_SETUP (rw) register accessor: Configures the delay between the SDA and SCL positive edge for a stop condition\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_scl_stop_setup::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_scl_stop_setup::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_scl_stop_setup`] module"]
pub type I2C_SCL_STOP_SETUP = crate::Reg<i2c_scl_stop_setup::I2C_SCL_STOP_SETUP_SPEC>;
#[doc = "Configures the delay between the SDA and SCL positive edge for a stop condition"]
pub mod i2c_scl_stop_setup;
#[doc = "I2C_FILTER_CFG (rw) register accessor: SCL and SDA filter configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_filter_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_filter_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_filter_cfg`] module"]
pub type I2C_FILTER_CFG = crate::Reg<i2c_filter_cfg::I2C_FILTER_CFG_SPEC>;
#[doc = "SCL and SDA filter configuration register"]
pub mod i2c_filter_cfg;
#[doc = "I2C_CLK_CONF (rw) register accessor: I2C CLK configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_clk_conf`] module"]
pub type I2C_CLK_CONF = crate::Reg<i2c_clk_conf::I2C_CLK_CONF_SPEC>;
#[doc = "I2C CLK configuration register"]
pub mod i2c_clk_conf;
#[doc = "I2C_COMD0 (rw) register accessor: I2C command register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_comd0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_comd0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_comd0`] module"]
pub type I2C_COMD0 = crate::Reg<i2c_comd0::I2C_COMD0_SPEC>;
#[doc = "I2C command register 0"]
pub mod i2c_comd0;
#[doc = "I2C_COMD1 (rw) register accessor: I2C command register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_comd1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_comd1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_comd1`] module"]
pub type I2C_COMD1 = crate::Reg<i2c_comd1::I2C_COMD1_SPEC>;
#[doc = "I2C command register 1"]
pub mod i2c_comd1;
#[doc = "I2C_COMD2 (rw) register accessor: I2C command register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_comd2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_comd2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_comd2`] module"]
pub type I2C_COMD2 = crate::Reg<i2c_comd2::I2C_COMD2_SPEC>;
#[doc = "I2C command register 2"]
pub mod i2c_comd2;
#[doc = "I2C_COMD3 (rw) register accessor: I2C command register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_comd3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_comd3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_comd3`] module"]
pub type I2C_COMD3 = crate::Reg<i2c_comd3::I2C_COMD3_SPEC>;
#[doc = "I2C command register 3"]
pub mod i2c_comd3;
#[doc = "I2C_COMD4 (rw) register accessor: I2C command register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_comd4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_comd4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_comd4`] module"]
pub type I2C_COMD4 = crate::Reg<i2c_comd4::I2C_COMD4_SPEC>;
#[doc = "I2C command register 4"]
pub mod i2c_comd4;
#[doc = "I2C_COMD5 (rw) register accessor: I2C command register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_comd5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_comd5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_comd5`] module"]
pub type I2C_COMD5 = crate::Reg<i2c_comd5::I2C_COMD5_SPEC>;
#[doc = "I2C command register 5"]
pub mod i2c_comd5;
#[doc = "I2C_COMD6 (rw) register accessor: I2C command register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_comd6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_comd6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_comd6`] module"]
pub type I2C_COMD6 = crate::Reg<i2c_comd6::I2C_COMD6_SPEC>;
#[doc = "I2C command register 6"]
pub mod i2c_comd6;
#[doc = "I2C_COMD7 (rw) register accessor: I2C command register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_comd7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_comd7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_comd7`] module"]
pub type I2C_COMD7 = crate::Reg<i2c_comd7::I2C_COMD7_SPEC>;
#[doc = "I2C command register 7"]
pub mod i2c_comd7;
#[doc = "I2C_SCL_ST_TIME_OUT (rw) register accessor: SCL status time out register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_scl_st_time_out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_scl_st_time_out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_scl_st_time_out`] module"]
pub type I2C_SCL_ST_TIME_OUT = crate::Reg<i2c_scl_st_time_out::I2C_SCL_ST_TIME_OUT_SPEC>;
#[doc = "SCL status time out register"]
pub mod i2c_scl_st_time_out;
#[doc = "I2C_SCL_MAIN_ST_TIME_OUT (rw) register accessor: SCL main status time out register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_scl_main_st_time_out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_scl_main_st_time_out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_scl_main_st_time_out`] module"]
pub type I2C_SCL_MAIN_ST_TIME_OUT =
    crate::Reg<i2c_scl_main_st_time_out::I2C_SCL_MAIN_ST_TIME_OUT_SPEC>;
#[doc = "SCL main status time out register"]
pub mod i2c_scl_main_st_time_out;
#[doc = "I2C_SCL_SP_CONF (rw) register accessor: Power configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_scl_sp_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_scl_sp_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_scl_sp_conf`] module"]
pub type I2C_SCL_SP_CONF = crate::Reg<i2c_scl_sp_conf::I2C_SCL_SP_CONF_SPEC>;
#[doc = "Power configuration register"]
pub mod i2c_scl_sp_conf;
#[doc = "I2C_DATE (rw) register accessor: Version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_date`] module"]
pub type I2C_DATE = crate::Reg<i2c_date::I2C_DATE_SPEC>;
#[doc = "Version register"]
pub mod i2c_date;
#[doc = "I2C_TXFIFO_START_ADDR (r) register accessor: I2C TXFIFO base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_txfifo_start_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_txfifo_start_addr`] module"]
pub type I2C_TXFIFO_START_ADDR = crate::Reg<i2c_txfifo_start_addr::I2C_TXFIFO_START_ADDR_SPEC>;
#[doc = "I2C TXFIFO base address register"]
pub mod i2c_txfifo_start_addr;
#[doc = "I2C_RXFIFO_START_ADDR (r) register accessor: I2C RXFIFO base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_rxfifo_start_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_rxfifo_start_addr`] module"]
pub type I2C_RXFIFO_START_ADDR = crate::Reg<i2c_rxfifo_start_addr::I2C_RXFIFO_START_ADDR_SPEC>;
#[doc = "I2C RXFIFO base address register"]
pub mod i2c_rxfifo_start_addr;
