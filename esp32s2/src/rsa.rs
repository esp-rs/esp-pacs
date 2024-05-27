#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    m_mem: [M_MEM; 128],
    z_mem: [Z_MEM; 128],
    y_mem: [Y_MEM; 128],
    x_mem: [X_MEM; 128],
    m_prime: M_PRIME,
    mode: MODE,
    clean: CLEAN,
    modexp_start: MODEXP_START,
    modmult_start: MODMULT_START,
    mult_start: MULT_START,
    idle: IDLE,
    clear_interrupt: CLEAR_INTERRUPT,
    constant_time: CONSTANT_TIME,
    search_enable: SEARCH_ENABLE,
    search_pos: SEARCH_POS,
    interrupt_ena: INTERRUPT_ENA,
    date: DATE,
}
impl RegisterBlock {
    ///0x00..0x200 - Represents M
    #[inline(always)]
    pub const fn m_mem(&self, n: usize) -> &M_MEM {
        &self.m_mem[n]
    }
    ///Iterator for array of:
    ///0x00..0x200 - Represents M
    #[inline(always)]
    pub fn m_mem_iter(&self) -> impl Iterator<Item = &M_MEM> {
        self.m_mem.iter()
    }
    ///0x200..0x400 - Represents Z
    #[inline(always)]
    pub const fn z_mem(&self, n: usize) -> &Z_MEM {
        &self.z_mem[n]
    }
    ///Iterator for array of:
    ///0x200..0x400 - Represents Z
    #[inline(always)]
    pub fn z_mem_iter(&self) -> impl Iterator<Item = &Z_MEM> {
        self.z_mem.iter()
    }
    ///0x400..0x600 - Represents Y
    #[inline(always)]
    pub const fn y_mem(&self, n: usize) -> &Y_MEM {
        &self.y_mem[n]
    }
    ///Iterator for array of:
    ///0x400..0x600 - Represents Y
    #[inline(always)]
    pub fn y_mem_iter(&self) -> impl Iterator<Item = &Y_MEM> {
        self.y_mem.iter()
    }
    ///0x600..0x800 - Represents X
    #[inline(always)]
    pub const fn x_mem(&self, n: usize) -> &X_MEM {
        &self.x_mem[n]
    }
    ///Iterator for array of:
    ///0x600..0x800 - Represents X
    #[inline(always)]
    pub fn x_mem_iter(&self) -> impl Iterator<Item = &X_MEM> {
        self.x_mem.iter()
    }
    ///0x800 - Register to store M'
    #[inline(always)]
    pub const fn m_prime(&self) -> &M_PRIME {
        &self.m_prime
    }
    ///0x804 - RSA length mode
    #[inline(always)]
    pub const fn mode(&self) -> &MODE {
        &self.mode
    }
    ///0x808 - RSA clean register
    #[inline(always)]
    pub const fn clean(&self) -> &CLEAN {
        &self.clean
    }
    ///0x80c - Modular exponentiation starting bit
    #[inline(always)]
    pub const fn modexp_start(&self) -> &MODEXP_START {
        &self.modexp_start
    }
    ///0x810 - Modular multiplication starting bit
    #[inline(always)]
    pub const fn modmult_start(&self) -> &MODMULT_START {
        &self.modmult_start
    }
    ///0x814 - Normal multiplication starting bit
    #[inline(always)]
    pub const fn mult_start(&self) -> &MULT_START {
        &self.mult_start
    }
    ///0x818 - RSA idle register
    #[inline(always)]
    pub const fn idle(&self) -> &IDLE {
        &self.idle
    }
    ///0x81c - RSA clear interrupt register
    #[inline(always)]
    pub const fn clear_interrupt(&self) -> &CLEAR_INTERRUPT {
        &self.clear_interrupt
    }
    ///0x820 - The constant_time option
    #[inline(always)]
    pub const fn constant_time(&self) -> &CONSTANT_TIME {
        &self.constant_time
    }
    ///0x824 - The search option
    #[inline(always)]
    pub const fn search_enable(&self) -> &SEARCH_ENABLE {
        &self.search_enable
    }
    ///0x828 - The search position
    #[inline(always)]
    pub const fn search_pos(&self) -> &SEARCH_POS {
        &self.search_pos
    }
    ///0x82c - RSA interrupt enable register
    #[inline(always)]
    pub const fn interrupt_ena(&self) -> &INTERRUPT_ENA {
        &self.interrupt_ena
    }
    ///0x830 - Version control register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**M_PRIME (rw) register accessor: Register to store M'

You can [`read`](crate::generic::Reg::read) this register and get [`m_prime::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_prime::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@m_prime`] module*/
pub type M_PRIME = crate::Reg<m_prime::M_PRIME_SPEC>;
///Register to store M'
pub mod m_prime;
/**MODE (rw) register accessor: RSA length mode

You can [`read`](crate::generic::Reg::read) this register and get [`mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mode`] module*/
pub type MODE = crate::Reg<mode::MODE_SPEC>;
///RSA length mode
pub mod mode;
/**CLEAN (r) register accessor: RSA clean register

You can [`read`](crate::generic::Reg::read) this register and get [`clean::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clean`] module*/
pub type CLEAN = crate::Reg<clean::CLEAN_SPEC>;
///RSA clean register
pub mod clean;
/**MODEXP_START (w) register accessor: Modular exponentiation starting bit

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modexp_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@modexp_start`] module*/
pub type MODEXP_START = crate::Reg<modexp_start::MODEXP_START_SPEC>;
///Modular exponentiation starting bit
pub mod modexp_start;
/**MODMULT_START (w) register accessor: Modular multiplication starting bit

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modmult_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@modmult_start`] module*/
pub type MODMULT_START = crate::Reg<modmult_start::MODMULT_START_SPEC>;
///Modular multiplication starting bit
pub mod modmult_start;
/**MULT_START (w) register accessor: Normal multiplication starting bit

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mult_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mult_start`] module*/
pub type MULT_START = crate::Reg<mult_start::MULT_START_SPEC>;
///Normal multiplication starting bit
pub mod mult_start;
/**IDLE (r) register accessor: RSA idle register

You can [`read`](crate::generic::Reg::read) this register and get [`idle::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@idle`] module*/
pub type IDLE = crate::Reg<idle::IDLE_SPEC>;
///RSA idle register
pub mod idle;
/**CLEAR_INTERRUPT (w) register accessor: RSA clear interrupt register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clear_interrupt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clear_interrupt`] module*/
pub type CLEAR_INTERRUPT = crate::Reg<clear_interrupt::CLEAR_INTERRUPT_SPEC>;
///RSA clear interrupt register
pub mod clear_interrupt;
/**CONSTANT_TIME (rw) register accessor: The constant_time option

You can [`read`](crate::generic::Reg::read) this register and get [`constant_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`constant_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@constant_time`] module*/
pub type CONSTANT_TIME = crate::Reg<constant_time::CONSTANT_TIME_SPEC>;
///The constant_time option
pub mod constant_time;
/**SEARCH_ENABLE (rw) register accessor: The search option

You can [`read`](crate::generic::Reg::read) this register and get [`search_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`search_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@search_enable`] module*/
pub type SEARCH_ENABLE = crate::Reg<search_enable::SEARCH_ENABLE_SPEC>;
///The search option
pub mod search_enable;
/**SEARCH_POS (rw) register accessor: The search position

You can [`read`](crate::generic::Reg::read) this register and get [`search_pos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`search_pos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@search_pos`] module*/
pub type SEARCH_POS = crate::Reg<search_pos::SEARCH_POS_SPEC>;
///The search position
pub mod search_pos;
/**INTERRUPT_ENA (rw) register accessor: RSA interrupt enable register

You can [`read`](crate::generic::Reg::read) this register and get [`interrupt_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@interrupt_ena`] module*/
pub type INTERRUPT_ENA = crate::Reg<interrupt_ena::INTERRUPT_ENA_SPEC>;
///RSA interrupt enable register
pub mod interrupt_ena;
/**DATE (rw) register accessor: Version control register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///Version control register
pub mod date;
/**M_MEM (w) register accessor: Represents M

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_mem::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

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
/**Y_MEM (w) register accessor: Represents Y

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`y_mem::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@y_mem`] module*/
pub type Y_MEM = crate::Reg<y_mem::Y_MEM_SPEC>;
///Represents Y
pub mod y_mem;
/**X_MEM (w) register accessor: Represents X

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`x_mem::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@x_mem`] module*/
pub type X_MEM = crate::Reg<x_mem::X_MEM_SPEC>;
///Represents X
pub mod x_mem;
