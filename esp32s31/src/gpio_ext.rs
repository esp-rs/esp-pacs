#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    clock_gate: CLOCK_GATE,
    sigmadelta_misc: SIGMADELTA_MISC,
    sigmadelta: [SIGMADELTA; 8],
    _reserved3: [u8; 0xb0],
    glitch_filter_ch: [GLITCH_FILTER_CH; 8],
    _reserved4: [u8; 0x20],
    etm_event_ch_cfg: [ETM_EVENT_CH_CFG; 8],
    _reserved5: [u8; 0x20],
    etm_task_p0_cfg: ETM_TASK_P0_CFG,
    etm_task_p1_cfg: ETM_TASK_P1_CFG,
    etm_task_p2_cfg: ETM_TASK_P2_CFG,
    etm_task_p3_cfg: ETM_TASK_P3_CFG,
    etm_task_p4_cfg: ETM_TASK_P4_CFG,
    etm_task_p5_cfg: ETM_TASK_P5_CFG,
    etm_task_p6_cfg: ETM_TASK_P6_CFG,
    etm_task_p7_cfg: ETM_TASK_P7_CFG,
    etm_task_p8_cfg: ETM_TASK_P8_CFG,
    etm_task_p9_cfg: ETM_TASK_P9_CFG,
    etm_task_p10_cfg: ETM_TASK_P10_CFG,
    etm_task_p11_cfg: ETM_TASK_P11_CFG,
    etm_task_p12_cfg: ETM_TASK_P12_CFG,
    _reserved18: [u8; 0x70],
    version: VERSION,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock Gating Configure Register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x04 - MISC Register"]
    #[inline(always)]
    pub const fn sigmadelta_misc(&self) -> &SIGMADELTA_MISC {
        &self.sigmadelta_misc
    }
    #[doc = "0x08..0x28 - Duty cycle configuration register for SDM channel %s"]
    #[inline(always)]
    pub const fn sigmadelta(&self, n: usize) -> &SIGMADELTA {
        &self.sigmadelta[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x28 - Duty cycle configuration register for SDM channel %s"]
    #[inline(always)]
    pub fn sigmadelta_iter(&self) -> impl Iterator<Item = &SIGMADELTA> {
        self.sigmadelta.iter()
    }
    #[doc = "0xd8..0xf8 - Glitch Filter Configure Register of Channel%s"]
    #[inline(always)]
    pub const fn glitch_filter_ch(&self, n: usize) -> &GLITCH_FILTER_CH {
        &self.glitch_filter_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xd8..0xf8 - Glitch Filter Configure Register of Channel%s"]
    #[inline(always)]
    pub fn glitch_filter_ch_iter(&self) -> impl Iterator<Item = &GLITCH_FILTER_CH> {
        self.glitch_filter_ch.iter()
    }
    #[doc = "0x118..0x138 - ETM configuration register for channel %s"]
    #[inline(always)]
    pub const fn etm_event_ch_cfg(&self, n: usize) -> &ETM_EVENT_CH_CFG {
        &self.etm_event_ch_cfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x118..0x138 - ETM configuration register for channel %s"]
    #[inline(always)]
    pub fn etm_event_ch_cfg_iter(&self) -> impl Iterator<Item = &ETM_EVENT_CH_CFG> {
        self.etm_event_ch_cfg.iter()
    }
    #[doc = "0x118 - ETM configuration register for channel 0"]
    #[inline(always)]
    pub const fn etm_event_ch0_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(0)
    }
    #[doc = "0x11c - ETM configuration register for channel 1"]
    #[inline(always)]
    pub const fn etm_event_ch1_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(1)
    }
    #[doc = "0x120 - ETM configuration register for channel 2"]
    #[inline(always)]
    pub const fn etm_event_ch2_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(2)
    }
    #[doc = "0x124 - ETM configuration register for channel 3"]
    #[inline(always)]
    pub const fn etm_event_ch3_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(3)
    }
    #[doc = "0x128 - ETM configuration register for channel 4"]
    #[inline(always)]
    pub const fn etm_event_ch4_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(4)
    }
    #[doc = "0x12c - ETM configuration register for channel 5"]
    #[inline(always)]
    pub const fn etm_event_ch5_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(5)
    }
    #[doc = "0x130 - ETM configuration register for channel 6"]
    #[inline(always)]
    pub const fn etm_event_ch6_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(6)
    }
    #[doc = "0x134 - ETM configuration register for channel 7"]
    #[inline(always)]
    pub const fn etm_event_ch7_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(7)
    }
    #[doc = "0x158 - GPIO selection register 0 for ETM"]
    #[inline(always)]
    pub const fn etm_task_p0_cfg(&self) -> &ETM_TASK_P0_CFG {
        &self.etm_task_p0_cfg
    }
    #[doc = "0x15c - GPIO selection register 1 for ETM"]
    #[inline(always)]
    pub const fn etm_task_p1_cfg(&self) -> &ETM_TASK_P1_CFG {
        &self.etm_task_p1_cfg
    }
    #[doc = "0x160 - GPIO selection register 2 for ETM"]
    #[inline(always)]
    pub const fn etm_task_p2_cfg(&self) -> &ETM_TASK_P2_CFG {
        &self.etm_task_p2_cfg
    }
    #[doc = "0x164 - GPIO selection register 3 for ETM"]
    #[inline(always)]
    pub const fn etm_task_p3_cfg(&self) -> &ETM_TASK_P3_CFG {
        &self.etm_task_p3_cfg
    }
    #[doc = "0x168 - GPIO selection register 4 for ETM"]
    #[inline(always)]
    pub const fn etm_task_p4_cfg(&self) -> &ETM_TASK_P4_CFG {
        &self.etm_task_p4_cfg
    }
    #[doc = "0x16c - GPIO selection register 5 for ETM"]
    #[inline(always)]
    pub const fn etm_task_p5_cfg(&self) -> &ETM_TASK_P5_CFG {
        &self.etm_task_p5_cfg
    }
    #[doc = "0x170 - GPIO selection register 6 for ETM"]
    #[inline(always)]
    pub const fn etm_task_p6_cfg(&self) -> &ETM_TASK_P6_CFG {
        &self.etm_task_p6_cfg
    }
    #[doc = "0x174 - GPIO selection register 7 for ETM"]
    #[inline(always)]
    pub const fn etm_task_p7_cfg(&self) -> &ETM_TASK_P7_CFG {
        &self.etm_task_p7_cfg
    }
    #[doc = "0x178 - GPIO selection register 8 for ETM"]
    #[inline(always)]
    pub const fn etm_task_p8_cfg(&self) -> &ETM_TASK_P8_CFG {
        &self.etm_task_p8_cfg
    }
    #[doc = "0x17c - GPIO selection register 9 for ETM"]
    #[inline(always)]
    pub const fn etm_task_p9_cfg(&self) -> &ETM_TASK_P9_CFG {
        &self.etm_task_p9_cfg
    }
    #[doc = "0x180 - GPIO selection register 10 for ETM"]
    #[inline(always)]
    pub const fn etm_task_p10_cfg(&self) -> &ETM_TASK_P10_CFG {
        &self.etm_task_p10_cfg
    }
    #[doc = "0x184 - GPIO selection register 11 for ETM"]
    #[inline(always)]
    pub const fn etm_task_p11_cfg(&self) -> &ETM_TASK_P11_CFG {
        &self.etm_task_p11_cfg
    }
    #[doc = "0x188 - GPIO selection register 12 for ETM"]
    #[inline(always)]
    pub const fn etm_task_p12_cfg(&self) -> &ETM_TASK_P12_CFG {
        &self.etm_task_p12_cfg
    }
    #[doc = "0x1fc - Version control register"]
    #[inline(always)]
    pub const fn version(&self) -> &VERSION {
        &self.version
    }
}
#[doc = "CLOCK_GATE (rw) register accessor: Clock Gating Configure Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Clock Gating Configure Register"]
pub mod clock_gate;
#[doc = "SIGMADELTA_MISC (rw) register accessor: MISC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sigmadelta_misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigmadelta_misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigmadelta_misc`] module"]
pub type SIGMADELTA_MISC = crate::Reg<sigmadelta_misc::SIGMADELTA_MISC_SPEC>;
#[doc = "MISC Register"]
pub mod sigmadelta_misc;
#[doc = "SIGMADELTA (rw) register accessor: Duty cycle configuration register for SDM channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`sigmadelta::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigmadelta::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigmadelta`] module"]
pub type SIGMADELTA = crate::Reg<sigmadelta::SIGMADELTA_SPEC>;
#[doc = "Duty cycle configuration register for SDM channel %s"]
pub mod sigmadelta;
#[doc = "GLITCH_FILTER_CH (rw) register accessor: Glitch Filter Configure Register of Channel%s\n\nYou can [`read`](crate::Reg::read) this register and get [`glitch_filter_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`glitch_filter_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glitch_filter_ch`] module"]
pub type GLITCH_FILTER_CH = crate::Reg<glitch_filter_ch::GLITCH_FILTER_CH_SPEC>;
#[doc = "Glitch Filter Configure Register of Channel%s"]
pub mod glitch_filter_ch;
#[doc = "ETM_EVENT_CH_CFG (rw) register accessor: ETM configuration register for channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_event_ch_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_event_ch_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_event_ch_cfg`] module"]
pub type ETM_EVENT_CH_CFG = crate::Reg<etm_event_ch_cfg::ETM_EVENT_CH_CFG_SPEC>;
#[doc = "ETM configuration register for channel %s"]
pub mod etm_event_ch_cfg;
#[doc = "ETM_TASK_P0_CFG (rw) register accessor: GPIO selection register 0 for ETM\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p0_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p0_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_task_p0_cfg`] module"]
pub type ETM_TASK_P0_CFG = crate::Reg<etm_task_p0_cfg::ETM_TASK_P0_CFG_SPEC>;
#[doc = "GPIO selection register 0 for ETM"]
pub mod etm_task_p0_cfg;
#[doc = "ETM_TASK_P1_CFG (rw) register accessor: GPIO selection register 1 for ETM\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p1_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p1_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_task_p1_cfg`] module"]
pub type ETM_TASK_P1_CFG = crate::Reg<etm_task_p1_cfg::ETM_TASK_P1_CFG_SPEC>;
#[doc = "GPIO selection register 1 for ETM"]
pub mod etm_task_p1_cfg;
#[doc = "ETM_TASK_P2_CFG (rw) register accessor: GPIO selection register 2 for ETM\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p2_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p2_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_task_p2_cfg`] module"]
pub type ETM_TASK_P2_CFG = crate::Reg<etm_task_p2_cfg::ETM_TASK_P2_CFG_SPEC>;
#[doc = "GPIO selection register 2 for ETM"]
pub mod etm_task_p2_cfg;
#[doc = "ETM_TASK_P3_CFG (rw) register accessor: GPIO selection register 3 for ETM\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p3_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p3_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_task_p3_cfg`] module"]
pub type ETM_TASK_P3_CFG = crate::Reg<etm_task_p3_cfg::ETM_TASK_P3_CFG_SPEC>;
#[doc = "GPIO selection register 3 for ETM"]
pub mod etm_task_p3_cfg;
#[doc = "ETM_TASK_P4_CFG (rw) register accessor: GPIO selection register 4 for ETM\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p4_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p4_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_task_p4_cfg`] module"]
pub type ETM_TASK_P4_CFG = crate::Reg<etm_task_p4_cfg::ETM_TASK_P4_CFG_SPEC>;
#[doc = "GPIO selection register 4 for ETM"]
pub mod etm_task_p4_cfg;
#[doc = "ETM_TASK_P5_CFG (rw) register accessor: GPIO selection register 5 for ETM\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p5_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p5_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_task_p5_cfg`] module"]
pub type ETM_TASK_P5_CFG = crate::Reg<etm_task_p5_cfg::ETM_TASK_P5_CFG_SPEC>;
#[doc = "GPIO selection register 5 for ETM"]
pub mod etm_task_p5_cfg;
#[doc = "ETM_TASK_P6_CFG (rw) register accessor: GPIO selection register 6 for ETM\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p6_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p6_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_task_p6_cfg`] module"]
pub type ETM_TASK_P6_CFG = crate::Reg<etm_task_p6_cfg::ETM_TASK_P6_CFG_SPEC>;
#[doc = "GPIO selection register 6 for ETM"]
pub mod etm_task_p6_cfg;
#[doc = "ETM_TASK_P7_CFG (rw) register accessor: GPIO selection register 7 for ETM\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p7_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p7_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_task_p7_cfg`] module"]
pub type ETM_TASK_P7_CFG = crate::Reg<etm_task_p7_cfg::ETM_TASK_P7_CFG_SPEC>;
#[doc = "GPIO selection register 7 for ETM"]
pub mod etm_task_p7_cfg;
#[doc = "ETM_TASK_P8_CFG (rw) register accessor: GPIO selection register 8 for ETM\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p8_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p8_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_task_p8_cfg`] module"]
pub type ETM_TASK_P8_CFG = crate::Reg<etm_task_p8_cfg::ETM_TASK_P8_CFG_SPEC>;
#[doc = "GPIO selection register 8 for ETM"]
pub mod etm_task_p8_cfg;
#[doc = "ETM_TASK_P9_CFG (rw) register accessor: GPIO selection register 9 for ETM\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p9_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p9_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_task_p9_cfg`] module"]
pub type ETM_TASK_P9_CFG = crate::Reg<etm_task_p9_cfg::ETM_TASK_P9_CFG_SPEC>;
#[doc = "GPIO selection register 9 for ETM"]
pub mod etm_task_p9_cfg;
#[doc = "ETM_TASK_P10_CFG (rw) register accessor: GPIO selection register 10 for ETM\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p10_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p10_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_task_p10_cfg`] module"]
pub type ETM_TASK_P10_CFG = crate::Reg<etm_task_p10_cfg::ETM_TASK_P10_CFG_SPEC>;
#[doc = "GPIO selection register 10 for ETM"]
pub mod etm_task_p10_cfg;
#[doc = "ETM_TASK_P11_CFG (rw) register accessor: GPIO selection register 11 for ETM\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p11_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p11_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_task_p11_cfg`] module"]
pub type ETM_TASK_P11_CFG = crate::Reg<etm_task_p11_cfg::ETM_TASK_P11_CFG_SPEC>;
#[doc = "GPIO selection register 11 for ETM"]
pub mod etm_task_p11_cfg;
#[doc = "ETM_TASK_P12_CFG (rw) register accessor: GPIO selection register 12 for ETM\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p12_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p12_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_task_p12_cfg`] module"]
pub type ETM_TASK_P12_CFG = crate::Reg<etm_task_p12_cfg::ETM_TASK_P12_CFG_SPEC>;
#[doc = "GPIO selection register 12 for ETM"]
pub mod etm_task_p12_cfg;
#[doc = "VERSION (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`version::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`version::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version`] module"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version control register"]
pub mod version;
