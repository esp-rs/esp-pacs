#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - WDT_CTL"]
    pub wdt_ctl: WDT_CTL,
    #[doc = "0x04 - Reload value for stage 0"]
    pub wdt_op: WDT_OP,
    #[doc = "0x08 - Reload value for stage 1"]
    pub wdt_op_nd: WDT_OP_ND,
    #[doc = "0x0c - Watchdog clock cycle count"]
    pub count: COUNT,
    #[doc = "0x10 - The current watchdog stage"]
    pub stage: STAGE,
    #[doc = "0x14 - Watchdog reset"]
    pub wdt_rst: WDT_RST,
    #[doc = "0x18 - Watchdog stage reset"]
    pub reset_stage: RESET_STAGE,
}
#[doc = "WDT_CTL (rw) register accessor: an alias for `Reg<WDT_CTL_SPEC>`"]
pub type WDT_CTL = crate::Reg<wdt_ctl::WDT_CTL_SPEC>;
#[doc = "WDT_CTL"]
pub mod wdt_ctl;
#[doc = "WDT_OP (rw) register accessor: an alias for `Reg<WDT_OP_SPEC>`"]
pub type WDT_OP = crate::Reg<wdt_op::WDT_OP_SPEC>;
#[doc = "Reload value for stage 0"]
pub mod wdt_op;
#[doc = "WDT_OP_ND (rw) register accessor: an alias for `Reg<WDT_OP_ND_SPEC>`"]
pub type WDT_OP_ND = crate::Reg<wdt_op_nd::WDT_OP_ND_SPEC>;
#[doc = "Reload value for stage 1"]
pub mod wdt_op_nd;
#[doc = "WDT_RST (rw) register accessor: an alias for `Reg<WDT_RST_SPEC>`"]
pub type WDT_RST = crate::Reg<wdt_rst::WDT_RST_SPEC>;
#[doc = "Watchdog reset"]
pub mod wdt_rst;
#[doc = "count (rw) register accessor: an alias for `Reg<COUNT_SPEC>`"]
pub type COUNT = crate::Reg<count::COUNT_SPEC>;
#[doc = "Watchdog clock cycle count"]
pub mod count;
#[doc = "stage (rw) register accessor: an alias for `Reg<STAGE_SPEC>`"]
pub type STAGE = crate::Reg<stage::STAGE_SPEC>;
#[doc = "The current watchdog stage"]
pub mod stage;
#[doc = "reset_stage (rw) register accessor: an alias for `Reg<RESET_STAGE_SPEC>`"]
pub type RESET_STAGE = crate::Reg<reset_stage::RESET_STAGE_SPEC>;
#[doc = "Watchdog stage reset"]
pub mod reset_stage;
