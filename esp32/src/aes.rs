#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    start: START,
    idle: IDLE,
    mode: MODE,
    _reserved3: [u8; 0x04],
    key_: [KEY_; 8],
    text_: [TEXT_; 4],
    endian: ENDIAN,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn start(&self) -> &START {
        &self.start
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn idle(&self) -> &IDLE {
        &self.idle
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn mode(&self) -> &MODE {
        &self.mode
    }
    #[doc = "0x10..0x30 - "]
    #[inline(always)]
    pub const fn key_(&self, n: usize) -> &KEY_ {
        &self.key_[n]
    }
    #[doc = "0x30..0x40 - "]
    #[inline(always)]
    pub const fn text_(&self, n: usize) -> &TEXT_ {
        &self.text_[n]
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn endian(&self) -> &ENDIAN {
        &self.endian
    }
}
#[doc = "START (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`] module"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = ""]
pub mod start;
#[doc = "IDLE (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idle::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idle`] module"]
pub type IDLE = crate::Reg<idle::IDLE_SPEC>;
#[doc = ""]
pub mod idle;
#[doc = "MODE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`] module"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = ""]
pub mod mode;
#[doc = "KEY_ (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_`] module"]
pub type KEY_ = crate::Reg<key_::KEY__SPEC>;
#[doc = ""]
pub mod key_;
#[doc = "TEXT_ (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`text_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`text_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@text_`] module"]
pub type TEXT_ = crate::Reg<text_::TEXT__SPEC>;
#[doc = ""]
pub mod text_;
#[doc = "ENDIAN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endian::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endian::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endian`] module"]
pub type ENDIAN = crate::Reg<endian::ENDIAN_SPEC>;
#[doc = ""]
pub mod endian;
