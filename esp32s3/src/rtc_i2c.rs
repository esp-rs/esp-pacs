#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
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
    cmd0: CMD0,
    cmd1: CMD1,
    cmd2: CMD2,
    cmd3: CMD3,
    cmd4: CMD4,
    cmd5: CMD5,
    cmd6: CMD6,
    cmd7: CMD7,
    cmd8: CMD8,
    cmd9: CMD9,
    cmd10: CMD10,
    cmd11: CMD11,
    cmd12: CMD12,
    cmd13: CMD13,
    cmd14: CMD14,
    cmd15: CMD15,
    _reserved30: [u8; 0x84],
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
    #[doc = "0x38 - i2c commond0 register"]
    #[inline(always)]
    pub const fn cmd0(&self) -> &CMD0 {
        &self.cmd0
    }
    #[doc = "0x3c - i2c commond1 register"]
    #[inline(always)]
    pub const fn cmd1(&self) -> &CMD1 {
        &self.cmd1
    }
    #[doc = "0x40 - i2c commond2 register"]
    #[inline(always)]
    pub const fn cmd2(&self) -> &CMD2 {
        &self.cmd2
    }
    #[doc = "0x44 - i2c commond3 register"]
    #[inline(always)]
    pub const fn cmd3(&self) -> &CMD3 {
        &self.cmd3
    }
    #[doc = "0x48 - i2c commond4 register"]
    #[inline(always)]
    pub const fn cmd4(&self) -> &CMD4 {
        &self.cmd4
    }
    #[doc = "0x4c - i2c commond5_register"]
    #[inline(always)]
    pub const fn cmd5(&self) -> &CMD5 {
        &self.cmd5
    }
    #[doc = "0x50 - i2c commond6 register"]
    #[inline(always)]
    pub const fn cmd6(&self) -> &CMD6 {
        &self.cmd6
    }
    #[doc = "0x54 - i2c commond7 register"]
    #[inline(always)]
    pub const fn cmd7(&self) -> &CMD7 {
        &self.cmd7
    }
    #[doc = "0x58 - i2c commond8 register"]
    #[inline(always)]
    pub const fn cmd8(&self) -> &CMD8 {
        &self.cmd8
    }
    #[doc = "0x5c - i2c commond9 register"]
    #[inline(always)]
    pub const fn cmd9(&self) -> &CMD9 {
        &self.cmd9
    }
    #[doc = "0x60 - i2c commond10 register"]
    #[inline(always)]
    pub const fn cmd10(&self) -> &CMD10 {
        &self.cmd10
    }
    #[doc = "0x64 - i2c commond11 register"]
    #[inline(always)]
    pub const fn cmd11(&self) -> &CMD11 {
        &self.cmd11
    }
    #[doc = "0x68 - i2c commond12 register"]
    #[inline(always)]
    pub const fn cmd12(&self) -> &CMD12 {
        &self.cmd12
    }
    #[doc = "0x6c - i2c commond13 register"]
    #[inline(always)]
    pub const fn cmd13(&self) -> &CMD13 {
        &self.cmd13
    }
    #[doc = "0x70 - i2c commond14 register"]
    #[inline(always)]
    pub const fn cmd14(&self) -> &CMD14 {
        &self.cmd14
    }
    #[doc = "0x74 - i2c commond15 register"]
    #[inline(always)]
    pub const fn cmd15(&self) -> &CMD15 {
        &self.cmd15
    }
    #[doc = "0xfc - version register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "SCL_LOW (rw) register accessor: configure low scl period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_low`] module"]
pub type SCL_LOW = crate::Reg<scl_low::SCL_LOW_SPEC>;
#[doc = "configure low scl period"]
pub mod scl_low;
#[doc = "CTRL (rw) register accessor: configure i2c ctrl\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "configure i2c ctrl"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: get i2c status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "get i2c status"]
pub mod status;
#[doc = "TO (rw) register accessor: configure time out\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`to::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`to::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@to`] module"]
pub type TO = crate::Reg<to::TO_SPEC>;
#[doc = "configure time out"]
pub mod to;
#[doc = "SLAVE_ADDR (rw) register accessor: configure slave id\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slave_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slave_addr`] module"]
pub type SLAVE_ADDR = crate::Reg<slave_addr::SLAVE_ADDR_SPEC>;
#[doc = "configure slave id"]
pub mod slave_addr;
#[doc = "SCL_HIGH (rw) register accessor: configure high scl period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_high`] module"]
pub type SCL_HIGH = crate::Reg<scl_high::SCL_HIGH_SPEC>;
#[doc = "configure high scl period"]
pub mod scl_high;
#[doc = "SDA_DUTY (rw) register accessor: configure sda duty\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sda_duty::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sda_duty::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sda_duty`] module"]
pub type SDA_DUTY = crate::Reg<sda_duty::SDA_DUTY_SPEC>;
#[doc = "configure sda duty"]
pub mod sda_duty;
#[doc = "SCL_START_PERIOD (rw) register accessor: configure scl start period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_start_period::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_start_period::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_start_period`] module"]
pub type SCL_START_PERIOD = crate::Reg<scl_start_period::SCL_START_PERIOD_SPEC>;
#[doc = "configure scl start period"]
pub mod scl_start_period;
#[doc = "SCL_STOP_PERIOD (rw) register accessor: configure scl stop period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_stop_period::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_stop_period::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_stop_period`] module"]
pub type SCL_STOP_PERIOD = crate::Reg<scl_stop_period::SCL_STOP_PERIOD_SPEC>;
#[doc = "configure scl stop period"]
pub mod scl_stop_period;
#[doc = "INT_CLR (w) register accessor: interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "interrupt clear register"]
pub mod int_clr;
#[doc = "INT_RAW (r) register accessor: interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "interrupt raw register"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: interrupt state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "interrupt state register"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "interrupt enable register"]
pub mod int_ena;
#[doc = "DATA (rw) register accessor: get i2c data status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`] module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "get i2c data status"]
pub mod data;
#[doc = "CMD0 (rw) register accessor: i2c commond0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd0`] module"]
pub type CMD0 = crate::Reg<cmd0::CMD0_SPEC>;
#[doc = "i2c commond0 register"]
pub mod cmd0;
#[doc = "CMD1 (rw) register accessor: i2c commond1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd1`] module"]
pub type CMD1 = crate::Reg<cmd1::CMD1_SPEC>;
#[doc = "i2c commond1 register"]
pub mod cmd1;
#[doc = "CMD2 (rw) register accessor: i2c commond2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd2`] module"]
pub type CMD2 = crate::Reg<cmd2::CMD2_SPEC>;
#[doc = "i2c commond2 register"]
pub mod cmd2;
#[doc = "CMD3 (rw) register accessor: i2c commond3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd3`] module"]
pub type CMD3 = crate::Reg<cmd3::CMD3_SPEC>;
#[doc = "i2c commond3 register"]
pub mod cmd3;
#[doc = "CMD4 (rw) register accessor: i2c commond4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd4`] module"]
pub type CMD4 = crate::Reg<cmd4::CMD4_SPEC>;
#[doc = "i2c commond4 register"]
pub mod cmd4;
#[doc = "CMD5 (rw) register accessor: i2c commond5_register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd5`] module"]
pub type CMD5 = crate::Reg<cmd5::CMD5_SPEC>;
#[doc = "i2c commond5_register"]
pub mod cmd5;
#[doc = "CMD6 (rw) register accessor: i2c commond6 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd6`] module"]
pub type CMD6 = crate::Reg<cmd6::CMD6_SPEC>;
#[doc = "i2c commond6 register"]
pub mod cmd6;
#[doc = "CMD7 (rw) register accessor: i2c commond7 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd7`] module"]
pub type CMD7 = crate::Reg<cmd7::CMD7_SPEC>;
#[doc = "i2c commond7 register"]
pub mod cmd7;
#[doc = "CMD8 (rw) register accessor: i2c commond8 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd8`] module"]
pub type CMD8 = crate::Reg<cmd8::CMD8_SPEC>;
#[doc = "i2c commond8 register"]
pub mod cmd8;
#[doc = "CMD9 (rw) register accessor: i2c commond9 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd9`] module"]
pub type CMD9 = crate::Reg<cmd9::CMD9_SPEC>;
#[doc = "i2c commond9 register"]
pub mod cmd9;
#[doc = "CMD10 (rw) register accessor: i2c commond10 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd10`] module"]
pub type CMD10 = crate::Reg<cmd10::CMD10_SPEC>;
#[doc = "i2c commond10 register"]
pub mod cmd10;
#[doc = "CMD11 (rw) register accessor: i2c commond11 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd11`] module"]
pub type CMD11 = crate::Reg<cmd11::CMD11_SPEC>;
#[doc = "i2c commond11 register"]
pub mod cmd11;
#[doc = "CMD12 (rw) register accessor: i2c commond12 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd12`] module"]
pub type CMD12 = crate::Reg<cmd12::CMD12_SPEC>;
#[doc = "i2c commond12 register"]
pub mod cmd12;
#[doc = "CMD13 (rw) register accessor: i2c commond13 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd13`] module"]
pub type CMD13 = crate::Reg<cmd13::CMD13_SPEC>;
#[doc = "i2c commond13 register"]
pub mod cmd13;
#[doc = "CMD14 (rw) register accessor: i2c commond14 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd14`] module"]
pub type CMD14 = crate::Reg<cmd14::CMD14_SPEC>;
#[doc = "i2c commond14 register"]
pub mod cmd14;
#[doc = "CMD15 (rw) register accessor: i2c commond15 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd15`] module"]
pub type CMD15 = crate::Reg<cmd15::CMD15_SPEC>;
#[doc = "i2c commond15 register"]
pub mod cmd15;
#[doc = "DATE (rw) register accessor: version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "version register"]
pub mod date;
