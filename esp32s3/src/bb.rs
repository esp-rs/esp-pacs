#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    _reserved0: [u8; 0x54],
    #[doc = "0x54 - Baseband control register"]
    pub bbpd_ctrl: BBPD_CTRL,
}
#[doc = "BBPD_CTRL (rw) register accessor: an alias for `Reg<BBPD_CTRL_SPEC>`"]
pub type BBPD_CTRL = crate::Reg<bbpd_ctrl::BBPD_CTRL_SPEC>;
#[doc = "Baseband control register"]
pub mod bbpd_ctrl;
