#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub scl_low_period: SCL_LOW_PERIOD,
    #[doc = "0x04 - "]
    pub ctrl: CTRL,
    #[doc = "0x08 - "]
    pub debug_status: DEBUG_STATUS,
    #[doc = "0x0c - "]
    pub timeout: TIMEOUT,
    #[doc = "0x10 - "]
    pub slave_addr: SLAVE_ADDR,
    _reserved5: [u8; 0x08],
    #[doc = "0x1c - "]
    pub data: DATA,
    #[doc = "0x20 - "]
    pub int_raw: INT_RAW,
    #[doc = "0x24 - "]
    pub int_clr: INT_CLR,
    #[doc = "0x28 - "]
    pub int_en: INT_EN,
    #[doc = "0x2c - "]
    pub int_st: INT_ST,
    #[doc = "0x30 - "]
    pub sda_duty: SDA_DUTY,
    _reserved11: [u8; 0x04],
    #[doc = "0x38 - "]
    pub scl_high_period: SCL_HIGH_PERIOD,
    _reserved12: [u8; 0x04],
    #[doc = "0x40 - "]
    pub scl_start_period: SCL_START_PERIOD,
    #[doc = "0x44 - "]
    pub scl_stop_period: SCL_STOP_PERIOD,
    #[doc = "0x48 - "]
    pub cmd: CMD,
}
#[doc = "SCL_LOW_PERIOD (rw) register accessor: an alias for `Reg<SCL_LOW_PERIOD_SPEC>`"]
pub type SCL_LOW_PERIOD = crate::Reg<scl_low_period::SCL_LOW_PERIOD_SPEC>;
#[doc = ""]
pub mod scl_low_period;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = ""]
pub mod ctrl;
#[doc = "DEBUG_STATUS (rw) register accessor: an alias for `Reg<DEBUG_STATUS_SPEC>`"]
pub type DEBUG_STATUS = crate::Reg<debug_status::DEBUG_STATUS_SPEC>;
#[doc = ""]
pub mod debug_status;
#[doc = "TIMEOUT (rw) register accessor: an alias for `Reg<TIMEOUT_SPEC>`"]
pub type TIMEOUT = crate::Reg<timeout::TIMEOUT_SPEC>;
#[doc = ""]
pub mod timeout;
#[doc = "SLAVE_ADDR (rw) register accessor: an alias for `Reg<SLAVE_ADDR_SPEC>`"]
pub type SLAVE_ADDR = crate::Reg<slave_addr::SLAVE_ADDR_SPEC>;
#[doc = ""]
pub mod slave_addr;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = ""]
pub mod data;
#[doc = "INT_RAW (rw) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = ""]
pub mod int_raw;
#[doc = "INT_CLR (rw) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = ""]
pub mod int_clr;
#[doc = "INT_EN (rw) register accessor: an alias for `Reg<INT_EN_SPEC>`"]
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
#[doc = ""]
pub mod int_en;
#[doc = "INT_ST (rw) register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = ""]
pub mod int_st;
#[doc = "SDA_DUTY (rw) register accessor: an alias for `Reg<SDA_DUTY_SPEC>`"]
pub type SDA_DUTY = crate::Reg<sda_duty::SDA_DUTY_SPEC>;
#[doc = ""]
pub mod sda_duty;
#[doc = "SCL_HIGH_PERIOD (rw) register accessor: an alias for `Reg<SCL_HIGH_PERIOD_SPEC>`"]
pub type SCL_HIGH_PERIOD = crate::Reg<scl_high_period::SCL_HIGH_PERIOD_SPEC>;
#[doc = ""]
pub mod scl_high_period;
#[doc = "SCL_START_PERIOD (rw) register accessor: an alias for `Reg<SCL_START_PERIOD_SPEC>`"]
pub type SCL_START_PERIOD = crate::Reg<scl_start_period::SCL_START_PERIOD_SPEC>;
#[doc = ""]
pub mod scl_start_period;
#[doc = "SCL_STOP_PERIOD (rw) register accessor: an alias for `Reg<SCL_STOP_PERIOD_SPEC>`"]
pub type SCL_STOP_PERIOD = crate::Reg<scl_stop_period::SCL_STOP_PERIOD_SPEC>;
#[doc = ""]
pub mod scl_stop_period;
#[doc = "CMD (rw) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = ""]
pub mod cmd;
