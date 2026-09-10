#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    filter_bank: [FILTER_BANK; 2],
    _reserved1: [u8; 0x04],
    rx_ctrl: RX_CTRL,
    rx_dma_list: RX_DMA_LIST,
    _reserved3: [u8; 0x44],
    filter_control: [FILTER_CONTROL; 4],
    _reserved4: [u8; 0x18],
    rx_ctrl_filter: [RX_CTRL_FILTER; 4],
    _reserved5: [u8; 0x06f0],
    crypto_control: CRYPTO_CONTROL,
    _reserved6: [u8; 0x0424],
    mac_interrupt: MAC_INTERRUPT,
    _reserved7: [u8; 0x5c],
    ctrl: CTRL,
    txq_state: TXQ_STATE,
    _reserved9: [u8; 0x30],
    tx_slot_config: [TX_SLOT_CONFIG; 5],
    _reserved10: [u8; 0x04bc],
    plcp1: (),
    _reserved11: [u8; 0x04],
    tx_pti: (),
    _reserved12: [u8; 0x04],
    ht_sig: (),
    _reserved13: [u8; 0x10],
    ht_unknown: (),
    _reserved14: [u8; 0x04],
    plcp2: (),
    _reserved15: [u8; 0x04],
    duration: (),
    _reserved16: [u8; 0x08],
    pmd: (),
    _reserved17: [u8; 0x0210],
    crypto_key_slot: [CRYPTO_KEY_SLOT; 25],
    _reserved18: [u8; 0x0818],
    mac_time: MAC_TIME,
    _reserved19: [u8; 0x0114],
    pwr_interrupt: PWR_INTERRUPT,
}
impl RegisterBlock {
    #[doc = "0x00..0x80 - Filter banks for frame reception. Bank zero is for the BSSID and bank one for the RA. Each filter bank has registers for four interfaces."]
    #[inline(always)]
    pub const fn filter_bank(&self, n: usize) -> &FILTER_BANK {
        &self.filter_bank[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x80 - Filter banks for frame reception. Bank zero is for the BSSID and bank one for the RA. Each filter bank has registers for four interfaces."]
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
    #[doc = "0xd8..0xe8 - Controls the RX filter for an interface"]
    #[inline(always)]
    pub const fn filter_control(&self, n: usize) -> &FILTER_CONTROL {
        &self.filter_control[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xd8..0xe8 - Controls the RX filter for an interface"]
    #[inline(always)]
    pub fn filter_control_iter(&self) -> impl Iterator<Item = &FILTER_CONTROL> {
        self.filter_control.iter()
    }
    #[doc = "0x100..0x110 - Configures which control frames pass the RX filter. Setting a bit lets that frame type pass the filter."]
    #[inline(always)]
    pub const fn rx_ctrl_filter(&self, n: usize) -> &RX_CTRL_FILTER {
        &self.rx_ctrl_filter[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x110 - Configures which control frames pass the RX filter. Setting a bit lets that frame type pass the filter."]
    #[inline(always)]
    pub fn rx_ctrl_filter_iter(&self) -> impl Iterator<Item = &RX_CTRL_FILTER> {
        self.rx_ctrl_filter.iter()
    }
    #[doc = "0x800..0x818 - Control registers for hardware crypto"]
    #[inline(always)]
    pub const fn crypto_control(&self) -> &CRYPTO_CONTROL {
        &self.crypto_control
    }
    #[doc = "0xc3c..0xc44 - Status and clear for the WIFI_MAC interrupt"]
    #[inline(always)]
    pub const fn mac_interrupt(&self) -> &MAC_INTERRUPT {
        &self.mac_interrupt
    }
    #[doc = "0xca0 - Exact name and meaning unknown, used for initializing the MAC"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0xca4..0xcb4 - State of transmission queues"]
    #[inline(always)]
    pub const fn txq_state(&self) -> &TXQ_STATE {
        &self.txq_state
    }
    #[doc = "0xce4..0xd0c - Used to configure the TX slot."]
    #[inline(always)]
    pub const fn tx_slot_config(&self, n: usize) -> &TX_SLOT_CONFIG {
        &self.tx_slot_config[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xce4..0xd0c - Used to configure the TX slot."]
    #[inline(always)]
    pub fn tx_slot_config_iter(&self) -> impl Iterator<Item = &TX_SLOT_CONFIG> {
        self.tx_slot_config.iter()
    }
    #[doc = "0x11c8..0x11dc - PLCP1"]
    #[inline(always)]
    pub const fn plcp1(&self, n: usize) -> &PLCP1 {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4552)
                .add(76 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x11c8..0x11dc - PLCP1"]
    #[inline(always)]
    pub fn plcp1_iter(&self) -> impl Iterator<Item = &PLCP1> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4552)
                .add(76 * n)
                .cast()
        })
    }
    #[doc = "0x11cc..0x11e0 - Coexistence priorities of the TX slot, written by hal_set_tx_pti"]
    #[inline(always)]
    pub const fn tx_pti(&self, n: usize) -> &TX_PTI {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4556)
                .add(76 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x11cc..0x11e0 - Coexistence priorities of the TX slot, written by hal_set_tx_pti"]
    #[inline(always)]
    pub fn tx_pti_iter(&self) -> impl Iterator<Item = &TX_PTI> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4556)
                .add(76 * n)
                .cast()
        })
    }
    #[doc = "0x11d0..0x11e4 - HT-SIG field in HT preamble"]
    #[inline(always)]
    pub const fn ht_sig(&self, n: usize) -> &HT_SIG {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4560)
                .add(76 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x11d0..0x11e4 - HT-SIG field in HT preamble"]
    #[inline(always)]
    pub fn ht_sig_iter(&self) -> impl Iterator<Item = &HT_SIG> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4560)
                .add(76 * n)
                .cast()
        })
    }
    #[doc = "0x11e0..0x11f4 - exact meaning and name unknown, related to HT"]
    #[inline(always)]
    pub const fn ht_unknown(&self, n: usize) -> &HT_UNKNOWN {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4576)
                .add(76 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x11e0..0x11f4 - exact meaning and name unknown, related to HT"]
    #[inline(always)]
    pub fn ht_unknown_iter(&self) -> impl Iterator<Item = &HT_UNKNOWN> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4576)
                .add(76 * n)
                .cast()
        })
    }
    #[doc = "0x11e4..0x11f8 - PLCP2"]
    #[inline(always)]
    pub const fn plcp2(&self, n: usize) -> &PLCP2 {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4580)
                .add(76 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x11e4..0x11f8 - PLCP2"]
    #[inline(always)]
    pub fn plcp2_iter(&self) -> impl Iterator<Item = &PLCP2> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4580)
                .add(76 * n)
                .cast()
        })
    }
    #[doc = "0x11e8..0x11fc - duration of the frame exchange"]
    #[inline(always)]
    pub const fn duration(&self, n: usize) -> &DURATION {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4584)
                .add(76 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x11e8..0x11fc - duration of the frame exchange"]
    #[inline(always)]
    pub fn duration_iter(&self) -> impl Iterator<Item = &DURATION> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4584)
                .add(76 * n)
                .cast()
        })
    }
    #[doc = "0x11f0..0x1204 - TX result of the slot, read by hal_mac_get_txq_pmd"]
    #[inline(always)]
    pub const fn pmd(&self, n: usize) -> &PMD {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4592)
                .add(76 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x11f0..0x1204 - TX result of the slot, read by hal_mac_get_txq_pmd"]
    #[inline(always)]
    pub fn pmd_iter(&self) -> impl Iterator<Item = &PMD> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4592)
                .add(76 * n)
                .cast()
        })
    }
    #[doc = "0x1400..0x17e8 - Cryptographic keys for MPDU encapsulation and decapsulation"]
    #[inline(always)]
    pub const fn crypto_key_slot(&self, n: usize) -> &CRYPTO_KEY_SLOT {
        &self.crypto_key_slot[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1400..0x17e8 - Cryptographic keys for MPDU encapsulation and decapsulation"]
    #[inline(always)]
    pub fn crypto_key_slot_iter(&self) -> impl Iterator<Item = &CRYPTO_KEY_SLOT> {
        self.crypto_key_slot.iter()
    }
    #[doc = "0x2000 - Current value of the MAC timer"]
    #[inline(always)]
    pub const fn mac_time(&self) -> &MAC_TIME {
        &self.mac_time
    }
    #[doc = "0x2118..0x2120 - Status and clear for the WIFI_PWR interrupt"]
    #[inline(always)]
    pub const fn pwr_interrupt(&self) -> &PWR_INTERRUPT {
        &self.pwr_interrupt
    }
}
#[doc = "Filter banks for frame reception. Bank zero is for the BSSID and bank one for the RA. Each filter bank has registers for four interfaces."]
pub use self::filter_bank::FILTER_BANK;
#[doc = r"Cluster"]
#[doc = "Filter banks for frame reception. Bank zero is for the BSSID and bank one for the RA. Each filter bank has registers for four interfaces."]
pub mod filter_bank;
#[doc = "RX_CTRL (rw) register accessor: Controls the reception of frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ctrl`] module"]
pub type RX_CTRL = crate::Reg<rx_ctrl::RX_CTRL_SPEC>;
#[doc = "Controls the reception of frames"]
pub mod rx_ctrl;
#[doc = "RX_DMA_LIST"]
pub use self::rx_dma_list::RX_DMA_LIST;
#[doc = r"Cluster"]
#[doc = "RX_DMA_LIST"]
pub mod rx_dma_list;
#[doc = "FILTER_CONTROL (rw) register accessor: Controls the RX filter for an interface\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_control`] module"]
pub type FILTER_CONTROL = crate::Reg<filter_control::FILTER_CONTROL_SPEC>;
#[doc = "Controls the RX filter for an interface"]
pub mod filter_control;
#[doc = "RX_CTRL_FILTER (rw) register accessor: Configures which control frames pass the RX filter. Setting a bit lets that frame type pass the filter.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ctrl_filter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ctrl_filter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ctrl_filter`] module"]
pub type RX_CTRL_FILTER = crate::Reg<rx_ctrl_filter::RX_CTRL_FILTER_SPEC>;
#[doc = "Configures which control frames pass the RX filter. Setting a bit lets that frame type pass the filter."]
pub mod rx_ctrl_filter;
#[doc = "Control registers for hardware crypto"]
pub use self::crypto_control::CRYPTO_CONTROL;
#[doc = r"Cluster"]
#[doc = "Control registers for hardware crypto"]
pub mod crypto_control;
#[doc = "Status and clear for the WIFI_MAC interrupt"]
pub use self::mac_interrupt::MAC_INTERRUPT;
#[doc = r"Cluster"]
#[doc = "Status and clear for the WIFI_MAC interrupt"]
pub mod mac_interrupt;
#[doc = "CTRL (rw) register accessor: Exact name and meaning unknown, used for initializing the MAC\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Exact name and meaning unknown, used for initializing the MAC"]
pub mod ctrl;
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
#[doc = "PLCP1 (rw) register accessor: PLCP1\n\nYou can [`read`](crate::Reg::read) this register and get [`plcp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plcp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plcp1`] module"]
pub type PLCP1 = crate::Reg<plcp1::PLCP1_SPEC>;
#[doc = "PLCP1"]
pub mod plcp1;
#[doc = "HT_SIG (rw) register accessor: HT-SIG field in HT preamble\n\nYou can [`read`](crate::Reg::read) this register and get [`ht_sig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ht_sig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ht_sig`] module"]
pub type HT_SIG = crate::Reg<ht_sig::HT_SIG_SPEC>;
#[doc = "HT-SIG field in HT preamble"]
pub mod ht_sig;
#[doc = "HT_UNKNOWN (rw) register accessor: exact meaning and name unknown, related to HT\n\nYou can [`read`](crate::Reg::read) this register and get [`ht_unknown::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ht_unknown::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ht_unknown`] module"]
pub type HT_UNKNOWN = crate::Reg<ht_unknown::HT_UNKNOWN_SPEC>;
#[doc = "exact meaning and name unknown, related to HT"]
pub mod ht_unknown;
#[doc = "PLCP2 (rw) register accessor: PLCP2\n\nYou can [`read`](crate::Reg::read) this register and get [`plcp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plcp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plcp2`] module"]
pub type PLCP2 = crate::Reg<plcp2::PLCP2_SPEC>;
#[doc = "PLCP2"]
pub mod plcp2;
#[doc = "DURATION (rw) register accessor: duration of the frame exchange\n\nYou can [`read`](crate::Reg::read) this register and get [`duration::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`duration::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@duration`] module"]
pub type DURATION = crate::Reg<duration::DURATION_SPEC>;
#[doc = "duration of the frame exchange"]
pub mod duration;
#[doc = "PMD (rw) register accessor: TX result of the slot, read by hal_mac_get_txq_pmd\n\nYou can [`read`](crate::Reg::read) this register and get [`pmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmd`] module"]
pub type PMD = crate::Reg<pmd::PMD_SPEC>;
#[doc = "TX result of the slot, read by hal_mac_get_txq_pmd"]
pub mod pmd;
#[doc = "Cryptographic keys for MPDU encapsulation and decapsulation"]
pub use self::crypto_key_slot::CRYPTO_KEY_SLOT;
#[doc = r"Cluster"]
#[doc = "Cryptographic keys for MPDU encapsulation and decapsulation"]
pub mod crypto_key_slot;
#[doc = "TX_PTI (rw) register accessor: Coexistence priorities of the TX slot, written by hal_set_tx_pti\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_pti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_pti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_pti`] module"]
pub type TX_PTI = crate::Reg<tx_pti::TX_PTI_SPEC>;
#[doc = "Coexistence priorities of the TX slot, written by hal_set_tx_pti"]
pub mod tx_pti;
#[doc = "MAC_TIME (rw) register accessor: Current value of the MAC timer\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_time`] module"]
pub type MAC_TIME = crate::Reg<mac_time::MAC_TIME_SPEC>;
#[doc = "Current value of the MAC timer"]
pub mod mac_time;
#[doc = "Status and clear for the WIFI_PWR interrupt"]
pub use self::pwr_interrupt::PWR_INTERRUPT;
#[doc = r"Cluster"]
#[doc = "Status and clear for the WIFI_PWR interrupt"]
pub mod pwr_interrupt;
