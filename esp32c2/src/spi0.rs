#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - SPI0 control register."]
    pub spi_mem_ctrl: crate::Reg<spi_mem_ctrl::SPI_MEM_CTRL_SPEC>,
    #[doc = "0x0c - SPI0 control1 register."]
    pub spi_mem_ctrl1: crate::Reg<spi_mem_ctrl1::SPI_MEM_CTRL1_SPEC>,
    #[doc = "0x10 - SPI0 control2 register."]
    pub spi_mem_ctrl2: crate::Reg<spi_mem_ctrl2::SPI_MEM_CTRL2_SPEC>,
    #[doc = "0x14 - SPI clock division control register."]
    pub spi_mem_clock: crate::Reg<spi_mem_clock::SPI_MEM_CLOCK_SPEC>,
    #[doc = "0x18 - SPI0 user register."]
    pub spi_mem_user: crate::Reg<spi_mem_user::SPI_MEM_USER_SPEC>,
    #[doc = "0x1c - SPI0 user1 register."]
    pub spi_mem_user1: crate::Reg<spi_mem_user1::SPI_MEM_USER1_SPEC>,
    #[doc = "0x20 - SPI0 user2 register."]
    pub spi_mem_user2: crate::Reg<spi_mem_user2::SPI_MEM_USER2_SPEC>,
    _reserved7: [u8; 0x08],
    #[doc = "0x2c - SPI0 read control register."]
    pub spi_mem_rd_status: crate::Reg<spi_mem_rd_status::SPI_MEM_RD_STATUS_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x34 - SPI0 misc register"]
    pub spi_mem_misc: crate::Reg<spi_mem_misc::SPI_MEM_MISC_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x3c - SPI0 bit mode control register."]
    pub spi_mem_cache_fctrl: crate::Reg<spi_mem_cache_fctrl::SPI_MEM_CACHE_FCTRL_SPEC>,
    _reserved10: [u8; 0x14],
    #[doc = "0x54 - SPI0 FSM status register"]
    pub spi_mem_fsm: crate::Reg<spi_mem_fsm::SPI_MEM_FSM_SPEC>,
    _reserved11: [u8; 0x50],
    #[doc = "0xa8 - SPI0 timing calibration register"]
    pub spi_mem_timing_cali: crate::Reg<spi_mem_timing_cali::SPI_MEM_TIMING_CALI_SPEC>,
    #[doc = "0xac - SPI0 input delay mode control register"]
    pub spi_mem_din_mode: crate::Reg<spi_mem_din_mode::SPI_MEM_DIN_MODE_SPEC>,
    #[doc = "0xb0 - SPI0 input delay number control register"]
    pub spi_mem_din_num: crate::Reg<spi_mem_din_num::SPI_MEM_DIN_NUM_SPEC>,
    #[doc = "0xb4 - SPI0 output delay mode control register"]
    pub spi_mem_dout_mode: crate::Reg<spi_mem_dout_mode::SPI_MEM_DOUT_MODE_SPEC>,
    _reserved15: [u8; 0x24],
    #[doc = "0xdc - SPI0 clk_gate register"]
    pub spi_mem_clock_gate: crate::Reg<spi_mem_clock_gate::SPI_MEM_CLOCK_GATE_SPEC>,
    #[doc = "0xe0 - SPI0 module clock select register"]
    pub spi_mem_core_clk_sel: crate::Reg<spi_mem_core_clk_sel::SPI_MEM_CORE_CLK_SEL_SPEC>,
    _reserved17: [u8; 0x0318],
    #[doc = "0x3fc - Version control register"]
    pub spi_mem_date: crate::Reg<spi_mem_date::SPI_MEM_DATE_SPEC>,
}
#[doc = "SPI_MEM_CTRL register accessor: an alias for `Reg<SPI_MEM_CTRL_SPEC>`"]
pub type SPI_MEM_CTRL = crate::Reg<spi_mem_ctrl::SPI_MEM_CTRL_SPEC>;
#[doc = "SPI0 control register."]
pub mod spi_mem_ctrl;
#[doc = "SPI_MEM_CTRL1 register accessor: an alias for `Reg<SPI_MEM_CTRL1_SPEC>`"]
pub type SPI_MEM_CTRL1 = crate::Reg<spi_mem_ctrl1::SPI_MEM_CTRL1_SPEC>;
#[doc = "SPI0 control1 register."]
pub mod spi_mem_ctrl1;
#[doc = "SPI_MEM_CTRL2 register accessor: an alias for `Reg<SPI_MEM_CTRL2_SPEC>`"]
pub type SPI_MEM_CTRL2 = crate::Reg<spi_mem_ctrl2::SPI_MEM_CTRL2_SPEC>;
#[doc = "SPI0 control2 register."]
pub mod spi_mem_ctrl2;
#[doc = "SPI_MEM_CLOCK register accessor: an alias for `Reg<SPI_MEM_CLOCK_SPEC>`"]
pub type SPI_MEM_CLOCK = crate::Reg<spi_mem_clock::SPI_MEM_CLOCK_SPEC>;
#[doc = "SPI clock division control register."]
pub mod spi_mem_clock;
#[doc = "SPI_MEM_USER register accessor: an alias for `Reg<SPI_MEM_USER_SPEC>`"]
pub type SPI_MEM_USER = crate::Reg<spi_mem_user::SPI_MEM_USER_SPEC>;
#[doc = "SPI0 user register."]
pub mod spi_mem_user;
#[doc = "SPI_MEM_USER1 register accessor: an alias for `Reg<SPI_MEM_USER1_SPEC>`"]
pub type SPI_MEM_USER1 = crate::Reg<spi_mem_user1::SPI_MEM_USER1_SPEC>;
#[doc = "SPI0 user1 register."]
pub mod spi_mem_user1;
#[doc = "SPI_MEM_USER2 register accessor: an alias for `Reg<SPI_MEM_USER2_SPEC>`"]
pub type SPI_MEM_USER2 = crate::Reg<spi_mem_user2::SPI_MEM_USER2_SPEC>;
#[doc = "SPI0 user2 register."]
pub mod spi_mem_user2;
#[doc = "SPI_MEM_RD_STATUS register accessor: an alias for `Reg<SPI_MEM_RD_STATUS_SPEC>`"]
pub type SPI_MEM_RD_STATUS = crate::Reg<spi_mem_rd_status::SPI_MEM_RD_STATUS_SPEC>;
#[doc = "SPI0 read control register."]
pub mod spi_mem_rd_status;
#[doc = "SPI_MEM_MISC register accessor: an alias for `Reg<SPI_MEM_MISC_SPEC>`"]
pub type SPI_MEM_MISC = crate::Reg<spi_mem_misc::SPI_MEM_MISC_SPEC>;
#[doc = "SPI0 misc register"]
pub mod spi_mem_misc;
#[doc = "SPI_MEM_CACHE_FCTRL register accessor: an alias for `Reg<SPI_MEM_CACHE_FCTRL_SPEC>`"]
pub type SPI_MEM_CACHE_FCTRL = crate::Reg<spi_mem_cache_fctrl::SPI_MEM_CACHE_FCTRL_SPEC>;
#[doc = "SPI0 bit mode control register."]
pub mod spi_mem_cache_fctrl;
#[doc = "SPI_MEM_FSM register accessor: an alias for `Reg<SPI_MEM_FSM_SPEC>`"]
pub type SPI_MEM_FSM = crate::Reg<spi_mem_fsm::SPI_MEM_FSM_SPEC>;
#[doc = "SPI0 FSM status register"]
pub mod spi_mem_fsm;
#[doc = "SPI_MEM_TIMING_CALI register accessor: an alias for `Reg<SPI_MEM_TIMING_CALI_SPEC>`"]
pub type SPI_MEM_TIMING_CALI = crate::Reg<spi_mem_timing_cali::SPI_MEM_TIMING_CALI_SPEC>;
#[doc = "SPI0 timing calibration register"]
pub mod spi_mem_timing_cali;
#[doc = "SPI_MEM_DIN_MODE register accessor: an alias for `Reg<SPI_MEM_DIN_MODE_SPEC>`"]
pub type SPI_MEM_DIN_MODE = crate::Reg<spi_mem_din_mode::SPI_MEM_DIN_MODE_SPEC>;
#[doc = "SPI0 input delay mode control register"]
pub mod spi_mem_din_mode;
#[doc = "SPI_MEM_DIN_NUM register accessor: an alias for `Reg<SPI_MEM_DIN_NUM_SPEC>`"]
pub type SPI_MEM_DIN_NUM = crate::Reg<spi_mem_din_num::SPI_MEM_DIN_NUM_SPEC>;
#[doc = "SPI0 input delay number control register"]
pub mod spi_mem_din_num;
#[doc = "SPI_MEM_DOUT_MODE register accessor: an alias for `Reg<SPI_MEM_DOUT_MODE_SPEC>`"]
pub type SPI_MEM_DOUT_MODE = crate::Reg<spi_mem_dout_mode::SPI_MEM_DOUT_MODE_SPEC>;
#[doc = "SPI0 output delay mode control register"]
pub mod spi_mem_dout_mode;
#[doc = "SPI_MEM_CLOCK_GATE register accessor: an alias for `Reg<SPI_MEM_CLOCK_GATE_SPEC>`"]
pub type SPI_MEM_CLOCK_GATE = crate::Reg<spi_mem_clock_gate::SPI_MEM_CLOCK_GATE_SPEC>;
#[doc = "SPI0 clk_gate register"]
pub mod spi_mem_clock_gate;
#[doc = "SPI_MEM_CORE_CLK_SEL register accessor: an alias for `Reg<SPI_MEM_CORE_CLK_SEL_SPEC>`"]
pub type SPI_MEM_CORE_CLK_SEL = crate::Reg<spi_mem_core_clk_sel::SPI_MEM_CORE_CLK_SEL_SPEC>;
#[doc = "SPI0 module clock select register"]
pub mod spi_mem_core_clk_sel;
#[doc = "SPI_MEM_DATE register accessor: an alias for `Reg<SPI_MEM_DATE_SPEC>`"]
pub type SPI_MEM_DATE = crate::Reg<spi_mem_date::SPI_MEM_DATE_SPEC>;
#[doc = "Version control register"]
pub mod spi_mem_date;
