#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    t0config: T0CONFIG,
    t0lo: T0LO,
    t0hi: T0HI,
    t0update: T0UPDATE,
    t0alarmlo: T0ALARMLO,
    t0alarmhi: T0ALARMHI,
    t0loadlo: T0LOADLO,
    t0loadhi: T0LOADHI,
    t0load: T0LOAD,
    _reserved9: [u8; 0x24],
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
    _reserved24: [u8; 0x74],
    ntimg_date: NTIMG_DATE,
    regclk: REGCLK,
}
impl RegisterBlock {
    #[doc = "0x00 - TIMG_T0CONFIG_REG."]
    #[inline(always)]
    pub const fn t0config(&self) -> &T0CONFIG {
        &self.t0config
    }
    #[doc = "0x04 - TIMG_T0LO_REG."]
    #[inline(always)]
    pub const fn t0lo(&self) -> &T0LO {
        &self.t0lo
    }
    #[doc = "0x08 - TIMG_T0HI_REG."]
    #[inline(always)]
    pub const fn t0hi(&self) -> &T0HI {
        &self.t0hi
    }
    #[doc = "0x0c - TIMG_T0UPDATE_REG."]
    #[inline(always)]
    pub const fn t0update(&self) -> &T0UPDATE {
        &self.t0update
    }
    #[doc = "0x10 - TIMG_T0ALARMLO_REG."]
    #[inline(always)]
    pub const fn t0alarmlo(&self) -> &T0ALARMLO {
        &self.t0alarmlo
    }
    #[doc = "0x14 - TIMG_T0ALARMHI_REG."]
    #[inline(always)]
    pub const fn t0alarmhi(&self) -> &T0ALARMHI {
        &self.t0alarmhi
    }
    #[doc = "0x18 - TIMG_T0LOADLO_REG."]
    #[inline(always)]
    pub const fn t0loadlo(&self) -> &T0LOADLO {
        &self.t0loadlo
    }
    #[doc = "0x1c - TIMG_T0LOADHI_REG."]
    #[inline(always)]
    pub const fn t0loadhi(&self) -> &T0LOADHI {
        &self.t0loadhi
    }
    #[doc = "0x20 - TIMG_T0LOAD_REG."]
    #[inline(always)]
    pub const fn t0load(&self) -> &T0LOAD {
        &self.t0load
    }
    #[doc = "0x48 - TIMG_WDTCONFIG0_REG."]
    #[inline(always)]
    pub const fn wdtconfig0(&self) -> &WDTCONFIG0 {
        &self.wdtconfig0
    }
    #[doc = "0x4c - TIMG_WDTCONFIG1_REG."]
    #[inline(always)]
    pub const fn wdtconfig1(&self) -> &WDTCONFIG1 {
        &self.wdtconfig1
    }
    #[doc = "0x50 - TIMG_WDTCONFIG2_REG."]
    #[inline(always)]
    pub const fn wdtconfig2(&self) -> &WDTCONFIG2 {
        &self.wdtconfig2
    }
    #[doc = "0x54 - TIMG_WDTCONFIG3_REG."]
    #[inline(always)]
    pub const fn wdtconfig3(&self) -> &WDTCONFIG3 {
        &self.wdtconfig3
    }
    #[doc = "0x58 - TIMG_WDTCONFIG4_REG."]
    #[inline(always)]
    pub const fn wdtconfig4(&self) -> &WDTCONFIG4 {
        &self.wdtconfig4
    }
    #[doc = "0x5c - TIMG_WDTCONFIG5_REG."]
    #[inline(always)]
    pub const fn wdtconfig5(&self) -> &WDTCONFIG5 {
        &self.wdtconfig5
    }
    #[doc = "0x60 - TIMG_WDTFEED_REG."]
    #[inline(always)]
    pub const fn wdtfeed(&self) -> &WDTFEED {
        &self.wdtfeed
    }
    #[doc = "0x64 - TIMG_WDTWPROTECT_REG."]
    #[inline(always)]
    pub const fn wdtwprotect(&self) -> &WDTWPROTECT {
        &self.wdtwprotect
    }
    #[doc = "0x68 - TIMG_RTCCALICFG_REG."]
    #[inline(always)]
    pub const fn rtccalicfg(&self) -> &RTCCALICFG {
        &self.rtccalicfg
    }
    #[doc = "0x6c - TIMG_RTCCALICFG1_REG."]
    #[inline(always)]
    pub const fn rtccalicfg1(&self) -> &RTCCALICFG1 {
        &self.rtccalicfg1
    }
    #[doc = "0x70 - INT_ENA_TIMG_REG"]
    #[inline(always)]
    pub const fn int_ena_timers(&self) -> &INT_ENA_TIMERS {
        &self.int_ena_timers
    }
    #[doc = "0x74 - INT_RAW_TIMG_REG"]
    #[inline(always)]
    pub const fn int_raw_timers(&self) -> &INT_RAW_TIMERS {
        &self.int_raw_timers
    }
    #[doc = "0x78 - INT_ST_TIMG_REG"]
    #[inline(always)]
    pub const fn int_st_timers(&self) -> &INT_ST_TIMERS {
        &self.int_st_timers
    }
    #[doc = "0x7c - INT_CLR_TIMG_REG"]
    #[inline(always)]
    pub const fn int_clr_timers(&self) -> &INT_CLR_TIMERS {
        &self.int_clr_timers
    }
    #[doc = "0x80 - TIMG_RTCCALICFG2_REG."]
    #[inline(always)]
    pub const fn rtccalicfg2(&self) -> &RTCCALICFG2 {
        &self.rtccalicfg2
    }
    #[doc = "0xf8 - TIMG_NTIMG_DATE_REG."]
    #[inline(always)]
    pub const fn ntimg_date(&self) -> &NTIMG_DATE {
        &self.ntimg_date
    }
    #[doc = "0xfc - TIMG_REGCLK_REG."]
    #[inline(always)]
    pub const fn regclk(&self) -> &REGCLK {
        &self.regclk
    }
}
#[doc = "T0CONFIG (rw) register accessor: TIMG_T0CONFIG_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0config`] module"]
pub type T0CONFIG = crate::Reg<t0config::T0CONFIG_SPEC>;
#[doc = "TIMG_T0CONFIG_REG."]
pub mod t0config;
#[doc = "T0LO (r) register accessor: TIMG_T0LO_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0lo`] module"]
pub type T0LO = crate::Reg<t0lo::T0LO_SPEC>;
#[doc = "TIMG_T0LO_REG."]
pub mod t0lo;
#[doc = "T0HI (r) register accessor: TIMG_T0HI_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0hi`] module"]
pub type T0HI = crate::Reg<t0hi::T0HI_SPEC>;
#[doc = "TIMG_T0HI_REG."]
pub mod t0hi;
#[doc = "T0UPDATE (rw) register accessor: TIMG_T0UPDATE_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0update::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0update::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0update`] module"]
pub type T0UPDATE = crate::Reg<t0update::T0UPDATE_SPEC>;
#[doc = "TIMG_T0UPDATE_REG."]
pub mod t0update;
#[doc = "T0ALARMLO (rw) register accessor: TIMG_T0ALARMLO_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0alarmlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0alarmlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0alarmlo`] module"]
pub type T0ALARMLO = crate::Reg<t0alarmlo::T0ALARMLO_SPEC>;
#[doc = "TIMG_T0ALARMLO_REG."]
pub mod t0alarmlo;
#[doc = "T0ALARMHI (rw) register accessor: TIMG_T0ALARMHI_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0alarmhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0alarmhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0alarmhi`] module"]
pub type T0ALARMHI = crate::Reg<t0alarmhi::T0ALARMHI_SPEC>;
#[doc = "TIMG_T0ALARMHI_REG."]
pub mod t0alarmhi;
#[doc = "T0LOADLO (rw) register accessor: TIMG_T0LOADLO_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0loadlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0loadlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0loadlo`] module"]
pub type T0LOADLO = crate::Reg<t0loadlo::T0LOADLO_SPEC>;
#[doc = "TIMG_T0LOADLO_REG."]
pub mod t0loadlo;
#[doc = "T0LOADHI (rw) register accessor: TIMG_T0LOADHI_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0loadhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0loadhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0loadhi`] module"]
pub type T0LOADHI = crate::Reg<t0loadhi::T0LOADHI_SPEC>;
#[doc = "TIMG_T0LOADHI_REG."]
pub mod t0loadhi;
#[doc = "T0LOAD (w) register accessor: TIMG_T0LOAD_REG.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0load::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0load`] module"]
pub type T0LOAD = crate::Reg<t0load::T0LOAD_SPEC>;
#[doc = "TIMG_T0LOAD_REG."]
pub mod t0load;
#[doc = "WDTCONFIG0 (rw) register accessor: TIMG_WDTCONFIG0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig0`] module"]
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
#[doc = "TIMG_WDTCONFIG0_REG."]
pub mod wdtconfig0;
#[doc = "WDTCONFIG1 (rw) register accessor: TIMG_WDTCONFIG1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig1`] module"]
pub type WDTCONFIG1 = crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>;
#[doc = "TIMG_WDTCONFIG1_REG."]
pub mod wdtconfig1;
#[doc = "WDTCONFIG2 (rw) register accessor: TIMG_WDTCONFIG2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig2`] module"]
pub type WDTCONFIG2 = crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>;
#[doc = "TIMG_WDTCONFIG2_REG."]
pub mod wdtconfig2;
#[doc = "WDTCONFIG3 (rw) register accessor: TIMG_WDTCONFIG3_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig3`] module"]
pub type WDTCONFIG3 = crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>;
#[doc = "TIMG_WDTCONFIG3_REG."]
pub mod wdtconfig3;
#[doc = "WDTCONFIG4 (rw) register accessor: TIMG_WDTCONFIG4_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig4`] module"]
pub type WDTCONFIG4 = crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>;
#[doc = "TIMG_WDTCONFIG4_REG."]
pub mod wdtconfig4;
#[doc = "WDTCONFIG5 (rw) register accessor: TIMG_WDTCONFIG5_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig5`] module"]
pub type WDTCONFIG5 = crate::Reg<wdtconfig5::WDTCONFIG5_SPEC>;
#[doc = "TIMG_WDTCONFIG5_REG."]
pub mod wdtconfig5;
#[doc = "WDTFEED (w) register accessor: TIMG_WDTFEED_REG.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtfeed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtfeed`] module"]
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
#[doc = "TIMG_WDTFEED_REG."]
pub mod wdtfeed;
#[doc = "WDTWPROTECT (rw) register accessor: TIMG_WDTWPROTECT_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtwprotect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtwprotect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtwprotect`] module"]
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
#[doc = "TIMG_WDTWPROTECT_REG."]
pub mod wdtwprotect;
#[doc = "RTCCALICFG (rw) register accessor: TIMG_RTCCALICFG_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccalicfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccalicfg`] module"]
pub type RTCCALICFG = crate::Reg<rtccalicfg::RTCCALICFG_SPEC>;
#[doc = "TIMG_RTCCALICFG_REG."]
pub mod rtccalicfg;
#[doc = "RTCCALICFG1 (r) register accessor: TIMG_RTCCALICFG1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccalicfg1`] module"]
pub type RTCCALICFG1 = crate::Reg<rtccalicfg1::RTCCALICFG1_SPEC>;
#[doc = "TIMG_RTCCALICFG1_REG."]
pub mod rtccalicfg1;
#[doc = "INT_ENA_TIMERS (rw) register accessor: INT_ENA_TIMG_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena_timers::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_timers::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena_timers`] module"]
pub type INT_ENA_TIMERS = crate::Reg<int_ena_timers::INT_ENA_TIMERS_SPEC>;
#[doc = "INT_ENA_TIMG_REG"]
pub mod int_ena_timers;
#[doc = "INT_RAW_TIMERS (r) register accessor: INT_RAW_TIMG_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw_timers::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw_timers`] module"]
pub type INT_RAW_TIMERS = crate::Reg<int_raw_timers::INT_RAW_TIMERS_SPEC>;
#[doc = "INT_RAW_TIMG_REG"]
pub mod int_raw_timers;
#[doc = "INT_ST_TIMERS (r) register accessor: INT_ST_TIMG_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_timers::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_timers`] module"]
pub type INT_ST_TIMERS = crate::Reg<int_st_timers::INT_ST_TIMERS_SPEC>;
#[doc = "INT_ST_TIMG_REG"]
pub mod int_st_timers;
#[doc = "INT_CLR_TIMERS (w) register accessor: INT_CLR_TIMG_REG\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr_timers::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr_timers`] module"]
pub type INT_CLR_TIMERS = crate::Reg<int_clr_timers::INT_CLR_TIMERS_SPEC>;
#[doc = "INT_CLR_TIMG_REG"]
pub mod int_clr_timers;
#[doc = "RTCCALICFG2 (rw) register accessor: TIMG_RTCCALICFG2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccalicfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccalicfg2`] module"]
pub type RTCCALICFG2 = crate::Reg<rtccalicfg2::RTCCALICFG2_SPEC>;
#[doc = "TIMG_RTCCALICFG2_REG."]
pub mod rtccalicfg2;
#[doc = "NTIMG_DATE (rw) register accessor: TIMG_NTIMG_DATE_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ntimg_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ntimg_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ntimg_date`] module"]
pub type NTIMG_DATE = crate::Reg<ntimg_date::NTIMG_DATE_SPEC>;
#[doc = "TIMG_NTIMG_DATE_REG."]
pub mod ntimg_date;
#[doc = "REGCLK (rw) register accessor: TIMG_REGCLK_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regclk`] module"]
pub type REGCLK = crate::Reg<regclk::REGCLK_SPEC>;
#[doc = "TIMG_REGCLK_REG."]
pub mod regclk;
