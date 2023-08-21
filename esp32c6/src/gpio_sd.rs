#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00..0x10 - Duty Cycle Configure Register of SDM%s"]
    pub sigmadelta: [SIGMADELTA; 4],
    _reserved1: [u8; 0x10],
    #[doc = "0x20 - Clock Gating Configure Register"]
    pub clock_gate: CLOCK_GATE,
    #[doc = "0x24 - MISC Register"]
    pub sigmadelta_misc: SIGMADELTA_MISC,
    _reserved3: [u8; 0x08],
    #[doc = "0x30..0x50 - Glitch Filter Configure Register of Channel%s"]
    pub glitch_filter_ch: [GLITCH_FILTER_CH; 8],
    _reserved4: [u8; 0x10],
    #[doc = "0x60..0x80 - Etm Config register of Channel%s"]
    pub etm_event_ch_cfg: [ETM_EVENT_CH_CFG; 8],
    _reserved5: [u8; 0x20],
    #[doc = "0xa0 - Etm Configure Register to decide which GPIO been chosen"]
    pub etm_task_p0_cfg: ETM_TASK_P0_CFG,
    #[doc = "0xa4 - Etm Configure Register to decide which GPIO been chosen"]
    pub etm_task_p1_cfg: ETM_TASK_P1_CFG,
    #[doc = "0xa8 - Etm Configure Register to decide which GPIO been chosen"]
    pub etm_task_p2_cfg: ETM_TASK_P2_CFG,
    #[doc = "0xac - Etm Configure Register to decide which GPIO been chosen"]
    pub etm_task_p3_cfg: ETM_TASK_P3_CFG,
    #[doc = "0xb0 - Etm Configure Register to decide which GPIO been chosen"]
    pub etm_task_p4_cfg: ETM_TASK_P4_CFG,
    #[doc = "0xb4 - Etm Configure Register to decide which GPIO been chosen"]
    pub etm_task_p5_cfg: ETM_TASK_P5_CFG,
    #[doc = "0xb8 - Etm Configure Register to decide which GPIO been chosen"]
    pub etm_task_p6_cfg: ETM_TASK_P6_CFG,
    #[doc = "0xbc - Etm Configure Register to decide which GPIO been chosen"]
    pub etm_task_p7_cfg: ETM_TASK_P7_CFG,
    _reserved13: [u8; 0x3c],
    #[doc = "0xfc - Version Control Register"]
    pub version: VERSION,
}
#[doc = "SIGMADELTA (rw) register accessor: Duty Cycle Configure Register of SDM%s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sigmadelta`] module"]
pub type SIGMADELTA = crate::Reg<sigmadelta::SIGMADELTA_SPEC>;
#[doc = "Duty Cycle Configure Register of SDM%s"]
pub mod sigmadelta;
#[doc = "CLOCK_GATE (rw) register accessor: Clock Gating Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Clock Gating Configure Register"]
pub mod clock_gate;
#[doc = "SIGMADELTA_MISC (rw) register accessor: MISC Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta_misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta_misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sigmadelta_misc`] module"]
pub type SIGMADELTA_MISC = crate::Reg<sigmadelta_misc::SIGMADELTA_MISC_SPEC>;
#[doc = "MISC Register"]
pub mod sigmadelta_misc;
#[doc = "GLITCH_FILTER_CH (rw) register accessor: Glitch Filter Configure Register of Channel%s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glitch_filter_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glitch_filter_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`glitch_filter_ch`] module"]
pub type GLITCH_FILTER_CH = crate::Reg<glitch_filter_ch::GLITCH_FILTER_CH_SPEC>;
#[doc = "Glitch Filter Configure Register of Channel%s"]
pub mod glitch_filter_ch;
#[doc = "ETM_EVENT_CH_CFG (rw) register accessor: Etm Config register of Channel%s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etm_event_ch_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_event_ch_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`etm_event_ch_cfg`] module"]
pub type ETM_EVENT_CH_CFG = crate::Reg<etm_event_ch_cfg::ETM_EVENT_CH_CFG_SPEC>;
#[doc = "Etm Config register of Channel%s"]
pub mod etm_event_ch_cfg;
#[doc = "ETM_TASK_P0_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etm_task_p0_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_task_p0_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`etm_task_p0_cfg`] module"]
pub type ETM_TASK_P0_CFG = crate::Reg<etm_task_p0_cfg::ETM_TASK_P0_CFG_SPEC>;
#[doc = "Etm Configure Register to decide which GPIO been chosen"]
pub mod etm_task_p0_cfg;
#[doc = "ETM_TASK_P1_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etm_task_p1_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_task_p1_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`etm_task_p1_cfg`] module"]
pub type ETM_TASK_P1_CFG = crate::Reg<etm_task_p1_cfg::ETM_TASK_P1_CFG_SPEC>;
#[doc = "Etm Configure Register to decide which GPIO been chosen"]
pub mod etm_task_p1_cfg;
#[doc = "ETM_TASK_P2_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etm_task_p2_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_task_p2_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`etm_task_p2_cfg`] module"]
pub type ETM_TASK_P2_CFG = crate::Reg<etm_task_p2_cfg::ETM_TASK_P2_CFG_SPEC>;
#[doc = "Etm Configure Register to decide which GPIO been chosen"]
pub mod etm_task_p2_cfg;
#[doc = "ETM_TASK_P3_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etm_task_p3_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_task_p3_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`etm_task_p3_cfg`] module"]
pub type ETM_TASK_P3_CFG = crate::Reg<etm_task_p3_cfg::ETM_TASK_P3_CFG_SPEC>;
#[doc = "Etm Configure Register to decide which GPIO been chosen"]
pub mod etm_task_p3_cfg;
#[doc = "ETM_TASK_P4_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etm_task_p4_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_task_p4_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`etm_task_p4_cfg`] module"]
pub type ETM_TASK_P4_CFG = crate::Reg<etm_task_p4_cfg::ETM_TASK_P4_CFG_SPEC>;
#[doc = "Etm Configure Register to decide which GPIO been chosen"]
pub mod etm_task_p4_cfg;
#[doc = "ETM_TASK_P5_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etm_task_p5_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_task_p5_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`etm_task_p5_cfg`] module"]
pub type ETM_TASK_P5_CFG = crate::Reg<etm_task_p5_cfg::ETM_TASK_P5_CFG_SPEC>;
#[doc = "Etm Configure Register to decide which GPIO been chosen"]
pub mod etm_task_p5_cfg;
#[doc = "ETM_TASK_P6_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etm_task_p6_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_task_p6_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`etm_task_p6_cfg`] module"]
pub type ETM_TASK_P6_CFG = crate::Reg<etm_task_p6_cfg::ETM_TASK_P6_CFG_SPEC>;
#[doc = "Etm Configure Register to decide which GPIO been chosen"]
pub mod etm_task_p6_cfg;
#[doc = "ETM_TASK_P7_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etm_task_p7_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_task_p7_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`etm_task_p7_cfg`] module"]
pub type ETM_TASK_P7_CFG = crate::Reg<etm_task_p7_cfg::ETM_TASK_P7_CFG_SPEC>;
#[doc = "Etm Configure Register to decide which GPIO been chosen"]
pub mod etm_task_p7_cfg;
#[doc = "VERSION (rw) register accessor: Version Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`version::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`version`] module"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Control Register"]
pub mod version;
