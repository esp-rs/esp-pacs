#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - SLC_CONF0"]
    pub slc_conf0: SLC_CONF0,
    #[doc = "0x04 - SLC_INT_RAW"]
    pub slc_int_raw: SLC_INT_RAW,
    #[doc = "0x08 - SLC_INT_STATUS"]
    pub slc_int_status: SLC_INT_STATUS,
    #[doc = "0x0c - SLC_INT_ENA"]
    pub slc_int_ena: SLC_INT_ENA,
    #[doc = "0x10 - SLC_INT_CLR"]
    pub slc_int_clr: SLC_INT_CLR,
    #[doc = "0x14 - SLC_RX_STATUS"]
    pub slc_rx_status: SLC_RX_STATUS,
    #[doc = "0x18 - SLC_RX_FIFO_PUSH"]
    pub slc_rx_fifo_push: SLC_RX_FIFO_PUSH,
    #[doc = "0x1c - SLC_TX_STATUS"]
    pub slc_tx_status: SLC_TX_STATUS,
    #[doc = "0x20 - SLC_TX_FIFO_POP"]
    pub slc_tx_fifo_pop: SLC_TX_FIFO_POP,
    #[doc = "0x24 - SLC_RX_LINK"]
    pub slc_rx_link: SLC_RX_LINK,
    #[doc = "0x28 - SLC_TX_LINK"]
    pub slc_tx_link: SLC_TX_LINK,
    #[doc = "0x2c - SLC_INTVEC_TOHOST"]
    pub slc_intvec_tohost: SLC_INTVEC_TOHOST,
    #[doc = "0x30 - SLC_TOKEN0"]
    pub slc_token0: SLC_TOKEN0,
    #[doc = "0x34 - SLC_TOKEN1"]
    pub slc_token1: SLC_TOKEN1,
    #[doc = "0x38 - SLC_CONF1"]
    pub slc_conf1: SLC_CONF1,
    #[doc = "0x3c - SLC_STATE0"]
    pub slc_state0: SLC_STATE0,
    #[doc = "0x40 - SLC_STATE1"]
    pub slc_state1: SLC_STATE1,
    #[doc = "0x44 - SLC_BRIDGE_CONF"]
    pub slc_bridge_conf: SLC_BRIDGE_CONF,
    #[doc = "0x48 - SLC_RX_EOF_DES_ADDR"]
    pub slc_rx_eof_des_addr: SLC_RX_EOF_DES_ADDR,
    #[doc = "0x4c - SLC_TX_EOF_DES_ADDR"]
    pub slc_tx_eof_des_addr: SLC_TX_EOF_DES_ADDR,
    #[doc = "0x50 - SLC_RX_EOF_BFR_DES_ADDR"]
    pub slc_rx_eof_bfr_des_addr: SLC_RX_EOF_BFR_DES_ADDR,
    #[doc = "0x54 - SLC_AHB_TEST"]
    pub slc_ahb_test: SLC_AHB_TEST,
    #[doc = "0x58 - SLC_SDIO_ST"]
    pub slc_sdio_st: SLC_SDIO_ST,
    #[doc = "0x5c - SLC_RX_DSCR_CONF"]
    pub slc_rx_dscr_conf: SLC_RX_DSCR_CONF,
    #[doc = "0x60 - SLC_TXLINK_DSCR"]
    pub slc_txlink_dscr: SLC_TXLINK_DSCR,
    #[doc = "0x64 - SLC_TXLINK_DSCR_BF0"]
    pub slc_txlink_dscr_bf0: SLC_TXLINK_DSCR_BF0,
    #[doc = "0x68 - SLC_TXLINK_DSCR_BF1"]
    pub slc_txlink_dscr_bf1: SLC_TXLINK_DSCR_BF1,
    #[doc = "0x6c - SLC_RXLINK_DSCR"]
    pub slc_rxlink_dscr: SLC_RXLINK_DSCR,
    #[doc = "0x70 - SLC_RXLINK_DSCR_BF0"]
    pub slc_rxlink_dscr_bf0: SLC_RXLINK_DSCR_BF0,
    #[doc = "0x74 - SLC_RXLINK_DSCR_BF1"]
    pub slc_rxlink_dscr_bf1: SLC_RXLINK_DSCR_BF1,
    #[doc = "0x78 - SLC_DATE"]
    pub slc_date: SLC_DATE,
    #[doc = "0x7c - SLC_ID"]
    pub slc_id: SLC_ID,
}
#[doc = "SLC_CONF0 (rw) register accessor: an alias for `Reg<SLC_CONF0_SPEC>`"]
pub type SLC_CONF0 = crate::Reg<slc_conf0::SLC_CONF0_SPEC>;
#[doc = "SLC_CONF0"]
pub mod slc_conf0;
#[doc = "SLC_INT_RAW (rw) register accessor: an alias for `Reg<SLC_INT_RAW_SPEC>`"]
pub type SLC_INT_RAW = crate::Reg<slc_int_raw::SLC_INT_RAW_SPEC>;
#[doc = "SLC_INT_RAW"]
pub mod slc_int_raw;
#[doc = "SLC_INT_STATUS (rw) register accessor: an alias for `Reg<SLC_INT_STATUS_SPEC>`"]
pub type SLC_INT_STATUS = crate::Reg<slc_int_status::SLC_INT_STATUS_SPEC>;
#[doc = "SLC_INT_STATUS"]
pub mod slc_int_status;
#[doc = "SLC_INT_ENA (rw) register accessor: an alias for `Reg<SLC_INT_ENA_SPEC>`"]
pub type SLC_INT_ENA = crate::Reg<slc_int_ena::SLC_INT_ENA_SPEC>;
#[doc = "SLC_INT_ENA"]
pub mod slc_int_ena;
#[doc = "SLC_INT_CLR (rw) register accessor: an alias for `Reg<SLC_INT_CLR_SPEC>`"]
pub type SLC_INT_CLR = crate::Reg<slc_int_clr::SLC_INT_CLR_SPEC>;
#[doc = "SLC_INT_CLR"]
pub mod slc_int_clr;
#[doc = "SLC_RX_STATUS (rw) register accessor: an alias for `Reg<SLC_RX_STATUS_SPEC>`"]
pub type SLC_RX_STATUS = crate::Reg<slc_rx_status::SLC_RX_STATUS_SPEC>;
#[doc = "SLC_RX_STATUS"]
pub mod slc_rx_status;
#[doc = "SLC_RX_FIFO_PUSH (rw) register accessor: an alias for `Reg<SLC_RX_FIFO_PUSH_SPEC>`"]
pub type SLC_RX_FIFO_PUSH = crate::Reg<slc_rx_fifo_push::SLC_RX_FIFO_PUSH_SPEC>;
#[doc = "SLC_RX_FIFO_PUSH"]
pub mod slc_rx_fifo_push;
#[doc = "SLC_TX_STATUS (rw) register accessor: an alias for `Reg<SLC_TX_STATUS_SPEC>`"]
pub type SLC_TX_STATUS = crate::Reg<slc_tx_status::SLC_TX_STATUS_SPEC>;
#[doc = "SLC_TX_STATUS"]
pub mod slc_tx_status;
#[doc = "SLC_TX_FIFO_POP (rw) register accessor: an alias for `Reg<SLC_TX_FIFO_POP_SPEC>`"]
pub type SLC_TX_FIFO_POP = crate::Reg<slc_tx_fifo_pop::SLC_TX_FIFO_POP_SPEC>;
#[doc = "SLC_TX_FIFO_POP"]
pub mod slc_tx_fifo_pop;
#[doc = "SLC_RX_LINK (rw) register accessor: an alias for `Reg<SLC_RX_LINK_SPEC>`"]
pub type SLC_RX_LINK = crate::Reg<slc_rx_link::SLC_RX_LINK_SPEC>;
#[doc = "SLC_RX_LINK"]
pub mod slc_rx_link;
#[doc = "SLC_TX_LINK (rw) register accessor: an alias for `Reg<SLC_TX_LINK_SPEC>`"]
pub type SLC_TX_LINK = crate::Reg<slc_tx_link::SLC_TX_LINK_SPEC>;
#[doc = "SLC_TX_LINK"]
pub mod slc_tx_link;
#[doc = "SLC_INTVEC_TOHOST (rw) register accessor: an alias for `Reg<SLC_INTVEC_TOHOST_SPEC>`"]
pub type SLC_INTVEC_TOHOST = crate::Reg<slc_intvec_tohost::SLC_INTVEC_TOHOST_SPEC>;
#[doc = "SLC_INTVEC_TOHOST"]
pub mod slc_intvec_tohost;
#[doc = "SLC_TOKEN0 (rw) register accessor: an alias for `Reg<SLC_TOKEN0_SPEC>`"]
pub type SLC_TOKEN0 = crate::Reg<slc_token0::SLC_TOKEN0_SPEC>;
#[doc = "SLC_TOKEN0"]
pub mod slc_token0;
#[doc = "SLC_TOKEN1 (rw) register accessor: an alias for `Reg<SLC_TOKEN1_SPEC>`"]
pub type SLC_TOKEN1 = crate::Reg<slc_token1::SLC_TOKEN1_SPEC>;
#[doc = "SLC_TOKEN1"]
pub mod slc_token1;
#[doc = "SLC_CONF1 (rw) register accessor: an alias for `Reg<SLC_CONF1_SPEC>`"]
pub type SLC_CONF1 = crate::Reg<slc_conf1::SLC_CONF1_SPEC>;
#[doc = "SLC_CONF1"]
pub mod slc_conf1;
#[doc = "SLC_STATE0 (rw) register accessor: an alias for `Reg<SLC_STATE0_SPEC>`"]
pub type SLC_STATE0 = crate::Reg<slc_state0::SLC_STATE0_SPEC>;
#[doc = "SLC_STATE0"]
pub mod slc_state0;
#[doc = "SLC_STATE1 (rw) register accessor: an alias for `Reg<SLC_STATE1_SPEC>`"]
pub type SLC_STATE1 = crate::Reg<slc_state1::SLC_STATE1_SPEC>;
#[doc = "SLC_STATE1"]
pub mod slc_state1;
#[doc = "SLC_BRIDGE_CONF (rw) register accessor: an alias for `Reg<SLC_BRIDGE_CONF_SPEC>`"]
pub type SLC_BRIDGE_CONF = crate::Reg<slc_bridge_conf::SLC_BRIDGE_CONF_SPEC>;
#[doc = "SLC_BRIDGE_CONF"]
pub mod slc_bridge_conf;
#[doc = "SLC_RX_EOF_DES_ADDR (rw) register accessor: an alias for `Reg<SLC_RX_EOF_DES_ADDR_SPEC>`"]
pub type SLC_RX_EOF_DES_ADDR = crate::Reg<slc_rx_eof_des_addr::SLC_RX_EOF_DES_ADDR_SPEC>;
#[doc = "SLC_RX_EOF_DES_ADDR"]
pub mod slc_rx_eof_des_addr;
#[doc = "SLC_TX_EOF_DES_ADDR (rw) register accessor: an alias for `Reg<SLC_TX_EOF_DES_ADDR_SPEC>`"]
pub type SLC_TX_EOF_DES_ADDR = crate::Reg<slc_tx_eof_des_addr::SLC_TX_EOF_DES_ADDR_SPEC>;
#[doc = "SLC_TX_EOF_DES_ADDR"]
pub mod slc_tx_eof_des_addr;
#[doc = "SLC_RX_EOF_BFR_DES_ADDR (rw) register accessor: an alias for `Reg<SLC_RX_EOF_BFR_DES_ADDR_SPEC>`"]
pub type SLC_RX_EOF_BFR_DES_ADDR =
    crate::Reg<slc_rx_eof_bfr_des_addr::SLC_RX_EOF_BFR_DES_ADDR_SPEC>;
#[doc = "SLC_RX_EOF_BFR_DES_ADDR"]
pub mod slc_rx_eof_bfr_des_addr;
#[doc = "SLC_AHB_TEST (rw) register accessor: an alias for `Reg<SLC_AHB_TEST_SPEC>`"]
pub type SLC_AHB_TEST = crate::Reg<slc_ahb_test::SLC_AHB_TEST_SPEC>;
#[doc = "SLC_AHB_TEST"]
pub mod slc_ahb_test;
#[doc = "SLC_SDIO_ST (rw) register accessor: an alias for `Reg<SLC_SDIO_ST_SPEC>`"]
pub type SLC_SDIO_ST = crate::Reg<slc_sdio_st::SLC_SDIO_ST_SPEC>;
#[doc = "SLC_SDIO_ST"]
pub mod slc_sdio_st;
#[doc = "SLC_RX_DSCR_CONF (rw) register accessor: an alias for `Reg<SLC_RX_DSCR_CONF_SPEC>`"]
pub type SLC_RX_DSCR_CONF = crate::Reg<slc_rx_dscr_conf::SLC_RX_DSCR_CONF_SPEC>;
#[doc = "SLC_RX_DSCR_CONF"]
pub mod slc_rx_dscr_conf;
#[doc = "SLC_TXLINK_DSCR (rw) register accessor: an alias for `Reg<SLC_TXLINK_DSCR_SPEC>`"]
pub type SLC_TXLINK_DSCR = crate::Reg<slc_txlink_dscr::SLC_TXLINK_DSCR_SPEC>;
#[doc = "SLC_TXLINK_DSCR"]
pub mod slc_txlink_dscr;
#[doc = "SLC_TXLINK_DSCR_BF0 (rw) register accessor: an alias for `Reg<SLC_TXLINK_DSCR_BF0_SPEC>`"]
pub type SLC_TXLINK_DSCR_BF0 = crate::Reg<slc_txlink_dscr_bf0::SLC_TXLINK_DSCR_BF0_SPEC>;
#[doc = "SLC_TXLINK_DSCR_BF0"]
pub mod slc_txlink_dscr_bf0;
#[doc = "SLC_TXLINK_DSCR_BF1 (rw) register accessor: an alias for `Reg<SLC_TXLINK_DSCR_BF1_SPEC>`"]
pub type SLC_TXLINK_DSCR_BF1 = crate::Reg<slc_txlink_dscr_bf1::SLC_TXLINK_DSCR_BF1_SPEC>;
#[doc = "SLC_TXLINK_DSCR_BF1"]
pub mod slc_txlink_dscr_bf1;
#[doc = "SLC_RXLINK_DSCR (rw) register accessor: an alias for `Reg<SLC_RXLINK_DSCR_SPEC>`"]
pub type SLC_RXLINK_DSCR = crate::Reg<slc_rxlink_dscr::SLC_RXLINK_DSCR_SPEC>;
#[doc = "SLC_RXLINK_DSCR"]
pub mod slc_rxlink_dscr;
#[doc = "SLC_RXLINK_DSCR_BF0 (rw) register accessor: an alias for `Reg<SLC_RXLINK_DSCR_BF0_SPEC>`"]
pub type SLC_RXLINK_DSCR_BF0 = crate::Reg<slc_rxlink_dscr_bf0::SLC_RXLINK_DSCR_BF0_SPEC>;
#[doc = "SLC_RXLINK_DSCR_BF0"]
pub mod slc_rxlink_dscr_bf0;
#[doc = "SLC_RXLINK_DSCR_BF1 (rw) register accessor: an alias for `Reg<SLC_RXLINK_DSCR_BF1_SPEC>`"]
pub type SLC_RXLINK_DSCR_BF1 = crate::Reg<slc_rxlink_dscr_bf1::SLC_RXLINK_DSCR_BF1_SPEC>;
#[doc = "SLC_RXLINK_DSCR_BF1"]
pub mod slc_rxlink_dscr_bf1;
#[doc = "SLC_DATE (rw) register accessor: an alias for `Reg<SLC_DATE_SPEC>`"]
pub type SLC_DATE = crate::Reg<slc_date::SLC_DATE_SPEC>;
#[doc = "SLC_DATE"]
pub mod slc_date;
#[doc = "SLC_ID (rw) register accessor: an alias for `Reg<SLC_ID_SPEC>`"]
pub type SLC_ID = crate::Reg<slc_id::SLC_ID_SPEC>;
#[doc = "SLC_ID"]
pub mod slc_id;
