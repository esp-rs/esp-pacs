#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI1 memory command register"]
    pub spi_mem_cmd: crate::Reg<spi_mem_cmd::SPI_MEM_CMD_SPEC>,
    #[doc = "0x04 - SPI1 address register"]
    pub spi_mem_addr: crate::Reg<spi_mem_addr::SPI_MEM_ADDR_SPEC>,
    #[doc = "0x08 - SPI1 control register."]
    pub spi_mem_ctrl: crate::Reg<spi_mem_ctrl::SPI_MEM_CTRL_SPEC>,
    #[doc = "0x0c - SPI1 control1 register."]
    pub spi_mem_ctrl1: crate::Reg<spi_mem_ctrl1::SPI_MEM_CTRL1_SPEC>,
    #[doc = "0x10 - SPI1 control2 register."]
    pub spi_mem_ctrl2: crate::Reg<spi_mem_ctrl2::SPI_MEM_CTRL2_SPEC>,
    #[doc = "0x14 - SPI1 clock division control register."]
    pub spi_mem_clock: crate::Reg<spi_mem_clock::SPI_MEM_CLOCK_SPEC>,
    #[doc = "0x18 - SPI1 user register."]
    pub spi_mem_user: crate::Reg<spi_mem_user::SPI_MEM_USER_SPEC>,
    #[doc = "0x1c - SPI1 user1 register."]
    pub spi_mem_user1: crate::Reg<spi_mem_user1::SPI_MEM_USER1_SPEC>,
    #[doc = "0x20 - SPI1 user2 register."]
    pub spi_mem_user2: crate::Reg<spi_mem_user2::SPI_MEM_USER2_SPEC>,
    #[doc = "0x24 - SPI1 send data bit length control register."]
    pub spi_mem_mosi_dlen: crate::Reg<spi_mem_mosi_dlen::SPI_MEM_MOSI_DLEN_SPEC>,
    #[doc = "0x28 - SPI1 receive data bit length control register."]
    pub spi_mem_miso_dlen: crate::Reg<spi_mem_miso_dlen::SPI_MEM_MISO_DLEN_SPEC>,
    #[doc = "0x2c - SPI1 status register."]
    pub spi_mem_rd_status: crate::Reg<spi_mem_rd_status::SPI_MEM_RD_STATUS_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x34 - SPI1 misc register"]
    pub spi_mem_misc: crate::Reg<spi_mem_misc::SPI_MEM_MISC_SPEC>,
    #[doc = "0x38 - SPI1 TX CRC data register."]
    pub spi_mem_tx_crc: crate::Reg<spi_mem_tx_crc::SPI_MEM_TX_CRC_SPEC>,
    #[doc = "0x3c - SPI1 bit mode control register."]
    pub spi_mem_cache_fctrl: crate::Reg<spi_mem_cache_fctrl::SPI_MEM_CACHE_FCTRL_SPEC>,
    _reserved15: [u8; 0x18],
    #[doc = "0x58 - SPI1 memory data buffer0"]
    pub spi_mem_w0: crate::Reg<spi_mem_w0::SPI_MEM_W0_SPEC>,
    #[doc = "0x5c - SPI1 memory data buffer1"]
    pub spi_mem_w1: crate::Reg<spi_mem_w1::SPI_MEM_W1_SPEC>,
    #[doc = "0x60 - SPI1 memory data buffer2"]
    pub spi_mem_w2: crate::Reg<spi_mem_w2::SPI_MEM_W2_SPEC>,
    #[doc = "0x64 - SPI1 memory data buffer3"]
    pub spi_mem_w3: crate::Reg<spi_mem_w3::SPI_MEM_W3_SPEC>,
    #[doc = "0x68 - SPI1 memory data buffer4"]
    pub spi_mem_w4: crate::Reg<spi_mem_w4::SPI_MEM_W4_SPEC>,
    #[doc = "0x6c - SPI1 memory data buffer5"]
    pub spi_mem_w5: crate::Reg<spi_mem_w5::SPI_MEM_W5_SPEC>,
    #[doc = "0x70 - SPI1 memory data buffer6"]
    pub spi_mem_w6: crate::Reg<spi_mem_w6::SPI_MEM_W6_SPEC>,
    #[doc = "0x74 - SPI1 memory data buffer7"]
    pub spi_mem_w7: crate::Reg<spi_mem_w7::SPI_MEM_W7_SPEC>,
    #[doc = "0x78 - SPI1 memory data buffer8"]
    pub spi_mem_w8: crate::Reg<spi_mem_w8::SPI_MEM_W8_SPEC>,
    #[doc = "0x7c - SPI1 memory data buffer9"]
    pub spi_mem_w9: crate::Reg<spi_mem_w9::SPI_MEM_W9_SPEC>,
    #[doc = "0x80 - SPI1 memory data buffer10"]
    pub spi_mem_w10: crate::Reg<spi_mem_w10::SPI_MEM_W10_SPEC>,
    #[doc = "0x84 - SPI1 memory data buffer11"]
    pub spi_mem_w11: crate::Reg<spi_mem_w11::SPI_MEM_W11_SPEC>,
    #[doc = "0x88 - SPI1 memory data buffer12"]
    pub spi_mem_w12: crate::Reg<spi_mem_w12::SPI_MEM_W12_SPEC>,
    #[doc = "0x8c - SPI1 memory data buffer13"]
    pub spi_mem_w13: crate::Reg<spi_mem_w13::SPI_MEM_W13_SPEC>,
    #[doc = "0x90 - SPI1 memory data buffer14"]
    pub spi_mem_w14: crate::Reg<spi_mem_w14::SPI_MEM_W14_SPEC>,
    #[doc = "0x94 - SPI1 memory data buffer15"]
    pub spi_mem_w15: crate::Reg<spi_mem_w15::SPI_MEM_W15_SPEC>,
    #[doc = "0x98 - SPI1 wait idle control register"]
    pub spi_mem_flash_waiti_ctrl:
        crate::Reg<spi_mem_flash_waiti_ctrl::SPI_MEM_FLASH_WAITI_CTRL_SPEC>,
    #[doc = "0x9c - SPI1 flash suspend control register"]
    pub spi_mem_flash_sus_ctrl: crate::Reg<spi_mem_flash_sus_ctrl::SPI_MEM_FLASH_SUS_CTRL_SPEC>,
    #[doc = "0xa0 - SPI1 flash suspend command register"]
    pub spi_mem_flash_sus_cmd: crate::Reg<spi_mem_flash_sus_cmd::SPI_MEM_FLASH_SUS_CMD_SPEC>,
    #[doc = "0xa4 - SPI1 flash suspend status register"]
    pub spi_mem_sus_status: crate::Reg<spi_mem_sus_status::SPI_MEM_SUS_STATUS_SPEC>,
    #[doc = "0xa8 - SPI1 timing control register"]
    pub spi_mem_timing_cali: crate::Reg<spi_mem_timing_cali::SPI_MEM_TIMING_CALI_SPEC>,
    _reserved36: [u8; 0x14],
    #[doc = "0xc0 - SPI1 interrupt enable register"]
    pub spi_mem_int_ena: crate::Reg<spi_mem_int_ena::SPI_MEM_INT_ENA_SPEC>,
    #[doc = "0xc4 - SPI1 interrupt clear register"]
    pub spi_mem_int_clr: crate::Reg<spi_mem_int_clr::SPI_MEM_INT_CLR_SPEC>,
    #[doc = "0xc8 - SPI1 interrupt raw register"]
    pub spi_mem_int_raw: crate::Reg<spi_mem_int_raw::SPI_MEM_INT_RAW_SPEC>,
    #[doc = "0xcc - SPI1 interrupt status register"]
    pub spi_mem_int_st: crate::Reg<spi_mem_int_st::SPI_MEM_INT_ST_SPEC>,
    _reserved40: [u8; 0x0c],
    #[doc = "0xdc - SPI1 clk_gate register"]
    pub spi_mem_clock_gate: crate::Reg<spi_mem_clock_gate::SPI_MEM_CLOCK_GATE_SPEC>,
    _reserved41: [u8; 0x031c],
    #[doc = "0x3fc - Version control register"]
    pub spi_mem_date: crate::Reg<spi_mem_date::SPI_MEM_DATE_SPEC>,
}
#[doc = "SPI_MEM_CMD register accessor: an alias for `Reg<SPI_MEM_CMD_SPEC>`"]
pub type SPI_MEM_CMD = crate::Reg<spi_mem_cmd::SPI_MEM_CMD_SPEC>;
#[doc = "SPI1 memory command register"]
pub mod spi_mem_cmd;
#[doc = "SPI_MEM_ADDR register accessor: an alias for `Reg<SPI_MEM_ADDR_SPEC>`"]
pub type SPI_MEM_ADDR = crate::Reg<spi_mem_addr::SPI_MEM_ADDR_SPEC>;
#[doc = "SPI1 address register"]
pub mod spi_mem_addr;
#[doc = "SPI_MEM_CTRL register accessor: an alias for `Reg<SPI_MEM_CTRL_SPEC>`"]
pub type SPI_MEM_CTRL = crate::Reg<spi_mem_ctrl::SPI_MEM_CTRL_SPEC>;
#[doc = "SPI1 control register."]
pub mod spi_mem_ctrl;
#[doc = "SPI_MEM_CTRL1 register accessor: an alias for `Reg<SPI_MEM_CTRL1_SPEC>`"]
pub type SPI_MEM_CTRL1 = crate::Reg<spi_mem_ctrl1::SPI_MEM_CTRL1_SPEC>;
#[doc = "SPI1 control1 register."]
pub mod spi_mem_ctrl1;
#[doc = "SPI_MEM_CTRL2 register accessor: an alias for `Reg<SPI_MEM_CTRL2_SPEC>`"]
pub type SPI_MEM_CTRL2 = crate::Reg<spi_mem_ctrl2::SPI_MEM_CTRL2_SPEC>;
#[doc = "SPI1 control2 register."]
pub mod spi_mem_ctrl2;
#[doc = "SPI_MEM_CLOCK register accessor: an alias for `Reg<SPI_MEM_CLOCK_SPEC>`"]
pub type SPI_MEM_CLOCK = crate::Reg<spi_mem_clock::SPI_MEM_CLOCK_SPEC>;
#[doc = "SPI1 clock division control register."]
pub mod spi_mem_clock;
#[doc = "SPI_MEM_USER register accessor: an alias for `Reg<SPI_MEM_USER_SPEC>`"]
pub type SPI_MEM_USER = crate::Reg<spi_mem_user::SPI_MEM_USER_SPEC>;
#[doc = "SPI1 user register."]
pub mod spi_mem_user;
#[doc = "SPI_MEM_USER1 register accessor: an alias for `Reg<SPI_MEM_USER1_SPEC>`"]
pub type SPI_MEM_USER1 = crate::Reg<spi_mem_user1::SPI_MEM_USER1_SPEC>;
#[doc = "SPI1 user1 register."]
pub mod spi_mem_user1;
#[doc = "SPI_MEM_USER2 register accessor: an alias for `Reg<SPI_MEM_USER2_SPEC>`"]
pub type SPI_MEM_USER2 = crate::Reg<spi_mem_user2::SPI_MEM_USER2_SPEC>;
#[doc = "SPI1 user2 register."]
pub mod spi_mem_user2;
#[doc = "SPI_MEM_MOSI_DLEN register accessor: an alias for `Reg<SPI_MEM_MOSI_DLEN_SPEC>`"]
pub type SPI_MEM_MOSI_DLEN = crate::Reg<spi_mem_mosi_dlen::SPI_MEM_MOSI_DLEN_SPEC>;
#[doc = "SPI1 send data bit length control register."]
pub mod spi_mem_mosi_dlen;
#[doc = "SPI_MEM_MISO_DLEN register accessor: an alias for `Reg<SPI_MEM_MISO_DLEN_SPEC>`"]
pub type SPI_MEM_MISO_DLEN = crate::Reg<spi_mem_miso_dlen::SPI_MEM_MISO_DLEN_SPEC>;
#[doc = "SPI1 receive data bit length control register."]
pub mod spi_mem_miso_dlen;
#[doc = "SPI_MEM_RD_STATUS register accessor: an alias for `Reg<SPI_MEM_RD_STATUS_SPEC>`"]
pub type SPI_MEM_RD_STATUS = crate::Reg<spi_mem_rd_status::SPI_MEM_RD_STATUS_SPEC>;
#[doc = "SPI1 status register."]
pub mod spi_mem_rd_status;
#[doc = "SPI_MEM_MISC register accessor: an alias for `Reg<SPI_MEM_MISC_SPEC>`"]
pub type SPI_MEM_MISC = crate::Reg<spi_mem_misc::SPI_MEM_MISC_SPEC>;
#[doc = "SPI1 misc register"]
pub mod spi_mem_misc;
#[doc = "SPI_MEM_TX_CRC register accessor: an alias for `Reg<SPI_MEM_TX_CRC_SPEC>`"]
pub type SPI_MEM_TX_CRC = crate::Reg<spi_mem_tx_crc::SPI_MEM_TX_CRC_SPEC>;
#[doc = "SPI1 TX CRC data register."]
pub mod spi_mem_tx_crc;
#[doc = "SPI_MEM_CACHE_FCTRL register accessor: an alias for `Reg<SPI_MEM_CACHE_FCTRL_SPEC>`"]
pub type SPI_MEM_CACHE_FCTRL = crate::Reg<spi_mem_cache_fctrl::SPI_MEM_CACHE_FCTRL_SPEC>;
#[doc = "SPI1 bit mode control register."]
pub mod spi_mem_cache_fctrl;
#[doc = "SPI_MEM_W0 register accessor: an alias for `Reg<SPI_MEM_W0_SPEC>`"]
pub type SPI_MEM_W0 = crate::Reg<spi_mem_w0::SPI_MEM_W0_SPEC>;
#[doc = "SPI1 memory data buffer0"]
pub mod spi_mem_w0;
#[doc = "SPI_MEM_W1 register accessor: an alias for `Reg<SPI_MEM_W1_SPEC>`"]
pub type SPI_MEM_W1 = crate::Reg<spi_mem_w1::SPI_MEM_W1_SPEC>;
#[doc = "SPI1 memory data buffer1"]
pub mod spi_mem_w1;
#[doc = "SPI_MEM_W2 register accessor: an alias for `Reg<SPI_MEM_W2_SPEC>`"]
pub type SPI_MEM_W2 = crate::Reg<spi_mem_w2::SPI_MEM_W2_SPEC>;
#[doc = "SPI1 memory data buffer2"]
pub mod spi_mem_w2;
#[doc = "SPI_MEM_W3 register accessor: an alias for `Reg<SPI_MEM_W3_SPEC>`"]
pub type SPI_MEM_W3 = crate::Reg<spi_mem_w3::SPI_MEM_W3_SPEC>;
#[doc = "SPI1 memory data buffer3"]
pub mod spi_mem_w3;
#[doc = "SPI_MEM_W4 register accessor: an alias for `Reg<SPI_MEM_W4_SPEC>`"]
pub type SPI_MEM_W4 = crate::Reg<spi_mem_w4::SPI_MEM_W4_SPEC>;
#[doc = "SPI1 memory data buffer4"]
pub mod spi_mem_w4;
#[doc = "SPI_MEM_W5 register accessor: an alias for `Reg<SPI_MEM_W5_SPEC>`"]
pub type SPI_MEM_W5 = crate::Reg<spi_mem_w5::SPI_MEM_W5_SPEC>;
#[doc = "SPI1 memory data buffer5"]
pub mod spi_mem_w5;
#[doc = "SPI_MEM_W6 register accessor: an alias for `Reg<SPI_MEM_W6_SPEC>`"]
pub type SPI_MEM_W6 = crate::Reg<spi_mem_w6::SPI_MEM_W6_SPEC>;
#[doc = "SPI1 memory data buffer6"]
pub mod spi_mem_w6;
#[doc = "SPI_MEM_W7 register accessor: an alias for `Reg<SPI_MEM_W7_SPEC>`"]
pub type SPI_MEM_W7 = crate::Reg<spi_mem_w7::SPI_MEM_W7_SPEC>;
#[doc = "SPI1 memory data buffer7"]
pub mod spi_mem_w7;
#[doc = "SPI_MEM_W8 register accessor: an alias for `Reg<SPI_MEM_W8_SPEC>`"]
pub type SPI_MEM_W8 = crate::Reg<spi_mem_w8::SPI_MEM_W8_SPEC>;
#[doc = "SPI1 memory data buffer8"]
pub mod spi_mem_w8;
#[doc = "SPI_MEM_W9 register accessor: an alias for `Reg<SPI_MEM_W9_SPEC>`"]
pub type SPI_MEM_W9 = crate::Reg<spi_mem_w9::SPI_MEM_W9_SPEC>;
#[doc = "SPI1 memory data buffer9"]
pub mod spi_mem_w9;
#[doc = "SPI_MEM_W10 register accessor: an alias for `Reg<SPI_MEM_W10_SPEC>`"]
pub type SPI_MEM_W10 = crate::Reg<spi_mem_w10::SPI_MEM_W10_SPEC>;
#[doc = "SPI1 memory data buffer10"]
pub mod spi_mem_w10;
#[doc = "SPI_MEM_W11 register accessor: an alias for `Reg<SPI_MEM_W11_SPEC>`"]
pub type SPI_MEM_W11 = crate::Reg<spi_mem_w11::SPI_MEM_W11_SPEC>;
#[doc = "SPI1 memory data buffer11"]
pub mod spi_mem_w11;
#[doc = "SPI_MEM_W12 register accessor: an alias for `Reg<SPI_MEM_W12_SPEC>`"]
pub type SPI_MEM_W12 = crate::Reg<spi_mem_w12::SPI_MEM_W12_SPEC>;
#[doc = "SPI1 memory data buffer12"]
pub mod spi_mem_w12;
#[doc = "SPI_MEM_W13 register accessor: an alias for `Reg<SPI_MEM_W13_SPEC>`"]
pub type SPI_MEM_W13 = crate::Reg<spi_mem_w13::SPI_MEM_W13_SPEC>;
#[doc = "SPI1 memory data buffer13"]
pub mod spi_mem_w13;
#[doc = "SPI_MEM_W14 register accessor: an alias for `Reg<SPI_MEM_W14_SPEC>`"]
pub type SPI_MEM_W14 = crate::Reg<spi_mem_w14::SPI_MEM_W14_SPEC>;
#[doc = "SPI1 memory data buffer14"]
pub mod spi_mem_w14;
#[doc = "SPI_MEM_W15 register accessor: an alias for `Reg<SPI_MEM_W15_SPEC>`"]
pub type SPI_MEM_W15 = crate::Reg<spi_mem_w15::SPI_MEM_W15_SPEC>;
#[doc = "SPI1 memory data buffer15"]
pub mod spi_mem_w15;
#[doc = "SPI_MEM_FLASH_WAITI_CTRL register accessor: an alias for `Reg<SPI_MEM_FLASH_WAITI_CTRL_SPEC>`"]
pub type SPI_MEM_FLASH_WAITI_CTRL =
    crate::Reg<spi_mem_flash_waiti_ctrl::SPI_MEM_FLASH_WAITI_CTRL_SPEC>;
#[doc = "SPI1 wait idle control register"]
pub mod spi_mem_flash_waiti_ctrl;
#[doc = "SPI_MEM_FLASH_SUS_CTRL register accessor: an alias for `Reg<SPI_MEM_FLASH_SUS_CTRL_SPEC>`"]
pub type SPI_MEM_FLASH_SUS_CTRL = crate::Reg<spi_mem_flash_sus_ctrl::SPI_MEM_FLASH_SUS_CTRL_SPEC>;
#[doc = "SPI1 flash suspend control register"]
pub mod spi_mem_flash_sus_ctrl;
#[doc = "SPI_MEM_FLASH_SUS_CMD register accessor: an alias for `Reg<SPI_MEM_FLASH_SUS_CMD_SPEC>`"]
pub type SPI_MEM_FLASH_SUS_CMD = crate::Reg<spi_mem_flash_sus_cmd::SPI_MEM_FLASH_SUS_CMD_SPEC>;
#[doc = "SPI1 flash suspend command register"]
pub mod spi_mem_flash_sus_cmd;
#[doc = "SPI_MEM_SUS_STATUS register accessor: an alias for `Reg<SPI_MEM_SUS_STATUS_SPEC>`"]
pub type SPI_MEM_SUS_STATUS = crate::Reg<spi_mem_sus_status::SPI_MEM_SUS_STATUS_SPEC>;
#[doc = "SPI1 flash suspend status register"]
pub mod spi_mem_sus_status;
#[doc = "SPI_MEM_TIMING_CALI register accessor: an alias for `Reg<SPI_MEM_TIMING_CALI_SPEC>`"]
pub type SPI_MEM_TIMING_CALI = crate::Reg<spi_mem_timing_cali::SPI_MEM_TIMING_CALI_SPEC>;
#[doc = "SPI1 timing control register"]
pub mod spi_mem_timing_cali;
#[doc = "SPI_MEM_INT_ENA register accessor: an alias for `Reg<SPI_MEM_INT_ENA_SPEC>`"]
pub type SPI_MEM_INT_ENA = crate::Reg<spi_mem_int_ena::SPI_MEM_INT_ENA_SPEC>;
#[doc = "SPI1 interrupt enable register"]
pub mod spi_mem_int_ena;
#[doc = "SPI_MEM_INT_CLR register accessor: an alias for `Reg<SPI_MEM_INT_CLR_SPEC>`"]
pub type SPI_MEM_INT_CLR = crate::Reg<spi_mem_int_clr::SPI_MEM_INT_CLR_SPEC>;
#[doc = "SPI1 interrupt clear register"]
pub mod spi_mem_int_clr;
#[doc = "SPI_MEM_INT_RAW register accessor: an alias for `Reg<SPI_MEM_INT_RAW_SPEC>`"]
pub type SPI_MEM_INT_RAW = crate::Reg<spi_mem_int_raw::SPI_MEM_INT_RAW_SPEC>;
#[doc = "SPI1 interrupt raw register"]
pub mod spi_mem_int_raw;
#[doc = "SPI_MEM_INT_ST register accessor: an alias for `Reg<SPI_MEM_INT_ST_SPEC>`"]
pub type SPI_MEM_INT_ST = crate::Reg<spi_mem_int_st::SPI_MEM_INT_ST_SPEC>;
#[doc = "SPI1 interrupt status register"]
pub mod spi_mem_int_st;
#[doc = "SPI_MEM_CLOCK_GATE register accessor: an alias for `Reg<SPI_MEM_CLOCK_GATE_SPEC>`"]
pub type SPI_MEM_CLOCK_GATE = crate::Reg<spi_mem_clock_gate::SPI_MEM_CLOCK_GATE_SPEC>;
#[doc = "SPI1 clk_gate register"]
pub mod spi_mem_clock_gate;
#[doc = "SPI_MEM_DATE register accessor: an alias for `Reg<SPI_MEM_DATE_SPEC>`"]
pub type SPI_MEM_DATE = crate::Reg<spi_mem_date::SPI_MEM_DATE_SPEC>;
#[doc = "Version control register"]
pub mod spi_mem_date;
