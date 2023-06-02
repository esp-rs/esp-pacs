#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub start: START,
    #[doc = "0x04 - "]
    pub idle: IDLE,
    #[doc = "0x08 - "]
    pub mode: MODE,
    _reserved3: [u8; 0x04],
    #[doc = "0x10..0x30 - "]
    pub key_: [KEY_; 8],
    #[doc = "0x30..0x40 - "]
    pub text_: [TEXT_; 4],
    #[doc = "0x40 - "]
    pub endian: ENDIAN,
}
#[doc = "START (w) register accessor: an alias for `Reg<START_SPEC>`"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = ""]
pub mod start;
#[doc = "IDLE (r) register accessor: an alias for `Reg<IDLE_SPEC>`"]
pub type IDLE = crate::Reg<idle::IDLE_SPEC>;
#[doc = ""]
pub mod idle;
#[doc = "MODE (rw) register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = ""]
pub mod mode;
#[doc = "KEY_ (rw) register accessor: an alias for `Reg<KEY__SPEC>`"]
pub type KEY_ = crate::Reg<key_::KEY__SPEC>;
#[doc = ""]
pub mod key_;
#[doc = "TEXT_ (rw) register accessor: an alias for `Reg<TEXT__SPEC>`"]
pub type TEXT_ = crate::Reg<text_::TEXT__SPEC>;
#[doc = ""]
pub mod text_;
#[doc = "ENDIAN (rw) register accessor: an alias for `Reg<ENDIAN_SPEC>`"]
pub type ENDIAN = crate::Reg<endian::ENDIAN_SPEC>;
#[doc = ""]
pub mod endian;
