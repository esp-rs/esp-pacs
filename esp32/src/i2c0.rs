#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub scl_low_period: SCL_LOW_PERIOD,
    #[doc = "0x04 - "]
    pub ctr: CTR,
    #[doc = "0x08 - "]
    pub sr: SR,
    #[doc = "0x0c - "]
    pub to: TO,
    #[doc = "0x10 - "]
    pub slave_addr: SLAVE_ADDR,
    #[doc = "0x14 - "]
    pub rxfifo_st: RXFIFO_ST,
    #[doc = "0x18 - "]
    pub fifo_conf: FIFO_CONF,
    #[doc = "0x1c - "]
    pub data: DATA,
    #[doc = "0x20 - "]
    pub int_raw: INT_RAW,
    #[doc = "0x24 - "]
    pub int_clr: INT_CLR,
    #[doc = "0x28 - "]
    pub int_ena: INT_ENA,
    #[doc = "0x2c - "]
    pub int_status: INT_STATUS,
    #[doc = "0x30 - "]
    pub sda_hold: SDA_HOLD,
    #[doc = "0x34 - "]
    pub sda_sample: SDA_SAMPLE,
    #[doc = "0x38 - "]
    pub scl_high_period: SCL_HIGH_PERIOD,
    _reserved15: [u8; 0x04],
    #[doc = "0x40 - "]
    pub scl_start_hold: SCL_START_HOLD,
    #[doc = "0x44 - "]
    pub scl_rstart_setup: SCL_RSTART_SETUP,
    #[doc = "0x48 - "]
    pub scl_stop_hold: SCL_STOP_HOLD,
    #[doc = "0x4c - "]
    pub scl_stop_setup: SCL_STOP_SETUP,
    #[doc = "0x50 - "]
    pub scl_filter_cfg: SCL_FILTER_CFG,
    #[doc = "0x54 - "]
    pub sda_filter_cfg: SDA_FILTER_CFG,
    #[doc = "0x58..0x98 - "]
    pub comd: [COMD; 16],
    _reserved22: [u8; 0x60],
    #[doc = "0xf8 - "]
    pub date: DATE,
    _reserved23: [u8; 0x04],
    #[doc = "0x100 - "]
    pub fifo_start_addr: FIFO_START_ADDR,
}
#[doc = "SCL_LOW_PERIOD (rw) register accessor: an alias for `Reg<SCL_LOW_PERIOD_SPEC>`"]
pub type SCL_LOW_PERIOD = crate::Reg<scl_low_period::SCL_LOW_PERIOD_SPEC>;
#[doc = ""]
pub mod scl_low_period;
#[doc = "CTR (rw) register accessor: an alias for `Reg<CTR_SPEC>`"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = ""]
pub mod ctr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = ""]
pub mod sr;
#[doc = "TO (rw) register accessor: an alias for `Reg<TO_SPEC>`"]
pub type TO = crate::Reg<to::TO_SPEC>;
#[doc = ""]
pub mod to;
#[doc = "SLAVE_ADDR (rw) register accessor: an alias for `Reg<SLAVE_ADDR_SPEC>`"]
pub type SLAVE_ADDR = crate::Reg<slave_addr::SLAVE_ADDR_SPEC>;
#[doc = ""]
pub mod slave_addr;
#[doc = "RXFIFO_ST (r) register accessor: an alias for `Reg<RXFIFO_ST_SPEC>`"]
pub type RXFIFO_ST = crate::Reg<rxfifo_st::RXFIFO_ST_SPEC>;
#[doc = ""]
pub mod rxfifo_st;
#[doc = "FIFO_CONF (rw) register accessor: an alias for `Reg<FIFO_CONF_SPEC>`"]
pub type FIFO_CONF = crate::Reg<fifo_conf::FIFO_CONF_SPEC>;
#[doc = ""]
pub mod fifo_conf;
#[doc = "DATA (r) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = ""]
pub mod data;
#[doc = "INT_RAW (r) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = ""]
pub mod int_raw;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = ""]
pub mod int_clr;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = ""]
pub mod int_ena;
#[doc = "INT_STATUS (r) register accessor: an alias for `Reg<INT_STATUS_SPEC>`"]
pub type INT_STATUS = crate::Reg<int_status::INT_STATUS_SPEC>;
#[doc = ""]
pub mod int_status;
#[doc = "SDA_HOLD (rw) register accessor: an alias for `Reg<SDA_HOLD_SPEC>`"]
pub type SDA_HOLD = crate::Reg<sda_hold::SDA_HOLD_SPEC>;
#[doc = ""]
pub mod sda_hold;
#[doc = "SDA_SAMPLE (rw) register accessor: an alias for `Reg<SDA_SAMPLE_SPEC>`"]
pub type SDA_SAMPLE = crate::Reg<sda_sample::SDA_SAMPLE_SPEC>;
#[doc = ""]
pub mod sda_sample;
#[doc = "SCL_HIGH_PERIOD (rw) register accessor: an alias for `Reg<SCL_HIGH_PERIOD_SPEC>`"]
pub type SCL_HIGH_PERIOD = crate::Reg<scl_high_period::SCL_HIGH_PERIOD_SPEC>;
#[doc = ""]
pub mod scl_high_period;
#[doc = "SCL_START_HOLD (rw) register accessor: an alias for `Reg<SCL_START_HOLD_SPEC>`"]
pub type SCL_START_HOLD = crate::Reg<scl_start_hold::SCL_START_HOLD_SPEC>;
#[doc = ""]
pub mod scl_start_hold;
#[doc = "SCL_RSTART_SETUP (rw) register accessor: an alias for `Reg<SCL_RSTART_SETUP_SPEC>`"]
pub type SCL_RSTART_SETUP = crate::Reg<scl_rstart_setup::SCL_RSTART_SETUP_SPEC>;
#[doc = ""]
pub mod scl_rstart_setup;
#[doc = "SCL_STOP_HOLD (rw) register accessor: an alias for `Reg<SCL_STOP_HOLD_SPEC>`"]
pub type SCL_STOP_HOLD = crate::Reg<scl_stop_hold::SCL_STOP_HOLD_SPEC>;
#[doc = ""]
pub mod scl_stop_hold;
#[doc = "SCL_STOP_SETUP (rw) register accessor: an alias for `Reg<SCL_STOP_SETUP_SPEC>`"]
pub type SCL_STOP_SETUP = crate::Reg<scl_stop_setup::SCL_STOP_SETUP_SPEC>;
#[doc = ""]
pub mod scl_stop_setup;
#[doc = "SCL_FILTER_CFG (rw) register accessor: an alias for `Reg<SCL_FILTER_CFG_SPEC>`"]
pub type SCL_FILTER_CFG = crate::Reg<scl_filter_cfg::SCL_FILTER_CFG_SPEC>;
#[doc = ""]
pub mod scl_filter_cfg;
#[doc = "SDA_FILTER_CFG (rw) register accessor: an alias for `Reg<SDA_FILTER_CFG_SPEC>`"]
pub type SDA_FILTER_CFG = crate::Reg<sda_filter_cfg::SDA_FILTER_CFG_SPEC>;
#[doc = ""]
pub mod sda_filter_cfg;
#[doc = "COMD (rw) register accessor: an alias for `Reg<COMD_SPEC>`"]
pub type COMD = crate::Reg<comd::COMD_SPEC>;
#[doc = ""]
pub mod comd;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
#[doc = "FIFO_START_ADDR (rw) register accessor: an alias for `Reg<FIFO_START_ADDR_SPEC>`"]
pub type FIFO_START_ADDR = crate::Reg<fifo_start_addr::FIFO_START_ADDR_SPEC>;
#[doc = ""]
pub mod fifo_start_addr;
