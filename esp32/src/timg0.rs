#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub t0config: T0CONFIG,
    #[doc = "0x04 - "]
    pub t0lo: T0LO,
    #[doc = "0x08 - "]
    pub t0hi: T0HI,
    #[doc = "0x0c - "]
    pub t0update: T0UPDATE,
    #[doc = "0x10 - "]
    pub t0alarmlo: T0ALARMLO,
    #[doc = "0x14 - "]
    pub t0alarmhi: T0ALARMHI,
    #[doc = "0x18 - "]
    pub t0loadlo: T0LOADLO,
    #[doc = "0x1c - "]
    pub t0loadhi: T0LOADHI,
    #[doc = "0x20 - "]
    pub t0load: T0LOAD,
    #[doc = "0x24 - "]
    pub t1config: T1CONFIG,
    #[doc = "0x28 - "]
    pub t1lo: T1LO,
    #[doc = "0x2c - "]
    pub t1hi: T1HI,
    #[doc = "0x30 - "]
    pub t1update: T1UPDATE,
    #[doc = "0x34 - "]
    pub t1alarmlo: T1ALARMLO,
    #[doc = "0x38 - "]
    pub t1alarmhi: T1ALARMHI,
    #[doc = "0x3c - "]
    pub t1loadlo: T1LOADLO,
    #[doc = "0x40 - "]
    pub t1loadhi: T1LOADHI,
    #[doc = "0x44 - "]
    pub t1load: T1LOAD,
    #[doc = "0x48 - "]
    pub wdtconfig0: WDTCONFIG0,
    #[doc = "0x4c - "]
    pub wdtconfig1: WDTCONFIG1,
    #[doc = "0x50 - "]
    pub wdtconfig2: WDTCONFIG2,
    #[doc = "0x54 - "]
    pub wdtconfig3: WDTCONFIG3,
    #[doc = "0x58 - "]
    pub wdtconfig4: WDTCONFIG4,
    #[doc = "0x5c - "]
    pub wdtconfig5: WDTCONFIG5,
    #[doc = "0x60 - "]
    pub wdtfeed: WDTFEED,
    #[doc = "0x64 - "]
    pub wdtwprotect: WDTWPROTECT,
    #[doc = "0x68 - "]
    pub rtccalicfg: RTCCALICFG,
    #[doc = "0x6c - "]
    pub rtccalicfg1: RTCCALICFG1,
    #[doc = "0x70 - "]
    pub lactconfig: LACTCONFIG,
    #[doc = "0x74 - "]
    pub lactrtc: LACTRTC,
    #[doc = "0x78 - "]
    pub lactlo: LACTLO,
    #[doc = "0x7c - "]
    pub lacthi: LACTHI,
    #[doc = "0x80 - "]
    pub lactupdate: LACTUPDATE,
    #[doc = "0x84 - "]
    pub lactalarmlo: LACTALARMLO,
    #[doc = "0x88 - "]
    pub lactalarmhi: LACTALARMHI,
    #[doc = "0x8c - "]
    pub lactloadlo: LACTLOADLO,
    #[doc = "0x90 - "]
    pub lactloadhi: LACTLOADHI,
    #[doc = "0x94 - "]
    pub lactload: LACTLOAD,
    #[doc = "0x98 - "]
    pub int_ena_timers: INT_ENA_TIMERS,
    #[doc = "0x9c - "]
    pub int_raw_timers: INT_RAW_TIMERS,
    #[doc = "0xa0 - "]
    pub int_st_timers: INT_ST_TIMERS,
    #[doc = "0xa4 - "]
    pub int_clr_timers: INT_CLR_TIMERS,
    _reserved42: [u8; 0x50],
    #[doc = "0xf8 - "]
    pub ntimers_date: NTIMERS_DATE,
    #[doc = "0xfc - "]
    pub timgclk: TIMGCLK,
}
#[doc = "T0CONFIG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`t0config`] module"]
pub type T0CONFIG = crate::Reg<t0config::T0CONFIG_SPEC>;
#[doc = ""]
pub mod t0config;
#[doc = "T0LO (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`t0lo`] module"]
pub type T0LO = crate::Reg<t0lo::T0LO_SPEC>;
#[doc = ""]
pub mod t0lo;
#[doc = "T0HI (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`t0hi`] module"]
pub type T0HI = crate::Reg<t0hi::T0HI_SPEC>;
#[doc = ""]
pub mod t0hi;
#[doc = "T0UPDATE (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0update::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`t0update`] module"]
pub type T0UPDATE = crate::Reg<t0update::T0UPDATE_SPEC>;
#[doc = ""]
pub mod t0update;
#[doc = "T0ALARMLO (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0alarmlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0alarmlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`t0alarmlo`] module"]
pub type T0ALARMLO = crate::Reg<t0alarmlo::T0ALARMLO_SPEC>;
#[doc = ""]
pub mod t0alarmlo;
#[doc = "T0ALARMHI (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0alarmhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0alarmhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`t0alarmhi`] module"]
pub type T0ALARMHI = crate::Reg<t0alarmhi::T0ALARMHI_SPEC>;
#[doc = ""]
pub mod t0alarmhi;
#[doc = "T0LOADLO (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0loadlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0loadlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`t0loadlo`] module"]
pub type T0LOADLO = crate::Reg<t0loadlo::T0LOADLO_SPEC>;
#[doc = ""]
pub mod t0loadlo;
#[doc = "T0LOADHI (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0loadhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0loadhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`t0loadhi`] module"]
pub type T0LOADHI = crate::Reg<t0loadhi::T0LOADHI_SPEC>;
#[doc = ""]
pub mod t0loadhi;
#[doc = "T0LOAD (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0load::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`t0load`] module"]
pub type T0LOAD = crate::Reg<t0load::T0LOAD_SPEC>;
#[doc = ""]
pub mod t0load;
#[doc = "T1CONFIG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`t1config`] module"]
pub type T1CONFIG = crate::Reg<t1config::T1CONFIG_SPEC>;
#[doc = ""]
pub mod t1config;
#[doc = "T1LO (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`t1lo`] module"]
pub type T1LO = crate::Reg<t1lo::T1LO_SPEC>;
#[doc = ""]
pub mod t1lo;
#[doc = "T1HI (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`t1hi`] module"]
pub type T1HI = crate::Reg<t1hi::T1HI_SPEC>;
#[doc = ""]
pub mod t1hi;
#[doc = "T1UPDATE (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1update::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`t1update`] module"]
pub type T1UPDATE = crate::Reg<t1update::T1UPDATE_SPEC>;
#[doc = ""]
pub mod t1update;
#[doc = "T1ALARMLO (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1alarmlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1alarmlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`t1alarmlo`] module"]
pub type T1ALARMLO = crate::Reg<t1alarmlo::T1ALARMLO_SPEC>;
#[doc = ""]
pub mod t1alarmlo;
#[doc = "T1ALARMHI (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1alarmhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1alarmhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`t1alarmhi`] module"]
pub type T1ALARMHI = crate::Reg<t1alarmhi::T1ALARMHI_SPEC>;
#[doc = ""]
pub mod t1alarmhi;
#[doc = "T1LOADLO (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1loadlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1loadlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`t1loadlo`] module"]
pub type T1LOADLO = crate::Reg<t1loadlo::T1LOADLO_SPEC>;
#[doc = ""]
pub mod t1loadlo;
#[doc = "T1LOADHI (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1loadhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1loadhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`t1loadhi`] module"]
pub type T1LOADHI = crate::Reg<t1loadhi::T1LOADHI_SPEC>;
#[doc = ""]
pub mod t1loadhi;
#[doc = "T1LOAD (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1load::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`t1load`] module"]
pub type T1LOAD = crate::Reg<t1load::T1LOAD_SPEC>;
#[doc = ""]
pub mod t1load;
#[doc = "WDTCONFIG0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtconfig0`] module"]
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
#[doc = ""]
pub mod wdtconfig0;
#[doc = "WDTCONFIG1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtconfig1`] module"]
pub type WDTCONFIG1 = crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>;
#[doc = ""]
pub mod wdtconfig1;
#[doc = "WDTCONFIG2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtconfig2`] module"]
pub type WDTCONFIG2 = crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>;
#[doc = ""]
pub mod wdtconfig2;
#[doc = "WDTCONFIG3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtconfig3`] module"]
pub type WDTCONFIG3 = crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>;
#[doc = ""]
pub mod wdtconfig3;
#[doc = "WDTCONFIG4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtconfig4`] module"]
pub type WDTCONFIG4 = crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>;
#[doc = ""]
pub mod wdtconfig4;
#[doc = "WDTCONFIG5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtconfig5`] module"]
pub type WDTCONFIG5 = crate::Reg<wdtconfig5::WDTCONFIG5_SPEC>;
#[doc = ""]
pub mod wdtconfig5;
#[doc = "WDTFEED (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtfeed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtfeed`] module"]
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
#[doc = ""]
pub mod wdtfeed;
#[doc = "WDTWPROTECT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtwprotect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtwprotect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtwprotect`] module"]
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
#[doc = ""]
pub mod wdtwprotect;
#[doc = "RTCCALICFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccalicfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtccalicfg`] module"]
pub type RTCCALICFG = crate::Reg<rtccalicfg::RTCCALICFG_SPEC>;
#[doc = ""]
pub mod rtccalicfg;
#[doc = "RTCCALICFG1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtccalicfg1`] module"]
pub type RTCCALICFG1 = crate::Reg<rtccalicfg1::RTCCALICFG1_SPEC>;
#[doc = ""]
pub mod rtccalicfg1;
#[doc = "LACTCONFIG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactconfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactconfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lactconfig`] module"]
pub type LACTCONFIG = crate::Reg<lactconfig::LACTCONFIG_SPEC>;
#[doc = ""]
pub mod lactconfig;
#[doc = "LACTRTC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactrtc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactrtc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lactrtc`] module"]
pub type LACTRTC = crate::Reg<lactrtc::LACTRTC_SPEC>;
#[doc = ""]
pub mod lactrtc;
#[doc = "LACTLO (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactlo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lactlo`] module"]
pub type LACTLO = crate::Reg<lactlo::LACTLO_SPEC>;
#[doc = ""]
pub mod lactlo;
#[doc = "LACTHI (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lacthi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lacthi`] module"]
pub type LACTHI = crate::Reg<lacthi::LACTHI_SPEC>;
#[doc = ""]
pub mod lacthi;
#[doc = "LACTUPDATE (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactupdate::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lactupdate`] module"]
pub type LACTUPDATE = crate::Reg<lactupdate::LACTUPDATE_SPEC>;
#[doc = ""]
pub mod lactupdate;
#[doc = "LACTALARMLO (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactalarmlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactalarmlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lactalarmlo`] module"]
pub type LACTALARMLO = crate::Reg<lactalarmlo::LACTALARMLO_SPEC>;
#[doc = ""]
pub mod lactalarmlo;
#[doc = "LACTALARMHI (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactalarmhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactalarmhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lactalarmhi`] module"]
pub type LACTALARMHI = crate::Reg<lactalarmhi::LACTALARMHI_SPEC>;
#[doc = ""]
pub mod lactalarmhi;
#[doc = "LACTLOADLO (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactloadlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactloadlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lactloadlo`] module"]
pub type LACTLOADLO = crate::Reg<lactloadlo::LACTLOADLO_SPEC>;
#[doc = ""]
pub mod lactloadlo;
#[doc = "LACTLOADHI (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactloadhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactloadhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lactloadhi`] module"]
pub type LACTLOADHI = crate::Reg<lactloadhi::LACTLOADHI_SPEC>;
#[doc = ""]
pub mod lactloadhi;
#[doc = "LACTLOAD (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactload::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lactload`] module"]
pub type LACTLOAD = crate::Reg<lactload::LACTLOAD_SPEC>;
#[doc = ""]
pub mod lactload;
#[doc = "INT_ENA_TIMERS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena_timers::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_timers::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_ena_timers`] module"]
pub type INT_ENA_TIMERS = crate::Reg<int_ena_timers::INT_ENA_TIMERS_SPEC>;
#[doc = ""]
pub mod int_ena_timers;
#[doc = "INT_RAW_TIMERS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw_timers::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_raw_timers`] module"]
pub type INT_RAW_TIMERS = crate::Reg<int_raw_timers::INT_RAW_TIMERS_SPEC>;
#[doc = ""]
pub mod int_raw_timers;
#[doc = "INT_ST_TIMERS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_timers::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_st_timers`] module"]
pub type INT_ST_TIMERS = crate::Reg<int_st_timers::INT_ST_TIMERS_SPEC>;
#[doc = ""]
pub mod int_st_timers;
#[doc = "INT_CLR_TIMERS (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr_timers::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_clr_timers`] module"]
pub type INT_CLR_TIMERS = crate::Reg<int_clr_timers::INT_CLR_TIMERS_SPEC>;
#[doc = ""]
pub mod int_clr_timers;
#[doc = "NTIMERS_DATE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ntimers_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ntimers_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ntimers_date`] module"]
pub type NTIMERS_DATE = crate::Reg<ntimers_date::NTIMERS_DATE_SPEC>;
#[doc = ""]
pub mod ntimers_date;
#[doc = "TIMGCLK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timgclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timgclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timgclk`] module"]
pub type TIMGCLK = crate::Reg<timgclk::TIMGCLK_SPEC>;
#[doc = ""]
pub mod timgclk;
