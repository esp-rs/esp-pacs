#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - SPI0 control register."]
    pub ctrl: CTRL,
    #[doc = "0x0c - SPI0 control 1 register."]
    pub ctrl1: CTRL1,
    #[doc = "0x10 - SPI0 control 2 register."]
    pub ctrl2: CTRL2,
    #[doc = "0x14 - SPI_CLK clock division register when SPI0 accesses to flash."]
    pub clock: CLOCK,
    #[doc = "0x18 - SPI0 user register."]
    pub user: USER,
    #[doc = "0x1c - SPI0 user1 register."]
    pub user1: USER1,
    #[doc = "0x20 - SPI0 user2 register."]
    pub user2: USER2,
    _reserved7: [u8; 0x08],
    #[doc = "0x2c - SPI0 read control register."]
    pub rd_status: RD_STATUS,
    #[doc = "0x30 - SPI0 extended address register."]
    pub ext_addr: EXT_ADDR,
    #[doc = "0x34 - SPI0 misc register"]
    pub misc: MISC,
    _reserved10: [u8; 0x04],
    #[doc = "0x3c - SPI0 external RAM bit mode control register."]
    pub cache_fctrl: CACHE_FCTRL,
    #[doc = "0x40 - SPI0 external RAM control register"]
    pub cache_sctrl: CACHE_SCTRL,
    #[doc = "0x44 - SPI0 external RAM mode control register"]
    pub sram_cmd: SRAM_CMD,
    #[doc = "0x48 - SPI0 external RAM DDR read command control register"]
    pub sram_drd_cmd: SRAM_DRD_CMD,
    #[doc = "0x4c - SPI0 external RAM DDR write command control register"]
    pub sram_dwr_cmd: SRAM_DWR_CMD,
    #[doc = "0x50 - SPI_CLK clock division register when SPI0 accesses to Ext_RAM."]
    pub sram_clk: SRAM_CLK,
    #[doc = "0x54 - SPI0 state machine(FSM) status register."]
    pub fsm: FSM,
    _reserved17: [u8; 0x50],
    #[doc = "0xa8 - SPI0 timing compensation register when accesses to flash."]
    pub timing_cali: TIMING_CALI,
    #[doc = "0xac - MSPI input timing delay mode control register when accesses to flash."]
    pub din_mode: DIN_MODE,
    #[doc = "0xb0 - MSPI input timing delay number control register when accesses to flash."]
    pub din_num: DIN_NUM,
    #[doc = "0xb4 - MSPI output timing delay mode control register when accesses to flash."]
    pub dout_mode: DOUT_MODE,
    _reserved21: [u8; 0x04],
    #[doc = "0xbc - SPI0 Ext_RAM timing compensation register."]
    pub spi_smem_timing_cali: SPI_SMEM_TIMING_CALI,
    #[doc = "0xc0 - MSPI input timing delay mode control register when accesses to Ext_RAM."]
    pub spi_smem_din_mode: SPI_SMEM_DIN_MODE,
    #[doc = "0xc4 - MSPI input timing delay number control register when accesses to Ext_RAM."]
    pub spi_smem_din_num: SPI_SMEM_DIN_NUM,
    #[doc = "0xc8 - MSPI output timing delay mode control register when accesses to Ext_RAM."]
    pub spi_smem_dout_mode: SPI_SMEM_DOUT_MODE,
    #[doc = "0xcc - MSPI ECC control register"]
    pub ecc_ctrl: ECC_CTRL,
    #[doc = "0xd0 - MSPI ECC error address register"]
    pub ecc_err_addr: ECC_ERR_ADDR,
    #[doc = "0xd4 - MSPI ECC error bits register"]
    pub ecc_err_bit: ECC_ERR_BIT,
    _reserved28: [u8; 0x04],
    #[doc = "0xdc - MSPI external RAM ECC and SPI CS timing control register"]
    pub spi_smem_ac: SPI_SMEM_AC,
    #[doc = "0xe0 - SPI0 flash DDR mode control register"]
    pub ddr: DDR,
    #[doc = "0xe4 - SPI0 external RAM DDR mode control register"]
    pub spi_smem_ddr: SPI_SMEM_DDR,
    #[doc = "0xe8 - SPI0 clk_gate register"]
    pub clock_gate: CLOCK_GATE,
    #[doc = "0xec - SPI0 module clock select register"]
    pub core_clk_sel: CORE_CLK_SEL,
    #[doc = "0xf0 - SPI1 interrupt enable register"]
    pub int_ena: INT_ENA,
    #[doc = "0xf4 - SPI1 interrupt clear register"]
    pub int_clr: INT_CLR,
    #[doc = "0xf8 - SPI1 interrupt raw register"]
    pub int_raw: INT_RAW,
    #[doc = "0xfc - SPI1 interrupt status register"]
    pub int_st: INT_ST,
    _reserved37: [u8; 0x02fc],
    #[doc = "0x3fc - SPI0 version control register"]
    pub date: DATE,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "SPI0 control register."]
pub mod ctrl;
#[doc = "CTRL1 (rw) register accessor: an alias for `Reg<CTRL1_SPEC>`"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "SPI0 control 1 register."]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: an alias for `Reg<CTRL2_SPEC>`"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "SPI0 control 2 register."]
pub mod ctrl2;
#[doc = "CLOCK (rw) register accessor: an alias for `Reg<CLOCK_SPEC>`"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = "SPI_CLK clock division register when SPI0 accesses to flash."]
pub mod clock;
#[doc = "USER (rw) register accessor: an alias for `Reg<USER_SPEC>`"]
pub type USER = crate::Reg<user::USER_SPEC>;
#[doc = "SPI0 user register."]
pub mod user;
#[doc = "USER1 (rw) register accessor: an alias for `Reg<USER1_SPEC>`"]
pub type USER1 = crate::Reg<user1::USER1_SPEC>;
#[doc = "SPI0 user1 register."]
pub mod user1;
#[doc = "USER2 (rw) register accessor: an alias for `Reg<USER2_SPEC>`"]
pub type USER2 = crate::Reg<user2::USER2_SPEC>;
#[doc = "SPI0 user2 register."]
pub mod user2;
#[doc = "RD_STATUS (rw) register accessor: an alias for `Reg<RD_STATUS_SPEC>`"]
pub type RD_STATUS = crate::Reg<rd_status::RD_STATUS_SPEC>;
#[doc = "SPI0 read control register."]
pub mod rd_status;
#[doc = "EXT_ADDR (rw) register accessor: an alias for `Reg<EXT_ADDR_SPEC>`"]
pub type EXT_ADDR = crate::Reg<ext_addr::EXT_ADDR_SPEC>;
#[doc = "SPI0 extended address register."]
pub mod ext_addr;
#[doc = "MISC (rw) register accessor: an alias for `Reg<MISC_SPEC>`"]
pub type MISC = crate::Reg<misc::MISC_SPEC>;
#[doc = "SPI0 misc register"]
pub mod misc;
#[doc = "CACHE_FCTRL (rw) register accessor: an alias for `Reg<CACHE_FCTRL_SPEC>`"]
pub type CACHE_FCTRL = crate::Reg<cache_fctrl::CACHE_FCTRL_SPEC>;
#[doc = "SPI0 external RAM bit mode control register."]
pub mod cache_fctrl;
#[doc = "CACHE_SCTRL (rw) register accessor: an alias for `Reg<CACHE_SCTRL_SPEC>`"]
pub type CACHE_SCTRL = crate::Reg<cache_sctrl::CACHE_SCTRL_SPEC>;
#[doc = "SPI0 external RAM control register"]
pub mod cache_sctrl;
#[doc = "SRAM_CMD (rw) register accessor: an alias for `Reg<SRAM_CMD_SPEC>`"]
pub type SRAM_CMD = crate::Reg<sram_cmd::SRAM_CMD_SPEC>;
#[doc = "SPI0 external RAM mode control register"]
pub mod sram_cmd;
#[doc = "SRAM_DRD_CMD (rw) register accessor: an alias for `Reg<SRAM_DRD_CMD_SPEC>`"]
pub type SRAM_DRD_CMD = crate::Reg<sram_drd_cmd::SRAM_DRD_CMD_SPEC>;
#[doc = "SPI0 external RAM DDR read command control register"]
pub mod sram_drd_cmd;
#[doc = "SRAM_DWR_CMD (rw) register accessor: an alias for `Reg<SRAM_DWR_CMD_SPEC>`"]
pub type SRAM_DWR_CMD = crate::Reg<sram_dwr_cmd::SRAM_DWR_CMD_SPEC>;
#[doc = "SPI0 external RAM DDR write command control register"]
pub mod sram_dwr_cmd;
#[doc = "SRAM_CLK (rw) register accessor: an alias for `Reg<SRAM_CLK_SPEC>`"]
pub type SRAM_CLK = crate::Reg<sram_clk::SRAM_CLK_SPEC>;
#[doc = "SPI_CLK clock division register when SPI0 accesses to Ext_RAM."]
pub mod sram_clk;
#[doc = "FSM (r) register accessor: an alias for `Reg<FSM_SPEC>`"]
pub type FSM = crate::Reg<fsm::FSM_SPEC>;
#[doc = "SPI0 state machine(FSM) status register."]
pub mod fsm;
#[doc = "TIMING_CALI (rw) register accessor: an alias for `Reg<TIMING_CALI_SPEC>`"]
pub type TIMING_CALI = crate::Reg<timing_cali::TIMING_CALI_SPEC>;
#[doc = "SPI0 timing compensation register when accesses to flash."]
pub mod timing_cali;
#[doc = "DIN_MODE (rw) register accessor: an alias for `Reg<DIN_MODE_SPEC>`"]
pub type DIN_MODE = crate::Reg<din_mode::DIN_MODE_SPEC>;
#[doc = "MSPI input timing delay mode control register when accesses to flash."]
pub mod din_mode;
#[doc = "DIN_NUM (rw) register accessor: an alias for `Reg<DIN_NUM_SPEC>`"]
pub type DIN_NUM = crate::Reg<din_num::DIN_NUM_SPEC>;
#[doc = "MSPI input timing delay number control register when accesses to flash."]
pub mod din_num;
#[doc = "DOUT_MODE (rw) register accessor: an alias for `Reg<DOUT_MODE_SPEC>`"]
pub type DOUT_MODE = crate::Reg<dout_mode::DOUT_MODE_SPEC>;
#[doc = "MSPI output timing delay mode control register when accesses to flash."]
pub mod dout_mode;
#[doc = "SPI_SMEM_TIMING_CALI (rw) register accessor: an alias for `Reg<SPI_SMEM_TIMING_CALI_SPEC>`"]
pub type SPI_SMEM_TIMING_CALI = crate::Reg<spi_smem_timing_cali::SPI_SMEM_TIMING_CALI_SPEC>;
#[doc = "SPI0 Ext_RAM timing compensation register."]
pub mod spi_smem_timing_cali;
#[doc = "SPI_SMEM_DIN_MODE (rw) register accessor: an alias for `Reg<SPI_SMEM_DIN_MODE_SPEC>`"]
pub type SPI_SMEM_DIN_MODE = crate::Reg<spi_smem_din_mode::SPI_SMEM_DIN_MODE_SPEC>;
#[doc = "MSPI input timing delay mode control register when accesses to Ext_RAM."]
pub mod spi_smem_din_mode;
#[doc = "SPI_SMEM_DIN_NUM (rw) register accessor: an alias for `Reg<SPI_SMEM_DIN_NUM_SPEC>`"]
pub type SPI_SMEM_DIN_NUM = crate::Reg<spi_smem_din_num::SPI_SMEM_DIN_NUM_SPEC>;
#[doc = "MSPI input timing delay number control register when accesses to Ext_RAM."]
pub mod spi_smem_din_num;
#[doc = "SPI_SMEM_DOUT_MODE (rw) register accessor: an alias for `Reg<SPI_SMEM_DOUT_MODE_SPEC>`"]
pub type SPI_SMEM_DOUT_MODE = crate::Reg<spi_smem_dout_mode::SPI_SMEM_DOUT_MODE_SPEC>;
#[doc = "MSPI output timing delay mode control register when accesses to Ext_RAM."]
pub mod spi_smem_dout_mode;
#[doc = "ECC_CTRL (rw) register accessor: an alias for `Reg<ECC_CTRL_SPEC>`"]
pub type ECC_CTRL = crate::Reg<ecc_ctrl::ECC_CTRL_SPEC>;
#[doc = "MSPI ECC control register"]
pub mod ecc_ctrl;
#[doc = "ECC_ERR_ADDR (r) register accessor: an alias for `Reg<ECC_ERR_ADDR_SPEC>`"]
pub type ECC_ERR_ADDR = crate::Reg<ecc_err_addr::ECC_ERR_ADDR_SPEC>;
#[doc = "MSPI ECC error address register"]
pub mod ecc_err_addr;
#[doc = "ECC_ERR_BIT (r) register accessor: an alias for `Reg<ECC_ERR_BIT_SPEC>`"]
pub type ECC_ERR_BIT = crate::Reg<ecc_err_bit::ECC_ERR_BIT_SPEC>;
#[doc = "MSPI ECC error bits register"]
pub mod ecc_err_bit;
#[doc = "SPI_SMEM_AC (rw) register accessor: an alias for `Reg<SPI_SMEM_AC_SPEC>`"]
pub type SPI_SMEM_AC = crate::Reg<spi_smem_ac::SPI_SMEM_AC_SPEC>;
#[doc = "MSPI external RAM ECC and SPI CS timing control register"]
pub mod spi_smem_ac;
#[doc = "DDR (rw) register accessor: an alias for `Reg<DDR_SPEC>`"]
pub type DDR = crate::Reg<ddr::DDR_SPEC>;
#[doc = "SPI0 flash DDR mode control register"]
pub mod ddr;
#[doc = "SPI_SMEM_DDR (rw) register accessor: an alias for `Reg<SPI_SMEM_DDR_SPEC>`"]
pub type SPI_SMEM_DDR = crate::Reg<spi_smem_ddr::SPI_SMEM_DDR_SPEC>;
#[doc = "SPI0 external RAM DDR mode control register"]
pub mod spi_smem_ddr;
#[doc = "CLOCK_GATE (rw) register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "SPI0 clk_gate register"]
pub mod clock_gate;
#[doc = "CORE_CLK_SEL (rw) register accessor: an alias for `Reg<CORE_CLK_SEL_SPEC>`"]
pub type CORE_CLK_SEL = crate::Reg<core_clk_sel::CORE_CLK_SEL_SPEC>;
#[doc = "SPI0 module clock select register"]
pub mod core_clk_sel;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "SPI1 interrupt enable register"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "SPI1 interrupt clear register"]
pub mod int_clr;
#[doc = "INT_RAW (rw) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "SPI1 interrupt raw register"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "SPI1 interrupt status register"]
pub mod int_st;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "SPI0 version control register"]
pub mod date;
