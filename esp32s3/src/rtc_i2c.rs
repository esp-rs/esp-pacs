#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    scl_low: SCL_LOW,
    ctrl: CTRL,
    status: STATUS,
    to: TO,
    slave_addr: SLAVE_ADDR,
    scl_high: SCL_HIGH,
    sda_duty: SDA_DUTY,
    scl_start_period: SCL_START_PERIOD,
    scl_stop_period: SCL_STOP_PERIOD,
    int_clr: INT_CLR,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    data: DATA,
    cmd: [CMD; 16],
    _reserved15: [u8; 0x84],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - configure low scl period"]
    #[inline(always)]
    pub const fn scl_low(&self) -> &SCL_LOW {
        &self.scl_low
    }
    #[doc = "0x04 - configure i2c ctrl"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x08 - get i2c status"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x0c - configure time out"]
    #[inline(always)]
    pub const fn to(&self) -> &TO {
        &self.to
    }
    #[doc = "0x10 - configure slave id"]
    #[inline(always)]
    pub const fn slave_addr(&self) -> &SLAVE_ADDR {
        &self.slave_addr
    }
    #[doc = "0x14 - configure high scl period"]
    #[inline(always)]
    pub const fn scl_high(&self) -> &SCL_HIGH {
        &self.scl_high
    }
    #[doc = "0x18 - configure sda duty"]
    #[inline(always)]
    pub const fn sda_duty(&self) -> &SDA_DUTY {
        &self.sda_duty
    }
    #[doc = "0x1c - configure scl start period"]
    #[inline(always)]
    pub const fn scl_start_period(&self) -> &SCL_START_PERIOD {
        &self.scl_start_period
    }
    #[doc = "0x20 - configure scl stop period"]
    #[inline(always)]
    pub const fn scl_stop_period(&self) -> &SCL_STOP_PERIOD {
        &self.scl_stop_period
    }
    #[doc = "0x24 - interrupt clear register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x28 - interrupt raw register"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x2c - interrupt state register"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x30 - interrupt enable register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x34 - get i2c data status"]
    #[inline(always)]
    pub const fn data(&self) -> &DATA {
        &self.data
    }
    #[doc = "0x38..0x78 - I2C command%s register"]
    #[inline(always)]
    pub const fn cmd(&self, n: usize) -> &CMD {
        &self.cmd[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x38..0x78 - I2C command%s register"]
    #[inline(always)]
    pub fn cmd_iter(&self) -> impl Iterator<Item = &CMD> {
        self.cmd.iter()
    }
    #[doc = "0xfc - version register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "SCL_LOW (rw) register accessor: configure low scl period\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_low`] module"]
pub type SCL_LOW = crate::Reg<scl_low::SCL_LOW_SPEC>;
#[doc = "configure low scl period"]
pub mod scl_low;
#[doc = "CTRL (rw) register accessor: configure i2c ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "configure i2c ctrl"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: get i2c status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "get i2c status"]
pub mod status;
#[doc = "TO (rw) register accessor: configure time out\n\nYou can [`read`](crate::Reg::read) this register and get [`to::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`to::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@to`] module"]
pub type TO = crate::Reg<to::TO_SPEC>;
#[doc = "configure time out"]
pub mod to;
#[doc = "SLAVE_ADDR (rw) register accessor: configure slave id\n\nYou can [`read`](crate::Reg::read) this register and get [`slave_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slave_addr`] module"]
pub type SLAVE_ADDR = crate::Reg<slave_addr::SLAVE_ADDR_SPEC>;
#[doc = "configure slave id"]
pub mod slave_addr;
#[doc = "SCL_HIGH (rw) register accessor: configure high scl period\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_high`] module"]
pub type SCL_HIGH = crate::Reg<scl_high::SCL_HIGH_SPEC>;
#[doc = "configure high scl period"]
pub mod scl_high;
#[doc = "SDA_DUTY (rw) register accessor: configure sda duty\n\nYou can [`read`](crate::Reg::read) this register and get [`sda_duty::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sda_duty::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sda_duty`] module"]
pub type SDA_DUTY = crate::Reg<sda_duty::SDA_DUTY_SPEC>;
#[doc = "configure sda duty"]
pub mod sda_duty;
#[doc = "SCL_START_PERIOD (rw) register accessor: configure scl start period\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_start_period::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_start_period::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_start_period`] module"]
pub type SCL_START_PERIOD = crate::Reg<scl_start_period::SCL_START_PERIOD_SPEC>;
#[doc = "configure scl start period"]
pub mod scl_start_period;
#[doc = "SCL_STOP_PERIOD (rw) register accessor: configure scl stop period\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_stop_period::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_stop_period::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_stop_period`] module"]
pub type SCL_STOP_PERIOD = crate::Reg<scl_stop_period::SCL_STOP_PERIOD_SPEC>;
#[doc = "configure scl stop period"]
pub mod scl_stop_period;
#[doc = "INT_CLR (w) register accessor: interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "interrupt clear register"]
pub mod int_clr;
#[doc = "INT_RAW (r) register accessor: interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "interrupt raw register"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: interrupt state register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "interrupt state register"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "interrupt enable register"]
pub mod int_ena;
#[doc = "DATA (rw) register accessor: get i2c data status\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`] module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "get i2c data status"]
pub mod data;
#[doc = "CMD (rw) register accessor: I2C command%s register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "I2C command%s register"]
pub mod cmd;
pub use crate::aes::date;
pub use crate::aes::DATE;
