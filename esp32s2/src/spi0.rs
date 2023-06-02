#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Command control register"]
    pub cmd: CMD,
    #[doc = "0x04 - Address value"]
    pub addr: ADDR,
    #[doc = "0x08 - SPI control register"]
    pub ctrl: CTRL,
    #[doc = "0x0c - SPI control register 1"]
    pub ctrl1: CTRL1,
    #[doc = "0x10 - SPI control register 2"]
    pub ctrl2: CTRL2,
    #[doc = "0x14 - SPI clock control register"]
    pub clock: CLOCK,
    #[doc = "0x18 - SPI USER control register"]
    pub user: USER,
    #[doc = "0x1c - SPI USER control register 1"]
    pub user1: USER1,
    #[doc = "0x20 - SPI USER control register 2"]
    pub user2: USER2,
    #[doc = "0x24 - MOSI length"]
    pub mosi_dlen: MOSI_DLEN,
    #[doc = "0x28 - MISO length"]
    pub miso_dlen: MISO_DLEN,
    #[doc = "0x2c - SPI misc register"]
    pub misc: MISC,
    #[doc = "0x30 - SPI slave control register"]
    pub slave: SLAVE,
    #[doc = "0x34 - SPI slave control register 1"]
    pub slave1: SLAVE1,
    #[doc = "0x38 - SPI slave Wr_BUF interrupt and CONF control register"]
    pub slv_wrbuf_dlen: SLV_WRBUF_DLEN,
    #[doc = "0x3c - SPI magic error and slave control register"]
    pub slv_rdbuf_dlen: SLV_RDBUF_DLEN,
    #[doc = "0x40 - SPI interrupt control register"]
    pub slv_rd_byte: SLV_RD_BYTE,
    #[doc = "0x44 - SPI master status and DMA read byte control register"]
    pub fsm: FSM,
    #[doc = "0x48 - SPI hold register"]
    pub hold: HOLD,
    #[doc = "0x4c - SPI DMA control register"]
    pub dma_conf: DMA_CONF,
    #[doc = "0x50 - SPI DMA TX link configuration"]
    pub dma_out_link: DMA_OUT_LINK,
    #[doc = "0x54 - SPI DMA RX link configuration"]
    pub dma_in_link: DMA_IN_LINK,
    #[doc = "0x58 - SPI DMA interrupt enable register"]
    pub dma_int_ena: DMA_INT_ENA,
    #[doc = "0x5c - SPI DMA interrupt raw register"]
    pub dma_int_raw: DMA_INT_RAW,
    #[doc = "0x60 - SPI DMA interrupt status register"]
    pub dma_int_st: DMA_INT_ST,
    #[doc = "0x64 - SPI DMA interrupt clear register"]
    pub dma_int_clr: DMA_INT_CLR,
    #[doc = "0x68 - The latest SPI DMA RX descriptor address receiving error"]
    pub in_err_eof_des_addr: IN_ERR_EOF_DES_ADDR,
    #[doc = "0x6c - The latest SPI DMA eof RX descriptor address"]
    pub in_suc_eof_des_addr: IN_SUC_EOF_DES_ADDR,
    #[doc = "0x70 - Current SPI DMA RX descriptor pointer"]
    pub inlink_dscr: INLINK_DSCR,
    #[doc = "0x74 - Next SPI DMA RX descriptor pointer"]
    pub inlink_dscr_bf0: INLINK_DSCR_BF0,
    #[doc = "0x78 - Current SPI DMA RX buffer pointer"]
    pub inlink_dscr_bf1: INLINK_DSCR_BF1,
    #[doc = "0x7c - The latest SPI DMA eof TX buffer address"]
    pub out_eof_bfr_des_addr: OUT_EOF_BFR_DES_ADDR,
    #[doc = "0x80 - The latest SPI DMA eof TX descriptor address"]
    pub out_eof_des_addr: OUT_EOF_DES_ADDR,
    #[doc = "0x84 - Current SPI DMA TX descriptor pointer"]
    pub outlink_dscr: OUTLINK_DSCR,
    #[doc = "0x88 - Next SPI DMA TX descriptor pointer"]
    pub outlink_dscr_bf0: OUTLINK_DSCR_BF0,
    #[doc = "0x8c - Current SPI DMA TX buffer pointer"]
    pub outlink_dscr_bf1: OUTLINK_DSCR_BF1,
    #[doc = "0x90 - SPI DMA TX status"]
    pub dma_outstatus: DMA_OUTSTATUS,
    #[doc = "0x94 - SPI DMA RX status"]
    pub dma_instatus: DMA_INSTATUS,
    #[doc = "0x98 - Data buffer 0"]
    pub w0: W0,
    #[doc = "0x9c - Data buffer 1"]
    pub w1: W1,
    #[doc = "0xa0 - Data buffer 2"]
    pub w2: W2,
    #[doc = "0xa4 - Data buffer 3"]
    pub w3: W3,
    #[doc = "0xa8 - Data buffer 4"]
    pub w4: W4,
    #[doc = "0xac - Data buffer 5"]
    pub w5: W5,
    #[doc = "0xb0 - Data buffer 6"]
    pub w6: W6,
    #[doc = "0xb4 - Data buffer 7"]
    pub w7: W7,
    #[doc = "0xb8 - Data buffer 8"]
    pub w8: W8,
    #[doc = "0xbc - Data buffer 9"]
    pub w9: W9,
    #[doc = "0xc0 - Data buffer 10"]
    pub w10: W10,
    #[doc = "0xc4 - Data buffer 11"]
    pub w11: W11,
    #[doc = "0xc8 - Data buffer 12"]
    pub w12: W12,
    #[doc = "0xcc - Data buffer 13"]
    pub w13: W13,
    #[doc = "0xd0 - Data buffer 14"]
    pub w14: W14,
    #[doc = "0xd4 - Data buffer 15"]
    pub w15: W15,
    #[doc = "0xd8 - Data buffer 16"]
    pub w16: W16,
    #[doc = "0xdc - Data buffer 17"]
    pub w17: W17,
    #[doc = "0xe0 - SPI input delay mode configuration"]
    pub din_mode: DIN_MODE,
    #[doc = "0xe4 - SPI input delay number configuration"]
    pub din_num: DIN_NUM,
    #[doc = "0xe8 - SPI output delay mode configuration"]
    pub dout_mode: DOUT_MODE,
    #[doc = "0xec - SPI output delay number configuration"]
    pub dout_num: DOUT_NUM,
    #[doc = "0xf0 - LCD frame control register"]
    pub lcd_ctrl: LCD_CTRL,
    #[doc = "0xf4 - LCD frame control1 register"]
    pub lcd_ctrl1: LCD_CTRL1,
    #[doc = "0xf8 - LCD frame control2 register"]
    pub lcd_ctrl2: LCD_CTRL2,
    #[doc = "0xfc - LCD delay number"]
    pub lcd_d_mode: LCD_D_MODE,
    #[doc = "0x100 - LCD delay mode"]
    pub lcd_d_num: LCD_D_NUM,
    _reserved65: [u8; 0x02f8],
    #[doc = "0x3fc - SPI version control"]
    pub reg_date: REG_DATE,
}
#[doc = "CMD (rw) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command control register"]
pub mod cmd;
#[doc = "ADDR (rw) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Address value"]
pub mod addr;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "SPI control register"]
pub mod ctrl;
#[doc = "CTRL1 (rw) register accessor: an alias for `Reg<CTRL1_SPEC>`"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "SPI control register 1"]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: an alias for `Reg<CTRL2_SPEC>`"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "SPI control register 2"]
pub mod ctrl2;
#[doc = "CLOCK (rw) register accessor: an alias for `Reg<CLOCK_SPEC>`"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = "SPI clock control register"]
pub mod clock;
#[doc = "USER (rw) register accessor: an alias for `Reg<USER_SPEC>`"]
pub type USER = crate::Reg<user::USER_SPEC>;
#[doc = "SPI USER control register"]
pub mod user;
#[doc = "USER1 (rw) register accessor: an alias for `Reg<USER1_SPEC>`"]
pub type USER1 = crate::Reg<user1::USER1_SPEC>;
#[doc = "SPI USER control register 1"]
pub mod user1;
#[doc = "USER2 (rw) register accessor: an alias for `Reg<USER2_SPEC>`"]
pub type USER2 = crate::Reg<user2::USER2_SPEC>;
#[doc = "SPI USER control register 2"]
pub mod user2;
#[doc = "MOSI_DLEN (rw) register accessor: an alias for `Reg<MOSI_DLEN_SPEC>`"]
pub type MOSI_DLEN = crate::Reg<mosi_dlen::MOSI_DLEN_SPEC>;
#[doc = "MOSI length"]
pub mod mosi_dlen;
#[doc = "MISO_DLEN (rw) register accessor: an alias for `Reg<MISO_DLEN_SPEC>`"]
pub type MISO_DLEN = crate::Reg<miso_dlen::MISO_DLEN_SPEC>;
#[doc = "MISO length"]
pub mod miso_dlen;
#[doc = "MISC (rw) register accessor: an alias for `Reg<MISC_SPEC>`"]
pub type MISC = crate::Reg<misc::MISC_SPEC>;
#[doc = "SPI misc register"]
pub mod misc;
#[doc = "SLAVE (rw) register accessor: an alias for `Reg<SLAVE_SPEC>`"]
pub type SLAVE = crate::Reg<slave::SLAVE_SPEC>;
#[doc = "SPI slave control register"]
pub mod slave;
#[doc = "SLAVE1 (rw) register accessor: an alias for `Reg<SLAVE1_SPEC>`"]
pub type SLAVE1 = crate::Reg<slave1::SLAVE1_SPEC>;
#[doc = "SPI slave control register 1"]
pub mod slave1;
#[doc = "SLV_WRBUF_DLEN (rw) register accessor: an alias for `Reg<SLV_WRBUF_DLEN_SPEC>`"]
pub type SLV_WRBUF_DLEN = crate::Reg<slv_wrbuf_dlen::SLV_WRBUF_DLEN_SPEC>;
#[doc = "SPI slave Wr_BUF interrupt and CONF control register"]
pub mod slv_wrbuf_dlen;
#[doc = "SLV_RDBUF_DLEN (rw) register accessor: an alias for `Reg<SLV_RDBUF_DLEN_SPEC>`"]
pub type SLV_RDBUF_DLEN = crate::Reg<slv_rdbuf_dlen::SLV_RDBUF_DLEN_SPEC>;
#[doc = "SPI magic error and slave control register"]
pub mod slv_rdbuf_dlen;
#[doc = "SLV_RD_BYTE (rw) register accessor: an alias for `Reg<SLV_RD_BYTE_SPEC>`"]
pub type SLV_RD_BYTE = crate::Reg<slv_rd_byte::SLV_RD_BYTE_SPEC>;
#[doc = "SPI interrupt control register"]
pub mod slv_rd_byte;
#[doc = "FSM (rw) register accessor: an alias for `Reg<FSM_SPEC>`"]
pub type FSM = crate::Reg<fsm::FSM_SPEC>;
#[doc = "SPI master status and DMA read byte control register"]
pub mod fsm;
#[doc = "HOLD (rw) register accessor: an alias for `Reg<HOLD_SPEC>`"]
pub type HOLD = crate::Reg<hold::HOLD_SPEC>;
#[doc = "SPI hold register"]
pub mod hold;
#[doc = "DMA_CONF (rw) register accessor: an alias for `Reg<DMA_CONF_SPEC>`"]
pub type DMA_CONF = crate::Reg<dma_conf::DMA_CONF_SPEC>;
#[doc = "SPI DMA control register"]
pub mod dma_conf;
#[doc = "DMA_OUT_LINK (rw) register accessor: an alias for `Reg<DMA_OUT_LINK_SPEC>`"]
pub type DMA_OUT_LINK = crate::Reg<dma_out_link::DMA_OUT_LINK_SPEC>;
#[doc = "SPI DMA TX link configuration"]
pub mod dma_out_link;
#[doc = "DMA_IN_LINK (rw) register accessor: an alias for `Reg<DMA_IN_LINK_SPEC>`"]
pub type DMA_IN_LINK = crate::Reg<dma_in_link::DMA_IN_LINK_SPEC>;
#[doc = "SPI DMA RX link configuration"]
pub mod dma_in_link;
#[doc = "DMA_INT_ENA (rw) register accessor: an alias for `Reg<DMA_INT_ENA_SPEC>`"]
pub type DMA_INT_ENA = crate::Reg<dma_int_ena::DMA_INT_ENA_SPEC>;
#[doc = "SPI DMA interrupt enable register"]
pub mod dma_int_ena;
#[doc = "DMA_INT_RAW (rw) register accessor: an alias for `Reg<DMA_INT_RAW_SPEC>`"]
pub type DMA_INT_RAW = crate::Reg<dma_int_raw::DMA_INT_RAW_SPEC>;
#[doc = "SPI DMA interrupt raw register"]
pub mod dma_int_raw;
#[doc = "DMA_INT_ST (rw) register accessor: an alias for `Reg<DMA_INT_ST_SPEC>`"]
pub type DMA_INT_ST = crate::Reg<dma_int_st::DMA_INT_ST_SPEC>;
#[doc = "SPI DMA interrupt status register"]
pub mod dma_int_st;
#[doc = "DMA_INT_CLR (rw) register accessor: an alias for `Reg<DMA_INT_CLR_SPEC>`"]
pub type DMA_INT_CLR = crate::Reg<dma_int_clr::DMA_INT_CLR_SPEC>;
#[doc = "SPI DMA interrupt clear register"]
pub mod dma_int_clr;
#[doc = "IN_ERR_EOF_DES_ADDR (r) register accessor: an alias for `Reg<IN_ERR_EOF_DES_ADDR_SPEC>`"]
pub type IN_ERR_EOF_DES_ADDR = crate::Reg<in_err_eof_des_addr::IN_ERR_EOF_DES_ADDR_SPEC>;
#[doc = "The latest SPI DMA RX descriptor address receiving error"]
pub mod in_err_eof_des_addr;
#[doc = "IN_SUC_EOF_DES_ADDR (r) register accessor: an alias for `Reg<IN_SUC_EOF_DES_ADDR_SPEC>`"]
pub type IN_SUC_EOF_DES_ADDR = crate::Reg<in_suc_eof_des_addr::IN_SUC_EOF_DES_ADDR_SPEC>;
#[doc = "The latest SPI DMA eof RX descriptor address"]
pub mod in_suc_eof_des_addr;
#[doc = "INLINK_DSCR (r) register accessor: an alias for `Reg<INLINK_DSCR_SPEC>`"]
pub type INLINK_DSCR = crate::Reg<inlink_dscr::INLINK_DSCR_SPEC>;
#[doc = "Current SPI DMA RX descriptor pointer"]
pub mod inlink_dscr;
#[doc = "INLINK_DSCR_BF0 (r) register accessor: an alias for `Reg<INLINK_DSCR_BF0_SPEC>`"]
pub type INLINK_DSCR_BF0 = crate::Reg<inlink_dscr_bf0::INLINK_DSCR_BF0_SPEC>;
#[doc = "Next SPI DMA RX descriptor pointer"]
pub mod inlink_dscr_bf0;
#[doc = "INLINK_DSCR_BF1 (r) register accessor: an alias for `Reg<INLINK_DSCR_BF1_SPEC>`"]
pub type INLINK_DSCR_BF1 = crate::Reg<inlink_dscr_bf1::INLINK_DSCR_BF1_SPEC>;
#[doc = "Current SPI DMA RX buffer pointer"]
pub mod inlink_dscr_bf1;
#[doc = "OUT_EOF_BFR_DES_ADDR (r) register accessor: an alias for `Reg<OUT_EOF_BFR_DES_ADDR_SPEC>`"]
pub type OUT_EOF_BFR_DES_ADDR = crate::Reg<out_eof_bfr_des_addr::OUT_EOF_BFR_DES_ADDR_SPEC>;
#[doc = "The latest SPI DMA eof TX buffer address"]
pub mod out_eof_bfr_des_addr;
#[doc = "OUT_EOF_DES_ADDR (r) register accessor: an alias for `Reg<OUT_EOF_DES_ADDR_SPEC>`"]
pub type OUT_EOF_DES_ADDR = crate::Reg<out_eof_des_addr::OUT_EOF_DES_ADDR_SPEC>;
#[doc = "The latest SPI DMA eof TX descriptor address"]
pub mod out_eof_des_addr;
#[doc = "OUTLINK_DSCR (r) register accessor: an alias for `Reg<OUTLINK_DSCR_SPEC>`"]
pub type OUTLINK_DSCR = crate::Reg<outlink_dscr::OUTLINK_DSCR_SPEC>;
#[doc = "Current SPI DMA TX descriptor pointer"]
pub mod outlink_dscr;
#[doc = "OUTLINK_DSCR_BF0 (r) register accessor: an alias for `Reg<OUTLINK_DSCR_BF0_SPEC>`"]
pub type OUTLINK_DSCR_BF0 = crate::Reg<outlink_dscr_bf0::OUTLINK_DSCR_BF0_SPEC>;
#[doc = "Next SPI DMA TX descriptor pointer"]
pub mod outlink_dscr_bf0;
#[doc = "OUTLINK_DSCR_BF1 (r) register accessor: an alias for `Reg<OUTLINK_DSCR_BF1_SPEC>`"]
pub type OUTLINK_DSCR_BF1 = crate::Reg<outlink_dscr_bf1::OUTLINK_DSCR_BF1_SPEC>;
#[doc = "Current SPI DMA TX buffer pointer"]
pub mod outlink_dscr_bf1;
#[doc = "DMA_OUTSTATUS (r) register accessor: an alias for `Reg<DMA_OUTSTATUS_SPEC>`"]
pub type DMA_OUTSTATUS = crate::Reg<dma_outstatus::DMA_OUTSTATUS_SPEC>;
#[doc = "SPI DMA TX status"]
pub mod dma_outstatus;
#[doc = "DMA_INSTATUS (r) register accessor: an alias for `Reg<DMA_INSTATUS_SPEC>`"]
pub type DMA_INSTATUS = crate::Reg<dma_instatus::DMA_INSTATUS_SPEC>;
#[doc = "SPI DMA RX status"]
pub mod dma_instatus;
#[doc = "W0 (rw) register accessor: an alias for `Reg<W0_SPEC>`"]
pub type W0 = crate::Reg<w0::W0_SPEC>;
#[doc = "Data buffer 0"]
pub mod w0;
#[doc = "W1 (rw) register accessor: an alias for `Reg<W1_SPEC>`"]
pub type W1 = crate::Reg<w1::W1_SPEC>;
#[doc = "Data buffer 1"]
pub mod w1;
#[doc = "W2 (rw) register accessor: an alias for `Reg<W2_SPEC>`"]
pub type W2 = crate::Reg<w2::W2_SPEC>;
#[doc = "Data buffer 2"]
pub mod w2;
#[doc = "W3 (rw) register accessor: an alias for `Reg<W3_SPEC>`"]
pub type W3 = crate::Reg<w3::W3_SPEC>;
#[doc = "Data buffer 3"]
pub mod w3;
#[doc = "W4 (rw) register accessor: an alias for `Reg<W4_SPEC>`"]
pub type W4 = crate::Reg<w4::W4_SPEC>;
#[doc = "Data buffer 4"]
pub mod w4;
#[doc = "W5 (rw) register accessor: an alias for `Reg<W5_SPEC>`"]
pub type W5 = crate::Reg<w5::W5_SPEC>;
#[doc = "Data buffer 5"]
pub mod w5;
#[doc = "W6 (rw) register accessor: an alias for `Reg<W6_SPEC>`"]
pub type W6 = crate::Reg<w6::W6_SPEC>;
#[doc = "Data buffer 6"]
pub mod w6;
#[doc = "W7 (rw) register accessor: an alias for `Reg<W7_SPEC>`"]
pub type W7 = crate::Reg<w7::W7_SPEC>;
#[doc = "Data buffer 7"]
pub mod w7;
#[doc = "W8 (rw) register accessor: an alias for `Reg<W8_SPEC>`"]
pub type W8 = crate::Reg<w8::W8_SPEC>;
#[doc = "Data buffer 8"]
pub mod w8;
#[doc = "W9 (rw) register accessor: an alias for `Reg<W9_SPEC>`"]
pub type W9 = crate::Reg<w9::W9_SPEC>;
#[doc = "Data buffer 9"]
pub mod w9;
#[doc = "W10 (rw) register accessor: an alias for `Reg<W10_SPEC>`"]
pub type W10 = crate::Reg<w10::W10_SPEC>;
#[doc = "Data buffer 10"]
pub mod w10;
#[doc = "W11 (rw) register accessor: an alias for `Reg<W11_SPEC>`"]
pub type W11 = crate::Reg<w11::W11_SPEC>;
#[doc = "Data buffer 11"]
pub mod w11;
#[doc = "W12 (rw) register accessor: an alias for `Reg<W12_SPEC>`"]
pub type W12 = crate::Reg<w12::W12_SPEC>;
#[doc = "Data buffer 12"]
pub mod w12;
#[doc = "W13 (rw) register accessor: an alias for `Reg<W13_SPEC>`"]
pub type W13 = crate::Reg<w13::W13_SPEC>;
#[doc = "Data buffer 13"]
pub mod w13;
#[doc = "W14 (rw) register accessor: an alias for `Reg<W14_SPEC>`"]
pub type W14 = crate::Reg<w14::W14_SPEC>;
#[doc = "Data buffer 14"]
pub mod w14;
#[doc = "W15 (rw) register accessor: an alias for `Reg<W15_SPEC>`"]
pub type W15 = crate::Reg<w15::W15_SPEC>;
#[doc = "Data buffer 15"]
pub mod w15;
#[doc = "W16 (rw) register accessor: an alias for `Reg<W16_SPEC>`"]
pub type W16 = crate::Reg<w16::W16_SPEC>;
#[doc = "Data buffer 16"]
pub mod w16;
#[doc = "W17 (rw) register accessor: an alias for `Reg<W17_SPEC>`"]
pub type W17 = crate::Reg<w17::W17_SPEC>;
#[doc = "Data buffer 17"]
pub mod w17;
#[doc = "DIN_MODE (rw) register accessor: an alias for `Reg<DIN_MODE_SPEC>`"]
pub type DIN_MODE = crate::Reg<din_mode::DIN_MODE_SPEC>;
#[doc = "SPI input delay mode configuration"]
pub mod din_mode;
#[doc = "DIN_NUM (rw) register accessor: an alias for `Reg<DIN_NUM_SPEC>`"]
pub type DIN_NUM = crate::Reg<din_num::DIN_NUM_SPEC>;
#[doc = "SPI input delay number configuration"]
pub mod din_num;
#[doc = "DOUT_MODE (rw) register accessor: an alias for `Reg<DOUT_MODE_SPEC>`"]
pub type DOUT_MODE = crate::Reg<dout_mode::DOUT_MODE_SPEC>;
#[doc = "SPI output delay mode configuration"]
pub mod dout_mode;
#[doc = "DOUT_NUM (rw) register accessor: an alias for `Reg<DOUT_NUM_SPEC>`"]
pub type DOUT_NUM = crate::Reg<dout_num::DOUT_NUM_SPEC>;
#[doc = "SPI output delay number configuration"]
pub mod dout_num;
#[doc = "LCD_CTRL (rw) register accessor: an alias for `Reg<LCD_CTRL_SPEC>`"]
pub type LCD_CTRL = crate::Reg<lcd_ctrl::LCD_CTRL_SPEC>;
#[doc = "LCD frame control register"]
pub mod lcd_ctrl;
#[doc = "LCD_CTRL1 (rw) register accessor: an alias for `Reg<LCD_CTRL1_SPEC>`"]
pub type LCD_CTRL1 = crate::Reg<lcd_ctrl1::LCD_CTRL1_SPEC>;
#[doc = "LCD frame control1 register"]
pub mod lcd_ctrl1;
#[doc = "LCD_CTRL2 (rw) register accessor: an alias for `Reg<LCD_CTRL2_SPEC>`"]
pub type LCD_CTRL2 = crate::Reg<lcd_ctrl2::LCD_CTRL2_SPEC>;
#[doc = "LCD frame control2 register"]
pub mod lcd_ctrl2;
#[doc = "LCD_D_MODE (rw) register accessor: an alias for `Reg<LCD_D_MODE_SPEC>`"]
pub type LCD_D_MODE = crate::Reg<lcd_d_mode::LCD_D_MODE_SPEC>;
#[doc = "LCD delay number"]
pub mod lcd_d_mode;
#[doc = "LCD_D_NUM (rw) register accessor: an alias for `Reg<LCD_D_NUM_SPEC>`"]
pub type LCD_D_NUM = crate::Reg<lcd_d_num::LCD_D_NUM_SPEC>;
#[doc = "LCD delay mode"]
pub mod lcd_d_num;
#[doc = "REG_DATE (rw) register accessor: an alias for `Reg<REG_DATE_SPEC>`"]
pub type REG_DATE = crate::Reg<reg_date::REG_DATE_SPEC>;
#[doc = "SPI version control"]
pub mod reg_date;
