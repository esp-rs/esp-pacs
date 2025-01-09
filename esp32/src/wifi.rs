#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    filter_bank: [FILTER_BANK; 2],
    _reserved1: [u8; 0x04],
    rx_ctrl: RX_CTRL,
    rx_dma_list: RX_DMA_LIST,
    _reserved3: [u8; 0x44],
    interface_rx_control: [INTERFACE_RX_CONTROL; 4],
    _reserved4: [u8; 0x01d4],
    hw_stat_ack_int: HW_STAT_ACK_INT,
    hw_stat_rts_int: HW_STAT_RTS_INT,
    hw_stat_cts_int: HW_STAT_CTS_INT,
    hw_stat_rifs_int: HW_STAT_RIFS_INT,
    hw_stat_rx_success: HW_STAT_RX_SUCCESS,
    hw_stat_rx_end: HW_STAT_RX_END,
    _reserved10: [u8; 0x04],
    hw_stat_hop_err: HW_STAT_HOP_ERR,
    hw_stat_full2: HW_STAT_FULL2,
    hw_stat_block_err: HW_STAT_BLOCK_ERR,
    _reserved13: [u8; 0x0964],
    mac_interrupt: MAC_INTERRUPT,
    _reserved14: [u8; 0x68],
    ctrl: CTRL,
    txq_state: TXQ_STATE,
    _reserved16: [u8; 0x30],
    tx_slot_config: [TX_SLOT_CONFIG; 5],
    _reserved17: [u8; 0x34],
    hw_stat_tx_rts: HW_STAT_TX_RTS,
    hw_stat_tx_cts: HW_STAT_TX_CTS,
    hw_stat_tx_ack: HW_STAT_TX_ACK,
    hw_stat_trcts: HW_STAT_TRCTS,
    hw_stat_trigger: HW_STAT_TRIGGER,
    hw_stat_tx_hung: HW_STAT_TX_HUNG,
    hw_stat_panic: HW_STAT_PANIC,
    _reserved24: [u8; 0x03f4],
    tx_slot_parameters: [TX_SLOT_PARAMETERS; 5],
}
impl RegisterBlock {
    #[doc = "0x00..0x80 - Filter banks for frame reception. Bank zero is for the BSSID and bank one for the RA. Each filter bank has registers for two interfaces."]
    #[inline(always)]
    pub const fn filter_bank(&self, n: usize) -> &FILTER_BANK {
        &self.filter_bank[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x80 - Filter banks for frame reception. Bank zero is for the BSSID and bank one for the RA. Each filter bank has registers for two interfaces."]
    #[inline(always)]
    pub fn filter_bank_iter(&self) -> impl Iterator<Item = &FILTER_BANK> {
        self.filter_bank.iter()
    }
    #[doc = "0x84 - Controls the reception of frames"]
    #[inline(always)]
    pub const fn rx_ctrl(&self) -> &RX_CTRL {
        &self.rx_ctrl
    }
    #[doc = "0x88..0x94 - RX_DMA_LIST"]
    #[inline(always)]
    pub const fn rx_dma_list(&self) -> &RX_DMA_LIST {
        &self.rx_dma_list
    }
    #[doc = "0xd8..0xe8 - Controls RX for an interface"]
    #[inline(always)]
    pub const fn interface_rx_control(&self, n: usize) -> &INTERFACE_RX_CONTROL {
        &self.interface_rx_control[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xd8..0xe8 - Controls RX for an interface"]
    #[inline(always)]
    pub fn interface_rx_control_iter(&self) -> impl Iterator<Item = &INTERFACE_RX_CONTROL> {
        self.interface_rx_control.iter()
    }
    #[doc = "0x2bc - "]
    #[inline(always)]
    pub const fn hw_stat_ack_int(&self) -> &HW_STAT_ACK_INT {
        &self.hw_stat_ack_int
    }
    #[doc = "0x2c0 - "]
    #[inline(always)]
    pub const fn hw_stat_rts_int(&self) -> &HW_STAT_RTS_INT {
        &self.hw_stat_rts_int
    }
    #[doc = "0x2c4 - "]
    #[inline(always)]
    pub const fn hw_stat_cts_int(&self) -> &HW_STAT_CTS_INT {
        &self.hw_stat_cts_int
    }
    #[doc = "0x2c8 - "]
    #[inline(always)]
    pub const fn hw_stat_rifs_int(&self) -> &HW_STAT_RIFS_INT {
        &self.hw_stat_rifs_int
    }
    #[doc = "0x2cc - "]
    #[inline(always)]
    pub const fn hw_stat_rx_success(&self) -> &HW_STAT_RX_SUCCESS {
        &self.hw_stat_rx_success
    }
    #[doc = "0x2d0 - "]
    #[inline(always)]
    pub const fn hw_stat_rx_end(&self) -> &HW_STAT_RX_END {
        &self.hw_stat_rx_end
    }
    #[doc = "0x2d8 - "]
    #[inline(always)]
    pub const fn hw_stat_hop_err(&self) -> &HW_STAT_HOP_ERR {
        &self.hw_stat_hop_err
    }
    #[doc = "0x2dc - "]
    #[inline(always)]
    pub const fn hw_stat_full2(&self) -> &HW_STAT_FULL2 {
        &self.hw_stat_full2
    }
    #[doc = "0x2e0 - "]
    #[inline(always)]
    pub const fn hw_stat_block_err(&self) -> &HW_STAT_BLOCK_ERR {
        &self.hw_stat_block_err
    }
    #[doc = "0xc48..0xc50 - Status and clear for the Wi-Fi MAC interrupt"]
    #[inline(always)]
    pub const fn mac_interrupt(&self) -> &MAC_INTERRUPT {
        &self.mac_interrupt
    }
    #[doc = "0xcb8 - Exact name and meaning unknown, used for initializing the MAC"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0xcbc..0xccc - State of transmission queues"]
    #[inline(always)]
    pub const fn txq_state(&self) -> &TXQ_STATE {
        &self.txq_state
    }
    #[doc = "0xcfc..0xd24 - Used to configure the TX slot."]
    #[inline(always)]
    pub const fn tx_slot_config(&self, n: usize) -> &TX_SLOT_CONFIG {
        &self.tx_slot_config[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xcfc..0xd24 - Used to configure the TX slot."]
    #[inline(always)]
    pub fn tx_slot_config_iter(&self) -> impl Iterator<Item = &TX_SLOT_CONFIG> {
        self.tx_slot_config.iter()
    }
    #[doc = "0xd58 - "]
    #[inline(always)]
    pub const fn hw_stat_tx_rts(&self) -> &HW_STAT_TX_RTS {
        &self.hw_stat_tx_rts
    }
    #[doc = "0xd5c - "]
    #[inline(always)]
    pub const fn hw_stat_tx_cts(&self) -> &HW_STAT_TX_CTS {
        &self.hw_stat_tx_cts
    }
    #[doc = "0xd60 - "]
    #[inline(always)]
    pub const fn hw_stat_tx_ack(&self) -> &HW_STAT_TX_ACK {
        &self.hw_stat_tx_ack
    }
    #[doc = "0xd64 - "]
    #[inline(always)]
    pub const fn hw_stat_trcts(&self) -> &HW_STAT_TRCTS {
        &self.hw_stat_trcts
    }
    #[doc = "0xd68 - "]
    #[inline(always)]
    pub const fn hw_stat_trigger(&self) -> &HW_STAT_TRIGGER {
        &self.hw_stat_trigger
    }
    #[doc = "0xd6c - "]
    #[inline(always)]
    pub const fn hw_stat_tx_hung(&self) -> &HW_STAT_TX_HUNG {
        &self.hw_stat_tx_hung
    }
    #[doc = "0xd70 - "]
    #[inline(always)]
    pub const fn hw_stat_panic(&self) -> &HW_STAT_PANIC {
        &self.hw_stat_panic
    }
    #[doc = "0x1168..0x1294 - Used to set transmission parameters for the slot"]
    #[inline(always)]
    pub const fn tx_slot_parameters(&self, n: usize) -> &TX_SLOT_PARAMETERS {
        &self.tx_slot_parameters[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1168..0x1294 - Used to set transmission parameters for the slot"]
    #[inline(always)]
    pub fn tx_slot_parameters_iter(&self) -> impl Iterator<Item = &TX_SLOT_PARAMETERS> {
        self.tx_slot_parameters.iter()
    }
}
#[doc = "RX_CTRL (rw) register accessor: Controls the reception of frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ctrl`] module"]
pub type RX_CTRL = crate::Reg<rx_ctrl::RX_CTRL_SPEC>;
#[doc = "Controls the reception of frames"]
pub mod rx_ctrl;
#[doc = "INTERFACE_RX_CONTROL (rw) register accessor: Controls RX for an interface\n\nYou can [`read`](crate::Reg::read) this register and get [`interface_rx_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interface_rx_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interface_rx_control`] module"]
pub type INTERFACE_RX_CONTROL = crate::Reg<interface_rx_control::INTERFACE_RX_CONTROL_SPEC>;
#[doc = "Controls RX for an interface"]
pub mod interface_rx_control;
#[doc = "HW_STAT_ACK_INT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hw_stat_ack_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_stat_ack_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_stat_ack_int`] module"]
pub type HW_STAT_ACK_INT = crate::Reg<hw_stat_ack_int::HW_STAT_ACK_INT_SPEC>;
#[doc = ""]
pub mod hw_stat_ack_int;
#[doc = "HW_STAT_RTS_INT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hw_stat_rts_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_stat_rts_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_stat_rts_int`] module"]
pub type HW_STAT_RTS_INT = crate::Reg<hw_stat_rts_int::HW_STAT_RTS_INT_SPEC>;
#[doc = ""]
pub mod hw_stat_rts_int;
#[doc = "HW_STAT_CTS_INT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hw_stat_cts_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_stat_cts_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_stat_cts_int`] module"]
pub type HW_STAT_CTS_INT = crate::Reg<hw_stat_cts_int::HW_STAT_CTS_INT_SPEC>;
#[doc = ""]
pub mod hw_stat_cts_int;
#[doc = "HW_STAT_RIFS_INT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hw_stat_rifs_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_stat_rifs_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_stat_rifs_int`] module"]
pub type HW_STAT_RIFS_INT = crate::Reg<hw_stat_rifs_int::HW_STAT_RIFS_INT_SPEC>;
#[doc = ""]
pub mod hw_stat_rifs_int;
#[doc = "HW_STAT_RX_SUCCESS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hw_stat_rx_success::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_stat_rx_success::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_stat_rx_success`] module"]
pub type HW_STAT_RX_SUCCESS = crate::Reg<hw_stat_rx_success::HW_STAT_RX_SUCCESS_SPEC>;
#[doc = ""]
pub mod hw_stat_rx_success;
#[doc = "HW_STAT_RX_END (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hw_stat_rx_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_stat_rx_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_stat_rx_end`] module"]
pub type HW_STAT_RX_END = crate::Reg<hw_stat_rx_end::HW_STAT_RX_END_SPEC>;
#[doc = ""]
pub mod hw_stat_rx_end;
#[doc = "HW_STAT_HOP_ERR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hw_stat_hop_err::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_stat_hop_err::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_stat_hop_err`] module"]
pub type HW_STAT_HOP_ERR = crate::Reg<hw_stat_hop_err::HW_STAT_HOP_ERR_SPEC>;
#[doc = ""]
pub mod hw_stat_hop_err;
#[doc = "HW_STAT_FULL2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hw_stat_full2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_stat_full2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_stat_full2`] module"]
pub type HW_STAT_FULL2 = crate::Reg<hw_stat_full2::HW_STAT_FULL2_SPEC>;
#[doc = ""]
pub mod hw_stat_full2;
#[doc = "HW_STAT_BLOCK_ERR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hw_stat_block_err::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_stat_block_err::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_stat_block_err`] module"]
pub type HW_STAT_BLOCK_ERR = crate::Reg<hw_stat_block_err::HW_STAT_BLOCK_ERR_SPEC>;
#[doc = ""]
pub mod hw_stat_block_err;
#[doc = "CTRL (rw) register accessor: Exact name and meaning unknown, used for initializing the MAC\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Exact name and meaning unknown, used for initializing the MAC"]
pub mod ctrl;
#[doc = "HW_STAT_TX_RTS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hw_stat_tx_rts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_stat_tx_rts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_stat_tx_rts`] module"]
pub type HW_STAT_TX_RTS = crate::Reg<hw_stat_tx_rts::HW_STAT_TX_RTS_SPEC>;
#[doc = ""]
pub mod hw_stat_tx_rts;
#[doc = "HW_STAT_TX_CTS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hw_stat_tx_cts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_stat_tx_cts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_stat_tx_cts`] module"]
pub type HW_STAT_TX_CTS = crate::Reg<hw_stat_tx_cts::HW_STAT_TX_CTS_SPEC>;
#[doc = ""]
pub mod hw_stat_tx_cts;
#[doc = "HW_STAT_TX_ACK (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hw_stat_tx_ack::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_stat_tx_ack::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_stat_tx_ack`] module"]
pub type HW_STAT_TX_ACK = crate::Reg<hw_stat_tx_ack::HW_STAT_TX_ACK_SPEC>;
#[doc = ""]
pub mod hw_stat_tx_ack;
#[doc = "HW_STAT_TRCTS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hw_stat_trcts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_stat_trcts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_stat_trcts`] module"]
pub type HW_STAT_TRCTS = crate::Reg<hw_stat_trcts::HW_STAT_TRCTS_SPEC>;
#[doc = ""]
pub mod hw_stat_trcts;
#[doc = "HW_STAT_TRIGGER (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hw_stat_trigger::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_stat_trigger::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_stat_trigger`] module"]
pub type HW_STAT_TRIGGER = crate::Reg<hw_stat_trigger::HW_STAT_TRIGGER_SPEC>;
#[doc = ""]
pub mod hw_stat_trigger;
#[doc = "HW_STAT_TX_HUNG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hw_stat_tx_hung::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_stat_tx_hung::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_stat_tx_hung`] module"]
pub type HW_STAT_TX_HUNG = crate::Reg<hw_stat_tx_hung::HW_STAT_TX_HUNG_SPEC>;
#[doc = ""]
pub mod hw_stat_tx_hung;
#[doc = "HW_STAT_PANIC (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hw_stat_panic::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_stat_panic::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_stat_panic`] module"]
pub type HW_STAT_PANIC = crate::Reg<hw_stat_panic::HW_STAT_PANIC_SPEC>;
#[doc = ""]
pub mod hw_stat_panic;
#[doc = "Filter banks for frame reception. Bank zero is for the BSSID and bank one for the RA. Each filter bank has registers for two interfaces."]
pub use self::filter_bank::FILTER_BANK;
#[doc = r"Cluster"]
#[doc = "Filter banks for frame reception. Bank zero is for the BSSID and bank one for the RA. Each filter bank has registers for two interfaces."]
pub mod filter_bank;
#[doc = "RX_DMA_LIST"]
pub use self::rx_dma_list::RX_DMA_LIST;
#[doc = r"Cluster"]
#[doc = "RX_DMA_LIST"]
pub mod rx_dma_list;
#[doc = "Status and clear for the Wi-Fi MAC interrupt"]
pub use self::mac_interrupt::MAC_INTERRUPT;
#[doc = r"Cluster"]
#[doc = "Status and clear for the Wi-Fi MAC interrupt"]
pub mod mac_interrupt;
#[doc = "State of transmission queues"]
pub use self::txq_state::TXQ_STATE;
#[doc = r"Cluster"]
#[doc = "State of transmission queues"]
pub mod txq_state;
#[doc = "Used to configure the TX slot."]
pub use self::tx_slot_config::TX_SLOT_CONFIG;
#[doc = r"Cluster"]
#[doc = "Used to configure the TX slot."]
pub mod tx_slot_config;
#[doc = "Used to set transmission parameters for the slot"]
pub use self::tx_slot_parameters::TX_SLOT_PARAMETERS;
#[doc = r"Cluster"]
#[doc = "Used to set transmission parameters for the slot"]
pub mod tx_slot_parameters;
