#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    mem_start_addr: MEM_START_ADDR,
    mem_end_addr: MEM_END_ADDR,
    mem_current_addr: MEM_CURRENT_ADDR,
    mem_addr_update: MEM_ADDR_UPDATE,
    fifo_status: FIFO_STATUS,
    intr_ena: INTR_ENA,
    intr_raw: INTR_RAW,
    intr_clr: INTR_CLR,
    trigger: TRIGGER,
    config: CONFIG,
    filter_control: FILTER_CONTROL,
    filter_match_control: FILTER_MATCH_CONTROL,
    filter_comparator_control: FILTER_COMPARATOR_CONTROL,
    filter_p_comparator_match: FILTER_P_COMPARATOR_MATCH,
    filter_s_comparator_match: FILTER_S_COMPARATOR_MATCH,
    resync_prolonged: RESYNC_PROLONGED,
    ahb_config: AHB_CONFIG,
    clock_gate: CLOCK_GATE,
    _reserved18: [u8; 0x03b4],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - mem start addr"]
    #[inline(always)]
    pub const fn mem_start_addr(&self) -> &MEM_START_ADDR {
        &self.mem_start_addr
    }
    #[doc = "0x04 - mem end addr"]
    #[inline(always)]
    pub const fn mem_end_addr(&self) -> &MEM_END_ADDR {
        &self.mem_end_addr
    }
    #[doc = "0x08 - mem current addr"]
    #[inline(always)]
    pub const fn mem_current_addr(&self) -> &MEM_CURRENT_ADDR {
        &self.mem_current_addr
    }
    #[doc = "0x0c - mem addr update"]
    #[inline(always)]
    pub const fn mem_addr_update(&self) -> &MEM_ADDR_UPDATE {
        &self.mem_addr_update
    }
    #[doc = "0x10 - fifo status register"]
    #[inline(always)]
    pub const fn fifo_status(&self) -> &FIFO_STATUS {
        &self.fifo_status
    }
    #[doc = "0x14 - interrupt enable register"]
    #[inline(always)]
    pub const fn intr_ena(&self) -> &INTR_ENA {
        &self.intr_ena
    }
    #[doc = "0x18 - interrupt status register"]
    #[inline(always)]
    pub const fn intr_raw(&self) -> &INTR_RAW {
        &self.intr_raw
    }
    #[doc = "0x1c - interrupt clear register"]
    #[inline(always)]
    pub const fn intr_clr(&self) -> &INTR_CLR {
        &self.intr_clr
    }
    #[doc = "0x20 - trigger register"]
    #[inline(always)]
    pub const fn trigger(&self) -> &TRIGGER {
        &self.trigger
    }
    #[doc = "0x24 - trace configuration register"]
    #[inline(always)]
    pub const fn config(&self) -> &CONFIG {
        &self.config
    }
    #[doc = "0x28 - filter control register"]
    #[inline(always)]
    pub const fn filter_control(&self) -> &FILTER_CONTROL {
        &self.filter_control
    }
    #[doc = "0x2c - filter match control register"]
    #[inline(always)]
    pub const fn filter_match_control(&self) -> &FILTER_MATCH_CONTROL {
        &self.filter_match_control
    }
    #[doc = "0x30 - filter comparator match control register"]
    #[inline(always)]
    pub const fn filter_comparator_control(&self) -> &FILTER_COMPARATOR_CONTROL {
        &self.filter_comparator_control
    }
    #[doc = "0x34 - primary comparator match value"]
    #[inline(always)]
    pub const fn filter_p_comparator_match(&self) -> &FILTER_P_COMPARATOR_MATCH {
        &self.filter_p_comparator_match
    }
    #[doc = "0x38 - secondary comparator match value"]
    #[inline(always)]
    pub const fn filter_s_comparator_match(&self) -> &FILTER_S_COMPARATOR_MATCH {
        &self.filter_s_comparator_match
    }
    #[doc = "0x3c - resync configuration register"]
    #[inline(always)]
    pub const fn resync_prolonged(&self) -> &RESYNC_PROLONGED {
        &self.resync_prolonged
    }
    #[doc = "0x40 - AHB config register"]
    #[inline(always)]
    pub const fn ahb_config(&self) -> &AHB_CONFIG {
        &self.ahb_config
    }
    #[doc = "0x44 - Clock gate control register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x3fc - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "MEM_START_ADDR (rw) register accessor: mem start addr\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_start_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_start_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_start_addr`] module"]
pub type MEM_START_ADDR = crate::Reg<mem_start_addr::MEM_START_ADDR_SPEC>;
#[doc = "mem start addr"]
pub mod mem_start_addr;
#[doc = "MEM_END_ADDR (rw) register accessor: mem end addr\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_end_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_end_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_end_addr`] module"]
pub type MEM_END_ADDR = crate::Reg<mem_end_addr::MEM_END_ADDR_SPEC>;
#[doc = "mem end addr"]
pub mod mem_end_addr;
#[doc = "MEM_CURRENT_ADDR (r) register accessor: mem current addr\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_current_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_current_addr`] module"]
pub type MEM_CURRENT_ADDR = crate::Reg<mem_current_addr::MEM_CURRENT_ADDR_SPEC>;
#[doc = "mem current addr"]
pub mod mem_current_addr;
#[doc = "MEM_ADDR_UPDATE (w) register accessor: mem addr update\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_addr_update::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_addr_update`] module"]
pub type MEM_ADDR_UPDATE = crate::Reg<mem_addr_update::MEM_ADDR_UPDATE_SPEC>;
#[doc = "mem addr update"]
pub mod mem_addr_update;
#[doc = "FIFO_STATUS (r) register accessor: fifo status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_status`] module"]
pub type FIFO_STATUS = crate::Reg<fifo_status::FIFO_STATUS_SPEC>;
#[doc = "fifo status register"]
pub mod fifo_status;
#[doc = "INTR_ENA (rw) register accessor: interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_ena`] module"]
pub type INTR_ENA = crate::Reg<intr_ena::INTR_ENA_SPEC>;
#[doc = "interrupt enable register"]
pub mod intr_ena;
#[doc = "INTR_RAW (r) register accessor: interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_raw`] module"]
pub type INTR_RAW = crate::Reg<intr_raw::INTR_RAW_SPEC>;
#[doc = "interrupt status register"]
pub mod intr_raw;
#[doc = "INTR_CLR (w) register accessor: interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_clr`] module"]
pub type INTR_CLR = crate::Reg<intr_clr::INTR_CLR_SPEC>;
#[doc = "interrupt clear register"]
pub mod intr_clr;
#[doc = "TRIGGER (rw) register accessor: trigger register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trigger::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trigger::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trigger`] module"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "trigger register"]
pub mod trigger;
#[doc = "CONFIG (rw) register accessor: trace configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "trace configuration register"]
pub mod config;
#[doc = "FILTER_CONTROL (rw) register accessor: filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filter_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filter_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_control`] module"]
pub type FILTER_CONTROL = crate::Reg<filter_control::FILTER_CONTROL_SPEC>;
#[doc = "filter control register"]
pub mod filter_control;
#[doc = "FILTER_MATCH_CONTROL (rw) register accessor: filter match control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filter_match_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filter_match_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_match_control`] module"]
pub type FILTER_MATCH_CONTROL = crate::Reg<filter_match_control::FILTER_MATCH_CONTROL_SPEC>;
#[doc = "filter match control register"]
pub mod filter_match_control;
#[doc = "FILTER_COMPARATOR_CONTROL (rw) register accessor: filter comparator match control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filter_comparator_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filter_comparator_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_comparator_control`] module"]
pub type FILTER_COMPARATOR_CONTROL =
    crate::Reg<filter_comparator_control::FILTER_COMPARATOR_CONTROL_SPEC>;
#[doc = "filter comparator match control register"]
pub mod filter_comparator_control;
#[doc = "FILTER_P_COMPARATOR_MATCH (rw) register accessor: primary comparator match value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filter_p_comparator_match::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filter_p_comparator_match::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_p_comparator_match`] module"]
pub type FILTER_P_COMPARATOR_MATCH =
    crate::Reg<filter_p_comparator_match::FILTER_P_COMPARATOR_MATCH_SPEC>;
#[doc = "primary comparator match value"]
pub mod filter_p_comparator_match;
#[doc = "FILTER_S_COMPARATOR_MATCH (rw) register accessor: secondary comparator match value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filter_s_comparator_match::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filter_s_comparator_match::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_s_comparator_match`] module"]
pub type FILTER_S_COMPARATOR_MATCH =
    crate::Reg<filter_s_comparator_match::FILTER_S_COMPARATOR_MATCH_SPEC>;
#[doc = "secondary comparator match value"]
pub mod filter_s_comparator_match;
#[doc = "RESYNC_PROLONGED (rw) register accessor: resync configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resync_prolonged::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resync_prolonged::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resync_prolonged`] module"]
pub type RESYNC_PROLONGED = crate::Reg<resync_prolonged::RESYNC_PROLONGED_SPEC>;
#[doc = "resync configuration register"]
pub mod resync_prolonged;
#[doc = "AHB_CONFIG (rw) register accessor: AHB config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_config`] module"]
pub type AHB_CONFIG = crate::Reg<ahb_config::AHB_CONFIG_SPEC>;
#[doc = "AHB config register"]
pub mod ahb_config;
#[doc = "CLOCK_GATE (rw) register accessor: Clock gate control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Clock gate control register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
