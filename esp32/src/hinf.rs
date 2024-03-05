#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg_data0: CFG_DATA0,
    cfg_data1: CFG_DATA1,
    _reserved2: [u8; 0x14],
    cfg_data7: CFG_DATA7,
    cis_conf0: CIS_CONF0,
    cis_conf1: CIS_CONF1,
    cis_conf2: CIS_CONF2,
    cis_conf3: CIS_CONF3,
    cis_conf4: CIS_CONF4,
    cis_conf5: CIS_CONF5,
    cis_conf6: CIS_CONF6,
    cis_conf7: CIS_CONF7,
    cfg_data16: CFG_DATA16,
    _reserved12: [u8; 0xb8],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn cfg_data0(&self) -> &CFG_DATA0 {
        &self.cfg_data0
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn cfg_data1(&self) -> &CFG_DATA1 {
        &self.cfg_data1
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn cfg_data7(&self) -> &CFG_DATA7 {
        &self.cfg_data7
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn cis_conf0(&self) -> &CIS_CONF0 {
        &self.cis_conf0
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn cis_conf1(&self) -> &CIS_CONF1 {
        &self.cis_conf1
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn cis_conf2(&self) -> &CIS_CONF2 {
        &self.cis_conf2
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn cis_conf3(&self) -> &CIS_CONF3 {
        &self.cis_conf3
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn cis_conf4(&self) -> &CIS_CONF4 {
        &self.cis_conf4
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn cis_conf5(&self) -> &CIS_CONF5 {
        &self.cis_conf5
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn cis_conf6(&self) -> &CIS_CONF6 {
        &self.cis_conf6
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn cis_conf7(&self) -> &CIS_CONF7 {
        &self.cis_conf7
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn cfg_data16(&self) -> &CFG_DATA16 {
        &self.cfg_data16
    }
    #[doc = "0xfc - "]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CFG_DATA0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data0`] module"]
pub type CFG_DATA0 = crate::Reg<cfg_data0::CFG_DATA0_SPEC>;
#[doc = ""]
pub mod cfg_data0;
#[doc = "CFG_DATA1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data1`] module"]
pub type CFG_DATA1 = crate::Reg<cfg_data1::CFG_DATA1_SPEC>;
#[doc = ""]
pub mod cfg_data1;
#[doc = "CFG_DATA7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_data7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_data7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data7`] module"]
pub type CFG_DATA7 = crate::Reg<cfg_data7::CFG_DATA7_SPEC>;
#[doc = ""]
pub mod cfg_data7;
#[doc = "CIS_CONF0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cis_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cis_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cis_conf0`] module"]
pub type CIS_CONF0 = crate::Reg<cis_conf0::CIS_CONF0_SPEC>;
#[doc = ""]
pub mod cis_conf0;
#[doc = "CIS_CONF1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cis_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cis_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cis_conf1`] module"]
pub type CIS_CONF1 = crate::Reg<cis_conf1::CIS_CONF1_SPEC>;
#[doc = ""]
pub mod cis_conf1;
#[doc = "CIS_CONF2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cis_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cis_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cis_conf2`] module"]
pub type CIS_CONF2 = crate::Reg<cis_conf2::CIS_CONF2_SPEC>;
#[doc = ""]
pub mod cis_conf2;
#[doc = "CIS_CONF3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cis_conf3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cis_conf3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cis_conf3`] module"]
pub type CIS_CONF3 = crate::Reg<cis_conf3::CIS_CONF3_SPEC>;
#[doc = ""]
pub mod cis_conf3;
#[doc = "CIS_CONF4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cis_conf4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cis_conf4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cis_conf4`] module"]
pub type CIS_CONF4 = crate::Reg<cis_conf4::CIS_CONF4_SPEC>;
#[doc = ""]
pub mod cis_conf4;
#[doc = "CIS_CONF5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cis_conf5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cis_conf5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cis_conf5`] module"]
pub type CIS_CONF5 = crate::Reg<cis_conf5::CIS_CONF5_SPEC>;
#[doc = ""]
pub mod cis_conf5;
#[doc = "CIS_CONF6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cis_conf6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cis_conf6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cis_conf6`] module"]
pub type CIS_CONF6 = crate::Reg<cis_conf6::CIS_CONF6_SPEC>;
#[doc = ""]
pub mod cis_conf6;
#[doc = "CIS_CONF7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cis_conf7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cis_conf7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cis_conf7`] module"]
pub type CIS_CONF7 = crate::Reg<cis_conf7::CIS_CONF7_SPEC>;
#[doc = ""]
pub mod cis_conf7;
#[doc = "CFG_DATA16 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_data16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_data16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data16`] module"]
pub type CFG_DATA16 = crate::Reg<cfg_data16::CFG_DATA16_SPEC>;
#[doc = ""]
pub mod cfg_data16;
#[doc = "DATE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
