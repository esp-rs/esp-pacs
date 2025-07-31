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
    _reserved4: [u8; 0x10],
    rx_ctrl_filter: [RX_CTRL_FILTER; 4],
    _reserved5: [u8; 0x01b4],
    hw_stat_ack_int: HW_STAT_ACK_INT,
    hw_stat_rts_int: HW_STAT_RTS_INT,
    hw_stat_cts_int: HW_STAT_CTS_INT,
    hw_stat_rifs_int: HW_STAT_RIFS_INT,
    hw_stat_rx_success: HW_STAT_RX_SUCCESS,
    hw_stat_rx_end: HW_STAT_RX_END,
    _reserved11: [u8; 0x04],
    hw_stat_hop_err: HW_STAT_HOP_ERR,
    hw_stat_full2: HW_STAT_FULL2,
    hw_stat_block_err: HW_STAT_BLOCK_ERR,
    _reserved14: [u8; 0x051c],
    crypto_control: CRYPTO_CONTROL,
    _reserved15: [u8; 0x03e8],
    mac_time: MAC_TIME,
    _reserved16: [u8; 0x44],
    mac_interrupt: MAC_INTERRUPT,
    _reserved17: [u8; 0x68],
    ctrl: CTRL,
    txq_state: TXQ_STATE,
    _reserved19: [u8; 0x30],
    tx_slot_config: [TX_SLOT_CONFIG; 5],
    _reserved20: [u8; 0x34],
    hw_stat_tx_rts: HW_STAT_TX_RTS,
    hw_stat_tx_cts: HW_STAT_TX_CTS,
    hw_stat_tx_ack: HW_STAT_TX_ACK,
    hw_stat_trcts: HW_STAT_TRCTS,
    hw_stat_trigger: HW_STAT_TRIGGER,
    hw_stat_tx_hung: HW_STAT_TX_HUNG,
    hw_stat_panic: HW_STAT_PANIC,
    _reserved27: [u8; 0x03f4],
    plcp1: (),
    _reserved28: [u8; 0x04],
    plcp2: (),
    _reserved29: [u8; 0x04],
    ht_sig: (),
    _reserved30: [u8; 0x04],
    ht_unknown: (),
    _reserved31: [u8; 0x04],
    duration: (),
    _reserved32: [u8; 0x08],
    pmd: (),
    _reserved33: [u8; 0x0280],
    crypto_key_slot: [CRYPTO_KEY_SLOT; 25],
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
    #[doc = "0xf8..0x108 - Configures which control frames pass the RX filter. Setting a bit lets that frame type pass the filter."]
    #[inline(always)]
    pub const fn rx_ctrl_filter(&self, n: usize) -> &RX_CTRL_FILTER {
        &self.rx_ctrl_filter[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xf8..0x108 - Configures which control frames pass the RX filter. Setting a bit lets that frame type pass the filter."]
    #[inline(always)]
    pub fn rx_ctrl_filter_iter(&self) -> impl Iterator<Item = &RX_CTRL_FILTER> {
        self.rx_ctrl_filter.iter()
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
    #[doc = "0x800..0x818 - Control registers for hardware crypto"]
    #[inline(always)]
    pub const fn crypto_control(&self) -> &CRYPTO_CONTROL {
        &self.crypto_control
    }
    #[doc = "0xc00 - Current value of the MAC timer"]
    #[inline(always)]
    pub const fn mac_time(&self) -> &MAC_TIME {
        &self.mac_time
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
    #[doc = "0x1168..0x117c - PLCP1"]
    #[inline(always)]
    pub const fn plcp1(&self, n: usize) -> &PLCP1 {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4456)
                .add(60 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1168..0x117c - PLCP1"]
    #[inline(always)]
    pub fn plcp1_iter(&self) -> impl Iterator<Item = &PLCP1> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4456)
                .add(60 * n)
                .cast()
        })
    }
    #[doc = "0x116c..0x1180 - PLCP2"]
    #[inline(always)]
    pub const fn plcp2(&self, n: usize) -> &PLCP2 {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4460)
                .add(60 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x116c..0x1180 - PLCP2"]
    #[inline(always)]
    pub fn plcp2_iter(&self) -> impl Iterator<Item = &PLCP2> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4460)
                .add(60 * n)
                .cast()
        })
    }
    #[doc = "0x1170..0x1184 - HT-SIG field in HT preamble"]
    #[inline(always)]
    pub const fn ht_sig(&self, n: usize) -> &HT_SIG {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4464)
                .add(60 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1170..0x1184 - HT-SIG field in HT preamble"]
    #[inline(always)]
    pub fn ht_sig_iter(&self) -> impl Iterator<Item = &HT_SIG> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4464)
                .add(60 * n)
                .cast()
        })
    }
    #[doc = "0x1174..0x1188 - exact meaning and name unknown, related to HT"]
    #[inline(always)]
    pub const fn ht_unknown(&self, n: usize) -> &HT_UNKNOWN {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4468)
                .add(60 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1174..0x1188 - exact meaning and name unknown, related to HT"]
    #[inline(always)]
    pub fn ht_unknown_iter(&self) -> impl Iterator<Item = &HT_UNKNOWN> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4468)
                .add(60 * n)
                .cast()
        })
    }
    #[doc = "0x1178..0x118c - duration of the frame exchange"]
    #[inline(always)]
    pub const fn duration(&self, n: usize) -> &DURATION {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4472)
                .add(60 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1178..0x118c - duration of the frame exchange"]
    #[inline(always)]
    pub fn duration_iter(&self) -> impl Iterator<Item = &DURATION> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4472)
                .add(60 * n)
                .cast()
        })
    }
    #[doc = "0x1180..0x1194 - "]
    #[inline(always)]
    pub const fn pmd(&self, n: usize) -> &PMD {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4480)
                .add(60 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1180..0x1194 - "]
    #[inline(always)]
    pub fn pmd_iter(&self) -> impl Iterator<Item = &PMD> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4480)
                .add(60 * n)
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
}
#[doc = "Filter banks for frame reception. Bank zero is for the BSSID and bank one for the RA. Each filter bank has registers for two interfaces."]
pub use self::filter_bank::FILTER_BANK;
#[doc = r"Cluster"]
#[doc = "Filter banks for frame reception. Bank zero is for the BSSID and bank one for the RA. Each filter bank has registers for two interfaces."]
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
#[doc = "Control registers for hardware crypto"]
pub use self::crypto_control::CRYPTO_CONTROL;
#[doc = r"Cluster"]
#[doc = "Control registers for hardware crypto"]
pub mod crypto_control;
#[doc = "MAC_TIME (rw) register accessor: Current value of the MAC timer\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_time`] module"]
pub type MAC_TIME = crate::Reg<mac_time::MAC_TIME_SPEC>;
#[doc = "Current value of the MAC timer"]
pub mod mac_time;
#[doc = "Status and clear for the Wi-Fi MAC interrupt"]
pub use self::mac_interrupt::MAC_INTERRUPT;
#[doc = r"Cluster"]
#[doc = "Status and clear for the Wi-Fi MAC interrupt"]
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
#[doc = "PLCP2 (rw) register accessor: PLCP2\n\nYou can [`read`](crate::Reg::read) this register and get [`plcp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plcp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plcp2`] module"]
pub type PLCP2 = crate::Reg<plcp2::PLCP2_SPEC>;
#[doc = "PLCP2"]
pub mod plcp2;
#[doc = "HT_SIG (rw) register accessor: HT-SIG field in HT preamble\n\nYou can [`read`](crate::Reg::read) this register and get [`ht_sig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ht_sig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ht_sig`] module"]
pub type HT_SIG = crate::Reg<ht_sig::HT_SIG_SPEC>;
#[doc = "HT-SIG field in HT preamble"]
pub mod ht_sig;
#[doc = "HT_UNKNOWN (rw) register accessor: exact meaning and name unknown, related to HT\n\nYou can [`read`](crate::Reg::read) this register and get [`ht_unknown::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ht_unknown::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ht_unknown`] module"]
pub type HT_UNKNOWN = crate::Reg<ht_unknown::HT_UNKNOWN_SPEC>;
#[doc = "exact meaning and name unknown, related to HT"]
pub mod ht_unknown;
#[doc = "DURATION (rw) register accessor: duration of the frame exchange\n\nYou can [`read`](crate::Reg::read) this register and get [`duration::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`duration::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@duration`] module"]
pub type DURATION = crate::Reg<duration::DURATION_SPEC>;
#[doc = "duration of the frame exchange"]
pub mod duration;
#[doc = "PMD (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmd`] module"]
pub type PMD = crate::Reg<pmd::PMD_SPEC>;
#[doc = ""]
pub mod pmd;
#[doc = "Cryptographic keys for MPDU encapsulation and decapsulation"]
pub use self::crypto_key_slot::CRYPTO_KEY_SLOT;
#[doc = r"Cluster"]
#[doc = "Cryptographic keys for MPDU encapsulation and decapsulation"]
pub mod crypto_key_slot;
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
