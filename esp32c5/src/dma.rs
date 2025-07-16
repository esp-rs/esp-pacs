#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    in_int_ch: [IN_INT_CH; 3],
    out_int_ch: [OUT_INT_CH; 3],
    ahb_test: AHB_TEST,
    misc_conf: MISC_CONF,
    date: DATE,
    _reserved5: [u8; 0x04],
    ch: [CH; 3],
    _reserved6: [u8; 0x2c],
    tx_ch_arb_weigh_ch: (),
    _reserved7: [u8; 0x04],
    tx_arb_weigh_opt_dir_ch: (),
    _reserved8: [u8; 0x74],
    rx_ch_arb_weigh_ch: (),
    _reserved9: [u8; 0x04],
    rx_arb_weigh_opt_dir_ch: (),
    _reserved10: [u8; 0x54],
    in_link_addr_ch: [IN_LINK_ADDR_CH; 3],
    out_link_addr_ch: [OUT_LINK_ADDR_CH; 3],
    intr_mem_start_addr: INTR_MEM_START_ADDR,
    intr_mem_end_addr: INTR_MEM_END_ADDR,
    arb_timeout_tx: ARB_TIMEOUT_TX,
    arb_timeout_rx: ARB_TIMEOUT_RX,
    weight_en_tx: WEIGHT_EN_TX,
    weight_en_rx: WEIGHT_EN_RX,
}
impl RegisterBlock {
    #[doc = "0x00..0x30 - Cluster IN_INT_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?"]
    #[inline(always)]
    pub const fn in_int_ch(&self, n: usize) -> &IN_INT_CH {
        &self.in_int_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x30 - Cluster IN_INT_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?"]
    #[inline(always)]
    pub fn in_int_ch_iter(&self) -> impl Iterator<Item = &IN_INT_CH> {
        self.in_int_ch.iter()
    }
    #[doc = "0x30..0x60 - Cluster OUT_INT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?"]
    #[inline(always)]
    pub const fn out_int_ch(&self, n: usize) -> &OUT_INT_CH {
        &self.out_int_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x60 - Cluster OUT_INT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?"]
    #[inline(always)]
    pub fn out_int_ch_iter(&self) -> impl Iterator<Item = &OUT_INT_CH> {
        self.out_int_ch.iter()
    }
    #[doc = "0x60 - reserved"]
    #[inline(always)]
    pub const fn ahb_test(&self) -> &AHB_TEST {
        &self.ahb_test
    }
    #[doc = "0x64 - Miscellaneous register"]
    #[inline(always)]
    pub const fn misc_conf(&self) -> &MISC_CONF {
        &self.misc_conf
    }
    #[doc = "0x68 - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    #[doc = "0x70..0x2b0 - Cluster CH%s, containing IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x2b0 - Cluster CH%s, containing IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
    #[doc = "0x2dc..0x2e8 - TX channel %s arbitration weight configuration register"]
    #[inline(always)]
    pub const fn tx_ch_arb_weigh_ch(&self, n: usize) -> &TX_CH_ARB_WEIGH_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(732)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2dc..0x2e8 - TX channel %s arbitration weight configuration register"]
    #[inline(always)]
    pub fn tx_ch_arb_weigh_ch_iter(&self) -> impl Iterator<Item = &TX_CH_ARB_WEIGH_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(732)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x2e0..0x2ec - TX channel %s weight arbitration optimization enable register"]
    #[inline(always)]
    pub const fn tx_arb_weigh_opt_dir_ch(&self, n: usize) -> &TX_ARB_WEIGH_OPT_DIR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(736)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2e0..0x2ec - TX channel %s weight arbitration optimization enable register"]
    #[inline(always)]
    pub fn tx_arb_weigh_opt_dir_ch_iter(&self) -> impl Iterator<Item = &TX_ARB_WEIGH_OPT_DIR_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(736)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x354..0x360 - RX channel %s arbitration weight configuration register"]
    #[inline(always)]
    pub const fn rx_ch_arb_weigh_ch(&self, n: usize) -> &RX_CH_ARB_WEIGH_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(852)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x354..0x360 - RX channel %s arbitration weight configuration register"]
    #[inline(always)]
    pub fn rx_ch_arb_weigh_ch_iter(&self) -> impl Iterator<Item = &RX_CH_ARB_WEIGH_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(852)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x358..0x364 - RX channel %s weight arbitration optimization enable register"]
    #[inline(always)]
    pub const fn rx_arb_weigh_opt_dir_ch(&self, n: usize) -> &RX_ARB_WEIGH_OPT_DIR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(856)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x358..0x364 - RX channel %s weight arbitration optimization enable register"]
    #[inline(always)]
    pub fn rx_arb_weigh_opt_dir_ch_iter(&self) -> impl Iterator<Item = &RX_ARB_WEIGH_OPT_DIR_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(856)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x3ac..0x3b8 - Link list descriptor address configuration of RX channel %s"]
    #[inline(always)]
    pub const fn in_link_addr_ch(&self, n: usize) -> &IN_LINK_ADDR_CH {
        &self.in_link_addr_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3ac..0x3b8 - Link list descriptor address configuration of RX channel %s"]
    #[inline(always)]
    pub fn in_link_addr_ch_iter(&self) -> impl Iterator<Item = &IN_LINK_ADDR_CH> {
        self.in_link_addr_ch.iter()
    }
    #[doc = "0x3b8..0x3c4 - Link list descriptor address configuration of TX channel %s"]
    #[inline(always)]
    pub const fn out_link_addr_ch(&self, n: usize) -> &OUT_LINK_ADDR_CH {
        &self.out_link_addr_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3b8..0x3c4 - Link list descriptor address configuration of TX channel %s"]
    #[inline(always)]
    pub fn out_link_addr_ch_iter(&self) -> impl Iterator<Item = &OUT_LINK_ADDR_CH> {
        self.out_link_addr_ch.iter()
    }
    #[doc = "0x3c4 - Accessible address space start address configuration register"]
    #[inline(always)]
    pub const fn intr_mem_start_addr(&self) -> &INTR_MEM_START_ADDR {
        &self.intr_mem_start_addr
    }
    #[doc = "0x3c8 - Accessible address space end address configuration register"]
    #[inline(always)]
    pub const fn intr_mem_end_addr(&self) -> &INTR_MEM_END_ADDR {
        &self.intr_mem_end_addr
    }
    #[doc = "0x3cc - TX arbitration timeout configuration register"]
    #[inline(always)]
    pub const fn arb_timeout_tx(&self) -> &ARB_TIMEOUT_TX {
        &self.arb_timeout_tx
    }
    #[doc = "0x3d0 - RX arbitration timeout configuration register"]
    #[inline(always)]
    pub const fn arb_timeout_rx(&self) -> &ARB_TIMEOUT_RX {
        &self.arb_timeout_rx
    }
    #[doc = "0x3d4 - TX weight arbitration enable register"]
    #[inline(always)]
    pub const fn weight_en_tx(&self) -> &WEIGHT_EN_TX {
        &self.weight_en_tx
    }
    #[doc = "0x3d8 - RX weight arbitration enable register"]
    #[inline(always)]
    pub const fn weight_en_rx(&self) -> &WEIGHT_EN_RX {
        &self.weight_en_rx
    }
}
#[doc = "Cluster IN_INT_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?"]
pub use self::in_int_ch::IN_INT_CH;
#[doc = r"Cluster"]
#[doc = "Cluster IN_INT_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?"]
pub mod in_int_ch;
#[doc = "Cluster OUT_INT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?"]
pub use self::out_int_ch::OUT_INT_CH;
#[doc = r"Cluster"]
#[doc = "Cluster OUT_INT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?"]
pub mod out_int_ch;
#[doc = "AHB_TEST (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_test`] module"]
pub type AHB_TEST = crate::Reg<ahb_test::AHB_TEST_SPEC>;
#[doc = "reserved"]
pub mod ahb_test;
#[doc = "MISC_CONF (rw) register accessor: Miscellaneous register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_conf`] module"]
pub type MISC_CONF = crate::Reg<misc_conf::MISC_CONF_SPEC>;
#[doc = "Miscellaneous register"]
pub mod misc_conf;
#[doc = "Cluster CH%s, containing IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?"]
pub use self::ch::CH;
pub use crate::aes::date;
pub use crate::aes::DATE;
#[doc = r"Cluster"]
#[doc = "Cluster CH%s, containing IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?"]
pub mod ch;
#[doc = "TX_CH_ARB_WEIGH_CH (rw) register accessor: TX channel %s arbitration weight configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_ch_arb_weigh_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ch_arb_weigh_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_ch_arb_weigh_ch`] module"]
pub type TX_CH_ARB_WEIGH_CH = crate::Reg<tx_ch_arb_weigh_ch::TX_CH_ARB_WEIGH_CH_SPEC>;
#[doc = "TX channel %s arbitration weight configuration register"]
pub mod tx_ch_arb_weigh_ch;
#[doc = "TX_ARB_WEIGH_OPT_DIR_CH (rw) register accessor: TX channel %s weight arbitration optimization enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_arb_weigh_opt_dir_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_arb_weigh_opt_dir_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_arb_weigh_opt_dir_ch`] module"]
pub type TX_ARB_WEIGH_OPT_DIR_CH =
    crate::Reg<tx_arb_weigh_opt_dir_ch::TX_ARB_WEIGH_OPT_DIR_CH_SPEC>;
#[doc = "TX channel %s weight arbitration optimization enable register"]
pub mod tx_arb_weigh_opt_dir_ch;
#[doc = "RX_CH_ARB_WEIGH_CH (rw) register accessor: RX channel %s arbitration weight configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ch_arb_weigh_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ch_arb_weigh_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ch_arb_weigh_ch`] module"]
pub type RX_CH_ARB_WEIGH_CH = crate::Reg<rx_ch_arb_weigh_ch::RX_CH_ARB_WEIGH_CH_SPEC>;
#[doc = "RX channel %s arbitration weight configuration register"]
pub mod rx_ch_arb_weigh_ch;
#[doc = "RX_ARB_WEIGH_OPT_DIR_CH (rw) register accessor: RX channel %s weight arbitration optimization enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_arb_weigh_opt_dir_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_arb_weigh_opt_dir_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_arb_weigh_opt_dir_ch`] module"]
pub type RX_ARB_WEIGH_OPT_DIR_CH =
    crate::Reg<rx_arb_weigh_opt_dir_ch::RX_ARB_WEIGH_OPT_DIR_CH_SPEC>;
#[doc = "RX channel %s weight arbitration optimization enable register"]
pub mod rx_arb_weigh_opt_dir_ch;
#[doc = "IN_LINK_ADDR_CH (rw) register accessor: Link list descriptor address configuration of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_link_addr_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_link_addr_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link_addr_ch`] module"]
pub type IN_LINK_ADDR_CH = crate::Reg<in_link_addr_ch::IN_LINK_ADDR_CH_SPEC>;
#[doc = "Link list descriptor address configuration of RX channel %s"]
pub mod in_link_addr_ch;
#[doc = "OUT_LINK_ADDR_CH (rw) register accessor: Link list descriptor address configuration of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_link_addr_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_link_addr_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link_addr_ch`] module"]
pub type OUT_LINK_ADDR_CH = crate::Reg<out_link_addr_ch::OUT_LINK_ADDR_CH_SPEC>;
#[doc = "Link list descriptor address configuration of TX channel %s"]
pub mod out_link_addr_ch;
#[doc = "INTR_MEM_START_ADDR (rw) register accessor: Accessible address space start address configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mem_start_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mem_start_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mem_start_addr`] module"]
pub type INTR_MEM_START_ADDR = crate::Reg<intr_mem_start_addr::INTR_MEM_START_ADDR_SPEC>;
#[doc = "Accessible address space start address configuration register"]
pub mod intr_mem_start_addr;
#[doc = "INTR_MEM_END_ADDR (rw) register accessor: Accessible address space end address configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mem_end_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mem_end_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mem_end_addr`] module"]
pub type INTR_MEM_END_ADDR = crate::Reg<intr_mem_end_addr::INTR_MEM_END_ADDR_SPEC>;
#[doc = "Accessible address space end address configuration register"]
pub mod intr_mem_end_addr;
#[doc = "ARB_TIMEOUT_TX (rw) register accessor: TX arbitration timeout configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_timeout_tx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_timeout_tx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_timeout_tx`] module"]
pub type ARB_TIMEOUT_TX = crate::Reg<arb_timeout_tx::ARB_TIMEOUT_TX_SPEC>;
#[doc = "TX arbitration timeout configuration register"]
pub mod arb_timeout_tx;
#[doc = "ARB_TIMEOUT_RX (rw) register accessor: RX arbitration timeout configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_timeout_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_timeout_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_timeout_rx`] module"]
pub type ARB_TIMEOUT_RX = crate::Reg<arb_timeout_rx::ARB_TIMEOUT_RX_SPEC>;
#[doc = "RX arbitration timeout configuration register"]
pub mod arb_timeout_rx;
#[doc = "WEIGHT_EN_TX (rw) register accessor: TX weight arbitration enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`weight_en_tx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`weight_en_tx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@weight_en_tx`] module"]
pub type WEIGHT_EN_TX = crate::Reg<weight_en_tx::WEIGHT_EN_TX_SPEC>;
#[doc = "TX weight arbitration enable register"]
pub mod weight_en_tx;
#[doc = "WEIGHT_EN_RX (rw) register accessor: RX weight arbitration enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`weight_en_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`weight_en_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@weight_en_rx`] module"]
pub type WEIGHT_EN_RX = crate::Reg<weight_en_rx::WEIGHT_EN_RX_SPEC>;
#[doc = "RX weight arbitration enable register"]
pub mod weight_en_rx;
