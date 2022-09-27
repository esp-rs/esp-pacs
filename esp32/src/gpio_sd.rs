#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub sigmadelta0: SIGMADELTA0,
    #[doc = "0x04 - "]
    pub sigmadelta1: SIGMADELTA1,
    #[doc = "0x08 - "]
    pub sigmadelta2: SIGMADELTA2,
    #[doc = "0x0c - "]
    pub sigmadelta3: SIGMADELTA3,
    #[doc = "0x10 - "]
    pub sigmadelta4: SIGMADELTA4,
    #[doc = "0x14 - "]
    pub sigmadelta5: SIGMADELTA5,
    #[doc = "0x18 - "]
    pub sigmadelta6: SIGMADELTA6,
    #[doc = "0x1c - "]
    pub sigmadelta7: SIGMADELTA7,
    #[doc = "0x20 - "]
    pub sigmadelta_cg: SIGMADELTA_CG,
    #[doc = "0x24 - "]
    pub sigmadelta_misc: SIGMADELTA_MISC,
    #[doc = "0x28 - "]
    pub sigmadelta_version: SIGMADELTA_VERSION,
}
#[doc = "SIGMADELTA0 (rw) register accessor: an alias for `Reg<SIGMADELTA0_SPEC>`"]
pub type SIGMADELTA0 = crate::Reg<sigmadelta0::SIGMADELTA0_SPEC>;
#[doc = ""]
pub mod sigmadelta0;
#[doc = "SIGMADELTA1 (rw) register accessor: an alias for `Reg<SIGMADELTA1_SPEC>`"]
pub type SIGMADELTA1 = crate::Reg<sigmadelta1::SIGMADELTA1_SPEC>;
#[doc = ""]
pub mod sigmadelta1;
#[doc = "SIGMADELTA2 (rw) register accessor: an alias for `Reg<SIGMADELTA2_SPEC>`"]
pub type SIGMADELTA2 = crate::Reg<sigmadelta2::SIGMADELTA2_SPEC>;
#[doc = ""]
pub mod sigmadelta2;
#[doc = "SIGMADELTA3 (rw) register accessor: an alias for `Reg<SIGMADELTA3_SPEC>`"]
pub type SIGMADELTA3 = crate::Reg<sigmadelta3::SIGMADELTA3_SPEC>;
#[doc = ""]
pub mod sigmadelta3;
#[doc = "SIGMADELTA4 (rw) register accessor: an alias for `Reg<SIGMADELTA4_SPEC>`"]
pub type SIGMADELTA4 = crate::Reg<sigmadelta4::SIGMADELTA4_SPEC>;
#[doc = ""]
pub mod sigmadelta4;
#[doc = "SIGMADELTA5 (rw) register accessor: an alias for `Reg<SIGMADELTA5_SPEC>`"]
pub type SIGMADELTA5 = crate::Reg<sigmadelta5::SIGMADELTA5_SPEC>;
#[doc = ""]
pub mod sigmadelta5;
#[doc = "SIGMADELTA6 (rw) register accessor: an alias for `Reg<SIGMADELTA6_SPEC>`"]
pub type SIGMADELTA6 = crate::Reg<sigmadelta6::SIGMADELTA6_SPEC>;
#[doc = ""]
pub mod sigmadelta6;
#[doc = "SIGMADELTA7 (rw) register accessor: an alias for `Reg<SIGMADELTA7_SPEC>`"]
pub type SIGMADELTA7 = crate::Reg<sigmadelta7::SIGMADELTA7_SPEC>;
#[doc = ""]
pub mod sigmadelta7;
#[doc = "SIGMADELTA_CG (rw) register accessor: an alias for `Reg<SIGMADELTA_CG_SPEC>`"]
pub type SIGMADELTA_CG = crate::Reg<sigmadelta_cg::SIGMADELTA_CG_SPEC>;
#[doc = ""]
pub mod sigmadelta_cg;
#[doc = "SIGMADELTA_MISC (rw) register accessor: an alias for `Reg<SIGMADELTA_MISC_SPEC>`"]
pub type SIGMADELTA_MISC = crate::Reg<sigmadelta_misc::SIGMADELTA_MISC_SPEC>;
#[doc = ""]
pub mod sigmadelta_misc;
#[doc = "SIGMADELTA_VERSION (rw) register accessor: an alias for `Reg<SIGMADELTA_VERSION_SPEC>`"]
pub type SIGMADELTA_VERSION = crate::Reg<sigmadelta_version::SIGMADELTA_VERSION_SPEC>;
#[doc = ""]
pub mod sigmadelta_version;
