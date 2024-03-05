#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    cmd: CMD,
    addr: ADDR,
    ctrl: CTRL,
    clock: CLOCK,
    user: USER,
    user1: USER1,
    user2: USER2,
    ms_dlen: MS_DLEN,
    misc: MISC,
    din_mode: DIN_MODE,
    din_num: DIN_NUM,
    dout_mode: DOUT_MODE,
    dma_conf: DMA_CONF,
    dma_int_ena: DMA_INT_ENA,
    dma_int_clr: DMA_INT_CLR,
    dma_int_raw: DMA_INT_RAW,
    dma_int_st: DMA_INT_ST,
    _reserved17: [u8; 0x54],
    w0: W0,
    w1: W1,
    w2: W2,
    w3: W3,
    w4: W4,
    w5: W5,
    w6: W6,
    w7: W7,
    w8: W8,
    w9: W9,
    w10: W10,
    w11: W11,
    w12: W12,
    w13: W13,
    w14: W14,
    w15: W15,
    _reserved33: [u8; 0x08],
    slave: SLAVE,
    slave1: SLAVE1,
    clk_gate: CLK_GATE,
    _reserved36: [u8; 0x04],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - Command control register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x04 - Address value register"]
    #[inline(always)]
    pub const fn addr(&self) -> &ADDR {
        &self.addr
    }
    #[doc = "0x08 - SPI control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x0c - SPI clock control register"]
    #[inline(always)]
    pub const fn clock(&self) -> &CLOCK {
        &self.clock
    }
    #[doc = "0x10 - SPI USER control register"]
    #[inline(always)]
    pub const fn user(&self) -> &USER {
        &self.user
    }
    #[doc = "0x14 - SPI USER control register 1"]
    #[inline(always)]
    pub const fn user1(&self) -> &USER1 {
        &self.user1
    }
    #[doc = "0x18 - SPI USER control register 2"]
    #[inline(always)]
    pub const fn user2(&self) -> &USER2 {
        &self.user2
    }
    #[doc = "0x1c - SPI data bit length control register"]
    #[inline(always)]
    pub const fn ms_dlen(&self) -> &MS_DLEN {
        &self.ms_dlen
    }
    #[doc = "0x20 - SPI misc register"]
    #[inline(always)]
    pub const fn misc(&self) -> &MISC {
        &self.misc
    }
    #[doc = "0x24 - SPI input delay mode configuration"]
    #[inline(always)]
    pub const fn din_mode(&self) -> &DIN_MODE {
        &self.din_mode
    }
    #[doc = "0x28 - SPI input delay number configuration"]
    #[inline(always)]
    pub const fn din_num(&self) -> &DIN_NUM {
        &self.din_num
    }
    #[doc = "0x2c - SPI output delay mode configuration"]
    #[inline(always)]
    pub const fn dout_mode(&self) -> &DOUT_MODE {
        &self.dout_mode
    }
    #[doc = "0x30 - SPI DMA control register"]
    #[inline(always)]
    pub const fn dma_conf(&self) -> &DMA_CONF {
        &self.dma_conf
    }
    #[doc = "0x34 - SPI DMA interrupt enable register"]
    #[inline(always)]
    pub const fn dma_int_ena(&self) -> &DMA_INT_ENA {
        &self.dma_int_ena
    }
    #[doc = "0x38 - SPI DMA interrupt clear register"]
    #[inline(always)]
    pub const fn dma_int_clr(&self) -> &DMA_INT_CLR {
        &self.dma_int_clr
    }
    #[doc = "0x3c - SPI DMA interrupt raw register"]
    #[inline(always)]
    pub const fn dma_int_raw(&self) -> &DMA_INT_RAW {
        &self.dma_int_raw
    }
    #[doc = "0x40 - SPI DMA interrupt status register"]
    #[inline(always)]
    pub const fn dma_int_st(&self) -> &DMA_INT_ST {
        &self.dma_int_st
    }
    #[doc = "0x98 - SPI CPU-controlled buffer0"]
    #[inline(always)]
    pub const fn w0(&self) -> &W0 {
        &self.w0
    }
    #[doc = "0x9c - SPI CPU-controlled buffer1"]
    #[inline(always)]
    pub const fn w1(&self) -> &W1 {
        &self.w1
    }
    #[doc = "0xa0 - SPI CPU-controlled buffer2"]
    #[inline(always)]
    pub const fn w2(&self) -> &W2 {
        &self.w2
    }
    #[doc = "0xa4 - SPI CPU-controlled buffer3"]
    #[inline(always)]
    pub const fn w3(&self) -> &W3 {
        &self.w3
    }
    #[doc = "0xa8 - SPI CPU-controlled buffer4"]
    #[inline(always)]
    pub const fn w4(&self) -> &W4 {
        &self.w4
    }
    #[doc = "0xac - SPI CPU-controlled buffer5"]
    #[inline(always)]
    pub const fn w5(&self) -> &W5 {
        &self.w5
    }
    #[doc = "0xb0 - SPI CPU-controlled buffer6"]
    #[inline(always)]
    pub const fn w6(&self) -> &W6 {
        &self.w6
    }
    #[doc = "0xb4 - SPI CPU-controlled buffer7"]
    #[inline(always)]
    pub const fn w7(&self) -> &W7 {
        &self.w7
    }
    #[doc = "0xb8 - SPI CPU-controlled buffer8"]
    #[inline(always)]
    pub const fn w8(&self) -> &W8 {
        &self.w8
    }
    #[doc = "0xbc - SPI CPU-controlled buffer9"]
    #[inline(always)]
    pub const fn w9(&self) -> &W9 {
        &self.w9
    }
    #[doc = "0xc0 - SPI CPU-controlled buffer10"]
    #[inline(always)]
    pub const fn w10(&self) -> &W10 {
        &self.w10
    }
    #[doc = "0xc4 - SPI CPU-controlled buffer11"]
    #[inline(always)]
    pub const fn w11(&self) -> &W11 {
        &self.w11
    }
    #[doc = "0xc8 - SPI CPU-controlled buffer12"]
    #[inline(always)]
    pub const fn w12(&self) -> &W12 {
        &self.w12
    }
    #[doc = "0xcc - SPI CPU-controlled buffer13"]
    #[inline(always)]
    pub const fn w13(&self) -> &W13 {
        &self.w13
    }
    #[doc = "0xd0 - SPI CPU-controlled buffer14"]
    #[inline(always)]
    pub const fn w14(&self) -> &W14 {
        &self.w14
    }
    #[doc = "0xd4 - SPI CPU-controlled buffer15"]
    #[inline(always)]
    pub const fn w15(&self) -> &W15 {
        &self.w15
    }
    #[doc = "0xe0 - SPI slave control register"]
    #[inline(always)]
    pub const fn slave(&self) -> &SLAVE {
        &self.slave
    }
    #[doc = "0xe4 - SPI slave control register 1"]
    #[inline(always)]
    pub const fn slave1(&self) -> &SLAVE1 {
        &self.slave1
    }
    #[doc = "0xe8 - SPI module clock and register clock control"]
    #[inline(always)]
    pub const fn clk_gate(&self) -> &CLK_GATE {
        &self.clk_gate
    }
    #[doc = "0xf0 - Version control"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CMD (rw) register accessor: Command control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command control register"]
pub mod cmd;
#[doc = "ADDR (rw) register accessor: Address value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`] module"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Address value register"]
pub mod addr;
#[doc = "CTRL (rw) register accessor: SPI control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "SPI control register"]
pub mod ctrl;
#[doc = "CLOCK (rw) register accessor: SPI clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock`] module"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = "SPI clock control register"]
pub mod clock;
#[doc = "USER (rw) register accessor: SPI USER control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`user::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user`] module"]
pub type USER = crate::Reg<user::USER_SPEC>;
#[doc = "SPI USER control register"]
pub mod user;
#[doc = "USER1 (rw) register accessor: SPI USER control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`user1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user1`] module"]
pub type USER1 = crate::Reg<user1::USER1_SPEC>;
#[doc = "SPI USER control register 1"]
pub mod user1;
#[doc = "USER2 (rw) register accessor: SPI USER control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`user2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user2`] module"]
pub type USER2 = crate::Reg<user2::USER2_SPEC>;
#[doc = "SPI USER control register 2"]
pub mod user2;
#[doc = "MS_DLEN (rw) register accessor: SPI data bit length control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms_dlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ms_dlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms_dlen`] module"]
pub type MS_DLEN = crate::Reg<ms_dlen::MS_DLEN_SPEC>;
#[doc = "SPI data bit length control register"]
pub mod ms_dlen;
#[doc = "MISC (rw) register accessor: SPI misc register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc`] module"]
pub type MISC = crate::Reg<misc::MISC_SPEC>;
#[doc = "SPI misc register"]
pub mod misc;
#[doc = "DIN_MODE (rw) register accessor: SPI input delay mode configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din_mode`] module"]
pub type DIN_MODE = crate::Reg<din_mode::DIN_MODE_SPEC>;
#[doc = "SPI input delay mode configuration"]
pub mod din_mode;
#[doc = "DIN_NUM (rw) register accessor: SPI input delay number configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din_num`] module"]
pub type DIN_NUM = crate::Reg<din_num::DIN_NUM_SPEC>;
#[doc = "SPI input delay number configuration"]
pub mod din_num;
#[doc = "DOUT_MODE (rw) register accessor: SPI output delay mode configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout_mode`] module"]
pub type DOUT_MODE = crate::Reg<dout_mode::DOUT_MODE_SPEC>;
#[doc = "SPI output delay mode configuration"]
pub mod dout_mode;
#[doc = "DMA_CONF (rw) register accessor: SPI DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_conf`] module"]
pub type DMA_CONF = crate::Reg<dma_conf::DMA_CONF_SPEC>;
#[doc = "SPI DMA control register"]
pub mod dma_conf;
#[doc = "DMA_INT_ENA (rw) register accessor: SPI DMA interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_ena`] module"]
pub type DMA_INT_ENA = crate::Reg<dma_int_ena::DMA_INT_ENA_SPEC>;
#[doc = "SPI DMA interrupt enable register"]
pub mod dma_int_ena;
#[doc = "DMA_INT_CLR (w) register accessor: SPI DMA interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_clr`] module"]
pub type DMA_INT_CLR = crate::Reg<dma_int_clr::DMA_INT_CLR_SPEC>;
#[doc = "SPI DMA interrupt clear register"]
pub mod dma_int_clr;
#[doc = "DMA_INT_RAW (rw) register accessor: SPI DMA interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_raw`] module"]
pub type DMA_INT_RAW = crate::Reg<dma_int_raw::DMA_INT_RAW_SPEC>;
#[doc = "SPI DMA interrupt raw register"]
pub mod dma_int_raw;
#[doc = "DMA_INT_ST (r) register accessor: SPI DMA interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_st`] module"]
pub type DMA_INT_ST = crate::Reg<dma_int_st::DMA_INT_ST_SPEC>;
#[doc = "SPI DMA interrupt status register"]
pub mod dma_int_st;
#[doc = "W0 (rw) register accessor: SPI CPU-controlled buffer0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0`] module"]
pub type W0 = crate::Reg<w0::W0_SPEC>;
#[doc = "SPI CPU-controlled buffer0"]
pub mod w0;
#[doc = "W1 (rw) register accessor: SPI CPU-controlled buffer1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w1`] module"]
pub type W1 = crate::Reg<w1::W1_SPEC>;
#[doc = "SPI CPU-controlled buffer1"]
pub mod w1;
#[doc = "W2 (rw) register accessor: SPI CPU-controlled buffer2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w2`] module"]
pub type W2 = crate::Reg<w2::W2_SPEC>;
#[doc = "SPI CPU-controlled buffer2"]
pub mod w2;
#[doc = "W3 (rw) register accessor: SPI CPU-controlled buffer3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w3`] module"]
pub type W3 = crate::Reg<w3::W3_SPEC>;
#[doc = "SPI CPU-controlled buffer3"]
pub mod w3;
#[doc = "W4 (rw) register accessor: SPI CPU-controlled buffer4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w4`] module"]
pub type W4 = crate::Reg<w4::W4_SPEC>;
#[doc = "SPI CPU-controlled buffer4"]
pub mod w4;
#[doc = "W5 (rw) register accessor: SPI CPU-controlled buffer5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w5`] module"]
pub type W5 = crate::Reg<w5::W5_SPEC>;
#[doc = "SPI CPU-controlled buffer5"]
pub mod w5;
#[doc = "W6 (rw) register accessor: SPI CPU-controlled buffer6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w6`] module"]
pub type W6 = crate::Reg<w6::W6_SPEC>;
#[doc = "SPI CPU-controlled buffer6"]
pub mod w6;
#[doc = "W7 (rw) register accessor: SPI CPU-controlled buffer7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w7`] module"]
pub type W7 = crate::Reg<w7::W7_SPEC>;
#[doc = "SPI CPU-controlled buffer7"]
pub mod w7;
#[doc = "W8 (rw) register accessor: SPI CPU-controlled buffer8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w8`] module"]
pub type W8 = crate::Reg<w8::W8_SPEC>;
#[doc = "SPI CPU-controlled buffer8"]
pub mod w8;
#[doc = "W9 (rw) register accessor: SPI CPU-controlled buffer9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w9`] module"]
pub type W9 = crate::Reg<w9::W9_SPEC>;
#[doc = "SPI CPU-controlled buffer9"]
pub mod w9;
#[doc = "W10 (rw) register accessor: SPI CPU-controlled buffer10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w10`] module"]
pub type W10 = crate::Reg<w10::W10_SPEC>;
#[doc = "SPI CPU-controlled buffer10"]
pub mod w10;
#[doc = "W11 (rw) register accessor: SPI CPU-controlled buffer11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w11`] module"]
pub type W11 = crate::Reg<w11::W11_SPEC>;
#[doc = "SPI CPU-controlled buffer11"]
pub mod w11;
#[doc = "W12 (rw) register accessor: SPI CPU-controlled buffer12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w12`] module"]
pub type W12 = crate::Reg<w12::W12_SPEC>;
#[doc = "SPI CPU-controlled buffer12"]
pub mod w12;
#[doc = "W13 (rw) register accessor: SPI CPU-controlled buffer13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w13`] module"]
pub type W13 = crate::Reg<w13::W13_SPEC>;
#[doc = "SPI CPU-controlled buffer13"]
pub mod w13;
#[doc = "W14 (rw) register accessor: SPI CPU-controlled buffer14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w14`] module"]
pub type W14 = crate::Reg<w14::W14_SPEC>;
#[doc = "SPI CPU-controlled buffer14"]
pub mod w14;
#[doc = "W15 (rw) register accessor: SPI CPU-controlled buffer15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w15`] module"]
pub type W15 = crate::Reg<w15::W15_SPEC>;
#[doc = "SPI CPU-controlled buffer15"]
pub mod w15;
#[doc = "SLAVE (rw) register accessor: SPI slave control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slave::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slave`] module"]
pub type SLAVE = crate::Reg<slave::SLAVE_SPEC>;
#[doc = "SPI slave control register"]
pub mod slave;
#[doc = "SLAVE1 (rw) register accessor: SPI slave control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slave1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slave1`] module"]
pub type SLAVE1 = crate::Reg<slave1::SLAVE1_SPEC>;
#[doc = "SPI slave control register 1"]
pub mod slave1;
#[doc = "CLK_GATE (rw) register accessor: SPI module clock and register clock control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gate`] module"]
pub type CLK_GATE = crate::Reg<clk_gate::CLK_GATE_SPEC>;
#[doc = "SPI module clock and register clock control"]
pub mod clk_gate;
#[doc = "DATE (rw) register accessor: Version control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control"]
pub mod date;
