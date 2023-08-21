#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00..0x10 - Duty Cycle Configure Register of SDM%s"]
    pub sigmadelta: [SIGMADELTA; 4],
    _reserved1: [u8; 0x10],
    #[doc = "0x20 - Clock Gating Configure Register"]
    pub sigmadelta_cg: SIGMADELTA_CG,
    #[doc = "0x24 - MISC Register"]
    pub sigmadelta_misc: SIGMADELTA_MISC,
    #[doc = "0x28 - Version Control Register"]
    pub sigmadelta_version: SIGMADELTA_VERSION,
}
#[doc = "SIGMADELTA (rw) register accessor: Duty Cycle Configure Register of SDM%s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sigmadelta`] module"]
pub type SIGMADELTA = crate::Reg<sigmadelta::SIGMADELTA_SPEC>;
#[doc = "Duty Cycle Configure Register of SDM%s"]
pub mod sigmadelta;
#[doc = "SIGMADELTA_CG (rw) register accessor: Clock Gating Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta_cg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta_cg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sigmadelta_cg`] module"]
pub type SIGMADELTA_CG = crate::Reg<sigmadelta_cg::SIGMADELTA_CG_SPEC>;
#[doc = "Clock Gating Configure Register"]
pub mod sigmadelta_cg;
#[doc = "SIGMADELTA_MISC (rw) register accessor: MISC Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta_misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta_misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sigmadelta_misc`] module"]
pub type SIGMADELTA_MISC = crate::Reg<sigmadelta_misc::SIGMADELTA_MISC_SPEC>;
#[doc = "MISC Register"]
pub mod sigmadelta_misc;
#[doc = "SIGMADELTA_VERSION (rw) register accessor: Version Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta_version::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta_version::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sigmadelta_version`] module"]
pub type SIGMADELTA_VERSION = crate::Reg<sigmadelta_version::SIGMADELTA_VERSION_SPEC>;
#[doc = "Version Control Register"]
pub mod sigmadelta_version;
