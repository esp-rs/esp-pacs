#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00..0x180 - Represents M"]
    pub m_mem: [M_MEM; 384],
    _reserved1: [u8; 0x80],
    #[doc = "0x200..0x380 - Represents Z"]
    pub z_mem: [Z_MEM; 384],
    _reserved2: [u8; 0x80],
    #[doc = "0x400..0x580 - Represents Y"]
    pub y_mem: [Y_MEM; 384],
    _reserved3: [u8; 0x80],
    #[doc = "0x600..0x780 - Represents X"]
    pub x_mem: [X_MEM; 384],
    _reserved4: [u8; 0x80],
    #[doc = "0x800 - Represents M’"]
    pub m_prime: M_PRIME,
    #[doc = "0x804 - Configures RSA length"]
    pub mode: MODE,
    #[doc = "0x808 - RSA clean register"]
    pub query_clean: QUERY_CLEAN,
    #[doc = "0x80c - Starts modular exponentiation"]
    pub set_start_modexp: SET_START_MODEXP,
    #[doc = "0x810 - Starts modular multiplication"]
    pub set_start_modmult: SET_START_MODMULT,
    #[doc = "0x814 - Starts multiplication"]
    pub set_start_mult: SET_START_MULT,
    #[doc = "0x818 - Represents the RSA status"]
    pub query_idle: QUERY_IDLE,
    #[doc = "0x81c - Clears RSA interrupt"]
    pub int_clr: INT_CLR,
    #[doc = "0x820 - Configures the constant_time option"]
    pub constant_time: CONSTANT_TIME,
    #[doc = "0x824 - Configures the search option"]
    pub search_enable: SEARCH_ENABLE,
    #[doc = "0x828 - Configures the search position"]
    pub search_pos: SEARCH_POS,
    #[doc = "0x82c - Enables the RSA interrupt"]
    pub int_ena: INT_ENA,
    #[doc = "0x830 - Version control register"]
    pub date: DATE,
}
#[doc = "M_MEM (rw) register accessor: an alias for `Reg<M_MEM_SPEC>`"]
pub type M_MEM = crate::Reg<m_mem::M_MEM_SPEC>;
#[doc = "Represents M"]
pub mod m_mem;
#[doc = "Z_MEM (rw) register accessor: an alias for `Reg<Z_MEM_SPEC>`"]
pub type Z_MEM = crate::Reg<z_mem::Z_MEM_SPEC>;
#[doc = "Represents Z"]
pub mod z_mem;
#[doc = "Y_MEM (rw) register accessor: an alias for `Reg<Y_MEM_SPEC>`"]
pub type Y_MEM = crate::Reg<y_mem::Y_MEM_SPEC>;
#[doc = "Represents Y"]
pub mod y_mem;
#[doc = "X_MEM (rw) register accessor: an alias for `Reg<X_MEM_SPEC>`"]
pub type X_MEM = crate::Reg<x_mem::X_MEM_SPEC>;
#[doc = "Represents X"]
pub mod x_mem;
#[doc = "M_PRIME (rw) register accessor: an alias for `Reg<M_PRIME_SPEC>`"]
pub type M_PRIME = crate::Reg<m_prime::M_PRIME_SPEC>;
#[doc = "Represents M’"]
pub mod m_prime;
#[doc = "MODE (rw) register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Configures RSA length"]
pub mod mode;
#[doc = "QUERY_CLEAN (r) register accessor: an alias for `Reg<QUERY_CLEAN_SPEC>`"]
pub type QUERY_CLEAN = crate::Reg<query_clean::QUERY_CLEAN_SPEC>;
#[doc = "RSA clean register"]
pub mod query_clean;
#[doc = "SET_START_MODEXP (w) register accessor: an alias for `Reg<SET_START_MODEXP_SPEC>`"]
pub type SET_START_MODEXP = crate::Reg<set_start_modexp::SET_START_MODEXP_SPEC>;
#[doc = "Starts modular exponentiation"]
pub mod set_start_modexp;
#[doc = "SET_START_MODMULT (w) register accessor: an alias for `Reg<SET_START_MODMULT_SPEC>`"]
pub type SET_START_MODMULT = crate::Reg<set_start_modmult::SET_START_MODMULT_SPEC>;
#[doc = "Starts modular multiplication"]
pub mod set_start_modmult;
#[doc = "SET_START_MULT (w) register accessor: an alias for `Reg<SET_START_MULT_SPEC>`"]
pub type SET_START_MULT = crate::Reg<set_start_mult::SET_START_MULT_SPEC>;
#[doc = "Starts multiplication"]
pub mod set_start_mult;
#[doc = "QUERY_IDLE (r) register accessor: an alias for `Reg<QUERY_IDLE_SPEC>`"]
pub type QUERY_IDLE = crate::Reg<query_idle::QUERY_IDLE_SPEC>;
#[doc = "Represents the RSA status"]
pub mod query_idle;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Clears RSA interrupt"]
pub mod int_clr;
#[doc = "CONSTANT_TIME (rw) register accessor: an alias for `Reg<CONSTANT_TIME_SPEC>`"]
pub type CONSTANT_TIME = crate::Reg<constant_time::CONSTANT_TIME_SPEC>;
#[doc = "Configures the constant_time option"]
pub mod constant_time;
#[doc = "SEARCH_ENABLE (rw) register accessor: an alias for `Reg<SEARCH_ENABLE_SPEC>`"]
pub type SEARCH_ENABLE = crate::Reg<search_enable::SEARCH_ENABLE_SPEC>;
#[doc = "Configures the search option"]
pub mod search_enable;
#[doc = "SEARCH_POS (rw) register accessor: an alias for `Reg<SEARCH_POS_SPEC>`"]
pub type SEARCH_POS = crate::Reg<search_pos::SEARCH_POS_SPEC>;
#[doc = "Configures the search position"]
pub mod search_pos;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Enables the RSA interrupt"]
pub mod int_ena;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
