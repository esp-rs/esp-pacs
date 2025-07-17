#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    clock_gate: CLOCK_GATE,
    _reserved1: [u8; 0x54],
    pad_comp_config_0: PAD_COMP_CONFIG_0,
    pad_comp_filter_0: PAD_COMP_FILTER_0,
    _reserved3: [u8; 0xb8],
    etm_event_ch_cfg: [ETM_EVENT_CH_CFG; 8],
    _reserved4: [u8; 0x20],
    etm_task_p0_cfg: ETM_TASK_P0_CFG,
    etm_task_p1_cfg: ETM_TASK_P1_CFG,
    etm_task_p2_cfg: ETM_TASK_P2_CFG,
    etm_task_p3_cfg: ETM_TASK_P3_CFG,
    etm_task_p4_cfg: ETM_TASK_P4_CFG,
    etm_task_p5_cfg: ETM_TASK_P5_CFG,
    _reserved10: [u8; 0x60],
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    pin_ctrl: PIN_CTRL,
    _reserved15: [u8; 0x18],
    version: VERSION,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock Gating Configure Register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x58 - PAD Compare configure Register"]
    #[inline(always)]
    pub const fn pad_comp_config_0(&self) -> &PAD_COMP_CONFIG_0 {
        &self.pad_comp_config_0
    }
    #[doc = "0x5c - Zero Detect filter Register"]
    #[inline(always)]
    pub const fn pad_comp_filter_0(&self) -> &PAD_COMP_FILTER_0 {
        &self.pad_comp_filter_0
    }
    #[doc = "0x118..0x138 - Etm Config register of Channel%s"]
    #[inline(always)]
    pub const fn etm_event_ch_cfg(&self, n: usize) -> &ETM_EVENT_CH_CFG {
        &self.etm_event_ch_cfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x118..0x138 - Etm Config register of Channel%s"]
    #[inline(always)]
    pub fn etm_event_ch_cfg_iter(&self) -> impl Iterator<Item = &ETM_EVENT_CH_CFG> {
        self.etm_event_ch_cfg.iter()
    }
    #[doc = "0x118 - Etm Config register of Channel0"]
    #[inline(always)]
    pub const fn etm_event_ch0_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(0)
    }
    #[doc = "0x11c - Etm Config register of Channel1"]
    #[inline(always)]
    pub const fn etm_event_ch1_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(1)
    }
    #[doc = "0x120 - Etm Config register of Channel2"]
    #[inline(always)]
    pub const fn etm_event_ch2_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(2)
    }
    #[doc = "0x124 - Etm Config register of Channel3"]
    #[inline(always)]
    pub const fn etm_event_ch3_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(3)
    }
    #[doc = "0x128 - Etm Config register of Channel4"]
    #[inline(always)]
    pub const fn etm_event_ch4_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(4)
    }
    #[doc = "0x12c - Etm Config register of Channel5"]
    #[inline(always)]
    pub const fn etm_event_ch5_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(5)
    }
    #[doc = "0x130 - Etm Config register of Channel6"]
    #[inline(always)]
    pub const fn etm_event_ch6_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(6)
    }
    #[doc = "0x134 - Etm Config register of Channel7"]
    #[inline(always)]
    pub const fn etm_event_ch7_cfg(&self) -> &ETM_EVENT_CH_CFG {
        self.etm_event_ch_cfg(7)
    }
    #[doc = "0x158 - Etm Configure Register to decide which GPIO been chosen"]
    #[inline(always)]
    pub const fn etm_task_p0_cfg(&self) -> &ETM_TASK_P0_CFG {
        &self.etm_task_p0_cfg
    }
    #[doc = "0x15c - Etm Configure Register to decide which GPIO been chosen"]
    #[inline(always)]
    pub const fn etm_task_p1_cfg(&self) -> &ETM_TASK_P1_CFG {
        &self.etm_task_p1_cfg
    }
    #[doc = "0x160 - Etm Configure Register to decide which GPIO been chosen"]
    #[inline(always)]
    pub const fn etm_task_p2_cfg(&self) -> &ETM_TASK_P2_CFG {
        &self.etm_task_p2_cfg
    }
    #[doc = "0x164 - Etm Configure Register to decide which GPIO been chosen"]
    #[inline(always)]
    pub const fn etm_task_p3_cfg(&self) -> &ETM_TASK_P3_CFG {
        &self.etm_task_p3_cfg
    }
    #[doc = "0x168 - Etm Configure Register to decide which GPIO been chosen"]
    #[inline(always)]
    pub const fn etm_task_p4_cfg(&self) -> &ETM_TASK_P4_CFG {
        &self.etm_task_p4_cfg
    }
    #[doc = "0x16c - Etm Configure Register to decide which GPIO been chosen"]
    #[inline(always)]
    pub const fn etm_task_p5_cfg(&self) -> &ETM_TASK_P5_CFG {
        &self.etm_task_p5_cfg
    }
    #[doc = "0x1d0 - GPIO_EXT interrupt raw register"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x1d4 - GPIO_EXT interrupt masked register"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x1d8 - GPIO_EXT interrupt enable register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x1dc - GPIO_EXT interrupt clear register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x1e0 - Clock Output Configuration Register"]
    #[inline(always)]
    pub const fn pin_ctrl(&self) -> &PIN_CTRL {
        &self.pin_ctrl
    }
    #[doc = "0x1fc - Version Control Register"]
    #[inline(always)]
    pub const fn version(&self) -> &VERSION {
        &self.version
    }
}
#[doc = "CLOCK_GATE (rw) register accessor: Clock Gating Configure Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Clock Gating Configure Register"]
pub mod clock_gate;
#[doc = "PAD_COMP_CONFIG_0 (rw) register accessor: PAD Compare configure Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_comp_config_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_comp_config_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_comp_config_0`] module"]
pub type PAD_COMP_CONFIG_0 = crate::Reg<pad_comp_config_0::PAD_COMP_CONFIG_0_SPEC>;
#[doc = "PAD Compare configure Register"]
pub mod pad_comp_config_0;
#[doc = "PAD_COMP_FILTER_0 (rw) register accessor: Zero Detect filter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_comp_filter_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_comp_filter_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_comp_filter_0`] module"]
pub type PAD_COMP_FILTER_0 = crate::Reg<pad_comp_filter_0::PAD_COMP_FILTER_0_SPEC>;
#[doc = "Zero Detect filter Register"]
pub mod pad_comp_filter_0;
#[doc = "ETM_EVENT_CH_CFG (rw) register accessor: Etm Config register of Channel%s\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_event_ch_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_event_ch_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_event_ch_cfg`] module"]
pub type ETM_EVENT_CH_CFG = crate::Reg<etm_event_ch_cfg::ETM_EVENT_CH_CFG_SPEC>;
#[doc = "Etm Config register of Channel%s"]
pub mod etm_event_ch_cfg;
#[doc = "ETM_TASK_P0_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p0_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p0_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_task_p0_cfg`] module"]
pub type ETM_TASK_P0_CFG = crate::Reg<etm_task_p0_cfg::ETM_TASK_P0_CFG_SPEC>;
#[doc = "Etm Configure Register to decide which GPIO been chosen"]
pub mod etm_task_p0_cfg;
#[doc = "ETM_TASK_P1_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p1_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p1_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_task_p1_cfg`] module"]
pub type ETM_TASK_P1_CFG = crate::Reg<etm_task_p1_cfg::ETM_TASK_P1_CFG_SPEC>;
#[doc = "Etm Configure Register to decide which GPIO been chosen"]
pub mod etm_task_p1_cfg;
#[doc = "ETM_TASK_P2_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p2_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p2_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_task_p2_cfg`] module"]
pub type ETM_TASK_P2_CFG = crate::Reg<etm_task_p2_cfg::ETM_TASK_P2_CFG_SPEC>;
#[doc = "Etm Configure Register to decide which GPIO been chosen"]
pub mod etm_task_p2_cfg;
#[doc = "ETM_TASK_P3_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p3_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p3_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_task_p3_cfg`] module"]
pub type ETM_TASK_P3_CFG = crate::Reg<etm_task_p3_cfg::ETM_TASK_P3_CFG_SPEC>;
#[doc = "Etm Configure Register to decide which GPIO been chosen"]
pub mod etm_task_p3_cfg;
#[doc = "ETM_TASK_P4_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p4_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p4_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_task_p4_cfg`] module"]
pub type ETM_TASK_P4_CFG = crate::Reg<etm_task_p4_cfg::ETM_TASK_P4_CFG_SPEC>;
#[doc = "Etm Configure Register to decide which GPIO been chosen"]
pub mod etm_task_p4_cfg;
#[doc = "ETM_TASK_P5_CFG (rw) register accessor: Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p5_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p5_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_task_p5_cfg`] module"]
pub type ETM_TASK_P5_CFG = crate::Reg<etm_task_p5_cfg::ETM_TASK_P5_CFG_SPEC>;
#[doc = "Etm Configure Register to decide which GPIO been chosen"]
pub mod etm_task_p5_cfg;
#[doc = "INT_RAW (r) register accessor: GPIO_EXT interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "GPIO_EXT interrupt raw register"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: GPIO_EXT interrupt masked register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "GPIO_EXT interrupt masked register"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: GPIO_EXT interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "GPIO_EXT interrupt enable register"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: GPIO_EXT interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "GPIO_EXT interrupt clear register"]
pub mod int_clr;
#[doc = "PIN_CTRL (rw) register accessor: Clock Output Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pin_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin_ctrl`] module"]
pub type PIN_CTRL = crate::Reg<pin_ctrl::PIN_CTRL_SPEC>;
#[doc = "Clock Output Configuration Register"]
pub mod pin_ctrl;
pub use crate::dma::date as version;
pub use crate::dma::DATE as VERSION;
