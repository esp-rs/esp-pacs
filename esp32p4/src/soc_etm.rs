#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    ch_ena_ad0: CH_ENA_AD0,
    ch_ena_ad0_set: CH_ENA_AD0_SET,
    ch_ena_ad0_clr: CH_ENA_AD0_CLR,
    ch_ena_ad1: CH_ENA_AD1,
    ch_ena_ad1_set: CH_ENA_AD1_SET,
    ch_ena_ad1_clr: CH_ENA_AD1_CLR,
    ch: [CH; 50],
    evt_st0: EVT_ST0,
    evt_st0_clr: EVT_ST0_CLR,
    evt_st1: EVT_ST1,
    evt_st1_clr: EVT_ST1_CLR,
    evt_st2: EVT_ST2,
    evt_st2_clr: EVT_ST2_CLR,
    evt_st3: EVT_ST3,
    evt_st3_clr: EVT_ST3_CLR,
    evt_st4: EVT_ST4,
    evt_st4_clr: EVT_ST4_CLR,
    evt_st5: EVT_ST5,
    evt_st5_clr: EVT_ST5_CLR,
    evt_st6: EVT_ST6,
    evt_st6_clr: EVT_ST6_CLR,
    evt_st7: EVT_ST7,
    evt_st7_clr: EVT_ST7_CLR,
    task_st0: TASK_ST0,
    task_st0_clr: TASK_ST0_CLR,
    task_st1: TASK_ST1,
    task_st1_clr: TASK_ST1_CLR,
    task_st2: TASK_ST2,
    task_st2_clr: TASK_ST2_CLR,
    task_st3: TASK_ST3,
    task_st3_clr: TASK_ST3_CLR,
    task_st4: TASK_ST4,
    task_st4_clr: TASK_ST4_CLR,
    task_st5: TASK_ST5,
    task_st5_clr: TASK_ST5_CLR,
    task_st6: TASK_ST6,
    task_st6_clr: TASK_ST6_CLR,
    clk_en: CLK_EN,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - Channel enable status register"]
    #[inline(always)]
    pub const fn ch_ena_ad0(&self) -> &CH_ENA_AD0 {
        &self.ch_ena_ad0
    }
    #[doc = "0x04 - Channel enable set register"]
    #[inline(always)]
    pub const fn ch_ena_ad0_set(&self) -> &CH_ENA_AD0_SET {
        &self.ch_ena_ad0_set
    }
    #[doc = "0x08 - Channel enable clear register"]
    #[inline(always)]
    pub const fn ch_ena_ad0_clr(&self) -> &CH_ENA_AD0_CLR {
        &self.ch_ena_ad0_clr
    }
    #[doc = "0x0c - Channel enable status register"]
    #[inline(always)]
    pub const fn ch_ena_ad1(&self) -> &CH_ENA_AD1 {
        &self.ch_ena_ad1
    }
    #[doc = "0x10 - Channel enable set register"]
    #[inline(always)]
    pub const fn ch_ena_ad1_set(&self) -> &CH_ENA_AD1_SET {
        &self.ch_ena_ad1_set
    }
    #[doc = "0x14 - Channel enable clear register"]
    #[inline(always)]
    pub const fn ch_ena_ad1_clr(&self) -> &CH_ENA_AD1_CLR {
        &self.ch_ena_ad1_clr
    }
    #[doc = "0x18..0x1a8 - Cluster CH%s, containing CH*_EVT_ID, CH*_TASK_ID"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x18..0x1a8 - Cluster CH%s, containing CH*_EVT_ID, CH*_TASK_ID"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
    #[doc = "0x1a8 - Events trigger status register"]
    #[inline(always)]
    pub const fn evt_st0(&self) -> &EVT_ST0 {
        &self.evt_st0
    }
    #[doc = "0x1ac - Events trigger status clear register"]
    #[inline(always)]
    pub const fn evt_st0_clr(&self) -> &EVT_ST0_CLR {
        &self.evt_st0_clr
    }
    #[doc = "0x1b0 - Events trigger status register"]
    #[inline(always)]
    pub const fn evt_st1(&self) -> &EVT_ST1 {
        &self.evt_st1
    }
    #[doc = "0x1b4 - Events trigger status clear register"]
    #[inline(always)]
    pub const fn evt_st1_clr(&self) -> &EVT_ST1_CLR {
        &self.evt_st1_clr
    }
    #[doc = "0x1b8 - Events trigger status register"]
    #[inline(always)]
    pub const fn evt_st2(&self) -> &EVT_ST2 {
        &self.evt_st2
    }
    #[doc = "0x1bc - Events trigger status clear register"]
    #[inline(always)]
    pub const fn evt_st2_clr(&self) -> &EVT_ST2_CLR {
        &self.evt_st2_clr
    }
    #[doc = "0x1c0 - Events trigger status register"]
    #[inline(always)]
    pub const fn evt_st3(&self) -> &EVT_ST3 {
        &self.evt_st3
    }
    #[doc = "0x1c4 - Events trigger status clear register"]
    #[inline(always)]
    pub const fn evt_st3_clr(&self) -> &EVT_ST3_CLR {
        &self.evt_st3_clr
    }
    #[doc = "0x1c8 - Events trigger status register"]
    #[inline(always)]
    pub const fn evt_st4(&self) -> &EVT_ST4 {
        &self.evt_st4
    }
    #[doc = "0x1cc - Events trigger status clear register"]
    #[inline(always)]
    pub const fn evt_st4_clr(&self) -> &EVT_ST4_CLR {
        &self.evt_st4_clr
    }
    #[doc = "0x1d0 - Events trigger status register"]
    #[inline(always)]
    pub const fn evt_st5(&self) -> &EVT_ST5 {
        &self.evt_st5
    }
    #[doc = "0x1d4 - Events trigger status clear register"]
    #[inline(always)]
    pub const fn evt_st5_clr(&self) -> &EVT_ST5_CLR {
        &self.evt_st5_clr
    }
    #[doc = "0x1d8 - Events trigger status register"]
    #[inline(always)]
    pub const fn evt_st6(&self) -> &EVT_ST6 {
        &self.evt_st6
    }
    #[doc = "0x1dc - Events trigger status clear register"]
    #[inline(always)]
    pub const fn evt_st6_clr(&self) -> &EVT_ST6_CLR {
        &self.evt_st6_clr
    }
    #[doc = "0x1e0 - Events trigger status register"]
    #[inline(always)]
    pub const fn evt_st7(&self) -> &EVT_ST7 {
        &self.evt_st7
    }
    #[doc = "0x1e4 - Events trigger status clear register"]
    #[inline(always)]
    pub const fn evt_st7_clr(&self) -> &EVT_ST7_CLR {
        &self.evt_st7_clr
    }
    #[doc = "0x1e8 - Tasks trigger status register"]
    #[inline(always)]
    pub const fn task_st0(&self) -> &TASK_ST0 {
        &self.task_st0
    }
    #[doc = "0x1ec - Tasks trigger status clear register"]
    #[inline(always)]
    pub const fn task_st0_clr(&self) -> &TASK_ST0_CLR {
        &self.task_st0_clr
    }
    #[doc = "0x1f0 - Tasks trigger status register"]
    #[inline(always)]
    pub const fn task_st1(&self) -> &TASK_ST1 {
        &self.task_st1
    }
    #[doc = "0x1f4 - Tasks trigger status clear register"]
    #[inline(always)]
    pub const fn task_st1_clr(&self) -> &TASK_ST1_CLR {
        &self.task_st1_clr
    }
    #[doc = "0x1f8 - Tasks trigger status register"]
    #[inline(always)]
    pub const fn task_st2(&self) -> &TASK_ST2 {
        &self.task_st2
    }
    #[doc = "0x1fc - Tasks trigger status clear register"]
    #[inline(always)]
    pub const fn task_st2_clr(&self) -> &TASK_ST2_CLR {
        &self.task_st2_clr
    }
    #[doc = "0x200 - Tasks trigger status register"]
    #[inline(always)]
    pub const fn task_st3(&self) -> &TASK_ST3 {
        &self.task_st3
    }
    #[doc = "0x204 - Tasks trigger status clear register"]
    #[inline(always)]
    pub const fn task_st3_clr(&self) -> &TASK_ST3_CLR {
        &self.task_st3_clr
    }
    #[doc = "0x208 - Tasks trigger status register"]
    #[inline(always)]
    pub const fn task_st4(&self) -> &TASK_ST4 {
        &self.task_st4
    }
    #[doc = "0x20c - Tasks trigger status clear register"]
    #[inline(always)]
    pub const fn task_st4_clr(&self) -> &TASK_ST4_CLR {
        &self.task_st4_clr
    }
    #[doc = "0x210 - Tasks trigger status register"]
    #[inline(always)]
    pub const fn task_st5(&self) -> &TASK_ST5 {
        &self.task_st5
    }
    #[doc = "0x214 - Tasks trigger status clear register"]
    #[inline(always)]
    pub const fn task_st5_clr(&self) -> &TASK_ST5_CLR {
        &self.task_st5_clr
    }
    #[doc = "0x218 - Tasks trigger status register"]
    #[inline(always)]
    pub const fn task_st6(&self) -> &TASK_ST6 {
        &self.task_st6
    }
    #[doc = "0x21c - Tasks trigger status clear register"]
    #[inline(always)]
    pub const fn task_st6_clr(&self) -> &TASK_ST6_CLR {
        &self.task_st6_clr
    }
    #[doc = "0x220 - ETM clock enable register"]
    #[inline(always)]
    pub const fn clk_en(&self) -> &CLK_EN {
        &self.clk_en
    }
    #[doc = "0x224 - ETM date register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CH_ENA_AD0 (rw) register accessor: Channel enable status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_ena_ad0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ena_ad0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ena_ad0`] module"]
pub type CH_ENA_AD0 = crate::Reg<ch_ena_ad0::CH_ENA_AD0_SPEC>;
#[doc = "Channel enable status register"]
pub mod ch_ena_ad0;
#[doc = "CH_ENA_AD0_SET (w) register accessor: Channel enable set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ena_ad0_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ena_ad0_set`] module"]
pub type CH_ENA_AD0_SET = crate::Reg<ch_ena_ad0_set::CH_ENA_AD0_SET_SPEC>;
#[doc = "Channel enable set register"]
pub mod ch_ena_ad0_set;
#[doc = "CH_ENA_AD0_CLR (w) register accessor: Channel enable clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ena_ad0_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ena_ad0_clr`] module"]
pub type CH_ENA_AD0_CLR = crate::Reg<ch_ena_ad0_clr::CH_ENA_AD0_CLR_SPEC>;
#[doc = "Channel enable clear register"]
pub mod ch_ena_ad0_clr;
#[doc = "CH_ENA_AD1 (rw) register accessor: Channel enable status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_ena_ad1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ena_ad1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ena_ad1`] module"]
pub type CH_ENA_AD1 = crate::Reg<ch_ena_ad1::CH_ENA_AD1_SPEC>;
#[doc = "Channel enable status register"]
pub mod ch_ena_ad1;
#[doc = "CH_ENA_AD1_SET (w) register accessor: Channel enable set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ena_ad1_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ena_ad1_set`] module"]
pub type CH_ENA_AD1_SET = crate::Reg<ch_ena_ad1_set::CH_ENA_AD1_SET_SPEC>;
#[doc = "Channel enable set register"]
pub mod ch_ena_ad1_set;
#[doc = "CH_ENA_AD1_CLR (w) register accessor: Channel enable clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ena_ad1_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ena_ad1_clr`] module"]
pub type CH_ENA_AD1_CLR = crate::Reg<ch_ena_ad1_clr::CH_ENA_AD1_CLR_SPEC>;
#[doc = "Channel enable clear register"]
pub mod ch_ena_ad1_clr;
#[doc = "Cluster CH%s, containing CH*_EVT_ID, CH*_TASK_ID"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "Cluster CH%s, containing CH*_EVT_ID, CH*_TASK_ID"]
pub mod ch;
#[doc = "EVT_ST0 (rw) register accessor: Events trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_st0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st0`] module"]
pub type EVT_ST0 = crate::Reg<evt_st0::EVT_ST0_SPEC>;
#[doc = "Events trigger status register"]
pub mod evt_st0;
#[doc = "EVT_ST0_CLR (w) register accessor: Events trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st0_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st0_clr`] module"]
pub type EVT_ST0_CLR = crate::Reg<evt_st0_clr::EVT_ST0_CLR_SPEC>;
#[doc = "Events trigger status clear register"]
pub mod evt_st0_clr;
#[doc = "EVT_ST1 (rw) register accessor: Events trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_st1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st1`] module"]
pub type EVT_ST1 = crate::Reg<evt_st1::EVT_ST1_SPEC>;
#[doc = "Events trigger status register"]
pub mod evt_st1;
#[doc = "EVT_ST1_CLR (w) register accessor: Events trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st1_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st1_clr`] module"]
pub type EVT_ST1_CLR = crate::Reg<evt_st1_clr::EVT_ST1_CLR_SPEC>;
#[doc = "Events trigger status clear register"]
pub mod evt_st1_clr;
#[doc = "EVT_ST2 (rw) register accessor: Events trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_st2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st2`] module"]
pub type EVT_ST2 = crate::Reg<evt_st2::EVT_ST2_SPEC>;
#[doc = "Events trigger status register"]
pub mod evt_st2;
#[doc = "EVT_ST2_CLR (w) register accessor: Events trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st2_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st2_clr`] module"]
pub type EVT_ST2_CLR = crate::Reg<evt_st2_clr::EVT_ST2_CLR_SPEC>;
#[doc = "Events trigger status clear register"]
pub mod evt_st2_clr;
#[doc = "EVT_ST3 (rw) register accessor: Events trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_st3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st3`] module"]
pub type EVT_ST3 = crate::Reg<evt_st3::EVT_ST3_SPEC>;
#[doc = "Events trigger status register"]
pub mod evt_st3;
#[doc = "EVT_ST3_CLR (w) register accessor: Events trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st3_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st3_clr`] module"]
pub type EVT_ST3_CLR = crate::Reg<evt_st3_clr::EVT_ST3_CLR_SPEC>;
#[doc = "Events trigger status clear register"]
pub mod evt_st3_clr;
#[doc = "EVT_ST4 (rw) register accessor: Events trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_st4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st4`] module"]
pub type EVT_ST4 = crate::Reg<evt_st4::EVT_ST4_SPEC>;
#[doc = "Events trigger status register"]
pub mod evt_st4;
#[doc = "EVT_ST4_CLR (w) register accessor: Events trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st4_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st4_clr`] module"]
pub type EVT_ST4_CLR = crate::Reg<evt_st4_clr::EVT_ST4_CLR_SPEC>;
#[doc = "Events trigger status clear register"]
pub mod evt_st4_clr;
#[doc = "EVT_ST5 (rw) register accessor: Events trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_st5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st5`] module"]
pub type EVT_ST5 = crate::Reg<evt_st5::EVT_ST5_SPEC>;
#[doc = "Events trigger status register"]
pub mod evt_st5;
#[doc = "EVT_ST5_CLR (w) register accessor: Events trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st5_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st5_clr`] module"]
pub type EVT_ST5_CLR = crate::Reg<evt_st5_clr::EVT_ST5_CLR_SPEC>;
#[doc = "Events trigger status clear register"]
pub mod evt_st5_clr;
#[doc = "EVT_ST6 (rw) register accessor: Events trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_st6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st6`] module"]
pub type EVT_ST6 = crate::Reg<evt_st6::EVT_ST6_SPEC>;
#[doc = "Events trigger status register"]
pub mod evt_st6;
#[doc = "EVT_ST6_CLR (w) register accessor: Events trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st6_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st6_clr`] module"]
pub type EVT_ST6_CLR = crate::Reg<evt_st6_clr::EVT_ST6_CLR_SPEC>;
#[doc = "Events trigger status clear register"]
pub mod evt_st6_clr;
#[doc = "EVT_ST7 (rw) register accessor: Events trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_st7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st7`] module"]
pub type EVT_ST7 = crate::Reg<evt_st7::EVT_ST7_SPEC>;
#[doc = "Events trigger status register"]
pub mod evt_st7;
#[doc = "EVT_ST7_CLR (w) register accessor: Events trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st7_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st7_clr`] module"]
pub type EVT_ST7_CLR = crate::Reg<evt_st7_clr::EVT_ST7_CLR_SPEC>;
#[doc = "Events trigger status clear register"]
pub mod evt_st7_clr;
#[doc = "TASK_ST0 (rw) register accessor: Tasks trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`task_st0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st0`] module"]
pub type TASK_ST0 = crate::Reg<task_st0::TASK_ST0_SPEC>;
#[doc = "Tasks trigger status register"]
pub mod task_st0;
#[doc = "TASK_ST0_CLR (w) register accessor: Tasks trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st0_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st0_clr`] module"]
pub type TASK_ST0_CLR = crate::Reg<task_st0_clr::TASK_ST0_CLR_SPEC>;
#[doc = "Tasks trigger status clear register"]
pub mod task_st0_clr;
#[doc = "TASK_ST1 (rw) register accessor: Tasks trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`task_st1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st1`] module"]
pub type TASK_ST1 = crate::Reg<task_st1::TASK_ST1_SPEC>;
#[doc = "Tasks trigger status register"]
pub mod task_st1;
#[doc = "TASK_ST1_CLR (w) register accessor: Tasks trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st1_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st1_clr`] module"]
pub type TASK_ST1_CLR = crate::Reg<task_st1_clr::TASK_ST1_CLR_SPEC>;
#[doc = "Tasks trigger status clear register"]
pub mod task_st1_clr;
#[doc = "TASK_ST2 (rw) register accessor: Tasks trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`task_st2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st2`] module"]
pub type TASK_ST2 = crate::Reg<task_st2::TASK_ST2_SPEC>;
#[doc = "Tasks trigger status register"]
pub mod task_st2;
#[doc = "TASK_ST2_CLR (w) register accessor: Tasks trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st2_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st2_clr`] module"]
pub type TASK_ST2_CLR = crate::Reg<task_st2_clr::TASK_ST2_CLR_SPEC>;
#[doc = "Tasks trigger status clear register"]
pub mod task_st2_clr;
#[doc = "TASK_ST3 (rw) register accessor: Tasks trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`task_st3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st3`] module"]
pub type TASK_ST3 = crate::Reg<task_st3::TASK_ST3_SPEC>;
#[doc = "Tasks trigger status register"]
pub mod task_st3;
#[doc = "TASK_ST3_CLR (w) register accessor: Tasks trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st3_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st3_clr`] module"]
pub type TASK_ST3_CLR = crate::Reg<task_st3_clr::TASK_ST3_CLR_SPEC>;
#[doc = "Tasks trigger status clear register"]
pub mod task_st3_clr;
#[doc = "TASK_ST4 (rw) register accessor: Tasks trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`task_st4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st4`] module"]
pub type TASK_ST4 = crate::Reg<task_st4::TASK_ST4_SPEC>;
#[doc = "Tasks trigger status register"]
pub mod task_st4;
#[doc = "TASK_ST4_CLR (w) register accessor: Tasks trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st4_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st4_clr`] module"]
pub type TASK_ST4_CLR = crate::Reg<task_st4_clr::TASK_ST4_CLR_SPEC>;
#[doc = "Tasks trigger status clear register"]
pub mod task_st4_clr;
#[doc = "TASK_ST5 (rw) register accessor: Tasks trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`task_st5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st5`] module"]
pub type TASK_ST5 = crate::Reg<task_st5::TASK_ST5_SPEC>;
#[doc = "Tasks trigger status register"]
pub mod task_st5;
#[doc = "TASK_ST5_CLR (w) register accessor: Tasks trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st5_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st5_clr`] module"]
pub type TASK_ST5_CLR = crate::Reg<task_st5_clr::TASK_ST5_CLR_SPEC>;
#[doc = "Tasks trigger status clear register"]
pub mod task_st5_clr;
#[doc = "TASK_ST6 (rw) register accessor: Tasks trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`task_st6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st6`] module"]
pub type TASK_ST6 = crate::Reg<task_st6::TASK_ST6_SPEC>;
#[doc = "Tasks trigger status register"]
pub mod task_st6;
#[doc = "TASK_ST6_CLR (w) register accessor: Tasks trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st6_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st6_clr`] module"]
pub type TASK_ST6_CLR = crate::Reg<task_st6_clr::TASK_ST6_CLR_SPEC>;
#[doc = "Tasks trigger status clear register"]
pub mod task_st6_clr;
#[doc = "CLK_EN (rw) register accessor: ETM clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_en`] module"]
pub type CLK_EN = crate::Reg<clk_en::CLK_EN_SPEC>;
#[doc = "ETM clock enable register"]
pub mod clk_en;
#[doc = "DATE (rw) register accessor: ETM date register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "ETM date register"]
pub mod date;
