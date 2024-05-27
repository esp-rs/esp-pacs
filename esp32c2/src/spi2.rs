#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
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
    dma_int_set: DMA_INT_SET,
    _reserved18: [u8; 0x50],
    w: [W; 16],
    _reserved19: [u8; 0x08],
    slave: SLAVE,
    slave1: SLAVE1,
    clk_gate: CLK_GATE,
    _reserved22: [u8; 0x04],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - Command control register
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    ///0x04 - Address value register
    #[inline(always)]
    pub const fn addr(&self) -> &ADDR {
        &self.addr
    }
    ///0x08 - SPI control register
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    ///0x0c - SPI clock control register
    #[inline(always)]
    pub const fn clock(&self) -> &CLOCK {
        &self.clock
    }
    ///0x10 - SPI USER control register
    #[inline(always)]
    pub const fn user(&self) -> &USER {
        &self.user
    }
    ///0x14 - SPI USER control register 1
    #[inline(always)]
    pub const fn user1(&self) -> &USER1 {
        &self.user1
    }
    ///0x18 - SPI USER control register 2
    #[inline(always)]
    pub const fn user2(&self) -> &USER2 {
        &self.user2
    }
    ///0x1c - SPI data bit length control register
    #[inline(always)]
    pub const fn ms_dlen(&self) -> &MS_DLEN {
        &self.ms_dlen
    }
    ///0x20 - SPI misc register
    #[inline(always)]
    pub const fn misc(&self) -> &MISC {
        &self.misc
    }
    ///0x24 - SPI input delay mode configuration
    #[inline(always)]
    pub const fn din_mode(&self) -> &DIN_MODE {
        &self.din_mode
    }
    ///0x28 - SPI input delay number configuration
    #[inline(always)]
    pub const fn din_num(&self) -> &DIN_NUM {
        &self.din_num
    }
    ///0x2c - SPI output delay mode configuration
    #[inline(always)]
    pub const fn dout_mode(&self) -> &DOUT_MODE {
        &self.dout_mode
    }
    ///0x30 - SPI DMA control register
    #[inline(always)]
    pub const fn dma_conf(&self) -> &DMA_CONF {
        &self.dma_conf
    }
    ///0x34 - SPI interrupt enable register
    #[inline(always)]
    pub const fn dma_int_ena(&self) -> &DMA_INT_ENA {
        &self.dma_int_ena
    }
    ///0x38 - SPI interrupt clear register
    #[inline(always)]
    pub const fn dma_int_clr(&self) -> &DMA_INT_CLR {
        &self.dma_int_clr
    }
    ///0x3c - SPI interrupt raw register
    #[inline(always)]
    pub const fn dma_int_raw(&self) -> &DMA_INT_RAW {
        &self.dma_int_raw
    }
    ///0x40 - SPI interrupt status register
    #[inline(always)]
    pub const fn dma_int_st(&self) -> &DMA_INT_ST {
        &self.dma_int_st
    }
    ///0x44 - SPI interrupt software set register
    #[inline(always)]
    pub const fn dma_int_set(&self) -> &DMA_INT_SET {
        &self.dma_int_set
    }
    ///0x98..0xd8 - SPI CPU-controlled buffer%s
    #[inline(always)]
    pub const fn w(&self, n: usize) -> &W {
        &self.w[n]
    }
    ///Iterator for array of:
    ///0x98..0xd8 - SPI CPU-controlled buffer%s
    #[inline(always)]
    pub fn w_iter(&self) -> impl Iterator<Item = &W> {
        self.w.iter()
    }
    ///0xe0 - SPI slave control register
    #[inline(always)]
    pub const fn slave(&self) -> &SLAVE {
        &self.slave
    }
    ///0xe4 - SPI slave control register 1
    #[inline(always)]
    pub const fn slave1(&self) -> &SLAVE1 {
        &self.slave1
    }
    ///0xe8 - SPI module clock and register clock control
    #[inline(always)]
    pub const fn clk_gate(&self) -> &CLK_GATE {
        &self.clk_gate
    }
    ///0xf0 - Version control
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**CMD (rw) register accessor: Command control register

You can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cmd`] module*/
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
///Command control register
pub mod cmd;
/**ADDR (rw) register accessor: Address value register

You can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@addr`] module*/
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
///Address value register
pub mod addr;
/**CTRL (rw) register accessor: SPI control register

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl`] module*/
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
///SPI control register
pub mod ctrl;
/**CLOCK (rw) register accessor: SPI clock control register

You can [`read`](crate::generic::Reg::read) this register and get [`clock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clock`] module*/
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
///SPI clock control register
pub mod clock;
/**USER (rw) register accessor: SPI USER control register

You can [`read`](crate::generic::Reg::read) this register and get [`user::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@user`] module*/
pub type USER = crate::Reg<user::USER_SPEC>;
///SPI USER control register
pub mod user;
/**USER1 (rw) register accessor: SPI USER control register 1

You can [`read`](crate::generic::Reg::read) this register and get [`user1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@user1`] module*/
pub type USER1 = crate::Reg<user1::USER1_SPEC>;
///SPI USER control register 1
pub mod user1;
/**USER2 (rw) register accessor: SPI USER control register 2

You can [`read`](crate::generic::Reg::read) this register and get [`user2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@user2`] module*/
pub type USER2 = crate::Reg<user2::USER2_SPEC>;
///SPI USER control register 2
pub mod user2;
/**MS_DLEN (rw) register accessor: SPI data bit length control register

You can [`read`](crate::generic::Reg::read) this register and get [`ms_dlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ms_dlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ms_dlen`] module*/
pub type MS_DLEN = crate::Reg<ms_dlen::MS_DLEN_SPEC>;
///SPI data bit length control register
pub mod ms_dlen;
/**MISC (rw) register accessor: SPI misc register

You can [`read`](crate::generic::Reg::read) this register and get [`misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@misc`] module*/
pub type MISC = crate::Reg<misc::MISC_SPEC>;
///SPI misc register
pub mod misc;
/**DIN_MODE (r) register accessor: SPI input delay mode configuration

You can [`read`](crate::generic::Reg::read) this register and get [`din_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@din_mode`] module*/
pub type DIN_MODE = crate::Reg<din_mode::DIN_MODE_SPEC>;
///SPI input delay mode configuration
pub mod din_mode;
/**DIN_NUM (r) register accessor: SPI input delay number configuration

You can [`read`](crate::generic::Reg::read) this register and get [`din_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@din_num`] module*/
pub type DIN_NUM = crate::Reg<din_num::DIN_NUM_SPEC>;
///SPI input delay number configuration
pub mod din_num;
/**DOUT_MODE (r) register accessor: SPI output delay mode configuration

You can [`read`](crate::generic::Reg::read) this register and get [`dout_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dout_mode`] module*/
pub type DOUT_MODE = crate::Reg<dout_mode::DOUT_MODE_SPEC>;
///SPI output delay mode configuration
pub mod dout_mode;
/**DMA_CONF (rw) register accessor: SPI DMA control register

You can [`read`](crate::generic::Reg::read) this register and get [`dma_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_conf`] module*/
pub type DMA_CONF = crate::Reg<dma_conf::DMA_CONF_SPEC>;
///SPI DMA control register
pub mod dma_conf;
/**DMA_INT_ENA (rw) register accessor: SPI interrupt enable register

You can [`read`](crate::generic::Reg::read) this register and get [`dma_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_int_ena`] module*/
pub type DMA_INT_ENA = crate::Reg<dma_int_ena::DMA_INT_ENA_SPEC>;
///SPI interrupt enable register
pub mod dma_int_ena;
/**DMA_INT_CLR (w) register accessor: SPI interrupt clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_int_clr`] module*/
pub type DMA_INT_CLR = crate::Reg<dma_int_clr::DMA_INT_CLR_SPEC>;
///SPI interrupt clear register
pub mod dma_int_clr;
/**DMA_INT_RAW (rw) register accessor: SPI interrupt raw register

You can [`read`](crate::generic::Reg::read) this register and get [`dma_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_int_raw`] module*/
pub type DMA_INT_RAW = crate::Reg<dma_int_raw::DMA_INT_RAW_SPEC>;
///SPI interrupt raw register
pub mod dma_int_raw;
/**DMA_INT_ST (r) register accessor: SPI interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`dma_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_int_st`] module*/
pub type DMA_INT_ST = crate::Reg<dma_int_st::DMA_INT_ST_SPEC>;
///SPI interrupt status register
pub mod dma_int_st;
/**DMA_INT_SET (w) register accessor: SPI interrupt software set register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_int_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_int_set`] module*/
pub type DMA_INT_SET = crate::Reg<dma_int_set::DMA_INT_SET_SPEC>;
///SPI interrupt software set register
pub mod dma_int_set;
/**W (rw) register accessor: SPI CPU-controlled buffer%s

You can [`read`](crate::generic::Reg::read) this register and get [`w::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@w`] module*/
pub type W = crate::Reg<w::W_SPEC>;
///SPI CPU-controlled buffer%s
pub mod w;
/**SLAVE (rw) register accessor: SPI slave control register

You can [`read`](crate::generic::Reg::read) this register and get [`slave::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slave`] module*/
pub type SLAVE = crate::Reg<slave::SLAVE_SPEC>;
///SPI slave control register
pub mod slave;
/**SLAVE1 (rw) register accessor: SPI slave control register 1

You can [`read`](crate::generic::Reg::read) this register and get [`slave1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slave1`] module*/
pub type SLAVE1 = crate::Reg<slave1::SLAVE1_SPEC>;
///SPI slave control register 1
pub mod slave1;
/**CLK_GATE (rw) register accessor: SPI module clock and register clock control

You can [`read`](crate::generic::Reg::read) this register and get [`clk_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clk_gate`] module*/
pub type CLK_GATE = crate::Reg<clk_gate::CLK_GATE_SPEC>;
///SPI module clock and register clock control
pub mod clk_gate;
/**DATE (rw) register accessor: Version control

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///Version control
pub mod date;
