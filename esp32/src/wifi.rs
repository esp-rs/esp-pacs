#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    filter_bank: [FILTER_BANK; 2],
    _reserved1: [u8; 0x04],
    rx_ctrl: RX_CTRL,
    rx_descr_base: RX_DESCR_BASE,
    rx_descr_next: RX_DESCR_NEXT,
    rx_descr_last: RX_DESCR_LAST,
    _reserved5: [u8; 0x44],
    unknown_rx_policy: [UNKNOWN_RX_POLICY; 2],
    _reserved6: [u8; 0x0b68],
    wifi_int_status: WIFI_INT_STATUS,
    wifi_int_clear: WIFI_INT_CLEAR,
    _reserved8: [u8; 0x68],
    ctrl: CTRL,
    tx_error_clear: TX_ERROR_CLEAR,
    tx_error_status: TX_ERROR_STATUS,
    tx_complete_clear: TX_COMPLETE_CLEAR,
    tx_complete_status: TX_COMPLETE_STATUS,
    _reserved13: [u8; 0x30],
    tx_slot_config: [TX_SLOT_CONFIG; 5],
    _reserved14: [u8; 0x0444],
    tx_slot_parameters: [TX_SLOT_PARAMETERS; 5],
    _reserved15: [u8; 0x016c],
    crypto_key_entry: [CRYPTO_KEY_ENTRY; 16],
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
    #[doc = "0x88 - base address of the RX DMA list"]
    #[inline(always)]
    pub const fn rx_descr_base(&self) -> &RX_DESCR_BASE {
        &self.rx_descr_base
    }
    #[doc = "0x8c - next item in the RX DMA list"]
    #[inline(always)]
    pub const fn rx_descr_next(&self) -> &RX_DESCR_NEXT {
        &self.rx_descr_next
    }
    #[doc = "0x90 - last item in RX DMA list"]
    #[inline(always)]
    pub const fn rx_descr_last(&self) -> &RX_DESCR_LAST {
        &self.rx_descr_last
    }
    #[doc = "0xd8..0xe0 - "]
    #[inline(always)]
    pub const fn unknown_rx_policy(&self, n: usize) -> &UNKNOWN_RX_POLICY {
        &self.unknown_rx_policy[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xd8..0xe0 - "]
    #[inline(always)]
    pub fn unknown_rx_policy_iter(&self) -> impl Iterator<Item = &UNKNOWN_RX_POLICY> {
        self.unknown_rx_policy.iter()
    }
    #[doc = "0xc48 - Interrupt status of WIFI peripheral"]
    #[inline(always)]
    pub const fn wifi_int_status(&self) -> &WIFI_INT_STATUS {
        &self.wifi_int_status
    }
    #[doc = "0xc4c - Interrupt status clear of WIFI peripheral"]
    #[inline(always)]
    pub const fn wifi_int_clear(&self) -> &WIFI_INT_CLEAR {
        &self.wifi_int_clear
    }
    #[doc = "0xcb8 - Exact name and meaning unknown, used for initializing the MAC"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0xcbc - Clear the error status of a slot"]
    #[inline(always)]
    pub const fn tx_error_clear(&self) -> &TX_ERROR_CLEAR {
        &self.tx_error_clear
    }
    #[doc = "0xcc0 - Error status of a slot"]
    #[inline(always)]
    pub const fn tx_error_status(&self) -> &TX_ERROR_STATUS {
        &self.tx_error_status
    }
    #[doc = "0xcc4 - Clear the completion status of a slot"]
    #[inline(always)]
    pub const fn tx_complete_clear(&self) -> &TX_COMPLETE_CLEAR {
        &self.tx_complete_clear
    }
    #[doc = "0xcc8 - Completion status of a slot"]
    #[inline(always)]
    pub const fn tx_complete_status(&self) -> &TX_COMPLETE_STATUS {
        &self.tx_complete_status
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
    #[doc = "0x1400..0x1680 - The cryptographic keys, to be used by the MAC"]
    #[inline(always)]
    pub const fn crypto_key_entry(&self, n: usize) -> &CRYPTO_KEY_ENTRY {
        &self.crypto_key_entry[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1400..0x1680 - The cryptographic keys, to be used by the MAC"]
    #[inline(always)]
    pub fn crypto_key_entry_iter(&self) -> impl Iterator<Item = &CRYPTO_KEY_ENTRY> {
        self.crypto_key_entry.iter()
    }
}
#[doc = "RX_CTRL (rw) register accessor: Controls the reception of frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ctrl`] module"]
pub type RX_CTRL = crate::Reg<rx_ctrl::RX_CTRL_SPEC>;
#[doc = "Controls the reception of frames"]
pub mod rx_ctrl;
#[doc = "RX_DESCR_BASE (rw) register accessor: base address of the RX DMA list\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_descr_base::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_descr_base::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_descr_base`] module"]
pub type RX_DESCR_BASE = crate::Reg<rx_descr_base::RX_DESCR_BASE_SPEC>;
#[doc = "base address of the RX DMA list"]
pub mod rx_descr_base;
#[doc = "RX_DESCR_NEXT (rw) register accessor: next item in the RX DMA list\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_descr_next::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_descr_next::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_descr_next`] module"]
pub type RX_DESCR_NEXT = crate::Reg<rx_descr_next::RX_DESCR_NEXT_SPEC>;
#[doc = "next item in the RX DMA list"]
pub mod rx_descr_next;
#[doc = "RX_DESCR_LAST (rw) register accessor: last item in RX DMA list\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_descr_last::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_descr_last::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_descr_last`] module"]
pub type RX_DESCR_LAST = crate::Reg<rx_descr_last::RX_DESCR_LAST_SPEC>;
#[doc = "last item in RX DMA list"]
pub mod rx_descr_last;
#[doc = "UNKNOWN_RX_POLICY (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`unknown_rx_policy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unknown_rx_policy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unknown_rx_policy`] module"]
pub type UNKNOWN_RX_POLICY = crate::Reg<unknown_rx_policy::UNKNOWN_RX_POLICY_SPEC>;
#[doc = ""]
pub mod unknown_rx_policy;
#[doc = "WIFI_INT_STATUS (rw) register accessor: Interrupt status of WIFI peripheral\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_int_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_int_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_int_status`] module"]
pub type WIFI_INT_STATUS = crate::Reg<wifi_int_status::WIFI_INT_STATUS_SPEC>;
#[doc = "Interrupt status of WIFI peripheral"]
pub mod wifi_int_status;
#[doc = "WIFI_INT_CLEAR (rw) register accessor: Interrupt status clear of WIFI peripheral\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_int_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_int_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_int_clear`] module"]
pub type WIFI_INT_CLEAR = crate::Reg<wifi_int_clear::WIFI_INT_CLEAR_SPEC>;
#[doc = "Interrupt status clear of WIFI peripheral"]
pub mod wifi_int_clear;
#[doc = "CTRL (rw) register accessor: Exact name and meaning unknown, used for initializing the MAC\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Exact name and meaning unknown, used for initializing the MAC"]
pub mod ctrl;
#[doc = "TX_ERROR_CLEAR (rw) register accessor: Clear the error status of a slot\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_error_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_error_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_error_clear`] module"]
pub type TX_ERROR_CLEAR = crate::Reg<tx_error_clear::TX_ERROR_CLEAR_SPEC>;
#[doc = "Clear the error status of a slot"]
pub mod tx_error_clear;
#[doc = "TX_ERROR_STATUS (rw) register accessor: Error status of a slot\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_error_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_error_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_error_status`] module"]
pub type TX_ERROR_STATUS = crate::Reg<tx_error_status::TX_ERROR_STATUS_SPEC>;
#[doc = "Error status of a slot"]
pub mod tx_error_status;
#[doc = "TX_COMPLETE_CLEAR (rw) register accessor: Clear the completion status of a slot\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_complete_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_complete_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_complete_clear`] module"]
pub type TX_COMPLETE_CLEAR = crate::Reg<tx_complete_clear::TX_COMPLETE_CLEAR_SPEC>;
#[doc = "Clear the completion status of a slot"]
pub mod tx_complete_clear;
#[doc = "TX_COMPLETE_STATUS (rw) register accessor: Completion status of a slot\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_complete_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_complete_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_complete_status`] module"]
pub type TX_COMPLETE_STATUS = crate::Reg<tx_complete_status::TX_COMPLETE_STATUS_SPEC>;
#[doc = "Completion status of a slot"]
pub mod tx_complete_status;
#[doc = "Filter banks for frame reception. Bank zero is for the BSSID and bank one for the RA. Each filter bank has registers for two interfaces."]
pub use self::filter_bank::FILTER_BANK;
#[doc = r"Cluster"]
#[doc = "Filter banks for frame reception. Bank zero is for the BSSID and bank one for the RA. Each filter bank has registers for two interfaces."]
pub mod filter_bank;
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
#[doc = "The cryptographic keys, to be used by the MAC"]
pub use self::crypto_key_entry::CRYPTO_KEY_ENTRY;
#[doc = r"Cluster"]
#[doc = "The cryptographic keys, to be used by the MAC"]
pub mod crypto_key_entry;
