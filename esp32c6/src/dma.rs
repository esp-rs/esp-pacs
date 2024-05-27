#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    in_int_ch: [IN_INT_CH; 3],
    out_int_ch: [OUT_INT_CH; 3],
    ahb_test: AHB_TEST,
    misc_conf: MISC_CONF,
    date: DATE,
    _reserved5: [u8; 0x04],
    ch: [CH; 3],
}
impl RegisterBlock {
    ///0x00..0x30 - Cluster IN_INT_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?
    #[inline(always)]
    pub const fn in_int_ch(&self, n: usize) -> &IN_INT_CH {
        &self.in_int_ch[n]
    }
    ///Iterator for array of:
    ///0x00..0x30 - Cluster IN_INT_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?
    #[inline(always)]
    pub fn in_int_ch_iter(&self) -> impl Iterator<Item = &IN_INT_CH> {
        self.in_int_ch.iter()
    }
    ///0x30..0x60 - Cluster OUT_INT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?
    #[inline(always)]
    pub const fn out_int_ch(&self, n: usize) -> &OUT_INT_CH {
        &self.out_int_ch[n]
    }
    ///Iterator for array of:
    ///0x30..0x60 - Cluster OUT_INT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?
    #[inline(always)]
    pub fn out_int_ch_iter(&self) -> impl Iterator<Item = &OUT_INT_CH> {
        self.out_int_ch.iter()
    }
    ///0x60 - reserved
    #[inline(always)]
    pub const fn ahb_test(&self) -> &AHB_TEST {
        &self.ahb_test
    }
    ///0x64 - MISC register
    #[inline(always)]
    pub const fn misc_conf(&self) -> &MISC_CONF {
        &self.misc_conf
    }
    ///0x68 - Version control register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    ///0x70..0x2b0 - Cluster CH%s, containing IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    ///Iterator for array of:
    ///0x70..0x2b0 - Cluster CH%s, containing IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
}
///Cluster IN_INT_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?
pub use self::in_int_ch::IN_INT_CH;
///Cluster
///Cluster IN_INT_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?
pub mod in_int_ch;
///Cluster OUT_INT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?
pub use self::out_int_ch::OUT_INT_CH;
///Cluster
///Cluster OUT_INT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?
pub mod out_int_ch;
/**AHB_TEST (rw) register accessor: reserved

You can [`read`](crate::generic::Reg::read) this register and get [`ahb_test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ahb_test`] module*/
pub type AHB_TEST = crate::Reg<ahb_test::AHB_TEST_SPEC>;
///reserved
pub mod ahb_test;
/**MISC_CONF (rw) register accessor: MISC register

You can [`read`](crate::generic::Reg::read) this register and get [`misc_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@misc_conf`] module*/
pub type MISC_CONF = crate::Reg<misc_conf::MISC_CONF_SPEC>;
///MISC register
pub mod misc_conf;
/**DATE (rw) register accessor: Version control register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///Version control register
pub mod date;
///Cluster CH%s, containing IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?
pub use self::ch::CH;
///Cluster
///Cluster CH%s, containing IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?
pub mod ch;
