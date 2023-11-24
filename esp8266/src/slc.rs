#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    slc_conf0: SLC_CONF0,
    slc_int_raw: SLC_INT_RAW,
    slc_int_status: SLC_INT_STATUS,
    slc_int_ena: SLC_INT_ENA,
    slc_int_clr: SLC_INT_CLR,
    slc_rx_status: SLC_RX_STATUS,
    slc_rx_fifo_push: SLC_RX_FIFO_PUSH,
    slc_tx_status: SLC_TX_STATUS,
    slc_tx_fifo_pop: SLC_TX_FIFO_POP,
    slc_rx_link: SLC_RX_LINK,
    slc_tx_link: SLC_TX_LINK,
    slc_intvec_tohost: SLC_INTVEC_TOHOST,
    slc_token0: SLC_TOKEN0,
    slc_token1: SLC_TOKEN1,
    slc_conf1: SLC_CONF1,
    slc_state0: SLC_STATE0,
    slc_state1: SLC_STATE1,
    slc_bridge_conf: SLC_BRIDGE_CONF,
    slc_rx_eof_des_addr: SLC_RX_EOF_DES_ADDR,
    slc_tx_eof_des_addr: SLC_TX_EOF_DES_ADDR,
    slc_rx_eof_bfr_des_addr: SLC_RX_EOF_BFR_DES_ADDR,
    slc_ahb_test: SLC_AHB_TEST,
    slc_sdio_st: SLC_SDIO_ST,
    slc_rx_dscr_conf: SLC_RX_DSCR_CONF,
    slc_txlink_dscr: SLC_TXLINK_DSCR,
    slc_txlink_dscr_bf0: SLC_TXLINK_DSCR_BF0,
    slc_txlink_dscr_bf1: SLC_TXLINK_DSCR_BF1,
    slc_rxlink_dscr: SLC_RXLINK_DSCR,
    slc_rxlink_dscr_bf0: SLC_RXLINK_DSCR_BF0,
    slc_rxlink_dscr_bf1: SLC_RXLINK_DSCR_BF1,
    slc_date: SLC_DATE,
    slc_id: SLC_ID,
}
impl RegisterBlock {
    #[doc = "0x00 - SLC_CONF0"]
    #[inline(always)]
    pub const fn slc_conf0(&self) -> &SLC_CONF0 {
        &self.slc_conf0
    }
    #[doc = "0x04 - SLC_INT_RAW"]
    #[inline(always)]
    pub const fn slc_int_raw(&self) -> &SLC_INT_RAW {
        &self.slc_int_raw
    }
    #[doc = "0x08 - SLC_INT_STATUS"]
    #[inline(always)]
    pub const fn slc_int_status(&self) -> &SLC_INT_STATUS {
        &self.slc_int_status
    }
    #[doc = "0x0c - SLC_INT_ENA"]
    #[inline(always)]
    pub const fn slc_int_ena(&self) -> &SLC_INT_ENA {
        &self.slc_int_ena
    }
    #[doc = "0x10 - SLC_INT_CLR"]
    #[inline(always)]
    pub const fn slc_int_clr(&self) -> &SLC_INT_CLR {
        &self.slc_int_clr
    }
    #[doc = "0x14 - SLC_RX_STATUS"]
    #[inline(always)]
    pub const fn slc_rx_status(&self) -> &SLC_RX_STATUS {
        &self.slc_rx_status
    }
    #[doc = "0x18 - SLC_RX_FIFO_PUSH"]
    #[inline(always)]
    pub const fn slc_rx_fifo_push(&self) -> &SLC_RX_FIFO_PUSH {
        &self.slc_rx_fifo_push
    }
    #[doc = "0x1c - SLC_TX_STATUS"]
    #[inline(always)]
    pub const fn slc_tx_status(&self) -> &SLC_TX_STATUS {
        &self.slc_tx_status
    }
    #[doc = "0x20 - SLC_TX_FIFO_POP"]
    #[inline(always)]
    pub const fn slc_tx_fifo_pop(&self) -> &SLC_TX_FIFO_POP {
        &self.slc_tx_fifo_pop
    }
    #[doc = "0x24 - SLC_RX_LINK"]
    #[inline(always)]
    pub const fn slc_rx_link(&self) -> &SLC_RX_LINK {
        &self.slc_rx_link
    }
    #[doc = "0x28 - SLC_TX_LINK"]
    #[inline(always)]
    pub const fn slc_tx_link(&self) -> &SLC_TX_LINK {
        &self.slc_tx_link
    }
    #[doc = "0x2c - SLC_INTVEC_TOHOST"]
    #[inline(always)]
    pub const fn slc_intvec_tohost(&self) -> &SLC_INTVEC_TOHOST {
        &self.slc_intvec_tohost
    }
    #[doc = "0x30 - SLC_TOKEN0"]
    #[inline(always)]
    pub const fn slc_token0(&self) -> &SLC_TOKEN0 {
        &self.slc_token0
    }
    #[doc = "0x34 - SLC_TOKEN1"]
    #[inline(always)]
    pub const fn slc_token1(&self) -> &SLC_TOKEN1 {
        &self.slc_token1
    }
    #[doc = "0x38 - SLC_CONF1"]
    #[inline(always)]
    pub const fn slc_conf1(&self) -> &SLC_CONF1 {
        &self.slc_conf1
    }
    #[doc = "0x3c - SLC_STATE0"]
    #[inline(always)]
    pub const fn slc_state0(&self) -> &SLC_STATE0 {
        &self.slc_state0
    }
    #[doc = "0x40 - SLC_STATE1"]
    #[inline(always)]
    pub const fn slc_state1(&self) -> &SLC_STATE1 {
        &self.slc_state1
    }
    #[doc = "0x44 - SLC_BRIDGE_CONF"]
    #[inline(always)]
    pub const fn slc_bridge_conf(&self) -> &SLC_BRIDGE_CONF {
        &self.slc_bridge_conf
    }
    #[doc = "0x48 - SLC_RX_EOF_DES_ADDR"]
    #[inline(always)]
    pub const fn slc_rx_eof_des_addr(&self) -> &SLC_RX_EOF_DES_ADDR {
        &self.slc_rx_eof_des_addr
    }
    #[doc = "0x4c - SLC_TX_EOF_DES_ADDR"]
    #[inline(always)]
    pub const fn slc_tx_eof_des_addr(&self) -> &SLC_TX_EOF_DES_ADDR {
        &self.slc_tx_eof_des_addr
    }
    #[doc = "0x50 - SLC_RX_EOF_BFR_DES_ADDR"]
    #[inline(always)]
    pub const fn slc_rx_eof_bfr_des_addr(&self) -> &SLC_RX_EOF_BFR_DES_ADDR {
        &self.slc_rx_eof_bfr_des_addr
    }
    #[doc = "0x54 - SLC_AHB_TEST"]
    #[inline(always)]
    pub const fn slc_ahb_test(&self) -> &SLC_AHB_TEST {
        &self.slc_ahb_test
    }
    #[doc = "0x58 - SLC_SDIO_ST"]
    #[inline(always)]
    pub const fn slc_sdio_st(&self) -> &SLC_SDIO_ST {
        &self.slc_sdio_st
    }
    #[doc = "0x5c - SLC_RX_DSCR_CONF"]
    #[inline(always)]
    pub const fn slc_rx_dscr_conf(&self) -> &SLC_RX_DSCR_CONF {
        &self.slc_rx_dscr_conf
    }
    #[doc = "0x60 - SLC_TXLINK_DSCR"]
    #[inline(always)]
    pub const fn slc_txlink_dscr(&self) -> &SLC_TXLINK_DSCR {
        &self.slc_txlink_dscr
    }
    #[doc = "0x64 - SLC_TXLINK_DSCR_BF0"]
    #[inline(always)]
    pub const fn slc_txlink_dscr_bf0(&self) -> &SLC_TXLINK_DSCR_BF0 {
        &self.slc_txlink_dscr_bf0
    }
    #[doc = "0x68 - SLC_TXLINK_DSCR_BF1"]
    #[inline(always)]
    pub const fn slc_txlink_dscr_bf1(&self) -> &SLC_TXLINK_DSCR_BF1 {
        &self.slc_txlink_dscr_bf1
    }
    #[doc = "0x6c - SLC_RXLINK_DSCR"]
    #[inline(always)]
    pub const fn slc_rxlink_dscr(&self) -> &SLC_RXLINK_DSCR {
        &self.slc_rxlink_dscr
    }
    #[doc = "0x70 - SLC_RXLINK_DSCR_BF0"]
    #[inline(always)]
    pub const fn slc_rxlink_dscr_bf0(&self) -> &SLC_RXLINK_DSCR_BF0 {
        &self.slc_rxlink_dscr_bf0
    }
    #[doc = "0x74 - SLC_RXLINK_DSCR_BF1"]
    #[inline(always)]
    pub const fn slc_rxlink_dscr_bf1(&self) -> &SLC_RXLINK_DSCR_BF1 {
        &self.slc_rxlink_dscr_bf1
    }
    #[doc = "0x78 - SLC_DATE"]
    #[inline(always)]
    pub const fn slc_date(&self) -> &SLC_DATE {
        &self.slc_date
    }
    #[doc = "0x7c - SLC_ID"]
    #[inline(always)]
    pub const fn slc_id(&self) -> &SLC_ID {
        &self.slc_id
    }
}
#[doc = "SLC_CONF0 (rw) register accessor: SLC_CONF0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_conf0`] module"]
pub type SLC_CONF0 = crate::Reg<slc_conf0::SLC_CONF0_SPEC>;
#[doc = "SLC_CONF0"]
pub mod slc_conf0;
#[doc = "SLC_INT_RAW (rw) register accessor: SLC_INT_RAW\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_int_raw`] module"]
pub type SLC_INT_RAW = crate::Reg<slc_int_raw::SLC_INT_RAW_SPEC>;
#[doc = "SLC_INT_RAW"]
pub mod slc_int_raw;
#[doc = "SLC_INT_STATUS (rw) register accessor: SLC_INT_STATUS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_int_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_int_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_int_status`] module"]
pub type SLC_INT_STATUS = crate::Reg<slc_int_status::SLC_INT_STATUS_SPEC>;
#[doc = "SLC_INT_STATUS"]
pub mod slc_int_status;
#[doc = "SLC_INT_ENA (rw) register accessor: SLC_INT_ENA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_int_ena`] module"]
pub type SLC_INT_ENA = crate::Reg<slc_int_ena::SLC_INT_ENA_SPEC>;
#[doc = "SLC_INT_ENA"]
pub mod slc_int_ena;
#[doc = "SLC_INT_CLR (rw) register accessor: SLC_INT_CLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_int_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_int_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_int_clr`] module"]
pub type SLC_INT_CLR = crate::Reg<slc_int_clr::SLC_INT_CLR_SPEC>;
#[doc = "SLC_INT_CLR"]
pub mod slc_int_clr;
#[doc = "SLC_RX_STATUS (rw) register accessor: SLC_RX_STATUS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_rx_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_rx_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_rx_status`] module"]
pub type SLC_RX_STATUS = crate::Reg<slc_rx_status::SLC_RX_STATUS_SPEC>;
#[doc = "SLC_RX_STATUS"]
pub mod slc_rx_status;
#[doc = "SLC_RX_FIFO_PUSH (rw) register accessor: SLC_RX_FIFO_PUSH\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_rx_fifo_push::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_rx_fifo_push::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_rx_fifo_push`] module"]
pub type SLC_RX_FIFO_PUSH = crate::Reg<slc_rx_fifo_push::SLC_RX_FIFO_PUSH_SPEC>;
#[doc = "SLC_RX_FIFO_PUSH"]
pub mod slc_rx_fifo_push;
#[doc = "SLC_TX_STATUS (rw) register accessor: SLC_TX_STATUS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_tx_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_tx_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_tx_status`] module"]
pub type SLC_TX_STATUS = crate::Reg<slc_tx_status::SLC_TX_STATUS_SPEC>;
#[doc = "SLC_TX_STATUS"]
pub mod slc_tx_status;
#[doc = "SLC_TX_FIFO_POP (rw) register accessor: SLC_TX_FIFO_POP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_tx_fifo_pop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_tx_fifo_pop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_tx_fifo_pop`] module"]
pub type SLC_TX_FIFO_POP = crate::Reg<slc_tx_fifo_pop::SLC_TX_FIFO_POP_SPEC>;
#[doc = "SLC_TX_FIFO_POP"]
pub mod slc_tx_fifo_pop;
#[doc = "SLC_RX_LINK (rw) register accessor: SLC_RX_LINK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_rx_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_rx_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_rx_link`] module"]
pub type SLC_RX_LINK = crate::Reg<slc_rx_link::SLC_RX_LINK_SPEC>;
#[doc = "SLC_RX_LINK"]
pub mod slc_rx_link;
#[doc = "SLC_TX_LINK (rw) register accessor: SLC_TX_LINK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_tx_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_tx_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_tx_link`] module"]
pub type SLC_TX_LINK = crate::Reg<slc_tx_link::SLC_TX_LINK_SPEC>;
#[doc = "SLC_TX_LINK"]
pub mod slc_tx_link;
#[doc = "SLC_INTVEC_TOHOST (rw) register accessor: SLC_INTVEC_TOHOST\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_intvec_tohost::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_intvec_tohost::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_intvec_tohost`] module"]
pub type SLC_INTVEC_TOHOST = crate::Reg<slc_intvec_tohost::SLC_INTVEC_TOHOST_SPEC>;
#[doc = "SLC_INTVEC_TOHOST"]
pub mod slc_intvec_tohost;
#[doc = "SLC_TOKEN0 (rw) register accessor: SLC_TOKEN0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_token0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_token0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_token0`] module"]
pub type SLC_TOKEN0 = crate::Reg<slc_token0::SLC_TOKEN0_SPEC>;
#[doc = "SLC_TOKEN0"]
pub mod slc_token0;
#[doc = "SLC_TOKEN1 (rw) register accessor: SLC_TOKEN1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_token1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_token1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_token1`] module"]
pub type SLC_TOKEN1 = crate::Reg<slc_token1::SLC_TOKEN1_SPEC>;
#[doc = "SLC_TOKEN1"]
pub mod slc_token1;
#[doc = "SLC_CONF1 (rw) register accessor: SLC_CONF1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_conf1`] module"]
pub type SLC_CONF1 = crate::Reg<slc_conf1::SLC_CONF1_SPEC>;
#[doc = "SLC_CONF1"]
pub mod slc_conf1;
#[doc = "SLC_STATE0 (rw) register accessor: SLC_STATE0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_state0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_state0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_state0`] module"]
pub type SLC_STATE0 = crate::Reg<slc_state0::SLC_STATE0_SPEC>;
#[doc = "SLC_STATE0"]
pub mod slc_state0;
#[doc = "SLC_STATE1 (rw) register accessor: SLC_STATE1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_state1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_state1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_state1`] module"]
pub type SLC_STATE1 = crate::Reg<slc_state1::SLC_STATE1_SPEC>;
#[doc = "SLC_STATE1"]
pub mod slc_state1;
#[doc = "SLC_BRIDGE_CONF (rw) register accessor: SLC_BRIDGE_CONF\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_bridge_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_bridge_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_bridge_conf`] module"]
pub type SLC_BRIDGE_CONF = crate::Reg<slc_bridge_conf::SLC_BRIDGE_CONF_SPEC>;
#[doc = "SLC_BRIDGE_CONF"]
pub mod slc_bridge_conf;
#[doc = "SLC_RX_EOF_DES_ADDR (rw) register accessor: SLC_RX_EOF_DES_ADDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_rx_eof_des_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_rx_eof_des_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_rx_eof_des_addr`] module"]
pub type SLC_RX_EOF_DES_ADDR = crate::Reg<slc_rx_eof_des_addr::SLC_RX_EOF_DES_ADDR_SPEC>;
#[doc = "SLC_RX_EOF_DES_ADDR"]
pub mod slc_rx_eof_des_addr;
#[doc = "SLC_TX_EOF_DES_ADDR (rw) register accessor: SLC_TX_EOF_DES_ADDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_tx_eof_des_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_tx_eof_des_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_tx_eof_des_addr`] module"]
pub type SLC_TX_EOF_DES_ADDR = crate::Reg<slc_tx_eof_des_addr::SLC_TX_EOF_DES_ADDR_SPEC>;
#[doc = "SLC_TX_EOF_DES_ADDR"]
pub mod slc_tx_eof_des_addr;
#[doc = "SLC_RX_EOF_BFR_DES_ADDR (rw) register accessor: SLC_RX_EOF_BFR_DES_ADDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_rx_eof_bfr_des_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_rx_eof_bfr_des_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_rx_eof_bfr_des_addr`] module"]
pub type SLC_RX_EOF_BFR_DES_ADDR =
    crate::Reg<slc_rx_eof_bfr_des_addr::SLC_RX_EOF_BFR_DES_ADDR_SPEC>;
#[doc = "SLC_RX_EOF_BFR_DES_ADDR"]
pub mod slc_rx_eof_bfr_des_addr;
#[doc = "SLC_AHB_TEST (rw) register accessor: SLC_AHB_TEST\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_ahb_test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_ahb_test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_ahb_test`] module"]
pub type SLC_AHB_TEST = crate::Reg<slc_ahb_test::SLC_AHB_TEST_SPEC>;
#[doc = "SLC_AHB_TEST"]
pub mod slc_ahb_test;
#[doc = "SLC_SDIO_ST (rw) register accessor: SLC_SDIO_ST\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_sdio_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_sdio_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_sdio_st`] module"]
pub type SLC_SDIO_ST = crate::Reg<slc_sdio_st::SLC_SDIO_ST_SPEC>;
#[doc = "SLC_SDIO_ST"]
pub mod slc_sdio_st;
#[doc = "SLC_RX_DSCR_CONF (rw) register accessor: SLC_RX_DSCR_CONF\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_rx_dscr_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_rx_dscr_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_rx_dscr_conf`] module"]
pub type SLC_RX_DSCR_CONF = crate::Reg<slc_rx_dscr_conf::SLC_RX_DSCR_CONF_SPEC>;
#[doc = "SLC_RX_DSCR_CONF"]
pub mod slc_rx_dscr_conf;
#[doc = "SLC_TXLINK_DSCR (rw) register accessor: SLC_TXLINK_DSCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_txlink_dscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_txlink_dscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_txlink_dscr`] module"]
pub type SLC_TXLINK_DSCR = crate::Reg<slc_txlink_dscr::SLC_TXLINK_DSCR_SPEC>;
#[doc = "SLC_TXLINK_DSCR"]
pub mod slc_txlink_dscr;
#[doc = "SLC_TXLINK_DSCR_BF0 (rw) register accessor: SLC_TXLINK_DSCR_BF0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_txlink_dscr_bf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_txlink_dscr_bf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_txlink_dscr_bf0`] module"]
pub type SLC_TXLINK_DSCR_BF0 = crate::Reg<slc_txlink_dscr_bf0::SLC_TXLINK_DSCR_BF0_SPEC>;
#[doc = "SLC_TXLINK_DSCR_BF0"]
pub mod slc_txlink_dscr_bf0;
#[doc = "SLC_TXLINK_DSCR_BF1 (rw) register accessor: SLC_TXLINK_DSCR_BF1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_txlink_dscr_bf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_txlink_dscr_bf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_txlink_dscr_bf1`] module"]
pub type SLC_TXLINK_DSCR_BF1 = crate::Reg<slc_txlink_dscr_bf1::SLC_TXLINK_DSCR_BF1_SPEC>;
#[doc = "SLC_TXLINK_DSCR_BF1"]
pub mod slc_txlink_dscr_bf1;
#[doc = "SLC_RXLINK_DSCR (rw) register accessor: SLC_RXLINK_DSCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_rxlink_dscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_rxlink_dscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_rxlink_dscr`] module"]
pub type SLC_RXLINK_DSCR = crate::Reg<slc_rxlink_dscr::SLC_RXLINK_DSCR_SPEC>;
#[doc = "SLC_RXLINK_DSCR"]
pub mod slc_rxlink_dscr;
#[doc = "SLC_RXLINK_DSCR_BF0 (rw) register accessor: SLC_RXLINK_DSCR_BF0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_rxlink_dscr_bf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_rxlink_dscr_bf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_rxlink_dscr_bf0`] module"]
pub type SLC_RXLINK_DSCR_BF0 = crate::Reg<slc_rxlink_dscr_bf0::SLC_RXLINK_DSCR_BF0_SPEC>;
#[doc = "SLC_RXLINK_DSCR_BF0"]
pub mod slc_rxlink_dscr_bf0;
#[doc = "SLC_RXLINK_DSCR_BF1 (rw) register accessor: SLC_RXLINK_DSCR_BF1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_rxlink_dscr_bf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_rxlink_dscr_bf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_rxlink_dscr_bf1`] module"]
pub type SLC_RXLINK_DSCR_BF1 = crate::Reg<slc_rxlink_dscr_bf1::SLC_RXLINK_DSCR_BF1_SPEC>;
#[doc = "SLC_RXLINK_DSCR_BF1"]
pub mod slc_rxlink_dscr_bf1;
#[doc = "SLC_DATE (rw) register accessor: SLC_DATE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_date`] module"]
pub type SLC_DATE = crate::Reg<slc_date::SLC_DATE_SPEC>;
#[doc = "SLC_DATE"]
pub mod slc_date;
#[doc = "SLC_ID (rw) register accessor: SLC_ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_id`] module"]
pub type SLC_ID = crate::Reg<slc_id::SLC_ID_SPEC>;
#[doc = "SLC_ID"]
pub mod slc_id;
