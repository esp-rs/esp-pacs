#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    _reserved0: [u8; 0xf8],
    #[doc = "0xf8 - Configure coprocessor timer"]
    pub ulp_cp_timer: ULP_CP_TIMER,
    #[doc = "0xfc - ULP-FSM configuration register"]
    pub ulp_cp_ctrl: ULP_CP_CTRL,
    #[doc = "0x100 - ULP-RISCV configuration register"]
    pub cocpu_ctrl: COCPU_CTRL,
    _reserved3: [u8; 0x2c],
    #[doc = "0x130 - Configure sleep cycle of the timer"]
    pub ulp_cp_timer_1: ULP_CP_TIMER_1,
}
#[doc = "ULP_CP_TIMER (rw) register accessor: an alias for `Reg<ULP_CP_TIMER_SPEC>`"]
pub type ULP_CP_TIMER = crate::Reg<ulp_cp_timer::ULP_CP_TIMER_SPEC>;
#[doc = "Configure coprocessor timer"]
pub mod ulp_cp_timer;
#[doc = "ULP_CP_CTRL (rw) register accessor: an alias for `Reg<ULP_CP_CTRL_SPEC>`"]
pub type ULP_CP_CTRL = crate::Reg<ulp_cp_ctrl::ULP_CP_CTRL_SPEC>;
#[doc = "ULP-FSM configuration register"]
pub mod ulp_cp_ctrl;
#[doc = "COCPU_CTRL (rw) register accessor: an alias for `Reg<COCPU_CTRL_SPEC>`"]
pub type COCPU_CTRL = crate::Reg<cocpu_ctrl::COCPU_CTRL_SPEC>;
#[doc = "ULP-RISCV configuration register"]
pub mod cocpu_ctrl;
#[doc = "ULP_CP_TIMER_1 (rw) register accessor: an alias for `Reg<ULP_CP_TIMER_1_SPEC>`"]
pub type ULP_CP_TIMER_1 = crate::Reg<ulp_cp_timer_1::ULP_CP_TIMER_1_SPEC>;
#[doc = "Configure sleep cycle of the timer"]
pub mod ulp_cp_timer_1;
