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
    ext_addr: EXT_ADDR,
    misc: MISC,
    tx_crc: TX_CRC,
    cache_fctrl: CACHE_FCTRL,
    cache_sctrl: CACHE_SCTRL,
    sram_cmd: SRAM_CMD,
    sram_drd_cmd: SRAM_DRD_CMD,
    sram_dwr_cmd: SRAM_DWR_CMD,
    sram_clk: SRAM_CLK,
    fsm: FSM,
    w: [W; 16],
    flash_waiti_ctrl: FLASH_WAITI_CTRL,
    flash_sus_cmd: FLASH_SUS_CMD,
    flash_sus_ctrl: FLASH_SUS_CTRL,
    sus_status: SUS_STATUS,
    timing_cali: TIMING_CALI,
    din_mode: DIN_MODE,
    din_num: DIN_NUM,
    dout_mode: DOUT_MODE,
    dout_num: DOUT_NUM,
    spi_smem_timing_cali: SPI_SMEM_TIMING_CALI,
    spi_smem_din_mode: SPI_SMEM_DIN_MODE,
    spi_smem_din_num: SPI_SMEM_DIN_NUM,
    spi_smem_dout_mode: SPI_SMEM_DOUT_MODE,
    spi_smem_dout_num: SPI_SMEM_DOUT_NUM,
    spi_smem_ac: SPI_SMEM_AC,
    ddr: DDR,
    spi_smem_ddr: SPI_SMEM_DDR,
    clock_gate: CLOCK_GATE,
    _reserved41: [u8; 0x031c],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI Memory Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x04 - SPI Memory Address Register"]
    #[inline(always)]
    pub const fn addr(&self) -> &ADDR {
        &self.addr
    }
    #[doc = "0x08 - SPI Memory Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x0c - SPI Memory Control1 Register"]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &CTRL1 {
        &self.ctrl1
    }
    #[doc = "0x10 - SPI Memory Control2 Register"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &CTRL2 {
        &self.ctrl2
    }
    #[doc = "0x14 - SPI Memory Clock Register"]
    #[inline(always)]
    pub const fn clock(&self) -> &CLOCK {
        &self.clock
    }
    #[doc = "0x18 - SPI Memory User Register"]
    #[inline(always)]
    pub const fn user(&self) -> &USER {
        &self.user
    }
    #[doc = "0x1c - SPI Memory User1 Register"]
    #[inline(always)]
    pub const fn user1(&self) -> &USER1 {
        &self.user1
    }
    #[doc = "0x20 - SPI Memory User2 Register"]
    #[inline(always)]
    pub const fn user2(&self) -> &USER2 {
        &self.user2
    }
    #[doc = "0x24 - SPI Memory MOSI Data Length Register"]
    #[inline(always)]
    pub const fn mosi_dlen(&self) -> &MOSI_DLEN {
        &self.mosi_dlen
    }
    #[doc = "0x28 - SPI Memory MISO Data Length Register"]
    #[inline(always)]
    pub const fn miso_dlen(&self) -> &MISO_DLEN {
        &self.miso_dlen
    }
    #[doc = "0x2c - SPI Memory Read Status Register"]
    #[inline(always)]
    pub const fn rd_status(&self) -> &RD_STATUS {
        &self.rd_status
    }
    #[doc = "0x30 - SPI Memory External Address Register"]
    #[inline(always)]
    pub const fn ext_addr(&self) -> &EXT_ADDR {
        &self.ext_addr
    }
    #[doc = "0x34 - SPI Memory Miscellaneous Register"]
    #[inline(always)]
    pub const fn misc(&self) -> &MISC {
        &self.misc
    }
    #[doc = "0x38 - SPI Memory Transmit CRC Register"]
    #[inline(always)]
    pub const fn tx_crc(&self) -> &TX_CRC {
        &self.tx_crc
    }
    #[doc = "0x3c - SPI Memory Cache Flash Control Register"]
    #[inline(always)]
    pub const fn cache_fctrl(&self) -> &CACHE_FCTRL {
        &self.cache_fctrl
    }
    #[doc = "0x40 - SPI Memory Cache SCTRL Register"]
    #[inline(always)]
    pub const fn cache_sctrl(&self) -> &CACHE_SCTRL {
        &self.cache_sctrl
    }
    #[doc = "0x44 - SPI Memory SRAM Command Register"]
    #[inline(always)]
    pub const fn sram_cmd(&self) -> &SRAM_CMD {
        &self.sram_cmd
    }
    #[doc = "0x48 - SPI Memory SRAM Read Command Register"]
    #[inline(always)]
    pub const fn sram_drd_cmd(&self) -> &SRAM_DRD_CMD {
        &self.sram_drd_cmd
    }
    #[doc = "0x4c - SPI Memory SRAM Write Command Register"]
    #[inline(always)]
    pub const fn sram_dwr_cmd(&self) -> &SRAM_DWR_CMD {
        &self.sram_dwr_cmd
    }
    #[doc = "0x50 - SPI Memory SRAM Clock Register"]
    #[inline(always)]
    pub const fn sram_clk(&self) -> &SRAM_CLK {
        &self.sram_clk
    }
    #[doc = "0x54 - SPI Memory FSM Register"]
    #[inline(always)]
    pub const fn fsm(&self) -> &FSM {
        &self.fsm
    }
    #[doc = "0x58..0x98 - "]
    #[inline(always)]
    pub const fn w(&self, n: usize) -> &W {
        &self.w[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x58..0x98 - "]
    #[inline(always)]
    pub fn w_iter(&self) -> impl Iterator<Item = &W> {
        self.w.iter()
    }
    #[doc = "0x98 - SPI Memory Flash Wait Idle Control Register"]
    #[inline(always)]
    pub const fn flash_waiti_ctrl(&self) -> &FLASH_WAITI_CTRL {
        &self.flash_waiti_ctrl
    }
    #[doc = "0x9c - SPI Memory Flash Suspend Command Register"]
    #[inline(always)]
    pub const fn flash_sus_cmd(&self) -> &FLASH_SUS_CMD {
        &self.flash_sus_cmd
    }
    #[doc = "0xa0 - SPI Memory Flash Suspend Control Register"]
    #[inline(always)]
    pub const fn flash_sus_ctrl(&self) -> &FLASH_SUS_CTRL {
        &self.flash_sus_ctrl
    }
    #[doc = "0xa4 - SPI Memory Suspend Status Register"]
    #[inline(always)]
    pub const fn sus_status(&self) -> &SUS_STATUS {
        &self.sus_status
    }
    #[doc = "0xa8 - SPI Memory Timing Calibration Register"]
    #[inline(always)]
    pub const fn timing_cali(&self) -> &TIMING_CALI {
        &self.timing_cali
    }
    #[doc = "0xac - SPI Memory Data In Mode Register"]
    #[inline(always)]
    pub const fn din_mode(&self) -> &DIN_MODE {
        &self.din_mode
    }
    #[doc = "0xb0 - SPI Memory Data In Number Register"]
    #[inline(always)]
    pub const fn din_num(&self) -> &DIN_NUM {
        &self.din_num
    }
    #[doc = "0xb4 - SPI Memory Data Out Mode Register"]
    #[inline(always)]
    pub const fn dout_mode(&self) -> &DOUT_MODE {
        &self.dout_mode
    }
    #[doc = "0xb8 - SPI Memory Data Out Number Register"]
    #[inline(always)]
    pub const fn dout_num(&self) -> &DOUT_NUM {
        &self.dout_num
    }
    #[doc = "0xbc - SPI Memory SRAM Timing Calibration Register"]
    #[inline(always)]
    pub const fn spi_smem_timing_cali(&self) -> &SPI_SMEM_TIMING_CALI {
        &self.spi_smem_timing_cali
    }
    #[doc = "0xc0 - SPI Memory SRAM Data In Mode Register"]
    #[inline(always)]
    pub const fn spi_smem_din_mode(&self) -> &SPI_SMEM_DIN_MODE {
        &self.spi_smem_din_mode
    }
    #[doc = "0xc4 - SPI Memory SRAM Data In Number Register"]
    #[inline(always)]
    pub const fn spi_smem_din_num(&self) -> &SPI_SMEM_DIN_NUM {
        &self.spi_smem_din_num
    }
    #[doc = "0xc8 - SPI Memory SRAM Data Out Mode Register"]
    #[inline(always)]
    pub const fn spi_smem_dout_mode(&self) -> &SPI_SMEM_DOUT_MODE {
        &self.spi_smem_dout_mode
    }
    #[doc = "0xcc - SPI Memory SRAM Data Out Number Register"]
    #[inline(always)]
    pub const fn spi_smem_dout_num(&self) -> &SPI_SMEM_DOUT_NUM {
        &self.spi_smem_dout_num
    }
    #[doc = "0xd0 - SPI Memory SRAM Access Register"]
    #[inline(always)]
    pub const fn spi_smem_ac(&self) -> &SPI_SMEM_AC {
        &self.spi_smem_ac
    }
    #[doc = "0xd4 - SPI Memory DDR Register"]
    #[inline(always)]
    pub const fn ddr(&self) -> &DDR {
        &self.ddr
    }
    #[doc = "0xd8 - SPI Memory SRAM DDR Register"]
    #[inline(always)]
    pub const fn spi_smem_ddr(&self) -> &SPI_SMEM_DDR {
        &self.spi_smem_ddr
    }
    #[doc = "0xdc - SPI Memory Clock Gate Register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x3fc - SPI Memory Date Register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CMD (rw) register accessor: SPI Memory Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "SPI Memory Command Register"]
pub mod cmd;
#[doc = "ADDR (rw) register accessor: SPI Memory Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`] module"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "SPI Memory Address Register"]
pub mod addr;
#[doc = "CTRL (rw) register accessor: SPI Memory Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "SPI Memory Control Register"]
pub mod ctrl;
#[doc = "CTRL1 (rw) register accessor: SPI Memory Control1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`] module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "SPI Memory Control1 Register"]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: SPI Memory Control2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`] module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "SPI Memory Control2 Register"]
pub mod ctrl2;
#[doc = "CLOCK (rw) register accessor: SPI Memory Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock`] module"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = "SPI Memory Clock Register"]
pub mod clock;
#[doc = "USER (rw) register accessor: SPI Memory User Register\n\nYou can [`read`](crate::Reg::read) this register and get [`user::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user`] module"]
pub type USER = crate::Reg<user::USER_SPEC>;
#[doc = "SPI Memory User Register"]
pub mod user;
#[doc = "USER1 (rw) register accessor: SPI Memory User1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`user1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user1`] module"]
pub type USER1 = crate::Reg<user1::USER1_SPEC>;
#[doc = "SPI Memory User1 Register"]
pub mod user1;
#[doc = "USER2 (rw) register accessor: SPI Memory User2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`user2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user2`] module"]
pub type USER2 = crate::Reg<user2::USER2_SPEC>;
#[doc = "SPI Memory User2 Register"]
pub mod user2;
#[doc = "MOSI_DLEN (rw) register accessor: SPI Memory MOSI Data Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mosi_dlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mosi_dlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mosi_dlen`] module"]
pub type MOSI_DLEN = crate::Reg<mosi_dlen::MOSI_DLEN_SPEC>;
#[doc = "SPI Memory MOSI Data Length Register"]
pub mod mosi_dlen;
#[doc = "MISO_DLEN (rw) register accessor: SPI Memory MISO Data Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`miso_dlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miso_dlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@miso_dlen`] module"]
pub type MISO_DLEN = crate::Reg<miso_dlen::MISO_DLEN_SPEC>;
#[doc = "SPI Memory MISO Data Length Register"]
pub mod miso_dlen;
#[doc = "RD_STATUS (rw) register accessor: SPI Memory Read Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_status`] module"]
pub type RD_STATUS = crate::Reg<rd_status::RD_STATUS_SPEC>;
#[doc = "SPI Memory Read Status Register"]
pub mod rd_status;
#[doc = "EXT_ADDR (rw) register accessor: SPI Memory External Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_addr`] module"]
pub type EXT_ADDR = crate::Reg<ext_addr::EXT_ADDR_SPEC>;
#[doc = "SPI Memory External Address Register"]
pub mod ext_addr;
#[doc = "MISC (rw) register accessor: SPI Memory Miscellaneous Register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc`] module"]
pub type MISC = crate::Reg<misc::MISC_SPEC>;
#[doc = "SPI Memory Miscellaneous Register"]
pub mod misc;
#[doc = "TX_CRC (rw) register accessor: SPI Memory Transmit CRC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_crc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_crc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_crc`] module"]
pub type TX_CRC = crate::Reg<tx_crc::TX_CRC_SPEC>;
#[doc = "SPI Memory Transmit CRC Register"]
pub mod tx_crc;
#[doc = "CACHE_FCTRL (rw) register accessor: SPI Memory Cache Flash Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_fctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_fctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_fctrl`] module"]
pub type CACHE_FCTRL = crate::Reg<cache_fctrl::CACHE_FCTRL_SPEC>;
#[doc = "SPI Memory Cache Flash Control Register"]
pub mod cache_fctrl;
#[doc = "CACHE_SCTRL (rw) register accessor: SPI Memory Cache SCTRL Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_sctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_sctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_sctrl`] module"]
pub type CACHE_SCTRL = crate::Reg<cache_sctrl::CACHE_SCTRL_SPEC>;
#[doc = "SPI Memory Cache SCTRL Register"]
pub mod cache_sctrl;
#[doc = "SRAM_CMD (rw) register accessor: SPI Memory SRAM Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_cmd`] module"]
pub type SRAM_CMD = crate::Reg<sram_cmd::SRAM_CMD_SPEC>;
#[doc = "SPI Memory SRAM Command Register"]
pub mod sram_cmd;
#[doc = "SRAM_DRD_CMD (rw) register accessor: SPI Memory SRAM Read Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_drd_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_drd_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_drd_cmd`] module"]
pub type SRAM_DRD_CMD = crate::Reg<sram_drd_cmd::SRAM_DRD_CMD_SPEC>;
#[doc = "SPI Memory SRAM Read Command Register"]
pub mod sram_drd_cmd;
#[doc = "SRAM_DWR_CMD (rw) register accessor: SPI Memory SRAM Write Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_dwr_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_dwr_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_dwr_cmd`] module"]
pub type SRAM_DWR_CMD = crate::Reg<sram_dwr_cmd::SRAM_DWR_CMD_SPEC>;
#[doc = "SPI Memory SRAM Write Command Register"]
pub mod sram_dwr_cmd;
#[doc = "SRAM_CLK (rw) register accessor: SPI Memory SRAM Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_clk`] module"]
pub type SRAM_CLK = crate::Reg<sram_clk::SRAM_CLK_SPEC>;
#[doc = "SPI Memory SRAM Clock Register"]
pub mod sram_clk;
#[doc = "FSM (r) register accessor: SPI Memory FSM Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fsm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm`] module"]
pub type FSM = crate::Reg<fsm::FSM_SPEC>;
#[doc = "SPI Memory FSM Register"]
pub mod fsm;
#[doc = "W (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w`] module"]
pub type W = crate::Reg<w::W_SPEC>;
#[doc = ""]
pub mod w;
#[doc = "FLASH_WAITI_CTRL (rw) register accessor: SPI Memory Flash Wait Idle Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_waiti_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_waiti_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_waiti_ctrl`] module"]
pub type FLASH_WAITI_CTRL = crate::Reg<flash_waiti_ctrl::FLASH_WAITI_CTRL_SPEC>;
#[doc = "SPI Memory Flash Wait Idle Control Register"]
pub mod flash_waiti_ctrl;
#[doc = "FLASH_SUS_CMD (rw) register accessor: SPI Memory Flash Suspend Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_sus_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_sus_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_sus_cmd`] module"]
pub type FLASH_SUS_CMD = crate::Reg<flash_sus_cmd::FLASH_SUS_CMD_SPEC>;
#[doc = "SPI Memory Flash Suspend Command Register"]
pub mod flash_sus_cmd;
#[doc = "FLASH_SUS_CTRL (rw) register accessor: SPI Memory Flash Suspend Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_sus_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_sus_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_sus_ctrl`] module"]
pub type FLASH_SUS_CTRL = crate::Reg<flash_sus_ctrl::FLASH_SUS_CTRL_SPEC>;
#[doc = "SPI Memory Flash Suspend Control Register"]
pub mod flash_sus_ctrl;
#[doc = "SUS_STATUS (r) register accessor: SPI Memory Suspend Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sus_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sus_status`] module"]
pub type SUS_STATUS = crate::Reg<sus_status::SUS_STATUS_SPEC>;
#[doc = "SPI Memory Suspend Status Register"]
pub mod sus_status;
#[doc = "TIMING_CALI (rw) register accessor: SPI Memory Timing Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timing_cali::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timing_cali::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timing_cali`] module"]
pub type TIMING_CALI = crate::Reg<timing_cali::TIMING_CALI_SPEC>;
#[doc = "SPI Memory Timing Calibration Register"]
pub mod timing_cali;
#[doc = "DIN_MODE (rw) register accessor: SPI Memory Data In Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`din_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din_mode`] module"]
pub type DIN_MODE = crate::Reg<din_mode::DIN_MODE_SPEC>;
#[doc = "SPI Memory Data In Mode Register"]
pub mod din_mode;
#[doc = "DIN_NUM (rw) register accessor: SPI Memory Data In Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`din_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din_num`] module"]
pub type DIN_NUM = crate::Reg<din_num::DIN_NUM_SPEC>;
#[doc = "SPI Memory Data In Number Register"]
pub mod din_num;
#[doc = "DOUT_MODE (rw) register accessor: SPI Memory Data Out Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dout_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout_mode`] module"]
pub type DOUT_MODE = crate::Reg<dout_mode::DOUT_MODE_SPEC>;
#[doc = "SPI Memory Data Out Mode Register"]
pub mod dout_mode;
#[doc = "DOUT_NUM (rw) register accessor: SPI Memory Data Out Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dout_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout_num`] module"]
pub type DOUT_NUM = crate::Reg<dout_num::DOUT_NUM_SPEC>;
#[doc = "SPI Memory Data Out Number Register"]
pub mod dout_num;
#[doc = "SPI_SMEM_TIMING_CALI (rw) register accessor: SPI Memory SRAM Timing Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_timing_cali::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_timing_cali::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_timing_cali`] module"]
pub type SPI_SMEM_TIMING_CALI = crate::Reg<spi_smem_timing_cali::SPI_SMEM_TIMING_CALI_SPEC>;
#[doc = "SPI Memory SRAM Timing Calibration Register"]
pub mod spi_smem_timing_cali;
#[doc = "SPI_SMEM_DIN_MODE (rw) register accessor: SPI Memory SRAM Data In Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_din_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_din_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_din_mode`] module"]
pub type SPI_SMEM_DIN_MODE = crate::Reg<spi_smem_din_mode::SPI_SMEM_DIN_MODE_SPEC>;
#[doc = "SPI Memory SRAM Data In Mode Register"]
pub mod spi_smem_din_mode;
#[doc = "SPI_SMEM_DIN_NUM (rw) register accessor: SPI Memory SRAM Data In Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_din_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_din_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_din_num`] module"]
pub type SPI_SMEM_DIN_NUM = crate::Reg<spi_smem_din_num::SPI_SMEM_DIN_NUM_SPEC>;
#[doc = "SPI Memory SRAM Data In Number Register"]
pub mod spi_smem_din_num;
#[doc = "SPI_SMEM_DOUT_MODE (rw) register accessor: SPI Memory SRAM Data Out Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_dout_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_dout_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_dout_mode`] module"]
pub type SPI_SMEM_DOUT_MODE = crate::Reg<spi_smem_dout_mode::SPI_SMEM_DOUT_MODE_SPEC>;
#[doc = "SPI Memory SRAM Data Out Mode Register"]
pub mod spi_smem_dout_mode;
#[doc = "SPI_SMEM_DOUT_NUM (rw) register accessor: SPI Memory SRAM Data Out Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_dout_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_dout_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_dout_num`] module"]
pub type SPI_SMEM_DOUT_NUM = crate::Reg<spi_smem_dout_num::SPI_SMEM_DOUT_NUM_SPEC>;
#[doc = "SPI Memory SRAM Data Out Number Register"]
pub mod spi_smem_dout_num;
#[doc = "SPI_SMEM_AC (rw) register accessor: SPI Memory SRAM Access Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_ac`] module"]
pub type SPI_SMEM_AC = crate::Reg<spi_smem_ac::SPI_SMEM_AC_SPEC>;
#[doc = "SPI Memory SRAM Access Register"]
pub mod spi_smem_ac;
#[doc = "DDR (rw) register accessor: SPI Memory DDR Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr`] module"]
pub type DDR = crate::Reg<ddr::DDR_SPEC>;
#[doc = "SPI Memory DDR Register"]
pub mod ddr;
#[doc = "SPI_SMEM_DDR (rw) register accessor: SPI Memory SRAM DDR Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_ddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_ddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_ddr`] module"]
pub type SPI_SMEM_DDR = crate::Reg<spi_smem_ddr::SPI_SMEM_DDR_SPEC>;
#[doc = "SPI Memory SRAM DDR Register"]
pub mod spi_smem_ddr;
#[doc = "CLOCK_GATE (rw) register accessor: SPI Memory Clock Gate Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "SPI Memory Clock Gate Register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: SPI Memory Date Register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "SPI Memory Date Register"]
pub mod date;
