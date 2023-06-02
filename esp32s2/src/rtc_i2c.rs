#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Configure the low level width of SCL"]
    pub scl_low: SCL_LOW,
    #[doc = "0x04 - Transmission setting"]
    pub ctrl: CTRL,
    #[doc = "0x08 - RTC I2C status"]
    pub status: STATUS,
    #[doc = "0x0c - Configure RTC I2C timeout"]
    pub to: TO,
    #[doc = "0x10 - Configure slave address"]
    pub slave_addr: SLAVE_ADDR,
    #[doc = "0x14 - Configure the high level width of SCL"]
    pub scl_high: SCL_HIGH,
    #[doc = "0x18 - Configure the SDA hold time after a negative SCL edge"]
    pub sda_duty: SDA_DUTY,
    #[doc = "0x1c - Configure the delay between the SDA and SCL negative edge for a start condition"]
    pub scl_start_period: SCL_START_PERIOD,
    #[doc = "0x20 - Configure the delay between SDA and SCL positive edge for a stop condition"]
    pub scl_stop_period: SCL_STOP_PERIOD,
    #[doc = "0x24 - Clear RTC I2C interrupt"]
    pub int_clr: INT_CLR,
    #[doc = "0x28 - RTC I2C raw interrupt"]
    pub int_raw: INT_RAW,
    #[doc = "0x2c - RTC I2C interrupt status"]
    pub int_st: INT_ST,
    #[doc = "0x30 - Enable RTC I2C interrupt"]
    pub int_ena: INT_ENA,
    #[doc = "0x34 - RTC I2C read data"]
    pub data: DATA,
    #[doc = "0x38..0x78 - RTC I2C Command %s"]
    pub cmd: [CMD; 16],
    _reserved15: [u8; 0x84],
    #[doc = "0xfc - Version control register"]
    pub date: DATE,
}
#[doc = "SCL_LOW (rw) register accessor: an alias for `Reg<SCL_LOW_SPEC>`"]
pub type SCL_LOW = crate::Reg<scl_low::SCL_LOW_SPEC>;
#[doc = "Configure the low level width of SCL"]
pub mod scl_low;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Transmission setting"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "RTC I2C status"]
pub mod status;
#[doc = "TO (rw) register accessor: an alias for `Reg<TO_SPEC>`"]
pub type TO = crate::Reg<to::TO_SPEC>;
#[doc = "Configure RTC I2C timeout"]
pub mod to;
#[doc = "SLAVE_ADDR (rw) register accessor: an alias for `Reg<SLAVE_ADDR_SPEC>`"]
pub type SLAVE_ADDR = crate::Reg<slave_addr::SLAVE_ADDR_SPEC>;
#[doc = "Configure slave address"]
pub mod slave_addr;
#[doc = "SCL_HIGH (rw) register accessor: an alias for `Reg<SCL_HIGH_SPEC>`"]
pub type SCL_HIGH = crate::Reg<scl_high::SCL_HIGH_SPEC>;
#[doc = "Configure the high level width of SCL"]
pub mod scl_high;
#[doc = "SDA_DUTY (rw) register accessor: an alias for `Reg<SDA_DUTY_SPEC>`"]
pub type SDA_DUTY = crate::Reg<sda_duty::SDA_DUTY_SPEC>;
#[doc = "Configure the SDA hold time after a negative SCL edge"]
pub mod sda_duty;
#[doc = "SCL_START_PERIOD (rw) register accessor: an alias for `Reg<SCL_START_PERIOD_SPEC>`"]
pub type SCL_START_PERIOD = crate::Reg<scl_start_period::SCL_START_PERIOD_SPEC>;
#[doc = "Configure the delay between the SDA and SCL negative edge for a start condition"]
pub mod scl_start_period;
#[doc = "SCL_STOP_PERIOD (rw) register accessor: an alias for `Reg<SCL_STOP_PERIOD_SPEC>`"]
pub type SCL_STOP_PERIOD = crate::Reg<scl_stop_period::SCL_STOP_PERIOD_SPEC>;
#[doc = "Configure the delay between SDA and SCL positive edge for a stop condition"]
pub mod scl_stop_period;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Clear RTC I2C interrupt"]
pub mod int_clr;
#[doc = "INT_RAW (r) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "RTC I2C raw interrupt"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "RTC I2C interrupt status"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Enable RTC I2C interrupt"]
pub mod int_ena;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "RTC I2C read data"]
pub mod data;
#[doc = "CMD (rw) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "RTC I2C Command %s"]
pub mod cmd;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
