#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    conf: CONF,
    clk: CLK,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    start: START,
    state: STATE,
    result: RESULT,
    timeout_limit: TIMEOUT_LIMIT,
    free: FREE,
    _reserved11: [u8; 0x10],
    key_0: KEY_0,
    key_1: KEY_1,
    key_2: KEY_2,
    key_3: KEY_3,
    key_4: KEY_4,
    key_5: KEY_5,
    key_6: KEY_6,
    key_7: KEY_7,
    key_8: KEY_8,
    key_9: KEY_9,
    key_10: KEY_10,
    key_11: KEY_11,
    _reserved23: [u8; 0x8c],
    date: DATE,
    _reserved24: [u8; 0x0100],
    sha_mode: SHA_MODE,
    _reserved25: [u8; 0x0c],
    sha_start: SHA_START,
    sha_continue: SHA_CONTINUE,
    sha_busy: SHA_BUSY,
    _reserved28: [u8; 0x64],
    message_mem: [MESSAGE_MEM; 64],
    _reserved29: [u8; 0x0120],
    r_mem: [R_MEM; 48],
    s_mem: [S_MEM; 48],
    z_mem: [Z_MEM; 48],
    qax_mem: [QAX_MEM; 48],
    qay_mem: [QAY_MEM; 48],
}
impl RegisterBlock {
    #[doc = "0x04 - ECDSA configure register"]
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    #[doc = "0x08 - ECDSA clock gate register"]
    #[inline(always)]
    pub const fn clk(&self) -> &CLK {
        &self.clk
    }
    #[doc = "0x0c - ECDSA interrupt raw register, valid in level."]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x10 - ECDSA interrupt status register."]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x14 - ECDSA interrupt enable register."]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x18 - ECDSA interrupt clear register."]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x1c - ECDSA start register"]
    #[inline(always)]
    pub const fn start(&self) -> &START {
        &self.start
    }
    #[doc = "0x20 - ECDSA status register"]
    #[inline(always)]
    pub const fn state(&self) -> &STATE {
        &self.state
    }
    #[doc = "0x24 - ECDSA result register"]
    #[inline(always)]
    pub const fn result(&self) -> &RESULT {
        &self.result
    }
    #[doc = "0x28 - ECDSA timeout limit configurate register"]
    #[inline(always)]
    pub const fn timeout_limit(&self) -> &TIMEOUT_LIMIT {
        &self.timeout_limit
    }
    #[doc = "0x2c - ECDSA free state"]
    #[inline(always)]
    pub const fn free(&self) -> &FREE {
        &self.free
    }
    #[doc = "0x40 - ECDSA key data register 0"]
    #[inline(always)]
    pub const fn key_0(&self) -> &KEY_0 {
        &self.key_0
    }
    #[doc = "0x44 - ECDSA key data register 1"]
    #[inline(always)]
    pub const fn key_1(&self) -> &KEY_1 {
        &self.key_1
    }
    #[doc = "0x48 - ECDSA key data register 2"]
    #[inline(always)]
    pub const fn key_2(&self) -> &KEY_2 {
        &self.key_2
    }
    #[doc = "0x4c - ECDSA key data register 3"]
    #[inline(always)]
    pub const fn key_3(&self) -> &KEY_3 {
        &self.key_3
    }
    #[doc = "0x50 - ECDSA key data register 4"]
    #[inline(always)]
    pub const fn key_4(&self) -> &KEY_4 {
        &self.key_4
    }
    #[doc = "0x54 - ECDSA key data register 5"]
    #[inline(always)]
    pub const fn key_5(&self) -> &KEY_5 {
        &self.key_5
    }
    #[doc = "0x58 - ECDSA key data register 6"]
    #[inline(always)]
    pub const fn key_6(&self) -> &KEY_6 {
        &self.key_6
    }
    #[doc = "0x5c - ECDSA key data register 7"]
    #[inline(always)]
    pub const fn key_7(&self) -> &KEY_7 {
        &self.key_7
    }
    #[doc = "0x60 - ECDSA key data register 8"]
    #[inline(always)]
    pub const fn key_8(&self) -> &KEY_8 {
        &self.key_8
    }
    #[doc = "0x64 - ECDSA key data register 9"]
    #[inline(always)]
    pub const fn key_9(&self) -> &KEY_9 {
        &self.key_9
    }
    #[doc = "0x68 - ECDSA key data register 10"]
    #[inline(always)]
    pub const fn key_10(&self) -> &KEY_10 {
        &self.key_10
    }
    #[doc = "0x6c - ECDSA key data register 11"]
    #[inline(always)]
    pub const fn key_11(&self) -> &KEY_11 {
        &self.key_11
    }
    #[doc = "0xfc - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    #[doc = "0x200 - ECDSA control SHA register"]
    #[inline(always)]
    pub const fn sha_mode(&self) -> &SHA_MODE {
        &self.sha_mode
    }
    #[doc = "0x210 - ECDSA control SHA register"]
    #[inline(always)]
    pub const fn sha_start(&self) -> &SHA_START {
        &self.sha_start
    }
    #[doc = "0x214 - ECDSA control SHA register"]
    #[inline(always)]
    pub const fn sha_continue(&self) -> &SHA_CONTINUE {
        &self.sha_continue
    }
    #[doc = "0x218 - ECDSA status register"]
    #[inline(always)]
    pub const fn sha_busy(&self) -> &SHA_BUSY {
        &self.sha_busy
    }
    #[doc = "0x280..0x2c0 - The memory that stores message."]
    #[inline(always)]
    pub const fn message_mem(&self, n: usize) -> &MESSAGE_MEM {
        &self.message_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x280..0x2c0 - The memory that stores message."]
    #[inline(always)]
    pub fn message_mem_iter(&self) -> impl Iterator<Item = &MESSAGE_MEM> {
        self.message_mem.iter()
    }
    #[doc = "0x3e0..0x410 - The memory that stores r."]
    #[inline(always)]
    pub const fn r_mem(&self, n: usize) -> &R_MEM {
        &self.r_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3e0..0x410 - The memory that stores r."]
    #[inline(always)]
    pub fn r_mem_iter(&self) -> impl Iterator<Item = &R_MEM> {
        self.r_mem.iter()
    }
    #[doc = "0x410..0x440 - The memory that stores s."]
    #[inline(always)]
    pub const fn s_mem(&self, n: usize) -> &S_MEM {
        &self.s_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x410..0x440 - The memory that stores s."]
    #[inline(always)]
    pub fn s_mem_iter(&self) -> impl Iterator<Item = &S_MEM> {
        self.s_mem.iter()
    }
    #[doc = "0x440..0x470 - The memory that stores software written z."]
    #[inline(always)]
    pub const fn z_mem(&self, n: usize) -> &Z_MEM {
        &self.z_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x440..0x470 - The memory that stores software written z."]
    #[inline(always)]
    pub fn z_mem_iter(&self) -> impl Iterator<Item = &Z_MEM> {
        self.z_mem.iter()
    }
    #[doc = "0x470..0x4a0 - The memory that stores x coordinates of QA or software written k."]
    #[inline(always)]
    pub const fn qax_mem(&self, n: usize) -> &QAX_MEM {
        &self.qax_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x470..0x4a0 - The memory that stores x coordinates of QA or software written k."]
    #[inline(always)]
    pub fn qax_mem_iter(&self) -> impl Iterator<Item = &QAX_MEM> {
        self.qax_mem.iter()
    }
    #[doc = "0x4a0..0x4d0 - The memory that stores y coordinates of QA."]
    #[inline(always)]
    pub const fn qay_mem(&self, n: usize) -> &QAY_MEM {
        &self.qay_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x4a0..0x4d0 - The memory that stores y coordinates of QA."]
    #[inline(always)]
    pub fn qay_mem_iter(&self) -> impl Iterator<Item = &QAY_MEM> {
        self.qay_mem.iter()
    }
}
#[doc = "CONF (rw) register accessor: ECDSA configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`] module"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "ECDSA configure register"]
pub mod conf;
#[doc = "CLK (rw) register accessor: ECDSA clock gate register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk`] module"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = "ECDSA clock gate register"]
pub mod clk;
#[doc = "INT_RAW (r) register accessor: ECDSA interrupt raw register, valid in level.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "ECDSA interrupt raw register, valid in level."]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: ECDSA interrupt status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "ECDSA interrupt status register."]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: ECDSA interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "ECDSA interrupt enable register."]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: ECDSA interrupt clear register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "ECDSA interrupt clear register."]
pub mod int_clr;
#[doc = "START (w) register accessor: ECDSA start register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`] module"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = "ECDSA start register"]
pub mod start;
#[doc = "STATE (r) register accessor: ECDSA status register\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`] module"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "ECDSA status register"]
pub mod state;
#[doc = "RESULT (r) register accessor: ECDSA result register\n\nYou can [`read`](crate::Reg::read) this register and get [`result::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result`] module"]
pub type RESULT = crate::Reg<result::RESULT_SPEC>;
#[doc = "ECDSA result register"]
pub mod result;
#[doc = "TIMEOUT_LIMIT (rw) register accessor: ECDSA timeout limit configurate register\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout_limit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout_limit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeout_limit`] module"]
pub type TIMEOUT_LIMIT = crate::Reg<timeout_limit::TIMEOUT_LIMIT_SPEC>;
#[doc = "ECDSA timeout limit configurate register"]
pub mod timeout_limit;
#[doc = "FREE (rw) register accessor: ECDSA free state\n\nYou can [`read`](crate::Reg::read) this register and get [`free::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`free::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@free`] module"]
pub type FREE = crate::Reg<free::FREE_SPEC>;
#[doc = "ECDSA free state"]
pub mod free;
#[doc = "KEY_0 (rw) register accessor: ECDSA key data register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`key_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_0`] module"]
pub type KEY_0 = crate::Reg<key_0::KEY_0_SPEC>;
#[doc = "ECDSA key data register 0"]
pub mod key_0;
#[doc = "KEY_1 (rw) register accessor: ECDSA key data register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`key_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_1`] module"]
pub type KEY_1 = crate::Reg<key_1::KEY_1_SPEC>;
#[doc = "ECDSA key data register 1"]
pub mod key_1;
#[doc = "KEY_2 (rw) register accessor: ECDSA key data register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`key_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_2`] module"]
pub type KEY_2 = crate::Reg<key_2::KEY_2_SPEC>;
#[doc = "ECDSA key data register 2"]
pub mod key_2;
#[doc = "KEY_3 (rw) register accessor: ECDSA key data register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`key_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_3`] module"]
pub type KEY_3 = crate::Reg<key_3::KEY_3_SPEC>;
#[doc = "ECDSA key data register 3"]
pub mod key_3;
#[doc = "KEY_4 (rw) register accessor: ECDSA key data register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`key_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_4`] module"]
pub type KEY_4 = crate::Reg<key_4::KEY_4_SPEC>;
#[doc = "ECDSA key data register 4"]
pub mod key_4;
#[doc = "KEY_5 (rw) register accessor: ECDSA key data register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`key_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_5`] module"]
pub type KEY_5 = crate::Reg<key_5::KEY_5_SPEC>;
#[doc = "ECDSA key data register 5"]
pub mod key_5;
#[doc = "KEY_6 (rw) register accessor: ECDSA key data register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`key_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_6`] module"]
pub type KEY_6 = crate::Reg<key_6::KEY_6_SPEC>;
#[doc = "ECDSA key data register 6"]
pub mod key_6;
#[doc = "KEY_7 (rw) register accessor: ECDSA key data register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`key_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_7`] module"]
pub type KEY_7 = crate::Reg<key_7::KEY_7_SPEC>;
#[doc = "ECDSA key data register 7"]
pub mod key_7;
#[doc = "KEY_8 (rw) register accessor: ECDSA key data register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`key_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_8`] module"]
pub type KEY_8 = crate::Reg<key_8::KEY_8_SPEC>;
#[doc = "ECDSA key data register 8"]
pub mod key_8;
#[doc = "KEY_9 (rw) register accessor: ECDSA key data register 9\n\nYou can [`read`](crate::Reg::read) this register and get [`key_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_9`] module"]
pub type KEY_9 = crate::Reg<key_9::KEY_9_SPEC>;
#[doc = "ECDSA key data register 9"]
pub mod key_9;
#[doc = "KEY_10 (rw) register accessor: ECDSA key data register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`key_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_10`] module"]
pub type KEY_10 = crate::Reg<key_10::KEY_10_SPEC>;
#[doc = "ECDSA key data register 10"]
pub mod key_10;
#[doc = "KEY_11 (rw) register accessor: ECDSA key data register 11\n\nYou can [`read`](crate::Reg::read) this register and get [`key_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_11`] module"]
pub type KEY_11 = crate::Reg<key_11::KEY_11_SPEC>;
#[doc = "ECDSA key data register 11"]
pub mod key_11;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
#[doc = "SHA_MODE (rw) register accessor: ECDSA control SHA register\n\nYou can [`read`](crate::Reg::read) this register and get [`sha_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sha_mode`] module"]
pub type SHA_MODE = crate::Reg<sha_mode::SHA_MODE_SPEC>;
#[doc = "ECDSA control SHA register"]
pub mod sha_mode;
#[doc = "SHA_START (w) register accessor: ECDSA control SHA register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sha_start`] module"]
pub type SHA_START = crate::Reg<sha_start::SHA_START_SPEC>;
#[doc = "ECDSA control SHA register"]
pub mod sha_start;
#[doc = "SHA_CONTINUE (w) register accessor: ECDSA control SHA register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_continue::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sha_continue`] module"]
pub type SHA_CONTINUE = crate::Reg<sha_continue::SHA_CONTINUE_SPEC>;
#[doc = "ECDSA control SHA register"]
pub mod sha_continue;
#[doc = "SHA_BUSY (r) register accessor: ECDSA status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sha_busy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sha_busy`] module"]
pub type SHA_BUSY = crate::Reg<sha_busy::SHA_BUSY_SPEC>;
#[doc = "ECDSA status register"]
pub mod sha_busy;
#[doc = "MESSAGE_MEM (rw) register accessor: The memory that stores message.\n\nYou can [`read`](crate::Reg::read) this register and get [`message_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`message_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@message_mem`] module"]
pub type MESSAGE_MEM = crate::Reg<message_mem::MESSAGE_MEM_SPEC>;
#[doc = "The memory that stores message."]
pub mod message_mem;
#[doc = "R_MEM (rw) register accessor: The memory that stores r.\n\nYou can [`read`](crate::Reg::read) this register and get [`r_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_mem`] module"]
pub type R_MEM = crate::Reg<r_mem::R_MEM_SPEC>;
#[doc = "The memory that stores r."]
pub mod r_mem;
#[doc = "S_MEM (rw) register accessor: The memory that stores s.\n\nYou can [`read`](crate::Reg::read) this register and get [`s_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s_mem`] module"]
pub type S_MEM = crate::Reg<s_mem::S_MEM_SPEC>;
#[doc = "The memory that stores s."]
pub mod s_mem;
#[doc = "Z_MEM (rw) register accessor: The memory that stores software written z.\n\nYou can [`read`](crate::Reg::read) this register and get [`z_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`z_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@z_mem`] module"]
pub type Z_MEM = crate::Reg<z_mem::Z_MEM_SPEC>;
#[doc = "The memory that stores software written z."]
pub mod z_mem;
#[doc = "QAX_MEM (rw) register accessor: The memory that stores x coordinates of QA or software written k.\n\nYou can [`read`](crate::Reg::read) this register and get [`qax_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qax_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qax_mem`] module"]
pub type QAX_MEM = crate::Reg<qax_mem::QAX_MEM_SPEC>;
#[doc = "The memory that stores x coordinates of QA or software written k."]
pub mod qax_mem;
#[doc = "QAY_MEM (rw) register accessor: The memory that stores y coordinates of QA.\n\nYou can [`read`](crate::Reg::read) this register and get [`qay_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qay_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qay_mem`] module"]
pub type QAY_MEM = crate::Reg<qay_mem::QAY_MEM_SPEC>;
#[doc = "The memory that stores y coordinates of QA."]
pub mod qay_mem;
