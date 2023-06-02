#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - need_des"]
    pub clk_en: CLK_EN,
    #[doc = "0x04 - need_des"]
    pub reset_en: RESET_EN,
    #[doc = "0x08 - need_des"]
    pub rng_data: RNG_DATA,
    #[doc = "0x0c - need_des"]
    pub cpu: CPU,
    #[doc = "0x10 - need_des"]
    pub bus_timeout: BUS_TIMEOUT,
    #[doc = "0x14 - need_des"]
    pub bus_timeout_addr: BUS_TIMEOUT_ADDR,
    #[doc = "0x18 - need_des"]
    pub bus_timeout_uid: BUS_TIMEOUT_UID,
    #[doc = "0x1c - need_des"]
    pub mem_ctrl: MEM_CTRL,
    #[doc = "0x20 - need_des"]
    pub interrupt_source: INTERRUPT_SOURCE,
    #[doc = "0x24 - need des"]
    pub debug_sel0: DEBUG_SEL0,
    #[doc = "0x28 - need des"]
    pub debug_sel1: DEBUG_SEL1,
    _reserved11: [u8; 0x03d0],
    #[doc = "0x3fc - need_des"]
    pub date: DATE,
}
#[doc = "CLK_EN (rw) register accessor: an alias for `Reg<CLK_EN_SPEC>`"]
pub type CLK_EN = crate::Reg<clk_en::CLK_EN_SPEC>;
#[doc = "need_des"]
pub mod clk_en;
#[doc = "RESET_EN (rw) register accessor: an alias for `Reg<RESET_EN_SPEC>`"]
pub type RESET_EN = crate::Reg<reset_en::RESET_EN_SPEC>;
#[doc = "need_des"]
pub mod reset_en;
#[doc = "RNG_DATA (r) register accessor: an alias for `Reg<RNG_DATA_SPEC>`"]
pub type RNG_DATA = crate::Reg<rng_data::RNG_DATA_SPEC>;
#[doc = "need_des"]
pub mod rng_data;
#[doc = "CPU (rw) register accessor: an alias for `Reg<CPU_SPEC>`"]
pub type CPU = crate::Reg<cpu::CPU_SPEC>;
#[doc = "need_des"]
pub mod cpu;
#[doc = "BUS_TIMEOUT (rw) register accessor: an alias for `Reg<BUS_TIMEOUT_SPEC>`"]
pub type BUS_TIMEOUT = crate::Reg<bus_timeout::BUS_TIMEOUT_SPEC>;
#[doc = "need_des"]
pub mod bus_timeout;
#[doc = "BUS_TIMEOUT_ADDR (r) register accessor: an alias for `Reg<BUS_TIMEOUT_ADDR_SPEC>`"]
pub type BUS_TIMEOUT_ADDR = crate::Reg<bus_timeout_addr::BUS_TIMEOUT_ADDR_SPEC>;
#[doc = "need_des"]
pub mod bus_timeout_addr;
#[doc = "BUS_TIMEOUT_UID (r) register accessor: an alias for `Reg<BUS_TIMEOUT_UID_SPEC>`"]
pub type BUS_TIMEOUT_UID = crate::Reg<bus_timeout_uid::BUS_TIMEOUT_UID_SPEC>;
#[doc = "need_des"]
pub mod bus_timeout_uid;
#[doc = "MEM_CTRL (rw) register accessor: an alias for `Reg<MEM_CTRL_SPEC>`"]
pub type MEM_CTRL = crate::Reg<mem_ctrl::MEM_CTRL_SPEC>;
#[doc = "need_des"]
pub mod mem_ctrl;
#[doc = "INTERRUPT_SOURCE (r) register accessor: an alias for `Reg<INTERRUPT_SOURCE_SPEC>`"]
pub type INTERRUPT_SOURCE = crate::Reg<interrupt_source::INTERRUPT_SOURCE_SPEC>;
#[doc = "need_des"]
pub mod interrupt_source;
#[doc = "DEBUG_SEL0 (rw) register accessor: an alias for `Reg<DEBUG_SEL0_SPEC>`"]
pub type DEBUG_SEL0 = crate::Reg<debug_sel0::DEBUG_SEL0_SPEC>;
#[doc = "need des"]
pub mod debug_sel0;
#[doc = "DEBUG_SEL1 (rw) register accessor: an alias for `Reg<DEBUG_SEL1_SPEC>`"]
pub type DEBUG_SEL1 = crate::Reg<debug_sel1::DEBUG_SEL1_SPEC>;
#[doc = "need des"]
pub mod debug_sel1;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need_des"]
pub mod date;
