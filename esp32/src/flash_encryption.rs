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
#[doc = "BUFFER_ (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buffer_::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`buffer_`] module"]
pub type BUFFER_ = crate::Reg<buffer_::BUFFER__SPEC>;
#[doc = ""]
pub mod buffer_;
#[doc = "START (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`start`] module"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = ""]
pub mod start;
#[doc = "ADDRESS (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`address::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`address`] module"]
pub type ADDRESS = crate::Reg<address::ADDRESS_SPEC>;
#[doc = ""]
pub mod address;
#[doc = "DONE (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`done::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`done`] module"]
pub type DONE = crate::Reg<done::DONE_SPEC>;
#[doc = ""]
pub mod done;
