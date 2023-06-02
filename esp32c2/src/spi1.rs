#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI1 memory command register"]
    pub cmd: CMD,
    #[doc = "0x04 - SPI1 address register"]
    pub addr: ADDR,
    #[doc = "0x08 - SPI1 control register."]
    pub ctrl: CTRL,
    #[doc = "0x0c - SPI1 control1 register."]
    pub ctrl1: CTRL1,
    #[doc = "0x10 - SPI1 control2 register."]
    pub ctrl2: CTRL2,
    #[doc = "0x14 - SPI1 clock division control register."]
    pub clock: CLOCK,
    #[doc = "0x18 - SPI1 user register."]
    pub user: USER,
    #[doc = "0x1c - SPI1 user1 register."]
    pub user1: USER1,
    #[doc = "0x20 - SPI1 user2 register."]
    pub user2: USER2,
    #[doc = "0x24 - SPI1 send data bit length control register."]
    pub mosi_dlen: MOSI_DLEN,
    #[doc = "0x28 - SPI1 receive data bit length control register."]
    pub miso_dlen: MISO_DLEN,
    #[doc = "0x2c - SPI1 status register."]
    pub rd_status: RD_STATUS,
    _reserved12: [u8; 0x04],
    #[doc = "0x34 - SPI1 misc register"]
    pub misc: MISC,
    #[doc = "0x38 - SPI1 TX CRC data register."]
    pub tx_crc: TX_CRC,
    #[doc = "0x3c - SPI1 bit mode control register."]
    pub cache_fctrl: CACHE_FCTRL,
    _reserved15: [u8; 0x18],
    #[doc = "0x58 - SPI1 memory data buffer0"]
    pub w0: W0,
    #[doc = "0x5c - SPI1 memory data buffer1"]
    pub w1: W1,
    #[doc = "0x60 - SPI1 memory data buffer2"]
    pub w2: W2,
    #[doc = "0x64 - SPI1 memory data buffer3"]
    pub w3: W3,
    #[doc = "0x68 - SPI1 memory data buffer4"]
    pub w4: W4,
    #[doc = "0x6c - SPI1 memory data buffer5"]
    pub w5: W5,
    #[doc = "0x70 - SPI1 memory data buffer6"]
    pub w6: W6,
    #[doc = "0x74 - SPI1 memory data buffer7"]
    pub w7: W7,
    #[doc = "0x78 - SPI1 memory data buffer8"]
    pub w8: W8,
    #[doc = "0x7c - SPI1 memory data buffer9"]
    pub w9: W9,
    #[doc = "0x80 - SPI1 memory data buffer10"]
    pub w10: W10,
    #[doc = "0x84 - SPI1 memory data buffer11"]
    pub w11: W11,
    #[doc = "0x88 - SPI1 memory data buffer12"]
    pub w12: W12,
    #[doc = "0x8c - SPI1 memory data buffer13"]
    pub w13: W13,
    #[doc = "0x90 - SPI1 memory data buffer14"]
    pub w14: W14,
    #[doc = "0x94 - SPI1 memory data buffer15"]
    pub w15: W15,
    #[doc = "0x98 - SPI1 wait idle control register"]
    pub flash_waiti_ctrl: FLASH_WAITI_CTRL,
    #[doc = "0x9c - SPI1 flash suspend control register"]
    pub flash_sus_ctrl: FLASH_SUS_CTRL,
    #[doc = "0xa0 - SPI1 flash suspend command register"]
    pub flash_sus_cmd: FLASH_SUS_CMD,
    #[doc = "0xa4 - SPI1 flash suspend status register"]
    pub sus_status: SUS_STATUS,
    #[doc = "0xa8 - SPI1 timing control register"]
    pub timing_cali: TIMING_CALI,
    _reserved36: [u8; 0x14],
    #[doc = "0xc0 - SPI1 interrupt enable register"]
    pub int_ena: INT_ENA,
    #[doc = "0xc4 - SPI1 interrupt clear register"]
    pub int_clr: INT_CLR,
    #[doc = "0xc8 - SPI1 interrupt raw register"]
    pub int_raw: INT_RAW,
    #[doc = "0xcc - SPI1 interrupt status register"]
    pub int_st: INT_ST,
    _reserved40: [u8; 0x0c],
    #[doc = "0xdc - SPI1 clk_gate register"]
    pub clock_gate: CLOCK_GATE,
    _reserved41: [u8; 0x031c],
    #[doc = "0x3fc - Version control register"]
    pub date: DATE,
}
#[doc = "CMD (rw) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "SPI1 memory command register"]
pub mod cmd;
#[doc = "ADDR (rw) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "SPI1 address register"]
pub mod addr;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "SPI1 control register."]
pub mod ctrl;
#[doc = "CTRL1 (rw) register accessor: an alias for `Reg<CTRL1_SPEC>`"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "SPI1 control1 register."]
pub mod ctrl1;
#[doc = "CTRL2 (w) register accessor: an alias for `Reg<CTRL2_SPEC>`"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "SPI1 control2 register."]
pub mod ctrl2;
#[doc = "CLOCK (rw) register accessor: an alias for `Reg<CLOCK_SPEC>`"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = "SPI1 clock division control register."]
pub mod clock;
#[doc = "USER (rw) register accessor: an alias for `Reg<USER_SPEC>`"]
pub type USER = crate::Reg<user::USER_SPEC>;
#[doc = "SPI1 user register."]
pub mod user;
#[doc = "USER1 (rw) register accessor: an alias for `Reg<USER1_SPEC>`"]
pub type USER1 = crate::Reg<user1::USER1_SPEC>;
#[doc = "SPI1 user1 register."]
pub mod user1;
#[doc = "USER2 (rw) register accessor: an alias for `Reg<USER2_SPEC>`"]
pub type USER2 = crate::Reg<user2::USER2_SPEC>;
#[doc = "SPI1 user2 register."]
pub mod user2;
#[doc = "MOSI_DLEN (rw) register accessor: an alias for `Reg<MOSI_DLEN_SPEC>`"]
pub type MOSI_DLEN = crate::Reg<mosi_dlen::MOSI_DLEN_SPEC>;
#[doc = "SPI1 send data bit length control register."]
pub mod mosi_dlen;
#[doc = "MISO_DLEN (rw) register accessor: an alias for `Reg<MISO_DLEN_SPEC>`"]
pub type MISO_DLEN = crate::Reg<miso_dlen::MISO_DLEN_SPEC>;
#[doc = "SPI1 receive data bit length control register."]
pub mod miso_dlen;
#[doc = "RD_STATUS (rw) register accessor: an alias for `Reg<RD_STATUS_SPEC>`"]
pub type RD_STATUS = crate::Reg<rd_status::RD_STATUS_SPEC>;
#[doc = "SPI1 status register."]
pub mod rd_status;
#[doc = "MISC (rw) register accessor: an alias for `Reg<MISC_SPEC>`"]
pub type MISC = crate::Reg<misc::MISC_SPEC>;
#[doc = "SPI1 misc register"]
pub mod misc;
#[doc = "TX_CRC (r) register accessor: an alias for `Reg<TX_CRC_SPEC>`"]
pub type TX_CRC = crate::Reg<tx_crc::TX_CRC_SPEC>;
#[doc = "SPI1 TX CRC data register."]
pub mod tx_crc;
#[doc = "CACHE_FCTRL (rw) register accessor: an alias for `Reg<CACHE_FCTRL_SPEC>`"]
pub type CACHE_FCTRL = crate::Reg<cache_fctrl::CACHE_FCTRL_SPEC>;
#[doc = "SPI1 bit mode control register."]
pub mod cache_fctrl;
#[doc = "W0 (rw) register accessor: an alias for `Reg<W0_SPEC>`"]
pub type W0 = crate::Reg<w0::W0_SPEC>;
#[doc = "SPI1 memory data buffer0"]
pub mod w0;
#[doc = "W1 (rw) register accessor: an alias for `Reg<W1_SPEC>`"]
pub type W1 = crate::Reg<w1::W1_SPEC>;
#[doc = "SPI1 memory data buffer1"]
pub mod w1;
#[doc = "W2 (rw) register accessor: an alias for `Reg<W2_SPEC>`"]
pub type W2 = crate::Reg<w2::W2_SPEC>;
#[doc = "SPI1 memory data buffer2"]
pub mod w2;
#[doc = "W3 (rw) register accessor: an alias for `Reg<W3_SPEC>`"]
pub type W3 = crate::Reg<w3::W3_SPEC>;
#[doc = "SPI1 memory data buffer3"]
pub mod w3;
#[doc = "W4 (rw) register accessor: an alias for `Reg<W4_SPEC>`"]
pub type W4 = crate::Reg<w4::W4_SPEC>;
#[doc = "SPI1 memory data buffer4"]
pub mod w4;
#[doc = "W5 (rw) register accessor: an alias for `Reg<W5_SPEC>`"]
pub type W5 = crate::Reg<w5::W5_SPEC>;
#[doc = "SPI1 memory data buffer5"]
pub mod w5;
#[doc = "W6 (rw) register accessor: an alias for `Reg<W6_SPEC>`"]
pub type W6 = crate::Reg<w6::W6_SPEC>;
#[doc = "SPI1 memory data buffer6"]
pub mod w6;
#[doc = "W7 (rw) register accessor: an alias for `Reg<W7_SPEC>`"]
pub type W7 = crate::Reg<w7::W7_SPEC>;
#[doc = "SPI1 memory data buffer7"]
pub mod w7;
#[doc = "W8 (rw) register accessor: an alias for `Reg<W8_SPEC>`"]
pub type W8 = crate::Reg<w8::W8_SPEC>;
#[doc = "SPI1 memory data buffer8"]
pub mod w8;
#[doc = "W9 (rw) register accessor: an alias for `Reg<W9_SPEC>`"]
pub type W9 = crate::Reg<w9::W9_SPEC>;
#[doc = "SPI1 memory data buffer9"]
pub mod w9;
#[doc = "W10 (rw) register accessor: an alias for `Reg<W10_SPEC>`"]
pub type W10 = crate::Reg<w10::W10_SPEC>;
#[doc = "SPI1 memory data buffer10"]
pub mod w10;
#[doc = "W11 (rw) register accessor: an alias for `Reg<W11_SPEC>`"]
pub type W11 = crate::Reg<w11::W11_SPEC>;
#[doc = "SPI1 memory data buffer11"]
pub mod w11;
#[doc = "W12 (rw) register accessor: an alias for `Reg<W12_SPEC>`"]
pub type W12 = crate::Reg<w12::W12_SPEC>;
#[doc = "SPI1 memory data buffer12"]
pub mod w12;
#[doc = "W13 (rw) register accessor: an alias for `Reg<W13_SPEC>`"]
pub type W13 = crate::Reg<w13::W13_SPEC>;
#[doc = "SPI1 memory data buffer13"]
pub mod w13;
#[doc = "W14 (rw) register accessor: an alias for `Reg<W14_SPEC>`"]
pub type W14 = crate::Reg<w14::W14_SPEC>;
#[doc = "SPI1 memory data buffer14"]
pub mod w14;
#[doc = "W15 (rw) register accessor: an alias for `Reg<W15_SPEC>`"]
pub type W15 = crate::Reg<w15::W15_SPEC>;
#[doc = "SPI1 memory data buffer15"]
pub mod w15;
#[doc = "FLASH_WAITI_CTRL (rw) register accessor: an alias for `Reg<FLASH_WAITI_CTRL_SPEC>`"]
pub type FLASH_WAITI_CTRL = crate::Reg<flash_waiti_ctrl::FLASH_WAITI_CTRL_SPEC>;
#[doc = "SPI1 wait idle control register"]
pub mod flash_waiti_ctrl;
#[doc = "FLASH_SUS_CTRL (rw) register accessor: an alias for `Reg<FLASH_SUS_CTRL_SPEC>`"]
pub type FLASH_SUS_CTRL = crate::Reg<flash_sus_ctrl::FLASH_SUS_CTRL_SPEC>;
#[doc = "SPI1 flash suspend control register"]
pub mod flash_sus_ctrl;
#[doc = "FLASH_SUS_CMD (rw) register accessor: an alias for `Reg<FLASH_SUS_CMD_SPEC>`"]
pub type FLASH_SUS_CMD = crate::Reg<flash_sus_cmd::FLASH_SUS_CMD_SPEC>;
#[doc = "SPI1 flash suspend command register"]
pub mod flash_sus_cmd;
#[doc = "SUS_STATUS (rw) register accessor: an alias for `Reg<SUS_STATUS_SPEC>`"]
pub type SUS_STATUS = crate::Reg<sus_status::SUS_STATUS_SPEC>;
#[doc = "SPI1 flash suspend status register"]
pub mod sus_status;
#[doc = "TIMING_CALI (r) register accessor: an alias for `Reg<TIMING_CALI_SPEC>`"]
pub type TIMING_CALI = crate::Reg<timing_cali::TIMING_CALI_SPEC>;
#[doc = "SPI1 timing control register"]
pub mod timing_cali;
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
#[doc = "CLOCK_GATE (rw) register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "SPI1 clk_gate register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
