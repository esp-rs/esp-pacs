#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    c_mem: [C_MEM; 396],
    iv_: [IV_; 4],
    _reserved2: [u8; 0x01c0],
    x_mem: [X_MEM; 128],
    z_mem: [Z_MEM; 128],
    _reserved4: [u8; 0x0200],
    set_start: SET_START,
    set_me: SET_ME,
    set_finish: SET_FINISH,
    query_busy: QUERY_BUSY,
    query_key_wrong: QUERY_KEY_WRONG,
    query_check: QUERY_CHECK,
    _reserved10: [u8; 0x08],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x630 - Memory C"]
    #[inline(always)]
    pub const fn c_mem(&self, n: usize) -> &C_MEM {
        &self.c_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x630 - Memory C"]
    #[inline(always)]
    pub fn c_mem_iter(&self) -> impl Iterator<Item = &C_MEM> {
        self.c_mem.iter()
    }
    #[doc = "0x630..0x640 - IV block data"]
    #[inline(always)]
    pub const fn iv_(&self, n: usize) -> &IV_ {
        &self.iv_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x630..0x640 - IV block data"]
    #[inline(always)]
    pub fn iv__iter(&self) -> impl Iterator<Item = &IV_> {
        self.iv_.iter()
    }
    #[doc = "0x800..0xa00 - Memory X"]
    #[inline(always)]
    pub const fn x_mem(&self, n: usize) -> &X_MEM {
        &self.x_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0xa00 - Memory X"]
    #[inline(always)]
    pub fn x_mem_iter(&self) -> impl Iterator<Item = &X_MEM> {
        self.x_mem.iter()
    }
    #[doc = "0xa00..0xc00 - Memory Z"]
    #[inline(always)]
    pub const fn z_mem(&self, n: usize) -> &Z_MEM {
        &self.z_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa00..0xc00 - Memory Z"]
    #[inline(always)]
    pub fn z_mem_iter(&self) -> impl Iterator<Item = &Z_MEM> {
        self.z_mem.iter()
    }
    #[doc = "0xe00 - Activates the DS peripheral"]
    #[inline(always)]
    pub const fn set_start(&self) -> &SET_START {
        &self.set_start
    }
    #[doc = "0xe04 - Starts DS operation"]
    #[inline(always)]
    pub const fn set_me(&self) -> &SET_ME {
        &self.set_me
    }
    #[doc = "0xe08 - Ends DS operation"]
    #[inline(always)]
    pub const fn set_finish(&self) -> &SET_FINISH {
        &self.set_finish
    }
    #[doc = "0xe0c - Status of the DS perihperal"]
    #[inline(always)]
    pub const fn query_busy(&self) -> &QUERY_BUSY {
        &self.query_busy
    }
    #[doc = "0xe10 - Checks the reason why DS_KEY is not ready"]
    #[inline(always)]
    pub const fn query_key_wrong(&self) -> &QUERY_KEY_WRONG {
        &self.query_key_wrong
    }
    #[doc = "0xe14 - Queries DS check result"]
    #[inline(always)]
    pub const fn query_check(&self) -> &QUERY_CHECK {
        &self.query_check
    }
    #[doc = "0xe20 - DS version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "C_MEM (rw) register accessor: Memory C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_mem`] module"]
pub type C_MEM = crate::Reg<c_mem::C_MEM_SPEC>;
#[doc = "Memory C"]
pub mod c_mem;
#[doc = "IV_ (rw) register accessor: IV block data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iv_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iv_`] module"]
pub type IV_ = crate::Reg<iv_::IV__SPEC>;
#[doc = "IV block data"]
pub mod iv_;
#[doc = "X_MEM (rw) register accessor: Memory X\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`x_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`x_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@x_mem`] module"]
pub type X_MEM = crate::Reg<x_mem::X_MEM_SPEC>;
#[doc = "Memory X"]
pub mod x_mem;
#[doc = "Z_MEM (rw) register accessor: Memory Z\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`z_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`z_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@z_mem`] module"]
pub type Z_MEM = crate::Reg<z_mem::Z_MEM_SPEC>;
#[doc = "Memory Z"]
pub mod z_mem;
#[doc = "SET_START (w) register accessor: Activates the DS peripheral\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set_start`] module"]
pub type SET_START = crate::Reg<set_start::SET_START_SPEC>;
#[doc = "Activates the DS peripheral"]
pub mod set_start;
#[doc = "SET_ME (w) register accessor: Starts DS operation\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_me::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set_me`] module"]
pub type SET_ME = crate::Reg<set_me::SET_ME_SPEC>;
#[doc = "Starts DS operation"]
pub mod set_me;
#[doc = "SET_FINISH (w) register accessor: Ends DS operation\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_finish::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set_finish`] module"]
pub type SET_FINISH = crate::Reg<set_finish::SET_FINISH_SPEC>;
#[doc = "Ends DS operation"]
pub mod set_finish;
#[doc = "QUERY_BUSY (r) register accessor: Status of the DS perihperal\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`query_busy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@query_busy`] module"]
pub type QUERY_BUSY = crate::Reg<query_busy::QUERY_BUSY_SPEC>;
#[doc = "Status of the DS perihperal"]
pub mod query_busy;
#[doc = "QUERY_KEY_WRONG (r) register accessor: Checks the reason why DS_KEY is not ready\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`query_key_wrong::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@query_key_wrong`] module"]
pub type QUERY_KEY_WRONG = crate::Reg<query_key_wrong::QUERY_KEY_WRONG_SPEC>;
#[doc = "Checks the reason why DS_KEY is not ready"]
pub mod query_key_wrong;
#[doc = "QUERY_CHECK (r) register accessor: Queries DS check result\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`query_check::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@query_check`] module"]
pub type QUERY_CHECK = crate::Reg<query_check::QUERY_CHECK_SPEC>;
#[doc = "Queries DS check result"]
pub mod query_check;
#[doc = "DATE (rw) register accessor: DS version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "DS version control register"]
pub mod date;
