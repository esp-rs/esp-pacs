#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x40],
    set_start: SET_START,
    set_para_purpose: SET_PARA_PURPOSE,
    set_para_key: SET_PARA_KEY,
    set_para_finish: SET_PARA_FINISH,
    set_message_one: SET_MESSAGE_ONE,
    set_message_ing: SET_MESSAGE_ING,
    set_message_end: SET_MESSAGE_END,
    set_result_finish: SET_RESULT_FINISH,
    set_invalidate_jtag: SET_INVALIDATE_JTAG,
    set_invalidate_ds: SET_INVALIDATE_DS,
    query_error: QUERY_ERROR,
    query_busy: QUERY_BUSY,
    _reserved12: [u8; 0x10],
    wr_message_: [WR_MESSAGE_; 16],
    rd_result_: [RD_RESULT_; 8],
    _reserved14: [u8; 0x10],
    set_message_pad: SET_MESSAGE_PAD,
    one_block: ONE_BLOCK,
    date: DATE,
}
impl RegisterBlock {
    ///0x40 - HMAC start control register
    #[inline(always)]
    pub const fn set_start(&self) -> &SET_START {
        &self.set_start
    }
    ///0x44 - HMAC parameter configuration register
    #[inline(always)]
    pub const fn set_para_purpose(&self) -> &SET_PARA_PURPOSE {
        &self.set_para_purpose
    }
    ///0x48 - HMAC key configuration register
    #[inline(always)]
    pub const fn set_para_key(&self) -> &SET_PARA_KEY {
        &self.set_para_key
    }
    ///0x4c - HMAC configuration completion register
    #[inline(always)]
    pub const fn set_para_finish(&self) -> &SET_PARA_FINISH {
        &self.set_para_finish
    }
    ///0x50 - HMAC one message control register
    #[inline(always)]
    pub const fn set_message_one(&self) -> &SET_MESSAGE_ONE {
        &self.set_message_one
    }
    ///0x54 - HMAC message continue register
    #[inline(always)]
    pub const fn set_message_ing(&self) -> &SET_MESSAGE_ING {
        &self.set_message_ing
    }
    ///0x58 - HMAC message end register
    #[inline(always)]
    pub const fn set_message_end(&self) -> &SET_MESSAGE_END {
        &self.set_message_end
    }
    ///0x5c - HMAC read result completion register
    #[inline(always)]
    pub const fn set_result_finish(&self) -> &SET_RESULT_FINISH {
        &self.set_result_finish
    }
    ///0x60 - Invalidate JTAG result register
    #[inline(always)]
    pub const fn set_invalidate_jtag(&self) -> &SET_INVALIDATE_JTAG {
        &self.set_invalidate_jtag
    }
    ///0x64 - Invalidate digital signature result register
    #[inline(always)]
    pub const fn set_invalidate_ds(&self) -> &SET_INVALIDATE_DS {
        &self.set_invalidate_ds
    }
    ///0x68 - The matching result between key and purpose user configured
    #[inline(always)]
    pub const fn query_error(&self) -> &QUERY_ERROR {
        &self.query_error
    }
    ///0x6c - The busy state of HMAC module
    #[inline(always)]
    pub const fn query_busy(&self) -> &QUERY_BUSY {
        &self.query_busy
    }
    ///0x80..0xc0 - Message register %s
    #[inline(always)]
    pub const fn wr_message_(&self, n: usize) -> &WR_MESSAGE_ {
        &self.wr_message_[n]
    }
    ///Iterator for array of:
    ///0x80..0xc0 - Message register %s
    #[inline(always)]
    pub fn wr_message__iter(&self) -> impl Iterator<Item = &WR_MESSAGE_> {
        self.wr_message_.iter()
    }
    ///0xc0..0xe0 - Hash result register %s
    #[inline(always)]
    pub const fn rd_result_(&self, n: usize) -> &RD_RESULT_ {
        &self.rd_result_[n]
    }
    ///Iterator for array of:
    ///0xc0..0xe0 - Hash result register %s
    #[inline(always)]
    pub fn rd_result__iter(&self) -> impl Iterator<Item = &RD_RESULT_> {
        self.rd_result_.iter()
    }
    ///0xf0 - Software padding register
    #[inline(always)]
    pub const fn set_message_pad(&self) -> &SET_MESSAGE_PAD {
        &self.set_message_pad
    }
    ///0xf4 - One block message register.
    #[inline(always)]
    pub const fn one_block(&self) -> &ONE_BLOCK {
        &self.one_block
    }
    ///0xf8 - Version control register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**SET_START (w) register accessor: HMAC start control register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@set_start`] module*/
pub type SET_START = crate::Reg<set_start::SET_START_SPEC>;
///HMAC start control register
pub mod set_start;
/**SET_PARA_PURPOSE (w) register accessor: HMAC parameter configuration register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_para_purpose::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@set_para_purpose`] module*/
pub type SET_PARA_PURPOSE = crate::Reg<set_para_purpose::SET_PARA_PURPOSE_SPEC>;
///HMAC parameter configuration register
pub mod set_para_purpose;
/**SET_PARA_KEY (w) register accessor: HMAC key configuration register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_para_key::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@set_para_key`] module*/
pub type SET_PARA_KEY = crate::Reg<set_para_key::SET_PARA_KEY_SPEC>;
///HMAC key configuration register
pub mod set_para_key;
/**SET_PARA_FINISH (w) register accessor: HMAC configuration completion register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_para_finish::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@set_para_finish`] module*/
pub type SET_PARA_FINISH = crate::Reg<set_para_finish::SET_PARA_FINISH_SPEC>;
///HMAC configuration completion register
pub mod set_para_finish;
/**SET_MESSAGE_ONE (w) register accessor: HMAC one message control register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_message_one::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@set_message_one`] module*/
pub type SET_MESSAGE_ONE = crate::Reg<set_message_one::SET_MESSAGE_ONE_SPEC>;
///HMAC one message control register
pub mod set_message_one;
/**SET_MESSAGE_ING (w) register accessor: HMAC message continue register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_message_ing::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@set_message_ing`] module*/
pub type SET_MESSAGE_ING = crate::Reg<set_message_ing::SET_MESSAGE_ING_SPEC>;
///HMAC message continue register
pub mod set_message_ing;
/**SET_MESSAGE_END (w) register accessor: HMAC message end register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_message_end::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@set_message_end`] module*/
pub type SET_MESSAGE_END = crate::Reg<set_message_end::SET_MESSAGE_END_SPEC>;
///HMAC message end register
pub mod set_message_end;
/**SET_RESULT_FINISH (w) register accessor: HMAC read result completion register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_result_finish::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@set_result_finish`] module*/
pub type SET_RESULT_FINISH = crate::Reg<set_result_finish::SET_RESULT_FINISH_SPEC>;
///HMAC read result completion register
pub mod set_result_finish;
/**SET_INVALIDATE_JTAG (w) register accessor: Invalidate JTAG result register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_invalidate_jtag::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@set_invalidate_jtag`] module*/
pub type SET_INVALIDATE_JTAG = crate::Reg<set_invalidate_jtag::SET_INVALIDATE_JTAG_SPEC>;
///Invalidate JTAG result register
pub mod set_invalidate_jtag;
/**SET_INVALIDATE_DS (w) register accessor: Invalidate digital signature result register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_invalidate_ds::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@set_invalidate_ds`] module*/
pub type SET_INVALIDATE_DS = crate::Reg<set_invalidate_ds::SET_INVALIDATE_DS_SPEC>;
///Invalidate digital signature result register
pub mod set_invalidate_ds;
/**QUERY_ERROR (r) register accessor: The matching result between key and purpose user configured

You can [`read`](crate::generic::Reg::read) this register and get [`query_error::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@query_error`] module*/
pub type QUERY_ERROR = crate::Reg<query_error::QUERY_ERROR_SPEC>;
///The matching result between key and purpose user configured
pub mod query_error;
/**QUERY_BUSY (r) register accessor: The busy state of HMAC module

You can [`read`](crate::generic::Reg::read) this register and get [`query_busy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@query_busy`] module*/
pub type QUERY_BUSY = crate::Reg<query_busy::QUERY_BUSY_SPEC>;
///The busy state of HMAC module
pub mod query_busy;
/**WR_MESSAGE_ (w) register accessor: Message register %s

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_message_::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wr_message_`] module*/
pub type WR_MESSAGE_ = crate::Reg<wr_message_::WR_MESSAGE__SPEC>;
///Message register %s
pub mod wr_message_;
/**RD_RESULT_ (r) register accessor: Hash result register %s

You can [`read`](crate::generic::Reg::read) this register and get [`rd_result_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_result_`] module*/
pub type RD_RESULT_ = crate::Reg<rd_result_::RD_RESULT__SPEC>;
///Hash result register %s
pub mod rd_result_;
/**SET_MESSAGE_PAD (w) register accessor: Software padding register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_message_pad::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@set_message_pad`] module*/
pub type SET_MESSAGE_PAD = crate::Reg<set_message_pad::SET_MESSAGE_PAD_SPEC>;
///Software padding register
pub mod set_message_pad;
/**ONE_BLOCK (w) register accessor: One block message register.

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`one_block::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@one_block`] module*/
pub type ONE_BLOCK = crate::Reg<one_block::ONE_BLOCK_SPEC>;
///One block message register.
pub mod one_block;
/**DATE (rw) register accessor: Version control register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///Version control register
pub mod date;
