#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    t: [T; 2],
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
    lactconfig: LACTCONFIG,
    lactrtc: LACTRTC,
    lactlo: LACTLO,
    lacthi: LACTHI,
    lactupdate: LACTUPDATE,
    lactalarmlo: LACTALARMLO,
    lactalarmhi: LACTALARMHI,
    lactloadlo: LACTLOADLO,
    lactloadhi: LACTLOADHI,
    lactload: LACTLOAD,
    int_ena: INT_ENA,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_clr: INT_CLR,
    _reserved25: [u8; 0x50],
    ntimers_date: NTIMERS_DATE,
    timgclk: TIMGCLK,
}
impl RegisterBlock {
    #[doc = "0x00..0x48 - Cluster T%s, containing T?CONFIG, T?LO, T?HI, T?UPDATE, T?ALARMLO, T?ALARMHI, T?LOADLO, T?LOADHI, T?LOAD"]
    #[inline(always)]
    pub const fn t(&self, n: usize) -> &T {
        &self.t[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x48 - Cluster T%s, containing T?CONFIG, T?LO, T?HI, T?UPDATE, T?ALARMLO, T?ALARMHI, T?LOADLO, T?LOADHI, T?LOAD"]
    #[inline(always)]
    pub fn t_iter(&self) -> impl Iterator<Item = &T> {
        self.t.iter()
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn wdtconfig0(&self) -> &WDTCONFIG0 {
        &self.wdtconfig0
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn wdtconfig1(&self) -> &WDTCONFIG1 {
        &self.wdtconfig1
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn wdtconfig2(&self) -> &WDTCONFIG2 {
        &self.wdtconfig2
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn wdtconfig3(&self) -> &WDTCONFIG3 {
        &self.wdtconfig3
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn wdtconfig4(&self) -> &WDTCONFIG4 {
        &self.wdtconfig4
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn wdtconfig5(&self) -> &WDTCONFIG5 {
        &self.wdtconfig5
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn wdtfeed(&self) -> &WDTFEED {
        &self.wdtfeed
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn wdtwprotect(&self) -> &WDTWPROTECT {
        &self.wdtwprotect
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn rtccalicfg(&self) -> &RTCCALICFG {
        &self.rtccalicfg
    }
    #[doc = "0x6c - "]
    #[inline(always)]
    pub const fn rtccalicfg1(&self) -> &RTCCALICFG1 {
        &self.rtccalicfg1
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn lactconfig(&self) -> &LACTCONFIG {
        &self.lactconfig
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn lactrtc(&self) -> &LACTRTC {
        &self.lactrtc
    }
    #[doc = "0x78 - "]
    #[inline(always)]
    pub const fn lactlo(&self) -> &LACTLO {
        &self.lactlo
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn lacthi(&self) -> &LACTHI {
        &self.lacthi
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn lactupdate(&self) -> &LACTUPDATE {
        &self.lactupdate
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn lactalarmlo(&self) -> &LACTALARMLO {
        &self.lactalarmlo
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn lactalarmhi(&self) -> &LACTALARMHI {
        &self.lactalarmhi
    }
    #[doc = "0x8c - "]
    #[inline(always)]
    pub const fn lactloadlo(&self) -> &LACTLOADLO {
        &self.lactloadlo
    }
    #[doc = "0x90 - "]
    #[inline(always)]
    pub const fn lactloadhi(&self) -> &LACTLOADHI {
        &self.lactloadhi
    }
    #[doc = "0x94 - "]
    #[inline(always)]
    pub const fn lactload(&self) -> &LACTLOAD {
        &self.lactload
    }
    #[doc = "0x98 - "]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x9c - "]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0xa0 - "]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0xa4 - "]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0xf8 - "]
    #[inline(always)]
    pub const fn ntimers_date(&self) -> &NTIMERS_DATE {
        &self.ntimers_date
    }
    #[doc = "0xfc - "]
    #[inline(always)]
    pub const fn timgclk(&self) -> &TIMGCLK {
        &self.timgclk
    }
}
#[doc = "Cluster T%s, containing T?CONFIG, T?LO, T?HI, T?UPDATE, T?ALARMLO, T?ALARMHI, T?LOADLO, T?LOADHI, T?LOAD"]
pub use self::t::T;
#[doc = r"Cluster"]
#[doc = "Cluster T%s, containing T?CONFIG, T?LO, T?HI, T?UPDATE, T?ALARMLO, T?ALARMHI, T?LOADLO, T?LOADHI, T?LOAD"]
pub mod t;
#[doc = "WDTCONFIG0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`wdtconfig0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtconfig0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig0`] module"]
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
#[doc = ""]
pub mod wdtconfig0;
#[doc = "WDTCONFIG1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`wdtconfig1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtconfig1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig1`] module"]
pub type WDTCONFIG1 = crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>;
#[doc = ""]
pub mod wdtconfig1;
#[doc = "WDTCONFIG2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`wdtconfig2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtconfig2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig2`] module"]
pub type WDTCONFIG2 = crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>;
#[doc = ""]
pub mod wdtconfig2;
#[doc = "WDTCONFIG3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`wdtconfig3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtconfig3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig3`] module"]
pub type WDTCONFIG3 = crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>;
#[doc = ""]
pub mod wdtconfig3;
#[doc = "WDTCONFIG4 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`wdtconfig4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtconfig4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig4`] module"]
pub type WDTCONFIG4 = crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>;
#[doc = ""]
pub mod wdtconfig4;
#[doc = "WDTCONFIG5 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`wdtconfig5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtconfig5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig5`] module"]
pub type WDTCONFIG5 = crate::Reg<wdtconfig5::WDTCONFIG5_SPEC>;
#[doc = ""]
pub mod wdtconfig5;
#[doc = "WDTFEED (w) register accessor: \n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtfeed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtfeed`] module"]
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
#[doc = ""]
pub mod wdtfeed;
#[doc = "WDTWPROTECT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`wdtwprotect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtwprotect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtwprotect`] module"]
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
#[doc = ""]
pub mod wdtwprotect;
#[doc = "RTCCALICFG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`rtccalicfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccalicfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccalicfg`] module"]
pub type RTCCALICFG = crate::Reg<rtccalicfg::RTCCALICFG_SPEC>;
#[doc = ""]
pub mod rtccalicfg;
#[doc = "RTCCALICFG1 (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`rtccalicfg1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccalicfg1`] module"]
pub type RTCCALICFG1 = crate::Reg<rtccalicfg1::RTCCALICFG1_SPEC>;
#[doc = ""]
pub mod rtccalicfg1;
#[doc = "LACTCONFIG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lactconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lactconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lactconfig`] module"]
pub type LACTCONFIG = crate::Reg<lactconfig::LACTCONFIG_SPEC>;
#[doc = ""]
pub mod lactconfig;
#[doc = "LACTRTC (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lactrtc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lactrtc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lactrtc`] module"]
pub type LACTRTC = crate::Reg<lactrtc::LACTRTC_SPEC>;
#[doc = ""]
pub mod lactrtc;
#[doc = "LACTLO (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lactlo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lactlo`] module"]
pub type LACTLO = crate::Reg<lactlo::LACTLO_SPEC>;
#[doc = ""]
pub mod lactlo;
#[doc = "LACTHI (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lacthi::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lacthi`] module"]
pub type LACTHI = crate::Reg<lacthi::LACTHI_SPEC>;
#[doc = ""]
pub mod lacthi;
#[doc = "LACTUPDATE (w) register accessor: \n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lactupdate::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lactupdate`] module"]
pub type LACTUPDATE = crate::Reg<lactupdate::LACTUPDATE_SPEC>;
#[doc = ""]
pub mod lactupdate;
#[doc = "LACTALARMLO (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lactalarmlo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lactalarmlo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lactalarmlo`] module"]
pub type LACTALARMLO = crate::Reg<lactalarmlo::LACTALARMLO_SPEC>;
#[doc = ""]
pub mod lactalarmlo;
#[doc = "LACTALARMHI (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lactalarmhi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lactalarmhi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lactalarmhi`] module"]
pub type LACTALARMHI = crate::Reg<lactalarmhi::LACTALARMHI_SPEC>;
#[doc = ""]
pub mod lactalarmhi;
#[doc = "LACTLOADLO (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lactloadlo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lactloadlo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lactloadlo`] module"]
pub type LACTLOADLO = crate::Reg<lactloadlo::LACTLOADLO_SPEC>;
#[doc = ""]
pub mod lactloadlo;
#[doc = "LACTLOADHI (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lactloadhi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lactloadhi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lactloadhi`] module"]
pub type LACTLOADHI = crate::Reg<lactloadhi::LACTLOADHI_SPEC>;
#[doc = ""]
pub mod lactloadhi;
#[doc = "LACTLOAD (w) register accessor: \n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lactload::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lactload`] module"]
pub type LACTLOAD = crate::Reg<lactload::LACTLOAD_SPEC>;
#[doc = ""]
pub mod lactload;
#[doc = "INT_ENA (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = ""]
pub mod int_ena;
#[doc = "INT_RAW (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = ""]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = ""]
pub mod int_st;
#[doc = "INT_CLR (w) register accessor: \n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = ""]
pub mod int_clr;
#[doc = "NTIMERS_DATE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ntimers_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ntimers_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ntimers_date`] module"]
pub type NTIMERS_DATE = crate::Reg<ntimers_date::NTIMERS_DATE_SPEC>;
#[doc = ""]
pub mod ntimers_date;
#[doc = "TIMGCLK (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`timgclk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timgclk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timgclk`] module"]
pub type TIMGCLK = crate::Reg<timgclk::TIMGCLK_SPEC>;
#[doc = ""]
pub mod timgclk;
