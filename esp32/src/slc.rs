#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    conf0: CONF0,
    _0int_raw: _0INT_RAW,
    _0int_st: _0INT_ST,
    _0int_ena: _0INT_ENA,
    _0int_clr: _0INT_CLR,
    _1int_raw: _1INT_RAW,
    _1int_st: _1INT_ST,
    _1int_ena: _1INT_ENA,
    _1int_clr: _1INT_CLR,
    rx_status: RX_STATUS,
    _0rxfifo_push: _0RXFIFO_PUSH,
    _1rxfifo_push: _1RXFIFO_PUSH,
    tx_status: TX_STATUS,
    _0txfifo_pop: _0TXFIFO_POP,
    _1txfifo_pop: _1TXFIFO_POP,
    _0rx_link: _0RX_LINK,
    _0tx_link: _0TX_LINK,
    _1rx_link: _1RX_LINK,
    _1tx_link: _1TX_LINK,
    intvec_tohost: INTVEC_TOHOST,
    _0token0: _0TOKEN0,
    _0token1: _0TOKEN1,
    _1token0: _1TOKEN0,
    _1token1: _1TOKEN1,
    conf1: CONF1,
    _0_state0: _0_STATE0,
    _0_state1: _0_STATE1,
    _1_state0: _1_STATE0,
    _1_state1: _1_STATE1,
    bridge_conf: BRIDGE_CONF,
    _0_to_eof_des_addr: _0_TO_EOF_DES_ADDR,
    _0_tx_eof_des_addr: _0_TX_EOF_DES_ADDR,
    _0_to_eof_bfr_des_addr: _0_TO_EOF_BFR_DES_ADDR,
    _1_to_eof_des_addr: _1_TO_EOF_DES_ADDR,
    _1_tx_eof_des_addr: _1_TX_EOF_DES_ADDR,
    _1_to_eof_bfr_des_addr: _1_TO_EOF_BFR_DES_ADDR,
    ahb_test: AHB_TEST,
    sdio_st: SDIO_ST,
    rx_dscr_conf: RX_DSCR_CONF,
    _0_txlink_dscr: _0_TXLINK_DSCR,
    _0_txlink_dscr_bf0: _0_TXLINK_DSCR_BF0,
    _0_txlink_dscr_bf1: _0_TXLINK_DSCR_BF1,
    _0_rxlink_dscr: _0_RXLINK_DSCR,
    _0_rxlink_dscr_bf0: _0_RXLINK_DSCR_BF0,
    _0_rxlink_dscr_bf1: _0_RXLINK_DSCR_BF1,
    _1_txlink_dscr: _1_TXLINK_DSCR,
    _1_txlink_dscr_bf0: _1_TXLINK_DSCR_BF0,
    _1_txlink_dscr_bf1: _1_TXLINK_DSCR_BF1,
    _1_rxlink_dscr: _1_RXLINK_DSCR,
    _1_rxlink_dscr_bf0: _1_RXLINK_DSCR_BF0,
    _1_rxlink_dscr_bf1: _1_RXLINK_DSCR_BF1,
    _0_tx_erreof_des_addr: _0_TX_ERREOF_DES_ADDR,
    _1_tx_erreof_des_addr: _1_TX_ERREOF_DES_ADDR,
    token_lat: TOKEN_LAT,
    tx_dscr_conf: TX_DSCR_CONF,
    cmd_infor0: CMD_INFOR0,
    cmd_infor1: CMD_INFOR1,
    _0_len_conf: _0_LEN_CONF,
    _0_length: _0_LENGTH,
    _0_txpkt_h_dscr: _0_TXPKT_H_DSCR,
    _0_txpkt_e_dscr: _0_TXPKT_E_DSCR,
    _0_rxpkt_h_dscr: _0_RXPKT_H_DSCR,
    _0_rxpkt_e_dscr: _0_RXPKT_E_DSCR,
    _0_txpktu_h_dscr: _0_TXPKTU_H_DSCR,
    _0_txpktu_e_dscr: _0_TXPKTU_E_DSCR,
    _0_rxpktu_h_dscr: _0_RXPKTU_H_DSCR,
    _0_rxpktu_e_dscr: _0_RXPKTU_E_DSCR,
    _reserved67: [u8; 0x08],
    seq_position: SEQ_POSITION,
    _0_dscr_rec_conf: _0_DSCR_REC_CONF,
    sdio_crc_st0: SDIO_CRC_ST0,
    sdio_crc_st1: SDIO_CRC_ST1,
    _0_eof_start_des: _0_EOF_START_DES,
    _0_push_dscr_addr: _0_PUSH_DSCR_ADDR,
    _0_done_dscr_addr: _0_DONE_DSCR_ADDR,
    _0_sub_start_des: _0_SUB_START_DES,
    _0_dscr_cnt: _0_DSCR_CNT,
    _0_len_lim_conf: _0_LEN_LIM_CONF,
    _0int_st1: _0INT_ST1,
    _0int_ena1: _0INT_ENA1,
    _1int_st1: _1INT_ST1,
    _1int_ena1: _1INT_ENA1,
    _reserved81: [u8; 0xac],
    date: DATE,
    id: ID,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn conf0(&self) -> &CONF0 {
        &self.conf0
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn _0int_raw(&self) -> &_0INT_RAW {
        &self._0int_raw
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn _0int_st(&self) -> &_0INT_ST {
        &self._0int_st
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn _0int_ena(&self) -> &_0INT_ENA {
        &self._0int_ena
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn _0int_clr(&self) -> &_0INT_CLR {
        &self._0int_clr
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn _1int_raw(&self) -> &_1INT_RAW {
        &self._1int_raw
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn _1int_st(&self) -> &_1INT_ST {
        &self._1int_st
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn _1int_ena(&self) -> &_1INT_ENA {
        &self._1int_ena
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn _1int_clr(&self) -> &_1INT_CLR {
        &self._1int_clr
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn rx_status(&self) -> &RX_STATUS {
        &self.rx_status
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn _0rxfifo_push(&self) -> &_0RXFIFO_PUSH {
        &self._0rxfifo_push
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn _1rxfifo_push(&self) -> &_1RXFIFO_PUSH {
        &self._1rxfifo_push
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn tx_status(&self) -> &TX_STATUS {
        &self.tx_status
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn _0txfifo_pop(&self) -> &_0TXFIFO_POP {
        &self._0txfifo_pop
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn _1txfifo_pop(&self) -> &_1TXFIFO_POP {
        &self._1txfifo_pop
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn _0rx_link(&self) -> &_0RX_LINK {
        &self._0rx_link
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn _0tx_link(&self) -> &_0TX_LINK {
        &self._0tx_link
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn _1rx_link(&self) -> &_1RX_LINK {
        &self._1rx_link
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn _1tx_link(&self) -> &_1TX_LINK {
        &self._1tx_link
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn intvec_tohost(&self) -> &INTVEC_TOHOST {
        &self.intvec_tohost
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn _0token0(&self) -> &_0TOKEN0 {
        &self._0token0
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn _0token1(&self) -> &_0TOKEN1 {
        &self._0token1
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn _1token0(&self) -> &_1TOKEN0 {
        &self._1token0
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn _1token1(&self) -> &_1TOKEN1 {
        &self._1token1
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn conf1(&self) -> &CONF1 {
        &self.conf1
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn _0_state0(&self) -> &_0_STATE0 {
        &self._0_state0
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn _0_state1(&self) -> &_0_STATE1 {
        &self._0_state1
    }
    #[doc = "0x6c - "]
    #[inline(always)]
    pub const fn _1_state0(&self) -> &_1_STATE0 {
        &self._1_state0
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn _1_state1(&self) -> &_1_STATE1 {
        &self._1_state1
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn bridge_conf(&self) -> &BRIDGE_CONF {
        &self.bridge_conf
    }
    #[doc = "0x78 - "]
    #[inline(always)]
    pub const fn _0_to_eof_des_addr(&self) -> &_0_TO_EOF_DES_ADDR {
        &self._0_to_eof_des_addr
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn _0_tx_eof_des_addr(&self) -> &_0_TX_EOF_DES_ADDR {
        &self._0_tx_eof_des_addr
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn _0_to_eof_bfr_des_addr(&self) -> &_0_TO_EOF_BFR_DES_ADDR {
        &self._0_to_eof_bfr_des_addr
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn _1_to_eof_des_addr(&self) -> &_1_TO_EOF_DES_ADDR {
        &self._1_to_eof_des_addr
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn _1_tx_eof_des_addr(&self) -> &_1_TX_EOF_DES_ADDR {
        &self._1_tx_eof_des_addr
    }
    #[doc = "0x8c - "]
    #[inline(always)]
    pub const fn _1_to_eof_bfr_des_addr(&self) -> &_1_TO_EOF_BFR_DES_ADDR {
        &self._1_to_eof_bfr_des_addr
    }
    #[doc = "0x90 - "]
    #[inline(always)]
    pub const fn ahb_test(&self) -> &AHB_TEST {
        &self.ahb_test
    }
    #[doc = "0x94 - "]
    #[inline(always)]
    pub const fn sdio_st(&self) -> &SDIO_ST {
        &self.sdio_st
    }
    #[doc = "0x98 - "]
    #[inline(always)]
    pub const fn rx_dscr_conf(&self) -> &RX_DSCR_CONF {
        &self.rx_dscr_conf
    }
    #[doc = "0x9c - "]
    #[inline(always)]
    pub const fn _0_txlink_dscr(&self) -> &_0_TXLINK_DSCR {
        &self._0_txlink_dscr
    }
    #[doc = "0xa0 - "]
    #[inline(always)]
    pub const fn _0_txlink_dscr_bf0(&self) -> &_0_TXLINK_DSCR_BF0 {
        &self._0_txlink_dscr_bf0
    }
    #[doc = "0xa4 - "]
    #[inline(always)]
    pub const fn _0_txlink_dscr_bf1(&self) -> &_0_TXLINK_DSCR_BF1 {
        &self._0_txlink_dscr_bf1
    }
    #[doc = "0xa8 - "]
    #[inline(always)]
    pub const fn _0_rxlink_dscr(&self) -> &_0_RXLINK_DSCR {
        &self._0_rxlink_dscr
    }
    #[doc = "0xac - "]
    #[inline(always)]
    pub const fn _0_rxlink_dscr_bf0(&self) -> &_0_RXLINK_DSCR_BF0 {
        &self._0_rxlink_dscr_bf0
    }
    #[doc = "0xb0 - "]
    #[inline(always)]
    pub const fn _0_rxlink_dscr_bf1(&self) -> &_0_RXLINK_DSCR_BF1 {
        &self._0_rxlink_dscr_bf1
    }
    #[doc = "0xb4 - "]
    #[inline(always)]
    pub const fn _1_txlink_dscr(&self) -> &_1_TXLINK_DSCR {
        &self._1_txlink_dscr
    }
    #[doc = "0xb8 - "]
    #[inline(always)]
    pub const fn _1_txlink_dscr_bf0(&self) -> &_1_TXLINK_DSCR_BF0 {
        &self._1_txlink_dscr_bf0
    }
    #[doc = "0xbc - "]
    #[inline(always)]
    pub const fn _1_txlink_dscr_bf1(&self) -> &_1_TXLINK_DSCR_BF1 {
        &self._1_txlink_dscr_bf1
    }
    #[doc = "0xc0 - "]
    #[inline(always)]
    pub const fn _1_rxlink_dscr(&self) -> &_1_RXLINK_DSCR {
        &self._1_rxlink_dscr
    }
    #[doc = "0xc4 - "]
    #[inline(always)]
    pub const fn _1_rxlink_dscr_bf0(&self) -> &_1_RXLINK_DSCR_BF0 {
        &self._1_rxlink_dscr_bf0
    }
    #[doc = "0xc8 - "]
    #[inline(always)]
    pub const fn _1_rxlink_dscr_bf1(&self) -> &_1_RXLINK_DSCR_BF1 {
        &self._1_rxlink_dscr_bf1
    }
    #[doc = "0xcc - "]
    #[inline(always)]
    pub const fn _0_tx_erreof_des_addr(&self) -> &_0_TX_ERREOF_DES_ADDR {
        &self._0_tx_erreof_des_addr
    }
    #[doc = "0xd0 - "]
    #[inline(always)]
    pub const fn _1_tx_erreof_des_addr(&self) -> &_1_TX_ERREOF_DES_ADDR {
        &self._1_tx_erreof_des_addr
    }
    #[doc = "0xd4 - "]
    #[inline(always)]
    pub const fn token_lat(&self) -> &TOKEN_LAT {
        &self.token_lat
    }
    #[doc = "0xd8 - "]
    #[inline(always)]
    pub const fn tx_dscr_conf(&self) -> &TX_DSCR_CONF {
        &self.tx_dscr_conf
    }
    #[doc = "0xdc - "]
    #[inline(always)]
    pub const fn cmd_infor0(&self) -> &CMD_INFOR0 {
        &self.cmd_infor0
    }
    #[doc = "0xe0 - "]
    #[inline(always)]
    pub const fn cmd_infor1(&self) -> &CMD_INFOR1 {
        &self.cmd_infor1
    }
    #[doc = "0xe4 - "]
    #[inline(always)]
    pub const fn _0_len_conf(&self) -> &_0_LEN_CONF {
        &self._0_len_conf
    }
    #[doc = "0xe8 - "]
    #[inline(always)]
    pub const fn _0_length(&self) -> &_0_LENGTH {
        &self._0_length
    }
    #[doc = "0xec - "]
    #[inline(always)]
    pub const fn _0_txpkt_h_dscr(&self) -> &_0_TXPKT_H_DSCR {
        &self._0_txpkt_h_dscr
    }
    #[doc = "0xf0 - "]
    #[inline(always)]
    pub const fn _0_txpkt_e_dscr(&self) -> &_0_TXPKT_E_DSCR {
        &self._0_txpkt_e_dscr
    }
    #[doc = "0xf4 - "]
    #[inline(always)]
    pub const fn _0_rxpkt_h_dscr(&self) -> &_0_RXPKT_H_DSCR {
        &self._0_rxpkt_h_dscr
    }
    #[doc = "0xf8 - "]
    #[inline(always)]
    pub const fn _0_rxpkt_e_dscr(&self) -> &_0_RXPKT_E_DSCR {
        &self._0_rxpkt_e_dscr
    }
    #[doc = "0xfc - "]
    #[inline(always)]
    pub const fn _0_txpktu_h_dscr(&self) -> &_0_TXPKTU_H_DSCR {
        &self._0_txpktu_h_dscr
    }
    #[doc = "0x100 - "]
    #[inline(always)]
    pub const fn _0_txpktu_e_dscr(&self) -> &_0_TXPKTU_E_DSCR {
        &self._0_txpktu_e_dscr
    }
    #[doc = "0x104 - "]
    #[inline(always)]
    pub const fn _0_rxpktu_h_dscr(&self) -> &_0_RXPKTU_H_DSCR {
        &self._0_rxpktu_h_dscr
    }
    #[doc = "0x108 - "]
    #[inline(always)]
    pub const fn _0_rxpktu_e_dscr(&self) -> &_0_RXPKTU_E_DSCR {
        &self._0_rxpktu_e_dscr
    }
    #[doc = "0x114 - "]
    #[inline(always)]
    pub const fn seq_position(&self) -> &SEQ_POSITION {
        &self.seq_position
    }
    #[doc = "0x118 - "]
    #[inline(always)]
    pub const fn _0_dscr_rec_conf(&self) -> &_0_DSCR_REC_CONF {
        &self._0_dscr_rec_conf
    }
    #[doc = "0x11c - "]
    #[inline(always)]
    pub const fn sdio_crc_st0(&self) -> &SDIO_CRC_ST0 {
        &self.sdio_crc_st0
    }
    #[doc = "0x120 - "]
    #[inline(always)]
    pub const fn sdio_crc_st1(&self) -> &SDIO_CRC_ST1 {
        &self.sdio_crc_st1
    }
    #[doc = "0x124 - "]
    #[inline(always)]
    pub const fn _0_eof_start_des(&self) -> &_0_EOF_START_DES {
        &self._0_eof_start_des
    }
    #[doc = "0x128 - "]
    #[inline(always)]
    pub const fn _0_push_dscr_addr(&self) -> &_0_PUSH_DSCR_ADDR {
        &self._0_push_dscr_addr
    }
    #[doc = "0x12c - "]
    #[inline(always)]
    pub const fn _0_done_dscr_addr(&self) -> &_0_DONE_DSCR_ADDR {
        &self._0_done_dscr_addr
    }
    #[doc = "0x130 - "]
    #[inline(always)]
    pub const fn _0_sub_start_des(&self) -> &_0_SUB_START_DES {
        &self._0_sub_start_des
    }
    #[doc = "0x134 - "]
    #[inline(always)]
    pub const fn _0_dscr_cnt(&self) -> &_0_DSCR_CNT {
        &self._0_dscr_cnt
    }
    #[doc = "0x138 - "]
    #[inline(always)]
    pub const fn _0_len_lim_conf(&self) -> &_0_LEN_LIM_CONF {
        &self._0_len_lim_conf
    }
    #[doc = "0x13c - "]
    #[inline(always)]
    pub const fn _0int_st1(&self) -> &_0INT_ST1 {
        &self._0int_st1
    }
    #[doc = "0x140 - "]
    #[inline(always)]
    pub const fn _0int_ena1(&self) -> &_0INT_ENA1 {
        &self._0int_ena1
    }
    #[doc = "0x144 - "]
    #[inline(always)]
    pub const fn _1int_st1(&self) -> &_1INT_ST1 {
        &self._1int_st1
    }
    #[doc = "0x148 - "]
    #[inline(always)]
    pub const fn _1int_ena1(&self) -> &_1INT_ENA1 {
        &self._1int_ena1
    }
    #[doc = "0x1f8 - "]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    #[doc = "0x1fc - "]
    #[inline(always)]
    pub const fn id(&self) -> &ID {
        &self.id
    }
}
#[doc = "CONF0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf0`] module"]
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
#[doc = ""]
pub mod conf0;
#[doc = "_0INT_RAW (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0int_raw`] module"]
pub type _0INT_RAW = crate::Reg<_0int_raw::_0INT_RAW_SPEC>;
#[doc = ""]
pub mod _0int_raw;
#[doc = "_0INT_ST (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0int_st`] module"]
pub type _0INT_ST = crate::Reg<_0int_st::_0INT_ST_SPEC>;
#[doc = ""]
pub mod _0int_st;
#[doc = "_0INT_ENA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0int_ena`] module"]
pub type _0INT_ENA = crate::Reg<_0int_ena::_0INT_ENA_SPEC>;
#[doc = ""]
pub mod _0int_ena;
#[doc = "_0INT_CLR (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0int_clr`] module"]
pub type _0INT_CLR = crate::Reg<_0int_clr::_0INT_CLR_SPEC>;
#[doc = ""]
pub mod _0int_clr;
#[doc = "_1INT_RAW (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1int_raw`] module"]
pub type _1INT_RAW = crate::Reg<_1int_raw::_1INT_RAW_SPEC>;
#[doc = ""]
pub mod _1int_raw;
#[doc = "_1INT_ST (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1int_st`] module"]
pub type _1INT_ST = crate::Reg<_1int_st::_1INT_ST_SPEC>;
#[doc = ""]
pub mod _1int_st;
#[doc = "_1INT_ENA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1int_ena`] module"]
pub type _1INT_ENA = crate::Reg<_1int_ena::_1INT_ENA_SPEC>;
#[doc = ""]
pub mod _1int_ena;
#[doc = "_1INT_CLR (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1int_clr`] module"]
pub type _1INT_CLR = crate::Reg<_1int_clr::_1INT_CLR_SPEC>;
#[doc = ""]
pub mod _1int_clr;
#[doc = "RX_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_status`] module"]
pub type RX_STATUS = crate::Reg<rx_status::RX_STATUS_SPEC>;
#[doc = ""]
pub mod rx_status;
#[doc = "_0RXFIFO_PUSH (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0rxfifo_push::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0rxfifo_push::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0rxfifo_push`] module"]
pub type _0RXFIFO_PUSH = crate::Reg<_0rxfifo_push::_0RXFIFO_PUSH_SPEC>;
#[doc = ""]
pub mod _0rxfifo_push;
#[doc = "_1RXFIFO_PUSH (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1rxfifo_push::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1rxfifo_push::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1rxfifo_push`] module"]
pub type _1RXFIFO_PUSH = crate::Reg<_1rxfifo_push::_1RXFIFO_PUSH_SPEC>;
#[doc = ""]
pub mod _1rxfifo_push;
#[doc = "TX_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_status`] module"]
pub type TX_STATUS = crate::Reg<tx_status::TX_STATUS_SPEC>;
#[doc = ""]
pub mod tx_status;
#[doc = "_0TXFIFO_POP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0txfifo_pop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0txfifo_pop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0txfifo_pop`] module"]
pub type _0TXFIFO_POP = crate::Reg<_0txfifo_pop::_0TXFIFO_POP_SPEC>;
#[doc = ""]
pub mod _0txfifo_pop;
#[doc = "_1TXFIFO_POP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1txfifo_pop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1txfifo_pop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1txfifo_pop`] module"]
pub type _1TXFIFO_POP = crate::Reg<_1txfifo_pop::_1TXFIFO_POP_SPEC>;
#[doc = ""]
pub mod _1txfifo_pop;
#[doc = "_0RX_LINK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0rx_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0rx_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0rx_link`] module"]
pub type _0RX_LINK = crate::Reg<_0rx_link::_0RX_LINK_SPEC>;
#[doc = ""]
pub mod _0rx_link;
#[doc = "_0TX_LINK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0tx_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0tx_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0tx_link`] module"]
pub type _0TX_LINK = crate::Reg<_0tx_link::_0TX_LINK_SPEC>;
#[doc = ""]
pub mod _0tx_link;
#[doc = "_1RX_LINK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1rx_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1rx_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1rx_link`] module"]
pub type _1RX_LINK = crate::Reg<_1rx_link::_1RX_LINK_SPEC>;
#[doc = ""]
pub mod _1rx_link;
#[doc = "_1TX_LINK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1tx_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1tx_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1tx_link`] module"]
pub type _1TX_LINK = crate::Reg<_1tx_link::_1TX_LINK_SPEC>;
#[doc = ""]
pub mod _1tx_link;
#[doc = "INTVEC_TOHOST (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intvec_tohost::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvec_tohost`] module"]
pub type INTVEC_TOHOST = crate::Reg<intvec_tohost::INTVEC_TOHOST_SPEC>;
#[doc = ""]
pub mod intvec_tohost;
#[doc = "_0TOKEN0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0token0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0token0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0token0`] module"]
pub type _0TOKEN0 = crate::Reg<_0token0::_0TOKEN0_SPEC>;
#[doc = ""]
pub mod _0token0;
#[doc = "_0TOKEN1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0token1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0token1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0token1`] module"]
pub type _0TOKEN1 = crate::Reg<_0token1::_0TOKEN1_SPEC>;
#[doc = ""]
pub mod _0token1;
#[doc = "_1TOKEN0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1token0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1token0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1token0`] module"]
pub type _1TOKEN0 = crate::Reg<_1token0::_1TOKEN0_SPEC>;
#[doc = ""]
pub mod _1token0;
#[doc = "_1TOKEN1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1token1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1token1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1token1`] module"]
pub type _1TOKEN1 = crate::Reg<_1token1::_1TOKEN1_SPEC>;
#[doc = ""]
pub mod _1token1;
#[doc = "CONF1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf1`] module"]
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
#[doc = ""]
pub mod conf1;
#[doc = "_0_STATE0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_state0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_state0`] module"]
pub type _0_STATE0 = crate::Reg<_0_state0::_0_STATE0_SPEC>;
#[doc = ""]
pub mod _0_state0;
#[doc = "_0_STATE1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_state1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_state1`] module"]
pub type _0_STATE1 = crate::Reg<_0_state1::_0_STATE1_SPEC>;
#[doc = ""]
pub mod _0_state1;
#[doc = "_1_STATE0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_state0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_state0`] module"]
pub type _1_STATE0 = crate::Reg<_1_state0::_1_STATE0_SPEC>;
#[doc = ""]
pub mod _1_state0;
#[doc = "_1_STATE1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_state1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_state1`] module"]
pub type _1_STATE1 = crate::Reg<_1_state1::_1_STATE1_SPEC>;
#[doc = ""]
pub mod _1_state1;
#[doc = "BRIDGE_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bridge_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bridge_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bridge_conf`] module"]
pub type BRIDGE_CONF = crate::Reg<bridge_conf::BRIDGE_CONF_SPEC>;
#[doc = ""]
pub mod bridge_conf;
#[doc = "_0_TO_EOF_DES_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_to_eof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_to_eof_des_addr`] module"]
pub type _0_TO_EOF_DES_ADDR = crate::Reg<_0_to_eof_des_addr::_0_TO_EOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod _0_to_eof_des_addr;
#[doc = "_0_TX_EOF_DES_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_tx_eof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_tx_eof_des_addr`] module"]
pub type _0_TX_EOF_DES_ADDR = crate::Reg<_0_tx_eof_des_addr::_0_TX_EOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod _0_tx_eof_des_addr;
#[doc = "_0_TO_EOF_BFR_DES_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_to_eof_bfr_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_to_eof_bfr_des_addr`] module"]
pub type _0_TO_EOF_BFR_DES_ADDR = crate::Reg<_0_to_eof_bfr_des_addr::_0_TO_EOF_BFR_DES_ADDR_SPEC>;
#[doc = ""]
pub mod _0_to_eof_bfr_des_addr;
#[doc = "_1_TO_EOF_DES_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_to_eof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_to_eof_des_addr`] module"]
pub type _1_TO_EOF_DES_ADDR = crate::Reg<_1_to_eof_des_addr::_1_TO_EOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod _1_to_eof_des_addr;
#[doc = "_1_TX_EOF_DES_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_tx_eof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_tx_eof_des_addr`] module"]
pub type _1_TX_EOF_DES_ADDR = crate::Reg<_1_tx_eof_des_addr::_1_TX_EOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod _1_tx_eof_des_addr;
#[doc = "_1_TO_EOF_BFR_DES_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_to_eof_bfr_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_to_eof_bfr_des_addr`] module"]
pub type _1_TO_EOF_BFR_DES_ADDR = crate::Reg<_1_to_eof_bfr_des_addr::_1_TO_EOF_BFR_DES_ADDR_SPEC>;
#[doc = ""]
pub mod _1_to_eof_bfr_des_addr;
#[doc = "AHB_TEST (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_test`] module"]
pub type AHB_TEST = crate::Reg<ahb_test::AHB_TEST_SPEC>;
#[doc = ""]
pub mod ahb_test;
#[doc = "SDIO_ST (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_st`] module"]
pub type SDIO_ST = crate::Reg<sdio_st::SDIO_ST_SPEC>;
#[doc = ""]
pub mod sdio_st;
#[doc = "RX_DSCR_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_dscr_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_dscr_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_dscr_conf`] module"]
pub type RX_DSCR_CONF = crate::Reg<rx_dscr_conf::RX_DSCR_CONF_SPEC>;
#[doc = ""]
pub mod rx_dscr_conf;
#[doc = "_0_TXLINK_DSCR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_txlink_dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_txlink_dscr`] module"]
pub type _0_TXLINK_DSCR = crate::Reg<_0_txlink_dscr::_0_TXLINK_DSCR_SPEC>;
#[doc = ""]
pub mod _0_txlink_dscr;
#[doc = "_0_TXLINK_DSCR_BF0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_txlink_dscr_bf0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_txlink_dscr_bf0`] module"]
pub type _0_TXLINK_DSCR_BF0 = crate::Reg<_0_txlink_dscr_bf0::_0_TXLINK_DSCR_BF0_SPEC>;
#[doc = ""]
pub mod _0_txlink_dscr_bf0;
#[doc = "_0_TXLINK_DSCR_BF1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_txlink_dscr_bf1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_txlink_dscr_bf1`] module"]
pub type _0_TXLINK_DSCR_BF1 = crate::Reg<_0_txlink_dscr_bf1::_0_TXLINK_DSCR_BF1_SPEC>;
#[doc = ""]
pub mod _0_txlink_dscr_bf1;
#[doc = "_0_RXLINK_DSCR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_rxlink_dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_rxlink_dscr`] module"]
pub type _0_RXLINK_DSCR = crate::Reg<_0_rxlink_dscr::_0_RXLINK_DSCR_SPEC>;
#[doc = ""]
pub mod _0_rxlink_dscr;
#[doc = "_0_RXLINK_DSCR_BF0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_rxlink_dscr_bf0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_rxlink_dscr_bf0`] module"]
pub type _0_RXLINK_DSCR_BF0 = crate::Reg<_0_rxlink_dscr_bf0::_0_RXLINK_DSCR_BF0_SPEC>;
#[doc = ""]
pub mod _0_rxlink_dscr_bf0;
#[doc = "_0_RXLINK_DSCR_BF1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_rxlink_dscr_bf1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_rxlink_dscr_bf1`] module"]
pub type _0_RXLINK_DSCR_BF1 = crate::Reg<_0_rxlink_dscr_bf1::_0_RXLINK_DSCR_BF1_SPEC>;
#[doc = ""]
pub mod _0_rxlink_dscr_bf1;
#[doc = "_1_TXLINK_DSCR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_txlink_dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_txlink_dscr`] module"]
pub type _1_TXLINK_DSCR = crate::Reg<_1_txlink_dscr::_1_TXLINK_DSCR_SPEC>;
#[doc = ""]
pub mod _1_txlink_dscr;
#[doc = "_1_TXLINK_DSCR_BF0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_txlink_dscr_bf0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_txlink_dscr_bf0`] module"]
pub type _1_TXLINK_DSCR_BF0 = crate::Reg<_1_txlink_dscr_bf0::_1_TXLINK_DSCR_BF0_SPEC>;
#[doc = ""]
pub mod _1_txlink_dscr_bf0;
#[doc = "_1_TXLINK_DSCR_BF1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_txlink_dscr_bf1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_txlink_dscr_bf1`] module"]
pub type _1_TXLINK_DSCR_BF1 = crate::Reg<_1_txlink_dscr_bf1::_1_TXLINK_DSCR_BF1_SPEC>;
#[doc = ""]
pub mod _1_txlink_dscr_bf1;
#[doc = "_1_RXLINK_DSCR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_rxlink_dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_rxlink_dscr`] module"]
pub type _1_RXLINK_DSCR = crate::Reg<_1_rxlink_dscr::_1_RXLINK_DSCR_SPEC>;
#[doc = ""]
pub mod _1_rxlink_dscr;
#[doc = "_1_RXLINK_DSCR_BF0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_rxlink_dscr_bf0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_rxlink_dscr_bf0`] module"]
pub type _1_RXLINK_DSCR_BF0 = crate::Reg<_1_rxlink_dscr_bf0::_1_RXLINK_DSCR_BF0_SPEC>;
#[doc = ""]
pub mod _1_rxlink_dscr_bf0;
#[doc = "_1_RXLINK_DSCR_BF1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_rxlink_dscr_bf1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_rxlink_dscr_bf1`] module"]
pub type _1_RXLINK_DSCR_BF1 = crate::Reg<_1_rxlink_dscr_bf1::_1_RXLINK_DSCR_BF1_SPEC>;
#[doc = ""]
pub mod _1_rxlink_dscr_bf1;
#[doc = "_0_TX_ERREOF_DES_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_tx_erreof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_tx_erreof_des_addr`] module"]
pub type _0_TX_ERREOF_DES_ADDR = crate::Reg<_0_tx_erreof_des_addr::_0_TX_ERREOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod _0_tx_erreof_des_addr;
#[doc = "_1_TX_ERREOF_DES_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_tx_erreof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_tx_erreof_des_addr`] module"]
pub type _1_TX_ERREOF_DES_ADDR = crate::Reg<_1_tx_erreof_des_addr::_1_TX_ERREOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod _1_tx_erreof_des_addr;
#[doc = "TOKEN_LAT (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`token_lat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@token_lat`] module"]
pub type TOKEN_LAT = crate::Reg<token_lat::TOKEN_LAT_SPEC>;
#[doc = ""]
pub mod token_lat;
#[doc = "TX_DSCR_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_dscr_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_dscr_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_dscr_conf`] module"]
pub type TX_DSCR_CONF = crate::Reg<tx_dscr_conf::TX_DSCR_CONF_SPEC>;
#[doc = ""]
pub mod tx_dscr_conf;
#[doc = "CMD_INFOR0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_infor0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_infor0`] module"]
pub type CMD_INFOR0 = crate::Reg<cmd_infor0::CMD_INFOR0_SPEC>;
#[doc = ""]
pub mod cmd_infor0;
#[doc = "CMD_INFOR1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_infor1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_infor1`] module"]
pub type CMD_INFOR1 = crate::Reg<cmd_infor1::CMD_INFOR1_SPEC>;
#[doc = ""]
pub mod cmd_infor1;
#[doc = "_0_LEN_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_len_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_len_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_len_conf`] module"]
pub type _0_LEN_CONF = crate::Reg<_0_len_conf::_0_LEN_CONF_SPEC>;
#[doc = ""]
pub mod _0_len_conf;
#[doc = "_0_LENGTH (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_length::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_length`] module"]
pub type _0_LENGTH = crate::Reg<_0_length::_0_LENGTH_SPEC>;
#[doc = ""]
pub mod _0_length;
#[doc = "_0_TXPKT_H_DSCR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_txpkt_h_dscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_txpkt_h_dscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_txpkt_h_dscr`] module"]
pub type _0_TXPKT_H_DSCR = crate::Reg<_0_txpkt_h_dscr::_0_TXPKT_H_DSCR_SPEC>;
#[doc = ""]
pub mod _0_txpkt_h_dscr;
#[doc = "_0_TXPKT_E_DSCR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_txpkt_e_dscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_txpkt_e_dscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_txpkt_e_dscr`] module"]
pub type _0_TXPKT_E_DSCR = crate::Reg<_0_txpkt_e_dscr::_0_TXPKT_E_DSCR_SPEC>;
#[doc = ""]
pub mod _0_txpkt_e_dscr;
#[doc = "_0_RXPKT_H_DSCR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_rxpkt_h_dscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_rxpkt_h_dscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_rxpkt_h_dscr`] module"]
pub type _0_RXPKT_H_DSCR = crate::Reg<_0_rxpkt_h_dscr::_0_RXPKT_H_DSCR_SPEC>;
#[doc = ""]
pub mod _0_rxpkt_h_dscr;
#[doc = "_0_RXPKT_E_DSCR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_rxpkt_e_dscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_rxpkt_e_dscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_rxpkt_e_dscr`] module"]
pub type _0_RXPKT_E_DSCR = crate::Reg<_0_rxpkt_e_dscr::_0_RXPKT_E_DSCR_SPEC>;
#[doc = ""]
pub mod _0_rxpkt_e_dscr;
#[doc = "_0_TXPKTU_H_DSCR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_txpktu_h_dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_txpktu_h_dscr`] module"]
pub type _0_TXPKTU_H_DSCR = crate::Reg<_0_txpktu_h_dscr::_0_TXPKTU_H_DSCR_SPEC>;
#[doc = ""]
pub mod _0_txpktu_h_dscr;
#[doc = "_0_TXPKTU_E_DSCR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_txpktu_e_dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_txpktu_e_dscr`] module"]
pub type _0_TXPKTU_E_DSCR = crate::Reg<_0_txpktu_e_dscr::_0_TXPKTU_E_DSCR_SPEC>;
#[doc = ""]
pub mod _0_txpktu_e_dscr;
#[doc = "_0_RXPKTU_H_DSCR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_rxpktu_h_dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_rxpktu_h_dscr`] module"]
pub type _0_RXPKTU_H_DSCR = crate::Reg<_0_rxpktu_h_dscr::_0_RXPKTU_H_DSCR_SPEC>;
#[doc = ""]
pub mod _0_rxpktu_h_dscr;
#[doc = "_0_RXPKTU_E_DSCR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_rxpktu_e_dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_rxpktu_e_dscr`] module"]
pub type _0_RXPKTU_E_DSCR = crate::Reg<_0_rxpktu_e_dscr::_0_RXPKTU_E_DSCR_SPEC>;
#[doc = ""]
pub mod _0_rxpktu_e_dscr;
#[doc = "SEQ_POSITION (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_position::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_position::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_position`] module"]
pub type SEQ_POSITION = crate::Reg<seq_position::SEQ_POSITION_SPEC>;
#[doc = ""]
pub mod seq_position;
#[doc = "_0_DSCR_REC_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_dscr_rec_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_dscr_rec_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_dscr_rec_conf`] module"]
pub type _0_DSCR_REC_CONF = crate::Reg<_0_dscr_rec_conf::_0_DSCR_REC_CONF_SPEC>;
#[doc = ""]
pub mod _0_dscr_rec_conf;
#[doc = "SDIO_CRC_ST0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_crc_st0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_crc_st0`] module"]
pub type SDIO_CRC_ST0 = crate::Reg<sdio_crc_st0::SDIO_CRC_ST0_SPEC>;
#[doc = ""]
pub mod sdio_crc_st0;
#[doc = "SDIO_CRC_ST1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_crc_st1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_crc_st1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_crc_st1`] module"]
pub type SDIO_CRC_ST1 = crate::Reg<sdio_crc_st1::SDIO_CRC_ST1_SPEC>;
#[doc = ""]
pub mod sdio_crc_st1;
#[doc = "_0_EOF_START_DES (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_eof_start_des::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_eof_start_des`] module"]
pub type _0_EOF_START_DES = crate::Reg<_0_eof_start_des::_0_EOF_START_DES_SPEC>;
#[doc = ""]
pub mod _0_eof_start_des;
#[doc = "_0_PUSH_DSCR_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_push_dscr_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_push_dscr_addr`] module"]
pub type _0_PUSH_DSCR_ADDR = crate::Reg<_0_push_dscr_addr::_0_PUSH_DSCR_ADDR_SPEC>;
#[doc = ""]
pub mod _0_push_dscr_addr;
#[doc = "_0_DONE_DSCR_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_done_dscr_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_done_dscr_addr`] module"]
pub type _0_DONE_DSCR_ADDR = crate::Reg<_0_done_dscr_addr::_0_DONE_DSCR_ADDR_SPEC>;
#[doc = ""]
pub mod _0_done_dscr_addr;
#[doc = "_0_SUB_START_DES (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_sub_start_des::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_sub_start_des`] module"]
pub type _0_SUB_START_DES = crate::Reg<_0_sub_start_des::_0_SUB_START_DES_SPEC>;
#[doc = ""]
pub mod _0_sub_start_des;
#[doc = "_0_DSCR_CNT (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_dscr_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_dscr_cnt`] module"]
pub type _0_DSCR_CNT = crate::Reg<_0_dscr_cnt::_0_DSCR_CNT_SPEC>;
#[doc = ""]
pub mod _0_dscr_cnt;
#[doc = "_0_LEN_LIM_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_len_lim_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_len_lim_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_len_lim_conf`] module"]
pub type _0_LEN_LIM_CONF = crate::Reg<_0_len_lim_conf::_0_LEN_LIM_CONF_SPEC>;
#[doc = ""]
pub mod _0_len_lim_conf;
#[doc = "_0INT_ST1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0int_st1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0int_st1`] module"]
pub type _0INT_ST1 = crate::Reg<_0int_st1::_0INT_ST1_SPEC>;
#[doc = ""]
pub mod _0int_st1;
#[doc = "_0INT_ENA1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0int_ena1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0int_ena1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0int_ena1`] module"]
pub type _0INT_ENA1 = crate::Reg<_0int_ena1::_0INT_ENA1_SPEC>;
#[doc = ""]
pub mod _0int_ena1;
#[doc = "_1INT_ST1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1int_st1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1int_st1`] module"]
pub type _1INT_ST1 = crate::Reg<_1int_st1::_1INT_ST1_SPEC>;
#[doc = ""]
pub mod _1int_st1;
#[doc = "_1INT_ENA1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1int_ena1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1int_ena1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1int_ena1`] module"]
pub type _1INT_ENA1 = crate::Reg<_1int_ena1::_1INT_ENA1_SPEC>;
#[doc = ""]
pub mod _1int_ena1;
#[doc = "DATE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
#[doc = "ID (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`] module"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = ""]
pub mod id;
