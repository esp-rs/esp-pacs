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
#[doc = "SCL_LOW (rw) register accessor: Configure the low level width of SCL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scl_low`] module"]
pub type SCL_LOW = crate::Reg<scl_low::SCL_LOW_SPEC>;
#[doc = "Configure the low level width of SCL"]
pub mod scl_low;
#[doc = "CTRL (rw) register accessor: Transmission setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Transmission setting"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: RTC I2C status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "RTC I2C status"]
pub mod status;
#[doc = "TO (rw) register accessor: Configure RTC I2C timeout\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`to::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`to::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`to`] module"]
pub type TO = crate::Reg<to::TO_SPEC>;
#[doc = "Configure RTC I2C timeout"]
pub mod to;
#[doc = "SLAVE_ADDR (rw) register accessor: Configure slave address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slave_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slave_addr`] module"]
pub type SLAVE_ADDR = crate::Reg<slave_addr::SLAVE_ADDR_SPEC>;
#[doc = "Configure slave address"]
pub mod slave_addr;
#[doc = "SCL_HIGH (rw) register accessor: Configure the high level width of SCL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scl_high`] module"]
pub type SCL_HIGH = crate::Reg<scl_high::SCL_HIGH_SPEC>;
#[doc = "Configure the high level width of SCL"]
pub mod scl_high;
#[doc = "SDA_DUTY (rw) register accessor: Configure the SDA hold time after a negative SCL edge\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sda_duty::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sda_duty::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sda_duty`] module"]
pub type SDA_DUTY = crate::Reg<sda_duty::SDA_DUTY_SPEC>;
#[doc = "Configure the SDA hold time after a negative SCL edge"]
pub mod sda_duty;
#[doc = "SCL_START_PERIOD (rw) register accessor: Configure the delay between the SDA and SCL negative edge for a start condition\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_start_period::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_start_period::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scl_start_period`] module"]
pub type SCL_START_PERIOD = crate::Reg<scl_start_period::SCL_START_PERIOD_SPEC>;
#[doc = "Configure the delay between the SDA and SCL negative edge for a start condition"]
pub mod scl_start_period;
#[doc = "SCL_STOP_PERIOD (rw) register accessor: Configure the delay between SDA and SCL positive edge for a stop condition\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_stop_period::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_stop_period::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scl_stop_period`] module"]
pub type SCL_STOP_PERIOD = crate::Reg<scl_stop_period::SCL_STOP_PERIOD_SPEC>;
#[doc = "Configure the delay between SDA and SCL positive edge for a stop condition"]
pub mod scl_stop_period;
#[doc = "INT_CLR (w) register accessor: Clear RTC I2C interrupt\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Clear RTC I2C interrupt"]
pub mod int_clr;
#[doc = "INT_RAW (r) register accessor: RTC I2C raw interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "RTC I2C raw interrupt"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: RTC I2C interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "RTC I2C interrupt status"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: Enable RTC I2C interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Enable RTC I2C interrupt"]
pub mod int_ena;
#[doc = "DATA (rw) register accessor: RTC I2C read data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data`] module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "RTC I2C read data"]
pub mod data;
#[doc = "CMD0 (rw) register accessor: RTC I2C Command 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd0`] module"]
pub type CMD0 = crate::Reg<cmd0::CMD0_SPEC>;
#[doc = "RTC I2C Command 0"]
pub mod cmd0;
#[doc = "CMD1 (rw) register accessor: RTC I2C Command 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd1`] module"]
pub type CMD1 = crate::Reg<cmd1::CMD1_SPEC>;
#[doc = "RTC I2C Command 1"]
pub mod cmd1;
#[doc = "CMD2 (rw) register accessor: RTC I2C Command 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd2`] module"]
pub type CMD2 = crate::Reg<cmd2::CMD2_SPEC>;
#[doc = "RTC I2C Command 2"]
pub mod cmd2;
#[doc = "CMD3 (rw) register accessor: RTC I2C Command 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd3`] module"]
pub type CMD3 = crate::Reg<cmd3::CMD3_SPEC>;
#[doc = "RTC I2C Command 3"]
pub mod cmd3;
#[doc = "CMD4 (rw) register accessor: RTC I2C Command 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd4`] module"]
pub type CMD4 = crate::Reg<cmd4::CMD4_SPEC>;
#[doc = "RTC I2C Command 4"]
pub mod cmd4;
#[doc = "CMD5 (rw) register accessor: RTC I2C Command 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd5`] module"]
pub type CMD5 = crate::Reg<cmd5::CMD5_SPEC>;
#[doc = "RTC I2C Command 5"]
pub mod cmd5;
#[doc = "CMD6 (rw) register accessor: RTC I2C Command 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd6`] module"]
pub type CMD6 = crate::Reg<cmd6::CMD6_SPEC>;
#[doc = "RTC I2C Command 6"]
pub mod cmd6;
#[doc = "CMD7 (rw) register accessor: RTC I2C Command 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd7`] module"]
pub type CMD7 = crate::Reg<cmd7::CMD7_SPEC>;
#[doc = "RTC I2C Command 7"]
pub mod cmd7;
#[doc = "CMD8 (rw) register accessor: RTC I2C Command 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd8`] module"]
pub type CMD8 = crate::Reg<cmd8::CMD8_SPEC>;
#[doc = "RTC I2C Command 8"]
pub mod cmd8;
#[doc = "CMD9 (rw) register accessor: RTC I2C Command 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd9`] module"]
pub type CMD9 = crate::Reg<cmd9::CMD9_SPEC>;
#[doc = "RTC I2C Command 9"]
pub mod cmd9;
#[doc = "CMD10 (rw) register accessor: RTC I2C Command 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd10`] module"]
pub type CMD10 = crate::Reg<cmd10::CMD10_SPEC>;
#[doc = "RTC I2C Command 10"]
pub mod cmd10;
#[doc = "CMD11 (rw) register accessor: RTC I2C Command 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd11`] module"]
pub type CMD11 = crate::Reg<cmd11::CMD11_SPEC>;
#[doc = "RTC I2C Command 11"]
pub mod cmd11;
#[doc = "CMD12 (rw) register accessor: RTC I2C Command 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd12`] module"]
pub type CMD12 = crate::Reg<cmd12::CMD12_SPEC>;
#[doc = "RTC I2C Command 12"]
pub mod cmd12;
#[doc = "CMD13 (rw) register accessor: RTC I2C Command 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd13`] module"]
pub type CMD13 = crate::Reg<cmd13::CMD13_SPEC>;
#[doc = "RTC I2C Command 13"]
pub mod cmd13;
#[doc = "CMD14 (rw) register accessor: RTC I2C Command 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd14`] module"]
pub type CMD14 = crate::Reg<cmd14::CMD14_SPEC>;
#[doc = "RTC I2C Command 14"]
pub mod cmd14;
#[doc = "CMD15 (rw) register accessor: RTC I2C Command 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd15`] module"]
pub type CMD15 = crate::Reg<cmd15::CMD15_SPEC>;
#[doc = "RTC I2C Command 15"]
pub mod cmd15;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
