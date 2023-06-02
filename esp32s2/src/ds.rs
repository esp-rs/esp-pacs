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
#[doc = "C_MEM (rw) register accessor: an alias for `Reg<C_MEM_SPEC>`"]
pub type C_MEM = crate::Reg<c_mem::C_MEM_SPEC>;
#[doc = "memory C"]
pub mod c_mem;
#[doc = "IV_ (w) register accessor: an alias for `Reg<IV__SPEC>`"]
pub type IV_ = crate::Reg<iv_::IV__SPEC>;
#[doc = "IV block data."]
pub mod iv_;
#[doc = "X_MEM (rw) register accessor: an alias for `Reg<X_MEM_SPEC>`"]
pub type X_MEM = crate::Reg<x_mem::X_MEM_SPEC>;
#[doc = "memory X"]
pub mod x_mem;
#[doc = "Z_MEM (rw) register accessor: an alias for `Reg<Z_MEM_SPEC>`"]
pub type Z_MEM = crate::Reg<z_mem::Z_MEM_SPEC>;
#[doc = "memory Z"]
pub mod z_mem;
#[doc = "SET_START (w) register accessor: an alias for `Reg<SET_START_SPEC>`"]
pub type SET_START = crate::Reg<set_start::SET_START_SPEC>;
#[doc = "Activates the DS peripheral"]
pub mod set_start;
#[doc = "SET_ME (w) register accessor: an alias for `Reg<SET_ME_SPEC>`"]
pub type SET_ME = crate::Reg<set_me::SET_ME_SPEC>;
#[doc = "Starts DS operation"]
pub mod set_me;
#[doc = "SET_FINISH (w) register accessor: an alias for `Reg<SET_FINISH_SPEC>`"]
pub type SET_FINISH = crate::Reg<set_finish::SET_FINISH_SPEC>;
#[doc = "Ends DS operation"]
pub mod set_finish;
#[doc = "QUERY_BUSY (r) register accessor: an alias for `Reg<QUERY_BUSY_SPEC>`"]
pub type QUERY_BUSY = crate::Reg<query_busy::QUERY_BUSY_SPEC>;
#[doc = "Status of the DS"]
pub mod query_busy;
#[doc = "QUERY_KEY_WRONG (r) register accessor: an alias for `Reg<QUERY_KEY_WRONG_SPEC>`"]
pub type QUERY_KEY_WRONG = crate::Reg<query_key_wrong::QUERY_KEY_WRONG_SPEC>;
#[doc = "Checks the reason why DS_KEY is not ready."]
pub mod query_key_wrong;
#[doc = "QUERY_CHECK (r) register accessor: an alias for `Reg<QUERY_CHECK_SPEC>`"]
pub type QUERY_CHECK = crate::Reg<query_check::QUERY_CHECK_SPEC>;
#[doc = "Queries DS check result"]
pub mod query_check;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
