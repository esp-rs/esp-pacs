#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    sigmadelta: [SIGMADELTA; 4],
    _reserved1: [u8; 0x10],
    clock_gate: CLOCK_GATE,
    sigmadelta_misc: SIGMADELTA_MISC,
    pad_comp_config: PAD_COMP_CONFIG,
    pad_comp_filter: PAD_COMP_FILTER,
    glitch_filter_ch: [GLITCH_FILTER_CH; 8],
    _reserved6: [u8; 0x10],
    etm_event_ch_cfg: [ETM_EVENT_CH_CFG; 8],
    _reserved7: [u8; 0x20],
    etm_task_p0_cfg: ETM_TASK_P0_CFG,
    etm_task_p1_cfg: ETM_TASK_P1_CFG,
    etm_task_p2_cfg: ETM_TASK_P2_CFG,
    etm_task_p3_cfg: ETM_TASK_P3_CFG,
    etm_task_p4_cfg: ETM_TASK_P4_CFG,
    etm_task_p5_cfg: ETM_TASK_P5_CFG,
    etm_task_p6_cfg: ETM_TASK_P6_CFG,
    _reserved14: [u8; 0x24],
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    _reserved18: [u8; 0x0c],
    version: VERSION,
}
impl RegisterBlock {
    #[doc = "0x00..0x10 - Duty Cycle Configure Register of SDM%s"]
    #[inline(always)]
    pub const fn sigmadelta(&self, n: usize) -> &SIGMADELTA {
        &self.sigmadelta[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x10 - Duty Cycle Configure Register of SDM%s"]
    #[inline(always)]
    pub fn sigmadelta_iter(&self) -> impl Iterator<Item = &SIGMADELTA> {
        self.sigmadelta.iter()
    }
    #[doc = "0x20 - Clock Gating Configure Register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x24 - MISC Register"]
    #[inline(always)]
    pub const fn sigmadelta_misc(&self) -> &SIGMADELTA_MISC {
        &self.sigmadelta_misc
    }
    #[doc = "0x28 - PAD Compare configure Register"]
    #[inline(always)]
    pub const fn pad_comp_config(&self) -> &PAD_COMP_CONFIG {
        &self.pad_comp_config
    }
    #[doc = "0x2c - Zero Detect filter Register"]
    #[inline(always)]
    pub const fn pad_comp_filter(&self) -> &PAD_COMP_FILTER {
        &self.pad_comp_filter
    }
    #[doc = "0x30..0x50 - Glitch Filter Configure Register of Channel%s"]
    #[inline(always)]
    pub const fn glitch_filter_ch(&self, n: usize) -> &GLITCH_FILTER_CH {
        &self.glitch_filter_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x50 - Glitch Filter Configure Register of Channel%s"]
    #[inline(always)]
    pub fn glitch_filter_ch_iter(&self) -> impl Iterator<Item = &GLITCH_FILTER_CH> {
        self.glitch_filter_ch.iter()
    }
    #[doc = "0x60..0x80 - Etm Config register of Channel%s"]
    #[inline(always)]
    pub const fn etm_event_ch_cfg(&self, n: usize) -> &ETM_EVENT_CH_CFG {
        &self.etm_event_ch_cfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x60..0x80 - Etm Config register of Channel%s"]
    #[inline(always)]
    pub fn etm_event_ch_cfg_iter(&self) -> impl Iterator<Item = &ETM_EVENT_CH_CFG> {
        self.etm_event_ch_cfg.iter()
    }
    #[doc = "0x60 - Etm Config register of Channel0"]
    #[inline(always)]
    pub const fn etm_event_ch0_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(0)
    }
    #[doc = "0x64 - Etm Config register of Channel1"]
    #[inline(always)]
    pub const fn etm_event_ch1_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(1)
    }
    #[doc = "0x68 - Etm Config register of Channel2"]
    #[inline(always)]
    pub const fn etm_event_ch2_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(2)
    }
    #[doc = "0x6c - Etm Config register of Channel3"]
    #[inline(always)]
    pub const fn etm_event_ch3_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(3)
    }
    #[doc = "0x70 - Etm Config register of Channel4"]
    #[inline(always)]
    pub const fn etm_event_ch4_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(4)
    }
    #[doc = "0x74 - Etm Config register of Channel5"]
    #[inline(always)]
    pub const fn etm_event_ch5_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(5)
    }
    #[doc = "0x78 - Etm Config register of Channel6"]
    #[inline(always)]
    pub const fn etm_event_ch6_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(6)
    }
    #[doc = "0x7c - Etm Config register of Channel7"]
    #[inline(always)]
    pub const fn etm_event_ch7_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(7)
    }
    #[doc = "0xa0 - Etm Configure Register to decide which GPIO been chosen"]
    #[inline(always)]
    pub const fn etm_task_p0_cfg(&self) -> &ETM_TASK_P0_CFG {
        &self.etm_task_p0_cfg
    }
    #[doc = "0xa4 - Etm Configure Register to decide which GPIO been chosen"]
    #[inline(always)]
    pub const fn etm_task_p1_cfg(&self) -> &ETM_TASK_P1_CFG {
        &self.etm_task_p1_cfg
    }
    #[doc = "0xa8 - Etm Configure Register to decide which GPIO been chosen"]
    #[inline(always)]
    pub const fn etm_task_p2_cfg(&self) -> &ETM_TASK_P2_CFG {
        &self.etm_task_p2_cfg
    }
    #[doc = "0xac - Etm Configure Register to decide which GPIO been chosen"]
    #[inline(always)]
    pub const fn etm_task_p3_cfg(&self) -> &ETM_TASK_P3_CFG {
        &self.etm_task_p3_cfg
    }
    #[doc = "0xb0 - Etm Configure Register to decide which GPIO been chosen"]
    #[inline(always)]
    pub const fn etm_task_p4_cfg(&self) -> &ETM_TASK_P4_CFG {
        &self.etm_task_p4_cfg
    }
    #[doc = "0xb4 - Etm Configure Register to decide which GPIO been chosen"]
    #[inline(always)]
    pub const fn etm_task_p5_cfg(&self) -> &ETM_TASK_P5_CFG {
        &self.etm_task_p5_cfg
    }
    #[doc = "0xb8 - Etm Configure Register to decide which GPIO been chosen"]
    #[inline(always)]
    pub const fn etm_task_p6_cfg(&self) -> &ETM_TASK_P6_CFG {
        &self.etm_task_p6_cfg
    }
    #[doc = "0xe0 - GPIOSD interrupt raw register"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0xe4 - GPIOSD interrupt masked register"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0xe8 - GPIOSD interrupt enable register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0xec - GPIOSD interrupt clear register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0xfc - Version Control Register"]
    #[inline(always)]
    pub const fn version(&self) -> &VERSION {
        &self.version
    }
}
#[doc = "SIGMADELTA (rw) register accessor: Duty Cycle Configure Register of SDM%s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigmadelta`] module"]
pub type SIGMADELTA = crate::Reg<sigmadelta::SIGMADELTA_SPEC>;
#[doc = "Duty Cycle Configure Register of SDM%s"]
pub mod sigmadelta;
#[doc = "CLOCK_GATE (rw) register accessor: Clock Gating Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Clock Gating Configure Register"]
pub mod clock_gate;
#[doc = "SIGMADELTA_MISC (rw) register accessor: MISC Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta_misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta_misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigmadelta_misc`] module"]
pub type SIGMADELTA_MISC = crate::Reg<sigmadelta_misc::SIGMADELTA_MISC_SPEC>;
#[doc = "MISC Register"]
pub mod sigmadelta_misc;
#[doc = "PAD_COMP_CONFIG (rw) register accessor: PAD Compare configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_comp_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_comp_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_comp_config`] module"]
pub type PAD_COMP_CONFIG = crate::Reg<pad_comp_config::PAD_COMP_CONFIG_SPEC>;
#[doc = "PAD Compare configure Register"]
pub mod pad_comp_config;
#[doc = "PAD_COMP_FILTER (rw) register accessor: Zero Detect filter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_comp_filter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_comp_filter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_comp_filter`] module"]
pub type PAD_COMP_FILTER = crate::Reg<pad_comp_filter::PAD_COMP_FILTER_SPEC>;
#[doc = "Zero Detect filter Register"]
pub mod pad_comp_filter;
#[doc = "GLITCH_FILTER_CH (rw) register accessor: Glitch Filter Configure Register of Channel%s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glitch_filter_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glitch_filter_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glitch_filter_ch`] module"]
pub type GLITCH_FILTER_CH = crate::Reg<glitch_filter_ch::GLITCH_FILTER_CH_SPEC>;
#[doc = "Glitch Filter Configure Register of Channel%s"]
pub mod glitch_filter_ch;
#[doc = "ETM_EVENT_CH_CFG (rw) register accessor: Etm Config register of Channel%s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etm_event_ch_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_event_ch_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_event_ch_cfg`] module"]
pub type ETM_EVENT_CH_CFG = crate::Reg<etm_event_ch_cfg::ETM_EVENT_CH_CFG_SPEC>;
#[doc = "Etm Config register of Channel%s"]
pub mod etm_event_ch_cfg;
#[doc = "ETM_TASK_P0_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etm_task_p0_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_task_p0_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_task_p0_cfg`] module"]
pub type ETM_TASK_P0_CFG = crate::Reg<etm_task_p0_cfg::ETM_TASK_P0_CFG_SPEC>;
#[doc = "Etm Configure Register to decide which GPIO been chosen"]
pub mod etm_task_p0_cfg;
#[doc = "ETM_TASK_P1_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etm_task_p1_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_task_p1_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_task_p1_cfg`] module"]
pub type ETM_TASK_P1_CFG = crate::Reg<etm_task_p1_cfg::ETM_TASK_P1_CFG_SPEC>;
#[doc = "Etm Configure Register to decide which GPIO been chosen"]
pub mod etm_task_p1_cfg;
#[doc = "ETM_TASK_P2_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etm_task_p2_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_task_p2_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_task_p2_cfg`] module"]
pub type ETM_TASK_P2_CFG = crate::Reg<etm_task_p2_cfg::ETM_TASK_P2_CFG_SPEC>;
#[doc = "Etm Configure Register to decide which GPIO been chosen"]
pub mod etm_task_p2_cfg;
#[doc = "ETM_TASK_P3_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etm_task_p3_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_task_p3_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_task_p3_cfg`] module"]
pub type ETM_TASK_P3_CFG = crate::Reg<etm_task_p3_cfg::ETM_TASK_P3_CFG_SPEC>;
#[doc = "Etm Configure Register to decide which GPIO been chosen"]
pub mod etm_task_p3_cfg;
#[doc = "ETM_TASK_P4_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etm_task_p4_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_task_p4_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_task_p4_cfg`] module"]
pub type ETM_TASK_P4_CFG = crate::Reg<etm_task_p4_cfg::ETM_TASK_P4_CFG_SPEC>;
#[doc = "Etm Configure Register to decide which GPIO been chosen"]
pub mod etm_task_p4_cfg;
#[doc = "ETM_TASK_P5_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etm_task_p5_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_task_p5_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_task_p5_cfg`] module"]
pub type ETM_TASK_P5_CFG = crate::Reg<etm_task_p5_cfg::ETM_TASK_P5_CFG_SPEC>;
#[doc = "Etm Configure Register to decide which GPIO been chosen"]
pub mod etm_task_p5_cfg;
#[doc = "ETM_TASK_P6_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etm_task_p6_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_task_p6_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_task_p6_cfg`] module"]
pub type ETM_TASK_P6_CFG = crate::Reg<etm_task_p6_cfg::ETM_TASK_P6_CFG_SPEC>;
#[doc = "Etm Configure Register to decide which GPIO been chosen"]
pub mod etm_task_p6_cfg;
#[doc = "INT_RAW (r) register accessor: GPIOSD interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "GPIOSD interrupt raw register"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: GPIOSD interrupt masked register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "GPIOSD interrupt masked register"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: GPIOSD interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "GPIOSD interrupt enable register"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: GPIOSD interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "GPIOSD interrupt clear register"]
pub mod int_clr;
#[doc = "VERSION (rw) register accessor: Version Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`version::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version`] module"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Control Register"]
pub mod version;
