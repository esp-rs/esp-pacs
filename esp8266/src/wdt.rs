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
#[doc = "WDT_CTL (rw) register accessor: WDT_CTL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdt_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdt_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdt_ctl`] module"]
pub type WDT_CTL = crate::Reg<wdt_ctl::WDT_CTL_SPEC>;
#[doc = "WDT_CTL"]
pub mod wdt_ctl;
#[doc = "WDT_OP (rw) register accessor: Reload value for stage 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdt_op::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdt_op::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdt_op`] module"]
pub type WDT_OP = crate::Reg<wdt_op::WDT_OP_SPEC>;
#[doc = "Reload value for stage 0"]
pub mod wdt_op;
#[doc = "WDT_OP_ND (rw) register accessor: Reload value for stage 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdt_op_nd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdt_op_nd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdt_op_nd`] module"]
pub type WDT_OP_ND = crate::Reg<wdt_op_nd::WDT_OP_ND_SPEC>;
#[doc = "Reload value for stage 1"]
pub mod wdt_op_nd;
#[doc = "WDT_RST (rw) register accessor: Watchdog reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdt_rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdt_rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdt_rst`] module"]
pub type WDT_RST = crate::Reg<wdt_rst::WDT_RST_SPEC>;
#[doc = "Watchdog reset"]
pub mod wdt_rst;
#[doc = "count (rw) register accessor: Watchdog clock cycle count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`count`] module"]
pub type COUNT = crate::Reg<count::COUNT_SPEC>;
#[doc = "Watchdog clock cycle count"]
pub mod count;
#[doc = "stage (rw) register accessor: The current watchdog stage\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stage::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stage::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stage`] module"]
pub type STAGE = crate::Reg<stage::STAGE_SPEC>;
#[doc = "The current watchdog stage"]
pub mod stage;
#[doc = "reset_stage (rw) register accessor: Watchdog stage reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_stage::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_stage::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`reset_stage`] module"]
pub type RESET_STAGE = crate::Reg<reset_stage::RESET_STAGE_SPEC>;
#[doc = "Watchdog stage reset"]
pub mod reset_stage;
