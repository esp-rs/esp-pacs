#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - mem start addr"]
    pub mem_start_addr: MEM_START_ADDR,
    #[doc = "0x04 - mem end addr"]
    pub mem_end_addr: MEM_END_ADDR,
    #[doc = "0x08 - mem current addr"]
    pub mem_current_addr: MEM_CURRENT_ADDR,
    #[doc = "0x0c - mem addr update"]
    pub mem_addr_update: MEM_ADDR_UPDATE,
    #[doc = "0x10 - fifo status register"]
    pub fifo_status: FIFO_STATUS,
    #[doc = "0x14 - interrupt enable register"]
    pub intr_ena: INTR_ENA,
    #[doc = "0x18 - interrupt status register"]
    pub intr_raw: INTR_RAW,
    #[doc = "0x1c - interrupt clear register"]
    pub intr_clr: INTR_CLR,
    #[doc = "0x20 - trigger register"]
    pub trigger: TRIGGER,
    #[doc = "0x24 - resync configuration register"]
    pub resync_prolonged: RESYNC_PROLONGED,
    #[doc = "0x28 - Clock gate control register"]
    pub clock_gate: CLOCK_GATE,
    _reserved11: [u8; 0x03d0],
    #[doc = "0x3fc - Version control register"]
    pub date: DATE,
}
#[doc = "MEM_START_ADDR (rw) register accessor: an alias for `Reg<MEM_START_ADDR_SPEC>`"]
pub type MEM_START_ADDR = crate::Reg<mem_start_addr::MEM_START_ADDR_SPEC>;
#[doc = "mem start addr"]
pub mod mem_start_addr;
#[doc = "MEM_END_ADDR (rw) register accessor: an alias for `Reg<MEM_END_ADDR_SPEC>`"]
pub type MEM_END_ADDR = crate::Reg<mem_end_addr::MEM_END_ADDR_SPEC>;
#[doc = "mem end addr"]
pub mod mem_end_addr;
#[doc = "MEM_CURRENT_ADDR (r) register accessor: an alias for `Reg<MEM_CURRENT_ADDR_SPEC>`"]
pub type MEM_CURRENT_ADDR = crate::Reg<mem_current_addr::MEM_CURRENT_ADDR_SPEC>;
#[doc = "mem current addr"]
pub mod mem_current_addr;
#[doc = "MEM_ADDR_UPDATE (w) register accessor: an alias for `Reg<MEM_ADDR_UPDATE_SPEC>`"]
pub type MEM_ADDR_UPDATE = crate::Reg<mem_addr_update::MEM_ADDR_UPDATE_SPEC>;
#[doc = "mem addr update"]
pub mod mem_addr_update;
#[doc = "FIFO_STATUS (r) register accessor: an alias for `Reg<FIFO_STATUS_SPEC>`"]
pub type FIFO_STATUS = crate::Reg<fifo_status::FIFO_STATUS_SPEC>;
#[doc = "fifo status register"]
pub mod fifo_status;
#[doc = "INTR_ENA (rw) register accessor: an alias for `Reg<INTR_ENA_SPEC>`"]
pub type INTR_ENA = crate::Reg<intr_ena::INTR_ENA_SPEC>;
#[doc = "interrupt enable register"]
pub mod intr_ena;
#[doc = "INTR_RAW (r) register accessor: an alias for `Reg<INTR_RAW_SPEC>`"]
pub type INTR_RAW = crate::Reg<intr_raw::INTR_RAW_SPEC>;
#[doc = "interrupt status register"]
pub mod intr_raw;
#[doc = "INTR_CLR (w) register accessor: an alias for `Reg<INTR_CLR_SPEC>`"]
pub type INTR_CLR = crate::Reg<intr_clr::INTR_CLR_SPEC>;
#[doc = "interrupt clear register"]
pub mod intr_clr;
#[doc = "TRIGGER (rw) register accessor: an alias for `Reg<TRIGGER_SPEC>`"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "trigger register"]
pub mod trigger;
#[doc = "RESYNC_PROLONGED (rw) register accessor: an alias for `Reg<RESYNC_PROLONGED_SPEC>`"]
pub type RESYNC_PROLONGED = crate::Reg<resync_prolonged::RESYNC_PROLONGED_SPEC>;
#[doc = "resync configuration register"]
pub mod resync_prolonged;
#[doc = "CLOCK_GATE (rw) register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Clock gate control register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
