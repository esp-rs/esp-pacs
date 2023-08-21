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
#[doc = "SIGMADELTA0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sigmadelta0`] module"]
pub type SIGMADELTA0 = crate::Reg<sigmadelta0::SIGMADELTA0_SPEC>;
#[doc = ""]
pub mod sigmadelta0;
#[doc = "SIGMADELTA1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sigmadelta1`] module"]
pub type SIGMADELTA1 = crate::Reg<sigmadelta1::SIGMADELTA1_SPEC>;
#[doc = ""]
pub mod sigmadelta1;
#[doc = "SIGMADELTA2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sigmadelta2`] module"]
pub type SIGMADELTA2 = crate::Reg<sigmadelta2::SIGMADELTA2_SPEC>;
#[doc = ""]
pub mod sigmadelta2;
#[doc = "SIGMADELTA3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sigmadelta3`] module"]
pub type SIGMADELTA3 = crate::Reg<sigmadelta3::SIGMADELTA3_SPEC>;
#[doc = ""]
pub mod sigmadelta3;
#[doc = "SIGMADELTA4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sigmadelta4`] module"]
pub type SIGMADELTA4 = crate::Reg<sigmadelta4::SIGMADELTA4_SPEC>;
#[doc = ""]
pub mod sigmadelta4;
#[doc = "SIGMADELTA5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sigmadelta5`] module"]
pub type SIGMADELTA5 = crate::Reg<sigmadelta5::SIGMADELTA5_SPEC>;
#[doc = ""]
pub mod sigmadelta5;
#[doc = "SIGMADELTA6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sigmadelta6`] module"]
pub type SIGMADELTA6 = crate::Reg<sigmadelta6::SIGMADELTA6_SPEC>;
#[doc = ""]
pub mod sigmadelta6;
#[doc = "SIGMADELTA7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sigmadelta7`] module"]
pub type SIGMADELTA7 = crate::Reg<sigmadelta7::SIGMADELTA7_SPEC>;
#[doc = ""]
pub mod sigmadelta7;
#[doc = "CG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cg`] module"]
pub type CG = crate::Reg<cg::CG_SPEC>;
#[doc = ""]
pub mod cg;
#[doc = "MISC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`misc`] module"]
pub type MISC = crate::Reg<misc::MISC_SPEC>;
#[doc = ""]
pub mod misc;
#[doc = "VERSION (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`version::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`version`] module"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = ""]
pub mod version;
