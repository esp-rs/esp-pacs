#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    text: [TEXT; 32],
    sha1_start: SHA1_START,
    sha1_continue: SHA1_CONTINUE,
    sha1_load: SHA1_LOAD,
    sha1_busy: SHA1_BUSY,
    sha256_start: SHA256_START,
    sha256_continue: SHA256_CONTINUE,
    sha256_load: SHA256_LOAD,
    sha256_busy: SHA256_BUSY,
    sha384_start: SHA384_START,
    sha384_continue: SHA384_CONTINUE,
    sha384_load: SHA384_LOAD,
    sha384_busy: SHA384_BUSY,
    sha512_start: SHA512_START,
    sha512_continue: SHA512_CONTINUE,
    sha512_load: SHA512_LOAD,
    sha512_busy: SHA512_BUSY,
}
impl RegisterBlock {
    ///0x00..0x80 -
    #[inline(always)]
    pub const fn text(&self, n: usize) -> &TEXT {
        &self.text[n]
    }
    ///Iterator for array of:
    ///0x00..0x80 -
    #[inline(always)]
    pub fn text_iter(&self) -> impl Iterator<Item = &TEXT> {
        self.text.iter()
    }
    ///0x80 -
    #[inline(always)]
    pub const fn sha1_start(&self) -> &SHA1_START {
        &self.sha1_start
    }
    ///0x84 -
    #[inline(always)]
    pub const fn sha1_continue(&self) -> &SHA1_CONTINUE {
        &self.sha1_continue
    }
    ///0x88 -
    #[inline(always)]
    pub const fn sha1_load(&self) -> &SHA1_LOAD {
        &self.sha1_load
    }
    ///0x8c -
    #[inline(always)]
    pub const fn sha1_busy(&self) -> &SHA1_BUSY {
        &self.sha1_busy
    }
    ///0x90 -
    #[inline(always)]
    pub const fn sha256_start(&self) -> &SHA256_START {
        &self.sha256_start
    }
    ///0x94 -
    #[inline(always)]
    pub const fn sha256_continue(&self) -> &SHA256_CONTINUE {
        &self.sha256_continue
    }
    ///0x98 -
    #[inline(always)]
    pub const fn sha256_load(&self) -> &SHA256_LOAD {
        &self.sha256_load
    }
    ///0x9c -
    #[inline(always)]
    pub const fn sha256_busy(&self) -> &SHA256_BUSY {
        &self.sha256_busy
    }
    ///0xa0 -
    #[inline(always)]
    pub const fn sha384_start(&self) -> &SHA384_START {
        &self.sha384_start
    }
    ///0xa4 -
    #[inline(always)]
    pub const fn sha384_continue(&self) -> &SHA384_CONTINUE {
        &self.sha384_continue
    }
    ///0xa8 -
    #[inline(always)]
    pub const fn sha384_load(&self) -> &SHA384_LOAD {
        &self.sha384_load
    }
    ///0xac -
    #[inline(always)]
    pub const fn sha384_busy(&self) -> &SHA384_BUSY {
        &self.sha384_busy
    }
    ///0xb0 -
    #[inline(always)]
    pub const fn sha512_start(&self) -> &SHA512_START {
        &self.sha512_start
    }
    ///0xb4 -
    #[inline(always)]
    pub const fn sha512_continue(&self) -> &SHA512_CONTINUE {
        &self.sha512_continue
    }
    ///0xb8 -
    #[inline(always)]
    pub const fn sha512_load(&self) -> &SHA512_LOAD {
        &self.sha512_load
    }
    ///0xbc -
    #[inline(always)]
    pub const fn sha512_busy(&self) -> &SHA512_BUSY {
        &self.sha512_busy
    }
}
/**TEXT (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`text::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`text::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@text`] module*/
pub type TEXT = crate::Reg<text::TEXT_SPEC>;
///
pub mod text;
/**SHA1_START (w) register accessor:

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha1_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sha1_start`] module*/
pub type SHA1_START = crate::Reg<sha1_start::SHA1_START_SPEC>;
///
pub mod sha1_start;
/**SHA1_CONTINUE (w) register accessor:

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha1_continue::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sha1_continue`] module*/
pub type SHA1_CONTINUE = crate::Reg<sha1_continue::SHA1_CONTINUE_SPEC>;
///
pub mod sha1_continue;
/**SHA1_LOAD (w) register accessor:

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha1_load::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sha1_load`] module*/
pub type SHA1_LOAD = crate::Reg<sha1_load::SHA1_LOAD_SPEC>;
///
pub mod sha1_load;
/**SHA1_BUSY (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`sha1_busy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sha1_busy`] module*/
pub type SHA1_BUSY = crate::Reg<sha1_busy::SHA1_BUSY_SPEC>;
///
pub mod sha1_busy;
/**SHA256_START (w) register accessor:

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha256_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sha256_start`] module*/
pub type SHA256_START = crate::Reg<sha256_start::SHA256_START_SPEC>;
///
pub mod sha256_start;
/**SHA256_LOAD (w) register accessor:

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha256_load::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sha256_load`] module*/
pub type SHA256_LOAD = crate::Reg<sha256_load::SHA256_LOAD_SPEC>;
///
pub mod sha256_load;
/**SHA256_CONTINUE (w) register accessor:

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha256_continue::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sha256_continue`] module*/
pub type SHA256_CONTINUE = crate::Reg<sha256_continue::SHA256_CONTINUE_SPEC>;
///
pub mod sha256_continue;
/**SHA256_BUSY (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`sha256_busy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sha256_busy`] module*/
pub type SHA256_BUSY = crate::Reg<sha256_busy::SHA256_BUSY_SPEC>;
///
pub mod sha256_busy;
/**SHA384_START (w) register accessor:

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha384_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sha384_start`] module*/
pub type SHA384_START = crate::Reg<sha384_start::SHA384_START_SPEC>;
///
pub mod sha384_start;
/**SHA384_CONTINUE (w) register accessor:

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha384_continue::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sha384_continue`] module*/
pub type SHA384_CONTINUE = crate::Reg<sha384_continue::SHA384_CONTINUE_SPEC>;
///
pub mod sha384_continue;
/**SHA384_LOAD (w) register accessor:

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha384_load::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sha384_load`] module*/
pub type SHA384_LOAD = crate::Reg<sha384_load::SHA384_LOAD_SPEC>;
///
pub mod sha384_load;
/**SHA384_BUSY (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`sha384_busy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sha384_busy`] module*/
pub type SHA384_BUSY = crate::Reg<sha384_busy::SHA384_BUSY_SPEC>;
///
pub mod sha384_busy;
/**SHA512_START (w) register accessor:

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha512_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sha512_start`] module*/
pub type SHA512_START = crate::Reg<sha512_start::SHA512_START_SPEC>;
///
pub mod sha512_start;
/**SHA512_CONTINUE (w) register accessor:

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha512_continue::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sha512_continue`] module*/
pub type SHA512_CONTINUE = crate::Reg<sha512_continue::SHA512_CONTINUE_SPEC>;
///
pub mod sha512_continue;
/**SHA512_LOAD (w) register accessor:

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha512_load::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sha512_load`] module*/
pub type SHA512_LOAD = crate::Reg<sha512_load::SHA512_LOAD_SPEC>;
///
pub mod sha512_load;
/**SHA512_BUSY (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`sha512_busy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sha512_busy`] module*/
pub type SHA512_BUSY = crate::Reg<sha512_busy::SHA512_BUSY_SPEC>;
///
pub mod sha512_busy;
