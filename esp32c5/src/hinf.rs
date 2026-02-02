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
    cis_conf_w: [CIS_CONF_W; 8],
    cfg_data16: CFG_DATA16,
    cfg_uhs1_int_mode: CFG_UHS1_INT_MODE,
    _reserved8: [u8; 0x0c],
    conf_status: CONF_STATUS,
    _reserved9: [u8; 0x58],
    sdio_slave_ldo_conf: SDIO_SLAVE_LDO_CONF,
    _reserved10: [u8; 0x48],
    sdio_date: SDIO_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - CFG_DATA0"]
    #[inline(always)]
    pub const fn cfg_data0(&self) -> &CFG_DATA0 {
        &self.cfg_data0
    }
    #[doc = "0x04 - CFG_DATA1"]
    #[inline(always)]
    pub const fn cfg_data1(&self) -> &CFG_DATA1 {
        &self.cfg_data1
    }
    #[doc = "0x08 - CFG_TIMING"]
    #[inline(always)]
    pub const fn cfg_timing(&self) -> &CFG_TIMING {
        &self.cfg_timing
    }
    #[doc = "0x0c - CFG_UPDATE"]
    #[inline(always)]
    pub const fn cfg_update(&self) -> &CFG_UPDATE {
        &self.cfg_update
    }
    #[doc = "0x1c - CFG_DATA7"]
    #[inline(always)]
    pub const fn cfg_data7(&self) -> &CFG_DATA7 {
        &self.cfg_data7
    }
    #[doc = "0x20..0x40 - SDIO cis configuration register"]
    #[inline(always)]
    pub const fn cis_conf_w(&self, n: usize) -> &CIS_CONF_W {
        &self.cis_conf_w[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x40 - SDIO cis configuration register"]
    #[inline(always)]
    pub fn cis_conf_w_iter(&self) -> impl Iterator<Item = &CIS_CONF_W> {
        self.cis_conf_w.iter()
    }
    #[doc = "0x40 - CFG_DATA16"]
    #[inline(always)]
    pub const fn cfg_data16(&self) -> &CFG_DATA16 {
        &self.cfg_data16
    }
    #[doc = "0x44 - CFG_UHS1_INT_MODE"]
    #[inline(always)]
    pub const fn cfg_uhs1_int_mode(&self) -> &CFG_UHS1_INT_MODE {
        &self.cfg_uhs1_int_mode
    }
    #[doc = "0x54 - CONF_STATUS"]
    #[inline(always)]
    pub const fn conf_status(&self) -> &CONF_STATUS {
        &self.conf_status
    }
    #[doc = "0xb0 - SDIO_SLAVE_LDO_CONF"]
    #[inline(always)]
    pub const fn sdio_slave_ldo_conf(&self) -> &SDIO_SLAVE_LDO_CONF {
        &self.sdio_slave_ldo_conf
    }
    #[doc = "0xfc - SDIO_DATE"]
    #[inline(always)]
    pub const fn sdio_date(&self) -> &SDIO_DATE {
        &self.sdio_date
    }
}
#[doc = "CFG_DATA0 (rw) register accessor: CFG_DATA0\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data0`] module"]
pub type CFG_DATA0 = crate::Reg<cfg_data0::CFG_DATA0_SPEC>;
#[doc = "CFG_DATA0"]
pub mod cfg_data0;
#[doc = "CFG_DATA1 (rw) register accessor: CFG_DATA1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data1`] module"]
pub type CFG_DATA1 = crate::Reg<cfg_data1::CFG_DATA1_SPEC>;
#[doc = "CFG_DATA1"]
pub mod cfg_data1;
#[doc = "CFG_TIMING (rw) register accessor: CFG_TIMING\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_timing`] module"]
pub type CFG_TIMING = crate::Reg<cfg_timing::CFG_TIMING_SPEC>;
#[doc = "CFG_TIMING"]
pub mod cfg_timing;
#[doc = "CFG_UPDATE (w) register accessor: CFG_UPDATE\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_update::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_update`] module"]
pub type CFG_UPDATE = crate::Reg<cfg_update::CFG_UPDATE_SPEC>;
#[doc = "CFG_UPDATE"]
pub mod cfg_update;
#[doc = "CFG_DATA7 (rw) register accessor: CFG_DATA7\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data7`] module"]
pub type CFG_DATA7 = crate::Reg<cfg_data7::CFG_DATA7_SPEC>;
#[doc = "CFG_DATA7"]
pub mod cfg_data7;
#[doc = "CIS_CONF_W (rw) register accessor: SDIO cis configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cis_conf_w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cis_conf_w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cis_conf_w`] module"]
pub type CIS_CONF_W = crate::Reg<cis_conf_w::CIS_CONF_W_SPEC>;
#[doc = "SDIO cis configuration register"]
pub mod cis_conf_w;
#[doc = "CFG_DATA16 (rw) register accessor: CFG_DATA16\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data16`] module"]
pub type CFG_DATA16 = crate::Reg<cfg_data16::CFG_DATA16_SPEC>;
#[doc = "CFG_DATA16"]
pub mod cfg_data16;
#[doc = "CFG_UHS1_INT_MODE (rw) register accessor: CFG_UHS1_INT_MODE\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_uhs1_int_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_uhs1_int_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_uhs1_int_mode`] module"]
pub type CFG_UHS1_INT_MODE = crate::Reg<cfg_uhs1_int_mode::CFG_UHS1_INT_MODE_SPEC>;
#[doc = "CFG_UHS1_INT_MODE"]
pub mod cfg_uhs1_int_mode;
#[doc = "CONF_STATUS (r) register accessor: CONF_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`conf_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf_status`] module"]
pub type CONF_STATUS = crate::Reg<conf_status::CONF_STATUS_SPEC>;
#[doc = "CONF_STATUS"]
pub mod conf_status;
#[doc = "SDIO_SLAVE_LDO_CONF (rw) register accessor: SDIO_SLAVE_LDO_CONF\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_slave_ldo_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_slave_ldo_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_slave_ldo_conf`] module"]
pub type SDIO_SLAVE_LDO_CONF = crate::Reg<sdio_slave_ldo_conf::SDIO_SLAVE_LDO_CONF_SPEC>;
#[doc = "SDIO_SLAVE_LDO_CONF"]
pub mod sdio_slave_ldo_conf;
pub use crate::aes::date as sdio_date;
pub use crate::aes::DATE as SDIO_DATE;
