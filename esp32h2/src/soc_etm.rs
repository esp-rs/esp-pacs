#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    ch_ena_ad0: CH_ENA_AD0,
    ch_ena_ad0_set: CH_ENA_AD0_SET,
    ch_ena_ad0_clr: CH_ENA_AD0_CLR,
    ch_ena_ad1: CH_ENA_AD1,
    ch_ena_ad1_set: CH_ENA_AD1_SET,
    ch_ena_ad1_clr: CH_ENA_AD1_CLR,
    ch: [CH; 50],
    clk_en: CLK_EN,
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - channel enable register
    #[inline(always)]
    pub const fn ch_ena_ad0(&self) -> &CH_ENA_AD0 {
        &self.ch_ena_ad0
    }
    ///0x04 - channel enable set register
    #[inline(always)]
    pub const fn ch_ena_ad0_set(&self) -> &CH_ENA_AD0_SET {
        &self.ch_ena_ad0_set
    }
    ///0x08 - channel enable clear register
    #[inline(always)]
    pub const fn ch_ena_ad0_clr(&self) -> &CH_ENA_AD0_CLR {
        &self.ch_ena_ad0_clr
    }
    ///0x0c - channel enable register
    #[inline(always)]
    pub const fn ch_ena_ad1(&self) -> &CH_ENA_AD1 {
        &self.ch_ena_ad1
    }
    ///0x10 - channel enable set register
    #[inline(always)]
    pub const fn ch_ena_ad1_set(&self) -> &CH_ENA_AD1_SET {
        &self.ch_ena_ad1_set
    }
    ///0x14 - channel enable clear register
    #[inline(always)]
    pub const fn ch_ena_ad1_clr(&self) -> &CH_ENA_AD1_CLR {
        &self.ch_ena_ad1_clr
    }
    ///0x18..0x1a8 - Cluster CH%s, containing CH*_EVT_ID, CH*_TASK_ID
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    ///Iterator for array of:
    ///0x18..0x1a8 - Cluster CH%s, containing CH*_EVT_ID, CH*_TASK_ID
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
    ///0x1a8 - etm clock enable register
    #[inline(always)]
    pub const fn clk_en(&self) -> &CLK_EN {
        &self.clk_en
    }
    ///0x1ac - etm date register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**CH_ENA_AD0 (rw) register accessor: channel enable register

You can [`read`](crate::generic::Reg::read) this register and get [`ch_ena_ad0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ena_ad0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ch_ena_ad0`] module*/
pub type CH_ENA_AD0 = crate::Reg<ch_ena_ad0::CH_ENA_AD0_SPEC>;
///channel enable register
pub mod ch_ena_ad0;
/**CH_ENA_AD0_SET (w) register accessor: channel enable set register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ena_ad0_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ch_ena_ad0_set`] module*/
pub type CH_ENA_AD0_SET = crate::Reg<ch_ena_ad0_set::CH_ENA_AD0_SET_SPEC>;
///channel enable set register
pub mod ch_ena_ad0_set;
/**CH_ENA_AD0_CLR (w) register accessor: channel enable clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ena_ad0_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ch_ena_ad0_clr`] module*/
pub type CH_ENA_AD0_CLR = crate::Reg<ch_ena_ad0_clr::CH_ENA_AD0_CLR_SPEC>;
///channel enable clear register
pub mod ch_ena_ad0_clr;
/**CH_ENA_AD1 (rw) register accessor: channel enable register

You can [`read`](crate::generic::Reg::read) this register and get [`ch_ena_ad1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ena_ad1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ch_ena_ad1`] module*/
pub type CH_ENA_AD1 = crate::Reg<ch_ena_ad1::CH_ENA_AD1_SPEC>;
///channel enable register
pub mod ch_ena_ad1;
/**CH_ENA_AD1_SET (w) register accessor: channel enable set register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ena_ad1_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ch_ena_ad1_set`] module*/
pub type CH_ENA_AD1_SET = crate::Reg<ch_ena_ad1_set::CH_ENA_AD1_SET_SPEC>;
///channel enable set register
pub mod ch_ena_ad1_set;
/**CH_ENA_AD1_CLR (w) register accessor: channel enable clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ena_ad1_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ch_ena_ad1_clr`] module*/
pub type CH_ENA_AD1_CLR = crate::Reg<ch_ena_ad1_clr::CH_ENA_AD1_CLR_SPEC>;
///channel enable clear register
pub mod ch_ena_ad1_clr;
///Cluster CH%s, containing CH*_EVT_ID, CH*_TASK_ID
pub use self::ch::CH;
///Cluster
///Cluster CH%s, containing CH*_EVT_ID, CH*_TASK_ID
pub mod ch;
/**CLK_EN (rw) register accessor: etm clock enable register

You can [`read`](crate::generic::Reg::read) this register and get [`clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clk_en`] module*/
pub type CLK_EN = crate::Reg<clk_en::CLK_EN_SPEC>;
///etm clock enable register
pub mod clk_en;
/**DATE (rw) register accessor: etm date register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///etm date register
pub mod date;
