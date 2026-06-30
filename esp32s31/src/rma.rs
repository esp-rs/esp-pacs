#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    config: CONFIG,
    trigger: TRIGGER,
    state: STATE,
    log: LOG,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    result_st: RESULT_ST,
    result_clr: RESULT_CLR,
    timeout_limit: TIMEOUT_LIMIT,
    _reserved11: [u8; 0x50],
    date: DATE,
    chip_info_source_mem: [CHIP_INFO_SOURCE_MEM; 4],
    nonce_mem: [NONCE_MEM; 4],
    usc_mem: [USC_MEM; 4],
    public_key_mem: [PUBLIC_KEY_MEM; 4],
    sign_mem: [SIGN_MEM; 4],
    cert_hash_mem: [CERT_HASH_MEM; 4],
    chip_info_mem: [CHIP_INFO_MEM; 4],
}
impl RegisterBlock {
    #[doc = "0x00 - Configures RMA algorithm"]
    #[inline(always)]
    pub const fn config(&self) -> &CONFIG {
        &self.config
    }
    #[doc = "0x04 - Starts the RMA module."]
    #[inline(always)]
    pub const fn trigger(&self) -> &TRIGGER {
        &self.trigger
    }
    #[doc = "0x08 - query state in rma"]
    #[inline(always)]
    pub const fn state(&self) -> &STATE {
        &self.state
    }
    #[doc = "0x0c - Query result in rma"]
    #[inline(always)]
    pub const fn log(&self) -> &LOG {
        &self.log
    }
    #[doc = "0x10 - RMA interrupt raw register, valid in level."]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x14 - RMA interrupt status register."]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x18 - RMA interrupt enable register."]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x1c - RMA interrupt clear register."]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x20 - RMA result reg."]
    #[inline(always)]
    pub const fn result_st(&self) -> &RESULT_ST {
        &self.result_st
    }
    #[doc = "0x24 - RMA clr result reg."]
    #[inline(always)]
    pub const fn result_clr(&self) -> &RESULT_CLR {
        &self.result_clr
    }
    #[doc = "0x28 - RMA timeout limit configurate register"]
    #[inline(always)]
    pub const fn timeout_limit(&self) -> &TIMEOUT_LIMIT {
        &self.timeout_limit
    }
    #[doc = "0x7c - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    #[doc = "0x80 - RMA chip info memory."]
    #[inline(always)]
    pub const fn chip_info_source_mem(&self, n: usize) -> &CHIP_INFO_SOURCE_MEM {
        &self.chip_info_source_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80 - RMA chip info memory."]
    #[inline(always)]
    pub fn chip_info_source_mem_iter(&self) -> impl Iterator<Item = &CHIP_INFO_SOURCE_MEM> {
        self.chip_info_source_mem.iter()
    }
    #[doc = "0x84 - RMA nonce memory."]
    #[inline(always)]
    pub const fn nonce_mem(&self, n: usize) -> &NONCE_MEM {
        &self.nonce_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x84 - RMA nonce memory."]
    #[inline(always)]
    pub fn nonce_mem_iter(&self) -> impl Iterator<Item = &NONCE_MEM> {
        self.nonce_mem.iter()
    }
    #[doc = "0x88 - RMA USC memory."]
    #[inline(always)]
    pub const fn usc_mem(&self, n: usize) -> &USC_MEM {
        &self.usc_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x88 - RMA USC memory."]
    #[inline(always)]
    pub fn usc_mem_iter(&self) -> impl Iterator<Item = &USC_MEM> {
        self.usc_mem.iter()
    }
    #[doc = "0x8c - RMA public key memory ."]
    #[inline(always)]
    pub const fn public_key_mem(&self, n: usize) -> &PUBLIC_KEY_MEM {
        &self.public_key_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x8c - RMA public key memory ."]
    #[inline(always)]
    pub fn public_key_mem_iter(&self) -> impl Iterator<Item = &PUBLIC_KEY_MEM> {
        self.public_key_mem.iter()
    }
    #[doc = "0x90 - RMA signature r memory ."]
    #[inline(always)]
    pub const fn sign_mem(&self, n: usize) -> &SIGN_MEM {
        &self.sign_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x90 - RMA signature r memory ."]
    #[inline(always)]
    pub fn sign_mem_iter(&self) -> impl Iterator<Item = &SIGN_MEM> {
        self.sign_mem.iter()
    }
    #[doc = "0x94 - RMA cert hash memory."]
    #[inline(always)]
    pub const fn cert_hash_mem(&self, n: usize) -> &CERT_HASH_MEM {
        &self.cert_hash_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x94 - RMA cert hash memory."]
    #[inline(always)]
    pub fn cert_hash_mem_iter(&self) -> impl Iterator<Item = &CERT_HASH_MEM> {
        self.cert_hash_mem.iter()
    }
    #[doc = "0x98 - RMA chip info memory ."]
    #[inline(always)]
    pub const fn chip_info_mem(&self, n: usize) -> &CHIP_INFO_MEM {
        &self.chip_info_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x98 - RMA chip info memory ."]
    #[inline(always)]
    pub fn chip_info_mem_iter(&self) -> impl Iterator<Item = &CHIP_INFO_MEM> {
        self.chip_info_mem.iter()
    }
}
#[doc = "CONFIG (rw) register accessor: Configures RMA algorithm\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configures RMA algorithm"]
pub mod config;
#[doc = "TRIGGER (w) register accessor: Starts the RMA module.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigger::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trigger`] module"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "Starts the RMA module."]
pub mod trigger;
#[doc = "STATE (r) register accessor: query state in rma\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`] module"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "query state in rma"]
pub mod state;
#[doc = "LOG (r) register accessor: Query result in rma\n\nYou can [`read`](crate::Reg::read) this register and get [`log::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log`] module"]
pub type LOG = crate::Reg<log::LOG_SPEC>;
#[doc = "Query result in rma"]
pub mod log;
#[doc = "INT_RAW (r) register accessor: RMA interrupt raw register, valid in level.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "RMA interrupt raw register, valid in level."]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: RMA interrupt status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "RMA interrupt status register."]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: RMA interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "RMA interrupt enable register."]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: RMA interrupt clear register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "RMA interrupt clear register."]
pub mod int_clr;
#[doc = "RESULT_ST (r) register accessor: RMA result reg.\n\nYou can [`read`](crate::Reg::read) this register and get [`result_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result_st`] module"]
pub type RESULT_ST = crate::Reg<result_st::RESULT_ST_SPEC>;
#[doc = "RMA result reg."]
pub mod result_st;
#[doc = "RESULT_CLR (w) register accessor: RMA clr result reg.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`result_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result_clr`] module"]
pub type RESULT_CLR = crate::Reg<result_clr::RESULT_CLR_SPEC>;
#[doc = "RMA clr result reg."]
pub mod result_clr;
#[doc = "TIMEOUT_LIMIT (rw) register accessor: RMA timeout limit configurate register\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout_limit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout_limit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeout_limit`] module"]
pub type TIMEOUT_LIMIT = crate::Reg<timeout_limit::TIMEOUT_LIMIT_SPEC>;
#[doc = "RMA timeout limit configurate register"]
pub mod timeout_limit;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
#[doc = "CHIP_INFO_SOURCE_MEM (rw) register accessor: RMA chip info memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`chip_info_source_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chip_info_source_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chip_info_source_mem`] module"]
pub type CHIP_INFO_SOURCE_MEM = crate::Reg<chip_info_source_mem::CHIP_INFO_SOURCE_MEM_SPEC>;
#[doc = "RMA chip info memory."]
pub mod chip_info_source_mem;
#[doc = "NONCE_MEM (rw) register accessor: RMA nonce memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`nonce_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nonce_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nonce_mem`] module"]
pub type NONCE_MEM = crate::Reg<nonce_mem::NONCE_MEM_SPEC>;
#[doc = "RMA nonce memory."]
pub mod nonce_mem;
#[doc = "USC_MEM (rw) register accessor: RMA USC memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`usc_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usc_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usc_mem`] module"]
pub type USC_MEM = crate::Reg<usc_mem::USC_MEM_SPEC>;
#[doc = "RMA USC memory."]
pub mod usc_mem;
#[doc = "PUBLIC_KEY_MEM (rw) register accessor: RMA public key memory .\n\nYou can [`read`](crate::Reg::read) this register and get [`public_key_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`public_key_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@public_key_mem`] module"]
pub type PUBLIC_KEY_MEM = crate::Reg<public_key_mem::PUBLIC_KEY_MEM_SPEC>;
#[doc = "RMA public key memory ."]
pub mod public_key_mem;
#[doc = "SIGN_MEM (rw) register accessor: RMA signature r memory .\n\nYou can [`read`](crate::Reg::read) this register and get [`sign_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sign_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sign_mem`] module"]
pub type SIGN_MEM = crate::Reg<sign_mem::SIGN_MEM_SPEC>;
#[doc = "RMA signature r memory ."]
pub mod sign_mem;
#[doc = "CERT_HASH_MEM (rw) register accessor: RMA cert hash memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`cert_hash_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cert_hash_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cert_hash_mem`] module"]
pub type CERT_HASH_MEM = crate::Reg<cert_hash_mem::CERT_HASH_MEM_SPEC>;
#[doc = "RMA cert hash memory."]
pub mod cert_hash_mem;
#[doc = "CHIP_INFO_MEM (rw) register accessor: RMA chip info memory .\n\nYou can [`read`](crate::Reg::read) this register and get [`chip_info_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chip_info_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chip_info_mem`] module"]
pub type CHIP_INFO_MEM = crate::Reg<chip_info_mem::CHIP_INFO_MEM_SPEC>;
#[doc = "RMA chip info memory ."]
pub mod chip_info_mem;
