#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    sigmadelta: [SIGMADELTA; 4],
    _reserved1: [u8; 0x10],
    clock_gate: CLOCK_GATE,
    sigmadelta_misc: SIGMADELTA_MISC,
    _reserved3: [u8; 0x08],
    glitch_filter_ch: [GLITCH_FILTER_CH; 8],
    _reserved4: [u8; 0x10],
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
    _reserved13: [u8; 0x3c],
    version: VERSION,
}
impl RegisterBlock {
    ///0x00..0x10 - Duty Cycle Configure Register of SDM%s
    #[inline(always)]
    pub const fn sigmadelta(&self, n: usize) -> &SIGMADELTA {
        &self.sigmadelta[n]
    }
    ///Iterator for array of:
    ///0x00..0x10 - Duty Cycle Configure Register of SDM%s
    #[inline(always)]
    pub fn sigmadelta_iter(&self) -> impl Iterator<Item = &SIGMADELTA> {
        self.sigmadelta.iter()
    }
    ///0x20 - Clock Gating Configure Register
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    ///0x24 - MISC Register
    #[inline(always)]
    pub const fn sigmadelta_misc(&self) -> &SIGMADELTA_MISC {
        &self.sigmadelta_misc
    }
    ///0x30..0x50 - Glitch Filter Configure Register of Channel%s
    #[inline(always)]
    pub const fn glitch_filter_ch(&self, n: usize) -> &GLITCH_FILTER_CH {
        &self.glitch_filter_ch[n]
    }
    ///Iterator for array of:
    ///0x30..0x50 - Glitch Filter Configure Register of Channel%s
    #[inline(always)]
    pub fn glitch_filter_ch_iter(&self) -> impl Iterator<Item = &GLITCH_FILTER_CH> {
        self.glitch_filter_ch.iter()
    }
    ///0x60..0x80 - Etm Config register of Channel%s
    #[inline(always)]
    pub const fn etm_event_ch_cfg(&self, n: usize) -> &ETM_EVENT_CH_CFG {
        &self.etm_event_ch_cfg[n]
    }
    ///Iterator for array of:
    ///0x60..0x80 - Etm Config register of Channel%s
    #[inline(always)]
    pub fn etm_event_ch_cfg_iter(&self) -> impl Iterator<Item = &ETM_EVENT_CH_CFG> {
        self.etm_event_ch_cfg.iter()
    }
    ///0x60 - Etm Config register of Channel0
    #[inline(always)]
    pub const fn etm_event_ch0_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(0)
    }
    ///0x64 - Etm Config register of Channel1
    #[inline(always)]
    pub const fn etm_event_ch1_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(1)
    }
    ///0x68 - Etm Config register of Channel2
    #[inline(always)]
    pub const fn etm_event_ch2_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(2)
    }
    ///0x6c - Etm Config register of Channel3
    #[inline(always)]
    pub const fn etm_event_ch3_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(3)
    }
    ///0x70 - Etm Config register of Channel4
    #[inline(always)]
    pub const fn etm_event_ch4_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(4)
    }
    ///0x74 - Etm Config register of Channel5
    #[inline(always)]
    pub const fn etm_event_ch5_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(5)
    }
    ///0x78 - Etm Config register of Channel6
    #[inline(always)]
    pub const fn etm_event_ch6_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(6)
    }
    ///0x7c - Etm Config register of Channel7
    #[inline(always)]
    pub const fn etm_event_ch7_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(7)
    }
    ///0xa0 - Etm Configure Register to decide which GPIO been chosen
    #[inline(always)]
    pub const fn etm_task_p0_cfg(&self) -> &ETM_TASK_P0_CFG {
        &self.etm_task_p0_cfg
    }
    ///0xa4 - Etm Configure Register to decide which GPIO been chosen
    #[inline(always)]
    pub const fn etm_task_p1_cfg(&self) -> &ETM_TASK_P1_CFG {
        &self.etm_task_p1_cfg
    }
    ///0xa8 - Etm Configure Register to decide which GPIO been chosen
    #[inline(always)]
    pub const fn etm_task_p2_cfg(&self) -> &ETM_TASK_P2_CFG {
        &self.etm_task_p2_cfg
    }
    ///0xac - Etm Configure Register to decide which GPIO been chosen
    #[inline(always)]
    pub const fn etm_task_p3_cfg(&self) -> &ETM_TASK_P3_CFG {
        &self.etm_task_p3_cfg
    }
    ///0xb0 - Etm Configure Register to decide which GPIO been chosen
    #[inline(always)]
    pub const fn etm_task_p4_cfg(&self) -> &ETM_TASK_P4_CFG {
        &self.etm_task_p4_cfg
    }
    ///0xb4 - Etm Configure Register to decide which GPIO been chosen
    #[inline(always)]
    pub const fn etm_task_p5_cfg(&self) -> &ETM_TASK_P5_CFG {
        &self.etm_task_p5_cfg
    }
    ///0xb8 - Etm Configure Register to decide which GPIO been chosen
    #[inline(always)]
    pub const fn etm_task_p6_cfg(&self) -> &ETM_TASK_P6_CFG {
        &self.etm_task_p6_cfg
    }
    ///0xbc - Etm Configure Register to decide which GPIO been chosen
    #[inline(always)]
    pub const fn etm_task_p7_cfg(&self) -> &ETM_TASK_P7_CFG {
        &self.etm_task_p7_cfg
    }
    ///0xfc - Version Control Register
    #[inline(always)]
    pub const fn version(&self) -> &VERSION {
        &self.version
    }
}
/**SIGMADELTA (rw) register accessor: Duty Cycle Configure Register of SDM%s

You can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sigmadelta`] module*/
pub type SIGMADELTA = crate::Reg<sigmadelta::SIGMADELTA_SPEC>;
///Duty Cycle Configure Register of SDM%s
pub mod sigmadelta;
/**CLOCK_GATE (rw) register accessor: Clock Gating Configure Register

You can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clock_gate`] module*/
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
///Clock Gating Configure Register
pub mod clock_gate;
/**SIGMADELTA_MISC (rw) register accessor: MISC Register

You can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta_misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta_misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sigmadelta_misc`] module*/
pub type SIGMADELTA_MISC = crate::Reg<sigmadelta_misc::SIGMADELTA_MISC_SPEC>;
///MISC Register
pub mod sigmadelta_misc;
/**GLITCH_FILTER_CH (rw) register accessor: Glitch Filter Configure Register of Channel%s

You can [`read`](crate::generic::Reg::read) this register and get [`glitch_filter_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glitch_filter_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@glitch_filter_ch`] module*/
pub type GLITCH_FILTER_CH = crate::Reg<glitch_filter_ch::GLITCH_FILTER_CH_SPEC>;
///Glitch Filter Configure Register of Channel%s
pub mod glitch_filter_ch;
/**ETM_EVENT_CH_CFG (rw) register accessor: Etm Config register of Channel%s

You can [`read`](crate::generic::Reg::read) this register and get [`etm_event_ch_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_event_ch_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@etm_event_ch_cfg`] module*/
pub type ETM_EVENT_CH_CFG = crate::Reg<etm_event_ch_cfg::ETM_EVENT_CH_CFG_SPEC>;
///Etm Config register of Channel%s
pub mod etm_event_ch_cfg;
/**ETM_TASK_P0_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen

You can [`read`](crate::generic::Reg::read) this register and get [`etm_task_p0_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_task_p0_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@etm_task_p0_cfg`] module*/
pub type ETM_TASK_P0_CFG = crate::Reg<etm_task_p0_cfg::ETM_TASK_P0_CFG_SPEC>;
///Etm Configure Register to decide which GPIO been chosen
pub mod etm_task_p0_cfg;
/**ETM_TASK_P1_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen

You can [`read`](crate::generic::Reg::read) this register and get [`etm_task_p1_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_task_p1_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@etm_task_p1_cfg`] module*/
pub type ETM_TASK_P1_CFG = crate::Reg<etm_task_p1_cfg::ETM_TASK_P1_CFG_SPEC>;
///Etm Configure Register to decide which GPIO been chosen
pub mod etm_task_p1_cfg;
/**ETM_TASK_P2_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen

You can [`read`](crate::generic::Reg::read) this register and get [`etm_task_p2_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_task_p2_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@etm_task_p2_cfg`] module*/
pub type ETM_TASK_P2_CFG = crate::Reg<etm_task_p2_cfg::ETM_TASK_P2_CFG_SPEC>;
///Etm Configure Register to decide which GPIO been chosen
pub mod etm_task_p2_cfg;
/**ETM_TASK_P3_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen

You can [`read`](crate::generic::Reg::read) this register and get [`etm_task_p3_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_task_p3_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@etm_task_p3_cfg`] module*/
pub type ETM_TASK_P3_CFG = crate::Reg<etm_task_p3_cfg::ETM_TASK_P3_CFG_SPEC>;
///Etm Configure Register to decide which GPIO been chosen
pub mod etm_task_p3_cfg;
/**ETM_TASK_P4_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen

You can [`read`](crate::generic::Reg::read) this register and get [`etm_task_p4_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_task_p4_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@etm_task_p4_cfg`] module*/
pub type ETM_TASK_P4_CFG = crate::Reg<etm_task_p4_cfg::ETM_TASK_P4_CFG_SPEC>;
///Etm Configure Register to decide which GPIO been chosen
pub mod etm_task_p4_cfg;
/**ETM_TASK_P5_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen

You can [`read`](crate::generic::Reg::read) this register and get [`etm_task_p5_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_task_p5_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@etm_task_p5_cfg`] module*/
pub type ETM_TASK_P5_CFG = crate::Reg<etm_task_p5_cfg::ETM_TASK_P5_CFG_SPEC>;
///Etm Configure Register to decide which GPIO been chosen
pub mod etm_task_p5_cfg;
/**ETM_TASK_P6_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen

You can [`read`](crate::generic::Reg::read) this register and get [`etm_task_p6_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_task_p6_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@etm_task_p6_cfg`] module*/
pub type ETM_TASK_P6_CFG = crate::Reg<etm_task_p6_cfg::ETM_TASK_P6_CFG_SPEC>;
///Etm Configure Register to decide which GPIO been chosen
pub mod etm_task_p6_cfg;
/**ETM_TASK_P7_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen

You can [`read`](crate::generic::Reg::read) this register and get [`etm_task_p7_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_task_p7_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@etm_task_p7_cfg`] module*/
pub type ETM_TASK_P7_CFG = crate::Reg<etm_task_p7_cfg::ETM_TASK_P7_CFG_SPEC>;
///Etm Configure Register to decide which GPIO been chosen
pub mod etm_task_p7_cfg;
/**VERSION (rw) register accessor: Version Control Register

You can [`read`](crate::generic::Reg::read) this register and get [`version::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`version::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@version`] module*/
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
///Version Control Register
pub mod version;
