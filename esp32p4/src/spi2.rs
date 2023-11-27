#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    spi_cmd: SPI_CMD,
    spi_addr: SPI_ADDR,
    spi_ctrl: SPI_CTRL,
    spi_clock: SPI_CLOCK,
    spi_user: SPI_USER,
    spi_user1: SPI_USER1,
    spi_user2: SPI_USER2,
    spi_ms_dlen: SPI_MS_DLEN,
    spi_misc: SPI_MISC,
    spi_din_mode: SPI_DIN_MODE,
    spi_din_num: SPI_DIN_NUM,
    spi_dout_mode: SPI_DOUT_MODE,
    spi_dma_conf: SPI_DMA_CONF,
    spi_dma_int_ena: SPI_DMA_INT_ENA,
    spi_dma_int_clr: SPI_DMA_INT_CLR,
    spi_dma_int_raw: SPI_DMA_INT_RAW,
    spi_dma_int_st: SPI_DMA_INT_ST,
    spi_dma_int_set: SPI_DMA_INT_SET,
    _reserved18: [u8; 0x50],
    spi_w0: SPI_W0,
    spi_w1: SPI_W1,
    spi_w2: SPI_W2,
    spi_w3: SPI_W3,
    spi_w4: SPI_W4,
    spi_w5: SPI_W5,
    spi_w6: SPI_W6,
    spi_w7: SPI_W7,
    spi_w8: SPI_W8,
    spi_w9: SPI_W9,
    spi_w10: SPI_W10,
    spi_w11: SPI_W11,
    spi_w12: SPI_W12,
    spi_w13: SPI_W13,
    spi_w14: SPI_W14,
    spi_w15: SPI_W15,
    _reserved34: [u8; 0x08],
    spi_slave: SPI_SLAVE,
    spi_slave1: SPI_SLAVE1,
    spi_clk_gate: SPI_CLK_GATE,
    _reserved37: [u8; 0x04],
    spi_date: SPI_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - Command control register"]
    #[inline(always)]
    pub const fn spi_cmd(&self) -> &SPI_CMD {
        &self.spi_cmd
    }
    #[doc = "0x04 - Address value register"]
    #[inline(always)]
    pub const fn spi_addr(&self) -> &SPI_ADDR {
        &self.spi_addr
    }
    #[doc = "0x08 - SPI control register"]
    #[inline(always)]
    pub const fn spi_ctrl(&self) -> &SPI_CTRL {
        &self.spi_ctrl
    }
    #[doc = "0x0c - SPI clock control register"]
    #[inline(always)]
    pub const fn spi_clock(&self) -> &SPI_CLOCK {
        &self.spi_clock
    }
    #[doc = "0x10 - SPI USER control register"]
    #[inline(always)]
    pub const fn spi_user(&self) -> &SPI_USER {
        &self.spi_user
    }
    #[doc = "0x14 - SPI USER control register 1"]
    #[inline(always)]
    pub const fn spi_user1(&self) -> &SPI_USER1 {
        &self.spi_user1
    }
    #[doc = "0x18 - SPI USER control register 2"]
    #[inline(always)]
    pub const fn spi_user2(&self) -> &SPI_USER2 {
        &self.spi_user2
    }
    #[doc = "0x1c - SPI data bit length control register"]
    #[inline(always)]
    pub const fn spi_ms_dlen(&self) -> &SPI_MS_DLEN {
        &self.spi_ms_dlen
    }
    #[doc = "0x20 - SPI misc register"]
    #[inline(always)]
    pub const fn spi_misc(&self) -> &SPI_MISC {
        &self.spi_misc
    }
    #[doc = "0x24 - SPI input delay mode configuration"]
    #[inline(always)]
    pub const fn spi_din_mode(&self) -> &SPI_DIN_MODE {
        &self.spi_din_mode
    }
    #[doc = "0x28 - SPI input delay number configuration"]
    #[inline(always)]
    pub const fn spi_din_num(&self) -> &SPI_DIN_NUM {
        &self.spi_din_num
    }
    #[doc = "0x2c - SPI output delay mode configuration"]
    #[inline(always)]
    pub const fn spi_dout_mode(&self) -> &SPI_DOUT_MODE {
        &self.spi_dout_mode
    }
    #[doc = "0x30 - SPI DMA control register"]
    #[inline(always)]
    pub const fn spi_dma_conf(&self) -> &SPI_DMA_CONF {
        &self.spi_dma_conf
    }
    #[doc = "0x34 - SPI interrupt enable register"]
    #[inline(always)]
    pub const fn spi_dma_int_ena(&self) -> &SPI_DMA_INT_ENA {
        &self.spi_dma_int_ena
    }
    #[doc = "0x38 - SPI interrupt clear register"]
    #[inline(always)]
    pub const fn spi_dma_int_clr(&self) -> &SPI_DMA_INT_CLR {
        &self.spi_dma_int_clr
    }
    #[doc = "0x3c - SPI interrupt raw register"]
    #[inline(always)]
    pub const fn spi_dma_int_raw(&self) -> &SPI_DMA_INT_RAW {
        &self.spi_dma_int_raw
    }
    #[doc = "0x40 - SPI interrupt status register"]
    #[inline(always)]
    pub const fn spi_dma_int_st(&self) -> &SPI_DMA_INT_ST {
        &self.spi_dma_int_st
    }
    #[doc = "0x44 - SPI interrupt software set register"]
    #[inline(always)]
    pub const fn spi_dma_int_set(&self) -> &SPI_DMA_INT_SET {
        &self.spi_dma_int_set
    }
    #[doc = "0x98 - SPI CPU-controlled buffer0"]
    #[inline(always)]
    pub const fn spi_w0(&self) -> &SPI_W0 {
        &self.spi_w0
    }
    #[doc = "0x9c - SPI CPU-controlled buffer1"]
    #[inline(always)]
    pub const fn spi_w1(&self) -> &SPI_W1 {
        &self.spi_w1
    }
    #[doc = "0xa0 - SPI CPU-controlled buffer2"]
    #[inline(always)]
    pub const fn spi_w2(&self) -> &SPI_W2 {
        &self.spi_w2
    }
    #[doc = "0xa4 - SPI CPU-controlled buffer3"]
    #[inline(always)]
    pub const fn spi_w3(&self) -> &SPI_W3 {
        &self.spi_w3
    }
    #[doc = "0xa8 - SPI CPU-controlled buffer4"]
    #[inline(always)]
    pub const fn spi_w4(&self) -> &SPI_W4 {
        &self.spi_w4
    }
    #[doc = "0xac - SPI CPU-controlled buffer5"]
    #[inline(always)]
    pub const fn spi_w5(&self) -> &SPI_W5 {
        &self.spi_w5
    }
    #[doc = "0xb0 - SPI CPU-controlled buffer6"]
    #[inline(always)]
    pub const fn spi_w6(&self) -> &SPI_W6 {
        &self.spi_w6
    }
    #[doc = "0xb4 - SPI CPU-controlled buffer7"]
    #[inline(always)]
    pub const fn spi_w7(&self) -> &SPI_W7 {
        &self.spi_w7
    }
    #[doc = "0xb8 - SPI CPU-controlled buffer8"]
    #[inline(always)]
    pub const fn spi_w8(&self) -> &SPI_W8 {
        &self.spi_w8
    }
    #[doc = "0xbc - SPI CPU-controlled buffer9"]
    #[inline(always)]
    pub const fn spi_w9(&self) -> &SPI_W9 {
        &self.spi_w9
    }
    #[doc = "0xc0 - SPI CPU-controlled buffer10"]
    #[inline(always)]
    pub const fn spi_w10(&self) -> &SPI_W10 {
        &self.spi_w10
    }
    #[doc = "0xc4 - SPI CPU-controlled buffer11"]
    #[inline(always)]
    pub const fn spi_w11(&self) -> &SPI_W11 {
        &self.spi_w11
    }
    #[doc = "0xc8 - SPI CPU-controlled buffer12"]
    #[inline(always)]
    pub const fn spi_w12(&self) -> &SPI_W12 {
        &self.spi_w12
    }
    #[doc = "0xcc - SPI CPU-controlled buffer13"]
    #[inline(always)]
    pub const fn spi_w13(&self) -> &SPI_W13 {
        &self.spi_w13
    }
    #[doc = "0xd0 - SPI CPU-controlled buffer14"]
    #[inline(always)]
    pub const fn spi_w14(&self) -> &SPI_W14 {
        &self.spi_w14
    }
    #[doc = "0xd4 - SPI CPU-controlled buffer15"]
    #[inline(always)]
    pub const fn spi_w15(&self) -> &SPI_W15 {
        &self.spi_w15
    }
    #[doc = "0xe0 - SPI slave control register"]
    #[inline(always)]
    pub const fn spi_slave(&self) -> &SPI_SLAVE {
        &self.spi_slave
    }
    #[doc = "0xe4 - SPI slave control register 1"]
    #[inline(always)]
    pub const fn spi_slave1(&self) -> &SPI_SLAVE1 {
        &self.spi_slave1
    }
    #[doc = "0xe8 - SPI module clock and register clock control"]
    #[inline(always)]
    pub const fn spi_clk_gate(&self) -> &SPI_CLK_GATE {
        &self.spi_clk_gate
    }
    #[doc = "0xf0 - Version control"]
    #[inline(always)]
    pub const fn spi_date(&self) -> &SPI_DATE {
        &self.spi_date
    }
}
#[doc = "SPI_CMD (rw) register accessor: Command control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_cmd`] module"]
pub type SPI_CMD = crate::Reg<spi_cmd::SPI_CMD_SPEC>;
#[doc = "Command control register"]
pub mod spi_cmd;
#[doc = "SPI_ADDR (rw) register accessor: Address value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_addr`] module"]
pub type SPI_ADDR = crate::Reg<spi_addr::SPI_ADDR_SPEC>;
#[doc = "Address value register"]
pub mod spi_addr;
#[doc = "SPI_CTRL (rw) register accessor: SPI control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_ctrl`] module"]
pub type SPI_CTRL = crate::Reg<spi_ctrl::SPI_CTRL_SPEC>;
#[doc = "SPI control register"]
pub mod spi_ctrl;
#[doc = "SPI_CLOCK (rw) register accessor: SPI clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_clock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_clock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_clock`] module"]
pub type SPI_CLOCK = crate::Reg<spi_clock::SPI_CLOCK_SPEC>;
#[doc = "SPI clock control register"]
pub mod spi_clock;
#[doc = "SPI_USER (rw) register accessor: SPI USER control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_user::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_user::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_user`] module"]
pub type SPI_USER = crate::Reg<spi_user::SPI_USER_SPEC>;
#[doc = "SPI USER control register"]
pub mod spi_user;
#[doc = "SPI_USER1 (rw) register accessor: SPI USER control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_user1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_user1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_user1`] module"]
pub type SPI_USER1 = crate::Reg<spi_user1::SPI_USER1_SPEC>;
#[doc = "SPI USER control register 1"]
pub mod spi_user1;
#[doc = "SPI_USER2 (rw) register accessor: SPI USER control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_user2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_user2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_user2`] module"]
pub type SPI_USER2 = crate::Reg<spi_user2::SPI_USER2_SPEC>;
#[doc = "SPI USER control register 2"]
pub mod spi_user2;
#[doc = "SPI_MS_DLEN (rw) register accessor: SPI data bit length control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ms_dlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ms_dlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_ms_dlen`] module"]
pub type SPI_MS_DLEN = crate::Reg<spi_ms_dlen::SPI_MS_DLEN_SPEC>;
#[doc = "SPI data bit length control register"]
pub mod spi_ms_dlen;
#[doc = "SPI_MISC (rw) register accessor: SPI misc register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_misc`] module"]
pub type SPI_MISC = crate::Reg<spi_misc::SPI_MISC_SPEC>;
#[doc = "SPI misc register"]
pub mod spi_misc;
#[doc = "SPI_DIN_MODE (rw) register accessor: SPI input delay mode configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_din_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_din_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_din_mode`] module"]
pub type SPI_DIN_MODE = crate::Reg<spi_din_mode::SPI_DIN_MODE_SPEC>;
#[doc = "SPI input delay mode configuration"]
pub mod spi_din_mode;
#[doc = "SPI_DIN_NUM (rw) register accessor: SPI input delay number configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_din_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_din_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_din_num`] module"]
pub type SPI_DIN_NUM = crate::Reg<spi_din_num::SPI_DIN_NUM_SPEC>;
#[doc = "SPI input delay number configuration"]
pub mod spi_din_num;
#[doc = "SPI_DOUT_MODE (rw) register accessor: SPI output delay mode configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_dout_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_dout_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_dout_mode`] module"]
pub type SPI_DOUT_MODE = crate::Reg<spi_dout_mode::SPI_DOUT_MODE_SPEC>;
#[doc = "SPI output delay mode configuration"]
pub mod spi_dout_mode;
#[doc = "SPI_DMA_CONF (rw) register accessor: SPI DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_dma_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_dma_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_dma_conf`] module"]
pub type SPI_DMA_CONF = crate::Reg<spi_dma_conf::SPI_DMA_CONF_SPEC>;
#[doc = "SPI DMA control register"]
pub mod spi_dma_conf;
#[doc = "SPI_DMA_INT_ENA (rw) register accessor: SPI interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_dma_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_dma_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_dma_int_ena`] module"]
pub type SPI_DMA_INT_ENA = crate::Reg<spi_dma_int_ena::SPI_DMA_INT_ENA_SPEC>;
#[doc = "SPI interrupt enable register"]
pub mod spi_dma_int_ena;
#[doc = "SPI_DMA_INT_CLR (w) register accessor: SPI interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_dma_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_dma_int_clr`] module"]
pub type SPI_DMA_INT_CLR = crate::Reg<spi_dma_int_clr::SPI_DMA_INT_CLR_SPEC>;
#[doc = "SPI interrupt clear register"]
pub mod spi_dma_int_clr;
#[doc = "SPI_DMA_INT_RAW (rw) register accessor: SPI interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_dma_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_dma_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_dma_int_raw`] module"]
pub type SPI_DMA_INT_RAW = crate::Reg<spi_dma_int_raw::SPI_DMA_INT_RAW_SPEC>;
#[doc = "SPI interrupt raw register"]
pub mod spi_dma_int_raw;
#[doc = "SPI_DMA_INT_ST (r) register accessor: SPI interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_dma_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_dma_int_st`] module"]
pub type SPI_DMA_INT_ST = crate::Reg<spi_dma_int_st::SPI_DMA_INT_ST_SPEC>;
#[doc = "SPI interrupt status register"]
pub mod spi_dma_int_st;
#[doc = "SPI_DMA_INT_SET (w) register accessor: SPI interrupt software set register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_dma_int_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_dma_int_set`] module"]
pub type SPI_DMA_INT_SET = crate::Reg<spi_dma_int_set::SPI_DMA_INT_SET_SPEC>;
#[doc = "SPI interrupt software set register"]
pub mod spi_dma_int_set;
#[doc = "SPI_W0 (rw) register accessor: SPI CPU-controlled buffer0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w0`] module"]
pub type SPI_W0 = crate::Reg<spi_w0::SPI_W0_SPEC>;
#[doc = "SPI CPU-controlled buffer0"]
pub mod spi_w0;
#[doc = "SPI_W1 (rw) register accessor: SPI CPU-controlled buffer1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w1`] module"]
pub type SPI_W1 = crate::Reg<spi_w1::SPI_W1_SPEC>;
#[doc = "SPI CPU-controlled buffer1"]
pub mod spi_w1;
#[doc = "SPI_W2 (rw) register accessor: SPI CPU-controlled buffer2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w2`] module"]
pub type SPI_W2 = crate::Reg<spi_w2::SPI_W2_SPEC>;
#[doc = "SPI CPU-controlled buffer2"]
pub mod spi_w2;
#[doc = "SPI_W3 (rw) register accessor: SPI CPU-controlled buffer3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w3`] module"]
pub type SPI_W3 = crate::Reg<spi_w3::SPI_W3_SPEC>;
#[doc = "SPI CPU-controlled buffer3"]
pub mod spi_w3;
#[doc = "SPI_W4 (rw) register accessor: SPI CPU-controlled buffer4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w4`] module"]
pub type SPI_W4 = crate::Reg<spi_w4::SPI_W4_SPEC>;
#[doc = "SPI CPU-controlled buffer4"]
pub mod spi_w4;
#[doc = "SPI_W5 (rw) register accessor: SPI CPU-controlled buffer5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w5`] module"]
pub type SPI_W5 = crate::Reg<spi_w5::SPI_W5_SPEC>;
#[doc = "SPI CPU-controlled buffer5"]
pub mod spi_w5;
#[doc = "SPI_W6 (rw) register accessor: SPI CPU-controlled buffer6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w6`] module"]
pub type SPI_W6 = crate::Reg<spi_w6::SPI_W6_SPEC>;
#[doc = "SPI CPU-controlled buffer6"]
pub mod spi_w6;
#[doc = "SPI_W7 (rw) register accessor: SPI CPU-controlled buffer7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w7`] module"]
pub type SPI_W7 = crate::Reg<spi_w7::SPI_W7_SPEC>;
#[doc = "SPI CPU-controlled buffer7"]
pub mod spi_w7;
#[doc = "SPI_W8 (rw) register accessor: SPI CPU-controlled buffer8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w8`] module"]
pub type SPI_W8 = crate::Reg<spi_w8::SPI_W8_SPEC>;
#[doc = "SPI CPU-controlled buffer8"]
pub mod spi_w8;
#[doc = "SPI_W9 (rw) register accessor: SPI CPU-controlled buffer9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w9`] module"]
pub type SPI_W9 = crate::Reg<spi_w9::SPI_W9_SPEC>;
#[doc = "SPI CPU-controlled buffer9"]
pub mod spi_w9;
#[doc = "SPI_W10 (rw) register accessor: SPI CPU-controlled buffer10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w10`] module"]
pub type SPI_W10 = crate::Reg<spi_w10::SPI_W10_SPEC>;
#[doc = "SPI CPU-controlled buffer10"]
pub mod spi_w10;
#[doc = "SPI_W11 (rw) register accessor: SPI CPU-controlled buffer11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w11`] module"]
pub type SPI_W11 = crate::Reg<spi_w11::SPI_W11_SPEC>;
#[doc = "SPI CPU-controlled buffer11"]
pub mod spi_w11;
#[doc = "SPI_W12 (rw) register accessor: SPI CPU-controlled buffer12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w12`] module"]
pub type SPI_W12 = crate::Reg<spi_w12::SPI_W12_SPEC>;
#[doc = "SPI CPU-controlled buffer12"]
pub mod spi_w12;
#[doc = "SPI_W13 (rw) register accessor: SPI CPU-controlled buffer13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w13`] module"]
pub type SPI_W13 = crate::Reg<spi_w13::SPI_W13_SPEC>;
#[doc = "SPI CPU-controlled buffer13"]
pub mod spi_w13;
#[doc = "SPI_W14 (rw) register accessor: SPI CPU-controlled buffer14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w14`] module"]
pub type SPI_W14 = crate::Reg<spi_w14::SPI_W14_SPEC>;
#[doc = "SPI CPU-controlled buffer14"]
pub mod spi_w14;
#[doc = "SPI_W15 (rw) register accessor: SPI CPU-controlled buffer15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w15`] module"]
pub type SPI_W15 = crate::Reg<spi_w15::SPI_W15_SPEC>;
#[doc = "SPI CPU-controlled buffer15"]
pub mod spi_w15;
#[doc = "SPI_SLAVE (rw) register accessor: SPI slave control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_slave::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_slave::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_slave`] module"]
pub type SPI_SLAVE = crate::Reg<spi_slave::SPI_SLAVE_SPEC>;
#[doc = "SPI slave control register"]
pub mod spi_slave;
#[doc = "SPI_SLAVE1 (rw) register accessor: SPI slave control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_slave1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_slave1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_slave1`] module"]
pub type SPI_SLAVE1 = crate::Reg<spi_slave1::SPI_SLAVE1_SPEC>;
#[doc = "SPI slave control register 1"]
pub mod spi_slave1;
#[doc = "SPI_CLK_GATE (rw) register accessor: SPI module clock and register clock control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_clk_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_clk_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_clk_gate`] module"]
pub type SPI_CLK_GATE = crate::Reg<spi_clk_gate::SPI_CLK_GATE_SPEC>;
#[doc = "SPI module clock and register clock control"]
pub mod spi_clk_gate;
#[doc = "SPI_DATE (rw) register accessor: Version control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_date`] module"]
pub type SPI_DATE = crate::Reg<spi_date::SPI_DATE_SPEC>;
#[doc = "Version control"]
pub mod spi_date;
