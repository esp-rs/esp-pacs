#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00..0x200 - Represents M"]
    pub m_mem: [M_MEM; 512],
    #[doc = "0x200..0x400 - Represents Z"]
    pub z_mem: [Z_MEM; 512],
    #[doc = "0x400..0x600 - Represents Y"]
    pub y_mem: [Y_MEM; 512],
    #[doc = "0x600..0x800 - Represents X"]
    pub x_mem: [X_MEM; 512],
    #[doc = "0x800 - Register to store M'"]
    pub m_prime: M_PRIME,
    #[doc = "0x804 - RSA length mode"]
    pub mode: MODE,
    #[doc = "0x808 - RSA clean register"]
    pub clean: CLEAN,
    #[doc = "0x80c - Modular exponentiation starting bit"]
    pub modexp_start: MODEXP_START,
    #[doc = "0x810 - Modular multiplication starting bit"]
    pub modmult_start: MODMULT_START,
    #[doc = "0x814 - Normal multiplication starting bit"]
    pub mult_start: MULT_START,
    #[doc = "0x818 - RSA idle register"]
    pub idle: IDLE,
    #[doc = "0x81c - RSA clear interrupt register"]
    pub clear_interrupt: CLEAR_INTERRUPT,
    #[doc = "0x820 - The constant_time option"]
    pub constant_time: CONSTANT_TIME,
    #[doc = "0x824 - The search option"]
    pub search_enable: SEARCH_ENABLE,
    #[doc = "0x828 - The search position"]
    pub search_pos: SEARCH_POS,
    #[doc = "0x82c - RSA interrupt enable register"]
    pub interrupt_ena: INTERRUPT_ENA,
    #[doc = "0x830 - Version control register"]
    pub date: DATE,
}
#[doc = "M_PRIME (rw) register accessor: an alias for `Reg<M_PRIME_SPEC>`"]
pub type M_PRIME = crate::Reg<m_prime::M_PRIME_SPEC>;
#[doc = "Register to store M'"]
pub mod m_prime;
#[doc = "MODE (rw) register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "RSA length mode"]
pub mod mode;
#[doc = "CLEAN (r) register accessor: an alias for `Reg<CLEAN_SPEC>`"]
pub type CLEAN = crate::Reg<clean::CLEAN_SPEC>;
#[doc = "RSA clean register"]
pub mod clean;
#[doc = "MODEXP_START (w) register accessor: an alias for `Reg<MODEXP_START_SPEC>`"]
pub type MODEXP_START = crate::Reg<modexp_start::MODEXP_START_SPEC>;
#[doc = "Modular exponentiation starting bit"]
pub mod modexp_start;
#[doc = "MODMULT_START (w) register accessor: an alias for `Reg<MODMULT_START_SPEC>`"]
pub type MODMULT_START = crate::Reg<modmult_start::MODMULT_START_SPEC>;
#[doc = "Modular multiplication starting bit"]
pub mod modmult_start;
#[doc = "MULT_START (w) register accessor: an alias for `Reg<MULT_START_SPEC>`"]
pub type MULT_START = crate::Reg<mult_start::MULT_START_SPEC>;
#[doc = "Normal multiplication starting bit"]
pub mod mult_start;
#[doc = "IDLE (r) register accessor: an alias for `Reg<IDLE_SPEC>`"]
pub type IDLE = crate::Reg<idle::IDLE_SPEC>;
#[doc = "RSA idle register"]
pub mod idle;
#[doc = "CLEAR_INTERRUPT (w) register accessor: an alias for `Reg<CLEAR_INTERRUPT_SPEC>`"]
pub type CLEAR_INTERRUPT = crate::Reg<clear_interrupt::CLEAR_INTERRUPT_SPEC>;
#[doc = "RSA clear interrupt register"]
pub mod clear_interrupt;
#[doc = "CONSTANT_TIME (rw) register accessor: an alias for `Reg<CONSTANT_TIME_SPEC>`"]
pub type CONSTANT_TIME = crate::Reg<constant_time::CONSTANT_TIME_SPEC>;
#[doc = "The constant_time option"]
pub mod constant_time;
#[doc = "SEARCH_ENABLE (rw) register accessor: an alias for `Reg<SEARCH_ENABLE_SPEC>`"]
pub type SEARCH_ENABLE = crate::Reg<search_enable::SEARCH_ENABLE_SPEC>;
#[doc = "The search option"]
pub mod search_enable;
#[doc = "SEARCH_POS (rw) register accessor: an alias for `Reg<SEARCH_POS_SPEC>`"]
pub type SEARCH_POS = crate::Reg<search_pos::SEARCH_POS_SPEC>;
#[doc = "The search position"]
pub mod search_pos;
#[doc = "INTERRUPT_ENA (rw) register accessor: an alias for `Reg<INTERRUPT_ENA_SPEC>`"]
pub type INTERRUPT_ENA = crate::Reg<interrupt_ena::INTERRUPT_ENA_SPEC>;
#[doc = "RSA interrupt enable register"]
pub mod interrupt_ena;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
#[doc = "M_MEM (w) register accessor: an alias for `Reg<M_MEM_SPEC>`"]
pub type M_MEM = crate::Reg<m_mem::M_MEM_SPEC>;
#[doc = "Represents M"]
pub mod m_mem;
#[doc = "Z_MEM (rw) register accessor: an alias for `Reg<Z_MEM_SPEC>`"]
pub type Z_MEM = crate::Reg<z_mem::Z_MEM_SPEC>;
#[doc = "Represents Z"]
pub mod z_mem;
#[doc = "Y_MEM (w) register accessor: an alias for `Reg<Y_MEM_SPEC>`"]
pub type Y_MEM = crate::Reg<y_mem::Y_MEM_SPEC>;
#[doc = "Represents Y"]
pub mod y_mem;
#[doc = "X_MEM (w) register accessor: an alias for `Reg<X_MEM_SPEC>`"]
pub type X_MEM = crate::Reg<x_mem::X_MEM_SPEC>;
#[doc = "Represents X"]
pub mod x_mem;
