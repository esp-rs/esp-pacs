#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub conf0: CONF0,
    #[doc = "0x04 - "]
    pub _0int_raw: _0INT_RAW,
    #[doc = "0x08 - "]
    pub _0int_st: _0INT_ST,
    #[doc = "0x0c - "]
    pub _0int_ena: _0INT_ENA,
    #[doc = "0x10 - "]
    pub _0int_clr: _0INT_CLR,
    #[doc = "0x14 - "]
    pub _1int_raw: _1INT_RAW,
    #[doc = "0x18 - "]
    pub _1int_st: _1INT_ST,
    #[doc = "0x1c - "]
    pub _1int_ena: _1INT_ENA,
    #[doc = "0x20 - "]
    pub _1int_clr: _1INT_CLR,
    #[doc = "0x24 - "]
    pub rx_status: RX_STATUS,
    #[doc = "0x28 - "]
    pub _0rxfifo_push: _0RXFIFO_PUSH,
    #[doc = "0x2c - "]
    pub _1rxfifo_push: _1RXFIFO_PUSH,
    #[doc = "0x30 - "]
    pub tx_status: TX_STATUS,
    #[doc = "0x34 - "]
    pub _0txfifo_pop: _0TXFIFO_POP,
    #[doc = "0x38 - "]
    pub _1txfifo_pop: _1TXFIFO_POP,
    #[doc = "0x3c - "]
    pub _0rx_link: _0RX_LINK,
    #[doc = "0x40 - "]
    pub _0tx_link: _0TX_LINK,
    #[doc = "0x44 - "]
    pub _1rx_link: _1RX_LINK,
    #[doc = "0x48 - "]
    pub _1tx_link: _1TX_LINK,
    #[doc = "0x4c - "]
    pub intvec_tohost: INTVEC_TOHOST,
    #[doc = "0x50 - "]
    pub _0token0: _0TOKEN0,
    #[doc = "0x54 - "]
    pub _0token1: _0TOKEN1,
    #[doc = "0x58 - "]
    pub _1token0: _1TOKEN0,
    #[doc = "0x5c - "]
    pub _1token1: _1TOKEN1,
    #[doc = "0x60 - "]
    pub conf1: CONF1,
    #[doc = "0x64 - "]
    pub _0_state0: _0_STATE0,
    #[doc = "0x68 - "]
    pub _0_state1: _0_STATE1,
    #[doc = "0x6c - "]
    pub _1_state0: _1_STATE0,
    #[doc = "0x70 - "]
    pub _1_state1: _1_STATE1,
    #[doc = "0x74 - "]
    pub bridge_conf: BRIDGE_CONF,
    #[doc = "0x78 - "]
    pub _0_to_eof_des_addr: _0_TO_EOF_DES_ADDR,
    #[doc = "0x7c - "]
    pub _0_tx_eof_des_addr: _0_TX_EOF_DES_ADDR,
    #[doc = "0x80 - "]
    pub _0_to_eof_bfr_des_addr: _0_TO_EOF_BFR_DES_ADDR,
    #[doc = "0x84 - "]
    pub _1_to_eof_des_addr: _1_TO_EOF_DES_ADDR,
    #[doc = "0x88 - "]
    pub _1_tx_eof_des_addr: _1_TX_EOF_DES_ADDR,
    #[doc = "0x8c - "]
    pub _1_to_eof_bfr_des_addr: _1_TO_EOF_BFR_DES_ADDR,
    #[doc = "0x90 - "]
    pub ahb_test: AHB_TEST,
    #[doc = "0x94 - "]
    pub sdio_st: SDIO_ST,
    #[doc = "0x98 - "]
    pub rx_dscr_conf: RX_DSCR_CONF,
    #[doc = "0x9c - "]
    pub _0_txlink_dscr: _0_TXLINK_DSCR,
    #[doc = "0xa0 - "]
    pub _0_txlink_dscr_bf0: _0_TXLINK_DSCR_BF0,
    #[doc = "0xa4 - "]
    pub _0_txlink_dscr_bf1: _0_TXLINK_DSCR_BF1,
    #[doc = "0xa8 - "]
    pub _0_rxlink_dscr: _0_RXLINK_DSCR,
    #[doc = "0xac - "]
    pub _0_rxlink_dscr_bf0: _0_RXLINK_DSCR_BF0,
    #[doc = "0xb0 - "]
    pub _0_rxlink_dscr_bf1: _0_RXLINK_DSCR_BF1,
    #[doc = "0xb4 - "]
    pub _1_txlink_dscr: _1_TXLINK_DSCR,
    #[doc = "0xb8 - "]
    pub _1_txlink_dscr_bf0: _1_TXLINK_DSCR_BF0,
    #[doc = "0xbc - "]
    pub _1_txlink_dscr_bf1: _1_TXLINK_DSCR_BF1,
    #[doc = "0xc0 - "]
    pub _1_rxlink_dscr: _1_RXLINK_DSCR,
    #[doc = "0xc4 - "]
    pub _1_rxlink_dscr_bf0: _1_RXLINK_DSCR_BF0,
    #[doc = "0xc8 - "]
    pub _1_rxlink_dscr_bf1: _1_RXLINK_DSCR_BF1,
    #[doc = "0xcc - "]
    pub _0_tx_erreof_des_addr: _0_TX_ERREOF_DES_ADDR,
    #[doc = "0xd0 - "]
    pub _1_tx_erreof_des_addr: _1_TX_ERREOF_DES_ADDR,
    #[doc = "0xd4 - "]
    pub token_lat: TOKEN_LAT,
    #[doc = "0xd8 - "]
    pub tx_dscr_conf: TX_DSCR_CONF,
    #[doc = "0xdc - "]
    pub cmd_infor0: CMD_INFOR0,
    #[doc = "0xe0 - "]
    pub cmd_infor1: CMD_INFOR1,
    #[doc = "0xe4 - "]
    pub _0_len_conf: _0_LEN_CONF,
    #[doc = "0xe8 - "]
    pub _0_length: _0_LENGTH,
    #[doc = "0xec - "]
    pub _0_txpkt_h_dscr: _0_TXPKT_H_DSCR,
    #[doc = "0xf0 - "]
    pub _0_txpkt_e_dscr: _0_TXPKT_E_DSCR,
    #[doc = "0xf4 - "]
    pub _0_rxpkt_h_dscr: _0_RXPKT_H_DSCR,
    #[doc = "0xf8 - "]
    pub _0_rxpkt_e_dscr: _0_RXPKT_E_DSCR,
    #[doc = "0xfc - "]
    pub _0_txpktu_h_dscr: _0_TXPKTU_H_DSCR,
    #[doc = "0x100 - "]
    pub _0_txpktu_e_dscr: _0_TXPKTU_E_DSCR,
    #[doc = "0x104 - "]
    pub _0_rxpktu_h_dscr: _0_RXPKTU_H_DSCR,
    #[doc = "0x108 - "]
    pub _0_rxpktu_e_dscr: _0_RXPKTU_E_DSCR,
    _reserved67: [u8; 0x08],
    #[doc = "0x114 - "]
    pub seq_position: SEQ_POSITION,
    #[doc = "0x118 - "]
    pub _0_dscr_rec_conf: _0_DSCR_REC_CONF,
    #[doc = "0x11c - "]
    pub sdio_crc_st0: SDIO_CRC_ST0,
    #[doc = "0x120 - "]
    pub sdio_crc_st1: SDIO_CRC_ST1,
    #[doc = "0x124 - "]
    pub _0_eof_start_des: _0_EOF_START_DES,
    #[doc = "0x128 - "]
    pub _0_push_dscr_addr: _0_PUSH_DSCR_ADDR,
    #[doc = "0x12c - "]
    pub _0_done_dscr_addr: _0_DONE_DSCR_ADDR,
    #[doc = "0x130 - "]
    pub _0_sub_start_des: _0_SUB_START_DES,
    #[doc = "0x134 - "]
    pub _0_dscr_cnt: _0_DSCR_CNT,
    #[doc = "0x138 - "]
    pub _0_len_lim_conf: _0_LEN_LIM_CONF,
    #[doc = "0x13c - "]
    pub _0int_st1: _0INT_ST1,
    #[doc = "0x140 - "]
    pub _0int_ena1: _0INT_ENA1,
    #[doc = "0x144 - "]
    pub _1int_st1: _1INT_ST1,
    #[doc = "0x148 - "]
    pub _1int_ena1: _1INT_ENA1,
    _reserved81: [u8; 0xac],
    #[doc = "0x1f8 - "]
    pub date: DATE,
    #[doc = "0x1fc - "]
    pub id: ID,
}
#[doc = "CONF0 (rw) register accessor: an alias for `Reg<CONF0_SPEC>`"]
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
#[doc = ""]
pub mod conf0;
#[doc = "_0INT_RAW (r) register accessor: an alias for `Reg<_0INT_RAW_SPEC>`"]
pub type _0INT_RAW = crate::Reg<_0int_raw::_0INT_RAW_SPEC>;
#[doc = ""]
pub mod _0int_raw;
#[doc = "_0INT_ST (r) register accessor: an alias for `Reg<_0INT_ST_SPEC>`"]
pub type _0INT_ST = crate::Reg<_0int_st::_0INT_ST_SPEC>;
#[doc = ""]
pub mod _0int_st;
#[doc = "_0INT_ENA (rw) register accessor: an alias for `Reg<_0INT_ENA_SPEC>`"]
pub type _0INT_ENA = crate::Reg<_0int_ena::_0INT_ENA_SPEC>;
#[doc = ""]
pub mod _0int_ena;
#[doc = "_0INT_CLR (w) register accessor: an alias for `Reg<_0INT_CLR_SPEC>`"]
pub type _0INT_CLR = crate::Reg<_0int_clr::_0INT_CLR_SPEC>;
#[doc = ""]
pub mod _0int_clr;
#[doc = "_1INT_RAW (r) register accessor: an alias for `Reg<_1INT_RAW_SPEC>`"]
pub type _1INT_RAW = crate::Reg<_1int_raw::_1INT_RAW_SPEC>;
#[doc = ""]
pub mod _1int_raw;
#[doc = "_1INT_ST (r) register accessor: an alias for `Reg<_1INT_ST_SPEC>`"]
pub type _1INT_ST = crate::Reg<_1int_st::_1INT_ST_SPEC>;
#[doc = ""]
pub mod _1int_st;
#[doc = "_1INT_ENA (rw) register accessor: an alias for `Reg<_1INT_ENA_SPEC>`"]
pub type _1INT_ENA = crate::Reg<_1int_ena::_1INT_ENA_SPEC>;
#[doc = ""]
pub mod _1int_ena;
#[doc = "_1INT_CLR (w) register accessor: an alias for `Reg<_1INT_CLR_SPEC>`"]
pub type _1INT_CLR = crate::Reg<_1int_clr::_1INT_CLR_SPEC>;
#[doc = ""]
pub mod _1int_clr;
#[doc = "RX_STATUS (r) register accessor: an alias for `Reg<RX_STATUS_SPEC>`"]
pub type RX_STATUS = crate::Reg<rx_status::RX_STATUS_SPEC>;
#[doc = ""]
pub mod rx_status;
#[doc = "_0RXFIFO_PUSH (rw) register accessor: an alias for `Reg<_0RXFIFO_PUSH_SPEC>`"]
pub type _0RXFIFO_PUSH = crate::Reg<_0rxfifo_push::_0RXFIFO_PUSH_SPEC>;
#[doc = ""]
pub mod _0rxfifo_push;
#[doc = "_1RXFIFO_PUSH (rw) register accessor: an alias for `Reg<_1RXFIFO_PUSH_SPEC>`"]
pub type _1RXFIFO_PUSH = crate::Reg<_1rxfifo_push::_1RXFIFO_PUSH_SPEC>;
#[doc = ""]
pub mod _1rxfifo_push;
#[doc = "TX_STATUS (r) register accessor: an alias for `Reg<TX_STATUS_SPEC>`"]
pub type TX_STATUS = crate::Reg<tx_status::TX_STATUS_SPEC>;
#[doc = ""]
pub mod tx_status;
#[doc = "_0TXFIFO_POP (rw) register accessor: an alias for `Reg<_0TXFIFO_POP_SPEC>`"]
pub type _0TXFIFO_POP = crate::Reg<_0txfifo_pop::_0TXFIFO_POP_SPEC>;
#[doc = ""]
pub mod _0txfifo_pop;
#[doc = "_1TXFIFO_POP (rw) register accessor: an alias for `Reg<_1TXFIFO_POP_SPEC>`"]
pub type _1TXFIFO_POP = crate::Reg<_1txfifo_pop::_1TXFIFO_POP_SPEC>;
#[doc = ""]
pub mod _1txfifo_pop;
#[doc = "_0RX_LINK (rw) register accessor: an alias for `Reg<_0RX_LINK_SPEC>`"]
pub type _0RX_LINK = crate::Reg<_0rx_link::_0RX_LINK_SPEC>;
#[doc = ""]
pub mod _0rx_link;
#[doc = "_0TX_LINK (rw) register accessor: an alias for `Reg<_0TX_LINK_SPEC>`"]
pub type _0TX_LINK = crate::Reg<_0tx_link::_0TX_LINK_SPEC>;
#[doc = ""]
pub mod _0tx_link;
#[doc = "_1RX_LINK (rw) register accessor: an alias for `Reg<_1RX_LINK_SPEC>`"]
pub type _1RX_LINK = crate::Reg<_1rx_link::_1RX_LINK_SPEC>;
#[doc = ""]
pub mod _1rx_link;
#[doc = "_1TX_LINK (rw) register accessor: an alias for `Reg<_1TX_LINK_SPEC>`"]
pub type _1TX_LINK = crate::Reg<_1tx_link::_1TX_LINK_SPEC>;
#[doc = ""]
pub mod _1tx_link;
#[doc = "INTVEC_TOHOST (w) register accessor: an alias for `Reg<INTVEC_TOHOST_SPEC>`"]
pub type INTVEC_TOHOST = crate::Reg<intvec_tohost::INTVEC_TOHOST_SPEC>;
#[doc = ""]
pub mod intvec_tohost;
#[doc = "_0TOKEN0 (rw) register accessor: an alias for `Reg<_0TOKEN0_SPEC>`"]
pub type _0TOKEN0 = crate::Reg<_0token0::_0TOKEN0_SPEC>;
#[doc = ""]
pub mod _0token0;
#[doc = "_0TOKEN1 (rw) register accessor: an alias for `Reg<_0TOKEN1_SPEC>`"]
pub type _0TOKEN1 = crate::Reg<_0token1::_0TOKEN1_SPEC>;
#[doc = ""]
pub mod _0token1;
#[doc = "_1TOKEN0 (rw) register accessor: an alias for `Reg<_1TOKEN0_SPEC>`"]
pub type _1TOKEN0 = crate::Reg<_1token0::_1TOKEN0_SPEC>;
#[doc = ""]
pub mod _1token0;
#[doc = "_1TOKEN1 (rw) register accessor: an alias for `Reg<_1TOKEN1_SPEC>`"]
pub type _1TOKEN1 = crate::Reg<_1token1::_1TOKEN1_SPEC>;
#[doc = ""]
pub mod _1token1;
#[doc = "CONF1 (rw) register accessor: an alias for `Reg<CONF1_SPEC>`"]
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
#[doc = ""]
pub mod conf1;
#[doc = "_0_STATE0 (r) register accessor: an alias for `Reg<_0_STATE0_SPEC>`"]
pub type _0_STATE0 = crate::Reg<_0_state0::_0_STATE0_SPEC>;
#[doc = ""]
pub mod _0_state0;
#[doc = "_0_STATE1 (r) register accessor: an alias for `Reg<_0_STATE1_SPEC>`"]
pub type _0_STATE1 = crate::Reg<_0_state1::_0_STATE1_SPEC>;
#[doc = ""]
pub mod _0_state1;
#[doc = "_1_STATE0 (r) register accessor: an alias for `Reg<_1_STATE0_SPEC>`"]
pub type _1_STATE0 = crate::Reg<_1_state0::_1_STATE0_SPEC>;
#[doc = ""]
pub mod _1_state0;
#[doc = "_1_STATE1 (r) register accessor: an alias for `Reg<_1_STATE1_SPEC>`"]
pub type _1_STATE1 = crate::Reg<_1_state1::_1_STATE1_SPEC>;
#[doc = ""]
pub mod _1_state1;
#[doc = "BRIDGE_CONF (rw) register accessor: an alias for `Reg<BRIDGE_CONF_SPEC>`"]
pub type BRIDGE_CONF = crate::Reg<bridge_conf::BRIDGE_CONF_SPEC>;
#[doc = ""]
pub mod bridge_conf;
#[doc = "_0_TO_EOF_DES_ADDR (r) register accessor: an alias for `Reg<_0_TO_EOF_DES_ADDR_SPEC>`"]
pub type _0_TO_EOF_DES_ADDR = crate::Reg<_0_to_eof_des_addr::_0_TO_EOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod _0_to_eof_des_addr;
#[doc = "_0_TX_EOF_DES_ADDR (r) register accessor: an alias for `Reg<_0_TX_EOF_DES_ADDR_SPEC>`"]
pub type _0_TX_EOF_DES_ADDR = crate::Reg<_0_tx_eof_des_addr::_0_TX_EOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod _0_tx_eof_des_addr;
#[doc = "_0_TO_EOF_BFR_DES_ADDR (r) register accessor: an alias for `Reg<_0_TO_EOF_BFR_DES_ADDR_SPEC>`"]
pub type _0_TO_EOF_BFR_DES_ADDR = crate::Reg<_0_to_eof_bfr_des_addr::_0_TO_EOF_BFR_DES_ADDR_SPEC>;
#[doc = ""]
pub mod _0_to_eof_bfr_des_addr;
#[doc = "_1_TO_EOF_DES_ADDR (r) register accessor: an alias for `Reg<_1_TO_EOF_DES_ADDR_SPEC>`"]
pub type _1_TO_EOF_DES_ADDR = crate::Reg<_1_to_eof_des_addr::_1_TO_EOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod _1_to_eof_des_addr;
#[doc = "_1_TX_EOF_DES_ADDR (r) register accessor: an alias for `Reg<_1_TX_EOF_DES_ADDR_SPEC>`"]
pub type _1_TX_EOF_DES_ADDR = crate::Reg<_1_tx_eof_des_addr::_1_TX_EOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod _1_tx_eof_des_addr;
#[doc = "_1_TO_EOF_BFR_DES_ADDR (r) register accessor: an alias for `Reg<_1_TO_EOF_BFR_DES_ADDR_SPEC>`"]
pub type _1_TO_EOF_BFR_DES_ADDR = crate::Reg<_1_to_eof_bfr_des_addr::_1_TO_EOF_BFR_DES_ADDR_SPEC>;
#[doc = ""]
pub mod _1_to_eof_bfr_des_addr;
#[doc = "AHB_TEST (rw) register accessor: an alias for `Reg<AHB_TEST_SPEC>`"]
pub type AHB_TEST = crate::Reg<ahb_test::AHB_TEST_SPEC>;
#[doc = ""]
pub mod ahb_test;
#[doc = "SDIO_ST (r) register accessor: an alias for `Reg<SDIO_ST_SPEC>`"]
pub type SDIO_ST = crate::Reg<sdio_st::SDIO_ST_SPEC>;
#[doc = ""]
pub mod sdio_st;
#[doc = "RX_DSCR_CONF (rw) register accessor: an alias for `Reg<RX_DSCR_CONF_SPEC>`"]
pub type RX_DSCR_CONF = crate::Reg<rx_dscr_conf::RX_DSCR_CONF_SPEC>;
#[doc = ""]
pub mod rx_dscr_conf;
#[doc = "_0_TXLINK_DSCR (r) register accessor: an alias for `Reg<_0_TXLINK_DSCR_SPEC>`"]
pub type _0_TXLINK_DSCR = crate::Reg<_0_txlink_dscr::_0_TXLINK_DSCR_SPEC>;
#[doc = ""]
pub mod _0_txlink_dscr;
#[doc = "_0_TXLINK_DSCR_BF0 (r) register accessor: an alias for `Reg<_0_TXLINK_DSCR_BF0_SPEC>`"]
pub type _0_TXLINK_DSCR_BF0 = crate::Reg<_0_txlink_dscr_bf0::_0_TXLINK_DSCR_BF0_SPEC>;
#[doc = ""]
pub mod _0_txlink_dscr_bf0;
#[doc = "_0_TXLINK_DSCR_BF1 (r) register accessor: an alias for `Reg<_0_TXLINK_DSCR_BF1_SPEC>`"]
pub type _0_TXLINK_DSCR_BF1 = crate::Reg<_0_txlink_dscr_bf1::_0_TXLINK_DSCR_BF1_SPEC>;
#[doc = ""]
pub mod _0_txlink_dscr_bf1;
#[doc = "_0_RXLINK_DSCR (r) register accessor: an alias for `Reg<_0_RXLINK_DSCR_SPEC>`"]
pub type _0_RXLINK_DSCR = crate::Reg<_0_rxlink_dscr::_0_RXLINK_DSCR_SPEC>;
#[doc = ""]
pub mod _0_rxlink_dscr;
#[doc = "_0_RXLINK_DSCR_BF0 (r) register accessor: an alias for `Reg<_0_RXLINK_DSCR_BF0_SPEC>`"]
pub type _0_RXLINK_DSCR_BF0 = crate::Reg<_0_rxlink_dscr_bf0::_0_RXLINK_DSCR_BF0_SPEC>;
#[doc = ""]
pub mod _0_rxlink_dscr_bf0;
#[doc = "_0_RXLINK_DSCR_BF1 (r) register accessor: an alias for `Reg<_0_RXLINK_DSCR_BF1_SPEC>`"]
pub type _0_RXLINK_DSCR_BF1 = crate::Reg<_0_rxlink_dscr_bf1::_0_RXLINK_DSCR_BF1_SPEC>;
#[doc = ""]
pub mod _0_rxlink_dscr_bf1;
#[doc = "_1_TXLINK_DSCR (r) register accessor: an alias for `Reg<_1_TXLINK_DSCR_SPEC>`"]
pub type _1_TXLINK_DSCR = crate::Reg<_1_txlink_dscr::_1_TXLINK_DSCR_SPEC>;
#[doc = ""]
pub mod _1_txlink_dscr;
#[doc = "_1_TXLINK_DSCR_BF0 (r) register accessor: an alias for `Reg<_1_TXLINK_DSCR_BF0_SPEC>`"]
pub type _1_TXLINK_DSCR_BF0 = crate::Reg<_1_txlink_dscr_bf0::_1_TXLINK_DSCR_BF0_SPEC>;
#[doc = ""]
pub mod _1_txlink_dscr_bf0;
#[doc = "_1_TXLINK_DSCR_BF1 (r) register accessor: an alias for `Reg<_1_TXLINK_DSCR_BF1_SPEC>`"]
pub type _1_TXLINK_DSCR_BF1 = crate::Reg<_1_txlink_dscr_bf1::_1_TXLINK_DSCR_BF1_SPEC>;
#[doc = ""]
pub mod _1_txlink_dscr_bf1;
#[doc = "_1_RXLINK_DSCR (r) register accessor: an alias for `Reg<_1_RXLINK_DSCR_SPEC>`"]
pub type _1_RXLINK_DSCR = crate::Reg<_1_rxlink_dscr::_1_RXLINK_DSCR_SPEC>;
#[doc = ""]
pub mod _1_rxlink_dscr;
#[doc = "_1_RXLINK_DSCR_BF0 (r) register accessor: an alias for `Reg<_1_RXLINK_DSCR_BF0_SPEC>`"]
pub type _1_RXLINK_DSCR_BF0 = crate::Reg<_1_rxlink_dscr_bf0::_1_RXLINK_DSCR_BF0_SPEC>;
#[doc = ""]
pub mod _1_rxlink_dscr_bf0;
#[doc = "_1_RXLINK_DSCR_BF1 (r) register accessor: an alias for `Reg<_1_RXLINK_DSCR_BF1_SPEC>`"]
pub type _1_RXLINK_DSCR_BF1 = crate::Reg<_1_rxlink_dscr_bf1::_1_RXLINK_DSCR_BF1_SPEC>;
#[doc = ""]
pub mod _1_rxlink_dscr_bf1;
#[doc = "_0_TX_ERREOF_DES_ADDR (r) register accessor: an alias for `Reg<_0_TX_ERREOF_DES_ADDR_SPEC>`"]
pub type _0_TX_ERREOF_DES_ADDR = crate::Reg<_0_tx_erreof_des_addr::_0_TX_ERREOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod _0_tx_erreof_des_addr;
#[doc = "_1_TX_ERREOF_DES_ADDR (r) register accessor: an alias for `Reg<_1_TX_ERREOF_DES_ADDR_SPEC>`"]
pub type _1_TX_ERREOF_DES_ADDR = crate::Reg<_1_tx_erreof_des_addr::_1_TX_ERREOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod _1_tx_erreof_des_addr;
#[doc = "TOKEN_LAT (r) register accessor: an alias for `Reg<TOKEN_LAT_SPEC>`"]
pub type TOKEN_LAT = crate::Reg<token_lat::TOKEN_LAT_SPEC>;
#[doc = ""]
pub mod token_lat;
#[doc = "TX_DSCR_CONF (rw) register accessor: an alias for `Reg<TX_DSCR_CONF_SPEC>`"]
pub type TX_DSCR_CONF = crate::Reg<tx_dscr_conf::TX_DSCR_CONF_SPEC>;
#[doc = ""]
pub mod tx_dscr_conf;
#[doc = "CMD_INFOR0 (r) register accessor: an alias for `Reg<CMD_INFOR0_SPEC>`"]
pub type CMD_INFOR0 = crate::Reg<cmd_infor0::CMD_INFOR0_SPEC>;
#[doc = ""]
pub mod cmd_infor0;
#[doc = "CMD_INFOR1 (r) register accessor: an alias for `Reg<CMD_INFOR1_SPEC>`"]
pub type CMD_INFOR1 = crate::Reg<cmd_infor1::CMD_INFOR1_SPEC>;
#[doc = ""]
pub mod cmd_infor1;
#[doc = "_0_LEN_CONF (rw) register accessor: an alias for `Reg<_0_LEN_CONF_SPEC>`"]
pub type _0_LEN_CONF = crate::Reg<_0_len_conf::_0_LEN_CONF_SPEC>;
#[doc = ""]
pub mod _0_len_conf;
#[doc = "_0_LENGTH (r) register accessor: an alias for `Reg<_0_LENGTH_SPEC>`"]
pub type _0_LENGTH = crate::Reg<_0_length::_0_LENGTH_SPEC>;
#[doc = ""]
pub mod _0_length;
#[doc = "_0_TXPKT_H_DSCR (rw) register accessor: an alias for `Reg<_0_TXPKT_H_DSCR_SPEC>`"]
pub type _0_TXPKT_H_DSCR = crate::Reg<_0_txpkt_h_dscr::_0_TXPKT_H_DSCR_SPEC>;
#[doc = ""]
pub mod _0_txpkt_h_dscr;
#[doc = "_0_TXPKT_E_DSCR (rw) register accessor: an alias for `Reg<_0_TXPKT_E_DSCR_SPEC>`"]
pub type _0_TXPKT_E_DSCR = crate::Reg<_0_txpkt_e_dscr::_0_TXPKT_E_DSCR_SPEC>;
#[doc = ""]
pub mod _0_txpkt_e_dscr;
#[doc = "_0_RXPKT_H_DSCR (rw) register accessor: an alias for `Reg<_0_RXPKT_H_DSCR_SPEC>`"]
pub type _0_RXPKT_H_DSCR = crate::Reg<_0_rxpkt_h_dscr::_0_RXPKT_H_DSCR_SPEC>;
#[doc = ""]
pub mod _0_rxpkt_h_dscr;
#[doc = "_0_RXPKT_E_DSCR (rw) register accessor: an alias for `Reg<_0_RXPKT_E_DSCR_SPEC>`"]
pub type _0_RXPKT_E_DSCR = crate::Reg<_0_rxpkt_e_dscr::_0_RXPKT_E_DSCR_SPEC>;
#[doc = ""]
pub mod _0_rxpkt_e_dscr;
#[doc = "_0_TXPKTU_H_DSCR (r) register accessor: an alias for `Reg<_0_TXPKTU_H_DSCR_SPEC>`"]
pub type _0_TXPKTU_H_DSCR = crate::Reg<_0_txpktu_h_dscr::_0_TXPKTU_H_DSCR_SPEC>;
#[doc = ""]
pub mod _0_txpktu_h_dscr;
#[doc = "_0_TXPKTU_E_DSCR (r) register accessor: an alias for `Reg<_0_TXPKTU_E_DSCR_SPEC>`"]
pub type _0_TXPKTU_E_DSCR = crate::Reg<_0_txpktu_e_dscr::_0_TXPKTU_E_DSCR_SPEC>;
#[doc = ""]
pub mod _0_txpktu_e_dscr;
#[doc = "_0_RXPKTU_H_DSCR (r) register accessor: an alias for `Reg<_0_RXPKTU_H_DSCR_SPEC>`"]
pub type _0_RXPKTU_H_DSCR = crate::Reg<_0_rxpktu_h_dscr::_0_RXPKTU_H_DSCR_SPEC>;
#[doc = ""]
pub mod _0_rxpktu_h_dscr;
#[doc = "_0_RXPKTU_E_DSCR (r) register accessor: an alias for `Reg<_0_RXPKTU_E_DSCR_SPEC>`"]
pub type _0_RXPKTU_E_DSCR = crate::Reg<_0_rxpktu_e_dscr::_0_RXPKTU_E_DSCR_SPEC>;
#[doc = ""]
pub mod _0_rxpktu_e_dscr;
#[doc = "SEQ_POSITION (rw) register accessor: an alias for `Reg<SEQ_POSITION_SPEC>`"]
pub type SEQ_POSITION = crate::Reg<seq_position::SEQ_POSITION_SPEC>;
#[doc = ""]
pub mod seq_position;
#[doc = "_0_DSCR_REC_CONF (rw) register accessor: an alias for `Reg<_0_DSCR_REC_CONF_SPEC>`"]
pub type _0_DSCR_REC_CONF = crate::Reg<_0_dscr_rec_conf::_0_DSCR_REC_CONF_SPEC>;
#[doc = ""]
pub mod _0_dscr_rec_conf;
#[doc = "SDIO_CRC_ST0 (r) register accessor: an alias for `Reg<SDIO_CRC_ST0_SPEC>`"]
pub type SDIO_CRC_ST0 = crate::Reg<sdio_crc_st0::SDIO_CRC_ST0_SPEC>;
#[doc = ""]
pub mod sdio_crc_st0;
#[doc = "SDIO_CRC_ST1 (rw) register accessor: an alias for `Reg<SDIO_CRC_ST1_SPEC>`"]
pub type SDIO_CRC_ST1 = crate::Reg<sdio_crc_st1::SDIO_CRC_ST1_SPEC>;
#[doc = ""]
pub mod sdio_crc_st1;
#[doc = "_0_EOF_START_DES (r) register accessor: an alias for `Reg<_0_EOF_START_DES_SPEC>`"]
pub type _0_EOF_START_DES = crate::Reg<_0_eof_start_des::_0_EOF_START_DES_SPEC>;
#[doc = ""]
pub mod _0_eof_start_des;
#[doc = "_0_PUSH_DSCR_ADDR (r) register accessor: an alias for `Reg<_0_PUSH_DSCR_ADDR_SPEC>`"]
pub type _0_PUSH_DSCR_ADDR = crate::Reg<_0_push_dscr_addr::_0_PUSH_DSCR_ADDR_SPEC>;
#[doc = ""]
pub mod _0_push_dscr_addr;
#[doc = "_0_DONE_DSCR_ADDR (r) register accessor: an alias for `Reg<_0_DONE_DSCR_ADDR_SPEC>`"]
pub type _0_DONE_DSCR_ADDR = crate::Reg<_0_done_dscr_addr::_0_DONE_DSCR_ADDR_SPEC>;
#[doc = ""]
pub mod _0_done_dscr_addr;
#[doc = "_0_SUB_START_DES (r) register accessor: an alias for `Reg<_0_SUB_START_DES_SPEC>`"]
pub type _0_SUB_START_DES = crate::Reg<_0_sub_start_des::_0_SUB_START_DES_SPEC>;
#[doc = ""]
pub mod _0_sub_start_des;
#[doc = "_0_DSCR_CNT (r) register accessor: an alias for `Reg<_0_DSCR_CNT_SPEC>`"]
pub type _0_DSCR_CNT = crate::Reg<_0_dscr_cnt::_0_DSCR_CNT_SPEC>;
#[doc = ""]
pub mod _0_dscr_cnt;
#[doc = "_0_LEN_LIM_CONF (rw) register accessor: an alias for `Reg<_0_LEN_LIM_CONF_SPEC>`"]
pub type _0_LEN_LIM_CONF = crate::Reg<_0_len_lim_conf::_0_LEN_LIM_CONF_SPEC>;
#[doc = ""]
pub mod _0_len_lim_conf;
#[doc = "_0INT_ST1 (r) register accessor: an alias for `Reg<_0INT_ST1_SPEC>`"]
pub type _0INT_ST1 = crate::Reg<_0int_st1::_0INT_ST1_SPEC>;
#[doc = ""]
pub mod _0int_st1;
#[doc = "_0INT_ENA1 (rw) register accessor: an alias for `Reg<_0INT_ENA1_SPEC>`"]
pub type _0INT_ENA1 = crate::Reg<_0int_ena1::_0INT_ENA1_SPEC>;
#[doc = ""]
pub mod _0int_ena1;
#[doc = "_1INT_ST1 (r) register accessor: an alias for `Reg<_1INT_ST1_SPEC>`"]
pub type _1INT_ST1 = crate::Reg<_1int_st1::_1INT_ST1_SPEC>;
#[doc = ""]
pub mod _1int_st1;
#[doc = "_1INT_ENA1 (rw) register accessor: an alias for `Reg<_1INT_ENA1_SPEC>`"]
pub type _1INT_ENA1 = crate::Reg<_1int_ena1::_1INT_ENA1_SPEC>;
#[doc = ""]
pub mod _1int_ena1;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
#[doc = "ID (rw) register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = ""]
pub mod id;
