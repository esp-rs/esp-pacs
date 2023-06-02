#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00..0x20 - "]
    pub buffer_: [BUFFER_; 8],
    #[doc = "0x20 - "]
    pub start: START,
    #[doc = "0x24 - "]
    pub address: ADDRESS,
    #[doc = "0x28 - "]
    pub done: DONE,
}
#[doc = "BUFFER_ (w) register accessor: an alias for `Reg<BUFFER__SPEC>`"]
pub type BUFFER_ = crate::Reg<buffer_::BUFFER__SPEC>;
#[doc = ""]
pub mod buffer_;
#[doc = "START (w) register accessor: an alias for `Reg<START_SPEC>`"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = ""]
pub mod start;
#[doc = "ADDRESS (w) register accessor: an alias for `Reg<ADDRESS_SPEC>`"]
pub type ADDRESS = crate::Reg<address::ADDRESS_SPEC>;
#[doc = ""]
pub mod address;
#[doc = "DONE (r) register accessor: an alias for `Reg<DONE_SPEC>`"]
pub type DONE = crate::Reg<done::DONE_SPEC>;
#[doc = ""]
pub mod done;
