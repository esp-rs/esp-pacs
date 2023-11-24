#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
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
    t1config: T1CONFIG,
    t1lo: T1LO,
    t1hi: T1HI,
    t1update: T1UPDATE,
    t1alarmlo: T1ALARMLO,
    t1alarmhi: T1ALARMHI,
    t1loadlo: T1LOADLO,
    t1loadhi: T1LOADHI,
    t1load: T1LOAD,
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
    int_ena_timers: INT_ENA_TIMERS,
    int_raw_timers: INT_RAW_TIMERS,
    int_st_timers: INT_ST_TIMERS,
    int_clr_timers: INT_CLR_TIMERS,
    _reserved42: [u8; 0x50],
    ntimers_date: NTIMERS_DATE,
    timgclk: TIMGCLK,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn t0config(&self) -> &T0CONFIG {
        &self.t0config
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn t0lo(&self) -> &T0LO {
        &self.t0lo
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn t0hi(&self) -> &T0HI {
        &self.t0hi
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn t0update(&self) -> &T0UPDATE {
        &self.t0update
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn t0alarmlo(&self) -> &T0ALARMLO {
        &self.t0alarmlo
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn t0alarmhi(&self) -> &T0ALARMHI {
        &self.t0alarmhi
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn t0loadlo(&self) -> &T0LOADLO {
        &self.t0loadlo
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn t0loadhi(&self) -> &T0LOADHI {
        &self.t0loadhi
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn t0load(&self) -> &T0LOAD {
        &self.t0load
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn t1config(&self) -> &T1CONFIG {
        &self.t1config
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn t1lo(&self) -> &T1LO {
        &self.t1lo
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn t1hi(&self) -> &T1HI {
        &self.t1hi
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn t1update(&self) -> &T1UPDATE {
        &self.t1update
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn t1alarmlo(&self) -> &T1ALARMLO {
        &self.t1alarmlo
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn t1alarmhi(&self) -> &T1ALARMHI {
        &self.t1alarmhi
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn t1loadlo(&self) -> &T1LOADLO {
        &self.t1loadlo
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn t1loadhi(&self) -> &T1LOADHI {
        &self.t1loadhi
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn t1load(&self) -> &T1LOAD {
        &self.t1load
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
    pub const fn int_ena_timers(&self) -> &INT_ENA_TIMERS {
        &self.int_ena_timers
    }
    #[doc = "0x9c - "]
    #[inline(always)]
    pub const fn int_raw_timers(&self) -> &INT_RAW_TIMERS {
        &self.int_raw_timers
    }
    #[doc = "0xa0 - "]
    #[inline(always)]
    pub const fn int_st_timers(&self) -> &INT_ST_TIMERS {
        &self.int_st_timers
    }
    #[doc = "0xa4 - "]
    #[inline(always)]
    pub const fn int_clr_timers(&self) -> &INT_CLR_TIMERS {
        &self.int_clr_timers
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
#[doc = "T0CONFIG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0config`] module"]
pub type T0CONFIG = crate::Reg<t0config::T0CONFIG_SPEC>;
#[doc = ""]
pub mod t0config;
#[doc = "T0LO (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0lo`] module"]
pub type T0LO = crate::Reg<t0lo::T0LO_SPEC>;
#[doc = ""]
pub mod t0lo;
#[doc = "T0HI (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0hi`] module"]
pub type T0HI = crate::Reg<t0hi::T0HI_SPEC>;
#[doc = ""]
pub mod t0hi;
#[doc = "T0UPDATE (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0update::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0update`] module"]
pub type T0UPDATE = crate::Reg<t0update::T0UPDATE_SPEC>;
#[doc = ""]
pub mod t0update;
#[doc = "T0ALARMLO (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0alarmlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0alarmlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0alarmlo`] module"]
pub type T0ALARMLO = crate::Reg<t0alarmlo::T0ALARMLO_SPEC>;
#[doc = ""]
pub mod t0alarmlo;
#[doc = "T0ALARMHI (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0alarmhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0alarmhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0alarmhi`] module"]
pub type T0ALARMHI = crate::Reg<t0alarmhi::T0ALARMHI_SPEC>;
#[doc = ""]
pub mod t0alarmhi;
#[doc = "T0LOADLO (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0loadlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0loadlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0loadlo`] module"]
pub type T0LOADLO = crate::Reg<t0loadlo::T0LOADLO_SPEC>;
#[doc = ""]
pub mod t0loadlo;
#[doc = "T0LOADHI (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0loadhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0loadhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0loadhi`] module"]
pub type T0LOADHI = crate::Reg<t0loadhi::T0LOADHI_SPEC>;
#[doc = ""]
pub mod t0loadhi;
#[doc = "T0LOAD (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0load::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0load`] module"]
pub type T0LOAD = crate::Reg<t0load::T0LOAD_SPEC>;
#[doc = ""]
pub mod t0load;
#[doc = "T1CONFIG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1config`] module"]
pub type T1CONFIG = crate::Reg<t1config::T1CONFIG_SPEC>;
#[doc = ""]
pub mod t1config;
#[doc = "T1LO (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1lo`] module"]
pub type T1LO = crate::Reg<t1lo::T1LO_SPEC>;
#[doc = ""]
pub mod t1lo;
#[doc = "T1HI (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1hi`] module"]
pub type T1HI = crate::Reg<t1hi::T1HI_SPEC>;
#[doc = ""]
pub mod t1hi;
#[doc = "T1UPDATE (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1update::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1update`] module"]
pub type T1UPDATE = crate::Reg<t1update::T1UPDATE_SPEC>;
#[doc = ""]
pub mod t1update;
#[doc = "T1ALARMLO (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1alarmlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1alarmlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1alarmlo`] module"]
pub type T1ALARMLO = crate::Reg<t1alarmlo::T1ALARMLO_SPEC>;
#[doc = ""]
pub mod t1alarmlo;
#[doc = "T1ALARMHI (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1alarmhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1alarmhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1alarmhi`] module"]
pub type T1ALARMHI = crate::Reg<t1alarmhi::T1ALARMHI_SPEC>;
#[doc = ""]
pub mod t1alarmhi;
#[doc = "T1LOADLO (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1loadlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1loadlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1loadlo`] module"]
pub type T1LOADLO = crate::Reg<t1loadlo::T1LOADLO_SPEC>;
#[doc = ""]
pub mod t1loadlo;
#[doc = "T1LOADHI (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1loadhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1loadhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1loadhi`] module"]
pub type T1LOADHI = crate::Reg<t1loadhi::T1LOADHI_SPEC>;
#[doc = ""]
pub mod t1loadhi;
#[doc = "T1LOAD (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1load::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1load`] module"]
pub type T1LOAD = crate::Reg<t1load::T1LOAD_SPEC>;
#[doc = ""]
pub mod t1load;
#[doc = "WDTCONFIG0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig0`] module"]
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
#[doc = ""]
pub mod wdtconfig0;
#[doc = "WDTCONFIG1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig1`] module"]
pub type WDTCONFIG1 = crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>;
#[doc = ""]
pub mod wdtconfig1;
#[doc = "WDTCONFIG2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig2`] module"]
pub type WDTCONFIG2 = crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>;
#[doc = ""]
pub mod wdtconfig2;
#[doc = "WDTCONFIG3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig3`] module"]
pub type WDTCONFIG3 = crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>;
#[doc = ""]
pub mod wdtconfig3;
#[doc = "WDTCONFIG4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig4`] module"]
pub type WDTCONFIG4 = crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>;
#[doc = ""]
pub mod wdtconfig4;
#[doc = "WDTCONFIG5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig5`] module"]
pub type WDTCONFIG5 = crate::Reg<wdtconfig5::WDTCONFIG5_SPEC>;
#[doc = ""]
pub mod wdtconfig5;
#[doc = "WDTFEED (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtfeed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtfeed`] module"]
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
#[doc = ""]
pub mod wdtfeed;
#[doc = "WDTWPROTECT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtwprotect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtwprotect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtwprotect`] module"]
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
#[doc = ""]
pub mod wdtwprotect;
#[doc = "RTCCALICFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccalicfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccalicfg`] module"]
pub type RTCCALICFG = crate::Reg<rtccalicfg::RTCCALICFG_SPEC>;
#[doc = ""]
pub mod rtccalicfg;
#[doc = "RTCCALICFG1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccalicfg1`] module"]
pub type RTCCALICFG1 = crate::Reg<rtccalicfg1::RTCCALICFG1_SPEC>;
#[doc = ""]
pub mod rtccalicfg1;
#[doc = "LACTCONFIG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactconfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactconfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lactconfig`] module"]
pub type LACTCONFIG = crate::Reg<lactconfig::LACTCONFIG_SPEC>;
#[doc = ""]
pub mod lactconfig;
#[doc = "LACTRTC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactrtc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactrtc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lactrtc`] module"]
pub type LACTRTC = crate::Reg<lactrtc::LACTRTC_SPEC>;
#[doc = ""]
pub mod lactrtc;
#[doc = "LACTLO (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactlo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lactlo`] module"]
pub type LACTLO = crate::Reg<lactlo::LACTLO_SPEC>;
#[doc = ""]
pub mod lactlo;
#[doc = "LACTHI (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lacthi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lacthi`] module"]
pub type LACTHI = crate::Reg<lacthi::LACTHI_SPEC>;
#[doc = ""]
pub mod lacthi;
#[doc = "LACTUPDATE (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactupdate::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lactupdate`] module"]
pub type LACTUPDATE = crate::Reg<lactupdate::LACTUPDATE_SPEC>;
#[doc = ""]
pub mod lactupdate;
#[doc = "LACTALARMLO (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactalarmlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactalarmlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lactalarmlo`] module"]
pub type LACTALARMLO = crate::Reg<lactalarmlo::LACTALARMLO_SPEC>;
#[doc = ""]
pub mod lactalarmlo;
#[doc = "LACTALARMHI (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactalarmhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactalarmhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lactalarmhi`] module"]
pub type LACTALARMHI = crate::Reg<lactalarmhi::LACTALARMHI_SPEC>;
#[doc = ""]
pub mod lactalarmhi;
#[doc = "LACTLOADLO (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactloadlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactloadlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lactloadlo`] module"]
pub type LACTLOADLO = crate::Reg<lactloadlo::LACTLOADLO_SPEC>;
#[doc = ""]
pub mod lactloadlo;
#[doc = "LACTLOADHI (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactloadhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactloadhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lactloadhi`] module"]
pub type LACTLOADHI = crate::Reg<lactloadhi::LACTLOADHI_SPEC>;
#[doc = ""]
pub mod lactloadhi;
#[doc = "LACTLOAD (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactload::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lactload`] module"]
pub type LACTLOAD = crate::Reg<lactload::LACTLOAD_SPEC>;
#[doc = ""]
pub mod lactload;
#[doc = "INT_ENA_TIMERS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena_timers::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_timers::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena_timers`] module"]
pub type INT_ENA_TIMERS = crate::Reg<int_ena_timers::INT_ENA_TIMERS_SPEC>;
#[doc = ""]
pub mod int_ena_timers;
#[doc = "INT_RAW_TIMERS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw_timers::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw_timers`] module"]
pub type INT_RAW_TIMERS = crate::Reg<int_raw_timers::INT_RAW_TIMERS_SPEC>;
#[doc = ""]
pub mod int_raw_timers;
#[doc = "INT_ST_TIMERS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_timers::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_timers`] module"]
pub type INT_ST_TIMERS = crate::Reg<int_st_timers::INT_ST_TIMERS_SPEC>;
#[doc = ""]
pub mod int_st_timers;
#[doc = "INT_CLR_TIMERS (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr_timers::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr_timers`] module"]
pub type INT_CLR_TIMERS = crate::Reg<int_clr_timers::INT_CLR_TIMERS_SPEC>;
#[doc = ""]
pub mod int_clr_timers;
#[doc = "NTIMERS_DATE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ntimers_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ntimers_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ntimers_date`] module"]
pub type NTIMERS_DATE = crate::Reg<ntimers_date::NTIMERS_DATE_SPEC>;
#[doc = ""]
pub mod ntimers_date;
#[doc = "TIMGCLK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timgclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timgclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timgclk`] module"]
pub type TIMGCLK = crate::Reg<timgclk::TIMGCLK_SPEC>;
#[doc = ""]
pub mod timgclk;
