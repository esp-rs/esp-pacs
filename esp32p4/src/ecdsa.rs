#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    conf: CONF,
    clk: CLK,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    start: START,
    state: STATE,
    result: RESULT,
    _reserved9: [u8; 0xd4],
    date: DATE,
    _reserved10: [u8; 0x0100],
    sha_mode: SHA_MODE,
    _reserved11: [u8; 0x0c],
    sha_start: SHA_START,
    sha_continue: SHA_CONTINUE,
    sha_busy: SHA_BUSY,
    _reserved14: [u8; 0x64],
    message_mem: [MESSAGE_MEM; 8],
    _reserved15: [u8; 0x0760],
    r_mem: [R_MEM; 8],
    s_mem: [S_MEM; 8],
    z_mem: [Z_MEM; 8],
    qax_mem: [QAX_MEM; 8],
    qay_mem: [QAY_MEM; 8],
}
impl RegisterBlock {
    ///0x04 - ECDSA configure register
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    ///0x08 - ECDSA clock gate register
    #[inline(always)]
    pub const fn clk(&self) -> &CLK {
        &self.clk
    }
    ///0x0c - ECDSA interrupt raw register, valid in level.
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x10 - ECDSA interrupt status register.
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x14 - ECDSA interrupt enable register.
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x18 - ECDSA interrupt clear register.
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x1c - ECDSA start register
    #[inline(always)]
    pub const fn start(&self) -> &START {
        &self.start
    }
    ///0x20 - ECDSA status register
    #[inline(always)]
    pub const fn state(&self) -> &STATE {
        &self.state
    }
    ///0x24 - ECDSA result register
    #[inline(always)]
    pub const fn result(&self) -> &RESULT {
        &self.result
    }
    ///0xfc - Version control register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    ///0x200 - ECDSA control SHA register
    #[inline(always)]
    pub const fn sha_mode(&self) -> &SHA_MODE {
        &self.sha_mode
    }
    ///0x210 - ECDSA control SHA register
    #[inline(always)]
    pub const fn sha_start(&self) -> &SHA_START {
        &self.sha_start
    }
    ///0x214 - ECDSA control SHA register
    #[inline(always)]
    pub const fn sha_continue(&self) -> &SHA_CONTINUE {
        &self.sha_continue
    }
    ///0x218 - ECDSA status register
    #[inline(always)]
    pub const fn sha_busy(&self) -> &SHA_BUSY {
        &self.sha_busy
    }
    ///0x280..0x2a0 - The memory that stores message.
    #[inline(always)]
    pub const fn message_mem(&self, n: usize) -> &MESSAGE_MEM {
        &self.message_mem[n]
    }
    ///Iterator for array of:
    ///0x280..0x2a0 - The memory that stores message.
    #[inline(always)]
    pub fn message_mem_iter(&self) -> impl Iterator<Item = &MESSAGE_MEM> {
        self.message_mem.iter()
    }
    ///0xa00..0xa20 - The memory that stores r.
    #[inline(always)]
    pub const fn r_mem(&self, n: usize) -> &R_MEM {
        &self.r_mem[n]
    }
    ///Iterator for array of:
    ///0xa00..0xa20 - The memory that stores r.
    #[inline(always)]
    pub fn r_mem_iter(&self) -> impl Iterator<Item = &R_MEM> {
        self.r_mem.iter()
    }
    ///0xa20..0xa40 - The memory that stores s.
    #[inline(always)]
    pub const fn s_mem(&self, n: usize) -> &S_MEM {
        &self.s_mem[n]
    }
    ///Iterator for array of:
    ///0xa20..0xa40 - The memory that stores s.
    #[inline(always)]
    pub fn s_mem_iter(&self) -> impl Iterator<Item = &S_MEM> {
        self.s_mem.iter()
    }
    ///0xa40..0xa60 - The memory that stores software written z.
    #[inline(always)]
    pub const fn z_mem(&self, n: usize) -> &Z_MEM {
        &self.z_mem[n]
    }
    ///Iterator for array of:
    ///0xa40..0xa60 - The memory that stores software written z.
    #[inline(always)]
    pub fn z_mem_iter(&self) -> impl Iterator<Item = &Z_MEM> {
        self.z_mem.iter()
    }
    ///0xa60..0xa80 - The memory that stores x coordinates of QA or software written k.
    #[inline(always)]
    pub const fn qax_mem(&self, n: usize) -> &QAX_MEM {
        &self.qax_mem[n]
    }
    ///Iterator for array of:
    ///0xa60..0xa80 - The memory that stores x coordinates of QA or software written k.
    #[inline(always)]
    pub fn qax_mem_iter(&self) -> impl Iterator<Item = &QAX_MEM> {
        self.qax_mem.iter()
    }
    ///0xa80..0xaa0 - The memory that stores y coordinates of QA.
    #[inline(always)]
    pub const fn qay_mem(&self, n: usize) -> &QAY_MEM {
        &self.qay_mem[n]
    }
    ///Iterator for array of:
    ///0xa80..0xaa0 - The memory that stores y coordinates of QA.
    #[inline(always)]
    pub fn qay_mem_iter(&self) -> impl Iterator<Item = &QAY_MEM> {
        self.qay_mem.iter()
    }
}
/**CONF (rw) register accessor: ECDSA configure register

You can [`read`](crate::generic::Reg::read) this register and get [`conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@conf`] module*/
pub type CONF = crate::Reg<conf::CONF_SPEC>;
///ECDSA configure register
pub mod conf;
/**CLK (rw) register accessor: ECDSA clock gate register

You can [`read`](crate::generic::Reg::read) this register and get [`clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clk`] module*/
pub type CLK = crate::Reg<clk::CLK_SPEC>;
///ECDSA clock gate register
pub mod clk;
/**INT_RAW (r) register accessor: ECDSA interrupt raw register, valid in level.

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///ECDSA interrupt raw register, valid in level.
pub mod int_raw;
/**INT_ST (r) register accessor: ECDSA interrupt status register.

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///ECDSA interrupt status register.
pub mod int_st;
/**INT_ENA (rw) register accessor: ECDSA interrupt enable register.

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///ECDSA interrupt enable register.
pub mod int_ena;
/**INT_CLR (w) register accessor: ECDSA interrupt clear register.

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///ECDSA interrupt clear register.
pub mod int_clr;
/**START (w) register accessor: ECDSA start register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@start`] module*/
pub type START = crate::Reg<start::START_SPEC>;
///ECDSA start register
pub mod start;
/**STATE (r) register accessor: ECDSA status register

You can [`read`](crate::generic::Reg::read) this register and get [`state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@state`] module*/
pub type STATE = crate::Reg<state::STATE_SPEC>;
///ECDSA status register
pub mod state;
/**RESULT (r) register accessor: ECDSA result register

You can [`read`](crate::generic::Reg::read) this register and get [`result::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@result`] module*/
pub type RESULT = crate::Reg<result::RESULT_SPEC>;
///ECDSA result register
pub mod result;
/**DATE (rw) register accessor: Version control register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///Version control register
pub mod date;
/**SHA_MODE (rw) register accessor: ECDSA control SHA register

You can [`read`](crate::generic::Reg::read) this register and get [`sha_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sha_mode`] module*/
pub type SHA_MODE = crate::Reg<sha_mode::SHA_MODE_SPEC>;
///ECDSA control SHA register
pub mod sha_mode;
/**SHA_START (w) register accessor: ECDSA control SHA register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sha_start`] module*/
pub type SHA_START = crate::Reg<sha_start::SHA_START_SPEC>;
///ECDSA control SHA register
pub mod sha_start;
/**SHA_CONTINUE (w) register accessor: ECDSA control SHA register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha_continue::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sha_continue`] module*/
pub type SHA_CONTINUE = crate::Reg<sha_continue::SHA_CONTINUE_SPEC>;
///ECDSA control SHA register
pub mod sha_continue;
/**SHA_BUSY (r) register accessor: ECDSA status register

You can [`read`](crate::generic::Reg::read) this register and get [`sha_busy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sha_busy`] module*/
pub type SHA_BUSY = crate::Reg<sha_busy::SHA_BUSY_SPEC>;
///ECDSA status register
pub mod sha_busy;
/**MESSAGE_MEM (rw) register accessor: The memory that stores message.

You can [`read`](crate::generic::Reg::read) this register and get [`message_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`message_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@message_mem`] module*/
pub type MESSAGE_MEM = crate::Reg<message_mem::MESSAGE_MEM_SPEC>;
///The memory that stores message.
pub mod message_mem;
/**R_MEM (rw) register accessor: The memory that stores r.

You can [`read`](crate::generic::Reg::read) this register and get [`r_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@r_mem`] module*/
pub type R_MEM = crate::Reg<r_mem::R_MEM_SPEC>;
///The memory that stores r.
pub mod r_mem;
/**S_MEM (rw) register accessor: The memory that stores s.

You can [`read`](crate::generic::Reg::read) this register and get [`s_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@s_mem`] module*/
pub type S_MEM = crate::Reg<s_mem::S_MEM_SPEC>;
///The memory that stores s.
pub mod s_mem;
/**Z_MEM (rw) register accessor: The memory that stores software written z.

You can [`read`](crate::generic::Reg::read) this register and get [`z_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`z_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@z_mem`] module*/
pub type Z_MEM = crate::Reg<z_mem::Z_MEM_SPEC>;
///The memory that stores software written z.
pub mod z_mem;
/**QAX_MEM (rw) register accessor: The memory that stores x coordinates of QA or software written k.

You can [`read`](crate::generic::Reg::read) this register and get [`qax_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qax_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@qax_mem`] module*/
pub type QAX_MEM = crate::Reg<qax_mem::QAX_MEM_SPEC>;
///The memory that stores x coordinates of QA or software written k.
pub mod qax_mem;
/**QAY_MEM (rw) register accessor: The memory that stores y coordinates of QA.

You can [`read`](crate::generic::Reg::read) this register and get [`qay_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qay_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@qay_mem`] module*/
pub type QAY_MEM = crate::Reg<qay_mem::QAY_MEM_SPEC>;
///The memory that stores y coordinates of QA.
pub mod qay_mem;
