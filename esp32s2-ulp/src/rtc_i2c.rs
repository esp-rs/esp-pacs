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
    #[doc = "0x38 - RTC I2C Command 0"]
    pub cmd0: CMD0,
    #[doc = "0x3c - RTC I2C Command 1"]
    pub cmd1: CMD1,
    #[doc = "0x40 - RTC I2C Command 2"]
    pub cmd2: CMD2,
    #[doc = "0x44 - RTC I2C Command 3"]
    pub cmd3: CMD3,
    #[doc = "0x48 - RTC I2C Command 4"]
    pub cmd4: CMD4,
    #[doc = "0x4c - RTC I2C Command 5"]
    pub cmd5: CMD5,
    #[doc = "0x50 - RTC I2C Command 6"]
    pub cmd6: CMD6,
    #[doc = "0x54 - RTC I2C Command 7"]
    pub cmd7: CMD7,
    #[doc = "0x58 - RTC I2C Command 8"]
    pub cmd8: CMD8,
    #[doc = "0x5c - RTC I2C Command 9"]
    pub cmd9: CMD9,
    #[doc = "0x60 - RTC I2C Command 10"]
    pub cmd10: CMD10,
    #[doc = "0x64 - RTC I2C Command 11"]
    pub cmd11: CMD11,
    #[doc = "0x68 - RTC I2C Command 12"]
    pub cmd12: CMD12,
    #[doc = "0x6c - RTC I2C Command 13"]
    pub cmd13: CMD13,
    #[doc = "0x70 - RTC I2C Command 14"]
    pub cmd14: CMD14,
    #[doc = "0x74 - RTC I2C Command 15"]
    pub cmd15: CMD15,
    _reserved30: [u8; 0x84],
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
#[doc = "CMD0 (rw) register accessor: an alias for `Reg<CMD0_SPEC>`"]
pub type CMD0 = crate::Reg<cmd0::CMD0_SPEC>;
#[doc = "RTC I2C Command 0"]
pub mod cmd0;
#[doc = "CMD1 (rw) register accessor: an alias for `Reg<CMD1_SPEC>`"]
pub type CMD1 = crate::Reg<cmd1::CMD1_SPEC>;
#[doc = "RTC I2C Command 1"]
pub mod cmd1;
#[doc = "CMD2 (rw) register accessor: an alias for `Reg<CMD2_SPEC>`"]
pub type CMD2 = crate::Reg<cmd2::CMD2_SPEC>;
#[doc = "RTC I2C Command 2"]
pub mod cmd2;
#[doc = "CMD3 (rw) register accessor: an alias for `Reg<CMD3_SPEC>`"]
pub type CMD3 = crate::Reg<cmd3::CMD3_SPEC>;
#[doc = "RTC I2C Command 3"]
pub mod cmd3;
#[doc = "CMD4 (rw) register accessor: an alias for `Reg<CMD4_SPEC>`"]
pub type CMD4 = crate::Reg<cmd4::CMD4_SPEC>;
#[doc = "RTC I2C Command 4"]
pub mod cmd4;
#[doc = "CMD5 (rw) register accessor: an alias for `Reg<CMD5_SPEC>`"]
pub type CMD5 = crate::Reg<cmd5::CMD5_SPEC>;
#[doc = "RTC I2C Command 5"]
pub mod cmd5;
#[doc = "CMD6 (rw) register accessor: an alias for `Reg<CMD6_SPEC>`"]
pub type CMD6 = crate::Reg<cmd6::CMD6_SPEC>;
#[doc = "RTC I2C Command 6"]
pub mod cmd6;
#[doc = "CMD7 (rw) register accessor: an alias for `Reg<CMD7_SPEC>`"]
pub type CMD7 = crate::Reg<cmd7::CMD7_SPEC>;
#[doc = "RTC I2C Command 7"]
pub mod cmd7;
#[doc = "CMD8 (rw) register accessor: an alias for `Reg<CMD8_SPEC>`"]
pub type CMD8 = crate::Reg<cmd8::CMD8_SPEC>;
#[doc = "RTC I2C Command 8"]
pub mod cmd8;
#[doc = "CMD9 (rw) register accessor: an alias for `Reg<CMD9_SPEC>`"]
pub type CMD9 = crate::Reg<cmd9::CMD9_SPEC>;
#[doc = "RTC I2C Command 9"]
pub mod cmd9;
#[doc = "CMD10 (rw) register accessor: an alias for `Reg<CMD10_SPEC>`"]
pub type CMD10 = crate::Reg<cmd10::CMD10_SPEC>;
#[doc = "RTC I2C Command 10"]
pub mod cmd10;
#[doc = "CMD11 (rw) register accessor: an alias for `Reg<CMD11_SPEC>`"]
pub type CMD11 = crate::Reg<cmd11::CMD11_SPEC>;
#[doc = "RTC I2C Command 11"]
pub mod cmd11;
#[doc = "CMD12 (rw) register accessor: an alias for `Reg<CMD12_SPEC>`"]
pub type CMD12 = crate::Reg<cmd12::CMD12_SPEC>;
#[doc = "RTC I2C Command 12"]
pub mod cmd12;
#[doc = "CMD13 (rw) register accessor: an alias for `Reg<CMD13_SPEC>`"]
pub type CMD13 = crate::Reg<cmd13::CMD13_SPEC>;
#[doc = "RTC I2C Command 13"]
pub mod cmd13;
#[doc = "CMD14 (rw) register accessor: an alias for `Reg<CMD14_SPEC>`"]
pub type CMD14 = crate::Reg<cmd14::CMD14_SPEC>;
#[doc = "RTC I2C Command 14"]
pub mod cmd14;
#[doc = "CMD15 (rw) register accessor: an alias for `Reg<CMD15_SPEC>`"]
pub type CMD15 = crate::Reg<cmd15::CMD15_SPEC>;
#[doc = "RTC I2C Command 15"]
pub mod cmd15;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
