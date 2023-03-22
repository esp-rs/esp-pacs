#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Command control register"]
    pub spi_cmd: SPI_CMD,
    #[doc = "0x04 - Address value register"]
    pub spi_addr: SPI_ADDR,
    #[doc = "0x08 - SPI control register"]
    pub spi_ctrl: SPI_CTRL,
    #[doc = "0x0c - SPI clock control register"]
    pub spi_clock: SPI_CLOCK,
    #[doc = "0x10 - SPI USER control register"]
    pub spi_user: SPI_USER,
    #[doc = "0x14 - SPI USER control register 1"]
    pub spi_user1: SPI_USER1,
    #[doc = "0x18 - SPI USER control register 2"]
    pub spi_user2: SPI_USER2,
    #[doc = "0x1c - SPI data bit length control register"]
    pub spi_ms_dlen: SPI_MS_DLEN,
    #[doc = "0x20 - SPI misc register"]
    pub spi_misc: SPI_MISC,
    #[doc = "0x24 - SPI input delay mode configuration"]
    pub spi_din_mode: SPI_DIN_MODE,
    #[doc = "0x28 - SPI input delay number configuration"]
    pub spi_din_num: SPI_DIN_NUM,
    #[doc = "0x2c - SPI output delay mode configuration"]
    pub spi_dout_mode: SPI_DOUT_MODE,
    #[doc = "0x30 - SPI DMA control register"]
    pub spi_dma_conf: SPI_DMA_CONF,
    #[doc = "0x34 - SPI interrupt enable register"]
    pub spi_dma_int_ena: SPI_DMA_INT_ENA,
    #[doc = "0x38 - SPI interrupt clear register"]
    pub spi_dma_int_clr: SPI_DMA_INT_CLR,
    #[doc = "0x3c - SPI interrupt raw register"]
    pub spi_dma_int_raw: SPI_DMA_INT_RAW,
    #[doc = "0x40 - SPI interrupt status register"]
    pub spi_dma_int_st: SPI_DMA_INT_ST,
    #[doc = "0x44 - SPI interrupt software set register"]
    pub spi_dma_int_set: SPI_DMA_INT_SET,
    _reserved18: [u8; 0x50],
    #[doc = "0x98 - SPI CPU-controlled buffer0"]
    pub spi_w0: SPI_W0,
    #[doc = "0x9c - SPI CPU-controlled buffer1"]
    pub spi_w1: SPI_W1,
    #[doc = "0xa0 - SPI CPU-controlled buffer2"]
    pub spi_w2: SPI_W2,
    #[doc = "0xa4 - SPI CPU-controlled buffer3"]
    pub spi_w3: SPI_W3,
    #[doc = "0xa8 - SPI CPU-controlled buffer4"]
    pub spi_w4: SPI_W4,
    #[doc = "0xac - SPI CPU-controlled buffer5"]
    pub spi_w5: SPI_W5,
    #[doc = "0xb0 - SPI CPU-controlled buffer6"]
    pub spi_w6: SPI_W6,
    #[doc = "0xb4 - SPI CPU-controlled buffer7"]
    pub spi_w7: SPI_W7,
    #[doc = "0xb8 - SPI CPU-controlled buffer8"]
    pub spi_w8: SPI_W8,
    #[doc = "0xbc - SPI CPU-controlled buffer9"]
    pub spi_w9: SPI_W9,
    #[doc = "0xc0 - SPI CPU-controlled buffer10"]
    pub spi_w10: SPI_W10,
    #[doc = "0xc4 - SPI CPU-controlled buffer11"]
    pub spi_w11: SPI_W11,
    #[doc = "0xc8 - SPI CPU-controlled buffer12"]
    pub spi_w12: SPI_W12,
    #[doc = "0xcc - SPI CPU-controlled buffer13"]
    pub spi_w13: SPI_W13,
    #[doc = "0xd0 - SPI CPU-controlled buffer14"]
    pub spi_w14: SPI_W14,
    #[doc = "0xd4 - SPI CPU-controlled buffer15"]
    pub spi_w15: SPI_W15,
    _reserved34: [u8; 0x08],
    #[doc = "0xe0 - SPI slave control register"]
    pub spi_slave: SPI_SLAVE,
    #[doc = "0xe4 - SPI slave control register 1"]
    pub spi_slave1: SPI_SLAVE1,
    #[doc = "0xe8 - SPI module clock and register clock control"]
    pub spi_clk_gate: SPI_CLK_GATE,
    _reserved37: [u8; 0x04],
    #[doc = "0xf0 - Version control"]
    pub spi_date: SPI_DATE,
}
#[doc = "SPI_CMD (rw) register accessor: an alias for `Reg<SPI_CMD_SPEC>`"]
pub type SPI_CMD = crate::Reg<spi_cmd::SPI_CMD_SPEC>;
#[doc = "Command control register"]
pub mod spi_cmd;
#[doc = "SPI_ADDR (rw) register accessor: an alias for `Reg<SPI_ADDR_SPEC>`"]
pub type SPI_ADDR = crate::Reg<spi_addr::SPI_ADDR_SPEC>;
#[doc = "Address value register"]
pub mod spi_addr;
#[doc = "SPI_CTRL (rw) register accessor: an alias for `Reg<SPI_CTRL_SPEC>`"]
pub type SPI_CTRL = crate::Reg<spi_ctrl::SPI_CTRL_SPEC>;
#[doc = "SPI control register"]
pub mod spi_ctrl;
#[doc = "SPI_CLOCK (rw) register accessor: an alias for `Reg<SPI_CLOCK_SPEC>`"]
pub type SPI_CLOCK = crate::Reg<spi_clock::SPI_CLOCK_SPEC>;
#[doc = "SPI clock control register"]
pub mod spi_clock;
#[doc = "SPI_USER (rw) register accessor: an alias for `Reg<SPI_USER_SPEC>`"]
pub type SPI_USER = crate::Reg<spi_user::SPI_USER_SPEC>;
#[doc = "SPI USER control register"]
pub mod spi_user;
#[doc = "SPI_USER1 (rw) register accessor: an alias for `Reg<SPI_USER1_SPEC>`"]
pub type SPI_USER1 = crate::Reg<spi_user1::SPI_USER1_SPEC>;
#[doc = "SPI USER control register 1"]
pub mod spi_user1;
#[doc = "SPI_USER2 (rw) register accessor: an alias for `Reg<SPI_USER2_SPEC>`"]
pub type SPI_USER2 = crate::Reg<spi_user2::SPI_USER2_SPEC>;
#[doc = "SPI USER control register 2"]
pub mod spi_user2;
#[doc = "SPI_MS_DLEN (rw) register accessor: an alias for `Reg<SPI_MS_DLEN_SPEC>`"]
pub type SPI_MS_DLEN = crate::Reg<spi_ms_dlen::SPI_MS_DLEN_SPEC>;
#[doc = "SPI data bit length control register"]
pub mod spi_ms_dlen;
#[doc = "SPI_MISC (rw) register accessor: an alias for `Reg<SPI_MISC_SPEC>`"]
pub type SPI_MISC = crate::Reg<spi_misc::SPI_MISC_SPEC>;
#[doc = "SPI misc register"]
pub mod spi_misc;
#[doc = "SPI_DIN_MODE (rw) register accessor: an alias for `Reg<SPI_DIN_MODE_SPEC>`"]
pub type SPI_DIN_MODE = crate::Reg<spi_din_mode::SPI_DIN_MODE_SPEC>;
#[doc = "SPI input delay mode configuration"]
pub mod spi_din_mode;
#[doc = "SPI_DIN_NUM (rw) register accessor: an alias for `Reg<SPI_DIN_NUM_SPEC>`"]
pub type SPI_DIN_NUM = crate::Reg<spi_din_num::SPI_DIN_NUM_SPEC>;
#[doc = "SPI input delay number configuration"]
pub mod spi_din_num;
#[doc = "SPI_DOUT_MODE (rw) register accessor: an alias for `Reg<SPI_DOUT_MODE_SPEC>`"]
pub type SPI_DOUT_MODE = crate::Reg<spi_dout_mode::SPI_DOUT_MODE_SPEC>;
#[doc = "SPI output delay mode configuration"]
pub mod spi_dout_mode;
#[doc = "SPI_DMA_CONF (rw) register accessor: an alias for `Reg<SPI_DMA_CONF_SPEC>`"]
pub type SPI_DMA_CONF = crate::Reg<spi_dma_conf::SPI_DMA_CONF_SPEC>;
#[doc = "SPI DMA control register"]
pub mod spi_dma_conf;
#[doc = "SPI_DMA_INT_ENA (rw) register accessor: an alias for `Reg<SPI_DMA_INT_ENA_SPEC>`"]
pub type SPI_DMA_INT_ENA = crate::Reg<spi_dma_int_ena::SPI_DMA_INT_ENA_SPEC>;
#[doc = "SPI interrupt enable register"]
pub mod spi_dma_int_ena;
#[doc = "SPI_DMA_INT_CLR (w) register accessor: an alias for `Reg<SPI_DMA_INT_CLR_SPEC>`"]
pub type SPI_DMA_INT_CLR = crate::Reg<spi_dma_int_clr::SPI_DMA_INT_CLR_SPEC>;
#[doc = "SPI interrupt clear register"]
pub mod spi_dma_int_clr;
#[doc = "SPI_DMA_INT_RAW (rw) register accessor: an alias for `Reg<SPI_DMA_INT_RAW_SPEC>`"]
pub type SPI_DMA_INT_RAW = crate::Reg<spi_dma_int_raw::SPI_DMA_INT_RAW_SPEC>;
#[doc = "SPI interrupt raw register"]
pub mod spi_dma_int_raw;
#[doc = "SPI_DMA_INT_ST (r) register accessor: an alias for `Reg<SPI_DMA_INT_ST_SPEC>`"]
pub type SPI_DMA_INT_ST = crate::Reg<spi_dma_int_st::SPI_DMA_INT_ST_SPEC>;
#[doc = "SPI interrupt status register"]
pub mod spi_dma_int_st;
#[doc = "SPI_DMA_INT_SET (w) register accessor: an alias for `Reg<SPI_DMA_INT_SET_SPEC>`"]
pub type SPI_DMA_INT_SET = crate::Reg<spi_dma_int_set::SPI_DMA_INT_SET_SPEC>;
#[doc = "SPI interrupt software set register"]
pub mod spi_dma_int_set;
#[doc = "SPI_W0 (rw) register accessor: an alias for `Reg<SPI_W0_SPEC>`"]
pub type SPI_W0 = crate::Reg<spi_w0::SPI_W0_SPEC>;
#[doc = "SPI CPU-controlled buffer0"]
pub mod spi_w0;
#[doc = "SPI_W1 (rw) register accessor: an alias for `Reg<SPI_W1_SPEC>`"]
pub type SPI_W1 = crate::Reg<spi_w1::SPI_W1_SPEC>;
#[doc = "SPI CPU-controlled buffer1"]
pub mod spi_w1;
#[doc = "SPI_W2 (rw) register accessor: an alias for `Reg<SPI_W2_SPEC>`"]
pub type SPI_W2 = crate::Reg<spi_w2::SPI_W2_SPEC>;
#[doc = "SPI CPU-controlled buffer2"]
pub mod spi_w2;
#[doc = "SPI_W3 (rw) register accessor: an alias for `Reg<SPI_W3_SPEC>`"]
pub type SPI_W3 = crate::Reg<spi_w3::SPI_W3_SPEC>;
#[doc = "SPI CPU-controlled buffer3"]
pub mod spi_w3;
#[doc = "SPI_W4 (rw) register accessor: an alias for `Reg<SPI_W4_SPEC>`"]
pub type SPI_W4 = crate::Reg<spi_w4::SPI_W4_SPEC>;
#[doc = "SPI CPU-controlled buffer4"]
pub mod spi_w4;
#[doc = "SPI_W5 (rw) register accessor: an alias for `Reg<SPI_W5_SPEC>`"]
pub type SPI_W5 = crate::Reg<spi_w5::SPI_W5_SPEC>;
#[doc = "SPI CPU-controlled buffer5"]
pub mod spi_w5;
#[doc = "SPI_W6 (rw) register accessor: an alias for `Reg<SPI_W6_SPEC>`"]
pub type SPI_W6 = crate::Reg<spi_w6::SPI_W6_SPEC>;
#[doc = "SPI CPU-controlled buffer6"]
pub mod spi_w6;
#[doc = "SPI_W7 (rw) register accessor: an alias for `Reg<SPI_W7_SPEC>`"]
pub type SPI_W7 = crate::Reg<spi_w7::SPI_W7_SPEC>;
#[doc = "SPI CPU-controlled buffer7"]
pub mod spi_w7;
#[doc = "SPI_W8 (rw) register accessor: an alias for `Reg<SPI_W8_SPEC>`"]
pub type SPI_W8 = crate::Reg<spi_w8::SPI_W8_SPEC>;
#[doc = "SPI CPU-controlled buffer8"]
pub mod spi_w8;
#[doc = "SPI_W9 (rw) register accessor: an alias for `Reg<SPI_W9_SPEC>`"]
pub type SPI_W9 = crate::Reg<spi_w9::SPI_W9_SPEC>;
#[doc = "SPI CPU-controlled buffer9"]
pub mod spi_w9;
#[doc = "SPI_W10 (rw) register accessor: an alias for `Reg<SPI_W10_SPEC>`"]
pub type SPI_W10 = crate::Reg<spi_w10::SPI_W10_SPEC>;
#[doc = "SPI CPU-controlled buffer10"]
pub mod spi_w10;
#[doc = "SPI_W11 (rw) register accessor: an alias for `Reg<SPI_W11_SPEC>`"]
pub type SPI_W11 = crate::Reg<spi_w11::SPI_W11_SPEC>;
#[doc = "SPI CPU-controlled buffer11"]
pub mod spi_w11;
#[doc = "SPI_W12 (rw) register accessor: an alias for `Reg<SPI_W12_SPEC>`"]
pub type SPI_W12 = crate::Reg<spi_w12::SPI_W12_SPEC>;
#[doc = "SPI CPU-controlled buffer12"]
pub mod spi_w12;
#[doc = "SPI_W13 (rw) register accessor: an alias for `Reg<SPI_W13_SPEC>`"]
pub type SPI_W13 = crate::Reg<spi_w13::SPI_W13_SPEC>;
#[doc = "SPI CPU-controlled buffer13"]
pub mod spi_w13;
#[doc = "SPI_W14 (rw) register accessor: an alias for `Reg<SPI_W14_SPEC>`"]
pub type SPI_W14 = crate::Reg<spi_w14::SPI_W14_SPEC>;
#[doc = "SPI CPU-controlled buffer14"]
pub mod spi_w14;
#[doc = "SPI_W15 (rw) register accessor: an alias for `Reg<SPI_W15_SPEC>`"]
pub type SPI_W15 = crate::Reg<spi_w15::SPI_W15_SPEC>;
#[doc = "SPI CPU-controlled buffer15"]
pub mod spi_w15;
#[doc = "SPI_SLAVE (rw) register accessor: an alias for `Reg<SPI_SLAVE_SPEC>`"]
pub type SPI_SLAVE = crate::Reg<spi_slave::SPI_SLAVE_SPEC>;
#[doc = "SPI slave control register"]
pub mod spi_slave;
#[doc = "SPI_SLAVE1 (rw) register accessor: an alias for `Reg<SPI_SLAVE1_SPEC>`"]
pub type SPI_SLAVE1 = crate::Reg<spi_slave1::SPI_SLAVE1_SPEC>;
#[doc = "SPI slave control register 1"]
pub mod spi_slave1;
#[doc = "SPI_CLK_GATE (rw) register accessor: an alias for `Reg<SPI_CLK_GATE_SPEC>`"]
pub type SPI_CLK_GATE = crate::Reg<spi_clk_gate::SPI_CLK_GATE_SPEC>;
#[doc = "SPI module clock and register clock control"]
pub mod spi_clk_gate;
#[doc = "SPI_DATE (rw) register accessor: an alias for `Reg<SPI_DATE_SPEC>`"]
pub type SPI_DATE = crate::Reg<spi_date::SPI_DATE_SPEC>;
#[doc = "Version control"]
pub mod spi_date;
