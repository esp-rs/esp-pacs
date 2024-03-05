#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    cmd: CMD,
    addr: ADDR,
    ctrl: CTRL,
    ctrl1: CTRL1,
    ctrl2: CTRL2,
    clock: CLOCK,
    user: USER,
    user1: USER1,
    user2: USER2,
    mosi_dlen: MOSI_DLEN,
    miso_dlen: MISO_DLEN,
    rd_status: RD_STATUS,
    _reserved12: [u8; 0x04],
    misc: MISC,
    tx_crc: TX_CRC,
    cache_fctrl: CACHE_FCTRL,
    _reserved15: [u8; 0x18],
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
    flash_waiti_ctrl: FLASH_WAITI_CTRL,
    flash_sus_ctrl: FLASH_SUS_CTRL,
    flash_sus_cmd: FLASH_SUS_CMD,
    sus_status: SUS_STATUS,
    timing_cali: TIMING_CALI,
    _reserved36: [u8; 0x14],
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    int_raw: INT_RAW,
    int_st: INT_ST,
    _reserved40: [u8; 0x0c],
    clock_gate: CLOCK_GATE,
    _reserved41: [u8; 0x031c],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI1 memory command register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x04 - SPI1 address register"]
    #[inline(always)]
    pub const fn addr(&self) -> &ADDR {
        &self.addr
    }
    #[doc = "0x08 - SPI1 control register."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x0c - SPI1 control1 register."]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &CTRL1 {
        &self.ctrl1
    }
    #[doc = "0x10 - SPI1 control2 register."]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &CTRL2 {
        &self.ctrl2
    }
    #[doc = "0x14 - SPI1 clock division control register."]
    #[inline(always)]
    pub const fn clock(&self) -> &CLOCK {
        &self.clock
    }
    #[doc = "0x18 - SPI1 user register."]
    #[inline(always)]
    pub const fn user(&self) -> &USER {
        &self.user
    }
    #[doc = "0x1c - SPI1 user1 register."]
    #[inline(always)]
    pub const fn user1(&self) -> &USER1 {
        &self.user1
    }
    #[doc = "0x20 - SPI1 user2 register."]
    #[inline(always)]
    pub const fn user2(&self) -> &USER2 {
        &self.user2
    }
    #[doc = "0x24 - SPI1 send data bit length control register."]
    #[inline(always)]
    pub const fn mosi_dlen(&self) -> &MOSI_DLEN {
        &self.mosi_dlen
    }
    #[doc = "0x28 - SPI1 receive data bit length control register."]
    #[inline(always)]
    pub const fn miso_dlen(&self) -> &MISO_DLEN {
        &self.miso_dlen
    }
    #[doc = "0x2c - SPI1 status register."]
    #[inline(always)]
    pub const fn rd_status(&self) -> &RD_STATUS {
        &self.rd_status
    }
    #[doc = "0x34 - SPI1 misc register"]
    #[inline(always)]
    pub const fn misc(&self) -> &MISC {
        &self.misc
    }
    #[doc = "0x38 - SPI1 TX CRC data register."]
    #[inline(always)]
    pub const fn tx_crc(&self) -> &TX_CRC {
        &self.tx_crc
    }
    #[doc = "0x3c - SPI1 bit mode control register."]
    #[inline(always)]
    pub const fn cache_fctrl(&self) -> &CACHE_FCTRL {
        &self.cache_fctrl
    }
    #[doc = "0x58 - SPI1 memory data buffer0"]
    #[inline(always)]
    pub const fn w0(&self) -> &W0 {
        &self.w0
    }
    #[doc = "0x5c - SPI1 memory data buffer1"]
    #[inline(always)]
    pub const fn w1(&self) -> &W1 {
        &self.w1
    }
    #[doc = "0x60 - SPI1 memory data buffer2"]
    #[inline(always)]
    pub const fn w2(&self) -> &W2 {
        &self.w2
    }
    #[doc = "0x64 - SPI1 memory data buffer3"]
    #[inline(always)]
    pub const fn w3(&self) -> &W3 {
        &self.w3
    }
    #[doc = "0x68 - SPI1 memory data buffer4"]
    #[inline(always)]
    pub const fn w4(&self) -> &W4 {
        &self.w4
    }
    #[doc = "0x6c - SPI1 memory data buffer5"]
    #[inline(always)]
    pub const fn w5(&self) -> &W5 {
        &self.w5
    }
    #[doc = "0x70 - SPI1 memory data buffer6"]
    #[inline(always)]
    pub const fn w6(&self) -> &W6 {
        &self.w6
    }
    #[doc = "0x74 - SPI1 memory data buffer7"]
    #[inline(always)]
    pub const fn w7(&self) -> &W7 {
        &self.w7
    }
    #[doc = "0x78 - SPI1 memory data buffer8"]
    #[inline(always)]
    pub const fn w8(&self) -> &W8 {
        &self.w8
    }
    #[doc = "0x7c - SPI1 memory data buffer9"]
    #[inline(always)]
    pub const fn w9(&self) -> &W9 {
        &self.w9
    }
    #[doc = "0x80 - SPI1 memory data buffer10"]
    #[inline(always)]
    pub const fn w10(&self) -> &W10 {
        &self.w10
    }
    #[doc = "0x84 - SPI1 memory data buffer11"]
    #[inline(always)]
    pub const fn w11(&self) -> &W11 {
        &self.w11
    }
    #[doc = "0x88 - SPI1 memory data buffer12"]
    #[inline(always)]
    pub const fn w12(&self) -> &W12 {
        &self.w12
    }
    #[doc = "0x8c - SPI1 memory data buffer13"]
    #[inline(always)]
    pub const fn w13(&self) -> &W13 {
        &self.w13
    }
    #[doc = "0x90 - SPI1 memory data buffer14"]
    #[inline(always)]
    pub const fn w14(&self) -> &W14 {
        &self.w14
    }
    #[doc = "0x94 - SPI1 memory data buffer15"]
    #[inline(always)]
    pub const fn w15(&self) -> &W15 {
        &self.w15
    }
    #[doc = "0x98 - SPI1 wait idle control register"]
    #[inline(always)]
    pub const fn flash_waiti_ctrl(&self) -> &FLASH_WAITI_CTRL {
        &self.flash_waiti_ctrl
    }
    #[doc = "0x9c - SPI1 flash suspend control register"]
    #[inline(always)]
    pub const fn flash_sus_ctrl(&self) -> &FLASH_SUS_CTRL {
        &self.flash_sus_ctrl
    }
    #[doc = "0xa0 - SPI1 flash suspend command register"]
    #[inline(always)]
    pub const fn flash_sus_cmd(&self) -> &FLASH_SUS_CMD {
        &self.flash_sus_cmd
    }
    #[doc = "0xa4 - SPI1 flash suspend status register"]
    #[inline(always)]
    pub const fn sus_status(&self) -> &SUS_STATUS {
        &self.sus_status
    }
    #[doc = "0xa8 - SPI1 timing control register"]
    #[inline(always)]
    pub const fn timing_cali(&self) -> &TIMING_CALI {
        &self.timing_cali
    }
    #[doc = "0xc0 - SPI1 interrupt enable register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0xc4 - SPI1 interrupt clear register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0xc8 - SPI1 interrupt raw register"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0xcc - SPI1 interrupt status register"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0xdc - SPI1 clk_gate register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x3fc - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CMD (rw) register accessor: SPI1 memory command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "SPI1 memory command register"]
pub mod cmd;
#[doc = "ADDR (rw) register accessor: SPI1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`] module"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "SPI1 address register"]
pub mod addr;
#[doc = "CTRL (rw) register accessor: SPI1 control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "SPI1 control register."]
pub mod ctrl;
#[doc = "CTRL1 (rw) register accessor: SPI1 control1 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`] module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "SPI1 control1 register."]
pub mod ctrl1;
#[doc = "CTRL2 (w) register accessor: SPI1 control2 register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`] module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "SPI1 control2 register."]
pub mod ctrl2;
#[doc = "CLOCK (rw) register accessor: SPI1 clock division control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock`] module"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = "SPI1 clock division control register."]
pub mod clock;
#[doc = "USER (rw) register accessor: SPI1 user register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`user::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user`] module"]
pub type USER = crate::Reg<user::USER_SPEC>;
#[doc = "SPI1 user register."]
pub mod user;
#[doc = "USER1 (rw) register accessor: SPI1 user1 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`user1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user1`] module"]
pub type USER1 = crate::Reg<user1::USER1_SPEC>;
#[doc = "SPI1 user1 register."]
pub mod user1;
#[doc = "USER2 (rw) register accessor: SPI1 user2 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`user2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user2`] module"]
pub type USER2 = crate::Reg<user2::USER2_SPEC>;
#[doc = "SPI1 user2 register."]
pub mod user2;
#[doc = "MOSI_DLEN (rw) register accessor: SPI1 send data bit length control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mosi_dlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mosi_dlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mosi_dlen`] module"]
pub type MOSI_DLEN = crate::Reg<mosi_dlen::MOSI_DLEN_SPEC>;
#[doc = "SPI1 send data bit length control register."]
pub mod mosi_dlen;
#[doc = "MISO_DLEN (rw) register accessor: SPI1 receive data bit length control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`miso_dlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`miso_dlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@miso_dlen`] module"]
pub type MISO_DLEN = crate::Reg<miso_dlen::MISO_DLEN_SPEC>;
#[doc = "SPI1 receive data bit length control register."]
pub mod miso_dlen;
#[doc = "RD_STATUS (rw) register accessor: SPI1 status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_status`] module"]
pub type RD_STATUS = crate::Reg<rd_status::RD_STATUS_SPEC>;
#[doc = "SPI1 status register."]
pub mod rd_status;
#[doc = "MISC (rw) register accessor: SPI1 misc register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc`] module"]
pub type MISC = crate::Reg<misc::MISC_SPEC>;
#[doc = "SPI1 misc register"]
pub mod misc;
#[doc = "TX_CRC (r) register accessor: SPI1 TX CRC data register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_crc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_crc`] module"]
pub type TX_CRC = crate::Reg<tx_crc::TX_CRC_SPEC>;
#[doc = "SPI1 TX CRC data register."]
pub mod tx_crc;
#[doc = "CACHE_FCTRL (rw) register accessor: SPI1 bit mode control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_fctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_fctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_fctrl`] module"]
pub type CACHE_FCTRL = crate::Reg<cache_fctrl::CACHE_FCTRL_SPEC>;
#[doc = "SPI1 bit mode control register."]
pub mod cache_fctrl;
#[doc = "W0 (rw) register accessor: SPI1 memory data buffer0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0`] module"]
pub type W0 = crate::Reg<w0::W0_SPEC>;
#[doc = "SPI1 memory data buffer0"]
pub mod w0;
#[doc = "W1 (rw) register accessor: SPI1 memory data buffer1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w1`] module"]
pub type W1 = crate::Reg<w1::W1_SPEC>;
#[doc = "SPI1 memory data buffer1"]
pub mod w1;
#[doc = "W2 (rw) register accessor: SPI1 memory data buffer2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w2`] module"]
pub type W2 = crate::Reg<w2::W2_SPEC>;
#[doc = "SPI1 memory data buffer2"]
pub mod w2;
#[doc = "W3 (rw) register accessor: SPI1 memory data buffer3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w3`] module"]
pub type W3 = crate::Reg<w3::W3_SPEC>;
#[doc = "SPI1 memory data buffer3"]
pub mod w3;
#[doc = "W4 (rw) register accessor: SPI1 memory data buffer4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w4`] module"]
pub type W4 = crate::Reg<w4::W4_SPEC>;
#[doc = "SPI1 memory data buffer4"]
pub mod w4;
#[doc = "W5 (rw) register accessor: SPI1 memory data buffer5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w5`] module"]
pub type W5 = crate::Reg<w5::W5_SPEC>;
#[doc = "SPI1 memory data buffer5"]
pub mod w5;
#[doc = "W6 (rw) register accessor: SPI1 memory data buffer6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w6`] module"]
pub type W6 = crate::Reg<w6::W6_SPEC>;
#[doc = "SPI1 memory data buffer6"]
pub mod w6;
#[doc = "W7 (rw) register accessor: SPI1 memory data buffer7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w7`] module"]
pub type W7 = crate::Reg<w7::W7_SPEC>;
#[doc = "SPI1 memory data buffer7"]
pub mod w7;
#[doc = "W8 (rw) register accessor: SPI1 memory data buffer8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w8`] module"]
pub type W8 = crate::Reg<w8::W8_SPEC>;
#[doc = "SPI1 memory data buffer8"]
pub mod w8;
#[doc = "W9 (rw) register accessor: SPI1 memory data buffer9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w9`] module"]
pub type W9 = crate::Reg<w9::W9_SPEC>;
#[doc = "SPI1 memory data buffer9"]
pub mod w9;
#[doc = "W10 (rw) register accessor: SPI1 memory data buffer10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w10`] module"]
pub type W10 = crate::Reg<w10::W10_SPEC>;
#[doc = "SPI1 memory data buffer10"]
pub mod w10;
#[doc = "W11 (rw) register accessor: SPI1 memory data buffer11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w11`] module"]
pub type W11 = crate::Reg<w11::W11_SPEC>;
#[doc = "SPI1 memory data buffer11"]
pub mod w11;
#[doc = "W12 (rw) register accessor: SPI1 memory data buffer12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w12`] module"]
pub type W12 = crate::Reg<w12::W12_SPEC>;
#[doc = "SPI1 memory data buffer12"]
pub mod w12;
#[doc = "W13 (rw) register accessor: SPI1 memory data buffer13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w13`] module"]
pub type W13 = crate::Reg<w13::W13_SPEC>;
#[doc = "SPI1 memory data buffer13"]
pub mod w13;
#[doc = "W14 (rw) register accessor: SPI1 memory data buffer14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w14`] module"]
pub type W14 = crate::Reg<w14::W14_SPEC>;
#[doc = "SPI1 memory data buffer14"]
pub mod w14;
#[doc = "W15 (rw) register accessor: SPI1 memory data buffer15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w15`] module"]
pub type W15 = crate::Reg<w15::W15_SPEC>;
#[doc = "SPI1 memory data buffer15"]
pub mod w15;
#[doc = "FLASH_WAITI_CTRL (rw) register accessor: SPI1 wait idle control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_waiti_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_waiti_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_waiti_ctrl`] module"]
pub type FLASH_WAITI_CTRL = crate::Reg<flash_waiti_ctrl::FLASH_WAITI_CTRL_SPEC>;
#[doc = "SPI1 wait idle control register"]
pub mod flash_waiti_ctrl;
#[doc = "FLASH_SUS_CTRL (rw) register accessor: SPI1 flash suspend control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_sus_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_sus_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_sus_ctrl`] module"]
pub type FLASH_SUS_CTRL = crate::Reg<flash_sus_ctrl::FLASH_SUS_CTRL_SPEC>;
#[doc = "SPI1 flash suspend control register"]
pub mod flash_sus_ctrl;
#[doc = "FLASH_SUS_CMD (rw) register accessor: SPI1 flash suspend command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_sus_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_sus_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_sus_cmd`] module"]
pub type FLASH_SUS_CMD = crate::Reg<flash_sus_cmd::FLASH_SUS_CMD_SPEC>;
#[doc = "SPI1 flash suspend command register"]
pub mod flash_sus_cmd;
#[doc = "SUS_STATUS (rw) register accessor: SPI1 flash suspend status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sus_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sus_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sus_status`] module"]
pub type SUS_STATUS = crate::Reg<sus_status::SUS_STATUS_SPEC>;
#[doc = "SPI1 flash suspend status register"]
pub mod sus_status;
#[doc = "TIMING_CALI (rw) register accessor: SPI1 timing control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timing_cali::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timing_cali::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timing_cali`] module"]
pub type TIMING_CALI = crate::Reg<timing_cali::TIMING_CALI_SPEC>;
#[doc = "SPI1 timing control register"]
pub mod timing_cali;
#[doc = "INT_ENA (rw) register accessor: SPI1 interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "SPI1 interrupt enable register"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: SPI1 interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "SPI1 interrupt clear register"]
pub mod int_clr;
#[doc = "INT_RAW (rw) register accessor: SPI1 interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "SPI1 interrupt raw register"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: SPI1 interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "SPI1 interrupt status register"]
pub mod int_st;
#[doc = "CLOCK_GATE (rw) register accessor: SPI1 clk_gate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "SPI1 clk_gate register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
