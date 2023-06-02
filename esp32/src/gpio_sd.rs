#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
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
    pub cg: CG,
    #[doc = "0x24 - "]
    pub misc: MISC,
    #[doc = "0x28 - "]
    pub version: VERSION,
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
#[doc = "CG (rw) register accessor: an alias for `Reg<CG_SPEC>`"]
pub type CG = crate::Reg<cg::CG_SPEC>;
#[doc = ""]
pub mod cg;
#[doc = "MISC (rw) register accessor: an alias for `Reg<MISC_SPEC>`"]
pub type MISC = crate::Reg<misc::MISC_SPEC>;
#[doc = ""]
pub mod misc;
#[doc = "VERSION (rw) register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = ""]
pub mod version;
