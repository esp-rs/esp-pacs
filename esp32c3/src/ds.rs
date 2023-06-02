#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00..0x200 - memory that stores Y"]
    pub y_mem: [Y_MEM; 512],
    #[doc = "0x200..0x400 - memory that stores M"]
    pub m_mem: [M_MEM; 512],
    #[doc = "0x400..0x600 - memory that stores Rb"]
    pub rb_mem: [RB_MEM; 512],
    #[doc = "0x600..0x630 - memory that stores BOX"]
    pub box_mem: [BOX_MEM; 48],
    _reserved4: [u8; 0x01d0],
    #[doc = "0x800..0xa00 - memory that stores X"]
    pub x_mem: [X_MEM; 512],
    #[doc = "0xa00..0xc00 - memory that stores Z"]
    pub z_mem: [Z_MEM; 512],
    _reserved6: [u8; 0x0200],
    #[doc = "0xe00 - DS start control register"]
    pub set_start: SET_START,
    #[doc = "0xe04 - DS continue control register"]
    pub set_continue: SET_CONTINUE,
    #[doc = "0xe08 - DS finish control register"]
    pub set_finish: SET_FINISH,
    #[doc = "0xe0c - DS query busy register"]
    pub query_busy: QUERY_BUSY,
    #[doc = "0xe10 - DS query key-wrong counter register"]
    pub query_key_wrong: QUERY_KEY_WRONG,
    #[doc = "0xe14 - DS query check result register"]
    pub query_check: QUERY_CHECK,
    _reserved12: [u8; 0x08],
    #[doc = "0xe20 - DS version control register"]
    pub date: DATE,
}
#[doc = "Y_MEM (rw) register accessor: an alias for `Reg<Y_MEM_SPEC>`"]
pub type Y_MEM = crate::Reg<y_mem::Y_MEM_SPEC>;
#[doc = "memory that stores Y"]
pub mod y_mem;
#[doc = "M_MEM (rw) register accessor: an alias for `Reg<M_MEM_SPEC>`"]
pub type M_MEM = crate::Reg<m_mem::M_MEM_SPEC>;
#[doc = "memory that stores M"]
pub mod m_mem;
#[doc = "RB_MEM (rw) register accessor: an alias for `Reg<RB_MEM_SPEC>`"]
pub type RB_MEM = crate::Reg<rb_mem::RB_MEM_SPEC>;
#[doc = "memory that stores Rb"]
pub mod rb_mem;
#[doc = "BOX_MEM (rw) register accessor: an alias for `Reg<BOX_MEM_SPEC>`"]
pub type BOX_MEM = crate::Reg<box_mem::BOX_MEM_SPEC>;
#[doc = "memory that stores BOX"]
pub mod box_mem;
#[doc = "X_MEM (rw) register accessor: an alias for `Reg<X_MEM_SPEC>`"]
pub type X_MEM = crate::Reg<x_mem::X_MEM_SPEC>;
#[doc = "memory that stores X"]
pub mod x_mem;
#[doc = "Z_MEM (rw) register accessor: an alias for `Reg<Z_MEM_SPEC>`"]
pub type Z_MEM = crate::Reg<z_mem::Z_MEM_SPEC>;
#[doc = "memory that stores Z"]
pub mod z_mem;
#[doc = "SET_START (w) register accessor: an alias for `Reg<SET_START_SPEC>`"]
pub type SET_START = crate::Reg<set_start::SET_START_SPEC>;
#[doc = "DS start control register"]
pub mod set_start;
#[doc = "SET_CONTINUE (w) register accessor: an alias for `Reg<SET_CONTINUE_SPEC>`"]
pub type SET_CONTINUE = crate::Reg<set_continue::SET_CONTINUE_SPEC>;
#[doc = "DS continue control register"]
pub mod set_continue;
#[doc = "SET_FINISH (w) register accessor: an alias for `Reg<SET_FINISH_SPEC>`"]
pub type SET_FINISH = crate::Reg<set_finish::SET_FINISH_SPEC>;
#[doc = "DS finish control register"]
pub mod set_finish;
#[doc = "QUERY_BUSY (r) register accessor: an alias for `Reg<QUERY_BUSY_SPEC>`"]
pub type QUERY_BUSY = crate::Reg<query_busy::QUERY_BUSY_SPEC>;
#[doc = "DS query busy register"]
pub mod query_busy;
#[doc = "QUERY_KEY_WRONG (r) register accessor: an alias for `Reg<QUERY_KEY_WRONG_SPEC>`"]
pub type QUERY_KEY_WRONG = crate::Reg<query_key_wrong::QUERY_KEY_WRONG_SPEC>;
#[doc = "DS query key-wrong counter register"]
pub mod query_key_wrong;
#[doc = "QUERY_CHECK (r) register accessor: an alias for `Reg<QUERY_CHECK_SPEC>`"]
pub type QUERY_CHECK = crate::Reg<query_check::QUERY_CHECK_SPEC>;
#[doc = "DS query check result register"]
pub mod query_check;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "DS version control register"]
pub mod date;
