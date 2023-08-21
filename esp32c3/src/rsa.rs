#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00..0x180 - The memory that stores M"]
    pub m_mem: [M_MEM; 384],
    _reserved1: [u8; 0x80],
    #[doc = "0x200..0x380 - The memory that stores Z"]
    pub z_mem: [Z_MEM; 384],
    _reserved2: [u8; 0x80],
    #[doc = "0x400..0x580 - The memory that stores Y"]
    pub y_mem: [Y_MEM; 384],
    _reserved3: [u8; 0x80],
    #[doc = "0x600..0x780 - The memory that stores X"]
    pub x_mem: [X_MEM; 384],
    _reserved4: [u8; 0x80],
    #[doc = "0x800 - RSA M_prime register"]
    pub m_prime: M_PRIME,
    #[doc = "0x804 - RSA mode register"]
    pub mode: MODE,
    #[doc = "0x808 - RSA query clean register"]
    pub query_clean: QUERY_CLEAN,
    #[doc = "0x80c - RSA modular exponentiation trigger register."]
    pub set_start_modexp: SET_START_MODEXP,
    #[doc = "0x810 - RSA modular multiplication trigger register."]
    pub set_start_modmult: SET_START_MODMULT,
    #[doc = "0x814 - RSA normal multiplication trigger register."]
    pub set_start_mult: SET_START_MULT,
    #[doc = "0x818 - RSA query idle register"]
    pub query_idle: QUERY_IDLE,
    #[doc = "0x81c - RSA interrupt clear register"]
    pub int_clr: INT_CLR,
    #[doc = "0x820 - RSA constant time option register"]
    pub constant_time: CONSTANT_TIME,
    #[doc = "0x824 - RSA search option"]
    pub search_enable: SEARCH_ENABLE,
    #[doc = "0x828 - RSA search position configure register"]
    pub search_pos: SEARCH_POS,
    #[doc = "0x82c - RSA interrupt enable register"]
    pub int_ena: INT_ENA,
    #[doc = "0x830 - RSA version control register"]
    pub date: DATE,
}
#[doc = "M_MEM (rw) register accessor: The memory that stores M\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m_mem`] module"]
pub type M_MEM = crate::Reg<m_mem::M_MEM_SPEC>;
#[doc = "The memory that stores M"]
pub mod m_mem;
#[doc = "Z_MEM (rw) register accessor: The memory that stores Z\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`z_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`z_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`z_mem`] module"]
pub type Z_MEM = crate::Reg<z_mem::Z_MEM_SPEC>;
#[doc = "The memory that stores Z"]
pub mod z_mem;
#[doc = "Y_MEM (rw) register accessor: The memory that stores Y\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`y_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`y_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`y_mem`] module"]
pub type Y_MEM = crate::Reg<y_mem::Y_MEM_SPEC>;
#[doc = "The memory that stores Y"]
pub mod y_mem;
#[doc = "X_MEM (rw) register accessor: The memory that stores X\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`x_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`x_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`x_mem`] module"]
pub type X_MEM = crate::Reg<x_mem::X_MEM_SPEC>;
#[doc = "The memory that stores X"]
pub mod x_mem;
#[doc = "M_PRIME (rw) register accessor: RSA M_prime register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m_prime::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_prime::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m_prime`] module"]
pub type M_PRIME = crate::Reg<m_prime::M_PRIME_SPEC>;
#[doc = "RSA M_prime register"]
pub mod m_prime;
#[doc = "MODE (rw) register accessor: RSA mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mode`] module"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "RSA mode register"]
pub mod mode;
#[doc = "QUERY_CLEAN (r) register accessor: RSA query clean register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`query_clean::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`query_clean`] module"]
pub type QUERY_CLEAN = crate::Reg<query_clean::QUERY_CLEAN_SPEC>;
#[doc = "RSA query clean register"]
pub mod query_clean;
#[doc = "SET_START_MODEXP (w) register accessor: RSA modular exponentiation trigger register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_start_modexp::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`set_start_modexp`] module"]
pub type SET_START_MODEXP = crate::Reg<set_start_modexp::SET_START_MODEXP_SPEC>;
#[doc = "RSA modular exponentiation trigger register."]
pub mod set_start_modexp;
#[doc = "SET_START_MODMULT (w) register accessor: RSA modular multiplication trigger register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_start_modmult::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`set_start_modmult`] module"]
pub type SET_START_MODMULT = crate::Reg<set_start_modmult::SET_START_MODMULT_SPEC>;
#[doc = "RSA modular multiplication trigger register."]
pub mod set_start_modmult;
#[doc = "SET_START_MULT (w) register accessor: RSA normal multiplication trigger register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_start_mult::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`set_start_mult`] module"]
pub type SET_START_MULT = crate::Reg<set_start_mult::SET_START_MULT_SPEC>;
#[doc = "RSA normal multiplication trigger register."]
pub mod set_start_mult;
#[doc = "QUERY_IDLE (r) register accessor: RSA query idle register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`query_idle::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`query_idle`] module"]
pub type QUERY_IDLE = crate::Reg<query_idle::QUERY_IDLE_SPEC>;
#[doc = "RSA query idle register"]
pub mod query_idle;
#[doc = "INT_CLR (w) register accessor: RSA interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "RSA interrupt clear register"]
pub mod int_clr;
#[doc = "CONSTANT_TIME (rw) register accessor: RSA constant time option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`constant_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`constant_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`constant_time`] module"]
pub type CONSTANT_TIME = crate::Reg<constant_time::CONSTANT_TIME_SPEC>;
#[doc = "RSA constant time option register"]
pub mod constant_time;
#[doc = "SEARCH_ENABLE (rw) register accessor: RSA search option\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`search_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`search_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`search_enable`] module"]
pub type SEARCH_ENABLE = crate::Reg<search_enable::SEARCH_ENABLE_SPEC>;
#[doc = "RSA search option"]
pub mod search_enable;
#[doc = "SEARCH_POS (rw) register accessor: RSA search position configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`search_pos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`search_pos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`search_pos`] module"]
pub type SEARCH_POS = crate::Reg<search_pos::SEARCH_POS_SPEC>;
#[doc = "RSA search position configure register"]
pub mod search_pos;
#[doc = "INT_ENA (rw) register accessor: RSA interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "RSA interrupt enable register"]
pub mod int_ena;
#[doc = "DATE (rw) register accessor: RSA version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "RSA version control register"]
pub mod date;
