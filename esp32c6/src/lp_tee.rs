#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Tee mode control register"]
    pub m0_mode_ctrl: M0_MODE_CTRL,
    #[doc = "0x04 - Clock gating register"]
    pub clock_gate: CLOCK_GATE,
    _reserved2: [u8; 0x88],
    #[doc = "0x90 - need_des"]
    pub force_acc_hp: FORCE_ACC_HP,
    _reserved3: [u8; 0x68],
    #[doc = "0xfc - Version register"]
    pub date: DATE,
}
#[doc = "M0_MODE_CTRL (rw) register accessor: an alias for `Reg<M0_MODE_CTRL_SPEC>`"]
pub type M0_MODE_CTRL = crate::Reg<m0_mode_ctrl::M0_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m0_mode_ctrl;
#[doc = "CLOCK_GATE (rw) register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Clock gating register"]
pub mod clock_gate;
#[doc = "FORCE_ACC_HP (rw) register accessor: an alias for `Reg<FORCE_ACC_HP_SPEC>`"]
pub type FORCE_ACC_HP = crate::Reg<force_acc_hp::FORCE_ACC_HP_SPEC>;
#[doc = "need_des"]
pub mod force_acc_hp;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version register"]
pub mod date;
