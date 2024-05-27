#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    m_mem: [M_MEM; 96],
    _reserved1: [u8; 0x80],
    z_mem: [Z_MEM; 96],
    _reserved2: [u8; 0x80],
    y_mem: [Y_MEM; 96],
    _reserved3: [u8; 0x80],
    x_mem: [X_MEM; 96],
    _reserved4: [u8; 0x80],
    m_prime: M_PRIME,
    mode: MODE,
    query_clean: QUERY_CLEAN,
    set_start_modexp: SET_START_MODEXP,
    set_start_modmult: SET_START_MODMULT,
    set_start_mult: SET_START_MULT,
    query_idle: QUERY_IDLE,
    int_clr: INT_CLR,
    constant_time: CONSTANT_TIME,
    search_enable: SEARCH_ENABLE,
    search_pos: SEARCH_POS,
    int_ena: INT_ENA,
    date: DATE,
}
impl RegisterBlock {
    ///0x00..0x180 - Represents M
    #[inline(always)]
    pub const fn m_mem(&self, n: usize) -> &M_MEM {
        &self.m_mem[n]
    }
    ///Iterator for array of:
    ///0x00..0x180 - Represents M
    #[inline(always)]
    pub fn m_mem_iter(&self) -> impl Iterator<Item = &M_MEM> {
        self.m_mem.iter()
    }
    ///0x200..0x380 - Represents Z
    #[inline(always)]
    pub const fn z_mem(&self, n: usize) -> &Z_MEM {
        &self.z_mem[n]
    }
    ///Iterator for array of:
    ///0x200..0x380 - Represents Z
    #[inline(always)]
    pub fn z_mem_iter(&self) -> impl Iterator<Item = &Z_MEM> {
        self.z_mem.iter()
    }
    ///0x400..0x580 - Represents Y
    #[inline(always)]
    pub const fn y_mem(&self, n: usize) -> &Y_MEM {
        &self.y_mem[n]
    }
    ///Iterator for array of:
    ///0x400..0x580 - Represents Y
    #[inline(always)]
    pub fn y_mem_iter(&self) -> impl Iterator<Item = &Y_MEM> {
        self.y_mem.iter()
    }
    ///0x600..0x780 - Represents X
    #[inline(always)]
    pub const fn x_mem(&self, n: usize) -> &X_MEM {
        &self.x_mem[n]
    }
    ///Iterator for array of:
    ///0x600..0x780 - Represents X
    #[inline(always)]
    pub fn x_mem_iter(&self) -> impl Iterator<Item = &X_MEM> {
        self.x_mem.iter()
    }
    ///0x800 - Represents M’
    #[inline(always)]
    pub const fn m_prime(&self) -> &M_PRIME {
        &self.m_prime
    }
    ///0x804 - Configures RSA length
    #[inline(always)]
    pub const fn mode(&self) -> &MODE {
        &self.mode
    }
    ///0x808 - RSA clean register
    #[inline(always)]
    pub const fn query_clean(&self) -> &QUERY_CLEAN {
        &self.query_clean
    }
    ///0x80c - Starts modular exponentiation
    #[inline(always)]
    pub const fn set_start_modexp(&self) -> &SET_START_MODEXP {
        &self.set_start_modexp
    }
    ///0x810 - Starts modular multiplication
    #[inline(always)]
    pub const fn set_start_modmult(&self) -> &SET_START_MODMULT {
        &self.set_start_modmult
    }
    ///0x814 - Starts multiplication
    #[inline(always)]
    pub const fn set_start_mult(&self) -> &SET_START_MULT {
        &self.set_start_mult
    }
    ///0x818 - Represents the RSA status
    #[inline(always)]
    pub const fn query_idle(&self) -> &QUERY_IDLE {
        &self.query_idle
    }
    ///0x81c - Clears RSA interrupt
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x820 - Configures the constant_time option
    #[inline(always)]
    pub const fn constant_time(&self) -> &CONSTANT_TIME {
        &self.constant_time
    }
    ///0x824 - Configures the search option
    #[inline(always)]
    pub const fn search_enable(&self) -> &SEARCH_ENABLE {
        &self.search_enable
    }
    ///0x828 - Configures the search position
    #[inline(always)]
    pub const fn search_pos(&self) -> &SEARCH_POS {
        &self.search_pos
    }
    ///0x82c - Enables the RSA interrupt
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x830 - Version control register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**M_MEM (rw) register accessor: Represents M

You can [`read`](crate::generic::Reg::read) this register and get [`m_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@m_mem`] module*/
pub type M_MEM = crate::Reg<m_mem::M_MEM_SPEC>;
///Represents M
pub mod m_mem;
/**Z_MEM (rw) register accessor: Represents Z

You can [`read`](crate::generic::Reg::read) this register and get [`z_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`z_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@z_mem`] module*/
pub type Z_MEM = crate::Reg<z_mem::Z_MEM_SPEC>;
///Represents Z
pub mod z_mem;
/**Y_MEM (rw) register accessor: Represents Y

You can [`read`](crate::generic::Reg::read) this register and get [`y_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`y_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@y_mem`] module*/
pub type Y_MEM = crate::Reg<y_mem::Y_MEM_SPEC>;
///Represents Y
pub mod y_mem;
/**X_MEM (rw) register accessor: Represents X

You can [`read`](crate::generic::Reg::read) this register and get [`x_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`x_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@x_mem`] module*/
pub type X_MEM = crate::Reg<x_mem::X_MEM_SPEC>;
///Represents X
pub mod x_mem;
/**M_PRIME (rw) register accessor: Represents M’

You can [`read`](crate::generic::Reg::read) this register and get [`m_prime::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_prime::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@m_prime`] module*/
pub type M_PRIME = crate::Reg<m_prime::M_PRIME_SPEC>;
///Represents M’
pub mod m_prime;
/**MODE (rw) register accessor: Configures RSA length

You can [`read`](crate::generic::Reg::read) this register and get [`mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mode`] module*/
pub type MODE = crate::Reg<mode::MODE_SPEC>;
///Configures RSA length
pub mod mode;
/**QUERY_CLEAN (r) register accessor: RSA clean register

You can [`read`](crate::generic::Reg::read) this register and get [`query_clean::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@query_clean`] module*/
pub type QUERY_CLEAN = crate::Reg<query_clean::QUERY_CLEAN_SPEC>;
///RSA clean register
pub mod query_clean;
/**SET_START_MODEXP (w) register accessor: Starts modular exponentiation

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_start_modexp::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@set_start_modexp`] module*/
pub type SET_START_MODEXP = crate::Reg<set_start_modexp::SET_START_MODEXP_SPEC>;
///Starts modular exponentiation
pub mod set_start_modexp;
/**SET_START_MODMULT (w) register accessor: Starts modular multiplication

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_start_modmult::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@set_start_modmult`] module*/
pub type SET_START_MODMULT = crate::Reg<set_start_modmult::SET_START_MODMULT_SPEC>;
///Starts modular multiplication
pub mod set_start_modmult;
/**SET_START_MULT (w) register accessor: Starts multiplication

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_start_mult::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@set_start_mult`] module*/
pub type SET_START_MULT = crate::Reg<set_start_mult::SET_START_MULT_SPEC>;
///Starts multiplication
pub mod set_start_mult;
/**QUERY_IDLE (r) register accessor: Represents the RSA status

You can [`read`](crate::generic::Reg::read) this register and get [`query_idle::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@query_idle`] module*/
pub type QUERY_IDLE = crate::Reg<query_idle::QUERY_IDLE_SPEC>;
///Represents the RSA status
pub mod query_idle;
/**INT_CLR (w) register accessor: Clears RSA interrupt

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///Clears RSA interrupt
pub mod int_clr;
/**CONSTANT_TIME (rw) register accessor: Configures the constant_time option

You can [`read`](crate::generic::Reg::read) this register and get [`constant_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`constant_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@constant_time`] module*/
pub type CONSTANT_TIME = crate::Reg<constant_time::CONSTANT_TIME_SPEC>;
///Configures the constant_time option
pub mod constant_time;
/**SEARCH_ENABLE (rw) register accessor: Configures the search option

You can [`read`](crate::generic::Reg::read) this register and get [`search_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`search_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@search_enable`] module*/
pub type SEARCH_ENABLE = crate::Reg<search_enable::SEARCH_ENABLE_SPEC>;
///Configures the search option
pub mod search_enable;
/**SEARCH_POS (rw) register accessor: Configures the search position

You can [`read`](crate::generic::Reg::read) this register and get [`search_pos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`search_pos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@search_pos`] module*/
pub type SEARCH_POS = crate::Reg<search_pos::SEARCH_POS_SPEC>;
///Configures the search position
pub mod search_pos;
/**INT_ENA (rw) register accessor: Enables the RSA interrupt

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///Enables the RSA interrupt
pub mod int_ena;
/**DATE (rw) register accessor: Version control register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///Version control register
pub mod date;
