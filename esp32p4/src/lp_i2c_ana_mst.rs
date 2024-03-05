#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    i2c0_ctrl: I2C0_CTRL,
    i2c1_ctrl: I2C1_CTRL,
    i2c0_conf: I2C0_CONF,
    i2c1_conf: I2C1_CONF,
    i2c_burst_conf: I2C_BURST_CONF,
    i2c_burst_status: I2C_BURST_STATUS,
    ana_conf0: ANA_CONF0,
    ana_conf1: ANA_CONF1,
    ana_conf2: ANA_CONF2,
    i2c0_ctrl1: I2C0_CTRL1,
    i2c1_ctrl1: I2C1_CTRL1,
    hw_i2c_ctrl: HW_I2C_CTRL,
    nouse: NOUSE,
    clk160m: CLK160M,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - need des"]
    #[inline(always)]
    pub const fn i2c0_ctrl(&self) -> &I2C0_CTRL {
        &self.i2c0_ctrl
    }
    #[doc = "0x04 - need des"]
    #[inline(always)]
    pub const fn i2c1_ctrl(&self) -> &I2C1_CTRL {
        &self.i2c1_ctrl
    }
    #[doc = "0x08 - need des"]
    #[inline(always)]
    pub const fn i2c0_conf(&self) -> &I2C0_CONF {
        &self.i2c0_conf
    }
    #[doc = "0x0c - need des"]
    #[inline(always)]
    pub const fn i2c1_conf(&self) -> &I2C1_CONF {
        &self.i2c1_conf
    }
    #[doc = "0x10 - need des"]
    #[inline(always)]
    pub const fn i2c_burst_conf(&self) -> &I2C_BURST_CONF {
        &self.i2c_burst_conf
    }
    #[doc = "0x14 - need des"]
    #[inline(always)]
    pub const fn i2c_burst_status(&self) -> &I2C_BURST_STATUS {
        &self.i2c_burst_status
    }
    #[doc = "0x18 - need des"]
    #[inline(always)]
    pub const fn ana_conf0(&self) -> &ANA_CONF0 {
        &self.ana_conf0
    }
    #[doc = "0x1c - need des"]
    #[inline(always)]
    pub const fn ana_conf1(&self) -> &ANA_CONF1 {
        &self.ana_conf1
    }
    #[doc = "0x20 - need des"]
    #[inline(always)]
    pub const fn ana_conf2(&self) -> &ANA_CONF2 {
        &self.ana_conf2
    }
    #[doc = "0x24 - need des"]
    #[inline(always)]
    pub const fn i2c0_ctrl1(&self) -> &I2C0_CTRL1 {
        &self.i2c0_ctrl1
    }
    #[doc = "0x28 - need des"]
    #[inline(always)]
    pub const fn i2c1_ctrl1(&self) -> &I2C1_CTRL1 {
        &self.i2c1_ctrl1
    }
    #[doc = "0x2c - need des"]
    #[inline(always)]
    pub const fn hw_i2c_ctrl(&self) -> &HW_I2C_CTRL {
        &self.hw_i2c_ctrl
    }
    #[doc = "0x30 - need des"]
    #[inline(always)]
    pub const fn nouse(&self) -> &NOUSE {
        &self.nouse
    }
    #[doc = "0x34 - need des"]
    #[inline(always)]
    pub const fn clk160m(&self) -> &CLK160M {
        &self.clk160m
    }
    #[doc = "0x38 - need des"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "I2C0_CTRL (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_ctrl`] module"]
pub type I2C0_CTRL = crate::Reg<i2c0_ctrl::I2C0_CTRL_SPEC>;
#[doc = "need des"]
pub mod i2c0_ctrl;
#[doc = "I2C1_CTRL (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_ctrl`] module"]
pub type I2C1_CTRL = crate::Reg<i2c1_ctrl::I2C1_CTRL_SPEC>;
#[doc = "need des"]
pub mod i2c1_ctrl;
#[doc = "I2C0_CONF (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c0_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c0_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_conf`] module"]
pub type I2C0_CONF = crate::Reg<i2c0_conf::I2C0_CONF_SPEC>;
#[doc = "need des"]
pub mod i2c0_conf;
#[doc = "I2C1_CONF (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c1_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c1_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_conf`] module"]
pub type I2C1_CONF = crate::Reg<i2c1_conf::I2C1_CONF_SPEC>;
#[doc = "need des"]
pub mod i2c1_conf;
#[doc = "I2C_BURST_CONF (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_burst_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_burst_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_burst_conf`] module"]
pub type I2C_BURST_CONF = crate::Reg<i2c_burst_conf::I2C_BURST_CONF_SPEC>;
#[doc = "need des"]
pub mod i2c_burst_conf;
#[doc = "I2C_BURST_STATUS (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_burst_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_burst_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_burst_status`] module"]
pub type I2C_BURST_STATUS = crate::Reg<i2c_burst_status::I2C_BURST_STATUS_SPEC>;
#[doc = "need des"]
pub mod i2c_burst_status;
#[doc = "ANA_CONF0 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ana_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_conf0`] module"]
pub type ANA_CONF0 = crate::Reg<ana_conf0::ANA_CONF0_SPEC>;
#[doc = "need des"]
pub mod ana_conf0;
#[doc = "ANA_CONF1 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ana_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_conf1`] module"]
pub type ANA_CONF1 = crate::Reg<ana_conf1::ANA_CONF1_SPEC>;
#[doc = "need des"]
pub mod ana_conf1;
#[doc = "ANA_CONF2 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ana_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_conf2`] module"]
pub type ANA_CONF2 = crate::Reg<ana_conf2::ANA_CONF2_SPEC>;
#[doc = "need des"]
pub mod ana_conf2;
#[doc = "I2C0_CTRL1 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c0_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c0_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_ctrl1`] module"]
pub type I2C0_CTRL1 = crate::Reg<i2c0_ctrl1::I2C0_CTRL1_SPEC>;
#[doc = "need des"]
pub mod i2c0_ctrl1;
#[doc = "I2C1_CTRL1 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c1_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c1_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_ctrl1`] module"]
pub type I2C1_CTRL1 = crate::Reg<i2c1_ctrl1::I2C1_CTRL1_SPEC>;
#[doc = "need des"]
pub mod i2c1_ctrl1;
#[doc = "HW_I2C_CTRL (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hw_i2c_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hw_i2c_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_i2c_ctrl`] module"]
pub type HW_I2C_CTRL = crate::Reg<hw_i2c_ctrl::HW_I2C_CTRL_SPEC>;
#[doc = "need des"]
pub mod hw_i2c_ctrl;
#[doc = "NOUSE (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nouse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nouse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nouse`] module"]
pub type NOUSE = crate::Reg<nouse::NOUSE_SPEC>;
#[doc = "need des"]
pub mod nouse;
#[doc = "CLK160M (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk160m::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk160m::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk160m`] module"]
pub type CLK160M = crate::Reg<clk160m::CLK160M_SPEC>;
#[doc = "need des"]
pub mod clk160m;
#[doc = "DATE (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need des"]
pub mod date;
