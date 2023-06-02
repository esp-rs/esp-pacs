#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Tee mode control register"]
    pub m0_mode_ctrl: M0_MODE_CTRL,
    #[doc = "0x04 - Tee mode control register"]
    pub m1_mode_ctrl: M1_MODE_CTRL,
    #[doc = "0x08 - Tee mode control register"]
    pub m2_mode_ctrl: M2_MODE_CTRL,
    #[doc = "0x0c - Tee mode control register"]
    pub m3_mode_ctrl: M3_MODE_CTRL,
    #[doc = "0x10 - Tee mode control register"]
    pub m4_mode_ctrl: M4_MODE_CTRL,
    #[doc = "0x14 - Tee mode control register"]
    pub m5_mode_ctrl: M5_MODE_CTRL,
    #[doc = "0x18 - Tee mode control register"]
    pub m6_mode_ctrl: M6_MODE_CTRL,
    #[doc = "0x1c - Tee mode control register"]
    pub m7_mode_ctrl: M7_MODE_CTRL,
    #[doc = "0x20 - Tee mode control register"]
    pub m8_mode_ctrl: M8_MODE_CTRL,
    #[doc = "0x24 - Tee mode control register"]
    pub m9_mode_ctrl: M9_MODE_CTRL,
    #[doc = "0x28 - Tee mode control register"]
    pub m10_mode_ctrl: M10_MODE_CTRL,
    #[doc = "0x2c - Tee mode control register"]
    pub m11_mode_ctrl: M11_MODE_CTRL,
    #[doc = "0x30 - Tee mode control register"]
    pub m12_mode_ctrl: M12_MODE_CTRL,
    #[doc = "0x34 - Tee mode control register"]
    pub m13_mode_ctrl: M13_MODE_CTRL,
    #[doc = "0x38 - Tee mode control register"]
    pub m14_mode_ctrl: M14_MODE_CTRL,
    #[doc = "0x3c - Tee mode control register"]
    pub m15_mode_ctrl: M15_MODE_CTRL,
    #[doc = "0x40 - Tee mode control register"]
    pub m16_mode_ctrl: M16_MODE_CTRL,
    #[doc = "0x44 - Tee mode control register"]
    pub m17_mode_ctrl: M17_MODE_CTRL,
    #[doc = "0x48 - Tee mode control register"]
    pub m18_mode_ctrl: M18_MODE_CTRL,
    #[doc = "0x4c - Tee mode control register"]
    pub m19_mode_ctrl: M19_MODE_CTRL,
    #[doc = "0x50 - Tee mode control register"]
    pub m20_mode_ctrl: M20_MODE_CTRL,
    #[doc = "0x54 - Tee mode control register"]
    pub m21_mode_ctrl: M21_MODE_CTRL,
    #[doc = "0x58 - Tee mode control register"]
    pub m22_mode_ctrl: M22_MODE_CTRL,
    #[doc = "0x5c - Tee mode control register"]
    pub m23_mode_ctrl: M23_MODE_CTRL,
    #[doc = "0x60 - Tee mode control register"]
    pub m24_mode_ctrl: M24_MODE_CTRL,
    #[doc = "0x64 - Tee mode control register"]
    pub m25_mode_ctrl: M25_MODE_CTRL,
    #[doc = "0x68 - Tee mode control register"]
    pub m26_mode_ctrl: M26_MODE_CTRL,
    #[doc = "0x6c - Tee mode control register"]
    pub m27_mode_ctrl: M27_MODE_CTRL,
    #[doc = "0x70 - Tee mode control register"]
    pub m28_mode_ctrl: M28_MODE_CTRL,
    #[doc = "0x74 - Tee mode control register"]
    pub m29_mode_ctrl: M29_MODE_CTRL,
    #[doc = "0x78 - Tee mode control register"]
    pub m30_mode_ctrl: M30_MODE_CTRL,
    #[doc = "0x7c - Tee mode control register"]
    pub m31_mode_ctrl: M31_MODE_CTRL,
    #[doc = "0x80 - Clock gating register"]
    pub clock_gate: CLOCK_GATE,
    _reserved33: [u8; 0x0f78],
    #[doc = "0xffc - Version register"]
    pub date: DATE,
}
#[doc = "M0_MODE_CTRL (rw) register accessor: an alias for `Reg<M0_MODE_CTRL_SPEC>`"]
pub type M0_MODE_CTRL = crate::Reg<m0_mode_ctrl::M0_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m0_mode_ctrl;
#[doc = "M1_MODE_CTRL (rw) register accessor: an alias for `Reg<M1_MODE_CTRL_SPEC>`"]
pub type M1_MODE_CTRL = crate::Reg<m1_mode_ctrl::M1_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m1_mode_ctrl;
#[doc = "M2_MODE_CTRL (rw) register accessor: an alias for `Reg<M2_MODE_CTRL_SPEC>`"]
pub type M2_MODE_CTRL = crate::Reg<m2_mode_ctrl::M2_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m2_mode_ctrl;
#[doc = "M3_MODE_CTRL (rw) register accessor: an alias for `Reg<M3_MODE_CTRL_SPEC>`"]
pub type M3_MODE_CTRL = crate::Reg<m3_mode_ctrl::M3_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m3_mode_ctrl;
#[doc = "M4_MODE_CTRL (rw) register accessor: an alias for `Reg<M4_MODE_CTRL_SPEC>`"]
pub type M4_MODE_CTRL = crate::Reg<m4_mode_ctrl::M4_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m4_mode_ctrl;
#[doc = "M5_MODE_CTRL (rw) register accessor: an alias for `Reg<M5_MODE_CTRL_SPEC>`"]
pub type M5_MODE_CTRL = crate::Reg<m5_mode_ctrl::M5_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m5_mode_ctrl;
#[doc = "M6_MODE_CTRL (rw) register accessor: an alias for `Reg<M6_MODE_CTRL_SPEC>`"]
pub type M6_MODE_CTRL = crate::Reg<m6_mode_ctrl::M6_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m6_mode_ctrl;
#[doc = "M7_MODE_CTRL (rw) register accessor: an alias for `Reg<M7_MODE_CTRL_SPEC>`"]
pub type M7_MODE_CTRL = crate::Reg<m7_mode_ctrl::M7_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m7_mode_ctrl;
#[doc = "M8_MODE_CTRL (rw) register accessor: an alias for `Reg<M8_MODE_CTRL_SPEC>`"]
pub type M8_MODE_CTRL = crate::Reg<m8_mode_ctrl::M8_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m8_mode_ctrl;
#[doc = "M9_MODE_CTRL (rw) register accessor: an alias for `Reg<M9_MODE_CTRL_SPEC>`"]
pub type M9_MODE_CTRL = crate::Reg<m9_mode_ctrl::M9_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m9_mode_ctrl;
#[doc = "M10_MODE_CTRL (rw) register accessor: an alias for `Reg<M10_MODE_CTRL_SPEC>`"]
pub type M10_MODE_CTRL = crate::Reg<m10_mode_ctrl::M10_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m10_mode_ctrl;
#[doc = "M11_MODE_CTRL (rw) register accessor: an alias for `Reg<M11_MODE_CTRL_SPEC>`"]
pub type M11_MODE_CTRL = crate::Reg<m11_mode_ctrl::M11_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m11_mode_ctrl;
#[doc = "M12_MODE_CTRL (rw) register accessor: an alias for `Reg<M12_MODE_CTRL_SPEC>`"]
pub type M12_MODE_CTRL = crate::Reg<m12_mode_ctrl::M12_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m12_mode_ctrl;
#[doc = "M13_MODE_CTRL (rw) register accessor: an alias for `Reg<M13_MODE_CTRL_SPEC>`"]
pub type M13_MODE_CTRL = crate::Reg<m13_mode_ctrl::M13_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m13_mode_ctrl;
#[doc = "M14_MODE_CTRL (rw) register accessor: an alias for `Reg<M14_MODE_CTRL_SPEC>`"]
pub type M14_MODE_CTRL = crate::Reg<m14_mode_ctrl::M14_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m14_mode_ctrl;
#[doc = "M15_MODE_CTRL (rw) register accessor: an alias for `Reg<M15_MODE_CTRL_SPEC>`"]
pub type M15_MODE_CTRL = crate::Reg<m15_mode_ctrl::M15_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m15_mode_ctrl;
#[doc = "M16_MODE_CTRL (rw) register accessor: an alias for `Reg<M16_MODE_CTRL_SPEC>`"]
pub type M16_MODE_CTRL = crate::Reg<m16_mode_ctrl::M16_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m16_mode_ctrl;
#[doc = "M17_MODE_CTRL (rw) register accessor: an alias for `Reg<M17_MODE_CTRL_SPEC>`"]
pub type M17_MODE_CTRL = crate::Reg<m17_mode_ctrl::M17_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m17_mode_ctrl;
#[doc = "M18_MODE_CTRL (rw) register accessor: an alias for `Reg<M18_MODE_CTRL_SPEC>`"]
pub type M18_MODE_CTRL = crate::Reg<m18_mode_ctrl::M18_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m18_mode_ctrl;
#[doc = "M19_MODE_CTRL (rw) register accessor: an alias for `Reg<M19_MODE_CTRL_SPEC>`"]
pub type M19_MODE_CTRL = crate::Reg<m19_mode_ctrl::M19_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m19_mode_ctrl;
#[doc = "M20_MODE_CTRL (rw) register accessor: an alias for `Reg<M20_MODE_CTRL_SPEC>`"]
pub type M20_MODE_CTRL = crate::Reg<m20_mode_ctrl::M20_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m20_mode_ctrl;
#[doc = "M21_MODE_CTRL (rw) register accessor: an alias for `Reg<M21_MODE_CTRL_SPEC>`"]
pub type M21_MODE_CTRL = crate::Reg<m21_mode_ctrl::M21_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m21_mode_ctrl;
#[doc = "M22_MODE_CTRL (rw) register accessor: an alias for `Reg<M22_MODE_CTRL_SPEC>`"]
pub type M22_MODE_CTRL = crate::Reg<m22_mode_ctrl::M22_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m22_mode_ctrl;
#[doc = "M23_MODE_CTRL (rw) register accessor: an alias for `Reg<M23_MODE_CTRL_SPEC>`"]
pub type M23_MODE_CTRL = crate::Reg<m23_mode_ctrl::M23_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m23_mode_ctrl;
#[doc = "M24_MODE_CTRL (rw) register accessor: an alias for `Reg<M24_MODE_CTRL_SPEC>`"]
pub type M24_MODE_CTRL = crate::Reg<m24_mode_ctrl::M24_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m24_mode_ctrl;
#[doc = "M25_MODE_CTRL (rw) register accessor: an alias for `Reg<M25_MODE_CTRL_SPEC>`"]
pub type M25_MODE_CTRL = crate::Reg<m25_mode_ctrl::M25_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m25_mode_ctrl;
#[doc = "M26_MODE_CTRL (rw) register accessor: an alias for `Reg<M26_MODE_CTRL_SPEC>`"]
pub type M26_MODE_CTRL = crate::Reg<m26_mode_ctrl::M26_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m26_mode_ctrl;
#[doc = "M27_MODE_CTRL (rw) register accessor: an alias for `Reg<M27_MODE_CTRL_SPEC>`"]
pub type M27_MODE_CTRL = crate::Reg<m27_mode_ctrl::M27_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m27_mode_ctrl;
#[doc = "M28_MODE_CTRL (rw) register accessor: an alias for `Reg<M28_MODE_CTRL_SPEC>`"]
pub type M28_MODE_CTRL = crate::Reg<m28_mode_ctrl::M28_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m28_mode_ctrl;
#[doc = "M29_MODE_CTRL (rw) register accessor: an alias for `Reg<M29_MODE_CTRL_SPEC>`"]
pub type M29_MODE_CTRL = crate::Reg<m29_mode_ctrl::M29_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m29_mode_ctrl;
#[doc = "M30_MODE_CTRL (rw) register accessor: an alias for `Reg<M30_MODE_CTRL_SPEC>`"]
pub type M30_MODE_CTRL = crate::Reg<m30_mode_ctrl::M30_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m30_mode_ctrl;
#[doc = "M31_MODE_CTRL (rw) register accessor: an alias for `Reg<M31_MODE_CTRL_SPEC>`"]
pub type M31_MODE_CTRL = crate::Reg<m31_mode_ctrl::M31_MODE_CTRL_SPEC>;
#[doc = "Tee mode control register"]
pub mod m31_mode_ctrl;
#[doc = "CLOCK_GATE (rw) register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Clock gating register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version register"]
pub mod date;
