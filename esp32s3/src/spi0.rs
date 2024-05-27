#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    ctrl: CTRL,
    ctrl1: CTRL1,
    ctrl2: CTRL2,
    clock: CLOCK,
    user: USER,
    user1: USER1,
    user2: USER2,
    _reserved7: [u8; 0x08],
    rd_status: RD_STATUS,
    ext_addr: EXT_ADDR,
    misc: MISC,
    _reserved10: [u8; 0x04],
    cache_fctrl: CACHE_FCTRL,
    cache_sctrl: CACHE_SCTRL,
    sram_cmd: SRAM_CMD,
    sram_drd_cmd: SRAM_DRD_CMD,
    sram_dwr_cmd: SRAM_DWR_CMD,
    sram_clk: SRAM_CLK,
    fsm: FSM,
    _reserved17: [u8; 0x50],
    timing_cali: TIMING_CALI,
    din_mode: DIN_MODE,
    din_num: DIN_NUM,
    dout_mode: DOUT_MODE,
    _reserved21: [u8; 0x04],
    spi_smem_timing_cali: SPI_SMEM_TIMING_CALI,
    spi_smem_din_mode: SPI_SMEM_DIN_MODE,
    spi_smem_din_num: SPI_SMEM_DIN_NUM,
    spi_smem_dout_mode: SPI_SMEM_DOUT_MODE,
    ecc_ctrl: ECC_CTRL,
    ecc_err_addr: ECC_ERR_ADDR,
    ecc_err_bit: ECC_ERR_BIT,
    _reserved28: [u8; 0x04],
    spi_smem_ac: SPI_SMEM_AC,
    ddr: DDR,
    spi_smem_ddr: SPI_SMEM_DDR,
    clock_gate: CLOCK_GATE,
    core_clk_sel: CORE_CLK_SEL,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    int_raw: INT_RAW,
    int_st: INT_ST,
    _reserved37: [u8; 0x02fc],
    date: DATE,
}
impl RegisterBlock {
    ///0x08 - SPI0 control register.
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    ///0x0c - SPI0 control 1 register.
    #[inline(always)]
    pub const fn ctrl1(&self) -> &CTRL1 {
        &self.ctrl1
    }
    ///0x10 - SPI0 control 2 register.
    #[inline(always)]
    pub const fn ctrl2(&self) -> &CTRL2 {
        &self.ctrl2
    }
    ///0x14 - SPI_CLK clock division register when SPI0 accesses to flash.
    #[inline(always)]
    pub const fn clock(&self) -> &CLOCK {
        &self.clock
    }
    ///0x18 - SPI0 user register.
    #[inline(always)]
    pub const fn user(&self) -> &USER {
        &self.user
    }
    ///0x1c - SPI0 user1 register.
    #[inline(always)]
    pub const fn user1(&self) -> &USER1 {
        &self.user1
    }
    ///0x20 - SPI0 user2 register.
    #[inline(always)]
    pub const fn user2(&self) -> &USER2 {
        &self.user2
    }
    ///0x2c - SPI0 read control register.
    #[inline(always)]
    pub const fn rd_status(&self) -> &RD_STATUS {
        &self.rd_status
    }
    ///0x30 - SPI0 extended address register.
    #[inline(always)]
    pub const fn ext_addr(&self) -> &EXT_ADDR {
        &self.ext_addr
    }
    ///0x34 - SPI0 misc register
    #[inline(always)]
    pub const fn misc(&self) -> &MISC {
        &self.misc
    }
    ///0x3c - SPI0 external RAM bit mode control register.
    #[inline(always)]
    pub const fn cache_fctrl(&self) -> &CACHE_FCTRL {
        &self.cache_fctrl
    }
    ///0x40 - SPI0 external RAM control register
    #[inline(always)]
    pub const fn cache_sctrl(&self) -> &CACHE_SCTRL {
        &self.cache_sctrl
    }
    ///0x44 - SPI0 external RAM mode control register
    #[inline(always)]
    pub const fn sram_cmd(&self) -> &SRAM_CMD {
        &self.sram_cmd
    }
    ///0x48 - SPI0 external RAM DDR read command control register
    #[inline(always)]
    pub const fn sram_drd_cmd(&self) -> &SRAM_DRD_CMD {
        &self.sram_drd_cmd
    }
    ///0x4c - SPI0 external RAM DDR write command control register
    #[inline(always)]
    pub const fn sram_dwr_cmd(&self) -> &SRAM_DWR_CMD {
        &self.sram_dwr_cmd
    }
    ///0x50 - SPI_CLK clock division register when SPI0 accesses to Ext_RAM.
    #[inline(always)]
    pub const fn sram_clk(&self) -> &SRAM_CLK {
        &self.sram_clk
    }
    ///0x54 - SPI0 state machine(FSM) status register.
    #[inline(always)]
    pub const fn fsm(&self) -> &FSM {
        &self.fsm
    }
    ///0xa8 - SPI0 timing compensation register when accesses to flash.
    #[inline(always)]
    pub const fn timing_cali(&self) -> &TIMING_CALI {
        &self.timing_cali
    }
    ///0xac - MSPI input timing delay mode control register when accesses to flash.
    #[inline(always)]
    pub const fn din_mode(&self) -> &DIN_MODE {
        &self.din_mode
    }
    ///0xb0 - MSPI input timing delay number control register when accesses to flash.
    #[inline(always)]
    pub const fn din_num(&self) -> &DIN_NUM {
        &self.din_num
    }
    ///0xb4 - MSPI output timing delay mode control register when accesses to flash.
    #[inline(always)]
    pub const fn dout_mode(&self) -> &DOUT_MODE {
        &self.dout_mode
    }
    ///0xbc - SPI0 Ext_RAM timing compensation register.
    #[inline(always)]
    pub const fn spi_smem_timing_cali(&self) -> &SPI_SMEM_TIMING_CALI {
        &self.spi_smem_timing_cali
    }
    ///0xc0 - MSPI input timing delay mode control register when accesses to Ext_RAM.
    #[inline(always)]
    pub const fn spi_smem_din_mode(&self) -> &SPI_SMEM_DIN_MODE {
        &self.spi_smem_din_mode
    }
    ///0xc4 - MSPI input timing delay number control register when accesses to Ext_RAM.
    #[inline(always)]
    pub const fn spi_smem_din_num(&self) -> &SPI_SMEM_DIN_NUM {
        &self.spi_smem_din_num
    }
    ///0xc8 - MSPI output timing delay mode control register when accesses to Ext_RAM.
    #[inline(always)]
    pub const fn spi_smem_dout_mode(&self) -> &SPI_SMEM_DOUT_MODE {
        &self.spi_smem_dout_mode
    }
    ///0xcc - MSPI ECC control register
    #[inline(always)]
    pub const fn ecc_ctrl(&self) -> &ECC_CTRL {
        &self.ecc_ctrl
    }
    ///0xd0 - MSPI ECC error address register
    #[inline(always)]
    pub const fn ecc_err_addr(&self) -> &ECC_ERR_ADDR {
        &self.ecc_err_addr
    }
    ///0xd4 - MSPI ECC error bits register
    #[inline(always)]
    pub const fn ecc_err_bit(&self) -> &ECC_ERR_BIT {
        &self.ecc_err_bit
    }
    ///0xdc - MSPI external RAM ECC and SPI CS timing control register
    #[inline(always)]
    pub const fn spi_smem_ac(&self) -> &SPI_SMEM_AC {
        &self.spi_smem_ac
    }
    ///0xe0 - SPI0 flash DDR mode control register
    #[inline(always)]
    pub const fn ddr(&self) -> &DDR {
        &self.ddr
    }
    ///0xe4 - SPI0 external RAM DDR mode control register
    #[inline(always)]
    pub const fn spi_smem_ddr(&self) -> &SPI_SMEM_DDR {
        &self.spi_smem_ddr
    }
    ///0xe8 - SPI0 clk_gate register
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    ///0xec - SPI0 module clock select register
    #[inline(always)]
    pub const fn core_clk_sel(&self) -> &CORE_CLK_SEL {
        &self.core_clk_sel
    }
    ///0xf0 - SPI1 interrupt enable register
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0xf4 - SPI1 interrupt clear register
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0xf8 - SPI1 interrupt raw register
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0xfc - SPI1 interrupt status register
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x3fc - SPI0 version control register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**CTRL (rw) register accessor: SPI0 control register.

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl`] module*/
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
///SPI0 control register.
pub mod ctrl;
/**CTRL1 (rw) register accessor: SPI0 control 1 register.

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl1`] module*/
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
///SPI0 control 1 register.
pub mod ctrl1;
/**CTRL2 (rw) register accessor: SPI0 control 2 register.

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl2`] module*/
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
///SPI0 control 2 register.
pub mod ctrl2;
/**CLOCK (rw) register accessor: SPI_CLK clock division register when SPI0 accesses to flash.

You can [`read`](crate::generic::Reg::read) this register and get [`clock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clock`] module*/
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
///SPI_CLK clock division register when SPI0 accesses to flash.
pub mod clock;
/**USER (rw) register accessor: SPI0 user register.

You can [`read`](crate::generic::Reg::read) this register and get [`user::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@user`] module*/
pub type USER = crate::Reg<user::USER_SPEC>;
///SPI0 user register.
pub mod user;
/**USER1 (rw) register accessor: SPI0 user1 register.

You can [`read`](crate::generic::Reg::read) this register and get [`user1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@user1`] module*/
pub type USER1 = crate::Reg<user1::USER1_SPEC>;
///SPI0 user1 register.
pub mod user1;
/**USER2 (rw) register accessor: SPI0 user2 register.

You can [`read`](crate::generic::Reg::read) this register and get [`user2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@user2`] module*/
pub type USER2 = crate::Reg<user2::USER2_SPEC>;
///SPI0 user2 register.
pub mod user2;
/**RD_STATUS (rw) register accessor: SPI0 read control register.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_status`] module*/
pub type RD_STATUS = crate::Reg<rd_status::RD_STATUS_SPEC>;
///SPI0 read control register.
pub mod rd_status;
/**EXT_ADDR (rw) register accessor: SPI0 extended address register.

You can [`read`](crate::generic::Reg::read) this register and get [`ext_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_addr`] module*/
pub type EXT_ADDR = crate::Reg<ext_addr::EXT_ADDR_SPEC>;
///SPI0 extended address register.
pub mod ext_addr;
/**MISC (rw) register accessor: SPI0 misc register

You can [`read`](crate::generic::Reg::read) this register and get [`misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@misc`] module*/
pub type MISC = crate::Reg<misc::MISC_SPEC>;
///SPI0 misc register
pub mod misc;
/**CACHE_FCTRL (rw) register accessor: SPI0 external RAM bit mode control register.

You can [`read`](crate::generic::Reg::read) this register and get [`cache_fctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_fctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cache_fctrl`] module*/
pub type CACHE_FCTRL = crate::Reg<cache_fctrl::CACHE_FCTRL_SPEC>;
///SPI0 external RAM bit mode control register.
pub mod cache_fctrl;
/**CACHE_SCTRL (rw) register accessor: SPI0 external RAM control register

You can [`read`](crate::generic::Reg::read) this register and get [`cache_sctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_sctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cache_sctrl`] module*/
pub type CACHE_SCTRL = crate::Reg<cache_sctrl::CACHE_SCTRL_SPEC>;
///SPI0 external RAM control register
pub mod cache_sctrl;
/**SRAM_CMD (rw) register accessor: SPI0 external RAM mode control register

You can [`read`](crate::generic::Reg::read) this register and get [`sram_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sram_cmd`] module*/
pub type SRAM_CMD = crate::Reg<sram_cmd::SRAM_CMD_SPEC>;
///SPI0 external RAM mode control register
pub mod sram_cmd;
/**SRAM_DRD_CMD (rw) register accessor: SPI0 external RAM DDR read command control register

You can [`read`](crate::generic::Reg::read) this register and get [`sram_drd_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_drd_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sram_drd_cmd`] module*/
pub type SRAM_DRD_CMD = crate::Reg<sram_drd_cmd::SRAM_DRD_CMD_SPEC>;
///SPI0 external RAM DDR read command control register
pub mod sram_drd_cmd;
/**SRAM_DWR_CMD (rw) register accessor: SPI0 external RAM DDR write command control register

You can [`read`](crate::generic::Reg::read) this register and get [`sram_dwr_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_dwr_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sram_dwr_cmd`] module*/
pub type SRAM_DWR_CMD = crate::Reg<sram_dwr_cmd::SRAM_DWR_CMD_SPEC>;
///SPI0 external RAM DDR write command control register
pub mod sram_dwr_cmd;
/**SRAM_CLK (rw) register accessor: SPI_CLK clock division register when SPI0 accesses to Ext_RAM.

You can [`read`](crate::generic::Reg::read) this register and get [`sram_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sram_clk`] module*/
pub type SRAM_CLK = crate::Reg<sram_clk::SRAM_CLK_SPEC>;
///SPI_CLK clock division register when SPI0 accesses to Ext_RAM.
pub mod sram_clk;
/**FSM (r) register accessor: SPI0 state machine(FSM) status register.

You can [`read`](crate::generic::Reg::read) this register and get [`fsm::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fsm`] module*/
pub type FSM = crate::Reg<fsm::FSM_SPEC>;
///SPI0 state machine(FSM) status register.
pub mod fsm;
/**TIMING_CALI (rw) register accessor: SPI0 timing compensation register when accesses to flash.

You can [`read`](crate::generic::Reg::read) this register and get [`timing_cali::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timing_cali::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timing_cali`] module*/
pub type TIMING_CALI = crate::Reg<timing_cali::TIMING_CALI_SPEC>;
///SPI0 timing compensation register when accesses to flash.
pub mod timing_cali;
/**DIN_MODE (rw) register accessor: MSPI input timing delay mode control register when accesses to flash.

You can [`read`](crate::generic::Reg::read) this register and get [`din_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@din_mode`] module*/
pub type DIN_MODE = crate::Reg<din_mode::DIN_MODE_SPEC>;
///MSPI input timing delay mode control register when accesses to flash.
pub mod din_mode;
/**DIN_NUM (rw) register accessor: MSPI input timing delay number control register when accesses to flash.

You can [`read`](crate::generic::Reg::read) this register and get [`din_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@din_num`] module*/
pub type DIN_NUM = crate::Reg<din_num::DIN_NUM_SPEC>;
///MSPI input timing delay number control register when accesses to flash.
pub mod din_num;
/**DOUT_MODE (rw) register accessor: MSPI output timing delay mode control register when accesses to flash.

You can [`read`](crate::generic::Reg::read) this register and get [`dout_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dout_mode`] module*/
pub type DOUT_MODE = crate::Reg<dout_mode::DOUT_MODE_SPEC>;
///MSPI output timing delay mode control register when accesses to flash.
pub mod dout_mode;
/**SPI_SMEM_TIMING_CALI (rw) register accessor: SPI0 Ext_RAM timing compensation register.

You can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_timing_cali::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_smem_timing_cali::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi_smem_timing_cali`] module*/
pub type SPI_SMEM_TIMING_CALI = crate::Reg<spi_smem_timing_cali::SPI_SMEM_TIMING_CALI_SPEC>;
///SPI0 Ext_RAM timing compensation register.
pub mod spi_smem_timing_cali;
/**SPI_SMEM_DIN_MODE (rw) register accessor: MSPI input timing delay mode control register when accesses to Ext_RAM.

You can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_din_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_smem_din_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi_smem_din_mode`] module*/
pub type SPI_SMEM_DIN_MODE = crate::Reg<spi_smem_din_mode::SPI_SMEM_DIN_MODE_SPEC>;
///MSPI input timing delay mode control register when accesses to Ext_RAM.
pub mod spi_smem_din_mode;
/**SPI_SMEM_DIN_NUM (rw) register accessor: MSPI input timing delay number control register when accesses to Ext_RAM.

You can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_din_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_smem_din_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi_smem_din_num`] module*/
pub type SPI_SMEM_DIN_NUM = crate::Reg<spi_smem_din_num::SPI_SMEM_DIN_NUM_SPEC>;
///MSPI input timing delay number control register when accesses to Ext_RAM.
pub mod spi_smem_din_num;
/**SPI_SMEM_DOUT_MODE (rw) register accessor: MSPI output timing delay mode control register when accesses to Ext_RAM.

You can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_dout_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_smem_dout_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi_smem_dout_mode`] module*/
pub type SPI_SMEM_DOUT_MODE = crate::Reg<spi_smem_dout_mode::SPI_SMEM_DOUT_MODE_SPEC>;
///MSPI output timing delay mode control register when accesses to Ext_RAM.
pub mod spi_smem_dout_mode;
/**ECC_CTRL (rw) register accessor: MSPI ECC control register

You can [`read`](crate::generic::Reg::read) this register and get [`ecc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ecc_ctrl`] module*/
pub type ECC_CTRL = crate::Reg<ecc_ctrl::ECC_CTRL_SPEC>;
///MSPI ECC control register
pub mod ecc_ctrl;
/**ECC_ERR_ADDR (r) register accessor: MSPI ECC error address register

You can [`read`](crate::generic::Reg::read) this register and get [`ecc_err_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ecc_err_addr`] module*/
pub type ECC_ERR_ADDR = crate::Reg<ecc_err_addr::ECC_ERR_ADDR_SPEC>;
///MSPI ECC error address register
pub mod ecc_err_addr;
/**ECC_ERR_BIT (r) register accessor: MSPI ECC error bits register

You can [`read`](crate::generic::Reg::read) this register and get [`ecc_err_bit::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ecc_err_bit`] module*/
pub type ECC_ERR_BIT = crate::Reg<ecc_err_bit::ECC_ERR_BIT_SPEC>;
///MSPI ECC error bits register
pub mod ecc_err_bit;
/**SPI_SMEM_AC (rw) register accessor: MSPI external RAM ECC and SPI CS timing control register

You can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_ac::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_smem_ac::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi_smem_ac`] module*/
pub type SPI_SMEM_AC = crate::Reg<spi_smem_ac::SPI_SMEM_AC_SPEC>;
///MSPI external RAM ECC and SPI CS timing control register
pub mod spi_smem_ac;
/**DDR (rw) register accessor: SPI0 flash DDR mode control register

You can [`read`](crate::generic::Reg::read) this register and get [`ddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ddr`] module*/
pub type DDR = crate::Reg<ddr::DDR_SPEC>;
///SPI0 flash DDR mode control register
pub mod ddr;
/**SPI_SMEM_DDR (rw) register accessor: SPI0 external RAM DDR mode control register

You can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_ddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_smem_ddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi_smem_ddr`] module*/
pub type SPI_SMEM_DDR = crate::Reg<spi_smem_ddr::SPI_SMEM_DDR_SPEC>;
///SPI0 external RAM DDR mode control register
pub mod spi_smem_ddr;
/**CLOCK_GATE (rw) register accessor: SPI0 clk_gate register

You can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clock_gate`] module*/
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
///SPI0 clk_gate register
pub mod clock_gate;
/**CORE_CLK_SEL (rw) register accessor: SPI0 module clock select register

You can [`read`](crate::generic::Reg::read) this register and get [`core_clk_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_clk_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@core_clk_sel`] module*/
pub type CORE_CLK_SEL = crate::Reg<core_clk_sel::CORE_CLK_SEL_SPEC>;
///SPI0 module clock select register
pub mod core_clk_sel;
/**INT_ENA (rw) register accessor: SPI1 interrupt enable register

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///SPI1 interrupt enable register
pub mod int_ena;
/**INT_CLR (w) register accessor: SPI1 interrupt clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///SPI1 interrupt clear register
pub mod int_clr;
/**INT_RAW (rw) register accessor: SPI1 interrupt raw register

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///SPI1 interrupt raw register
pub mod int_raw;
/**INT_ST (r) register accessor: SPI1 interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///SPI1 interrupt status register
pub mod int_st;
/**DATE (rw) register accessor: SPI0 version control register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///SPI0 version control register
pub mod date;
