#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    clk: CLK,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    static_: STATIC,
    lock: LOCK,
    conf: CONF,
    start: START,
    state: STATE,
    result: RESULT,
    key_vld: KEY_VLD,
    huk_vld: HUK_VLD,
    _reserved13: [u8; 0xc4],
    date: DATE,
    assist_info_mem: [ASSIST_INFO_MEM; 64],
    public_info_mem: [PUBLIC_INFO_MEM; 64],
    sw_init_key_mem: [SW_INIT_KEY_MEM; 32],
}
impl RegisterBlock {
    #[doc = "0x04 - Key Manager clock gate control register"]
    #[inline(always)]
    pub const fn clk(&self) -> &CLK {
        &self.clk
    }
    #[doc = "0x08 - Key Manager interrupt raw register, valid in level."]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x0c - Key Manager interrupt status register."]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x10 - Key Manager interrupt enable register."]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x14 - Key Manager interrupt clear register."]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x18 - Key Manager static configuration register"]
    #[inline(always)]
    pub const fn static_(&self) -> &STATIC {
        &self.static_
    }
    #[doc = "0x1c - Key Manager static configuration locker register"]
    #[inline(always)]
    pub const fn lock(&self) -> &LOCK {
        &self.lock
    }
    #[doc = "0x20 - Key Manager configuration register"]
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    #[doc = "0x24 - Key Manager control register"]
    #[inline(always)]
    pub const fn start(&self) -> &START {
        &self.start
    }
    #[doc = "0x28 - Key Manager state register"]
    #[inline(always)]
    pub const fn state(&self) -> &STATE {
        &self.state
    }
    #[doc = "0x2c - Key Manager operation result register"]
    #[inline(always)]
    pub const fn result(&self) -> &RESULT {
        &self.result
    }
    #[doc = "0x30 - Key Manager key status register"]
    #[inline(always)]
    pub const fn key_vld(&self) -> &KEY_VLD {
        &self.key_vld
    }
    #[doc = "0x34 - Key Manager HUK status register"]
    #[inline(always)]
    pub const fn huk_vld(&self) -> &HUK_VLD {
        &self.huk_vld
    }
    #[doc = "0xfc - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    #[doc = "0x100..0x140 - The memory that stores assist key info."]
    #[inline(always)]
    pub const fn assist_info_mem(&self, n: usize) -> &ASSIST_INFO_MEM {
        &self.assist_info_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x140 - The memory that stores assist key info."]
    #[inline(always)]
    pub fn assist_info_mem_iter(&self) -> impl Iterator<Item = &ASSIST_INFO_MEM> {
        self.assist_info_mem.iter()
    }
    #[doc = "0x140..0x180 - The memory that stores public key info."]
    #[inline(always)]
    pub const fn public_info_mem(&self, n: usize) -> &PUBLIC_INFO_MEM {
        &self.public_info_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x180 - The memory that stores public key info."]
    #[inline(always)]
    pub fn public_info_mem_iter(&self) -> impl Iterator<Item = &PUBLIC_INFO_MEM> {
        self.public_info_mem.iter()
    }
    #[doc = "0x180..0x1a0 - The memory that stores software written init key."]
    #[inline(always)]
    pub const fn sw_init_key_mem(&self, n: usize) -> &SW_INIT_KEY_MEM {
        &self.sw_init_key_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x180..0x1a0 - The memory that stores software written init key."]
    #[inline(always)]
    pub fn sw_init_key_mem_iter(&self) -> impl Iterator<Item = &SW_INIT_KEY_MEM> {
        self.sw_init_key_mem.iter()
    }
}
#[doc = "CLK (rw) register accessor: Key Manager clock gate control register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk`] module"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = "Key Manager clock gate control register"]
pub mod clk;
#[doc = "INT_RAW (r) register accessor: Key Manager interrupt raw register, valid in level.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Key Manager interrupt raw register, valid in level."]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: Key Manager interrupt status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Key Manager interrupt status register."]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: Key Manager interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Key Manager interrupt enable register."]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: Key Manager interrupt clear register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Key Manager interrupt clear register."]
pub mod int_clr;
#[doc = "STATIC (rw) register accessor: Key Manager static configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`static_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`static_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@static_`] module"]
pub type STATIC = crate::Reg<static_::STATIC_SPEC>;
#[doc = "Key Manager static configuration register"]
pub mod static_;
#[doc = "LOCK (rw) register accessor: Key Manager static configuration locker register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Key Manager static configuration locker register"]
pub mod lock;
#[doc = "CONF (rw) register accessor: Key Manager configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`] module"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "Key Manager configuration register"]
pub mod conf;
#[doc = "START (w) register accessor: Key Manager control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`] module"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = "Key Manager control register"]
pub mod start;
#[doc = "STATE (r) register accessor: Key Manager state register\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`] module"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "Key Manager state register"]
pub mod state;
#[doc = "RESULT (r) register accessor: Key Manager operation result register\n\nYou can [`read`](crate::Reg::read) this register and get [`result::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result`] module"]
pub type RESULT = crate::Reg<result::RESULT_SPEC>;
#[doc = "Key Manager operation result register"]
pub mod result;
#[doc = "KEY_VLD (r) register accessor: Key Manager key status register\n\nYou can [`read`](crate::Reg::read) this register and get [`key_vld::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_vld`] module"]
pub type KEY_VLD = crate::Reg<key_vld::KEY_VLD_SPEC>;
#[doc = "Key Manager key status register"]
pub mod key_vld;
#[doc = "HUK_VLD (r) register accessor: Key Manager HUK status register\n\nYou can [`read`](crate::Reg::read) this register and get [`huk_vld::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@huk_vld`] module"]
pub type HUK_VLD = crate::Reg<huk_vld::HUK_VLD_SPEC>;
#[doc = "Key Manager HUK status register"]
pub mod huk_vld;
pub use crate::aes::date;
pub use crate::aes::DATE;
#[doc = "ASSIST_INFO_MEM (rw) register accessor: The memory that stores assist key info.\n\nYou can [`read`](crate::Reg::read) this register and get [`assist_info_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`assist_info_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@assist_info_mem`] module"]
pub type ASSIST_INFO_MEM = crate::Reg<assist_info_mem::ASSIST_INFO_MEM_SPEC>;
#[doc = "The memory that stores assist key info."]
pub mod assist_info_mem;
#[doc = "PUBLIC_INFO_MEM (rw) register accessor: The memory that stores public key info.\n\nYou can [`read`](crate::Reg::read) this register and get [`public_info_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`public_info_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@public_info_mem`] module"]
pub type PUBLIC_INFO_MEM = crate::Reg<public_info_mem::PUBLIC_INFO_MEM_SPEC>;
#[doc = "The memory that stores public key info."]
pub mod public_info_mem;
#[doc = "SW_INIT_KEY_MEM (rw) register accessor: The memory that stores software written init key.\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_init_key_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_init_key_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_init_key_mem`] module"]
pub type SW_INIT_KEY_MEM = crate::Reg<sw_init_key_mem::SW_INIT_KEY_MEM_SPEC>;
#[doc = "The memory that stores software written init key."]
pub mod sw_init_key_mem;
