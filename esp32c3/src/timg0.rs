#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    t: [T; 1],
    _reserved1: [u8; 0x24],
    wdtconfig0: WDTCONFIG0,
    wdtconfig1: WDTCONFIG1,
    wdtconfig2: WDTCONFIG2,
    wdtconfig3: WDTCONFIG3,
    wdtconfig4: WDTCONFIG4,
    wdtconfig5: WDTCONFIG5,
    wdtfeed: WDTFEED,
    wdtwprotect: WDTWPROTECT,
    rtccalicfg: RTCCALICFG,
    rtccalicfg1: RTCCALICFG1,
    int_ena_timers: INT_ENA_TIMERS,
    int_raw_timers: INT_RAW_TIMERS,
    int_st_timers: INT_ST_TIMERS,
    int_clr_timers: INT_CLR_TIMERS,
    rtccalicfg2: RTCCALICFG2,
    _reserved16: [u8; 0x74],
    ntimg_date: NTIMG_DATE,
    regclk: REGCLK,
}
impl RegisterBlock {
    ///0x00..0x24 - Cluster T%s, containing T?CONFIG, T?LO, T?HI, T?UPDATE, T?ALARMLO, T?ALARMHI, T?LOADLO, T?LOADHI, T?LOAD
    #[inline(always)]
    pub const fn t(&self, n: usize) -> &T {
        &self.t[n]
    }
    ///Iterator for array of:
    ///0x00..0x24 - Cluster T%s, containing T?CONFIG, T?LO, T?HI, T?UPDATE, T?ALARMLO, T?ALARMHI, T?LOADLO, T?LOADHI, T?LOAD
    #[inline(always)]
    pub fn t_iter(&self) -> impl Iterator<Item = &T> {
        self.t.iter()
    }
    ///0x48 - TIMG_WDTCONFIG0_REG.
    #[inline(always)]
    pub const fn wdtconfig0(&self) -> &WDTCONFIG0 {
        &self.wdtconfig0
    }
    ///0x4c - TIMG_WDTCONFIG1_REG.
    #[inline(always)]
    pub const fn wdtconfig1(&self) -> &WDTCONFIG1 {
        &self.wdtconfig1
    }
    ///0x50 - TIMG_WDTCONFIG2_REG.
    #[inline(always)]
    pub const fn wdtconfig2(&self) -> &WDTCONFIG2 {
        &self.wdtconfig2
    }
    ///0x54 - TIMG_WDTCONFIG3_REG.
    #[inline(always)]
    pub const fn wdtconfig3(&self) -> &WDTCONFIG3 {
        &self.wdtconfig3
    }
    ///0x58 - TIMG_WDTCONFIG4_REG.
    #[inline(always)]
    pub const fn wdtconfig4(&self) -> &WDTCONFIG4 {
        &self.wdtconfig4
    }
    ///0x5c - TIMG_WDTCONFIG5_REG.
    #[inline(always)]
    pub const fn wdtconfig5(&self) -> &WDTCONFIG5 {
        &self.wdtconfig5
    }
    ///0x60 - TIMG_WDTFEED_REG.
    #[inline(always)]
    pub const fn wdtfeed(&self) -> &WDTFEED {
        &self.wdtfeed
    }
    ///0x64 - TIMG_WDTWPROTECT_REG.
    #[inline(always)]
    pub const fn wdtwprotect(&self) -> &WDTWPROTECT {
        &self.wdtwprotect
    }
    ///0x68 - TIMG_RTCCALICFG_REG.
    #[inline(always)]
    pub const fn rtccalicfg(&self) -> &RTCCALICFG {
        &self.rtccalicfg
    }
    ///0x6c - TIMG_RTCCALICFG1_REG.
    #[inline(always)]
    pub const fn rtccalicfg1(&self) -> &RTCCALICFG1 {
        &self.rtccalicfg1
    }
    ///0x70 - INT_ENA_TIMG_REG
    #[inline(always)]
    pub const fn int_ena_timers(&self) -> &INT_ENA_TIMERS {
        &self.int_ena_timers
    }
    ///0x74 - INT_RAW_TIMG_REG
    #[inline(always)]
    pub const fn int_raw_timers(&self) -> &INT_RAW_TIMERS {
        &self.int_raw_timers
    }
    ///0x78 - INT_ST_TIMG_REG
    #[inline(always)]
    pub const fn int_st_timers(&self) -> &INT_ST_TIMERS {
        &self.int_st_timers
    }
    ///0x7c - INT_CLR_TIMG_REG
    #[inline(always)]
    pub const fn int_clr_timers(&self) -> &INT_CLR_TIMERS {
        &self.int_clr_timers
    }
    ///0x80 - TIMG_RTCCALICFG2_REG.
    #[inline(always)]
    pub const fn rtccalicfg2(&self) -> &RTCCALICFG2 {
        &self.rtccalicfg2
    }
    ///0xf8 - TIMG_NTIMG_DATE_REG.
    #[inline(always)]
    pub const fn ntimg_date(&self) -> &NTIMG_DATE {
        &self.ntimg_date
    }
    ///0xfc - TIMG_REGCLK_REG.
    #[inline(always)]
    pub const fn regclk(&self) -> &REGCLK {
        &self.regclk
    }
}
///Cluster T%s, containing T?CONFIG, T?LO, T?HI, T?UPDATE, T?ALARMLO, T?ALARMHI, T?LOADLO, T?LOADHI, T?LOAD
pub use self::t::T;
///Cluster
///Cluster T%s, containing T?CONFIG, T?LO, T?HI, T?UPDATE, T?ALARMLO, T?ALARMHI, T?LOADLO, T?LOADHI, T?LOAD
pub mod t;
/**WDTCONFIG0 (rw) register accessor: TIMG_WDTCONFIG0_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig0`] module*/
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
///TIMG_WDTCONFIG0_REG.
pub mod wdtconfig0;
/**WDTCONFIG1 (rw) register accessor: TIMG_WDTCONFIG1_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig1`] module*/
pub type WDTCONFIG1 = crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>;
///TIMG_WDTCONFIG1_REG.
pub mod wdtconfig1;
/**WDTCONFIG2 (rw) register accessor: TIMG_WDTCONFIG2_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig2`] module*/
pub type WDTCONFIG2 = crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>;
///TIMG_WDTCONFIG2_REG.
pub mod wdtconfig2;
/**WDTCONFIG3 (rw) register accessor: TIMG_WDTCONFIG3_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig3`] module*/
pub type WDTCONFIG3 = crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>;
///TIMG_WDTCONFIG3_REG.
pub mod wdtconfig3;
/**WDTCONFIG4 (rw) register accessor: TIMG_WDTCONFIG4_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig4`] module*/
pub type WDTCONFIG4 = crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>;
///TIMG_WDTCONFIG4_REG.
pub mod wdtconfig4;
/**WDTCONFIG5 (rw) register accessor: TIMG_WDTCONFIG5_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig5`] module*/
pub type WDTCONFIG5 = crate::Reg<wdtconfig5::WDTCONFIG5_SPEC>;
///TIMG_WDTCONFIG5_REG.
pub mod wdtconfig5;
/**WDTFEED (w) register accessor: TIMG_WDTFEED_REG.

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtfeed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtfeed`] module*/
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
///TIMG_WDTFEED_REG.
pub mod wdtfeed;
/**WDTWPROTECT (rw) register accessor: TIMG_WDTWPROTECT_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`wdtwprotect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtwprotect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtwprotect`] module*/
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
///TIMG_WDTWPROTECT_REG.
pub mod wdtwprotect;
/**RTCCALICFG (rw) register accessor: TIMG_RTCCALICFG_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccalicfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtccalicfg`] module*/
pub type RTCCALICFG = crate::Reg<rtccalicfg::RTCCALICFG_SPEC>;
///TIMG_RTCCALICFG_REG.
pub mod rtccalicfg;
/**RTCCALICFG1 (r) register accessor: TIMG_RTCCALICFG1_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtccalicfg1`] module*/
pub type RTCCALICFG1 = crate::Reg<rtccalicfg1::RTCCALICFG1_SPEC>;
///TIMG_RTCCALICFG1_REG.
pub mod rtccalicfg1;
/**INT_ENA_TIMERS (rw) register accessor: INT_ENA_TIMG_REG

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena_timers::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_timers::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena_timers`] module*/
pub type INT_ENA_TIMERS = crate::Reg<int_ena_timers::INT_ENA_TIMERS_SPEC>;
///INT_ENA_TIMG_REG
pub mod int_ena_timers;
/**INT_RAW_TIMERS (r) register accessor: INT_RAW_TIMG_REG

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw_timers::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw_timers`] module*/
pub type INT_RAW_TIMERS = crate::Reg<int_raw_timers::INT_RAW_TIMERS_SPEC>;
///INT_RAW_TIMG_REG
pub mod int_raw_timers;
/**INT_ST_TIMERS (r) register accessor: INT_ST_TIMG_REG

You can [`read`](crate::generic::Reg::read) this register and get [`int_st_timers::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st_timers`] module*/
pub type INT_ST_TIMERS = crate::Reg<int_st_timers::INT_ST_TIMERS_SPEC>;
///INT_ST_TIMG_REG
pub mod int_st_timers;
/**INT_CLR_TIMERS (w) register accessor: INT_CLR_TIMG_REG

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr_timers::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr_timers`] module*/
pub type INT_CLR_TIMERS = crate::Reg<int_clr_timers::INT_CLR_TIMERS_SPEC>;
///INT_CLR_TIMG_REG
pub mod int_clr_timers;
/**RTCCALICFG2 (rw) register accessor: TIMG_RTCCALICFG2_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccalicfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtccalicfg2`] module*/
pub type RTCCALICFG2 = crate::Reg<rtccalicfg2::RTCCALICFG2_SPEC>;
///TIMG_RTCCALICFG2_REG.
pub mod rtccalicfg2;
/**NTIMG_DATE (rw) register accessor: TIMG_NTIMG_DATE_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`ntimg_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ntimg_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ntimg_date`] module*/
pub type NTIMG_DATE = crate::Reg<ntimg_date::NTIMG_DATE_SPEC>;
///TIMG_NTIMG_DATE_REG.
pub mod ntimg_date;
/**REGCLK (rw) register accessor: TIMG_REGCLK_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`regclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@regclk`] module*/
pub type REGCLK = crate::Reg<regclk::REGCLK_SPEC>;
///TIMG_REGCLK_REG.
pub mod regclk;
