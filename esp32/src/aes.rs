#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    start: START,
    idle: IDLE,
    mode: MODE,
    _reserved3: [u8; 0x04],
    key: [KEY; 8],
    text: [TEXT; 4],
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
    pub const fn key(&self, n: usize) -> &KEY {
        &self.key[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x30 - "]
    #[inline(always)]
    pub fn key_iter(&self) -> impl Iterator<Item = &KEY> {
        self.key.iter()
    }
    #[doc = "0x30..0x40 - "]
    #[inline(always)]
    pub const fn text(&self, n: usize) -> &TEXT {
        &self.text[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x40 - "]
    #[inline(always)]
    pub fn text_iter(&self) -> impl Iterator<Item = &TEXT> {
        self.text.iter()
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
#[doc = "KEY (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key`] module"]
pub type KEY = crate::Reg<key::KEY_SPEC>;
#[doc = ""]
pub mod key;
#[doc = "TEXT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`text::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`text::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@text`] module"]
pub type TEXT = crate::Reg<text::TEXT_SPEC>;
#[doc = ""]
pub mod text;
#[doc = "ENDIAN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endian::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endian::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endian`] module"]
pub type ENDIAN = crate::Reg<endian::ENDIAN_SPEC>;
#[doc = ""]
pub mod endian;
