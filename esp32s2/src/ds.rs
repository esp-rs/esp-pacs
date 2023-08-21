#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00..0x630 - memory C"]
    pub c_mem: [C_MEM; 1584],
    #[doc = "0x630..0x640 - IV block data."]
    pub iv_: [IV_; 4],
    _reserved2: [u8; 0x01c0],
    #[doc = "0x800..0xa00 - memory X"]
    pub x_mem: [X_MEM; 512],
    #[doc = "0xa00..0xc00 - memory Z"]
    pub z_mem: [Z_MEM; 512],
    _reserved4: [u8; 0x0200],
    #[doc = "0xe00 - Activates the DS peripheral"]
    pub set_start: SET_START,
    #[doc = "0xe04 - Starts DS operation"]
    pub set_me: SET_ME,
    #[doc = "0xe08 - Ends DS operation"]
    pub set_finish: SET_FINISH,
    #[doc = "0xe0c - Status of the DS"]
    pub query_busy: QUERY_BUSY,
    #[doc = "0xe10 - Checks the reason why DS_KEY is not ready."]
    pub query_key_wrong: QUERY_KEY_WRONG,
    #[doc = "0xe14 - Queries DS check result"]
    pub query_check: QUERY_CHECK,
    _reserved10: [u8; 0x08],
    #[doc = "0xe20 - Version control register"]
    pub date: DATE,
}
#[doc = "C_MEM (rw) register accessor: memory C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c_mem`] module"]
pub type C_MEM = crate::Reg<c_mem::C_MEM_SPEC>;
#[doc = "memory C"]
pub mod c_mem;
#[doc = "IV_ (w) register accessor: IV block data.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv_::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`iv_`] module"]
pub type IV_ = crate::Reg<iv_::IV__SPEC>;
#[doc = "IV block data."]
pub mod iv_;
#[doc = "X_MEM (rw) register accessor: memory X\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`x_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`x_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`x_mem`] module"]
pub type X_MEM = crate::Reg<x_mem::X_MEM_SPEC>;
#[doc = "memory X"]
pub mod x_mem;
#[doc = "Z_MEM (rw) register accessor: memory Z\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`z_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`z_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`z_mem`] module"]
pub type Z_MEM = crate::Reg<z_mem::Z_MEM_SPEC>;
#[doc = "memory Z"]
pub mod z_mem;
#[doc = "SET_START (w) register accessor: Activates the DS peripheral\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`set_start`] module"]
pub type SET_START = crate::Reg<set_start::SET_START_SPEC>;
#[doc = "Activates the DS peripheral"]
pub mod set_start;
#[doc = "SET_ME (w) register accessor: Starts DS operation\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_me::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`set_me`] module"]
pub type SET_ME = crate::Reg<set_me::SET_ME_SPEC>;
#[doc = "Starts DS operation"]
pub mod set_me;
#[doc = "SET_FINISH (w) register accessor: Ends DS operation\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_finish::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`set_finish`] module"]
pub type SET_FINISH = crate::Reg<set_finish::SET_FINISH_SPEC>;
#[doc = "Ends DS operation"]
pub mod set_finish;
#[doc = "QUERY_BUSY (r) register accessor: Status of the DS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`query_busy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`query_busy`] module"]
pub type QUERY_BUSY = crate::Reg<query_busy::QUERY_BUSY_SPEC>;
#[doc = "Status of the DS"]
pub mod query_busy;
#[doc = "QUERY_KEY_WRONG (r) register accessor: Checks the reason why DS_KEY is not ready.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`query_key_wrong::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`query_key_wrong`] module"]
pub type QUERY_KEY_WRONG = crate::Reg<query_key_wrong::QUERY_KEY_WRONG_SPEC>;
#[doc = "Checks the reason why DS_KEY is not ready."]
pub mod query_key_wrong;
#[doc = "QUERY_CHECK (r) register accessor: Queries DS check result\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`query_check::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`query_check`] module"]
pub type QUERY_CHECK = crate::Reg<query_check::QUERY_CHECK_SPEC>;
#[doc = "Queries DS check result"]
pub mod query_check;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
