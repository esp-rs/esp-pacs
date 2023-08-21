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
#[doc = "Y_MEM (rw) register accessor: memory that stores Y\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`y_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`y_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`y_mem`] module"]
pub type Y_MEM = crate::Reg<y_mem::Y_MEM_SPEC>;
#[doc = "memory that stores Y"]
pub mod y_mem;
#[doc = "M_MEM (rw) register accessor: memory that stores M\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m_mem`] module"]
pub type M_MEM = crate::Reg<m_mem::M_MEM_SPEC>;
#[doc = "memory that stores M"]
pub mod m_mem;
#[doc = "RB_MEM (rw) register accessor: memory that stores Rb\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rb_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rb_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rb_mem`] module"]
pub type RB_MEM = crate::Reg<rb_mem::RB_MEM_SPEC>;
#[doc = "memory that stores Rb"]
pub mod rb_mem;
#[doc = "BOX_MEM (rw) register accessor: memory that stores BOX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`box_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`box_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`box_mem`] module"]
pub type BOX_MEM = crate::Reg<box_mem::BOX_MEM_SPEC>;
#[doc = "memory that stores BOX"]
pub mod box_mem;
#[doc = "X_MEM (rw) register accessor: memory that stores X\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`x_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`x_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`x_mem`] module"]
pub type X_MEM = crate::Reg<x_mem::X_MEM_SPEC>;
#[doc = "memory that stores X"]
pub mod x_mem;
#[doc = "Z_MEM (rw) register accessor: memory that stores Z\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`z_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`z_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`z_mem`] module"]
pub type Z_MEM = crate::Reg<z_mem::Z_MEM_SPEC>;
#[doc = "memory that stores Z"]
pub mod z_mem;
#[doc = "SET_START (w) register accessor: DS start control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`set_start`] module"]
pub type SET_START = crate::Reg<set_start::SET_START_SPEC>;
#[doc = "DS start control register"]
pub mod set_start;
#[doc = "SET_CONTINUE (w) register accessor: DS continue control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_continue::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`set_continue`] module"]
pub type SET_CONTINUE = crate::Reg<set_continue::SET_CONTINUE_SPEC>;
#[doc = "DS continue control register"]
pub mod set_continue;
#[doc = "SET_FINISH (w) register accessor: DS finish control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_finish::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`set_finish`] module"]
pub type SET_FINISH = crate::Reg<set_finish::SET_FINISH_SPEC>;
#[doc = "DS finish control register"]
pub mod set_finish;
#[doc = "QUERY_BUSY (r) register accessor: DS query busy register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`query_busy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`query_busy`] module"]
pub type QUERY_BUSY = crate::Reg<query_busy::QUERY_BUSY_SPEC>;
#[doc = "DS query busy register"]
pub mod query_busy;
#[doc = "QUERY_KEY_WRONG (r) register accessor: DS query key-wrong counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`query_key_wrong::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`query_key_wrong`] module"]
pub type QUERY_KEY_WRONG = crate::Reg<query_key_wrong::QUERY_KEY_WRONG_SPEC>;
#[doc = "DS query key-wrong counter register"]
pub mod query_key_wrong;
#[doc = "QUERY_CHECK (r) register accessor: DS query check result register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`query_check::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`query_check`] module"]
pub type QUERY_CHECK = crate::Reg<query_check::QUERY_CHECK_SPEC>;
#[doc = "DS query check result register"]
pub mod query_check;
#[doc = "DATE (rw) register accessor: DS version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "DS version control register"]
pub mod date;
