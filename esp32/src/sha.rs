#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x80 - "]
    pub text_: [TEXT_; 32],
    _reserved_1_sha1: [u8; 0x04],
    _reserved2: [u8; 0x04],
    #[doc = "0x88 - "]
    pub sha1_load: SHA1_LOAD,
    #[doc = "0x8c - "]
    pub sha1_busy: SHA1_BUSY,
    _reserved_4_sha256: [u8; 0x04],
    #[doc = "0x94 - "]
    pub sha256_continue: SHA256_CONTINUE,
    _reserved6: [u8; 0x04],
    #[doc = "0x9c - "]
    pub sha256_busy: SHA256_BUSY,
    #[doc = "0xa0 - "]
    pub sha384_start: SHA384_START,
    #[doc = "0xa4 - "]
    pub sha384_continue: SHA384_CONTINUE,
    #[doc = "0xa8 - "]
    pub sha384_load: SHA384_LOAD,
    #[doc = "0xac - "]
    pub sha384_busy: SHA384_BUSY,
    #[doc = "0xb0 - "]
    pub sha512_start: SHA512_START,
    #[doc = "0xb4 - "]
    pub sha512_continue: SHA512_CONTINUE,
    #[doc = "0xb8 - "]
    pub sha512_load: SHA512_LOAD,
    #[doc = "0xbc - "]
    pub sha512_busy: SHA512_BUSY,
}
impl RegisterBlock {
    #[doc = "0x80 - "]
    #[inline(always)]
    pub fn sha1_continue(&self) -> &SHA1_CONTINUE {
        unsafe { &*(((self as *const Self) as *const u8).add(128usize) as *const SHA1_CONTINUE) }
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub fn sha1_start(&self) -> &SHA1_START {
        unsafe { &*(((self as *const Self) as *const u8).add(128usize) as *const SHA1_START) }
    }
    #[doc = "0x90 - "]
    #[inline(always)]
    pub fn sha256_load(&self) -> &SHA256_LOAD {
        unsafe { &*(((self as *const Self) as *const u8).add(144usize) as *const SHA256_LOAD) }
    }
    #[doc = "0x90 - "]
    #[inline(always)]
    pub fn sha256_start(&self) -> &SHA256_START {
        unsafe { &*(((self as *const Self) as *const u8).add(144usize) as *const SHA256_START) }
    }
}
#[doc = "TEXT_ (rw) register accessor: an alias for `Reg<TEXT__SPEC>`"]
pub type TEXT_ = crate::Reg<text_::TEXT__SPEC>;
#[doc = ""]
pub mod text_;
#[doc = "SHA1_START (w) register accessor: an alias for `Reg<SHA1_START_SPEC>`"]
pub type SHA1_START = crate::Reg<sha1_start::SHA1_START_SPEC>;
#[doc = ""]
pub mod sha1_start;
#[doc = "SHA1_CONTINUE (w) register accessor: an alias for `Reg<SHA1_CONTINUE_SPEC>`"]
pub type SHA1_CONTINUE = crate::Reg<sha1_continue::SHA1_CONTINUE_SPEC>;
#[doc = ""]
pub mod sha1_continue;
#[doc = "SHA1_LOAD (w) register accessor: an alias for `Reg<SHA1_LOAD_SPEC>`"]
pub type SHA1_LOAD = crate::Reg<sha1_load::SHA1_LOAD_SPEC>;
#[doc = ""]
pub mod sha1_load;
#[doc = "SHA1_BUSY (w) register accessor: an alias for `Reg<SHA1_BUSY_SPEC>`"]
pub type SHA1_BUSY = crate::Reg<sha1_busy::SHA1_BUSY_SPEC>;
#[doc = ""]
pub mod sha1_busy;
#[doc = "SHA256_START (w) register accessor: an alias for `Reg<SHA256_START_SPEC>`"]
pub type SHA256_START = crate::Reg<sha256_start::SHA256_START_SPEC>;
#[doc = ""]
pub mod sha256_start;
#[doc = "SHA256_LOAD (w) register accessor: an alias for `Reg<SHA256_LOAD_SPEC>`"]
pub type SHA256_LOAD = crate::Reg<sha256_load::SHA256_LOAD_SPEC>;
#[doc = ""]
pub mod sha256_load;
#[doc = "SHA256_CONTINUE (w) register accessor: an alias for `Reg<SHA256_CONTINUE_SPEC>`"]
pub type SHA256_CONTINUE = crate::Reg<sha256_continue::SHA256_CONTINUE_SPEC>;
#[doc = ""]
pub mod sha256_continue;
#[doc = "SHA256_BUSY (r) register accessor: an alias for `Reg<SHA256_BUSY_SPEC>`"]
pub type SHA256_BUSY = crate::Reg<sha256_busy::SHA256_BUSY_SPEC>;
#[doc = ""]
pub mod sha256_busy;
#[doc = "SHA384_START (w) register accessor: an alias for `Reg<SHA384_START_SPEC>`"]
pub type SHA384_START = crate::Reg<sha384_start::SHA384_START_SPEC>;
#[doc = ""]
pub mod sha384_start;
#[doc = "SHA384_CONTINUE (w) register accessor: an alias for `Reg<SHA384_CONTINUE_SPEC>`"]
pub type SHA384_CONTINUE = crate::Reg<sha384_continue::SHA384_CONTINUE_SPEC>;
#[doc = ""]
pub mod sha384_continue;
#[doc = "SHA384_LOAD (w) register accessor: an alias for `Reg<SHA384_LOAD_SPEC>`"]
pub type SHA384_LOAD = crate::Reg<sha384_load::SHA384_LOAD_SPEC>;
#[doc = ""]
pub mod sha384_load;
#[doc = "SHA384_BUSY (r) register accessor: an alias for `Reg<SHA384_BUSY_SPEC>`"]
pub type SHA384_BUSY = crate::Reg<sha384_busy::SHA384_BUSY_SPEC>;
#[doc = ""]
pub mod sha384_busy;
#[doc = "SHA512_START (w) register accessor: an alias for `Reg<SHA512_START_SPEC>`"]
pub type SHA512_START = crate::Reg<sha512_start::SHA512_START_SPEC>;
#[doc = ""]
pub mod sha512_start;
#[doc = "SHA512_CONTINUE (w) register accessor: an alias for `Reg<SHA512_CONTINUE_SPEC>`"]
pub type SHA512_CONTINUE = crate::Reg<sha512_continue::SHA512_CONTINUE_SPEC>;
#[doc = ""]
pub mod sha512_continue;
#[doc = "SHA512_LOAD (w) register accessor: an alias for `Reg<SHA512_LOAD_SPEC>`"]
pub type SHA512_LOAD = crate::Reg<sha512_load::SHA512_LOAD_SPEC>;
#[doc = ""]
pub mod sha512_load;
#[doc = "SHA512_BUSY (r) register accessor: an alias for `Reg<SHA512_BUSY_SPEC>`"]
pub type SHA512_BUSY = crate::Reg<sha512_busy::SHA512_BUSY_SPEC>;
#[doc = ""]
pub mod sha512_busy;
