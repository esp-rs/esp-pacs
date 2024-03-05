#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    buffer_: [BUFFER_; 8],
    start: START,
    address: ADDRESS,
    done: DONE,
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - "]
    #[inline(always)]
    pub const fn buffer_(&self, n: usize) -> &BUFFER_ {
        &self.buffer_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - "]
    #[inline(always)]
    pub fn buffer__iter(&self) -> impl Iterator<Item = &BUFFER_> {
        self.buffer_.iter()
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn start(&self) -> &START {
        &self.start
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn address(&self) -> &ADDRESS {
        &self.address
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn done(&self) -> &DONE {
        &self.done
    }
}
#[doc = "BUFFER_ (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buffer_::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buffer_`] module"]
pub type BUFFER_ = crate::Reg<buffer_::BUFFER__SPEC>;
#[doc = ""]
pub mod buffer_;
#[doc = "START (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`] module"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = ""]
pub mod start;
#[doc = "ADDRESS (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`address::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address`] module"]
pub type ADDRESS = crate::Reg<address::ADDRESS_SPEC>;
#[doc = ""]
pub mod address;
#[doc = "DONE (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`done::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@done`] module"]
pub type DONE = crate::Reg<done::DONE_SPEC>;
#[doc = ""]
pub mod done;
