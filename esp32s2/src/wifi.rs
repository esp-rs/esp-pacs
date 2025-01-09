#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    filter_bank: [FILTER_BANK; 2],
    _reserved1: [u8; 0x08],
    rx_ctrl: RX_CTRL,
    _reserved2: [u8; 0x04],
    rx_dma_list: RX_DMA_LIST,
    _reserved3: [u8; 0x44],
    interface_rx_control: [INTERFACE_RX_CONTROL; 4],
    _reserved4: [u8; 0x0b4c],
    mac_interrupt: MAC_INTERRUPT,
    _reserved5: [u8; 0x58],
    txq_state: TXQ_STATE,
    _reserved6: [u8; 0x0c],
    ctrl: CTRL,
    _reserved7: [u8; 0x20],
    tx_slot_config: [TX_SLOT_CONFIG; 5],
    _reserved8: [u8; 0x0464],
    tx_slot_parameters: [TX_SLOT_PARAMETERS; 5],
    _reserved9: [u8; 0x0f0c],
    pwr_interrupt: PWR_INTERRUPT,
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
    #[doc = "0x88 - Controls the reception of frames"]
    #[inline(always)]
    pub const fn rx_ctrl(&self) -> &RX_CTRL {
        &self.rx_ctrl
    }
    #[doc = "0x90..0x9c - RX_DMA_LIST"]
    #[inline(always)]
    pub const fn rx_dma_list(&self) -> &RX_DMA_LIST {
        &self.rx_dma_list
    }
    #[doc = "0xe0..0xf0 - Controls RX for an interface"]
    #[inline(always)]
    pub const fn interface_rx_control(&self, n: usize) -> &INTERFACE_RX_CONTROL {
        &self.interface_rx_control[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xe0..0xf0 - Controls RX for an interface"]
    #[inline(always)]
    pub fn interface_rx_control_iter(&self) -> impl Iterator<Item = &INTERFACE_RX_CONTROL> {
        self.interface_rx_control.iter()
    }
    #[doc = "0xc3c..0xc44 - Status and clear for the WIFI_MAC interrupt"]
    #[inline(always)]
    pub const fn mac_interrupt(&self) -> &MAC_INTERRUPT {
        &self.mac_interrupt
    }
    #[doc = "0xc9c..0xcac - State of transmission queues"]
    #[inline(always)]
    pub const fn txq_state(&self) -> &TXQ_STATE {
        &self.txq_state
    }
    #[doc = "0xcb8 - Exact name and meaning unknown, used for initializing the MAC"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0xcdc..0xd04 - Used to configure the TX slot."]
    #[inline(always)]
    pub const fn tx_slot_config(&self, n: usize) -> &TX_SLOT_CONFIG {
        &self.tx_slot_config[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xcdc..0xd04 - Used to configure the TX slot."]
    #[inline(always)]
    pub fn tx_slot_config_iter(&self) -> impl Iterator<Item = &TX_SLOT_CONFIG> {
        self.tx_slot_config.iter()
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
    #[doc = "0x21a0..0x21a8 - Status and clear for the WIFI_PWR interrupt"]
    #[inline(always)]
    pub const fn pwr_interrupt(&self) -> &PWR_INTERRUPT {
        &self.pwr_interrupt
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
#[doc = "CTRL (rw) register accessor: Exact name and meaning unknown, used for initializing the MAC\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Exact name and meaning unknown, used for initializing the MAC"]
pub mod ctrl;
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
#[doc = "Status and clear for the WIFI_MAC interrupt"]
pub use self::mac_interrupt::MAC_INTERRUPT;
#[doc = r"Cluster"]
#[doc = "Status and clear for the WIFI_MAC interrupt"]
pub mod mac_interrupt;
#[doc = "Status and clear for the WIFI_PWR interrupt"]
pub use self::pwr_interrupt::PWR_INTERRUPT;
#[doc = r"Cluster"]
#[doc = "Status and clear for the WIFI_PWR interrupt"]
pub mod pwr_interrupt;
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
