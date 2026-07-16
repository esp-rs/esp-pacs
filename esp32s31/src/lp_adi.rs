#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    ver_date: VER_DATE,
    dcdc_top_2_sts: DCDC_TOP_2_STS,
    dcdc_top_2_cfg: DCDC_TOP_2_CFG,
    pm_top_sts: PM_TOP_STS,
    pm_top_cfg: PM_TOP_CFG,
}
impl RegisterBlock {
    #[doc = "0x00 - need_des"]
    #[inline(always)]
    pub const fn ver_date(&self) -> &VER_DATE {
        &self.ver_date
    }
    #[doc = "0x04 - ANALOG DCDC 2 related status"]
    #[inline(always)]
    pub const fn dcdc_top_2_sts(&self) -> &DCDC_TOP_2_STS {
        &self.dcdc_top_2_sts
    }
    #[doc = "0x08 - ANALOG DCDC 2 related CONFIG"]
    #[inline(always)]
    pub const fn dcdc_top_2_cfg(&self) -> &DCDC_TOP_2_CFG {
        &self.dcdc_top_2_cfg
    }
    #[doc = "0x0c - ANALOG PM TOP related status"]
    #[inline(always)]
    pub const fn pm_top_sts(&self) -> &PM_TOP_STS {
        &self.pm_top_sts
    }
    #[doc = "0x10 - ANALOG PM TOP related CONFIG"]
    #[inline(always)]
    pub const fn pm_top_cfg(&self) -> &PM_TOP_CFG {
        &self.pm_top_cfg
    }
}
#[doc = "VER_DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ver_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ver_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ver_date`] module"]
pub type VER_DATE = crate::Reg<ver_date::VER_DATE_SPEC>;
#[doc = "need_des"]
pub mod ver_date;
#[doc = "DCDC_TOP_2_STS (r) register accessor: ANALOG DCDC 2 related status\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc_top_2_sts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_top_2_sts`] module"]
pub type DCDC_TOP_2_STS = crate::Reg<dcdc_top_2_sts::DCDC_TOP_2_STS_SPEC>;
#[doc = "ANALOG DCDC 2 related status"]
pub mod dcdc_top_2_sts;
#[doc = "DCDC_TOP_2_CFG (rw) register accessor: ANALOG DCDC 2 related CONFIG\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc_top_2_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_top_2_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_top_2_cfg`] module"]
pub type DCDC_TOP_2_CFG = crate::Reg<dcdc_top_2_cfg::DCDC_TOP_2_CFG_SPEC>;
#[doc = "ANALOG DCDC 2 related CONFIG"]
pub mod dcdc_top_2_cfg;
#[doc = "PM_TOP_STS (r) register accessor: ANALOG PM TOP related status\n\nYou can [`read`](crate::Reg::read) this register and get [`pm_top_sts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pm_top_sts`] module"]
pub type PM_TOP_STS = crate::Reg<pm_top_sts::PM_TOP_STS_SPEC>;
#[doc = "ANALOG PM TOP related status"]
pub mod pm_top_sts;
#[doc = "PM_TOP_CFG (rw) register accessor: ANALOG PM TOP related CONFIG\n\nYou can [`read`](crate::Reg::read) this register and get [`pm_top_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pm_top_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pm_top_cfg`] module"]
pub type PM_TOP_CFG = crate::Reg<pm_top_cfg::PM_TOP_CFG_SPEC>;
#[doc = "ANALOG PM TOP related CONFIG"]
pub mod pm_top_cfg;
