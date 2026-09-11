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
    _reserved19: [u8; 0x08],
    tsf_ctrl: TSF_CTRL,
    tsf_load_low: TSF_LOAD_LOW,
    tsf_load_high: TSF_LOAD_HIGH,
    tsf_time_low: TSF_TIME_LOW,
    tsf_time_high: TSF_TIME_HIGH,
    tbtt_start: TBTT_START,
    _reserved25: [u8; 0x04],
    tsf_cfg: (),
    _reserved26: [u8; 0x08],
    tbtt_cfg: (),
    _reserved27: [u8; 0x2c],
    tsf_timer_cfg: (),
    _reserved28: [u8; 0x04],
    tsf_timer_target: (),
    _reserved29: [u8; 0xb0],
    pwr_int_enable: PWR_INT_ENABLE,
    _reserved30: [u8; 0x04],
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
    #[doc = "0x200c - Control for the per-interface TSF counters. Written by tsf_hal_get_counter_value, tsf_hal_set_counter_value and tsf_hal_set_tbtt_start_time."]
    #[inline(always)]
    pub const fn tsf_ctrl(&self) -> &TSF_CTRL {
        &self.tsf_ctrl
    }
    #[doc = "0x2010 - Low word of the value loaded into a TSF counter"]
    #[inline(always)]
    pub const fn tsf_load_low(&self) -> &TSF_LOAD_LOW {
        &self.tsf_load_low
    }
    #[doc = "0x2014 - High word of the value loaded into a TSF counter"]
    #[inline(always)]
    pub const fn tsf_load_high(&self) -> &TSF_LOAD_HIGH {
        &self.tsf_load_high
    }
    #[doc = "0x2018 - Low word of the latched TSF counter"]
    #[inline(always)]
    pub const fn tsf_time_low(&self) -> &TSF_TIME_LOW {
        &self.tsf_time_low
    }
    #[doc = "0x201c - High word of the latched TSF counter"]
    #[inline(always)]
    pub const fn tsf_time_high(&self) -> &TSF_TIME_HIGH {
        &self.tsf_time_high
    }
    #[doc = "0x2020 - TBTT start time, loaded into an interface with TSF_CTRL.LOAD_TBTT_START"]
    #[inline(always)]
    pub const fn tbtt_start(&self) -> &TBTT_START {
        &self.tbtt_start
    }
    #[doc = "0x2028..0x2038 - TSF and TBTT configuration of an interface"]
    #[inline(always)]
    pub const fn tsf_cfg(&self, n: usize) -> &TSF_CFG {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8232)
                .add(12 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2028..0x2038 - TSF and TBTT configuration of an interface"]
    #[inline(always)]
    pub fn tsf_cfg_iter(&self) -> impl Iterator<Item = &TSF_CFG> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8232)
                .add(12 * n)
                .cast()
        })
    }
    #[doc = "0x2030..0x2040 - TBTT interval and early time of an interface"]
    #[inline(always)]
    pub const fn tbtt_cfg(&self, n: usize) -> &TBTT_CFG {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8240)
                .add(12 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2030..0x2040 - TBTT interval and early time of an interface"]
    #[inline(always)]
    pub fn tbtt_cfg_iter(&self) -> impl Iterator<Item = &TBTT_CFG> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8240)
                .add(12 * n)
                .cast()
        })
    }
    #[doc = "0x205c..0x206c - Configuration of a TSF timer"]
    #[inline(always)]
    pub const fn tsf_timer_cfg(&self, n: usize) -> &TSF_TIMER_CFG {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8284)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x205c..0x206c - Configuration of a TSF timer"]
    #[inline(always)]
    pub fn tsf_timer_cfg_iter(&self) -> impl Iterator<Item = &TSF_TIMER_CFG> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8284)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x2060..0x2070 - Target of a TSF timer, written by tsf_hal_set_timer_target"]
    #[inline(always)]
    pub const fn tsf_timer_target(&self, n: usize) -> &TSF_TIMER_TARGET {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8288)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2060..0x2070 - Target of a TSF timer, written by tsf_hal_set_timer_target"]
    #[inline(always)]
    pub fn tsf_timer_target_iter(&self) -> impl Iterator<Item = &TSF_TIMER_TARGET> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8288)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x2110 - Interrupt enable for the WIFI_PWR interrupt. TBTT of interface n is bit (4 - n), TSF timer n is bit (8 - n)."]
    #[inline(always)]
    pub const fn pwr_int_enable(&self) -> &PWR_INT_ENABLE {
        &self.pwr_int_enable
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
#[doc = "TSF_CTRL (rw) register accessor: Control for the per-interface TSF counters. Written by tsf_hal_get_counter_value, tsf_hal_set_counter_value and tsf_hal_set_tbtt_start_time.\n\nYou can [`read`](crate::Reg::read) this register and get [`tsf_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsf_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsf_ctrl`] module"]
pub type TSF_CTRL = crate::Reg<tsf_ctrl::TSF_CTRL_SPEC>;
#[doc = "Control for the per-interface TSF counters. Written by tsf_hal_get_counter_value, tsf_hal_set_counter_value and tsf_hal_set_tbtt_start_time."]
pub mod tsf_ctrl;
#[doc = "TSF_LOAD_LOW (rw) register accessor: Low word of the value loaded into a TSF counter\n\nYou can [`read`](crate::Reg::read) this register and get [`tsf_load_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsf_load_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsf_load_low`] module"]
pub type TSF_LOAD_LOW = crate::Reg<tsf_load_low::TSF_LOAD_LOW_SPEC>;
#[doc = "Low word of the value loaded into a TSF counter"]
pub mod tsf_load_low;
#[doc = "TSF_LOAD_HIGH (rw) register accessor: High word of the value loaded into a TSF counter\n\nYou can [`read`](crate::Reg::read) this register and get [`tsf_load_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsf_load_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsf_load_high`] module"]
pub type TSF_LOAD_HIGH = crate::Reg<tsf_load_high::TSF_LOAD_HIGH_SPEC>;
#[doc = "High word of the value loaded into a TSF counter"]
pub mod tsf_load_high;
#[doc = "TSF_TIME_LOW (rw) register accessor: Low word of the latched TSF counter\n\nYou can [`read`](crate::Reg::read) this register and get [`tsf_time_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsf_time_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsf_time_low`] module"]
pub type TSF_TIME_LOW = crate::Reg<tsf_time_low::TSF_TIME_LOW_SPEC>;
#[doc = "Low word of the latched TSF counter"]
pub mod tsf_time_low;
#[doc = "TSF_TIME_HIGH (rw) register accessor: High word of the latched TSF counter\n\nYou can [`read`](crate::Reg::read) this register and get [`tsf_time_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsf_time_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsf_time_high`] module"]
pub type TSF_TIME_HIGH = crate::Reg<tsf_time_high::TSF_TIME_HIGH_SPEC>;
#[doc = "High word of the latched TSF counter"]
pub mod tsf_time_high;
#[doc = "TBTT_START (rw) register accessor: TBTT start time, loaded into an interface with TSF_CTRL.LOAD_TBTT_START\n\nYou can [`read`](crate::Reg::read) this register and get [`tbtt_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbtt_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbtt_start`] module"]
pub type TBTT_START = crate::Reg<tbtt_start::TBTT_START_SPEC>;
#[doc = "TBTT start time, loaded into an interface with TSF_CTRL.LOAD_TBTT_START"]
pub mod tbtt_start;
#[doc = "TSF_CFG (rw) register accessor: TSF and TBTT configuration of an interface\n\nYou can [`read`](crate::Reg::read) this register and get [`tsf_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsf_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsf_cfg`] module"]
pub type TSF_CFG = crate::Reg<tsf_cfg::TSF_CFG_SPEC>;
#[doc = "TSF and TBTT configuration of an interface"]
pub mod tsf_cfg;
#[doc = "TBTT_CFG (rw) register accessor: TBTT interval and early time of an interface\n\nYou can [`read`](crate::Reg::read) this register and get [`tbtt_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbtt_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbtt_cfg`] module"]
pub type TBTT_CFG = crate::Reg<tbtt_cfg::TBTT_CFG_SPEC>;
#[doc = "TBTT interval and early time of an interface"]
pub mod tbtt_cfg;
#[doc = "TSF_TIMER_CFG (rw) register accessor: Configuration of a TSF timer\n\nYou can [`read`](crate::Reg::read) this register and get [`tsf_timer_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsf_timer_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsf_timer_cfg`] module"]
pub type TSF_TIMER_CFG = crate::Reg<tsf_timer_cfg::TSF_TIMER_CFG_SPEC>;
#[doc = "Configuration of a TSF timer"]
pub mod tsf_timer_cfg;
#[doc = "TSF_TIMER_TARGET (rw) register accessor: Target of a TSF timer, written by tsf_hal_set_timer_target\n\nYou can [`read`](crate::Reg::read) this register and get [`tsf_timer_target::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsf_timer_target::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsf_timer_target`] module"]
pub type TSF_TIMER_TARGET = crate::Reg<tsf_timer_target::TSF_TIMER_TARGET_SPEC>;
#[doc = "Target of a TSF timer, written by tsf_hal_set_timer_target"]
pub mod tsf_timer_target;
#[doc = "PWR_INT_ENABLE (rw) register accessor: Interrupt enable for the WIFI_PWR interrupt. TBTT of interface n is bit (4 - n), TSF timer n is bit (8 - n).\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_int_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_int_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_int_enable`] module"]
pub type PWR_INT_ENABLE = crate::Reg<pwr_int_enable::PWR_INT_ENABLE_SPEC>;
#[doc = "Interrupt enable for the WIFI_PWR interrupt. TBTT of interface n is bit (4 - n), TSF timer n is bit (8 - n)."]
pub mod pwr_int_enable;
#[doc = "Status and clear for the WIFI_PWR interrupt"]
pub use self::pwr_interrupt::PWR_INTERRUPT;
#[doc = r"Cluster"]
#[doc = "Status and clear for the WIFI_PWR interrupt"]
pub mod pwr_interrupt;
