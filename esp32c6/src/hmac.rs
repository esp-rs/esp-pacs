#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    _reserved0: [u8; 0x40],
    #[doc = "0x40 - Process control register 0."]
    pub set_start: SET_START,
    #[doc = "0x44 - Configure purpose."]
    pub set_para_purpose: SET_PARA_PURPOSE,
    #[doc = "0x48 - Configure key."]
    pub set_para_key: SET_PARA_KEY,
    #[doc = "0x4c - Finish initial configuration."]
    pub set_para_finish: SET_PARA_FINISH,
    #[doc = "0x50 - Process control register 1."]
    pub set_message_one: SET_MESSAGE_ONE,
    #[doc = "0x54 - Process control register 2."]
    pub set_message_ing: SET_MESSAGE_ING,
    #[doc = "0x58 - Process control register 3."]
    pub set_message_end: SET_MESSAGE_END,
    #[doc = "0x5c - Process control register 4."]
    pub set_result_finish: SET_RESULT_FINISH,
    #[doc = "0x60 - Invalidate register 0."]
    pub set_invalidate_jtag: SET_INVALIDATE_JTAG,
    #[doc = "0x64 - Invalidate register 1."]
    pub set_invalidate_ds: SET_INVALIDATE_DS,
    #[doc = "0x68 - Error register."]
    pub query_error: QUERY_ERROR,
    #[doc = "0x6c - Busy register."]
    pub query_busy: QUERY_BUSY,
    _reserved12: [u8; 0x10],
    #[doc = "0x80..0xc0 - Message block memory."]
    pub wr_message_mem: [WR_MESSAGE_MEM; 64],
    #[doc = "0xc0..0xe0 - Result from upstream."]
    pub rd_result_mem: [RD_RESULT_MEM; 32],
    _reserved14: [u8; 0x10],
    #[doc = "0xf0 - Process control register 5."]
    pub set_message_pad: SET_MESSAGE_PAD,
    #[doc = "0xf4 - Process control register 6."]
    pub one_block: ONE_BLOCK,
    #[doc = "0xf8 - Jtag register 0."]
    pub soft_jtag_ctrl: SOFT_JTAG_CTRL,
    #[doc = "0xfc - Jtag register 1."]
    pub wr_jtag: WR_JTAG,
    _reserved18: [u8; 0xfc],
    #[doc = "0x1fc - Date register."]
    pub date: DATE,
}
#[doc = "SET_START (w) register accessor: Process control register 0.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`set_start`] module"]
pub type SET_START = crate::Reg<set_start::SET_START_SPEC>;
#[doc = "Process control register 0."]
pub mod set_start;
#[doc = "SET_PARA_PURPOSE (w) register accessor: Configure purpose.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_para_purpose::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`set_para_purpose`] module"]
pub type SET_PARA_PURPOSE = crate::Reg<set_para_purpose::SET_PARA_PURPOSE_SPEC>;
#[doc = "Configure purpose."]
pub mod set_para_purpose;
#[doc = "SET_PARA_KEY (w) register accessor: Configure key.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_para_key::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`set_para_key`] module"]
pub type SET_PARA_KEY = crate::Reg<set_para_key::SET_PARA_KEY_SPEC>;
#[doc = "Configure key."]
pub mod set_para_key;
#[doc = "SET_PARA_FINISH (w) register accessor: Finish initial configuration.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_para_finish::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`set_para_finish`] module"]
pub type SET_PARA_FINISH = crate::Reg<set_para_finish::SET_PARA_FINISH_SPEC>;
#[doc = "Finish initial configuration."]
pub mod set_para_finish;
#[doc = "SET_MESSAGE_ONE (w) register accessor: Process control register 1.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_message_one::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`set_message_one`] module"]
pub type SET_MESSAGE_ONE = crate::Reg<set_message_one::SET_MESSAGE_ONE_SPEC>;
#[doc = "Process control register 1."]
pub mod set_message_one;
#[doc = "SET_MESSAGE_ING (w) register accessor: Process control register 2.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_message_ing::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`set_message_ing`] module"]
pub type SET_MESSAGE_ING = crate::Reg<set_message_ing::SET_MESSAGE_ING_SPEC>;
#[doc = "Process control register 2."]
pub mod set_message_ing;
#[doc = "SET_MESSAGE_END (w) register accessor: Process control register 3.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_message_end::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`set_message_end`] module"]
pub type SET_MESSAGE_END = crate::Reg<set_message_end::SET_MESSAGE_END_SPEC>;
#[doc = "Process control register 3."]
pub mod set_message_end;
#[doc = "SET_RESULT_FINISH (w) register accessor: Process control register 4.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_result_finish::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`set_result_finish`] module"]
pub type SET_RESULT_FINISH = crate::Reg<set_result_finish::SET_RESULT_FINISH_SPEC>;
#[doc = "Process control register 4."]
pub mod set_result_finish;
#[doc = "SET_INVALIDATE_JTAG (w) register accessor: Invalidate register 0.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_invalidate_jtag::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`set_invalidate_jtag`] module"]
pub type SET_INVALIDATE_JTAG = crate::Reg<set_invalidate_jtag::SET_INVALIDATE_JTAG_SPEC>;
#[doc = "Invalidate register 0."]
pub mod set_invalidate_jtag;
#[doc = "SET_INVALIDATE_DS (w) register accessor: Invalidate register 1.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_invalidate_ds::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`set_invalidate_ds`] module"]
pub type SET_INVALIDATE_DS = crate::Reg<set_invalidate_ds::SET_INVALIDATE_DS_SPEC>;
#[doc = "Invalidate register 1."]
pub mod set_invalidate_ds;
#[doc = "QUERY_ERROR (r) register accessor: Error register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`query_error::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`query_error`] module"]
pub type QUERY_ERROR = crate::Reg<query_error::QUERY_ERROR_SPEC>;
#[doc = "Error register."]
pub mod query_error;
#[doc = "QUERY_BUSY (r) register accessor: Busy register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`query_busy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`query_busy`] module"]
pub type QUERY_BUSY = crate::Reg<query_busy::QUERY_BUSY_SPEC>;
#[doc = "Busy register."]
pub mod query_busy;
#[doc = "WR_MESSAGE_MEM (rw) register accessor: Message block memory.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_message_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_message_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wr_message_mem`] module"]
pub type WR_MESSAGE_MEM = crate::Reg<wr_message_mem::WR_MESSAGE_MEM_SPEC>;
#[doc = "Message block memory."]
pub mod wr_message_mem;
#[doc = "RD_RESULT_MEM (rw) register accessor: Result from upstream.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_result_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd_result_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_result_mem`] module"]
pub type RD_RESULT_MEM = crate::Reg<rd_result_mem::RD_RESULT_MEM_SPEC>;
#[doc = "Result from upstream."]
pub mod rd_result_mem;
#[doc = "SET_MESSAGE_PAD (w) register accessor: Process control register 5.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_message_pad::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`set_message_pad`] module"]
pub type SET_MESSAGE_PAD = crate::Reg<set_message_pad::SET_MESSAGE_PAD_SPEC>;
#[doc = "Process control register 5."]
pub mod set_message_pad;
#[doc = "ONE_BLOCK (w) register accessor: Process control register 6.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`one_block::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`one_block`] module"]
pub type ONE_BLOCK = crate::Reg<one_block::ONE_BLOCK_SPEC>;
#[doc = "Process control register 6."]
pub mod one_block;
#[doc = "SOFT_JTAG_CTRL (w) register accessor: Jtag register 0.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soft_jtag_ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`soft_jtag_ctrl`] module"]
pub type SOFT_JTAG_CTRL = crate::Reg<soft_jtag_ctrl::SOFT_JTAG_CTRL_SPEC>;
#[doc = "Jtag register 0."]
pub mod soft_jtag_ctrl;
#[doc = "WR_JTAG (w) register accessor: Jtag register 1.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_jtag::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wr_jtag`] module"]
pub type WR_JTAG = crate::Reg<wr_jtag::WR_JTAG_SPEC>;
#[doc = "Jtag register 1."]
pub mod wr_jtag;
#[doc = "DATE (rw) register accessor: Date register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Date register."]
pub mod date;
