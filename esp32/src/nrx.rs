#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    _reserved0: [u8; 0xd4],
    #[doc = "0xd4 - WiFi RX control register"]
    pub nrxpd_ctrl: NRXPD_CTRL,
}
#[doc = "NRXPD_CTRL (rw) register accessor: an alias for `Reg<NRXPD_CTRL_SPEC>`"]
pub type NRXPD_CTRL = crate::Reg<nrxpd_ctrl::NRXPD_CTRL_SPEC>;
#[doc = "WiFi RX control register"]
pub mod nrxpd_ctrl;
