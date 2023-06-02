#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI1 memory command register"]
    pub spi_mem_cmd: SPI_MEM_CMD,
    #[doc = "0x04 - SPI1 address register"]
    pub spi_mem_addr: SPI_MEM_ADDR,
    #[doc = "0x08 - SPI1 control register."]
    pub spi_mem_ctrl: SPI_MEM_CTRL,
    #[doc = "0x0c - SPI1 control1 register."]
    pub spi_mem_ctrl1: SPI_MEM_CTRL1,
    #[doc = "0x10 - SPI1 control2 register."]
    pub spi_mem_ctrl2: SPI_MEM_CTRL2,
    #[doc = "0x14 - SPI1 clock division control register."]
    pub spi_mem_clock: SPI_MEM_CLOCK,
    #[doc = "0x18 - SPI1 user register."]
    pub spi_mem_user: SPI_MEM_USER,
    #[doc = "0x1c - SPI1 user1 register."]
    pub spi_mem_user1: SPI_MEM_USER1,
    #[doc = "0x20 - SPI1 user2 register."]
    pub spi_mem_user2: SPI_MEM_USER2,
    #[doc = "0x24 - SPI1 send data bit length control register."]
    pub spi_mem_mosi_dlen: SPI_MEM_MOSI_DLEN,
    #[doc = "0x28 - SPI1 receive data bit length control register."]
    pub spi_mem_miso_dlen: SPI_MEM_MISO_DLEN,
    #[doc = "0x2c - SPI1 status register."]
    pub spi_mem_rd_status: SPI_MEM_RD_STATUS,
    _reserved12: [u8; 0x04],
    #[doc = "0x34 - SPI1 misc register"]
    pub spi_mem_misc: SPI_MEM_MISC,
    #[doc = "0x38 - SPI1 TX CRC data register."]
    pub spi_mem_tx_crc: SPI_MEM_TX_CRC,
    #[doc = "0x3c - SPI1 bit mode control register."]
    pub spi_mem_cache_fctrl: SPI_MEM_CACHE_FCTRL,
    _reserved15: [u8; 0x18],
    #[doc = "0x58 - SPI1 memory data buffer0"]
    pub spi_mem_w0: SPI_MEM_W0,
    #[doc = "0x5c - SPI1 memory data buffer1"]
    pub spi_mem_w1: SPI_MEM_W1,
    #[doc = "0x60 - SPI1 memory data buffer2"]
    pub spi_mem_w2: SPI_MEM_W2,
    #[doc = "0x64 - SPI1 memory data buffer3"]
    pub spi_mem_w3: SPI_MEM_W3,
    #[doc = "0x68 - SPI1 memory data buffer4"]
    pub spi_mem_w4: SPI_MEM_W4,
    #[doc = "0x6c - SPI1 memory data buffer5"]
    pub spi_mem_w5: SPI_MEM_W5,
    #[doc = "0x70 - SPI1 memory data buffer6"]
    pub spi_mem_w6: SPI_MEM_W6,
    #[doc = "0x74 - SPI1 memory data buffer7"]
    pub spi_mem_w7: SPI_MEM_W7,
    #[doc = "0x78 - SPI1 memory data buffer8"]
    pub spi_mem_w8: SPI_MEM_W8,
    #[doc = "0x7c - SPI1 memory data buffer9"]
    pub spi_mem_w9: SPI_MEM_W9,
    #[doc = "0x80 - SPI1 memory data buffer10"]
    pub spi_mem_w10: SPI_MEM_W10,
    #[doc = "0x84 - SPI1 memory data buffer11"]
    pub spi_mem_w11: SPI_MEM_W11,
    #[doc = "0x88 - SPI1 memory data buffer12"]
    pub spi_mem_w12: SPI_MEM_W12,
    #[doc = "0x8c - SPI1 memory data buffer13"]
    pub spi_mem_w13: SPI_MEM_W13,
    #[doc = "0x90 - SPI1 memory data buffer14"]
    pub spi_mem_w14: SPI_MEM_W14,
    #[doc = "0x94 - SPI1 memory data buffer15"]
    pub spi_mem_w15: SPI_MEM_W15,
    #[doc = "0x98 - SPI1 wait idle control register"]
    pub spi_mem_flash_waiti_ctrl: SPI_MEM_FLASH_WAITI_CTRL,
    #[doc = "0x9c - SPI1 flash suspend control register"]
    pub spi_mem_flash_sus_ctrl: SPI_MEM_FLASH_SUS_CTRL,
    #[doc = "0xa0 - SPI1 flash suspend command register"]
    pub spi_mem_flash_sus_cmd: SPI_MEM_FLASH_SUS_CMD,
    #[doc = "0xa4 - SPI1 flash suspend status register"]
    pub spi_mem_sus_status: SPI_MEM_SUS_STATUS,
    _reserved35: [u8; 0x18],
    #[doc = "0xc0 - SPI1 interrupt enable register"]
    pub spi_mem_int_ena: SPI_MEM_INT_ENA,
    #[doc = "0xc4 - SPI1 interrupt clear register"]
    pub spi_mem_int_clr: SPI_MEM_INT_CLR,
    #[doc = "0xc8 - SPI1 interrupt raw register"]
    pub spi_mem_int_raw: SPI_MEM_INT_RAW,
    #[doc = "0xcc - SPI1 interrupt status register"]
    pub spi_mem_int_st: SPI_MEM_INT_ST,
    _reserved39: [u8; 0x04],
    #[doc = "0xd4 - SPI1 DDR control register"]
    pub spi_mem_ddr: SPI_MEM_DDR,
    _reserved40: [u8; 0xa8],
    #[doc = "0x180 - SPI1 timing control register"]
    pub spi_mem_timing_cali: SPI_MEM_TIMING_CALI,
    _reserved41: [u8; 0x7c],
    #[doc = "0x200 - SPI1 clk_gate register"]
    pub spi_mem_clock_gate: SPI_MEM_CLOCK_GATE,
    _reserved42: [u8; 0x01f8],
    #[doc = "0x3fc - Version control register"]
    pub spi_mem_date: SPI_MEM_DATE,
}
#[doc = "SPI_MEM_CMD (rw) register accessor: an alias for `Reg<SPI_MEM_CMD_SPEC>`"]
pub type SPI_MEM_CMD = crate::Reg<spi_mem_cmd::SPI_MEM_CMD_SPEC>;
#[doc = "SPI1 memory command register"]
pub mod spi_mem_cmd;
#[doc = "SPI_MEM_ADDR (rw) register accessor: an alias for `Reg<SPI_MEM_ADDR_SPEC>`"]
pub type SPI_MEM_ADDR = crate::Reg<spi_mem_addr::SPI_MEM_ADDR_SPEC>;
#[doc = "SPI1 address register"]
pub mod spi_mem_addr;
#[doc = "SPI_MEM_CTRL (rw) register accessor: an alias for `Reg<SPI_MEM_CTRL_SPEC>`"]
pub type SPI_MEM_CTRL = crate::Reg<spi_mem_ctrl::SPI_MEM_CTRL_SPEC>;
#[doc = "SPI1 control register."]
pub mod spi_mem_ctrl;
#[doc = "SPI_MEM_CTRL1 (rw) register accessor: an alias for `Reg<SPI_MEM_CTRL1_SPEC>`"]
pub type SPI_MEM_CTRL1 = crate::Reg<spi_mem_ctrl1::SPI_MEM_CTRL1_SPEC>;
#[doc = "SPI1 control1 register."]
pub mod spi_mem_ctrl1;
#[doc = "SPI_MEM_CTRL2 (w) register accessor: an alias for `Reg<SPI_MEM_CTRL2_SPEC>`"]
pub type SPI_MEM_CTRL2 = crate::Reg<spi_mem_ctrl2::SPI_MEM_CTRL2_SPEC>;
#[doc = "SPI1 control2 register."]
pub mod spi_mem_ctrl2;
#[doc = "SPI_MEM_CLOCK (rw) register accessor: an alias for `Reg<SPI_MEM_CLOCK_SPEC>`"]
pub type SPI_MEM_CLOCK = crate::Reg<spi_mem_clock::SPI_MEM_CLOCK_SPEC>;
#[doc = "SPI1 clock division control register."]
pub mod spi_mem_clock;
#[doc = "SPI_MEM_USER (rw) register accessor: an alias for `Reg<SPI_MEM_USER_SPEC>`"]
pub type SPI_MEM_USER = crate::Reg<spi_mem_user::SPI_MEM_USER_SPEC>;
#[doc = "SPI1 user register."]
pub mod spi_mem_user;
#[doc = "SPI_MEM_USER1 (rw) register accessor: an alias for `Reg<SPI_MEM_USER1_SPEC>`"]
pub type SPI_MEM_USER1 = crate::Reg<spi_mem_user1::SPI_MEM_USER1_SPEC>;
#[doc = "SPI1 user1 register."]
pub mod spi_mem_user1;
#[doc = "SPI_MEM_USER2 (rw) register accessor: an alias for `Reg<SPI_MEM_USER2_SPEC>`"]
pub type SPI_MEM_USER2 = crate::Reg<spi_mem_user2::SPI_MEM_USER2_SPEC>;
#[doc = "SPI1 user2 register."]
pub mod spi_mem_user2;
#[doc = "SPI_MEM_MOSI_DLEN (rw) register accessor: an alias for `Reg<SPI_MEM_MOSI_DLEN_SPEC>`"]
pub type SPI_MEM_MOSI_DLEN = crate::Reg<spi_mem_mosi_dlen::SPI_MEM_MOSI_DLEN_SPEC>;
#[doc = "SPI1 send data bit length control register."]
pub mod spi_mem_mosi_dlen;
#[doc = "SPI_MEM_MISO_DLEN (rw) register accessor: an alias for `Reg<SPI_MEM_MISO_DLEN_SPEC>`"]
pub type SPI_MEM_MISO_DLEN = crate::Reg<spi_mem_miso_dlen::SPI_MEM_MISO_DLEN_SPEC>;
#[doc = "SPI1 receive data bit length control register."]
pub mod spi_mem_miso_dlen;
#[doc = "SPI_MEM_RD_STATUS (rw) register accessor: an alias for `Reg<SPI_MEM_RD_STATUS_SPEC>`"]
pub type SPI_MEM_RD_STATUS = crate::Reg<spi_mem_rd_status::SPI_MEM_RD_STATUS_SPEC>;
#[doc = "SPI1 status register."]
pub mod spi_mem_rd_status;
#[doc = "SPI_MEM_MISC (rw) register accessor: an alias for `Reg<SPI_MEM_MISC_SPEC>`"]
pub type SPI_MEM_MISC = crate::Reg<spi_mem_misc::SPI_MEM_MISC_SPEC>;
#[doc = "SPI1 misc register"]
pub mod spi_mem_misc;
#[doc = "SPI_MEM_TX_CRC (r) register accessor: an alias for `Reg<SPI_MEM_TX_CRC_SPEC>`"]
pub type SPI_MEM_TX_CRC = crate::Reg<spi_mem_tx_crc::SPI_MEM_TX_CRC_SPEC>;
#[doc = "SPI1 TX CRC data register."]
pub mod spi_mem_tx_crc;
#[doc = "SPI_MEM_CACHE_FCTRL (rw) register accessor: an alias for `Reg<SPI_MEM_CACHE_FCTRL_SPEC>`"]
pub type SPI_MEM_CACHE_FCTRL = crate::Reg<spi_mem_cache_fctrl::SPI_MEM_CACHE_FCTRL_SPEC>;
#[doc = "SPI1 bit mode control register."]
pub mod spi_mem_cache_fctrl;
#[doc = "SPI_MEM_W0 (rw) register accessor: an alias for `Reg<SPI_MEM_W0_SPEC>`"]
pub type SPI_MEM_W0 = crate::Reg<spi_mem_w0::SPI_MEM_W0_SPEC>;
#[doc = "SPI1 memory data buffer0"]
pub mod spi_mem_w0;
#[doc = "SPI_MEM_W1 (rw) register accessor: an alias for `Reg<SPI_MEM_W1_SPEC>`"]
pub type SPI_MEM_W1 = crate::Reg<spi_mem_w1::SPI_MEM_W1_SPEC>;
#[doc = "SPI1 memory data buffer1"]
pub mod spi_mem_w1;
#[doc = "SPI_MEM_W2 (rw) register accessor: an alias for `Reg<SPI_MEM_W2_SPEC>`"]
pub type SPI_MEM_W2 = crate::Reg<spi_mem_w2::SPI_MEM_W2_SPEC>;
#[doc = "SPI1 memory data buffer2"]
pub mod spi_mem_w2;
#[doc = "SPI_MEM_W3 (rw) register accessor: an alias for `Reg<SPI_MEM_W3_SPEC>`"]
pub type SPI_MEM_W3 = crate::Reg<spi_mem_w3::SPI_MEM_W3_SPEC>;
#[doc = "SPI1 memory data buffer3"]
pub mod spi_mem_w3;
#[doc = "SPI_MEM_W4 (rw) register accessor: an alias for `Reg<SPI_MEM_W4_SPEC>`"]
pub type SPI_MEM_W4 = crate::Reg<spi_mem_w4::SPI_MEM_W4_SPEC>;
#[doc = "SPI1 memory data buffer4"]
pub mod spi_mem_w4;
#[doc = "SPI_MEM_W5 (rw) register accessor: an alias for `Reg<SPI_MEM_W5_SPEC>`"]
pub type SPI_MEM_W5 = crate::Reg<spi_mem_w5::SPI_MEM_W5_SPEC>;
#[doc = "SPI1 memory data buffer5"]
pub mod spi_mem_w5;
#[doc = "SPI_MEM_W6 (rw) register accessor: an alias for `Reg<SPI_MEM_W6_SPEC>`"]
pub type SPI_MEM_W6 = crate::Reg<spi_mem_w6::SPI_MEM_W6_SPEC>;
#[doc = "SPI1 memory data buffer6"]
pub mod spi_mem_w6;
#[doc = "SPI_MEM_W7 (rw) register accessor: an alias for `Reg<SPI_MEM_W7_SPEC>`"]
pub type SPI_MEM_W7 = crate::Reg<spi_mem_w7::SPI_MEM_W7_SPEC>;
#[doc = "SPI1 memory data buffer7"]
pub mod spi_mem_w7;
#[doc = "SPI_MEM_W8 (rw) register accessor: an alias for `Reg<SPI_MEM_W8_SPEC>`"]
pub type SPI_MEM_W8 = crate::Reg<spi_mem_w8::SPI_MEM_W8_SPEC>;
#[doc = "SPI1 memory data buffer8"]
pub mod spi_mem_w8;
#[doc = "SPI_MEM_W9 (rw) register accessor: an alias for `Reg<SPI_MEM_W9_SPEC>`"]
pub type SPI_MEM_W9 = crate::Reg<spi_mem_w9::SPI_MEM_W9_SPEC>;
#[doc = "SPI1 memory data buffer9"]
pub mod spi_mem_w9;
#[doc = "SPI_MEM_W10 (rw) register accessor: an alias for `Reg<SPI_MEM_W10_SPEC>`"]
pub type SPI_MEM_W10 = crate::Reg<spi_mem_w10::SPI_MEM_W10_SPEC>;
#[doc = "SPI1 memory data buffer10"]
pub mod spi_mem_w10;
#[doc = "SPI_MEM_W11 (rw) register accessor: an alias for `Reg<SPI_MEM_W11_SPEC>`"]
pub type SPI_MEM_W11 = crate::Reg<spi_mem_w11::SPI_MEM_W11_SPEC>;
#[doc = "SPI1 memory data buffer11"]
pub mod spi_mem_w11;
#[doc = "SPI_MEM_W12 (rw) register accessor: an alias for `Reg<SPI_MEM_W12_SPEC>`"]
pub type SPI_MEM_W12 = crate::Reg<spi_mem_w12::SPI_MEM_W12_SPEC>;
#[doc = "SPI1 memory data buffer12"]
pub mod spi_mem_w12;
#[doc = "SPI_MEM_W13 (rw) register accessor: an alias for `Reg<SPI_MEM_W13_SPEC>`"]
pub type SPI_MEM_W13 = crate::Reg<spi_mem_w13::SPI_MEM_W13_SPEC>;
#[doc = "SPI1 memory data buffer13"]
pub mod spi_mem_w13;
#[doc = "SPI_MEM_W14 (rw) register accessor: an alias for `Reg<SPI_MEM_W14_SPEC>`"]
pub type SPI_MEM_W14 = crate::Reg<spi_mem_w14::SPI_MEM_W14_SPEC>;
#[doc = "SPI1 memory data buffer14"]
pub mod spi_mem_w14;
#[doc = "SPI_MEM_W15 (rw) register accessor: an alias for `Reg<SPI_MEM_W15_SPEC>`"]
pub type SPI_MEM_W15 = crate::Reg<spi_mem_w15::SPI_MEM_W15_SPEC>;
#[doc = "SPI1 memory data buffer15"]
pub mod spi_mem_w15;
#[doc = "SPI_MEM_FLASH_WAITI_CTRL (rw) register accessor: an alias for `Reg<SPI_MEM_FLASH_WAITI_CTRL_SPEC>`"]
pub type SPI_MEM_FLASH_WAITI_CTRL =
    crate::Reg<spi_mem_flash_waiti_ctrl::SPI_MEM_FLASH_WAITI_CTRL_SPEC>;
#[doc = "SPI1 wait idle control register"]
pub mod spi_mem_flash_waiti_ctrl;
#[doc = "SPI_MEM_FLASH_SUS_CTRL (rw) register accessor: an alias for `Reg<SPI_MEM_FLASH_SUS_CTRL_SPEC>`"]
pub type SPI_MEM_FLASH_SUS_CTRL = crate::Reg<spi_mem_flash_sus_ctrl::SPI_MEM_FLASH_SUS_CTRL_SPEC>;
#[doc = "SPI1 flash suspend control register"]
pub mod spi_mem_flash_sus_ctrl;
#[doc = "SPI_MEM_FLASH_SUS_CMD (rw) register accessor: an alias for `Reg<SPI_MEM_FLASH_SUS_CMD_SPEC>`"]
pub type SPI_MEM_FLASH_SUS_CMD = crate::Reg<spi_mem_flash_sus_cmd::SPI_MEM_FLASH_SUS_CMD_SPEC>;
#[doc = "SPI1 flash suspend command register"]
pub mod spi_mem_flash_sus_cmd;
#[doc = "SPI_MEM_SUS_STATUS (rw) register accessor: an alias for `Reg<SPI_MEM_SUS_STATUS_SPEC>`"]
pub type SPI_MEM_SUS_STATUS = crate::Reg<spi_mem_sus_status::SPI_MEM_SUS_STATUS_SPEC>;
#[doc = "SPI1 flash suspend status register"]
pub mod spi_mem_sus_status;
#[doc = "SPI_MEM_INT_ENA (rw) register accessor: an alias for `Reg<SPI_MEM_INT_ENA_SPEC>`"]
pub type SPI_MEM_INT_ENA = crate::Reg<spi_mem_int_ena::SPI_MEM_INT_ENA_SPEC>;
#[doc = "SPI1 interrupt enable register"]
pub mod spi_mem_int_ena;
#[doc = "SPI_MEM_INT_CLR (w) register accessor: an alias for `Reg<SPI_MEM_INT_CLR_SPEC>`"]
pub type SPI_MEM_INT_CLR = crate::Reg<spi_mem_int_clr::SPI_MEM_INT_CLR_SPEC>;
#[doc = "SPI1 interrupt clear register"]
pub mod spi_mem_int_clr;
#[doc = "SPI_MEM_INT_RAW (rw) register accessor: an alias for `Reg<SPI_MEM_INT_RAW_SPEC>`"]
pub type SPI_MEM_INT_RAW = crate::Reg<spi_mem_int_raw::SPI_MEM_INT_RAW_SPEC>;
#[doc = "SPI1 interrupt raw register"]
pub mod spi_mem_int_raw;
#[doc = "SPI_MEM_INT_ST (r) register accessor: an alias for `Reg<SPI_MEM_INT_ST_SPEC>`"]
pub type SPI_MEM_INT_ST = crate::Reg<spi_mem_int_st::SPI_MEM_INT_ST_SPEC>;
#[doc = "SPI1 interrupt status register"]
pub mod spi_mem_int_st;
#[doc = "SPI_MEM_DDR (r) register accessor: an alias for `Reg<SPI_MEM_DDR_SPEC>`"]
pub type SPI_MEM_DDR = crate::Reg<spi_mem_ddr::SPI_MEM_DDR_SPEC>;
#[doc = "SPI1 DDR control register"]
pub mod spi_mem_ddr;
#[doc = "SPI_MEM_TIMING_CALI (rw) register accessor: an alias for `Reg<SPI_MEM_TIMING_CALI_SPEC>`"]
pub type SPI_MEM_TIMING_CALI = crate::Reg<spi_mem_timing_cali::SPI_MEM_TIMING_CALI_SPEC>;
#[doc = "SPI1 timing control register"]
pub mod spi_mem_timing_cali;
#[doc = "SPI_MEM_CLOCK_GATE (rw) register accessor: an alias for `Reg<SPI_MEM_CLOCK_GATE_SPEC>`"]
pub type SPI_MEM_CLOCK_GATE = crate::Reg<spi_mem_clock_gate::SPI_MEM_CLOCK_GATE_SPEC>;
#[doc = "SPI1 clk_gate register"]
pub mod spi_mem_clock_gate;
#[doc = "SPI_MEM_DATE (rw) register accessor: an alias for `Reg<SPI_MEM_DATE_SPEC>`"]
pub type SPI_MEM_DATE = crate::Reg<spi_mem_date::SPI_MEM_DATE_SPEC>;
#[doc = "Version control register"]
pub mod spi_mem_date;
