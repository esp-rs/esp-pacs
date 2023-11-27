#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    spi_mem_cmd: SPI_MEM_CMD,
    spi_mem_addr: SPI_MEM_ADDR,
    spi_mem_ctrl: SPI_MEM_CTRL,
    spi_mem_ctrl1: SPI_MEM_CTRL1,
    spi_mem_ctrl2: SPI_MEM_CTRL2,
    spi_mem_clock: SPI_MEM_CLOCK,
    spi_mem_user: SPI_MEM_USER,
    spi_mem_user1: SPI_MEM_USER1,
    spi_mem_user2: SPI_MEM_USER2,
    spi_mem_mosi_dlen: SPI_MEM_MOSI_DLEN,
    spi_mem_miso_dlen: SPI_MEM_MISO_DLEN,
    spi_mem_rd_status: SPI_MEM_RD_STATUS,
    _reserved12: [u8; 0x04],
    spi_mem_misc: SPI_MEM_MISC,
    spi_mem_tx_crc: SPI_MEM_TX_CRC,
    spi_mem_cache_fctrl: SPI_MEM_CACHE_FCTRL,
    _reserved15: [u8; 0x18],
    spi_mem_w0: SPI_MEM_W0,
    spi_mem_w1: SPI_MEM_W1,
    spi_mem_w2: SPI_MEM_W2,
    spi_mem_w3: SPI_MEM_W3,
    spi_mem_w4: SPI_MEM_W4,
    spi_mem_w5: SPI_MEM_W5,
    spi_mem_w6: SPI_MEM_W6,
    spi_mem_w7: SPI_MEM_W7,
    spi_mem_w8: SPI_MEM_W8,
    spi_mem_w9: SPI_MEM_W9,
    spi_mem_w10: SPI_MEM_W10,
    spi_mem_w11: SPI_MEM_W11,
    spi_mem_w12: SPI_MEM_W12,
    spi_mem_w13: SPI_MEM_W13,
    spi_mem_w14: SPI_MEM_W14,
    spi_mem_w15: SPI_MEM_W15,
    spi_mem_flash_waiti_ctrl: SPI_MEM_FLASH_WAITI_CTRL,
    spi_mem_flash_sus_ctrl: SPI_MEM_FLASH_SUS_CTRL,
    spi_mem_flash_sus_cmd: SPI_MEM_FLASH_SUS_CMD,
    spi_mem_sus_status: SPI_MEM_SUS_STATUS,
    _reserved35: [u8; 0x18],
    spi_mem_int_ena: SPI_MEM_INT_ENA,
    spi_mem_int_clr: SPI_MEM_INT_CLR,
    spi_mem_int_raw: SPI_MEM_INT_RAW,
    spi_mem_int_st: SPI_MEM_INT_ST,
    _reserved39: [u8; 0x04],
    spi_mem_ddr: SPI_MEM_DDR,
    _reserved40: [u8; 0xa8],
    spi_mem_timing_cali: SPI_MEM_TIMING_CALI,
    _reserved41: [u8; 0x7c],
    spi_mem_clock_gate: SPI_MEM_CLOCK_GATE,
    _reserved42: [u8; 0x01f8],
    spi_mem_date: SPI_MEM_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI1 memory command register"]
    #[inline(always)]
    pub const fn spi_mem_cmd(&self) -> &SPI_MEM_CMD {
        &self.spi_mem_cmd
    }
    #[doc = "0x04 - SPI1 address register"]
    #[inline(always)]
    pub const fn spi_mem_addr(&self) -> &SPI_MEM_ADDR {
        &self.spi_mem_addr
    }
    #[doc = "0x08 - SPI1 control register."]
    #[inline(always)]
    pub const fn spi_mem_ctrl(&self) -> &SPI_MEM_CTRL {
        &self.spi_mem_ctrl
    }
    #[doc = "0x0c - SPI1 control1 register."]
    #[inline(always)]
    pub const fn spi_mem_ctrl1(&self) -> &SPI_MEM_CTRL1 {
        &self.spi_mem_ctrl1
    }
    #[doc = "0x10 - SPI1 control2 register."]
    #[inline(always)]
    pub const fn spi_mem_ctrl2(&self) -> &SPI_MEM_CTRL2 {
        &self.spi_mem_ctrl2
    }
    #[doc = "0x14 - SPI1 clock division control register."]
    #[inline(always)]
    pub const fn spi_mem_clock(&self) -> &SPI_MEM_CLOCK {
        &self.spi_mem_clock
    }
    #[doc = "0x18 - SPI1 user register."]
    #[inline(always)]
    pub const fn spi_mem_user(&self) -> &SPI_MEM_USER {
        &self.spi_mem_user
    }
    #[doc = "0x1c - SPI1 user1 register."]
    #[inline(always)]
    pub const fn spi_mem_user1(&self) -> &SPI_MEM_USER1 {
        &self.spi_mem_user1
    }
    #[doc = "0x20 - SPI1 user2 register."]
    #[inline(always)]
    pub const fn spi_mem_user2(&self) -> &SPI_MEM_USER2 {
        &self.spi_mem_user2
    }
    #[doc = "0x24 - SPI1 send data bit length control register."]
    #[inline(always)]
    pub const fn spi_mem_mosi_dlen(&self) -> &SPI_MEM_MOSI_DLEN {
        &self.spi_mem_mosi_dlen
    }
    #[doc = "0x28 - SPI1 receive data bit length control register."]
    #[inline(always)]
    pub const fn spi_mem_miso_dlen(&self) -> &SPI_MEM_MISO_DLEN {
        &self.spi_mem_miso_dlen
    }
    #[doc = "0x2c - SPI1 status register."]
    #[inline(always)]
    pub const fn spi_mem_rd_status(&self) -> &SPI_MEM_RD_STATUS {
        &self.spi_mem_rd_status
    }
    #[doc = "0x34 - SPI1 misc register"]
    #[inline(always)]
    pub const fn spi_mem_misc(&self) -> &SPI_MEM_MISC {
        &self.spi_mem_misc
    }
    #[doc = "0x38 - SPI1 TX CRC data register."]
    #[inline(always)]
    pub const fn spi_mem_tx_crc(&self) -> &SPI_MEM_TX_CRC {
        &self.spi_mem_tx_crc
    }
    #[doc = "0x3c - SPI1 bit mode control register."]
    #[inline(always)]
    pub const fn spi_mem_cache_fctrl(&self) -> &SPI_MEM_CACHE_FCTRL {
        &self.spi_mem_cache_fctrl
    }
    #[doc = "0x58 - SPI1 memory data buffer0"]
    #[inline(always)]
    pub const fn spi_mem_w0(&self) -> &SPI_MEM_W0 {
        &self.spi_mem_w0
    }
    #[doc = "0x5c - SPI1 memory data buffer1"]
    #[inline(always)]
    pub const fn spi_mem_w1(&self) -> &SPI_MEM_W1 {
        &self.spi_mem_w1
    }
    #[doc = "0x60 - SPI1 memory data buffer2"]
    #[inline(always)]
    pub const fn spi_mem_w2(&self) -> &SPI_MEM_W2 {
        &self.spi_mem_w2
    }
    #[doc = "0x64 - SPI1 memory data buffer3"]
    #[inline(always)]
    pub const fn spi_mem_w3(&self) -> &SPI_MEM_W3 {
        &self.spi_mem_w3
    }
    #[doc = "0x68 - SPI1 memory data buffer4"]
    #[inline(always)]
    pub const fn spi_mem_w4(&self) -> &SPI_MEM_W4 {
        &self.spi_mem_w4
    }
    #[doc = "0x6c - SPI1 memory data buffer5"]
    #[inline(always)]
    pub const fn spi_mem_w5(&self) -> &SPI_MEM_W5 {
        &self.spi_mem_w5
    }
    #[doc = "0x70 - SPI1 memory data buffer6"]
    #[inline(always)]
    pub const fn spi_mem_w6(&self) -> &SPI_MEM_W6 {
        &self.spi_mem_w6
    }
    #[doc = "0x74 - SPI1 memory data buffer7"]
    #[inline(always)]
    pub const fn spi_mem_w7(&self) -> &SPI_MEM_W7 {
        &self.spi_mem_w7
    }
    #[doc = "0x78 - SPI1 memory data buffer8"]
    #[inline(always)]
    pub const fn spi_mem_w8(&self) -> &SPI_MEM_W8 {
        &self.spi_mem_w8
    }
    #[doc = "0x7c - SPI1 memory data buffer9"]
    #[inline(always)]
    pub const fn spi_mem_w9(&self) -> &SPI_MEM_W9 {
        &self.spi_mem_w9
    }
    #[doc = "0x80 - SPI1 memory data buffer10"]
    #[inline(always)]
    pub const fn spi_mem_w10(&self) -> &SPI_MEM_W10 {
        &self.spi_mem_w10
    }
    #[doc = "0x84 - SPI1 memory data buffer11"]
    #[inline(always)]
    pub const fn spi_mem_w11(&self) -> &SPI_MEM_W11 {
        &self.spi_mem_w11
    }
    #[doc = "0x88 - SPI1 memory data buffer12"]
    #[inline(always)]
    pub const fn spi_mem_w12(&self) -> &SPI_MEM_W12 {
        &self.spi_mem_w12
    }
    #[doc = "0x8c - SPI1 memory data buffer13"]
    #[inline(always)]
    pub const fn spi_mem_w13(&self) -> &SPI_MEM_W13 {
        &self.spi_mem_w13
    }
    #[doc = "0x90 - SPI1 memory data buffer14"]
    #[inline(always)]
    pub const fn spi_mem_w14(&self) -> &SPI_MEM_W14 {
        &self.spi_mem_w14
    }
    #[doc = "0x94 - SPI1 memory data buffer15"]
    #[inline(always)]
    pub const fn spi_mem_w15(&self) -> &SPI_MEM_W15 {
        &self.spi_mem_w15
    }
    #[doc = "0x98 - SPI1 wait idle control register"]
    #[inline(always)]
    pub const fn spi_mem_flash_waiti_ctrl(&self) -> &SPI_MEM_FLASH_WAITI_CTRL {
        &self.spi_mem_flash_waiti_ctrl
    }
    #[doc = "0x9c - SPI1 flash suspend control register"]
    #[inline(always)]
    pub const fn spi_mem_flash_sus_ctrl(&self) -> &SPI_MEM_FLASH_SUS_CTRL {
        &self.spi_mem_flash_sus_ctrl
    }
    #[doc = "0xa0 - SPI1 flash suspend command register"]
    #[inline(always)]
    pub const fn spi_mem_flash_sus_cmd(&self) -> &SPI_MEM_FLASH_SUS_CMD {
        &self.spi_mem_flash_sus_cmd
    }
    #[doc = "0xa4 - SPI1 flash suspend status register"]
    #[inline(always)]
    pub const fn spi_mem_sus_status(&self) -> &SPI_MEM_SUS_STATUS {
        &self.spi_mem_sus_status
    }
    #[doc = "0xc0 - SPI1 interrupt enable register"]
    #[inline(always)]
    pub const fn spi_mem_int_ena(&self) -> &SPI_MEM_INT_ENA {
        &self.spi_mem_int_ena
    }
    #[doc = "0xc4 - SPI1 interrupt clear register"]
    #[inline(always)]
    pub const fn spi_mem_int_clr(&self) -> &SPI_MEM_INT_CLR {
        &self.spi_mem_int_clr
    }
    #[doc = "0xc8 - SPI1 interrupt raw register"]
    #[inline(always)]
    pub const fn spi_mem_int_raw(&self) -> &SPI_MEM_INT_RAW {
        &self.spi_mem_int_raw
    }
    #[doc = "0xcc - SPI1 interrupt status register"]
    #[inline(always)]
    pub const fn spi_mem_int_st(&self) -> &SPI_MEM_INT_ST {
        &self.spi_mem_int_st
    }
    #[doc = "0xd4 - SPI1 DDR control register"]
    #[inline(always)]
    pub const fn spi_mem_ddr(&self) -> &SPI_MEM_DDR {
        &self.spi_mem_ddr
    }
    #[doc = "0x180 - SPI1 timing control register"]
    #[inline(always)]
    pub const fn spi_mem_timing_cali(&self) -> &SPI_MEM_TIMING_CALI {
        &self.spi_mem_timing_cali
    }
    #[doc = "0x200 - SPI1 clk_gate register"]
    #[inline(always)]
    pub const fn spi_mem_clock_gate(&self) -> &SPI_MEM_CLOCK_GATE {
        &self.spi_mem_clock_gate
    }
    #[doc = "0x3fc - Version control register"]
    #[inline(always)]
    pub const fn spi_mem_date(&self) -> &SPI_MEM_DATE {
        &self.spi_mem_date
    }
}
#[doc = "SPI_MEM_CMD (rw) register accessor: SPI1 memory command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_cmd`] module"]
pub type SPI_MEM_CMD = crate::Reg<spi_mem_cmd::SPI_MEM_CMD_SPEC>;
#[doc = "SPI1 memory command register"]
pub mod spi_mem_cmd;
#[doc = "SPI_MEM_ADDR (rw) register accessor: SPI1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_addr`] module"]
pub type SPI_MEM_ADDR = crate::Reg<spi_mem_addr::SPI_MEM_ADDR_SPEC>;
#[doc = "SPI1 address register"]
pub mod spi_mem_addr;
#[doc = "SPI_MEM_CTRL (rw) register accessor: SPI1 control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_ctrl`] module"]
pub type SPI_MEM_CTRL = crate::Reg<spi_mem_ctrl::SPI_MEM_CTRL_SPEC>;
#[doc = "SPI1 control register."]
pub mod spi_mem_ctrl;
#[doc = "SPI_MEM_CTRL1 (rw) register accessor: SPI1 control1 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_ctrl1`] module"]
pub type SPI_MEM_CTRL1 = crate::Reg<spi_mem_ctrl1::SPI_MEM_CTRL1_SPEC>;
#[doc = "SPI1 control1 register."]
pub mod spi_mem_ctrl1;
#[doc = "SPI_MEM_CTRL2 (w) register accessor: SPI1 control2 register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_ctrl2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_ctrl2`] module"]
pub type SPI_MEM_CTRL2 = crate::Reg<spi_mem_ctrl2::SPI_MEM_CTRL2_SPEC>;
#[doc = "SPI1 control2 register."]
pub mod spi_mem_ctrl2;
#[doc = "SPI_MEM_CLOCK (rw) register accessor: SPI1 clock division control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_clock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_clock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_clock`] module"]
pub type SPI_MEM_CLOCK = crate::Reg<spi_mem_clock::SPI_MEM_CLOCK_SPEC>;
#[doc = "SPI1 clock division control register."]
pub mod spi_mem_clock;
#[doc = "SPI_MEM_USER (rw) register accessor: SPI1 user register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_user::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_user::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_user`] module"]
pub type SPI_MEM_USER = crate::Reg<spi_mem_user::SPI_MEM_USER_SPEC>;
#[doc = "SPI1 user register."]
pub mod spi_mem_user;
#[doc = "SPI_MEM_USER1 (rw) register accessor: SPI1 user1 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_user1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_user1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_user1`] module"]
pub type SPI_MEM_USER1 = crate::Reg<spi_mem_user1::SPI_MEM_USER1_SPEC>;
#[doc = "SPI1 user1 register."]
pub mod spi_mem_user1;
#[doc = "SPI_MEM_USER2 (rw) register accessor: SPI1 user2 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_user2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_user2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_user2`] module"]
pub type SPI_MEM_USER2 = crate::Reg<spi_mem_user2::SPI_MEM_USER2_SPEC>;
#[doc = "SPI1 user2 register."]
pub mod spi_mem_user2;
#[doc = "SPI_MEM_MOSI_DLEN (rw) register accessor: SPI1 send data bit length control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_mosi_dlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_mosi_dlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_mosi_dlen`] module"]
pub type SPI_MEM_MOSI_DLEN = crate::Reg<spi_mem_mosi_dlen::SPI_MEM_MOSI_DLEN_SPEC>;
#[doc = "SPI1 send data bit length control register."]
pub mod spi_mem_mosi_dlen;
#[doc = "SPI_MEM_MISO_DLEN (rw) register accessor: SPI1 receive data bit length control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_miso_dlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_miso_dlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_miso_dlen`] module"]
pub type SPI_MEM_MISO_DLEN = crate::Reg<spi_mem_miso_dlen::SPI_MEM_MISO_DLEN_SPEC>;
#[doc = "SPI1 receive data bit length control register."]
pub mod spi_mem_miso_dlen;
#[doc = "SPI_MEM_RD_STATUS (rw) register accessor: SPI1 status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_rd_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_rd_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_rd_status`] module"]
pub type SPI_MEM_RD_STATUS = crate::Reg<spi_mem_rd_status::SPI_MEM_RD_STATUS_SPEC>;
#[doc = "SPI1 status register."]
pub mod spi_mem_rd_status;
#[doc = "SPI_MEM_MISC (rw) register accessor: SPI1 misc register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_misc`] module"]
pub type SPI_MEM_MISC = crate::Reg<spi_mem_misc::SPI_MEM_MISC_SPEC>;
#[doc = "SPI1 misc register"]
pub mod spi_mem_misc;
#[doc = "SPI_MEM_TX_CRC (r) register accessor: SPI1 TX CRC data register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_tx_crc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_tx_crc`] module"]
pub type SPI_MEM_TX_CRC = crate::Reg<spi_mem_tx_crc::SPI_MEM_TX_CRC_SPEC>;
#[doc = "SPI1 TX CRC data register."]
pub mod spi_mem_tx_crc;
#[doc = "SPI_MEM_CACHE_FCTRL (rw) register accessor: SPI1 bit mode control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_cache_fctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_cache_fctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_cache_fctrl`] module"]
pub type SPI_MEM_CACHE_FCTRL = crate::Reg<spi_mem_cache_fctrl::SPI_MEM_CACHE_FCTRL_SPEC>;
#[doc = "SPI1 bit mode control register."]
pub mod spi_mem_cache_fctrl;
#[doc = "SPI_MEM_W0 (rw) register accessor: SPI1 memory data buffer0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_w0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_w0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_w0`] module"]
pub type SPI_MEM_W0 = crate::Reg<spi_mem_w0::SPI_MEM_W0_SPEC>;
#[doc = "SPI1 memory data buffer0"]
pub mod spi_mem_w0;
#[doc = "SPI_MEM_W1 (rw) register accessor: SPI1 memory data buffer1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_w1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_w1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_w1`] module"]
pub type SPI_MEM_W1 = crate::Reg<spi_mem_w1::SPI_MEM_W1_SPEC>;
#[doc = "SPI1 memory data buffer1"]
pub mod spi_mem_w1;
#[doc = "SPI_MEM_W2 (rw) register accessor: SPI1 memory data buffer2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_w2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_w2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_w2`] module"]
pub type SPI_MEM_W2 = crate::Reg<spi_mem_w2::SPI_MEM_W2_SPEC>;
#[doc = "SPI1 memory data buffer2"]
pub mod spi_mem_w2;
#[doc = "SPI_MEM_W3 (rw) register accessor: SPI1 memory data buffer3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_w3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_w3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_w3`] module"]
pub type SPI_MEM_W3 = crate::Reg<spi_mem_w3::SPI_MEM_W3_SPEC>;
#[doc = "SPI1 memory data buffer3"]
pub mod spi_mem_w3;
#[doc = "SPI_MEM_W4 (rw) register accessor: SPI1 memory data buffer4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_w4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_w4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_w4`] module"]
pub type SPI_MEM_W4 = crate::Reg<spi_mem_w4::SPI_MEM_W4_SPEC>;
#[doc = "SPI1 memory data buffer4"]
pub mod spi_mem_w4;
#[doc = "SPI_MEM_W5 (rw) register accessor: SPI1 memory data buffer5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_w5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_w5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_w5`] module"]
pub type SPI_MEM_W5 = crate::Reg<spi_mem_w5::SPI_MEM_W5_SPEC>;
#[doc = "SPI1 memory data buffer5"]
pub mod spi_mem_w5;
#[doc = "SPI_MEM_W6 (rw) register accessor: SPI1 memory data buffer6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_w6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_w6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_w6`] module"]
pub type SPI_MEM_W6 = crate::Reg<spi_mem_w6::SPI_MEM_W6_SPEC>;
#[doc = "SPI1 memory data buffer6"]
pub mod spi_mem_w6;
#[doc = "SPI_MEM_W7 (rw) register accessor: SPI1 memory data buffer7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_w7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_w7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_w7`] module"]
pub type SPI_MEM_W7 = crate::Reg<spi_mem_w7::SPI_MEM_W7_SPEC>;
#[doc = "SPI1 memory data buffer7"]
pub mod spi_mem_w7;
#[doc = "SPI_MEM_W8 (rw) register accessor: SPI1 memory data buffer8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_w8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_w8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_w8`] module"]
pub type SPI_MEM_W8 = crate::Reg<spi_mem_w8::SPI_MEM_W8_SPEC>;
#[doc = "SPI1 memory data buffer8"]
pub mod spi_mem_w8;
#[doc = "SPI_MEM_W9 (rw) register accessor: SPI1 memory data buffer9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_w9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_w9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_w9`] module"]
pub type SPI_MEM_W9 = crate::Reg<spi_mem_w9::SPI_MEM_W9_SPEC>;
#[doc = "SPI1 memory data buffer9"]
pub mod spi_mem_w9;
#[doc = "SPI_MEM_W10 (rw) register accessor: SPI1 memory data buffer10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_w10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_w10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_w10`] module"]
pub type SPI_MEM_W10 = crate::Reg<spi_mem_w10::SPI_MEM_W10_SPEC>;
#[doc = "SPI1 memory data buffer10"]
pub mod spi_mem_w10;
#[doc = "SPI_MEM_W11 (rw) register accessor: SPI1 memory data buffer11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_w11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_w11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_w11`] module"]
pub type SPI_MEM_W11 = crate::Reg<spi_mem_w11::SPI_MEM_W11_SPEC>;
#[doc = "SPI1 memory data buffer11"]
pub mod spi_mem_w11;
#[doc = "SPI_MEM_W12 (rw) register accessor: SPI1 memory data buffer12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_w12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_w12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_w12`] module"]
pub type SPI_MEM_W12 = crate::Reg<spi_mem_w12::SPI_MEM_W12_SPEC>;
#[doc = "SPI1 memory data buffer12"]
pub mod spi_mem_w12;
#[doc = "SPI_MEM_W13 (rw) register accessor: SPI1 memory data buffer13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_w13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_w13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_w13`] module"]
pub type SPI_MEM_W13 = crate::Reg<spi_mem_w13::SPI_MEM_W13_SPEC>;
#[doc = "SPI1 memory data buffer13"]
pub mod spi_mem_w13;
#[doc = "SPI_MEM_W14 (rw) register accessor: SPI1 memory data buffer14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_w14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_w14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_w14`] module"]
pub type SPI_MEM_W14 = crate::Reg<spi_mem_w14::SPI_MEM_W14_SPEC>;
#[doc = "SPI1 memory data buffer14"]
pub mod spi_mem_w14;
#[doc = "SPI_MEM_W15 (rw) register accessor: SPI1 memory data buffer15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_w15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_w15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_w15`] module"]
pub type SPI_MEM_W15 = crate::Reg<spi_mem_w15::SPI_MEM_W15_SPEC>;
#[doc = "SPI1 memory data buffer15"]
pub mod spi_mem_w15;
#[doc = "SPI_MEM_FLASH_WAITI_CTRL (rw) register accessor: SPI1 wait idle control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_flash_waiti_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_flash_waiti_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_flash_waiti_ctrl`] module"]
pub type SPI_MEM_FLASH_WAITI_CTRL =
    crate::Reg<spi_mem_flash_waiti_ctrl::SPI_MEM_FLASH_WAITI_CTRL_SPEC>;
#[doc = "SPI1 wait idle control register"]
pub mod spi_mem_flash_waiti_ctrl;
#[doc = "SPI_MEM_FLASH_SUS_CTRL (rw) register accessor: SPI1 flash suspend control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_flash_sus_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_flash_sus_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_flash_sus_ctrl`] module"]
pub type SPI_MEM_FLASH_SUS_CTRL = crate::Reg<spi_mem_flash_sus_ctrl::SPI_MEM_FLASH_SUS_CTRL_SPEC>;
#[doc = "SPI1 flash suspend control register"]
pub mod spi_mem_flash_sus_ctrl;
#[doc = "SPI_MEM_FLASH_SUS_CMD (rw) register accessor: SPI1 flash suspend command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_flash_sus_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_flash_sus_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_flash_sus_cmd`] module"]
pub type SPI_MEM_FLASH_SUS_CMD = crate::Reg<spi_mem_flash_sus_cmd::SPI_MEM_FLASH_SUS_CMD_SPEC>;
#[doc = "SPI1 flash suspend command register"]
pub mod spi_mem_flash_sus_cmd;
#[doc = "SPI_MEM_SUS_STATUS (rw) register accessor: SPI1 flash suspend status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_sus_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_sus_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_sus_status`] module"]
pub type SPI_MEM_SUS_STATUS = crate::Reg<spi_mem_sus_status::SPI_MEM_SUS_STATUS_SPEC>;
#[doc = "SPI1 flash suspend status register"]
pub mod spi_mem_sus_status;
#[doc = "SPI_MEM_INT_ENA (rw) register accessor: SPI1 interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_int_ena`] module"]
pub type SPI_MEM_INT_ENA = crate::Reg<spi_mem_int_ena::SPI_MEM_INT_ENA_SPEC>;
#[doc = "SPI1 interrupt enable register"]
pub mod spi_mem_int_ena;
#[doc = "SPI_MEM_INT_CLR (w) register accessor: SPI1 interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_int_clr`] module"]
pub type SPI_MEM_INT_CLR = crate::Reg<spi_mem_int_clr::SPI_MEM_INT_CLR_SPEC>;
#[doc = "SPI1 interrupt clear register"]
pub mod spi_mem_int_clr;
#[doc = "SPI_MEM_INT_RAW (rw) register accessor: SPI1 interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_int_raw`] module"]
pub type SPI_MEM_INT_RAW = crate::Reg<spi_mem_int_raw::SPI_MEM_INT_RAW_SPEC>;
#[doc = "SPI1 interrupt raw register"]
pub mod spi_mem_int_raw;
#[doc = "SPI_MEM_INT_ST (r) register accessor: SPI1 interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_int_st`] module"]
pub type SPI_MEM_INT_ST = crate::Reg<spi_mem_int_st::SPI_MEM_INT_ST_SPEC>;
#[doc = "SPI1 interrupt status register"]
pub mod spi_mem_int_st;
#[doc = "SPI_MEM_DDR (rw) register accessor: SPI1 DDR control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_ddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_ddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_ddr`] module"]
pub type SPI_MEM_DDR = crate::Reg<spi_mem_ddr::SPI_MEM_DDR_SPEC>;
#[doc = "SPI1 DDR control register"]
pub mod spi_mem_ddr;
#[doc = "SPI_MEM_TIMING_CALI (rw) register accessor: SPI1 timing control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_timing_cali::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_timing_cali::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_timing_cali`] module"]
pub type SPI_MEM_TIMING_CALI = crate::Reg<spi_mem_timing_cali::SPI_MEM_TIMING_CALI_SPEC>;
#[doc = "SPI1 timing control register"]
pub mod spi_mem_timing_cali;
#[doc = "SPI_MEM_CLOCK_GATE (rw) register accessor: SPI1 clk_gate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_clock_gate`] module"]
pub type SPI_MEM_CLOCK_GATE = crate::Reg<spi_mem_clock_gate::SPI_MEM_CLOCK_GATE_SPEC>;
#[doc = "SPI1 clk_gate register"]
pub mod spi_mem_clock_gate;
#[doc = "SPI_MEM_DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_date`] module"]
pub type SPI_MEM_DATE = crate::Reg<spi_mem_date::SPI_MEM_DATE_SPEC>;
#[doc = "Version control register"]
pub mod spi_mem_date;
