#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00..0x20 - Duty Cycle Configure Register of SDM%s"]
    pub sigmadelta: [SIGMADELTA; 8],
    #[doc = "0x20 - Clock Gating Configure Register"]
    pub sigmadelta_cg: SIGMADELTA_CG,
    #[doc = "0x24 - MISC Register"]
    pub sigmadelta_misc: SIGMADELTA_MISC,
    #[doc = "0x28 - Version Control Register"]
    pub sigmadelta_version: SIGMADELTA_VERSION,
}
#[doc = "SIGMADELTA (rw) register accessor: an alias for `Reg<SIGMADELTA_SPEC>`"]
pub type SIGMADELTA = crate::Reg<sigmadelta::SIGMADELTA_SPEC>;
#[doc = "Duty Cycle Configure Register of SDM%s"]
pub mod sigmadelta;
#[doc = "SIGMADELTA_CG (rw) register accessor: an alias for `Reg<SIGMADELTA_CG_SPEC>`"]
pub type SIGMADELTA_CG = crate::Reg<sigmadelta_cg::SIGMADELTA_CG_SPEC>;
#[doc = "Clock Gating Configure Register"]
pub mod sigmadelta_cg;
#[doc = "SIGMADELTA_MISC (rw) register accessor: an alias for `Reg<SIGMADELTA_MISC_SPEC>`"]
pub type SIGMADELTA_MISC = crate::Reg<sigmadelta_misc::SIGMADELTA_MISC_SPEC>;
#[doc = "MISC Register"]
pub mod sigmadelta_misc;
#[doc = "SIGMADELTA_VERSION (rw) register accessor: an alias for `Reg<SIGMADELTA_VERSION_SPEC>`"]
pub type SIGMADELTA_VERSION = crate::Reg<sigmadelta_version::SIGMADELTA_VERSION_SPEC>;
#[doc = "Version Control Register"]
pub mod sigmadelta_version;
