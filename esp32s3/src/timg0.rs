#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer %s configuration register"]
    pub t0config: TCONFIG,
    #[doc = "0x04 - Timer %s current value, low 32 bits"]
    pub t0lo: TLO,
    #[doc = "0x08 - Timer %s current value, high 22 bits"]
    pub t0hi: THI,
    #[doc = "0x0c - Write to copy current timer value to TIMGn_T%s_(LO/HI)_REG"]
    pub t0update: TUPDATE,
    #[doc = "0x10 - Timer %s alarm value, low 32 bits"]
    pub t0alarmlo: TALARMLO,
    #[doc = "0x14 - Timer %s alarm value, high bits"]
    pub t0alarmhi: TALARMHI,
    #[doc = "0x18 - Timer %s reload value, low 32 bits"]
    pub t0loadlo: TLOADLO,
    #[doc = "0x1c - Timer %s reload value, high 22 bits"]
    pub t0loadhi: TLOADHI,
    #[doc = "0x20 - Write to reload timer from TIMG_T%s_(LOADLOLOADHI)_REG"]
    pub t0load: TLOAD,
    #[doc = "0x24 - Timer %s configuration register"]
    pub t1config: TCONFIG,
    #[doc = "0x28 - Timer %s current value, low 32 bits"]
    pub t1lo: TLO,
    #[doc = "0x2c - Timer %s current value, high 22 bits"]
    pub t1hi: THI,
    #[doc = "0x30 - Write to copy current timer value to TIMGn_T%s_(LO/HI)_REG"]
    pub t1update: TUPDATE,
    #[doc = "0x34 - Timer %s alarm value, low 32 bits"]
    pub t1alarmlo: TALARMLO,
    #[doc = "0x38 - Timer %s alarm value, high bits"]
    pub t1alarmhi: TALARMHI,
    #[doc = "0x3c - Timer %s reload value, low 32 bits"]
    pub t1loadlo: TLOADLO,
    #[doc = "0x40 - Timer %s reload value, high 22 bits"]
    pub t1loadhi: TLOADHI,
    #[doc = "0x44 - Write to reload timer from TIMG_T%s_(LOADLOLOADHI)_REG"]
    pub t1load: TLOAD,
    #[doc = "0x48 - Watchdog timer configuration register"]
    pub wdtconfig0: WDTCONFIG0,
    #[doc = "0x4c - Watchdog timer prescaler register"]
    pub wdtconfig1: WDTCONFIG1,
    #[doc = "0x50 - Watchdog timer stage 0 timeout value"]
    pub wdtconfig2: WDTCONFIG2,
    #[doc = "0x54 - Watchdog timer stage 1 timeout value"]
    pub wdtconfig3: WDTCONFIG3,
    #[doc = "0x58 - Watchdog timer stage 2 timeout value"]
    pub wdtconfig4: WDTCONFIG4,
    #[doc = "0x5c - Watchdog timer stage 3 timeout value"]
    pub wdtconfig5: WDTCONFIG5,
    #[doc = "0x60 - Write to feed the watchdog timer"]
    pub wdtfeed: WDTFEED,
    #[doc = "0x64 - Watchdog write protect register"]
    pub wdtwprotect: WDTWPROTECT,
    #[doc = "0x68 - RTC calibration configure register"]
    pub rtccalicfg: RTCCALICFG,
    #[doc = "0x6c - RTC calibration configure1 register"]
    pub rtccalicfg1: RTCCALICFG1,
    #[doc = "0x70 - Interrupt enable bits"]
    pub int_ena_timers: INT_ENA_TIMERS,
    #[doc = "0x74 - Raw interrupt status"]
    pub int_raw_timers: INT_RAW_TIMERS,
    #[doc = "0x78 - Masked interrupt status"]
    pub int_st_timers: INT_ST_TIMERS,
    #[doc = "0x7c - Interrupt clear bits"]
    pub int_clr_timers: INT_CLR_TIMERS,
    #[doc = "0x80 - Timer group calibration register"]
    pub rtccalicfg2: RTCCALICFG2,
    _reserved33: [u8; 0x74],
    #[doc = "0xf8 - Timer version control register"]
    pub ntimers_date: NTIMERS_DATE,
    #[doc = "0xfc - Timer group clock gate register"]
    pub regclk: REGCLK,
}
#[doc = "TCONFIG (rw) register accessor: Timer %s configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tconfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tconfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tconfig`] module"]
pub type TCONFIG = crate::Reg<tconfig::TCONFIG_SPEC>;
#[doc = "Timer %s configuration register"]
pub mod tconfig;
#[doc = "TLO (r) register accessor: Timer %s current value, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tlo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tlo`] module"]
pub type TLO = crate::Reg<tlo::TLO_SPEC>;
#[doc = "Timer %s current value, low 32 bits"]
pub mod tlo;
#[doc = "THI (r) register accessor: Timer %s current value, high 22 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`thi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`thi`] module"]
pub type THI = crate::Reg<thi::THI_SPEC>;
#[doc = "Timer %s current value, high 22 bits"]
pub mod thi;
#[doc = "TUPDATE (rw) register accessor: Write to copy current timer value to TIMGn_T%s_(LO/HI)_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tupdate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tupdate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tupdate`] module"]
pub type TUPDATE = crate::Reg<tupdate::TUPDATE_SPEC>;
#[doc = "Write to copy current timer value to TIMGn_T%s_(LO/HI)_REG"]
pub mod tupdate;
#[doc = "TALARMLO (rw) register accessor: Timer %s alarm value, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`talarmlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`talarmlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`talarmlo`] module"]
pub type TALARMLO = crate::Reg<talarmlo::TALARMLO_SPEC>;
#[doc = "Timer %s alarm value, low 32 bits"]
pub mod talarmlo;
#[doc = "TALARMHI (rw) register accessor: Timer %s alarm value, high bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`talarmhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`talarmhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`talarmhi`] module"]
pub type TALARMHI = crate::Reg<talarmhi::TALARMHI_SPEC>;
#[doc = "Timer %s alarm value, high bits"]
pub mod talarmhi;
#[doc = "TLOADLO (rw) register accessor: Timer %s reload value, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tloadlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tloadlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tloadlo`] module"]
pub type TLOADLO = crate::Reg<tloadlo::TLOADLO_SPEC>;
#[doc = "Timer %s reload value, low 32 bits"]
pub mod tloadlo;
#[doc = "TLOADHI (rw) register accessor: Timer %s reload value, high 22 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tloadhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tloadhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tloadhi`] module"]
pub type TLOADHI = crate::Reg<tloadhi::TLOADHI_SPEC>;
#[doc = "Timer %s reload value, high 22 bits"]
pub mod tloadhi;
#[doc = "TLOAD (w) register accessor: Write to reload timer from TIMG_T%s_(LOADLOLOADHI)_REG\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tload::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tload`] module"]
pub type TLOAD = crate::Reg<tload::TLOAD_SPEC>;
#[doc = "Write to reload timer from TIMG_T%s_(LOADLOLOADHI)_REG"]
pub mod tload;
#[doc = "WDTCONFIG0 (rw) register accessor: Watchdog timer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtconfig0`] module"]
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
#[doc = "Watchdog timer configuration register"]
pub mod wdtconfig0;
#[doc = "WDTCONFIG1 (rw) register accessor: Watchdog timer prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtconfig1`] module"]
pub type WDTCONFIG1 = crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>;
#[doc = "Watchdog timer prescaler register"]
pub mod wdtconfig1;
#[doc = "WDTCONFIG2 (rw) register accessor: Watchdog timer stage 0 timeout value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtconfig2`] module"]
pub type WDTCONFIG2 = crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>;
#[doc = "Watchdog timer stage 0 timeout value"]
pub mod wdtconfig2;
#[doc = "WDTCONFIG3 (rw) register accessor: Watchdog timer stage 1 timeout value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtconfig3`] module"]
pub type WDTCONFIG3 = crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>;
#[doc = "Watchdog timer stage 1 timeout value"]
pub mod wdtconfig3;
#[doc = "WDTCONFIG4 (rw) register accessor: Watchdog timer stage 2 timeout value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtconfig4`] module"]
pub type WDTCONFIG4 = crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>;
#[doc = "Watchdog timer stage 2 timeout value"]
pub mod wdtconfig4;
#[doc = "WDTCONFIG5 (rw) register accessor: Watchdog timer stage 3 timeout value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtconfig5`] module"]
pub type WDTCONFIG5 = crate::Reg<wdtconfig5::WDTCONFIG5_SPEC>;
#[doc = "Watchdog timer stage 3 timeout value"]
pub mod wdtconfig5;
#[doc = "WDTFEED (w) register accessor: Write to feed the watchdog timer\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtfeed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtfeed`] module"]
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
#[doc = "Write to feed the watchdog timer"]
pub mod wdtfeed;
#[doc = "WDTWPROTECT (rw) register accessor: Watchdog write protect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtwprotect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtwprotect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtwprotect`] module"]
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
#[doc = "Watchdog write protect register"]
pub mod wdtwprotect;
#[doc = "RTCCALICFG (rw) register accessor: RTC calibration configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccalicfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtccalicfg`] module"]
pub type RTCCALICFG = crate::Reg<rtccalicfg::RTCCALICFG_SPEC>;
#[doc = "RTC calibration configure register"]
pub mod rtccalicfg;
#[doc = "RTCCALICFG1 (r) register accessor: RTC calibration configure1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtccalicfg1`] module"]
pub type RTCCALICFG1 = crate::Reg<rtccalicfg1::RTCCALICFG1_SPEC>;
#[doc = "RTC calibration configure1 register"]
pub mod rtccalicfg1;
#[doc = "INT_ENA_TIMERS (rw) register accessor: Interrupt enable bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena_timers::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_timers::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_ena_timers`] module"]
pub type INT_ENA_TIMERS = crate::Reg<int_ena_timers::INT_ENA_TIMERS_SPEC>;
#[doc = "Interrupt enable bits"]
pub mod int_ena_timers;
#[doc = "INT_RAW_TIMERS (rw) register accessor: Raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw_timers::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw_timers::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_raw_timers`] module"]
pub type INT_RAW_TIMERS = crate::Reg<int_raw_timers::INT_RAW_TIMERS_SPEC>;
#[doc = "Raw interrupt status"]
pub mod int_raw_timers;
#[doc = "INT_ST_TIMERS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_timers::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_st_timers`] module"]
pub type INT_ST_TIMERS = crate::Reg<int_st_timers::INT_ST_TIMERS_SPEC>;
#[doc = "Masked interrupt status"]
pub mod int_st_timers;
#[doc = "INT_CLR_TIMERS (w) register accessor: Interrupt clear bits\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr_timers::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_clr_timers`] module"]
pub type INT_CLR_TIMERS = crate::Reg<int_clr_timers::INT_CLR_TIMERS_SPEC>;
#[doc = "Interrupt clear bits"]
pub mod int_clr_timers;
#[doc = "RTCCALICFG2 (rw) register accessor: Timer group calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccalicfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtccalicfg2`] module"]
pub type RTCCALICFG2 = crate::Reg<rtccalicfg2::RTCCALICFG2_SPEC>;
#[doc = "Timer group calibration register"]
pub mod rtccalicfg2;
#[doc = "NTIMERS_DATE (rw) register accessor: Timer version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ntimers_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ntimers_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ntimers_date`] module"]
pub type NTIMERS_DATE = crate::Reg<ntimers_date::NTIMERS_DATE_SPEC>;
#[doc = "Timer version control register"]
pub mod ntimers_date;
#[doc = "REGCLK (rw) register accessor: Timer group clock gate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`regclk`] module"]
pub type REGCLK = crate::Reg<regclk::REGCLK_SPEC>;
#[doc = "Timer group clock gate register"]
pub mod regclk;
