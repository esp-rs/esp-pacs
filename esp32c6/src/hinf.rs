#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg_data0: CFG_DATA0,
    cfg_data1: CFG_DATA1,
    cfg_timing: CFG_TIMING,
    cfg_update: CFG_UPDATE,
    _reserved4: [u8; 0x0c],
    cfg_data7: CFG_DATA7,
    cis_conf_w0: CIS_CONF_W0,
    cis_conf_w1: CIS_CONF_W1,
    cis_conf_w2: CIS_CONF_W2,
    cis_conf_w3: CIS_CONF_W3,
    cis_conf_w4: CIS_CONF_W4,
    cis_conf_w5: CIS_CONF_W5,
    cis_conf_w6: CIS_CONF_W6,
    cis_conf_w7: CIS_CONF_W7,
    cfg_data16: CFG_DATA16,
    cfg_uhs1_int_mode: CFG_UHS1_INT_MODE,
    _reserved15: [u8; 0x0c],
    conf_status: CONF_STATUS,
    _reserved16: [u8; 0x4c],
    sdio_slave_eco_low: SDIO_SLAVE_ECO_LOW,
    sdio_slave_eco_high: SDIO_SLAVE_ECO_HIGH,
    sdio_slave_eco_conf: SDIO_SLAVE_ECO_CONF,
    sdio_slave_ldo_conf: SDIO_SLAVE_LDO_CONF,
    _reserved20: [u8; 0x48],
    sdio_date: SDIO_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - Configure sdio cis content"]
    #[inline(always)]
    pub const fn cfg_data0(&self) -> &CFG_DATA0 {
        &self.cfg_data0
    }
    #[doc = "0x04 - SDIO configuration register"]
    #[inline(always)]
    pub const fn cfg_data1(&self) -> &CFG_DATA1 {
        &self.cfg_data1
    }
    #[doc = "0x08 - Timing configuration registers"]
    #[inline(always)]
    pub const fn cfg_timing(&self) -> &CFG_TIMING {
        &self.cfg_timing
    }
    #[doc = "0x0c - update sdio configurations"]
    #[inline(always)]
    pub const fn cfg_update(&self) -> &CFG_UPDATE {
        &self.cfg_update
    }
    #[doc = "0x1c - SDIO configuration register"]
    #[inline(always)]
    pub const fn cfg_data7(&self) -> &CFG_DATA7 {
        &self.cfg_data7
    }
    #[doc = "0x20 - SDIO cis configuration register"]
    #[inline(always)]
    pub const fn cis_conf_w0(&self) -> &CIS_CONF_W0 {
        &self.cis_conf_w0
    }
    #[doc = "0x24 - SDIO cis configuration register"]
    #[inline(always)]
    pub const fn cis_conf_w1(&self) -> &CIS_CONF_W1 {
        &self.cis_conf_w1
    }
    #[doc = "0x28 - SDIO cis configuration register"]
    #[inline(always)]
    pub const fn cis_conf_w2(&self) -> &CIS_CONF_W2 {
        &self.cis_conf_w2
    }
    #[doc = "0x2c - SDIO cis configuration register"]
    #[inline(always)]
    pub const fn cis_conf_w3(&self) -> &CIS_CONF_W3 {
        &self.cis_conf_w3
    }
    #[doc = "0x30 - SDIO cis configuration register"]
    #[inline(always)]
    pub const fn cis_conf_w4(&self) -> &CIS_CONF_W4 {
        &self.cis_conf_w4
    }
    #[doc = "0x34 - SDIO cis configuration register"]
    #[inline(always)]
    pub const fn cis_conf_w5(&self) -> &CIS_CONF_W5 {
        &self.cis_conf_w5
    }
    #[doc = "0x38 - SDIO cis configuration register"]
    #[inline(always)]
    pub const fn cis_conf_w6(&self) -> &CIS_CONF_W6 {
        &self.cis_conf_w6
    }
    #[doc = "0x3c - SDIO cis configuration register"]
    #[inline(always)]
    pub const fn cis_conf_w7(&self) -> &CIS_CONF_W7 {
        &self.cis_conf_w7
    }
    #[doc = "0x40 - SDIO cis configuration register"]
    #[inline(always)]
    pub const fn cfg_data16(&self) -> &CFG_DATA16 {
        &self.cfg_data16
    }
    #[doc = "0x44 - configure int to start and end ahead of time in uhs1 mode"]
    #[inline(always)]
    pub const fn cfg_uhs1_int_mode(&self) -> &CFG_UHS1_INT_MODE {
        &self.cfg_uhs1_int_mode
    }
    #[doc = "0x54 - func0 config0 status"]
    #[inline(always)]
    pub const fn conf_status(&self) -> &CONF_STATUS {
        &self.conf_status
    }
    #[doc = "0xa4 - sdio_slave redundant control registers"]
    #[inline(always)]
    pub const fn sdio_slave_eco_low(&self) -> &SDIO_SLAVE_ECO_LOW {
        &self.sdio_slave_eco_low
    }
    #[doc = "0xa8 - sdio_slave redundant control registers"]
    #[inline(always)]
    pub const fn sdio_slave_eco_high(&self) -> &SDIO_SLAVE_ECO_HIGH {
        &self.sdio_slave_eco_high
    }
    #[doc = "0xac - sdio_slave redundant control registers"]
    #[inline(always)]
    pub const fn sdio_slave_eco_conf(&self) -> &SDIO_SLAVE_ECO_CONF {
        &self.sdio_slave_eco_conf
    }
    #[doc = "0xb0 - sdio slave ldo control register"]
    #[inline(always)]
    pub const fn sdio_slave_ldo_conf(&self) -> &SDIO_SLAVE_LDO_CONF {
        &self.sdio_slave_ldo_conf
    }
    #[doc = "0xfc - ******* Description ***********"]
    #[inline(always)]
    pub const fn sdio_date(&self) -> &SDIO_DATE {
        &self.sdio_date
    }
}
#[doc = "CFG_DATA0 (rw) register accessor: Configure sdio cis content\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data0`] module"]
pub type CFG_DATA0 = crate::Reg<cfg_data0::CFG_DATA0_SPEC>;
#[doc = "Configure sdio cis content"]
pub mod cfg_data0;
#[doc = "CFG_DATA1 (rw) register accessor: SDIO configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data1`] module"]
pub type CFG_DATA1 = crate::Reg<cfg_data1::CFG_DATA1_SPEC>;
#[doc = "SDIO configuration register"]
pub mod cfg_data1;
#[doc = "CFG_TIMING (rw) register accessor: Timing configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_timing`] module"]
pub type CFG_TIMING = crate::Reg<cfg_timing::CFG_TIMING_SPEC>;
#[doc = "Timing configuration registers"]
pub mod cfg_timing;
#[doc = "CFG_UPDATE (w) register accessor: update sdio configurations\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_update::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_update`] module"]
pub type CFG_UPDATE = crate::Reg<cfg_update::CFG_UPDATE_SPEC>;
#[doc = "update sdio configurations"]
pub mod cfg_update;
#[doc = "CFG_DATA7 (rw) register accessor: SDIO configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data7`] module"]
pub type CFG_DATA7 = crate::Reg<cfg_data7::CFG_DATA7_SPEC>;
#[doc = "SDIO configuration register"]
pub mod cfg_data7;
#[doc = "CIS_CONF_W0 (rw) register accessor: SDIO cis configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cis_conf_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cis_conf_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cis_conf_w0`] module"]
pub type CIS_CONF_W0 = crate::Reg<cis_conf_w0::CIS_CONF_W0_SPEC>;
#[doc = "SDIO cis configuration register"]
pub mod cis_conf_w0;
#[doc = "CIS_CONF_W1 (rw) register accessor: SDIO cis configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cis_conf_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cis_conf_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cis_conf_w1`] module"]
pub type CIS_CONF_W1 = crate::Reg<cis_conf_w1::CIS_CONF_W1_SPEC>;
#[doc = "SDIO cis configuration register"]
pub mod cis_conf_w1;
#[doc = "CIS_CONF_W2 (rw) register accessor: SDIO cis configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cis_conf_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cis_conf_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cis_conf_w2`] module"]
pub type CIS_CONF_W2 = crate::Reg<cis_conf_w2::CIS_CONF_W2_SPEC>;
#[doc = "SDIO cis configuration register"]
pub mod cis_conf_w2;
#[doc = "CIS_CONF_W3 (rw) register accessor: SDIO cis configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cis_conf_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cis_conf_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cis_conf_w3`] module"]
pub type CIS_CONF_W3 = crate::Reg<cis_conf_w3::CIS_CONF_W3_SPEC>;
#[doc = "SDIO cis configuration register"]
pub mod cis_conf_w3;
#[doc = "CIS_CONF_W4 (rw) register accessor: SDIO cis configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cis_conf_w4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cis_conf_w4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cis_conf_w4`] module"]
pub type CIS_CONF_W4 = crate::Reg<cis_conf_w4::CIS_CONF_W4_SPEC>;
#[doc = "SDIO cis configuration register"]
pub mod cis_conf_w4;
#[doc = "CIS_CONF_W5 (rw) register accessor: SDIO cis configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cis_conf_w5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cis_conf_w5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cis_conf_w5`] module"]
pub type CIS_CONF_W5 = crate::Reg<cis_conf_w5::CIS_CONF_W5_SPEC>;
#[doc = "SDIO cis configuration register"]
pub mod cis_conf_w5;
#[doc = "CIS_CONF_W6 (rw) register accessor: SDIO cis configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cis_conf_w6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cis_conf_w6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cis_conf_w6`] module"]
pub type CIS_CONF_W6 = crate::Reg<cis_conf_w6::CIS_CONF_W6_SPEC>;
#[doc = "SDIO cis configuration register"]
pub mod cis_conf_w6;
#[doc = "CIS_CONF_W7 (rw) register accessor: SDIO cis configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cis_conf_w7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cis_conf_w7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cis_conf_w7`] module"]
pub type CIS_CONF_W7 = crate::Reg<cis_conf_w7::CIS_CONF_W7_SPEC>;
#[doc = "SDIO cis configuration register"]
pub mod cis_conf_w7;
#[doc = "CFG_DATA16 (rw) register accessor: SDIO cis configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data16`] module"]
pub type CFG_DATA16 = crate::Reg<cfg_data16::CFG_DATA16_SPEC>;
#[doc = "SDIO cis configuration register"]
pub mod cfg_data16;
#[doc = "CFG_UHS1_INT_MODE (rw) register accessor: configure int to start and end ahead of time in uhs1 mode\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_uhs1_int_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_uhs1_int_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_uhs1_int_mode`] module"]
pub type CFG_UHS1_INT_MODE = crate::Reg<cfg_uhs1_int_mode::CFG_UHS1_INT_MODE_SPEC>;
#[doc = "configure int to start and end ahead of time in uhs1 mode"]
pub mod cfg_uhs1_int_mode;
#[doc = "CONF_STATUS (r) register accessor: func0 config0 status\n\nYou can [`read`](crate::Reg::read) this register and get [`conf_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf_status`] module"]
pub type CONF_STATUS = crate::Reg<conf_status::CONF_STATUS_SPEC>;
#[doc = "func0 config0 status"]
pub mod conf_status;
#[doc = "SDIO_SLAVE_ECO_LOW (rw) register accessor: sdio_slave redundant control registers\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_slave_eco_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_slave_eco_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_slave_eco_low`] module"]
pub type SDIO_SLAVE_ECO_LOW = crate::Reg<sdio_slave_eco_low::SDIO_SLAVE_ECO_LOW_SPEC>;
#[doc = "sdio_slave redundant control registers"]
pub mod sdio_slave_eco_low;
#[doc = "SDIO_SLAVE_ECO_HIGH (rw) register accessor: sdio_slave redundant control registers\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_slave_eco_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_slave_eco_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_slave_eco_high`] module"]
pub type SDIO_SLAVE_ECO_HIGH = crate::Reg<sdio_slave_eco_high::SDIO_SLAVE_ECO_HIGH_SPEC>;
#[doc = "sdio_slave redundant control registers"]
pub mod sdio_slave_eco_high;
#[doc = "SDIO_SLAVE_ECO_CONF (rw) register accessor: sdio_slave redundant control registers\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_slave_eco_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_slave_eco_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_slave_eco_conf`] module"]
pub type SDIO_SLAVE_ECO_CONF = crate::Reg<sdio_slave_eco_conf::SDIO_SLAVE_ECO_CONF_SPEC>;
#[doc = "sdio_slave redundant control registers"]
pub mod sdio_slave_eco_conf;
#[doc = "SDIO_SLAVE_LDO_CONF (rw) register accessor: sdio slave ldo control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_slave_ldo_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_slave_ldo_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_slave_ldo_conf`] module"]
pub type SDIO_SLAVE_LDO_CONF = crate::Reg<sdio_slave_ldo_conf::SDIO_SLAVE_LDO_CONF_SPEC>;
#[doc = "sdio slave ldo control register"]
pub mod sdio_slave_ldo_conf;
#[doc = "SDIO_DATE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_date`] module"]
pub type SDIO_DATE = crate::Reg<sdio_date::SDIO_DATE_SPEC>;
#[doc = "******* Description ***********"]
pub mod sdio_date;
