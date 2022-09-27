#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub m_prime: M_PRIME,
    #[doc = "0x04 - "]
    pub modexp_mode: MODEXP_MODE,
    #[doc = "0x08 - "]
    pub modexp_start: MODEXP_START,
    #[doc = "0x0c - "]
    pub mult_mode: MULT_MODE,
    #[doc = "0x10 - "]
    pub mult_start: MULT_START,
    #[doc = "0x14 - "]
    pub interrupt: INTERRUPT,
    #[doc = "0x18 - "]
    pub clean: CLEAN,
}
#[doc = "M_PRIME (rw) register accessor: an alias for `Reg<M_PRIME_SPEC>`"]
pub type M_PRIME = crate::Reg<m_prime::M_PRIME_SPEC>;
#[doc = ""]
pub mod m_prime;
#[doc = "MODEXP_MODE (rw) register accessor: an alias for `Reg<MODEXP_MODE_SPEC>`"]
pub type MODEXP_MODE = crate::Reg<modexp_mode::MODEXP_MODE_SPEC>;
#[doc = ""]
pub mod modexp_mode;
#[doc = "MODEXP_START (w) register accessor: an alias for `Reg<MODEXP_START_SPEC>`"]
pub type MODEXP_START = crate::Reg<modexp_start::MODEXP_START_SPEC>;
#[doc = ""]
pub mod modexp_start;
#[doc = "MULT_MODE (rw) register accessor: an alias for `Reg<MULT_MODE_SPEC>`"]
pub type MULT_MODE = crate::Reg<mult_mode::MULT_MODE_SPEC>;
#[doc = ""]
pub mod mult_mode;
#[doc = "MULT_START (w) register accessor: an alias for `Reg<MULT_START_SPEC>`"]
pub type MULT_START = crate::Reg<mult_start::MULT_START_SPEC>;
#[doc = ""]
pub mod mult_start;
#[doc = "INTERRUPT (rw) register accessor: an alias for `Reg<INTERRUPT_SPEC>`"]
pub type INTERRUPT = crate::Reg<interrupt::INTERRUPT_SPEC>;
#[doc = ""]
pub mod interrupt;
#[doc = "CLEAN (r) register accessor: an alias for `Reg<CLEAN_SPEC>`"]
pub type CLEAN = crate::Reg<clean::CLEAN_SPEC>;
#[doc = ""]
pub mod clean;
