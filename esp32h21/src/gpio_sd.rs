#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    gpio_ext_clock_gate: GPIO_EXT_CLOCK_GATE,
    gpio_ext_sigmadelta_misc: GPIO_EXT_SIGMADELTA_MISC,
    gpio_ext_sigmadelta: [GPIO_EXT_SIGMADELTA; 4],
    _reserved3: [u8; 0x40],
    gpio_ext_pad_comp_config_0: GPIO_EXT_PAD_COMP_CONFIG_0,
    gpio_ext_pad_comp_filter_0: GPIO_EXT_PAD_COMP_FILTER_0,
    _reserved5: [u8; 0x78],
    gpio_ext_glitch_filter_ch: [GPIO_EXT_GLITCH_FILTER_CH; 8],
    _reserved6: [u8; 0x20],
    gpio_ext_etm_event_ch_cfg: [GPIO_EXT_ETM_EVENT_CH_CFG; 8],
    _reserved7: [u8; 0x20],
    gpio_ext_etm_task_p0_cfg: GPIO_EXT_ETM_TASK_P0_CFG,
    gpio_ext_etm_task_p1_cfg: GPIO_EXT_ETM_TASK_P1_CFG,
    gpio_ext_etm_task_p2_cfg: GPIO_EXT_ETM_TASK_P2_CFG,
    gpio_ext_etm_task_p3_cfg: GPIO_EXT_ETM_TASK_P3_CFG,
    gpio_ext_etm_task_p4_cfg: GPIO_EXT_ETM_TASK_P4_CFG,
    gpio_ext_etm_task_p5_cfg: GPIO_EXT_ETM_TASK_P5_CFG,
    _reserved13: [u8; 0x60],
    gpio_ext_int_raw: GPIO_EXT_INT_RAW,
    gpio_ext_int_st: GPIO_EXT_INT_ST,
    gpio_ext_int_ena: GPIO_EXT_INT_ENA,
    gpio_ext_int_clr: GPIO_EXT_INT_CLR,
    gpio_ext_pin_ctrl: GPIO_EXT_PIN_CTRL,
    _reserved18: [u8; 0x18],
    gpio_ext_version: GPIO_EXT_VERSION,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock Gating Configure Register"]
    #[inline(always)]
    pub const fn gpio_ext_clock_gate(&self) -> &GPIO_EXT_CLOCK_GATE {
        &self.gpio_ext_clock_gate
    }
    #[doc = "0x04 - MISC Register"]
    #[inline(always)]
    pub const fn gpio_ext_sigmadelta_misc(&self) -> &GPIO_EXT_SIGMADELTA_MISC {
        &self.gpio_ext_sigmadelta_misc
    }
    #[doc = "0x08..0x18 - Duty Cycle Configure Register of SDM%s"]
    #[inline(always)]
    pub const fn gpio_ext_sigmadelta(&self, n: usize) -> &GPIO_EXT_SIGMADELTA {
        &self.gpio_ext_sigmadelta[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x18 - Duty Cycle Configure Register of SDM%s"]
    #[inline(always)]
    pub fn gpio_ext_sigmadelta_iter(&self) -> impl Iterator<Item = &GPIO_EXT_SIGMADELTA> {
        self.gpio_ext_sigmadelta.iter()
    }
    #[doc = "0x58 - PAD Compare configure Register"]
    #[inline(always)]
    pub const fn gpio_ext_pad_comp_config_0(&self) -> &GPIO_EXT_PAD_COMP_CONFIG_0 {
        &self.gpio_ext_pad_comp_config_0
    }
    #[doc = "0x5c - Zero Detect filter Register"]
    #[inline(always)]
    pub const fn gpio_ext_pad_comp_filter_0(&self) -> &GPIO_EXT_PAD_COMP_FILTER_0 {
        &self.gpio_ext_pad_comp_filter_0
    }
    #[doc = "0xd8..0xf8 - Glitch Filter Configure Register of Channel%s"]
    #[inline(always)]
    pub const fn gpio_ext_glitch_filter_ch(&self, n: usize) -> &GPIO_EXT_GLITCH_FILTER_CH {
        &self.gpio_ext_glitch_filter_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xd8..0xf8 - Glitch Filter Configure Register of Channel%s"]
    #[inline(always)]
    pub fn gpio_ext_glitch_filter_ch_iter(
        &self,
    ) -> impl Iterator<Item = &GPIO_EXT_GLITCH_FILTER_CH> {
        self.gpio_ext_glitch_filter_ch.iter()
    }
    #[doc = "0x118..0x138 - Etm Config register of Channel%s"]
    #[inline(always)]
    pub const fn gpio_ext_etm_event_ch_cfg(&self, n: usize) -> &GPIO_EXT_ETM_EVENT_CH_CFG {
        &self.gpio_ext_etm_event_ch_cfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x118..0x138 - Etm Config register of Channel%s"]
    #[inline(always)]
    pub fn gpio_ext_etm_event_ch_cfg_iter(
        &self,
    ) -> impl Iterator<Item = &GPIO_EXT_ETM_EVENT_CH_CFG> {
        self.gpio_ext_etm_event_ch_cfg.iter()
    }
    #[doc = "0x118 - Etm Config register of Channel0"]
    #[inline(always)]
    pub const fn gpio_ext_etm_event_ch0_cfg(&self) -> &GPIO_EXT_ETM_EVENT_CH_CFG {
        self.gpio_ext_etm_event_ch_cfg(0)
    }
    #[doc = "0x11c - Etm Config register of Channel1"]
    #[inline(always)]
    pub const fn gpio_ext_etm_event_ch1_cfg(&self) -> &GPIO_EXT_ETM_EVENT_CH_CFG {
        self.gpio_ext_etm_event_ch_cfg(1)
    }
    #[doc = "0x120 - Etm Config register of Channel2"]
    #[inline(always)]
    pub const fn gpio_ext_etm_event_ch2_cfg(&self) -> &GPIO_EXT_ETM_EVENT_CH_CFG {
        self.gpio_ext_etm_event_ch_cfg(2)
    }
    #[doc = "0x124 - Etm Config register of Channel3"]
    #[inline(always)]
    pub const fn gpio_ext_etm_event_ch3_cfg(&self) -> &GPIO_EXT_ETM_EVENT_CH_CFG {
        self.gpio_ext_etm_event_ch_cfg(3)
    }
    #[doc = "0x128 - Etm Config register of Channel4"]
    #[inline(always)]
    pub const fn gpio_ext_etm_event_ch4_cfg(&self) -> &GPIO_EXT_ETM_EVENT_CH_CFG {
        self.gpio_ext_etm_event_ch_cfg(4)
    }
    #[doc = "0x12c - Etm Config register of Channel5"]
    #[inline(always)]
    pub const fn gpio_ext_etm_event_ch5_cfg(&self) -> &GPIO_EXT_ETM_EVENT_CH_CFG {
        self.gpio_ext_etm_event_ch_cfg(5)
    }
    #[doc = "0x130 - Etm Config register of Channel6"]
    #[inline(always)]
    pub const fn gpio_ext_etm_event_ch6_cfg(&self) -> &GPIO_EXT_ETM_EVENT_CH_CFG {
        self.gpio_ext_etm_event_ch_cfg(6)
    }
    #[doc = "0x134 - Etm Config register of Channel7"]
    #[inline(always)]
    pub const fn gpio_ext_etm_event_ch7_cfg(&self) -> &GPIO_EXT_ETM_EVENT_CH_CFG {
        self.gpio_ext_etm_event_ch_cfg(7)
    }
    #[doc = "0x158 - Etm Configure Register to decide which GPIO been chosen"]
    #[inline(always)]
    pub const fn gpio_ext_etm_task_p0_cfg(&self) -> &GPIO_EXT_ETM_TASK_P0_CFG {
        &self.gpio_ext_etm_task_p0_cfg
    }
    #[doc = "0x15c - Etm Configure Register to decide which GPIO been chosen"]
    #[inline(always)]
    pub const fn gpio_ext_etm_task_p1_cfg(&self) -> &GPIO_EXT_ETM_TASK_P1_CFG {
        &self.gpio_ext_etm_task_p1_cfg
    }
    #[doc = "0x160 - Etm Configure Register to decide which GPIO been chosen"]
    #[inline(always)]
    pub const fn gpio_ext_etm_task_p2_cfg(&self) -> &GPIO_EXT_ETM_TASK_P2_CFG {
        &self.gpio_ext_etm_task_p2_cfg
    }
    #[doc = "0x164 - Etm Configure Register to decide which GPIO been chosen"]
    #[inline(always)]
    pub const fn gpio_ext_etm_task_p3_cfg(&self) -> &GPIO_EXT_ETM_TASK_P3_CFG {
        &self.gpio_ext_etm_task_p3_cfg
    }
    #[doc = "0x168 - Etm Configure Register to decide which GPIO been chosen"]
    #[inline(always)]
    pub const fn gpio_ext_etm_task_p4_cfg(&self) -> &GPIO_EXT_ETM_TASK_P4_CFG {
        &self.gpio_ext_etm_task_p4_cfg
    }
    #[doc = "0x16c - Etm Configure Register to decide which GPIO been chosen"]
    #[inline(always)]
    pub const fn gpio_ext_etm_task_p5_cfg(&self) -> &GPIO_EXT_ETM_TASK_P5_CFG {
        &self.gpio_ext_etm_task_p5_cfg
    }
    #[doc = "0x1d0 - GPIO_EXT interrupt raw register"]
    #[inline(always)]
    pub const fn gpio_ext_int_raw(&self) -> &GPIO_EXT_INT_RAW {
        &self.gpio_ext_int_raw
    }
    #[doc = "0x1d4 - GPIO_EXT interrupt masked register"]
    #[inline(always)]
    pub const fn gpio_ext_int_st(&self) -> &GPIO_EXT_INT_ST {
        &self.gpio_ext_int_st
    }
    #[doc = "0x1d8 - GPIO_EXT interrupt enable register"]
    #[inline(always)]
    pub const fn gpio_ext_int_ena(&self) -> &GPIO_EXT_INT_ENA {
        &self.gpio_ext_int_ena
    }
    #[doc = "0x1dc - GPIO_EXT interrupt clear register"]
    #[inline(always)]
    pub const fn gpio_ext_int_clr(&self) -> &GPIO_EXT_INT_CLR {
        &self.gpio_ext_int_clr
    }
    #[doc = "0x1e0 - Clock Output Configuration Register"]
    #[inline(always)]
    pub const fn gpio_ext_pin_ctrl(&self) -> &GPIO_EXT_PIN_CTRL {
        &self.gpio_ext_pin_ctrl
    }
    #[doc = "0x1fc - Version Control Register"]
    #[inline(always)]
    pub const fn gpio_ext_version(&self) -> &GPIO_EXT_VERSION {
        &self.gpio_ext_version
    }
}
#[doc = "GPIO_EXT_CLOCK_GATE (rw) register accessor: Clock Gating Configure Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ext_clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ext_clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_ext_clock_gate`] module"]
pub type GPIO_EXT_CLOCK_GATE = crate::Reg<gpio_ext_clock_gate::GPIO_EXT_CLOCK_GATE_SPEC>;
#[doc = "Clock Gating Configure Register"]
pub mod gpio_ext_clock_gate;
#[doc = "GPIO_EXT_SIGMADELTA_MISC (rw) register accessor: MISC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ext_sigmadelta_misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ext_sigmadelta_misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_ext_sigmadelta_misc`] module"]
pub type GPIO_EXT_SIGMADELTA_MISC =
    crate::Reg<gpio_ext_sigmadelta_misc::GPIO_EXT_SIGMADELTA_MISC_SPEC>;
#[doc = "MISC Register"]
pub mod gpio_ext_sigmadelta_misc;
#[doc = "GPIO_EXT_SIGMADELTA (rw) register accessor: Duty Cycle Configure Register of SDM%s\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ext_sigmadelta::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ext_sigmadelta::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_ext_sigmadelta`] module"]
pub type GPIO_EXT_SIGMADELTA = crate::Reg<gpio_ext_sigmadelta::GPIO_EXT_SIGMADELTA_SPEC>;
#[doc = "Duty Cycle Configure Register of SDM%s"]
pub mod gpio_ext_sigmadelta;
#[doc = "GPIO_EXT_PAD_COMP_CONFIG_0 (rw) register accessor: PAD Compare configure Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ext_pad_comp_config_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ext_pad_comp_config_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_ext_pad_comp_config_0`] module"]
pub type GPIO_EXT_PAD_COMP_CONFIG_0 =
    crate::Reg<gpio_ext_pad_comp_config_0::GPIO_EXT_PAD_COMP_CONFIG_0_SPEC>;
#[doc = "PAD Compare configure Register"]
pub mod gpio_ext_pad_comp_config_0;
#[doc = "GPIO_EXT_PAD_COMP_FILTER_0 (rw) register accessor: Zero Detect filter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ext_pad_comp_filter_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ext_pad_comp_filter_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_ext_pad_comp_filter_0`] module"]
pub type GPIO_EXT_PAD_COMP_FILTER_0 =
    crate::Reg<gpio_ext_pad_comp_filter_0::GPIO_EXT_PAD_COMP_FILTER_0_SPEC>;
#[doc = "Zero Detect filter Register"]
pub mod gpio_ext_pad_comp_filter_0;
#[doc = "GPIO_EXT_GLITCH_FILTER_CH (rw) register accessor: Glitch Filter Configure Register of Channel%s\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ext_glitch_filter_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ext_glitch_filter_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_ext_glitch_filter_ch`] module"]
pub type GPIO_EXT_GLITCH_FILTER_CH =
    crate::Reg<gpio_ext_glitch_filter_ch::GPIO_EXT_GLITCH_FILTER_CH_SPEC>;
#[doc = "Glitch Filter Configure Register of Channel%s"]
pub mod gpio_ext_glitch_filter_ch;
#[doc = "GPIO_EXT_ETM_EVENT_CH_CFG (rw) register accessor: Etm Config register of Channel%s\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ext_etm_event_ch_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ext_etm_event_ch_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_ext_etm_event_ch_cfg`] module"]
pub type GPIO_EXT_ETM_EVENT_CH_CFG =
    crate::Reg<gpio_ext_etm_event_ch_cfg::GPIO_EXT_ETM_EVENT_CH_CFG_SPEC>;
#[doc = "Etm Config register of Channel%s"]
pub mod gpio_ext_etm_event_ch_cfg;
#[doc = "GPIO_EXT_ETM_TASK_P0_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ext_etm_task_p0_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ext_etm_task_p0_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_ext_etm_task_p0_cfg`] module"]
pub type GPIO_EXT_ETM_TASK_P0_CFG =
    crate::Reg<gpio_ext_etm_task_p0_cfg::GPIO_EXT_ETM_TASK_P0_CFG_SPEC>;
#[doc = "Etm Configure Register to decide which GPIO been chosen"]
pub mod gpio_ext_etm_task_p0_cfg;
#[doc = "GPIO_EXT_ETM_TASK_P1_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ext_etm_task_p1_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ext_etm_task_p1_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_ext_etm_task_p1_cfg`] module"]
pub type GPIO_EXT_ETM_TASK_P1_CFG =
    crate::Reg<gpio_ext_etm_task_p1_cfg::GPIO_EXT_ETM_TASK_P1_CFG_SPEC>;
#[doc = "Etm Configure Register to decide which GPIO been chosen"]
pub mod gpio_ext_etm_task_p1_cfg;
#[doc = "GPIO_EXT_ETM_TASK_P2_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ext_etm_task_p2_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ext_etm_task_p2_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_ext_etm_task_p2_cfg`] module"]
pub type GPIO_EXT_ETM_TASK_P2_CFG =
    crate::Reg<gpio_ext_etm_task_p2_cfg::GPIO_EXT_ETM_TASK_P2_CFG_SPEC>;
#[doc = "Etm Configure Register to decide which GPIO been chosen"]
pub mod gpio_ext_etm_task_p2_cfg;
#[doc = "GPIO_EXT_ETM_TASK_P3_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ext_etm_task_p3_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ext_etm_task_p3_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_ext_etm_task_p3_cfg`] module"]
pub type GPIO_EXT_ETM_TASK_P3_CFG =
    crate::Reg<gpio_ext_etm_task_p3_cfg::GPIO_EXT_ETM_TASK_P3_CFG_SPEC>;
#[doc = "Etm Configure Register to decide which GPIO been chosen"]
pub mod gpio_ext_etm_task_p3_cfg;
#[doc = "GPIO_EXT_ETM_TASK_P4_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ext_etm_task_p4_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ext_etm_task_p4_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_ext_etm_task_p4_cfg`] module"]
pub type GPIO_EXT_ETM_TASK_P4_CFG =
    crate::Reg<gpio_ext_etm_task_p4_cfg::GPIO_EXT_ETM_TASK_P4_CFG_SPEC>;
#[doc = "Etm Configure Register to decide which GPIO been chosen"]
pub mod gpio_ext_etm_task_p4_cfg;
#[doc = "GPIO_EXT_ETM_TASK_P5_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ext_etm_task_p5_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ext_etm_task_p5_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_ext_etm_task_p5_cfg`] module"]
pub type GPIO_EXT_ETM_TASK_P5_CFG =
    crate::Reg<gpio_ext_etm_task_p5_cfg::GPIO_EXT_ETM_TASK_P5_CFG_SPEC>;
#[doc = "Etm Configure Register to decide which GPIO been chosen"]
pub mod gpio_ext_etm_task_p5_cfg;
#[doc = "GPIO_EXT_INT_RAW (r) register accessor: GPIO_EXT interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ext_int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_ext_int_raw`] module"]
pub type GPIO_EXT_INT_RAW = crate::Reg<gpio_ext_int_raw::GPIO_EXT_INT_RAW_SPEC>;
#[doc = "GPIO_EXT interrupt raw register"]
pub mod gpio_ext_int_raw;
#[doc = "GPIO_EXT_INT_ST (r) register accessor: GPIO_EXT interrupt masked register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ext_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_ext_int_st`] module"]
pub type GPIO_EXT_INT_ST = crate::Reg<gpio_ext_int_st::GPIO_EXT_INT_ST_SPEC>;
#[doc = "GPIO_EXT interrupt masked register"]
pub mod gpio_ext_int_st;
#[doc = "GPIO_EXT_INT_ENA (rw) register accessor: GPIO_EXT interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ext_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ext_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_ext_int_ena`] module"]
pub type GPIO_EXT_INT_ENA = crate::Reg<gpio_ext_int_ena::GPIO_EXT_INT_ENA_SPEC>;
#[doc = "GPIO_EXT interrupt enable register"]
pub mod gpio_ext_int_ena;
#[doc = "GPIO_EXT_INT_CLR (w) register accessor: GPIO_EXT interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ext_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_ext_int_clr`] module"]
pub type GPIO_EXT_INT_CLR = crate::Reg<gpio_ext_int_clr::GPIO_EXT_INT_CLR_SPEC>;
#[doc = "GPIO_EXT interrupt clear register"]
pub mod gpio_ext_int_clr;
#[doc = "GPIO_EXT_PIN_CTRL (rw) register accessor: Clock Output Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ext_pin_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ext_pin_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_ext_pin_ctrl`] module"]
pub type GPIO_EXT_PIN_CTRL = crate::Reg<gpio_ext_pin_ctrl::GPIO_EXT_PIN_CTRL_SPEC>;
#[doc = "Clock Output Configuration Register"]
pub mod gpio_ext_pin_ctrl;
#[doc = "GPIO_EXT_VERSION (rw) register accessor: Version Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ext_version::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ext_version::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_ext_version`] module"]
pub type GPIO_EXT_VERSION = crate::Reg<gpio_ext_version::GPIO_EXT_VERSION_SPEC>;
#[doc = "Version Control Register"]
pub mod gpio_ext_version;
