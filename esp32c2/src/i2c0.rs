#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configures the low level width of the SCL Clock"]
    pub scl_low_period: crate::Reg<scl_low_period::SCL_LOW_PERIOD_SPEC>,
    #[doc = "0x04 - Transmission setting"]
    pub ctr: crate::Reg<ctr::CTR_SPEC>,
    #[doc = "0x08 - Describe I2C work status."]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x0c - Setting time out control for receiving data."]
    pub to: crate::Reg<to::TO_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x14 - FIFO status register."]
    pub fifo_st: crate::Reg<fifo_st::FIFO_ST_SPEC>,
    #[doc = "0x18 - FIFO configuration register."]
    pub fifo_conf: crate::Reg<fifo_conf::FIFO_CONF_SPEC>,
    #[doc = "0x1c - Rx FIFO read data."]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x20 - Raw interrupt status"]
    pub int_raw: crate::Reg<int_raw::INT_RAW_SPEC>,
    #[doc = "0x24 - Interrupt clear bits"]
    pub int_clr: crate::Reg<int_clr::INT_CLR_SPEC>,
    #[doc = "0x28 - Interrupt enable bits"]
    pub int_ena: crate::Reg<int_ena::INT_ENA_SPEC>,
    #[doc = "0x2c - Status of captured I2C communication events"]
    pub int_status: crate::Reg<int_status::INT_STATUS_SPEC>,
    #[doc = "0x30 - Configures the hold time after a negative SCL edge."]
    pub sda_hold: crate::Reg<sda_hold::SDA_HOLD_SPEC>,
    #[doc = "0x34 - Configures the sample time after a positive SCL edge."]
    pub sda_sample: crate::Reg<sda_sample::SDA_SAMPLE_SPEC>,
    #[doc = "0x38 - Configures the high level width of SCL"]
    pub scl_high_period: crate::Reg<scl_high_period::SCL_HIGH_PERIOD_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x40 - Configures the delay between the SDA and SCL negative edge for a start condition"]
    pub scl_start_hold: crate::Reg<scl_start_hold::SCL_START_HOLD_SPEC>,
    #[doc = "0x44 - Configures the delay between the positive edge of SCL and the negative edge of SDA"]
    pub scl_rstart_setup: crate::Reg<scl_rstart_setup::SCL_RSTART_SETUP_SPEC>,
    #[doc = "0x48 - Configures the delay after the SCL clock edge for a stop condition"]
    pub scl_stop_hold: crate::Reg<scl_stop_hold::SCL_STOP_HOLD_SPEC>,
    #[doc = "0x4c - Configures the delay between the SDA and SCL positive edge for a stop condition"]
    pub scl_stop_setup: crate::Reg<scl_stop_setup::SCL_STOP_SETUP_SPEC>,
    #[doc = "0x50 - SCL and SDA filter configuration register"]
    pub filter_cfg: crate::Reg<filter_cfg::FILTER_CFG_SPEC>,
    #[doc = "0x54 - I2C CLK configuration register"]
    pub clk_conf: crate::Reg<clk_conf::CLK_CONF_SPEC>,
    #[doc = "0x58 - I2C command register 0"]
    pub comd0: crate::Reg<comd0::COMD0_SPEC>,
    #[doc = "0x5c - I2C command register 1"]
    pub comd1: crate::Reg<comd1::COMD1_SPEC>,
    #[doc = "0x60 - I2C command register 2"]
    pub comd2: crate::Reg<comd2::COMD2_SPEC>,
    #[doc = "0x64 - I2C command register 3"]
    pub comd3: crate::Reg<comd3::COMD3_SPEC>,
    #[doc = "0x68 - I2C command register 4"]
    pub comd4: crate::Reg<comd4::COMD4_SPEC>,
    #[doc = "0x6c - I2C command register 5"]
    pub comd5: crate::Reg<comd5::COMD5_SPEC>,
    #[doc = "0x70 - I2C command register 6"]
    pub comd6: crate::Reg<comd6::COMD6_SPEC>,
    #[doc = "0x74 - I2C command register 7"]
    pub comd7: crate::Reg<comd7::COMD7_SPEC>,
    #[doc = "0x78 - SCL status time out register"]
    pub scl_st_time_out: crate::Reg<scl_st_time_out::SCL_ST_TIME_OUT_SPEC>,
    #[doc = "0x7c - SCL main status time out register"]
    pub scl_main_st_time_out: crate::Reg<scl_main_st_time_out::SCL_MAIN_ST_TIME_OUT_SPEC>,
    #[doc = "0x80 - Power configuration register"]
    pub scl_sp_conf: crate::Reg<scl_sp_conf::SCL_SP_CONF_SPEC>,
    _reserved31: [u8; 0x74],
    #[doc = "0xf8 - Version register"]
    pub date: crate::Reg<date::DATE_SPEC>,
    _reserved32: [u8; 0x04],
    #[doc = "0x100 - I2C TXFIFO base address register"]
    pub txfifo_start_addr: crate::Reg<txfifo_start_addr::TXFIFO_START_ADDR_SPEC>,
    _reserved33: [u8; 0x7c],
    #[doc = "0x180 - I2C RXFIFO base address register"]
    pub rxfifo_start_addr: crate::Reg<rxfifo_start_addr::RXFIFO_START_ADDR_SPEC>,
}
#[doc = "SCL_LOW_PERIOD register accessor: an alias for `Reg<SCL_LOW_PERIOD_SPEC>`"]
pub type SCL_LOW_PERIOD = crate::Reg<scl_low_period::SCL_LOW_PERIOD_SPEC>;
#[doc = "Configures the low level width of the SCL Clock"]
pub mod scl_low_period;
#[doc = "CTR register accessor: an alias for `Reg<CTR_SPEC>`"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = "Transmission setting"]
pub mod ctr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Describe I2C work status."]
pub mod sr;
#[doc = "TO register accessor: an alias for `Reg<TO_SPEC>`"]
pub type TO = crate::Reg<to::TO_SPEC>;
#[doc = "Setting time out control for receiving data."]
pub mod to;
#[doc = "FIFO_ST register accessor: an alias for `Reg<FIFO_ST_SPEC>`"]
pub type FIFO_ST = crate::Reg<fifo_st::FIFO_ST_SPEC>;
#[doc = "FIFO status register."]
pub mod fifo_st;
#[doc = "FIFO_CONF register accessor: an alias for `Reg<FIFO_CONF_SPEC>`"]
pub type FIFO_CONF = crate::Reg<fifo_conf::FIFO_CONF_SPEC>;
#[doc = "FIFO configuration register."]
pub mod fifo_conf;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Rx FIFO read data."]
pub mod data;
#[doc = "INT_RAW register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Raw interrupt status"]
pub mod int_raw;
#[doc = "INT_CLR register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt clear bits"]
pub mod int_clr;
#[doc = "INT_ENA register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt enable bits"]
pub mod int_ena;
#[doc = "INT_STATUS register accessor: an alias for `Reg<INT_STATUS_SPEC>`"]
pub type INT_STATUS = crate::Reg<int_status::INT_STATUS_SPEC>;
#[doc = "Status of captured I2C communication events"]
pub mod int_status;
#[doc = "SDA_HOLD register accessor: an alias for `Reg<SDA_HOLD_SPEC>`"]
pub type SDA_HOLD = crate::Reg<sda_hold::SDA_HOLD_SPEC>;
#[doc = "Configures the hold time after a negative SCL edge."]
pub mod sda_hold;
#[doc = "SDA_SAMPLE register accessor: an alias for `Reg<SDA_SAMPLE_SPEC>`"]
pub type SDA_SAMPLE = crate::Reg<sda_sample::SDA_SAMPLE_SPEC>;
#[doc = "Configures the sample time after a positive SCL edge."]
pub mod sda_sample;
#[doc = "SCL_HIGH_PERIOD register accessor: an alias for `Reg<SCL_HIGH_PERIOD_SPEC>`"]
pub type SCL_HIGH_PERIOD = crate::Reg<scl_high_period::SCL_HIGH_PERIOD_SPEC>;
#[doc = "Configures the high level width of SCL"]
pub mod scl_high_period;
#[doc = "SCL_START_HOLD register accessor: an alias for `Reg<SCL_START_HOLD_SPEC>`"]
pub type SCL_START_HOLD = crate::Reg<scl_start_hold::SCL_START_HOLD_SPEC>;
#[doc = "Configures the delay between the SDA and SCL negative edge for a start condition"]
pub mod scl_start_hold;
#[doc = "SCL_RSTART_SETUP register accessor: an alias for `Reg<SCL_RSTART_SETUP_SPEC>`"]
pub type SCL_RSTART_SETUP = crate::Reg<scl_rstart_setup::SCL_RSTART_SETUP_SPEC>;
#[doc = "Configures the delay between the positive edge of SCL and the negative edge of SDA"]
pub mod scl_rstart_setup;
#[doc = "SCL_STOP_HOLD register accessor: an alias for `Reg<SCL_STOP_HOLD_SPEC>`"]
pub type SCL_STOP_HOLD = crate::Reg<scl_stop_hold::SCL_STOP_HOLD_SPEC>;
#[doc = "Configures the delay after the SCL clock edge for a stop condition"]
pub mod scl_stop_hold;
#[doc = "SCL_STOP_SETUP register accessor: an alias for `Reg<SCL_STOP_SETUP_SPEC>`"]
pub type SCL_STOP_SETUP = crate::Reg<scl_stop_setup::SCL_STOP_SETUP_SPEC>;
#[doc = "Configures the delay between the SDA and SCL positive edge for a stop condition"]
pub mod scl_stop_setup;
#[doc = "FILTER_CFG register accessor: an alias for `Reg<FILTER_CFG_SPEC>`"]
pub type FILTER_CFG = crate::Reg<filter_cfg::FILTER_CFG_SPEC>;
#[doc = "SCL and SDA filter configuration register"]
pub mod filter_cfg;
#[doc = "CLK_CONF register accessor: an alias for `Reg<CLK_CONF_SPEC>`"]
pub type CLK_CONF = crate::Reg<clk_conf::CLK_CONF_SPEC>;
#[doc = "I2C CLK configuration register"]
pub mod clk_conf;
#[doc = "COMD0 register accessor: an alias for `Reg<COMD0_SPEC>`"]
pub type COMD0 = crate::Reg<comd0::COMD0_SPEC>;
#[doc = "I2C command register 0"]
pub mod comd0;
#[doc = "COMD1 register accessor: an alias for `Reg<COMD1_SPEC>`"]
pub type COMD1 = crate::Reg<comd1::COMD1_SPEC>;
#[doc = "I2C command register 1"]
pub mod comd1;
#[doc = "COMD2 register accessor: an alias for `Reg<COMD2_SPEC>`"]
pub type COMD2 = crate::Reg<comd2::COMD2_SPEC>;
#[doc = "I2C command register 2"]
pub mod comd2;
#[doc = "COMD3 register accessor: an alias for `Reg<COMD3_SPEC>`"]
pub type COMD3 = crate::Reg<comd3::COMD3_SPEC>;
#[doc = "I2C command register 3"]
pub mod comd3;
#[doc = "COMD4 register accessor: an alias for `Reg<COMD4_SPEC>`"]
pub type COMD4 = crate::Reg<comd4::COMD4_SPEC>;
#[doc = "I2C command register 4"]
pub mod comd4;
#[doc = "COMD5 register accessor: an alias for `Reg<COMD5_SPEC>`"]
pub type COMD5 = crate::Reg<comd5::COMD5_SPEC>;
#[doc = "I2C command register 5"]
pub mod comd5;
#[doc = "COMD6 register accessor: an alias for `Reg<COMD6_SPEC>`"]
pub type COMD6 = crate::Reg<comd6::COMD6_SPEC>;
#[doc = "I2C command register 6"]
pub mod comd6;
#[doc = "COMD7 register accessor: an alias for `Reg<COMD7_SPEC>`"]
pub type COMD7 = crate::Reg<comd7::COMD7_SPEC>;
#[doc = "I2C command register 7"]
pub mod comd7;
#[doc = "SCL_ST_TIME_OUT register accessor: an alias for `Reg<SCL_ST_TIME_OUT_SPEC>`"]
pub type SCL_ST_TIME_OUT = crate::Reg<scl_st_time_out::SCL_ST_TIME_OUT_SPEC>;
#[doc = "SCL status time out register"]
pub mod scl_st_time_out;
#[doc = "SCL_MAIN_ST_TIME_OUT register accessor: an alias for `Reg<SCL_MAIN_ST_TIME_OUT_SPEC>`"]
pub type SCL_MAIN_ST_TIME_OUT = crate::Reg<scl_main_st_time_out::SCL_MAIN_ST_TIME_OUT_SPEC>;
#[doc = "SCL main status time out register"]
pub mod scl_main_st_time_out;
#[doc = "SCL_SP_CONF register accessor: an alias for `Reg<SCL_SP_CONF_SPEC>`"]
pub type SCL_SP_CONF = crate::Reg<scl_sp_conf::SCL_SP_CONF_SPEC>;
#[doc = "Power configuration register"]
pub mod scl_sp_conf;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version register"]
pub mod date;
#[doc = "TXFIFO_START_ADDR register accessor: an alias for `Reg<TXFIFO_START_ADDR_SPEC>`"]
pub type TXFIFO_START_ADDR = crate::Reg<txfifo_start_addr::TXFIFO_START_ADDR_SPEC>;
#[doc = "I2C TXFIFO base address register"]
pub mod txfifo_start_addr;
#[doc = "RXFIFO_START_ADDR register accessor: an alias for `Reg<RXFIFO_START_ADDR_SPEC>`"]
pub type RXFIFO_START_ADDR = crate::Reg<rxfifo_start_addr::RXFIFO_START_ADDR_SPEC>;
#[doc = "I2C RXFIFO base address register"]
pub mod rxfifo_start_addr;
