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
    ntimers_date: NTIMERS_DATE,
    regclk: REGCLK,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer %s configuration register"]
    #[inline(always)]
    pub const fn t0config(&self) -> &T0CONFIG {
        &self.t0config
    }
    #[doc = "0x04 - Timer %s current value, low 32 bits"]
    #[inline(always)]
    pub const fn t0lo(&self) -> &T0LO {
        &self.t0lo
    }
    #[doc = "0x08 - Timer %s current value, high 22 bits"]
    #[inline(always)]
    pub const fn t0hi(&self) -> &T0HI {
        &self.t0hi
    }
    #[doc = "0x0c - Write to copy current timer value to TIMGn_T%s_(LO/HI)_REG"]
    #[inline(always)]
    pub const fn t0update(&self) -> &T0UPDATE {
        &self.t0update
    }
    #[doc = "0x10 - Timer %s alarm value, low 32 bits"]
    #[inline(always)]
    pub const fn t0alarmlo(&self) -> &T0ALARMLO {
        &self.t0alarmlo
    }
    #[doc = "0x14 - Timer %s alarm value, high bits"]
    #[inline(always)]
    pub const fn t0alarmhi(&self) -> &T0ALARMHI {
        &self.t0alarmhi
    }
    #[doc = "0x18 - Timer %s reload value, low 32 bits"]
    #[inline(always)]
    pub const fn t0loadlo(&self) -> &T0LOADLO {
        &self.t0loadlo
    }
    #[doc = "0x1c - Timer %s reload value, high 22 bits"]
    #[inline(always)]
    pub const fn t0loadhi(&self) -> &T0LOADHI {
        &self.t0loadhi
    }
    #[doc = "0x20 - Write to reload timer from TIMG_T%s_(LOADLOLOADHI)_REG"]
    #[inline(always)]
    pub const fn t0load(&self) -> &T0LOAD {
        &self.t0load
    }
    #[doc = "0x48 - Watchdog timer configuration register"]
    #[inline(always)]
    pub const fn wdtconfig0(&self) -> &WDTCONFIG0 {
        &self.wdtconfig0
    }
    #[doc = "0x4c - Watchdog timer prescaler register"]
    #[inline(always)]
    pub const fn wdtconfig1(&self) -> &WDTCONFIG1 {
        &self.wdtconfig1
    }
    #[doc = "0x50 - Watchdog timer stage 0 timeout value"]
    #[inline(always)]
    pub const fn wdtconfig2(&self) -> &WDTCONFIG2 {
        &self.wdtconfig2
    }
    #[doc = "0x54 - Watchdog timer stage 1 timeout value"]
    #[inline(always)]
    pub const fn wdtconfig3(&self) -> &WDTCONFIG3 {
        &self.wdtconfig3
    }
    #[doc = "0x58 - Watchdog timer stage 2 timeout value"]
    #[inline(always)]
    pub const fn wdtconfig4(&self) -> &WDTCONFIG4 {
        &self.wdtconfig4
    }
    #[doc = "0x5c - Watchdog timer stage 3 timeout value"]
    #[inline(always)]
    pub const fn wdtconfig5(&self) -> &WDTCONFIG5 {
        &self.wdtconfig5
    }
    #[doc = "0x60 - Write to feed the watchdog timer"]
    #[inline(always)]
    pub const fn wdtfeed(&self) -> &WDTFEED {
        &self.wdtfeed
    }
    #[doc = "0x64 - Watchdog write protect register"]
    #[inline(always)]
    pub const fn wdtwprotect(&self) -> &WDTWPROTECT {
        &self.wdtwprotect
    }
    #[doc = "0x68 - RTC calibration configure register"]
    #[inline(always)]
    pub const fn rtccalicfg(&self) -> &RTCCALICFG {
        &self.rtccalicfg
    }
    #[doc = "0x6c - RTC calibration configure1 register"]
    #[inline(always)]
    pub const fn rtccalicfg1(&self) -> &RTCCALICFG1 {
        &self.rtccalicfg1
    }
    #[doc = "0x70 - Interrupt enable bits"]
    #[inline(always)]
    pub const fn int_ena_timers(&self) -> &INT_ENA_TIMERS {
        &self.int_ena_timers
    }
    #[doc = "0x74 - Raw interrupt status"]
    #[inline(always)]
    pub const fn int_raw_timers(&self) -> &INT_RAW_TIMERS {
        &self.int_raw_timers
    }
    #[doc = "0x78 - Masked interrupt status"]
    #[inline(always)]
    pub const fn int_st_timers(&self) -> &INT_ST_TIMERS {
        &self.int_st_timers
    }
    #[doc = "0x7c - Interrupt clear bits"]
    #[inline(always)]
    pub const fn int_clr_timers(&self) -> &INT_CLR_TIMERS {
        &self.int_clr_timers
    }
    #[doc = "0x80 - Timer group calibration register"]
    #[inline(always)]
    pub const fn rtccalicfg2(&self) -> &RTCCALICFG2 {
        &self.rtccalicfg2
    }
    #[doc = "0xf8 - Timer version control register"]
    #[inline(always)]
    pub const fn ntimers_date(&self) -> &NTIMERS_DATE {
        &self.ntimers_date
    }
    #[doc = "0xfc - Timer group clock gate register"]
    #[inline(always)]
    pub const fn regclk(&self) -> &REGCLK {
        &self.regclk
    }
}
#[doc = "T0CONFIG (rw) register accessor: Timer %s configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0config`] module"]
pub type T0CONFIG = crate::Reg<t0config::T0CONFIG_SPEC>;
#[doc = "Timer %s configuration register"]
pub mod t0config;
#[doc = "T0LO (r) register accessor: Timer %s current value, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0lo`] module"]
pub type T0LO = crate::Reg<t0lo::T0LO_SPEC>;
#[doc = "Timer %s current value, low 32 bits"]
pub mod t0lo;
#[doc = "T0HI (r) register accessor: Timer %s current value, high 22 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0hi`] module"]
pub type T0HI = crate::Reg<t0hi::T0HI_SPEC>;
#[doc = "Timer %s current value, high 22 bits"]
pub mod t0hi;
#[doc = "T0UPDATE (rw) register accessor: Write to copy current timer value to TIMGn_T%s_(LO/HI)_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0update::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0update::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0update`] module"]
pub type T0UPDATE = crate::Reg<t0update::T0UPDATE_SPEC>;
#[doc = "Write to copy current timer value to TIMGn_T%s_(LO/HI)_REG"]
pub mod t0update;
#[doc = "T0ALARMLO (rw) register accessor: Timer %s alarm value, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0alarmlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0alarmlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0alarmlo`] module"]
pub type T0ALARMLO = crate::Reg<t0alarmlo::T0ALARMLO_SPEC>;
#[doc = "Timer %s alarm value, low 32 bits"]
pub mod t0alarmlo;
#[doc = "T0ALARMHI (rw) register accessor: Timer %s alarm value, high bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0alarmhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0alarmhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0alarmhi`] module"]
pub type T0ALARMHI = crate::Reg<t0alarmhi::T0ALARMHI_SPEC>;
#[doc = "Timer %s alarm value, high bits"]
pub mod t0alarmhi;
#[doc = "T0LOADLO (rw) register accessor: Timer %s reload value, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0loadlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0loadlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0loadlo`] module"]
pub type T0LOADLO = crate::Reg<t0loadlo::T0LOADLO_SPEC>;
#[doc = "Timer %s reload value, low 32 bits"]
pub mod t0loadlo;
#[doc = "T0LOADHI (rw) register accessor: Timer %s reload value, high 22 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0loadhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0loadhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0loadhi`] module"]
pub type T0LOADHI = crate::Reg<t0loadhi::T0LOADHI_SPEC>;
#[doc = "Timer %s reload value, high 22 bits"]
pub mod t0loadhi;
#[doc = "T0LOAD (w) register accessor: Write to reload timer from TIMG_T%s_(LOADLOLOADHI)_REG\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0load::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0load`] module"]
pub type T0LOAD = crate::Reg<t0load::T0LOAD_SPEC>;
#[doc = "Write to reload timer from TIMG_T%s_(LOADLOLOADHI)_REG"]
pub mod t0load;
#[doc = "WDTCONFIG0 (rw) register accessor: Watchdog timer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig0`] module"]
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
#[doc = "Watchdog timer configuration register"]
pub mod wdtconfig0;
#[doc = "WDTCONFIG1 (rw) register accessor: Watchdog timer prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig1`] module"]
pub type WDTCONFIG1 = crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>;
#[doc = "Watchdog timer prescaler register"]
pub mod wdtconfig1;
#[doc = "WDTCONFIG2 (rw) register accessor: Watchdog timer stage 0 timeout value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig2`] module"]
pub type WDTCONFIG2 = crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>;
#[doc = "Watchdog timer stage 0 timeout value"]
pub mod wdtconfig2;
#[doc = "WDTCONFIG3 (rw) register accessor: Watchdog timer stage 1 timeout value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig3`] module"]
pub type WDTCONFIG3 = crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>;
#[doc = "Watchdog timer stage 1 timeout value"]
pub mod wdtconfig3;
#[doc = "WDTCONFIG4 (rw) register accessor: Watchdog timer stage 2 timeout value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig4`] module"]
pub type WDTCONFIG4 = crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>;
#[doc = "Watchdog timer stage 2 timeout value"]
pub mod wdtconfig4;
#[doc = "WDTCONFIG5 (rw) register accessor: Watchdog timer stage 3 timeout value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig5`] module"]
pub type WDTCONFIG5 = crate::Reg<wdtconfig5::WDTCONFIG5_SPEC>;
#[doc = "Watchdog timer stage 3 timeout value"]
pub mod wdtconfig5;
#[doc = "WDTFEED (w) register accessor: Write to feed the watchdog timer\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtfeed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtfeed`] module"]
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
#[doc = "Write to feed the watchdog timer"]
pub mod wdtfeed;
#[doc = "WDTWPROTECT (rw) register accessor: Watchdog write protect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtwprotect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtwprotect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtwprotect`] module"]
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
#[doc = "Watchdog write protect register"]
pub mod wdtwprotect;
#[doc = "RTCCALICFG (rw) register accessor: RTC calibration configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccalicfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccalicfg`] module"]
pub type RTCCALICFG = crate::Reg<rtccalicfg::RTCCALICFG_SPEC>;
#[doc = "RTC calibration configure register"]
pub mod rtccalicfg;
#[doc = "RTCCALICFG1 (r) register accessor: RTC calibration configure1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccalicfg1`] module"]
pub type RTCCALICFG1 = crate::Reg<rtccalicfg1::RTCCALICFG1_SPEC>;
#[doc = "RTC calibration configure1 register"]
pub mod rtccalicfg1;
#[doc = "INT_ENA_TIMERS (rw) register accessor: Interrupt enable bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena_timers::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_timers::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena_timers`] module"]
pub type INT_ENA_TIMERS = crate::Reg<int_ena_timers::INT_ENA_TIMERS_SPEC>;
#[doc = "Interrupt enable bits"]
pub mod int_ena_timers;
#[doc = "INT_RAW_TIMERS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw_timers::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw_timers`] module"]
pub type INT_RAW_TIMERS = crate::Reg<int_raw_timers::INT_RAW_TIMERS_SPEC>;
#[doc = "Raw interrupt status"]
pub mod int_raw_timers;
#[doc = "INT_ST_TIMERS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_timers::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_timers`] module"]
pub type INT_ST_TIMERS = crate::Reg<int_st_timers::INT_ST_TIMERS_SPEC>;
#[doc = "Masked interrupt status"]
pub mod int_st_timers;
#[doc = "INT_CLR_TIMERS (w) register accessor: Interrupt clear bits\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr_timers::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr_timers`] module"]
pub type INT_CLR_TIMERS = crate::Reg<int_clr_timers::INT_CLR_TIMERS_SPEC>;
#[doc = "Interrupt clear bits"]
pub mod int_clr_timers;
#[doc = "RTCCALICFG2 (rw) register accessor: Timer group calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccalicfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccalicfg2`] module"]
pub type RTCCALICFG2 = crate::Reg<rtccalicfg2::RTCCALICFG2_SPEC>;
#[doc = "Timer group calibration register"]
pub mod rtccalicfg2;
#[doc = "NTIMERS_DATE (rw) register accessor: Timer version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ntimers_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ntimers_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ntimers_date`] module"]
pub type NTIMERS_DATE = crate::Reg<ntimers_date::NTIMERS_DATE_SPEC>;
#[doc = "Timer version control register"]
pub mod ntimers_date;
#[doc = "REGCLK (rw) register accessor: Timer group clock gate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regclk`] module"]
pub type REGCLK = crate::Reg<regclk::REGCLK_SPEC>;
#[doc = "Timer group clock gate register"]
pub mod regclk;
