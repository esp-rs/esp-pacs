#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
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
    #[doc = "0x40 - HMAC start control register"]
    #[inline(always)]
    pub const fn set_start(&self) -> &SET_START {
        &self.set_start
    }
    #[doc = "0x44 - HMAC parameter configuration register"]
    #[inline(always)]
    pub const fn set_para_purpose(&self) -> &SET_PARA_PURPOSE {
        &self.set_para_purpose
    }
    #[doc = "0x48 - HMAC key configuration register"]
    #[inline(always)]
    pub const fn set_para_key(&self) -> &SET_PARA_KEY {
        &self.set_para_key
    }
    #[doc = "0x4c - HMAC configuration completion register"]
    #[inline(always)]
    pub const fn set_para_finish(&self) -> &SET_PARA_FINISH {
        &self.set_para_finish
    }
    #[doc = "0x50 - HMAC one message control register"]
    #[inline(always)]
    pub const fn set_message_one(&self) -> &SET_MESSAGE_ONE {
        &self.set_message_one
    }
    #[doc = "0x54 - HMAC message continue register"]
    #[inline(always)]
    pub const fn set_message_ing(&self) -> &SET_MESSAGE_ING {
        &self.set_message_ing
    }
    #[doc = "0x58 - HMAC message end register"]
    #[inline(always)]
    pub const fn set_message_end(&self) -> &SET_MESSAGE_END {
        &self.set_message_end
    }
    #[doc = "0x5c - HMAC read result completion register"]
    #[inline(always)]
    pub const fn set_result_finish(&self) -> &SET_RESULT_FINISH {
        &self.set_result_finish
    }
    #[doc = "0x60 - Invalidate JTAG result register"]
    #[inline(always)]
    pub const fn set_invalidate_jtag(&self) -> &SET_INVALIDATE_JTAG {
        &self.set_invalidate_jtag
    }
    #[doc = "0x64 - Invalidate digital signature result register"]
    #[inline(always)]
    pub const fn set_invalidate_ds(&self) -> &SET_INVALIDATE_DS {
        &self.set_invalidate_ds
    }
    #[doc = "0x68 - The matching result between key and purpose user configured"]
    #[inline(always)]
    pub const fn query_error(&self) -> &QUERY_ERROR {
        &self.query_error
    }
    #[doc = "0x6c - The busy state of HMAC module"]
    #[inline(always)]
    pub const fn query_busy(&self) -> &QUERY_BUSY {
        &self.query_busy
    }
    #[doc = "0x80..0xc0 - Message register %s"]
    #[inline(always)]
    pub const fn wr_message_(&self, n: usize) -> &WR_MESSAGE_ {
        &self.wr_message_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0xc0 - Message register %s"]
    #[inline(always)]
    pub fn wr_message__iter(&self) -> impl Iterator<Item = &WR_MESSAGE_> {
        self.wr_message_.iter()
    }
    #[doc = "0xc0..0xe0 - Hash result register %s"]
    #[inline(always)]
    pub const fn rd_result_(&self, n: usize) -> &RD_RESULT_ {
        &self.rd_result_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc0..0xe0 - Hash result register %s"]
    #[inline(always)]
    pub fn rd_result__iter(&self) -> impl Iterator<Item = &RD_RESULT_> {
        self.rd_result_.iter()
    }
    #[doc = "0xf0 - Software padding register"]
    #[inline(always)]
    pub const fn set_message_pad(&self) -> &SET_MESSAGE_PAD {
        &self.set_message_pad
    }
    #[doc = "0xf4 - One block message register."]
    #[inline(always)]
    pub const fn one_block(&self) -> &ONE_BLOCK {
        &self.one_block
    }
    #[doc = "0xf8 - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "SET_START (w) register accessor: HMAC start control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set_start`] module"]
pub type SET_START = crate::Reg<set_start::SET_START_SPEC>;
#[doc = "HMAC start control register"]
pub mod set_start;
#[doc = "SET_PARA_PURPOSE (w) register accessor: HMAC parameter configuration register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_para_purpose::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set_para_purpose`] module"]
pub type SET_PARA_PURPOSE = crate::Reg<set_para_purpose::SET_PARA_PURPOSE_SPEC>;
#[doc = "HMAC parameter configuration register"]
pub mod set_para_purpose;
#[doc = "SET_PARA_KEY (w) register accessor: HMAC key configuration register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_para_key::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set_para_key`] module"]
pub type SET_PARA_KEY = crate::Reg<set_para_key::SET_PARA_KEY_SPEC>;
#[doc = "HMAC key configuration register"]
pub mod set_para_key;
#[doc = "SET_PARA_FINISH (w) register accessor: HMAC configuration completion register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_para_finish::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set_para_finish`] module"]
pub type SET_PARA_FINISH = crate::Reg<set_para_finish::SET_PARA_FINISH_SPEC>;
#[doc = "HMAC configuration completion register"]
pub mod set_para_finish;
#[doc = "SET_MESSAGE_ONE (w) register accessor: HMAC one message control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_message_one::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set_message_one`] module"]
pub type SET_MESSAGE_ONE = crate::Reg<set_message_one::SET_MESSAGE_ONE_SPEC>;
#[doc = "HMAC one message control register"]
pub mod set_message_one;
#[doc = "SET_MESSAGE_ING (w) register accessor: HMAC message continue register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_message_ing::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set_message_ing`] module"]
pub type SET_MESSAGE_ING = crate::Reg<set_message_ing::SET_MESSAGE_ING_SPEC>;
#[doc = "HMAC message continue register"]
pub mod set_message_ing;
#[doc = "SET_MESSAGE_END (w) register accessor: HMAC message end register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_message_end::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set_message_end`] module"]
pub type SET_MESSAGE_END = crate::Reg<set_message_end::SET_MESSAGE_END_SPEC>;
#[doc = "HMAC message end register"]
pub mod set_message_end;
#[doc = "SET_RESULT_FINISH (w) register accessor: HMAC read result completion register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_result_finish::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set_result_finish`] module"]
pub type SET_RESULT_FINISH = crate::Reg<set_result_finish::SET_RESULT_FINISH_SPEC>;
#[doc = "HMAC read result completion register"]
pub mod set_result_finish;
#[doc = "SET_INVALIDATE_JTAG (w) register accessor: Invalidate JTAG result register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_invalidate_jtag::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set_invalidate_jtag`] module"]
pub type SET_INVALIDATE_JTAG = crate::Reg<set_invalidate_jtag::SET_INVALIDATE_JTAG_SPEC>;
#[doc = "Invalidate JTAG result register"]
pub mod set_invalidate_jtag;
#[doc = "SET_INVALIDATE_DS (w) register accessor: Invalidate digital signature result register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_invalidate_ds::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set_invalidate_ds`] module"]
pub type SET_INVALIDATE_DS = crate::Reg<set_invalidate_ds::SET_INVALIDATE_DS_SPEC>;
#[doc = "Invalidate digital signature result register"]
pub mod set_invalidate_ds;
#[doc = "QUERY_ERROR (r) register accessor: The matching result between key and purpose user configured\n\nYou can [`read`](crate::Reg::read) this register and get [`query_error::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@query_error`] module"]
pub type QUERY_ERROR = crate::Reg<query_error::QUERY_ERROR_SPEC>;
#[doc = "The matching result between key and purpose user configured"]
pub mod query_error;
#[doc = "QUERY_BUSY (r) register accessor: The busy state of HMAC module\n\nYou can [`read`](crate::Reg::read) this register and get [`query_busy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@query_busy`] module"]
pub type QUERY_BUSY = crate::Reg<query_busy::QUERY_BUSY_SPEC>;
#[doc = "The busy state of HMAC module"]
pub mod query_busy;
#[doc = "WR_MESSAGE_ (w) register accessor: Message register %s\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_message_::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_message_`] module"]
pub type WR_MESSAGE_ = crate::Reg<wr_message_::WR_MESSAGE__SPEC>;
#[doc = "Message register %s"]
pub mod wr_message_;
#[doc = "RD_RESULT_ (r) register accessor: Hash result register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_result_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_result_`] module"]
pub type RD_RESULT_ = crate::Reg<rd_result_::RD_RESULT__SPEC>;
#[doc = "Hash result register %s"]
pub mod rd_result_;
#[doc = "SET_MESSAGE_PAD (w) register accessor: Software padding register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_message_pad::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set_message_pad`] module"]
pub type SET_MESSAGE_PAD = crate::Reg<set_message_pad::SET_MESSAGE_PAD_SPEC>;
#[doc = "Software padding register"]
pub mod set_message_pad;
#[doc = "ONE_BLOCK (w) register accessor: One block message register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`one_block::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@one_block`] module"]
pub type ONE_BLOCK = crate::Reg<one_block::ONE_BLOCK_SPEC>;
#[doc = "One block message register."]
pub mod one_block;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
