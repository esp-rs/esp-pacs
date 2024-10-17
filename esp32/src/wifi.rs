#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    bssid_filter_addr_low: BSSID_FILTER_ADDR_LOW,
    bssid_filter_addr_high: BSSID_FILTER_ADDR_HIGH,
    _reserved2: [u8; 0x18],
    bssid_filter_addr_mask_low: BSSID_FILTER_ADDR_MASK_LOW,
    bssid_filter_addr_mask_high: BSSID_FILTER_ADDR_MASK_HIGH,
    _reserved4: [u8; 0x18],
    ra_filter_addr_low: RA_FILTER_ADDR_LOW,
    ra_filter_addr_high: RA_FILTER_ADDR_HIGH,
    _reserved6: [u8; 0x18],
    ra_filter_addr_mask_low: RA_FILTER_ADDR_MASK_LOW,
    ra_filter_addr_mask_high: RA_FILTER_ADDR_MASK_HIGH,
    _reserved8: [u8; 0x1c],
    rx_ctrl: RX_CTRL,
    rx_descr_base: RX_DESCR_BASE,
    rx_descr_next: RX_DESCR_NEXT,
    rx_descr_last: RX_DESCR_LAST,
    _reserved12: [u8; 0x0bb4],
    wifi_int_status: WIFI_INT_STATUS,
    wifi_int_clear: WIFI_INT_CLEAR,
    _reserved14: [u8; 0x68],
    ctrl_reg: CTRL_REG,
    tx_error_clear: TX_ERROR_CLEAR,
    tx_error_status: TX_ERROR_STATUS,
    tx_complete_clear: TX_COMPLETE_CLEAR,
    tx_complete_status: TX_COMPLETE_STATUS,
    _reserved19: [u8; 0x30],
    tx_slot_config: [TX_SLOT_CONFIG; 5],
    _reserved20: [u8; 0x0444],
    tx_slot_parameters: [TX_SLOT_PARAMETERS; 5],
}
impl RegisterBlock {
    #[doc = "0x00 - First 4 bytes of BSSID MAC address filter"]
    #[inline(always)]
    pub const fn bssid_filter_addr_low(&self) -> &BSSID_FILTER_ADDR_LOW {
        &self.bssid_filter_addr_low
    }
    #[doc = "0x04 - last 2 bytes of BSSID MAC address filter"]
    #[inline(always)]
    pub const fn bssid_filter_addr_high(&self) -> &BSSID_FILTER_ADDR_HIGH {
        &self.bssid_filter_addr_high
    }
    #[doc = "0x20 - mask applied to BSSID MAC address filter"]
    #[inline(always)]
    pub const fn bssid_filter_addr_mask_low(&self) -> &BSSID_FILTER_ADDR_MASK_LOW {
        &self.bssid_filter_addr_mask_low
    }
    #[doc = "0x24 - mask applied to BSSID MAC address filter"]
    #[inline(always)]
    pub const fn bssid_filter_addr_mask_high(&self) -> &BSSID_FILTER_ADDR_MASK_HIGH {
        &self.bssid_filter_addr_mask_high
    }
    #[doc = "0x40 - first 4 bytes of RA MAC address filter"]
    #[inline(always)]
    pub const fn ra_filter_addr_low(&self) -> &RA_FILTER_ADDR_LOW {
        &self.ra_filter_addr_low
    }
    #[doc = "0x44 - last 2 bytes of RA MAC address filter"]
    #[inline(always)]
    pub const fn ra_filter_addr_high(&self) -> &RA_FILTER_ADDR_HIGH {
        &self.ra_filter_addr_high
    }
    #[doc = "0x60 - mask applied to RA MAC address filter"]
    #[inline(always)]
    pub const fn ra_filter_addr_mask_low(&self) -> &RA_FILTER_ADDR_MASK_LOW {
        &self.ra_filter_addr_mask_low
    }
    #[doc = "0x64 - mask applied to RA MAC address filter"]
    #[inline(always)]
    pub const fn ra_filter_addr_mask_high(&self) -> &RA_FILTER_ADDR_MASK_HIGH {
        &self.ra_filter_addr_mask_high
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
    #[doc = "0xcb8 - Exact name and meaning unknown, used initializing the MAC"]
    #[inline(always)]
    pub const fn ctrl_reg(&self) -> &CTRL_REG {
        &self.ctrl_reg
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
    #[doc = "0xcc4 - "]
    #[inline(always)]
    pub const fn tx_complete_clear(&self) -> &TX_COMPLETE_CLEAR {
        &self.tx_complete_clear
    }
    #[doc = "0xcc8 - "]
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
}
#[doc = "BSSID_FILTER_ADDR_LOW (rw) register accessor: First 4 bytes of BSSID MAC address filter\n\nYou can [`read`](crate::Reg::read) this register and get [`bssid_filter_addr_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bssid_filter_addr_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bssid_filter_addr_low`] module"]
pub type BSSID_FILTER_ADDR_LOW = crate::Reg<bssid_filter_addr_low::BSSID_FILTER_ADDR_LOW_SPEC>;
#[doc = "First 4 bytes of BSSID MAC address filter"]
pub mod bssid_filter_addr_low;
#[doc = "BSSID_FILTER_ADDR_HIGH (rw) register accessor: last 2 bytes of BSSID MAC address filter\n\nYou can [`read`](crate::Reg::read) this register and get [`bssid_filter_addr_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bssid_filter_addr_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bssid_filter_addr_high`] module"]
pub type BSSID_FILTER_ADDR_HIGH = crate::Reg<bssid_filter_addr_high::BSSID_FILTER_ADDR_HIGH_SPEC>;
#[doc = "last 2 bytes of BSSID MAC address filter"]
pub mod bssid_filter_addr_high;
#[doc = "BSSID_FILTER_ADDR_MASK_LOW (rw) register accessor: mask applied to BSSID MAC address filter\n\nYou can [`read`](crate::Reg::read) this register and get [`bssid_filter_addr_mask_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bssid_filter_addr_mask_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bssid_filter_addr_mask_low`] module"]
pub type BSSID_FILTER_ADDR_MASK_LOW =
    crate::Reg<bssid_filter_addr_mask_low::BSSID_FILTER_ADDR_MASK_LOW_SPEC>;
#[doc = "mask applied to BSSID MAC address filter"]
pub mod bssid_filter_addr_mask_low;
#[doc = "BSSID_FILTER_ADDR_MASK_HIGH (rw) register accessor: mask applied to BSSID MAC address filter\n\nYou can [`read`](crate::Reg::read) this register and get [`bssid_filter_addr_mask_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bssid_filter_addr_mask_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bssid_filter_addr_mask_high`] module"]
pub type BSSID_FILTER_ADDR_MASK_HIGH =
    crate::Reg<bssid_filter_addr_mask_high::BSSID_FILTER_ADDR_MASK_HIGH_SPEC>;
#[doc = "mask applied to BSSID MAC address filter"]
pub mod bssid_filter_addr_mask_high;
#[doc = "RA_FILTER_ADDR_LOW (rw) register accessor: first 4 bytes of RA MAC address filter\n\nYou can [`read`](crate::Reg::read) this register and get [`ra_filter_addr_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ra_filter_addr_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ra_filter_addr_low`] module"]
pub type RA_FILTER_ADDR_LOW = crate::Reg<ra_filter_addr_low::RA_FILTER_ADDR_LOW_SPEC>;
#[doc = "first 4 bytes of RA MAC address filter"]
pub mod ra_filter_addr_low;
#[doc = "RA_FILTER_ADDR_HIGH (rw) register accessor: last 2 bytes of RA MAC address filter\n\nYou can [`read`](crate::Reg::read) this register and get [`ra_filter_addr_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ra_filter_addr_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ra_filter_addr_high`] module"]
pub type RA_FILTER_ADDR_HIGH = crate::Reg<ra_filter_addr_high::RA_FILTER_ADDR_HIGH_SPEC>;
#[doc = "last 2 bytes of RA MAC address filter"]
pub mod ra_filter_addr_high;
#[doc = "RA_FILTER_ADDR_MASK_LOW (rw) register accessor: mask applied to RA MAC address filter\n\nYou can [`read`](crate::Reg::read) this register and get [`ra_filter_addr_mask_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ra_filter_addr_mask_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ra_filter_addr_mask_low`] module"]
pub type RA_FILTER_ADDR_MASK_LOW =
    crate::Reg<ra_filter_addr_mask_low::RA_FILTER_ADDR_MASK_LOW_SPEC>;
#[doc = "mask applied to RA MAC address filter"]
pub mod ra_filter_addr_mask_low;
#[doc = "RA_FILTER_ADDR_MASK_HIGH (rw) register accessor: mask applied to RA MAC address filter\n\nYou can [`read`](crate::Reg::read) this register and get [`ra_filter_addr_mask_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ra_filter_addr_mask_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ra_filter_addr_mask_high`] module"]
pub type RA_FILTER_ADDR_MASK_HIGH =
    crate::Reg<ra_filter_addr_mask_high::RA_FILTER_ADDR_MASK_HIGH_SPEC>;
#[doc = "mask applied to RA MAC address filter"]
pub mod ra_filter_addr_mask_high;
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
#[doc = "WIFI_INT_STATUS (rw) register accessor: Interrupt status of WIFI peripheral\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_int_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_int_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_int_status`] module"]
pub type WIFI_INT_STATUS = crate::Reg<wifi_int_status::WIFI_INT_STATUS_SPEC>;
#[doc = "Interrupt status of WIFI peripheral"]
pub mod wifi_int_status;
#[doc = "WIFI_INT_CLEAR (rw) register accessor: Interrupt status clear of WIFI peripheral\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_int_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_int_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_int_clear`] module"]
pub type WIFI_INT_CLEAR = crate::Reg<wifi_int_clear::WIFI_INT_CLEAR_SPEC>;
#[doc = "Interrupt status clear of WIFI peripheral"]
pub mod wifi_int_clear;
#[doc = "CTRL_REG (rw) register accessor: Exact name and meaning unknown, used initializing the MAC\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_reg`] module"]
pub type CTRL_REG = crate::Reg<ctrl_reg::CTRL_REG_SPEC>;
#[doc = "Exact name and meaning unknown, used initializing the MAC"]
pub mod ctrl_reg;
#[doc = "TX_ERROR_CLEAR (rw) register accessor: Clear the error status of a slot\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_error_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_error_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_error_clear`] module"]
pub type TX_ERROR_CLEAR = crate::Reg<tx_error_clear::TX_ERROR_CLEAR_SPEC>;
#[doc = "Clear the error status of a slot"]
pub mod tx_error_clear;
#[doc = "TX_ERROR_STATUS (rw) register accessor: Error status of a slot\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_error_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_error_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_error_status`] module"]
pub type TX_ERROR_STATUS = crate::Reg<tx_error_status::TX_ERROR_STATUS_SPEC>;
#[doc = "Error status of a slot"]
pub mod tx_error_status;
#[doc = "TX_COMPLETE_CLEAR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`tx_complete_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_complete_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_complete_clear`] module"]
pub type TX_COMPLETE_CLEAR = crate::Reg<tx_complete_clear::TX_COMPLETE_CLEAR_SPEC>;
#[doc = ""]
pub mod tx_complete_clear;
#[doc = "TX_COMPLETE_STATUS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`tx_complete_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_complete_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_complete_status`] module"]
pub type TX_COMPLETE_STATUS = crate::Reg<tx_complete_status::TX_COMPLETE_STATUS_SPEC>;
#[doc = ""]
pub mod tx_complete_status;
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
