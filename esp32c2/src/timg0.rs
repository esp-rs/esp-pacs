#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer %s configuration register"]
    pub t0config: T0CONFIG,
    #[doc = "0x04 - Timer %s current value, low 32 bits"]
    pub t0lo: T0LO,
    #[doc = "0x08 - Timer %s current value, high 22 bits"]
    pub t0hi: T0HI,
    #[doc = "0x0c - Write to copy current timer value to TIMGn_T%s_(LO/HI)_REG"]
    pub t0update: T0UPDATE,
    #[doc = "0x10 - Timer %s alarm value, low 32 bits"]
    pub t0alarmlo: T0ALARMLO,
    #[doc = "0x14 - Timer %s alarm value, high bits"]
    pub t0alarmhi: T0ALARMHI,
    #[doc = "0x18 - Timer %s reload value, low 32 bits"]
    pub t0loadlo: T0LOADLO,
    #[doc = "0x1c - Timer %s reload value, high 22 bits"]
    pub t0loadhi: T0LOADHI,
    #[doc = "0x20 - Write to reload timer from TIMG_T%s_(LOADLOLOADHI)_REG"]
    pub t0load: T0LOAD,
    _reserved9: [u8; 0x24],
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
    _reserved24: [u8; 0x74],
    #[doc = "0xf8 - Timer version control register"]
    pub ntimers_date: NTIMERS_DATE,
    #[doc = "0xfc - Timer group clock gate register"]
    pub regclk: REGCLK,
}
#[doc = "T0CONFIG (rw) register accessor: an alias for `Reg<T0CONFIG_SPEC>`"]
pub type T0CONFIG = crate::Reg<t0config::T0CONFIG_SPEC>;
#[doc = "Timer %s configuration register"]
pub mod t0config;
#[doc = "T0LO (r) register accessor: an alias for `Reg<T0LO_SPEC>`"]
pub type T0LO = crate::Reg<t0lo::T0LO_SPEC>;
#[doc = "Timer %s current value, low 32 bits"]
pub mod t0lo;
#[doc = "T0HI (r) register accessor: an alias for `Reg<T0HI_SPEC>`"]
pub type T0HI = crate::Reg<t0hi::T0HI_SPEC>;
#[doc = "Timer %s current value, high 22 bits"]
pub mod t0hi;
#[doc = "T0UPDATE (rw) register accessor: an alias for `Reg<T0UPDATE_SPEC>`"]
pub type T0UPDATE = crate::Reg<t0update::T0UPDATE_SPEC>;
#[doc = "Write to copy current timer value to TIMGn_T%s_(LO/HI)_REG"]
pub mod t0update;
#[doc = "T0ALARMLO (rw) register accessor: an alias for `Reg<T0ALARMLO_SPEC>`"]
pub type T0ALARMLO = crate::Reg<t0alarmlo::T0ALARMLO_SPEC>;
#[doc = "Timer %s alarm value, low 32 bits"]
pub mod t0alarmlo;
#[doc = "T0ALARMHI (rw) register accessor: an alias for `Reg<T0ALARMHI_SPEC>`"]
pub type T0ALARMHI = crate::Reg<t0alarmhi::T0ALARMHI_SPEC>;
#[doc = "Timer %s alarm value, high bits"]
pub mod t0alarmhi;
#[doc = "T0LOADLO (rw) register accessor: an alias for `Reg<T0LOADLO_SPEC>`"]
pub type T0LOADLO = crate::Reg<t0loadlo::T0LOADLO_SPEC>;
#[doc = "Timer %s reload value, low 32 bits"]
pub mod t0loadlo;
#[doc = "T0LOADHI (rw) register accessor: an alias for `Reg<T0LOADHI_SPEC>`"]
pub type T0LOADHI = crate::Reg<t0loadhi::T0LOADHI_SPEC>;
#[doc = "Timer %s reload value, high 22 bits"]
pub mod t0loadhi;
#[doc = "T0LOAD (w) register accessor: an alias for `Reg<T0LOAD_SPEC>`"]
pub type T0LOAD = crate::Reg<t0load::T0LOAD_SPEC>;
#[doc = "Write to reload timer from TIMG_T%s_(LOADLOLOADHI)_REG"]
pub mod t0load;
#[doc = "WDTCONFIG0 (rw) register accessor: an alias for `Reg<WDTCONFIG0_SPEC>`"]
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
#[doc = "Watchdog timer configuration register"]
pub mod wdtconfig0;
#[doc = "WDTCONFIG1 (rw) register accessor: an alias for `Reg<WDTCONFIG1_SPEC>`"]
pub type WDTCONFIG1 = crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>;
#[doc = "Watchdog timer prescaler register"]
pub mod wdtconfig1;
#[doc = "WDTCONFIG2 (rw) register accessor: an alias for `Reg<WDTCONFIG2_SPEC>`"]
pub type WDTCONFIG2 = crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>;
#[doc = "Watchdog timer stage 0 timeout value"]
pub mod wdtconfig2;
#[doc = "WDTCONFIG3 (rw) register accessor: an alias for `Reg<WDTCONFIG3_SPEC>`"]
pub type WDTCONFIG3 = crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>;
#[doc = "Watchdog timer stage 1 timeout value"]
pub mod wdtconfig3;
#[doc = "WDTCONFIG4 (rw) register accessor: an alias for `Reg<WDTCONFIG4_SPEC>`"]
pub type WDTCONFIG4 = crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>;
#[doc = "Watchdog timer stage 2 timeout value"]
pub mod wdtconfig4;
#[doc = "WDTCONFIG5 (rw) register accessor: an alias for `Reg<WDTCONFIG5_SPEC>`"]
pub type WDTCONFIG5 = crate::Reg<wdtconfig5::WDTCONFIG5_SPEC>;
#[doc = "Watchdog timer stage 3 timeout value"]
pub mod wdtconfig5;
#[doc = "WDTFEED (w) register accessor: an alias for `Reg<WDTFEED_SPEC>`"]
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
#[doc = "Write to feed the watchdog timer"]
pub mod wdtfeed;
#[doc = "WDTWPROTECT (rw) register accessor: an alias for `Reg<WDTWPROTECT_SPEC>`"]
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
#[doc = "Watchdog write protect register"]
pub mod wdtwprotect;
#[doc = "RTCCALICFG (rw) register accessor: an alias for `Reg<RTCCALICFG_SPEC>`"]
pub type RTCCALICFG = crate::Reg<rtccalicfg::RTCCALICFG_SPEC>;
#[doc = "RTC calibration configure register"]
pub mod rtccalicfg;
#[doc = "RTCCALICFG1 (r) register accessor: an alias for `Reg<RTCCALICFG1_SPEC>`"]
pub type RTCCALICFG1 = crate::Reg<rtccalicfg1::RTCCALICFG1_SPEC>;
#[doc = "RTC calibration configure1 register"]
pub mod rtccalicfg1;
#[doc = "INT_ENA_TIMERS (rw) register accessor: an alias for `Reg<INT_ENA_TIMERS_SPEC>`"]
pub type INT_ENA_TIMERS = crate::Reg<int_ena_timers::INT_ENA_TIMERS_SPEC>;
#[doc = "Interrupt enable bits"]
pub mod int_ena_timers;
#[doc = "INT_RAW_TIMERS (r) register accessor: an alias for `Reg<INT_RAW_TIMERS_SPEC>`"]
pub type INT_RAW_TIMERS = crate::Reg<int_raw_timers::INT_RAW_TIMERS_SPEC>;
#[doc = "Raw interrupt status"]
pub mod int_raw_timers;
#[doc = "INT_ST_TIMERS (r) register accessor: an alias for `Reg<INT_ST_TIMERS_SPEC>`"]
pub type INT_ST_TIMERS = crate::Reg<int_st_timers::INT_ST_TIMERS_SPEC>;
#[doc = "Masked interrupt status"]
pub mod int_st_timers;
#[doc = "INT_CLR_TIMERS (w) register accessor: an alias for `Reg<INT_CLR_TIMERS_SPEC>`"]
pub type INT_CLR_TIMERS = crate::Reg<int_clr_timers::INT_CLR_TIMERS_SPEC>;
#[doc = "Interrupt clear bits"]
pub mod int_clr_timers;
#[doc = "RTCCALICFG2 (rw) register accessor: an alias for `Reg<RTCCALICFG2_SPEC>`"]
pub type RTCCALICFG2 = crate::Reg<rtccalicfg2::RTCCALICFG2_SPEC>;
#[doc = "Timer group calibration register"]
pub mod rtccalicfg2;
#[doc = "NTIMERS_DATE (rw) register accessor: an alias for `Reg<NTIMERS_DATE_SPEC>`"]
pub type NTIMERS_DATE = crate::Reg<ntimers_date::NTIMERS_DATE_SPEC>;
#[doc = "Timer version control register"]
pub mod ntimers_date;
#[doc = "REGCLK (rw) register accessor: an alias for `Reg<REGCLK_SPEC>`"]
pub type REGCLK = crate::Reg<regclk::REGCLK_SPEC>;
#[doc = "Timer group clock gate register"]
pub mod regclk;
